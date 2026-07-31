//! Anthropic Messages API adapter.
//!
//! `AnthropicAdapter` translates the provider-agnostic `CompletionRequest` into
//! Anthropic's wire format and maps the response back. The existing `LlmClient`
//! in `mod.rs` is kept unchanged; this is a new parallel type.
//!
//! Streaming (`complete_stream`) is genuinely incremental: it uses
//! `reqwest::Response::bytes_stream()` and an SSE line buffer that accumulates
//! bytes across chunk boundaries — including mid-line, mid-event, and mid-UTF-8
//! splits — emitting each `StreamEvent` the moment its SSE event is complete.
//! Transport errors mid-stream surface as `Err` items rather than silent
//! truncation.

use std::pin::Pin;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};

use super::adapter::{
    drain_lines, is_false, ChatRole, CompletionRequest, CompletionResponse, EmbedRequest,
    EmbedResponse, ProviderAdapter, ProviderError, ReasoningEffort, SseAcc, StreamEvent, ToolCall,
    Usage,
};

const ANTHROPIC_API_VERSION: &str = "2023-06-01";

// ── Wire types (Anthropic → serialise) ───────────────────────────────────────

#[derive(serde::Serialize)]
struct AnthropicReq {
    model: String,
    max_tokens: u32,
    /// Either a plain JSON string (no caching) or a content-block array with
    /// `cache_control` (when `cache_system_prompt` is set on the request).
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<serde_json::Value>,
    messages: Vec<AnthropicMsg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AnthropicTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "is_false")]
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_config: Option<AnthropicOutputConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thinking: Option<AnthropicThinking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
}

/// Merged structured-output + reasoning-effort config for Anthropic.
#[derive(serde::Serialize)]
struct AnthropicOutputConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effort: Option<&'static str>,
}

/// Adaptive thinking toggle for Anthropic.
#[derive(serde::Serialize)]
struct AnthropicThinking {
    #[serde(rename = "type")]
    ty: &'static str,
}

#[derive(serde::Serialize)]
struct AnthropicMsg {
    role: String,
    content: String,
}

#[derive(serde::Serialize)]
struct AnthropicTool {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    input_schema: serde_json::Value,
}

// ── Wire types (Anthropic → deserialise) ─────────────────────────────────────

#[derive(serde::Deserialize)]
struct AnthropicResp {
    id: String,
    content: Vec<AnthropicBlock>,
    stop_reason: Option<String>,
    usage: AnthropicUsage,
}

#[derive(serde::Deserialize)]
struct AnthropicBlock {
    #[serde(rename = "type")]
    ty: String,
    text: Option<String>,
    id: Option<String>,
    name: Option<String>,
    input: Option<serde_json::Value>,
}

#[derive(serde::Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
    #[serde(default)]
    cache_creation_input_tokens: u32,
    #[serde(default)]
    cache_read_input_tokens: u32,
}

// ── Incremental SSE event dispatcher ─────────────────────────────────────────

/// Dispatch one completed Anthropic SSE event into zero or more [`StreamEvent`]s.
///
/// `input_tokens`, `cache_creation_tokens`, and `cache_read_tokens` are mutable
/// state carried across events: populated from `message_start`, forwarded in the
/// `Done` event emitted from `message_delta`.
///
/// `current_tool` tracks the `id` and `name` from the most recent
/// `content_block_start` event with `type="tool_use"`. This state is needed
/// because `input_json_delta` events (which carry the argument JSON fragments)
/// arrive in subsequent events with no id/name of their own.
pub(crate) fn dispatch_anthropic_event(
    ev_type: Option<String>,
    data: Option<String>,
    input_tokens: &mut u32,
    cache_creation_tokens: &mut u32,
    cache_read_tokens: &mut u32,
    current_tool: &mut Option<(String, String)>,
) -> Vec<Result<StreamEvent, ProviderError>> {
    let mut out = Vec::new();
    let (Some(ev_type), Some(data)) = (ev_type, data) else {
        return out;
    };

    match ev_type.as_str() {
        "message_start" => {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&data) {
                let usage = &val["message"]["usage"];
                *input_tokens = usage["input_tokens"].as_u64().unwrap_or(0) as u32;
                *cache_creation_tokens =
                    usage["cache_creation_input_tokens"].as_u64().unwrap_or(0) as u32;
                *cache_read_tokens = usage["cache_read_input_tokens"].as_u64().unwrap_or(0) as u32;
            }
        }
        "content_block_start" => {
            // Capture id + name for tool_use blocks so that subsequent
            // input_json_delta events can carry them. Reset for non-tool blocks.
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&data) {
                let cb = &val["content_block"];
                if cb["type"].as_str() == Some("tool_use") {
                    let id = cb["id"].as_str().unwrap_or("").to_string();
                    let name = cb["name"].as_str().unwrap_or("").to_string();
                    *current_tool = Some((id, name));
                } else {
                    *current_tool = None;
                }
            }
        }
        "content_block_delta" => match serde_json::from_str::<serde_json::Value>(&data) {
            Err(e) => out.push(Err(ProviderError::Malformed(e.to_string()))),
            Ok(val) => {
                let delta = &val["delta"];
                match delta["type"].as_str().unwrap_or("") {
                    "text_delta" => {
                        if let Some(text) = delta["text"].as_str() {
                            out.push(Ok(StreamEvent::Token(text.to_string())));
                        }
                    }
                    "input_json_delta" => {
                        let args_delta = delta["partial_json"].as_str().unwrap_or("").to_string();
                        let (id, name) = current_tool
                            .as_ref()
                            .map(|(i, n)| (i.clone(), n.clone()))
                            .unwrap_or_default();
                        out.push(Ok(StreamEvent::ToolCallDelta {
                            id,
                            name,
                            arguments_delta: args_delta,
                        }));
                    }
                    _ => {}
                }
            }
        },
        "message_delta" => match serde_json::from_str::<serde_json::Value>(&data) {
            Err(e) => out.push(Err(ProviderError::Malformed(e.to_string()))),
            Ok(val) => {
                let stop_reason = val["delta"]["stop_reason"].as_str().map(str::to_string);
                let output_tokens = val["usage"]["output_tokens"].as_u64().unwrap_or(0) as u32;
                out.push(Ok(StreamEvent::Done {
                    stop_reason,
                    usage: Some(Usage {
                        input_tokens: *input_tokens,
                        output_tokens,
                        cache_creation_tokens: *cache_creation_tokens,
                        cache_read_tokens: *cache_read_tokens,
                    }),
                }));
            }
        },
        // content_block_stop, message_stop, ping — no events emitted
        _ => {}
    }
    out
}

// ── AnthropicAdapter ──────────────────────────────────────────────────────────

/// Anthropic Messages API adapter.
///
/// Cloning is cheap — `reqwest::Client` holds an `Arc` internally.
#[derive(Clone)]
pub struct AnthropicAdapter {
    pub(crate) http: reqwest::Client,
    pub(crate) api_key: String,
    pub(crate) base_url: String,
}

impl std::fmt::Debug for AnthropicAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnthropicAdapter")
            .field("base_url", &self.base_url)
            .field("api_key", &"***")
            .finish()
    }
}

impl AnthropicAdapter {
    /// Build an adapter. `base_url` defaults to `https://api.anthropic.com`.
    pub fn new(
        api_key: impl Into<String>,
        base_url: Option<String>,
        timeout_secs: u64,
    ) -> Result<Self, ProviderError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "anthropic-version",
            HeaderValue::from_static(ANTHROPIC_API_VERSION),
        );
        let http = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(Duration::from_secs(timeout_secs))
            .default_headers(headers)
            .build()
            .map_err(ProviderError::Http)?;
        Ok(Self {
            http,
            api_key: api_key.into(),
            base_url: base_url.unwrap_or_else(|| "https://api.anthropic.com".to_string()),
        })
    }

    fn build_request(&self, req: &CompletionRequest, stream: bool) -> AnthropicReq {
        let messages: Vec<AnthropicMsg> = req
            .messages
            .iter()
            .filter(|m| m.role != ChatRole::System)
            .map(|m| AnthropicMsg {
                role: match m.role {
                    ChatRole::User => "user".to_string(),
                    ChatRole::Assistant => "assistant".to_string(),
                    ChatRole::System => "user".to_string(), // filtered above
                },
                content: m.content.clone(),
            })
            .collect();

        // System prompt: top-level `system` field, else first System message.
        // When cache_system_prompt is set, emit as a content-block array with
        // cache_control so Anthropic knows to cache at this prefix boundary.
        // Otherwise emit as a plain JSON string (existing behavior).
        let system = req
            .system
            .clone()
            .or_else(|| {
                req.messages
                    .iter()
                    .find(|m| m.role == ChatRole::System)
                    .map(|m| m.content.clone())
            })
            .map(|text| {
                if req.cache_system_prompt {
                    serde_json::json!([{
                        "type": "text",
                        "text": text,
                        "cache_control": {"type": "ephemeral"}
                    }])
                } else {
                    serde_json::Value::String(text)
                }
            });

        let tools = req.tools.as_ref().map(|ts| {
            ts.iter()
                .map(|t| AnthropicTool {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    input_schema: t.parameters.clone(),
                })
                .collect()
        });

        // Merge json_schema and reasoning_effort into a single output_config block.
        let output_config = {
            let format = req
                .json_schema
                .as_ref()
                .map(|schema| serde_json::json!({"type": "json_schema", "schema": schema}));
            let effort = req.reasoning_effort.map(|e| match e {
                ReasoningEffort::Low => "low",
                ReasoningEffort::Medium => "medium",
                ReasoningEffort::High => "high",
                ReasoningEffort::XHigh => "xhigh",
                ReasoningEffort::Max => "max",
            });
            if format.is_none() && effort.is_none() {
                None
            } else {
                Some(AnthropicOutputConfig { format, effort })
            }
        };

        let thinking = req
            .adaptive_thinking
            .then_some(AnthropicThinking { ty: "adaptive" });

        AnthropicReq {
            model: req.model.clone(),
            max_tokens: req.max_tokens,
            system,
            messages,
            tools,
            temperature: req.temperature,
            top_p: req.top_p,
            stream,
            output_config,
            thinking,
            stop_sequences: req.stop_sequences.clone(),
        }
    }

    fn map_error(status: u16, body: &str) -> ProviderError {
        match status {
            401 | 403 => ProviderError::Auth,
            429 | 529 => ProviderError::RateLimited {
                retry_after_secs: None,
            },
            400 if body.contains("context_length") || body.contains("too long") => {
                ProviderError::ContextOverflow
            }
            500..=599 => ProviderError::ProviderUnavailable,
            _ => ProviderError::Malformed(format!("HTTP {status}")),
        }
    }
}

#[async_trait::async_trait]
impl ProviderAdapter for AnthropicAdapter {
    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, ProviderError> {
        let wire = self.build_request(&req, false);
        let url = format!("{}/v1/messages", self.base_url);

        let resp = self
            .http
            .post(&url)
            .header("x-api-key", &self.api_key)
            .json(&wire)
            .send()
            .await
            .map_err(ProviderError::Http)?;

        let status = resp.status().as_u16();

        // Parse retry-after before consuming body
        let retry_after_secs = if status == 429 || status == 529 {
            resp.headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
        } else {
            None
        };

        if !resp.status().is_success() {
            if status == 429 || status == 529 {
                return Err(ProviderError::RateLimited { retry_after_secs });
            }
            let body = resp.text().await.unwrap_or_default();
            return Err(Self::map_error(status, &body));
        }

        let parsed: AnthropicResp = resp
            .json()
            .await
            .map_err(|e| ProviderError::Malformed(e.to_string()))?;

        let content = parsed
            .content
            .iter()
            .filter(|b| b.ty == "text")
            .filter_map(|b| b.text.as_deref())
            .collect::<Vec<_>>()
            .join("");

        let tool_calls = parsed
            .content
            .iter()
            .filter(|b| b.ty == "tool_use")
            .filter_map(|b| {
                Some(ToolCall {
                    id: b.id.clone()?,
                    name: b.name.clone()?,
                    arguments: b.input.clone().unwrap_or(serde_json::Value::Null),
                })
            })
            .collect();

        Ok(CompletionResponse {
            id: parsed.id,
            content,
            tool_calls,
            stop_reason: parsed.stop_reason,
            usage: Usage {
                input_tokens: parsed.usage.input_tokens,
                output_tokens: parsed.usage.output_tokens,
                cache_creation_tokens: parsed.usage.cache_creation_input_tokens,
                cache_read_tokens: parsed.usage.cache_read_input_tokens,
            },
        })
    }

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
    > {
        let wire = self.build_request(&req, true);
        let url = format!("{}/v1/messages", self.base_url);

        let resp = self
            .http
            .post(&url)
            .header("x-api-key", &self.api_key)
            .json(&wire)
            .send()
            .await
            .map_err(ProviderError::Http)?;

        let status = resp.status().as_u16();
        if !resp.status().is_success() {
            // Read Retry-After before consuming the body for 429/529.
            if status == 429 || status == 529 {
                let retry_after_secs = resp
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok());
                return Err(ProviderError::RateLimited { retry_after_secs });
            }
            let body = resp.text().await.unwrap_or_default();
            return Err(Self::map_error(status, &body));
        }

        // Switch to the byte stream for genuinely incremental SSE parsing.
        // `bytes_stream()` yields `Result<Bytes, reqwest::Error>` chunks.
        let byte_stream = resp.bytes_stream();

        Ok(Box::pin(async_stream::stream! {
            tokio::pin!(byte_stream);
            // Byte-level line buffer — accumulates bytes across chunk boundaries,
            // including mid-line, mid-event, and mid-UTF-8-codepoint splits.
            let mut buf: Vec<u8> = Vec::new();
            // SSE field accumulator (event: + data:) between blank-line boundaries.
            let mut acc = SseAcc::new();
            // Token counts from message_start, forwarded to message_delta Done event.
            let mut input_tokens: u32 = 0;
            let mut cache_creation_tokens: u32 = 0;
            let mut cache_read_tokens: u32 = 0;
            // id + name from the most recent content_block_start(tool_use), used
            // by subsequent input_json_delta events to identify the tool call.
            let mut current_tool: Option<(String, String)> = None;

            loop {
                // Poll the next chunk without requiring a StreamExt import.
                // `poll_fn` + `futures_core::Stream::poll_next` is stable Rust.
                let item = std::future::poll_fn(|cx| {
                    futures_core::Stream::poll_next(byte_stream.as_mut(), cx)
                }).await;

                match item {
                    None => {
                        // Clean EOF from server. Flush any remaining bytes as a
                        // final (unterminated) line — handles providers that omit
                        // the trailing blank line.
                        if !buf.is_empty() {
                            let line = std::mem::take(&mut buf);
                            if acc.feed(&line) {
                                let (ev, data) = acc.take();
                                for event in dispatch_anthropic_event(ev, data, &mut input_tokens, &mut cache_creation_tokens, &mut cache_read_tokens, &mut current_tool) {
                                    yield event;
                                }
                            }
                        }
                        break;
                    }
                    Some(Err(e)) => {
                        // Transport error mid-stream surfaces as an Err item so
                        // the consumer knows the stream was cut short.
                        yield Err(ProviderError::Http(e));
                        return;
                    }
                    Some(Ok(chunk)) => {
                        buf.extend_from_slice(&chunk);
                        // Extract every newline-terminated line from buf; partial
                        // lines remain buffered until the next chunk arrives.
                        let lines = drain_lines(&mut buf);
                        for line in lines {
                            if acc.feed(&line) {
                                // Blank line: dispatch the accumulated SSE event.
                                let (ev, data) = acc.take();
                                for event in dispatch_anthropic_event(ev, data, &mut input_tokens, &mut cache_creation_tokens, &mut cache_read_tokens, &mut current_tool) {
                                    yield event;
                                }
                            }
                        }
                    }
                }
            }
        }))
    }

    async fn embed(&self, _req: EmbedRequest) -> Result<EmbedResponse, ProviderError> {
        Err(ProviderError::NotSupported(
            "Anthropic does not provide an embeddings API",
        ))
    }
}

// ── SSE parser (batch convenience — test helper only) ────────────────────────

/// Parse a complete Anthropic SSE response body into a vec of [`StreamEvent`]s.
///
/// Internally uses the same [`drain_lines`] + [`SseAcc`] + [`dispatch_anthropic_event`]
/// helpers as `complete_stream`, so the two code paths are identical by construction.
#[cfg(test)]
pub(crate) fn parse_anthropic_sse(body: String) -> Vec<Result<StreamEvent, ProviderError>> {
    let mut events = Vec::new();
    let mut buf: Vec<u8> = body.into_bytes();
    let mut acc = SseAcc::new();
    let mut input_tokens: u32 = 0;
    let mut cache_creation_tokens: u32 = 0;
    let mut cache_read_tokens: u32 = 0;
    let mut current_tool: Option<(String, String)> = None;

    let lines = drain_lines(&mut buf);
    for line in lines {
        if acc.feed(&line) {
            let (ev, data) = acc.take();
            events.extend(dispatch_anthropic_event(
                ev,
                data,
                &mut input_tokens,
                &mut cache_creation_tokens,
                &mut cache_read_tokens,
                &mut current_tool,
            ));
        }
    }
    // Handle any remaining bytes (body without trailing blank line)
    if !buf.is_empty() {
        if acc.feed(&buf) {
            let (ev, data) = acc.take();
            events.extend(dispatch_anthropic_event(
                ev,
                data,
                &mut input_tokens,
                &mut cache_creation_tokens,
                &mut cache_read_tokens,
                &mut current_tool,
            ));
        }
    }
    events
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::adapter::{ChatMessage, ChatRole, CompletionRequest, ToolDefinition};

    fn sample_request() -> CompletionRequest {
        CompletionRequest {
            model: "claude-opus-4-8".to_string(),
            messages: vec![ChatMessage {
                role: ChatRole::User,
                content: "Hello".to_string(),
            }],
            system: Some("You are helpful.".to_string()),
            tools: None,
            temperature: None,
            top_p: None,
            max_tokens: 512,
            ..Default::default()
        }
    }

    #[test]
    fn anthropic_complete_request_serializes_correctly() {
        let adapter = AnthropicAdapter {
            http: reqwest::Client::new(),
            api_key: "sk-test".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
        };
        let req = sample_request();
        let wire = adapter.build_request(&req, false);
        let json = serde_json::to_string(&wire).unwrap();

        assert!(
            json.contains("\"model\":\"claude-opus-4-8\""),
            "model field"
        );
        assert!(json.contains("\"max_tokens\":512"), "max_tokens field");
        assert!(
            json.contains("\"system\":\"You are helpful.\""),
            "system field"
        );
        assert!(json.contains("\"role\":\"user\""), "role field");
        assert!(json.contains("\"content\":\"Hello\""), "content field");
        assert!(
            !json.contains("\"stream\""),
            "stream should be absent when false"
        );
    }

    #[test]
    fn anthropic_complete_request_with_tool_serializes() {
        let adapter = AnthropicAdapter {
            http: reqwest::Client::new(),
            api_key: "sk-test".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
        };
        let mut req = sample_request();
        req.tools = Some(vec![ToolDefinition {
            name: "get_weather".to_string(),
            description: Some("Get weather".to_string()),
            parameters: serde_json::json!({"type": "object", "properties": {}}),
        }]);
        let wire = adapter.build_request(&req, false);
        let json = serde_json::to_string(&wire).unwrap();
        assert!(json.contains("\"get_weather\""), "tool name");
        assert!(json.contains("\"input_schema\""), "input_schema key");
    }

    #[test]
    fn anthropic_sse_parsing() {
        let sse = "\
event: message_start\n\
data: {\"type\":\"message_start\",\"message\":{\"id\":\"msg_01\",\"usage\":{\"input_tokens\":10}}}\n\
\n\
event: content_block_start\n\
data: {\"type\":\"content_block_start\",\"index\":0,\"content_block\":{\"type\":\"text\",\"text\":\"\"}}\n\
\n\
event: content_block_delta\n\
data: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"text_delta\",\"text\":\"Hello\"}}\n\
\n\
event: content_block_delta\n\
data: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"text_delta\",\"text\":\" world\"}}\n\
\n\
event: message_delta\n\
data: {\"type\":\"message_delta\",\"delta\":{\"stop_reason\":\"end_turn\"},\"usage\":{\"output_tokens\":5}}\n\
\n\
event: message_stop\n\
data: {\"type\":\"message_stop\"}\n\
\n\
";
        let events = parse_anthropic_sse(sse.to_string());
        // Filter to Ok events
        let ok_events: Vec<&StreamEvent> = events.iter().filter_map(|e| e.as_ref().ok()).collect();

        // Should have: Token("Hello"), Token(" world"), Done { stop_reason: Some("end_turn"), ... }
        assert!(
            ok_events.len() >= 3,
            "expected at least 3 events, got {}",
            ok_events.len()
        );

        assert!(
            matches!(ok_events[0], StreamEvent::Token(t) if t == "Hello"),
            "first event should be Token(Hello)"
        );
        assert!(
            matches!(ok_events[1], StreamEvent::Token(t) if t == " world"),
            "second event should be Token( world)"
        );
        assert!(
            matches!(ok_events[2], StreamEvent::Done { stop_reason: Some(r), .. } if r == "end_turn"),
            "third event should be Done with end_turn"
        );
    }

    #[test]
    fn anthropic_tool_use_response_parses() {
        let json = r#"{
            "id": "msg_tool",
            "type": "message",
            "role": "assistant",
            "content": [
                {
                    "type": "tool_use",
                    "id": "toolu_01",
                    "name": "get_weather",
                    "input": {"location": "London"}
                }
            ],
            "stop_reason": "tool_use",
            "usage": {"input_tokens": 20, "output_tokens": 8}
        }"#;
        let parsed: AnthropicResp = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.content[0].ty, "tool_use");
        assert_eq!(parsed.content[0].id.as_deref(), Some("toolu_01"));
        assert_eq!(parsed.content[0].name.as_deref(), Some("get_weather"));
        let input = parsed.content[0].input.as_ref().unwrap();
        assert_eq!(input["location"], "London");
    }

    // ── incremental parser boundary tests ────────────────────────────────────
    //
    // These exercise `dispatch_anthropic_event` + `drain_lines` + `SseAcc`
    // directly, feeding byte slices split at positions that would break a
    // naive line-at-a-time parser.

    /// Run the same incremental logic used in `complete_stream` over an
    /// arbitrary sequence of byte slices (simulating reqwest chunk delivery).
    fn parse_chunks(chunks: &[&[u8]]) -> Vec<Result<StreamEvent, ProviderError>> {
        let mut events = Vec::new();
        let mut buf: Vec<u8> = Vec::new();
        let mut acc = SseAcc::new();
        let mut input_tokens: u32 = 0;
        let mut cache_creation_tokens: u32 = 0;
        let mut cache_read_tokens: u32 = 0;
        let mut current_tool: Option<(String, String)> = None;

        for chunk in chunks {
            buf.extend_from_slice(chunk);
            let lines = drain_lines(&mut buf);
            for line in lines {
                if acc.feed(&line) {
                    let (ev, data) = acc.take();
                    events.extend(dispatch_anthropic_event(
                        ev,
                        data,
                        &mut input_tokens,
                        &mut cache_creation_tokens,
                        &mut cache_read_tokens,
                        &mut current_tool,
                    ));
                }
            }
        }
        // Flush remaining (no trailing newline in last chunk)
        if !buf.is_empty() {
            if acc.feed(&buf) {
                let (ev, data) = acc.take();
                events.extend(dispatch_anthropic_event(
                    ev,
                    data,
                    &mut input_tokens,
                    &mut cache_creation_tokens,
                    &mut cache_read_tokens,
                    &mut current_tool,
                ));
            }
        }
        events
    }

    #[test]
    fn incremental_mid_line_split() {
        // SSE event bytes split mid-way through the "event:" line — the first
        // chunk doesn't contain a complete line, so no event should fire yet.
        let full: &[u8] = b"\
event: content_block_delta\n\
data: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"text_delta\",\"text\":\"Hi\"}}\n\
\n";
        // Split at byte 12 — inside "event: content_block_delta"
        let (a, b) = full.split_at(12);
        let events = parse_chunks(&[a, b]);
        assert_eq!(events.len(), 1, "expected exactly 1 event");
        assert!(
            matches!(&events[0], Ok(StreamEvent::Token(t)) if t == "Hi"),
            "expected Token(Hi), got {:?}",
            events[0]
        );
    }

    #[test]
    fn incremental_mid_event_split() {
        // First chunk has the "event:" line only; second has the "data:" line
        // and the terminating blank line.  No event should fire after chunk 1.
        let chunk1: &[u8] = b"event: content_block_delta\n";
        let chunk2: &[u8] =
            b"data: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"text_delta\",\"text\":\"World\"}}\n\n";
        let events = parse_chunks(&[chunk1, chunk2]);
        assert_eq!(events.len(), 1, "expected exactly 1 event");
        assert!(
            matches!(&events[0], Ok(StreamEvent::Token(t)) if t == "World"),
            "expected Token(World), got {:?}",
            events[0]
        );
    }

    #[test]
    fn incremental_multi_event_multi_chunk() {
        // Two Token events across three chunks; the blank-line separator of
        // the first event arrives in the middle of chunk 2.
        let chunk1: &[u8] =
            b"event: content_block_delta\ndata: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"text_delta\",\"text\":\"A\"}}";
        // chunk1 ends mid-event (no blank line yet)
        let chunk2: &[u8] = b"\n\nevent: content_block_delta\n";
        // chunk2 completes event 1 (its blank line) and starts event 2
        let chunk3: &[u8] =
            b"data: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"text_delta\",\"text\":\"B\"}}\n\n";
        let events = parse_chunks(&[chunk1, chunk2, chunk3]);
        assert_eq!(events.len(), 2, "expected 2 Token events, got {:?}", events);
        assert!(matches!(&events[0], Ok(StreamEvent::Token(t)) if t == "A"));
        assert!(matches!(&events[1], Ok(StreamEvent::Token(t)) if t == "B"));
    }

    #[test]
    fn incremental_mid_utf8_codepoint_split() {
        // "é" = U+00E9 = 0xC3 0xA9 in UTF-8. Split the raw bytes of the data
        // value between 0xC3 (first byte) and 0xA9 (second byte).
        // 0x0A (\n) cannot appear as a UTF-8 continuation byte, so drain_lines
        // is safe to call on partial multi-byte sequences.
        let prefix: &[u8] =
            b"event: content_block_delta\ndata: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"text_delta\",\"text\":\"Hi\xC3";
        let suffix: &[u8] = b"\xA9\"}}\n\n";
        let events = parse_chunks(&[prefix, suffix]);
        assert_eq!(events.len(), 1, "expected exactly 1 event");
        match &events[0] {
            Ok(StreamEvent::Token(t)) => {
                assert!(t.contains("Hi"), "token should contain 'Hi': {:?}", t);
            }
            other => panic!("expected Token, got {:?}", other),
        }
    }

    #[test]
    fn incremental_done_event_carries_usage() {
        // Verify input_tokens from message_start flows into Done usage in message_delta.
        let chunks: &[&[u8]] = &[
            b"event: message_start\ndata: {\"type\":\"message_start\",\"message\":{\"id\":\"m1\",\"usage\":{\"input_tokens\":42}}}\n\n",
            b"event: message_delta\ndata: {\"type\":\"message_delta\",\"delta\":{\"stop_reason\":\"end_turn\"},\"usage\":{\"output_tokens\":7}}\n\n",
        ];
        let events = parse_chunks(chunks);
        assert_eq!(events.len(), 1, "expected exactly 1 Done event");
        match &events[0] {
            Ok(StreamEvent::Done { stop_reason, usage }) => {
                assert_eq!(stop_reason.as_deref(), Some("end_turn"));
                let u = usage.expect("usage should be present");
                assert_eq!(u.input_tokens, 42);
                assert_eq!(u.output_tokens, 7);
            }
            other => panic!("expected Done, got {:?}", other),
        }
    }

    #[test]
    fn tool_call_stream_carries_id_and_name() {
        // Verify that ToolCallDelta events carry the id and name from the
        // preceding content_block_start(tool_use) event.
        let chunks: &[&[u8]] = &[
            b"event: message_start\ndata: {\"type\":\"message_start\",\"message\":{\"id\":\"m1\",\"usage\":{\"input_tokens\":10}}}\n\n",
            b"event: content_block_start\ndata: {\"type\":\"content_block_start\",\"index\":0,\"content_block\":{\"type\":\"tool_use\",\"id\":\"toolu_abc\",\"name\":\"get_weather\",\"input\":{}}}\n\n",
            b"event: content_block_delta\ndata: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"input_json_delta\",\"partial_json\":\"{\\\"loc\"}}\n\n",
            b"event: content_block_delta\ndata: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"input_json_delta\",\"partial_json\":\"ation\\\":\\\"London\\\"}\"}}\n\n",
            b"event: message_delta\ndata: {\"type\":\"message_delta\",\"delta\":{\"stop_reason\":\"tool_use\"},\"usage\":{\"output_tokens\":5}}\n\n",
        ];
        let events = parse_chunks(chunks);

        // Collect ToolCallDelta events
        let deltas: Vec<&StreamEvent> = events
            .iter()
            .filter_map(|e| e.as_ref().ok())
            .filter(|e| matches!(e, StreamEvent::ToolCallDelta { .. }))
            .collect();

        assert!(!deltas.is_empty(), "expected at least one ToolCallDelta");
        for delta in &deltas {
            match delta {
                StreamEvent::ToolCallDelta { id, name, .. } => {
                    assert_eq!(
                        id, "toolu_abc",
                        "id must be populated from content_block_start"
                    );
                    assert_eq!(
                        name, "get_weather",
                        "name must be populated from content_block_start"
                    );
                }
                _ => panic!("expected ToolCallDelta"),
            }
        }

        // Last event should be Done with tool_use stop reason
        let done = events
            .iter()
            .filter_map(|e| e.as_ref().ok())
            .last()
            .unwrap();
        assert!(
            matches!(done, StreamEvent::Done { stop_reason: Some(r), .. } if r == "tool_use"),
            "last event should be Done(tool_use), got {:?}",
            done
        );
    }

    // ── New capability wire-mapping tests ────────────────────────────────────

    #[test]
    fn cache_system_prompt_emits_cache_control_array() {
        let adapter = AnthropicAdapter {
            http: reqwest::Client::new(),
            api_key: "sk-test".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
        };
        let req = CompletionRequest {
            system: Some("Be helpful.".to_string()),
            cache_system_prompt: true,
            ..sample_request()
        };
        let wire = adapter.build_request(&req, false);
        let json = serde_json::to_string(&wire).unwrap();
        // system must be an array with cache_control, not a plain string
        assert!(
            json.contains("\"cache_control\""),
            "cache_control must be present: {json}"
        );
        assert!(
            json.contains("\"ephemeral\""),
            "cache_control type must be ephemeral: {json}"
        );
        assert!(
            json.contains("\"type\":\"text\""),
            "content block type must be text: {json}"
        );
        // Must NOT be the plain-string form
        assert!(
            !json.contains("\"system\":\"Be helpful.\""),
            "system must not be a plain string when caching: {json}"
        );
    }

    #[test]
    fn no_cache_system_prompt_emits_plain_string() {
        let adapter = AnthropicAdapter {
            http: reqwest::Client::new(),
            api_key: "sk-test".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
        };
        let req = sample_request(); // cache_system_prompt defaults to false
        let wire = adapter.build_request(&req, false);
        let json = serde_json::to_string(&wire).unwrap();
        assert!(
            json.contains("\"system\":\"You are helpful.\""),
            "system must be a plain string when not caching: {json}"
        );
        assert!(
            !json.contains("cache_control"),
            "cache_control must be absent: {json}"
        );
    }

    #[test]
    fn json_schema_serializes_output_config_format() {
        let adapter = AnthropicAdapter {
            http: reqwest::Client::new(),
            api_key: "sk-test".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
        };
        let schema = serde_json::json!({
            "type": "object",
            "properties": {"name": {"type": "string"}}
        });
        let req = CompletionRequest {
            json_schema: Some(schema),
            ..sample_request()
        };
        let wire = adapter.build_request(&req, false);
        let json = serde_json::to_string(&wire).unwrap();
        assert!(
            json.contains("\"output_config\""),
            "output_config must be present: {json}"
        );
        assert!(
            json.contains("\"json_schema\""),
            "format.type must be json_schema: {json}"
        );
    }

    #[test]
    fn reasoning_effort_serializes_into_output_config_effort() {
        let adapter = AnthropicAdapter {
            http: reqwest::Client::new(),
            api_key: "sk-test".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
        };
        use crate::llm::adapter::ReasoningEffort;
        let req = CompletionRequest {
            reasoning_effort: Some(ReasoningEffort::High),
            ..sample_request()
        };
        let wire = adapter.build_request(&req, false);
        let json = serde_json::to_string(&wire).unwrap();
        assert!(
            json.contains("\"effort\":\"high\""),
            "effort must be 'high': {json}"
        );
    }

    #[test]
    fn reasoning_effort_xhigh_serializes_to_xhigh() {
        let adapter = AnthropicAdapter {
            http: reqwest::Client::new(),
            api_key: "sk-test".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
        };
        use crate::llm::adapter::ReasoningEffort;
        let req = CompletionRequest {
            reasoning_effort: Some(ReasoningEffort::XHigh),
            ..sample_request()
        };
        let wire = adapter.build_request(&req, false);
        let json = serde_json::to_string(&wire).unwrap();
        assert!(
            json.contains("\"effort\":\"xhigh\""),
            "effort must be 'xhigh': {json}"
        );
    }

    #[test]
    fn reasoning_effort_max_serializes_to_max() {
        let adapter = AnthropicAdapter {
            http: reqwest::Client::new(),
            api_key: "sk-test".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
        };
        use crate::llm::adapter::ReasoningEffort;
        let req = CompletionRequest {
            reasoning_effort: Some(ReasoningEffort::Max),
            ..sample_request()
        };
        let wire = adapter.build_request(&req, false);
        let json = serde_json::to_string(&wire).unwrap();
        assert!(
            json.contains("\"effort\":\"max\""),
            "effort must be 'max': {json}"
        );
    }

    #[test]
    fn adaptive_thinking_serializes_thinking_field() {
        let adapter = AnthropicAdapter {
            http: reqwest::Client::new(),
            api_key: "sk-test".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
        };
        let req = CompletionRequest {
            adaptive_thinking: true,
            ..sample_request()
        };
        let wire = adapter.build_request(&req, false);
        let json = serde_json::to_string(&wire).unwrap();
        assert!(
            json.contains("\"thinking\""),
            "thinking field must be present: {json}"
        );
        assert!(
            json.contains("\"adaptive\""),
            "thinking.type must be adaptive: {json}"
        );
    }

    #[test]
    fn stop_sequences_serializes() {
        let adapter = AnthropicAdapter {
            http: reqwest::Client::new(),
            api_key: "sk-test".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
        };
        let req = CompletionRequest {
            stop_sequences: Some(vec!["<END>".to_string(), "STOP".to_string()]),
            ..sample_request()
        };
        let wire = adapter.build_request(&req, false);
        let json = serde_json::to_string(&wire).unwrap();
        assert!(
            json.contains("\"stop_sequences\""),
            "stop_sequences must be present: {json}"
        );
        assert!(
            json.contains("\"<END>\""),
            "first stop sequence must appear"
        );
    }

    #[test]
    fn cache_tokens_populated_in_done_event() {
        // Verify that cache_creation_input_tokens and cache_read_input_tokens from
        // the message_start event flow through to the Done event's Usage.
        let chunks: &[&[u8]] = &[
            b"event: message_start\ndata: {\"type\":\"message_start\",\"message\":{\"id\":\"m1\",\"usage\":{\"input_tokens\":10,\"cache_creation_input_tokens\":500,\"cache_read_input_tokens\":200}}}\n\n",
            b"event: message_delta\ndata: {\"type\":\"message_delta\",\"delta\":{\"stop_reason\":\"end_turn\"},\"usage\":{\"output_tokens\":5}}\n\n",
        ];
        let events = parse_chunks(chunks);
        assert_eq!(events.len(), 1, "expected exactly 1 Done event");
        match &events[0] {
            Ok(StreamEvent::Done { usage, .. }) => {
                let u = usage.expect("usage must be present");
                assert_eq!(u.input_tokens, 10);
                assert_eq!(u.cache_creation_tokens, 500);
                assert_eq!(u.cache_read_tokens, 200);
                assert_eq!(u.output_tokens, 5);
            }
            other => panic!("expected Done, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn anthropic_stream_429_retry_after_populated() {
        // Verify complete_stream reads the Retry-After header on 429 and returns
        // RateLimited { retry_after_secs: Some(42) } rather than None.
        use wiremock::{
            matchers::{method, path},
            Mock, MockServer, ResponseTemplate,
        };

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/messages"))
            .respond_with(
                ResponseTemplate::new(429)
                    .append_header("retry-after", "42")
                    .set_body_string("{\"error\":{\"type\":\"rate_limit_error\"}}"),
            )
            .mount(&server)
            .await;

        let adapter = AnthropicAdapter::new("sk-test", Some(server.uri()), 5).unwrap();
        let req = sample_request();
        let result = adapter.complete_stream(req).await;
        assert!(
            matches!(
                result,
                Err(ProviderError::RateLimited {
                    retry_after_secs: Some(42)
                })
            ),
            "expected RateLimited with retry_after_secs=42, got {:?}",
            result.map(|_| "Ok(stream)")
        );
    }
}
