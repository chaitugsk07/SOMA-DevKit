//! Provider-agnostic LLM types and the `ProviderAdapter` trait.
//!
//! All concrete providers implement `ProviderAdapter`. The consuming service
//! calls the adapter methods with fully-formed requests; no routing, fallback,
//! or provider-selection logic lives here.

use std::pin::Pin;

// ── Utilities shared across adapters ─────────────────────────────────────────

/// serde `skip_serializing_if` helper: skip a `bool` field when it is `false`.
///
/// Used by both `AnthropicAdapter` and `OpenAICompatibleAdapter` for their
/// `stream` field — the field must be absent from the JSON body on non-streaming
/// calls, and `serde(skip_serializing_if = "is_false")` achieves this.
pub(crate) fn is_false(b: &bool) -> bool {
    !*b
}

// ── Shared request / response types ──────────────────────────────────────────

/// Role of a chat message turn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatRole {
    System,
    User,
    Assistant,
}

/// A single message in a conversation.
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

/// A tool the model may call (JSON Schema parameters).
#[derive(Debug, Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: Option<String>,
    /// JSON Schema object describing the tool's parameters.
    pub parameters: serde_json::Value,
}

/// Unified completion request — provider-agnostic.
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub system: Option<String>,
    pub tools: Option<Vec<ToolDefinition>>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub max_tokens: u32,
}

/// A tool call emitted by the model.
#[derive(Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Token usage for a completion.
#[derive(Debug, Clone, Copy, Default)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

/// Unified completion response.
#[derive(Debug, Clone)]
pub struct CompletionResponse {
    /// Provider-assigned message id.
    pub id: String,
    /// Concatenated text content blocks.
    pub content: String,
    /// Tool calls the model requested.
    pub tool_calls: Vec<ToolCall>,
    pub stop_reason: Option<String>,
    pub usage: Usage,
}

/// An incremental event from a streaming completion.
#[derive(Debug, Clone)]
pub enum StreamEvent {
    /// Incremental text token.
    Token(String),
    /// Incremental tool-call argument delta.
    ToolCallDelta {
        id: String,
        name: String,
        arguments_delta: String,
    },
    /// Stream finished.
    Done {
        stop_reason: Option<String>,
        usage: Option<Usage>,
    },
}

/// Embeddings request.
#[derive(Debug, Clone)]
pub struct EmbedRequest {
    pub model: String,
    pub texts: Vec<String>,
    /// Optional output dimension for providers/models that support shortening embeddings.
    pub dimensions: Option<usize>,
}

/// Embeddings response.
#[derive(Debug, Clone)]
pub struct EmbedResponse {
    pub embeddings: Vec<Vec<f32>>,
    pub usage: Option<Usage>,
}

// ── ProviderError ─────────────────────────────────────────────────────────────

/// Provider-level error.
///
/// Display and Debug implementations MUST NOT include API key material.
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    /// Authentication failed (401/403). No key value is included.
    #[error("authentication failed")]
    Auth,
    /// Rate limited by the provider.
    #[error("rate limited (retry after {retry_after_secs:?}s)")]
    RateLimited { retry_after_secs: Option<u64> },
    /// The request input exceeded the model's context window.
    #[error("input context overflow")]
    ContextOverflow,
    /// Provider-side error (5xx or network).
    #[error("provider unavailable")]
    ProviderUnavailable,
    /// Provider rejected the request (4xx). Carries the raw response body so
    /// callers can log the real rejection reason (e.g. "temperature not supported").
    ///
    /// SAFETY: body is the API error message from the response body, never a
    /// request header or credential. Display/Debug MUST NOT be extended with
    /// header values.
    #[error("bad request (HTTP {status}): {body}")]
    BadRequest { status: u16, body: String },
    /// A response could not be parsed. The message must never contain auth material.
    #[error("malformed response: {0}")]
    Malformed(String),
    /// The operation is not supported by this provider.
    #[error("not supported: {0}")]
    NotSupported(&'static str),
    /// Underlying HTTP transport error.
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

// ── Incremental SSE parsing utilities ────────────────────────────────────────

/// Drain all complete newline-terminated lines from a byte buffer, returning
/// them without the terminating newline. Bytes belonging to an incomplete line
/// (no `\n` seen yet) remain in `buf`.
///
/// Handles both `\r\n` and bare `\n`. Safe with multi-byte UTF-8: the byte
/// 0x0A (`\n`) cannot appear as a continuation byte in a valid UTF-8 sequence
/// (continuation bytes are 0x80–0xBF), so splitting on 0x0A is always correct.
pub(crate) fn drain_lines(buf: &mut Vec<u8>) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    loop {
        match buf.iter().position(|&b| b == b'\n') {
            None => break,
            Some(nl) => {
                // Strip optional carriage-return before the newline
                let end = if nl > 0 && buf[nl - 1] == b'\r' {
                    nl - 1
                } else {
                    nl
                };
                out.push(buf[..end].to_vec());
                buf.drain(..=nl);
            }
        }
    }
    out
}

/// Accumulates SSE field lines (`event:`, `data:`) between blank-line event
/// boundaries (RFC 8895).
///
/// Feed each decoded line via [`SseAcc::feed`]; the method returns `true` when
/// a blank line is received, signalling that the accumulated event should be
/// dispatched. Call [`SseAcc::take`] to retrieve and reset the fields.
pub(crate) struct SseAcc {
    pub(crate) event: Option<String>,
    pub(crate) data: Option<String>,
}

impl SseAcc {
    pub(crate) fn new() -> Self {
        Self {
            event: None,
            data: None,
        }
    }

    /// Feed a line (without its terminating `\n`). Returns `true` if the line
    /// is blank, indicating the current event is complete and ready to dispatch.
    pub(crate) fn feed(&mut self, line: &[u8]) -> bool {
        if line.is_empty() {
            return true;
        }
        // SSE field prefixes are ASCII; the value portion may be UTF-8.
        let s = String::from_utf8_lossy(line);
        if let Some(ev) = s.strip_prefix("event: ") {
            self.event = Some(ev.to_string());
        } else if let Some(data) = s.strip_prefix("data: ") {
            // RFC 8895 §9.2.6: when an event has multiple `data:` lines, their
            // values are concatenated with a U+000A LINE FEED between them.
            match self.data.as_mut() {
                Some(existing) => {
                    existing.push('\n');
                    existing.push_str(data);
                }
                None => self.data = Some(data.to_string()),
            }
        }
        // Comment lines (`:`) and unrecognised fields are ignored per RFC 8895.
        false
    }

    /// Take and clear the accumulated event and data fields.
    pub(crate) fn take(&mut self) -> (Option<String>, Option<String>) {
        (self.event.take(), self.data.take())
    }
}

// ── ProviderAdapter trait ─────────────────────────────────────────────────────

/// Implemented by every LLM provider. The trait is object-safe via `async_trait`.
///
/// Implementors own the HTTP client and credentials. All routing, retry, and
/// provider-selection logic lives in the consuming service.
#[async_trait::async_trait]
pub trait ProviderAdapter: Send + Sync {
    /// Non-streaming completion.
    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, ProviderError>;

    /// Streaming completion. Returns a pinned stream of events emitted
    /// incrementally as SSE event boundaries are parsed from the byte stream.
    ///
    /// Transport errors mid-stream are emitted as `Err` items rather than
    /// silently truncating the stream.
    async fn complete_stream(
        &self,
        req: CompletionRequest,
    ) -> Result<
        Pin<
            Box<
                dyn futures_core::Stream<Item = Result<StreamEvent, ProviderError>>
                    + Send
                    + 'static,
            >,
        >,
        ProviderError,
    >;

    /// Compute text embeddings.
    async fn embed(&self, req: EmbedRequest) -> Result<EmbedResponse, ProviderError>;
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drain_lines_basic() {
        let mut buf = b"hello\nworld\n".to_vec();
        let lines = drain_lines(&mut buf);
        assert_eq!(lines, vec![b"hello".to_vec(), b"world".to_vec()]);
        assert!(buf.is_empty());
    }

    #[test]
    fn drain_lines_partial_line_stays_in_buf() {
        let mut buf = b"hello\nwor".to_vec();
        let lines = drain_lines(&mut buf);
        assert_eq!(lines, vec![b"hello".to_vec()]);
        assert_eq!(buf, b"wor");
    }

    #[test]
    fn drain_lines_crlf() {
        let mut buf = b"foo\r\nbar\r\n".to_vec();
        let lines = drain_lines(&mut buf);
        assert_eq!(lines, vec![b"foo".to_vec(), b"bar".to_vec()]);
    }

    #[test]
    fn drain_lines_blank_line() {
        let mut buf = b"event: x\n\n".to_vec();
        let lines = drain_lines(&mut buf);
        assert_eq!(lines, vec![b"event: x".to_vec(), b"".to_vec()]);
        assert!(buf.is_empty());
    }

    #[test]
    fn drain_lines_mid_utf8_boundary() {
        // "é" = 0xC3 0xA9 — split: first byte arrives, then second byte + newline
        let mut buf = vec![0xC3u8];
        let lines = drain_lines(&mut buf);
        assert!(lines.is_empty(), "no complete line yet");
        assert_eq!(buf, vec![0xC3u8], "first byte retained");

        // Complete the codepoint and add newline
        buf.extend_from_slice(b"\xA9\n");
        let lines = drain_lines(&mut buf);
        assert_eq!(lines.len(), 1);
        assert_eq!(String::from_utf8_lossy(&lines[0]), "é");
        assert!(buf.is_empty());
    }

    #[test]
    fn sse_acc_accumulates_and_dispatches() {
        let mut acc = SseAcc::new();
        assert!(!acc.feed(b"event: message_start"));
        assert!(!acc.feed(b"data: {\"type\":\"message_start\"}"));
        assert!(acc.feed(b""), "blank line triggers dispatch");
        let (ev, data) = acc.take();
        assert_eq!(ev.as_deref(), Some("message_start"));
        assert_eq!(data.as_deref(), Some("{\"type\":\"message_start\"}"));
        // After take, fields are cleared
        let (ev2, data2) = acc.take();
        assert!(ev2.is_none());
        assert!(data2.is_none());
    }

    #[test]
    fn sse_acc_ignores_comment_lines() {
        let mut acc = SseAcc::new();
        acc.feed(b": this is a comment");
        acc.feed(b"data: hello");
        acc.feed(b"");
        let (ev, data) = acc.take();
        assert!(ev.is_none());
        assert_eq!(data.as_deref(), Some("hello"));
    }

    #[test]
    fn sse_acc_multi_data_lines_concatenated() {
        // RFC 8895 §9.2.6: multiple `data:` lines within one SSE event must be
        // concatenated with '\n', not overwritten. A proxy that re-chunks an
        // OpenAI or Anthropic response may produce multi-line data events.
        let mut acc = SseAcc::new();
        assert!(!acc.feed(b"data: first line"));
        assert!(!acc.feed(b"data: second line"));
        assert!(acc.feed(b""), "blank line triggers dispatch");
        let (_, data) = acc.take();
        assert_eq!(
            data.as_deref(),
            Some("first line\nsecond line"),
            "data lines must be joined with a newline, not overwritten"
        );
    }

    #[test]
    fn sse_acc_multi_data_lines_three_lines() {
        let mut acc = SseAcc::new();
        acc.feed(b"data: a");
        acc.feed(b"data: b");
        acc.feed(b"data: c");
        acc.feed(b"");
        let (_, data) = acc.take();
        assert_eq!(data.as_deref(), Some("a\nb\nc"));
    }
}
