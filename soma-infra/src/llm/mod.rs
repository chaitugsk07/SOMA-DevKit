//! Anthropic LLM HTTP client for soma-platform services.
//!
//! Pure plumbing: configure and construct, then hand the client to the caller.
//! No prompt construction, no agent loop, no tool dispatch, no retry/backoff,
//! no streaming, no token-budget enforcement — those belong in the service that
//! knows what it is building.
//!
//! Build a client from an explicit config with [`LlmClient::new`] or pull it
//! straight from the environment with [`LlmClient::from_env`]. Then call
//! [`LlmClient::messages`] with a fully-formed [`MessagesRequest`].
//!
//! # Rate limiting
//!
//! On HTTP 429 or 529, `messages()` returns [`LlmError::RateLimited`]
//! immediately — it does NOT sleep or retry. The caller owns the retry loop and
//! can read `retry_after_secs` from the error to schedule its own backoff.
//!
//! # Streaming
//!
//! Not implemented — deferred. ponytail follow-up: add a `messages_stream`
//! method returning a `Stream<Item = …>` once the calling services have a
//! concrete need for incremental responses.

pub mod adapter;
pub mod anthropic;
pub mod openai;
pub mod router;
pub mod tokens;

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use tracing::Instrument as _;

/// Tunables for the Anthropic HTTP client. Build from explicit values with
/// [`LlmConfig::new`] or from the environment with [`LlmConfig::from_env`].
#[derive(Debug, Clone)]
pub struct LlmConfig {
    /// Anthropic API key (bearer credential for `x-api-key` header).
    pub api_key: String,
    /// Model identifier, e.g. `"claude-opus-4-8"`, `"claude-sonnet-4-6"`,
    /// `"claude-haiku-4-5"`. No date suffix — bare string only.
    pub model: String,
    /// Per-request HTTP timeout in seconds.
    pub timeout_secs: u64,
    /// Maximum retries the caller's own retry loop may wish to perform.
    /// Stored here as a config knob; `messages()` does NOT use it.
    pub max_retries: u32,
}

impl LlmConfig {
    /// A config with sane defaults for the given key and model.
    ///
    /// Defaults: `timeout_secs = 30`, `max_retries = 3`.
    // ponytail: 30 s covers typical LLM latency; raise per workload if needed.
    // max_retries = 3 is a conventional starting point; the caller's retry loop
    // reads this, messages() itself never retries.
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: model.into(),
            timeout_secs: 30,
            max_retries: 3,
        }
    }

    /// Read `ANTHROPIC_API_KEY` and `ANTHROPIC_MODEL` (both required) plus
    /// optional `ANTHROPIC_TIMEOUT_SECS` and `ANTHROPIC_MAX_RETRIES`.
    ///
    /// Both `ANTHROPIC_API_KEY` and `ANTHROPIC_MODEL` are required — there is
    /// no default model. This is intentional: silently falling back to a
    /// different (potentially expensive) model tier would be a billing hazard.
    pub fn from_env() -> Result<Self, LlmError> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| LlmError::MissingEnv("ANTHROPIC_API_KEY"))?;
        let model = std::env::var("ANTHROPIC_MODEL")
            .map_err(|_| LlmError::MissingEnv("ANTHROPIC_MODEL"))?;
        let mut cfg = Self::new(api_key, model);
        if let Ok(v) = std::env::var("ANTHROPIC_TIMEOUT_SECS") {
            cfg.timeout_secs = v
                .parse()
                .map_err(|_| LlmError::InvalidEnv("ANTHROPIC_TIMEOUT_SECS"))?;
        }
        if let Ok(v) = std::env::var("ANTHROPIC_MAX_RETRIES") {
            cfg.max_retries = v
                .parse()
                .map_err(|_| LlmError::InvalidEnv("ANTHROPIC_MAX_RETRIES"))?;
        }
        Ok(cfg)
    }
}

/// Errors from building an LLM client or executing a request.
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    /// A required environment variable was missing.
    #[error("required env var {0} is not set")]
    MissingEnv(&'static str),
    /// An environment variable held an unparseable value.
    #[error("env var {0} has an invalid value")]
    InvalidEnv(&'static str),
    /// An error from the underlying HTTP transport.
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    /// Anthropic returned a non-2xx, non-rate-limit response.
    #[error("Anthropic API error {status}: {body}")]
    Api { status: u16, body: String },
    /// Anthropic returned 429 (rate limited) or 529 (overloaded).
    /// The caller's retry loop should back off by `retry_after_secs` if set.
    #[error("rate limited by Anthropic (retry after {retry_after_secs:?}s)")]
    RateLimited { retry_after_secs: Option<u64> },
}

// ── Wire types ────────────────────────────────────────────────────────────────

/// Message role: `"user"` or `"assistant"` in the Anthropic wire format.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// A single turn in a conversation.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// Request body for `POST /v1/messages`.
///
/// `system` and `tools` are omitted from the JSON when `None`
/// (`#[serde(skip_serializing_if = "Option::is_none")]`).
#[derive(serde::Serialize)]
pub struct MessagesRequest {
    pub model: String,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
}

/// Response body from a successful `POST /v1/messages`.
#[derive(serde::Deserialize, Debug)]
pub struct MessagesResponse {
    pub id: String,
    pub content: Vec<ContentBlock>,
    pub stop_reason: Option<String>,
    pub usage: TokenUsage,
}

/// A single block in the `content` array of a [`MessagesResponse`].
///
/// `type` is a reserved keyword in Rust; `r#type` maps to the JSON `"type"`
/// field via serde's raw-identifier support (no explicit rename needed).
#[derive(serde::Deserialize, Debug)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub r#type: String,
    pub text: Option<String>,
}

/// Token usage reported by Anthropic.
#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

// ── LlmClient ─────────────────────────────────────────────────────────────────

const ANTHROPIC_BASE_URL: &str = "https://api.anthropic.com";
const ANTHROPIC_VERSION: &str = "2023-06-01";

/// A thin, cheaply-cloneable Anthropic HTTP client.
///
/// Cloning is cheap — `reqwest::Client` holds an `Arc` internally.
/// The caller owns all prompt construction, retry logic, and token budgeting.
#[derive(Clone)]
pub struct LlmClient {
    http: reqwest::Client,
    api_key: String,
    model: String,
}

impl std::fmt::Debug for LlmClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LlmClient")
            .field("model", &self.model)
            .field("api_key", &"***")
            .finish()
    }
}

impl LlmClient {
    /// Build a client from a [`LlmConfig`].
    ///
    /// Constructs the underlying `reqwest::Client` with the configured timeout.
    pub fn new(cfg: LlmConfig) -> Result<Self, LlmError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "anthropic-version",
            HeaderValue::from_static(ANTHROPIC_VERSION),
        );
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(cfg.timeout_secs))
            .default_headers(headers)
            .build()?;
        Ok(Self {
            http,
            api_key: cfg.api_key,
            model: cfg.model,
        })
    }

    /// Convenience: [`LlmConfig::from_env`] then [`LlmClient::new`].
    pub fn from_env() -> Result<Self, LlmError> {
        Self::new(LlmConfig::from_env()?)
    }

    /// POST `req` to `POST /v1/messages` and return the parsed response.
    ///
    /// Emits a `gen_ai.chat` OTel span (via `tracing`) for every call.
    /// `gen_ai.usage.input_tokens` and `gen_ai.usage.output_tokens` are
    /// recorded on the span after a successful response.
    ///
    /// - 429 / 529 → [`LlmError::RateLimited`] (no retry, no sleep — caller owns it).
    /// - other non-2xx → [`LlmError::Api`] with the status code and body.
    /// - 2xx → deserialized [`MessagesResponse`].
    pub async fn messages(&self, req: &MessagesRequest) -> Result<MessagesResponse, LlmError> {
        let span = tracing::info_span!(
            "chat",
            "gen_ai.provider.name" = "anthropic",
            "gen_ai.operation.name" = "chat",
            "gen_ai.request.model" = %req.model,
            "gen_ai.usage.input_tokens" = tracing::field::Empty,
            "gen_ai.usage.output_tokens" = tracing::field::Empty,
        );
        async {
            let resp = self.messages_inner(req).await?;
            tracing::Span::current().record("gen_ai.usage.input_tokens", resp.usage.input_tokens);
            tracing::Span::current().record("gen_ai.usage.output_tokens", resp.usage.output_tokens);
            Ok(resp)
        }
        .instrument(span)
        .await
    }

    async fn messages_inner(&self, req: &MessagesRequest) -> Result<MessagesResponse, LlmError> {
        let url = format!("{ANTHROPIC_BASE_URL}/v1/messages");
        let response = self
            .http
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("content-type", "application/json")
            .json(req)
            .send()
            .await?;

        let status = response.status();

        if status == 429 || status == 529 {
            // Parse `retry-after` header (seconds as integer) if present.
            let retry_after_secs = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());
            return Err(LlmError::RateLimited { retry_after_secs });
        }

        if !status.is_success() {
            let status_u16 = status.as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(LlmError::Api {
                status: status_u16,
                body,
            });
        }

        let parsed = response.json::<MessagesResponse>().await?;
        Ok(parsed)
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── LlmConfig::new defaults ───────────────────────────────────────────────

    #[test]
    fn new_sets_documented_defaults() {
        let cfg = LlmConfig::new("sk-test", "claude-opus-4-8");
        assert_eq!(cfg.api_key, "sk-test");
        assert_eq!(cfg.model, "claude-opus-4-8");
        assert_eq!(cfg.timeout_secs, 30);
        assert_eq!(cfg.max_retries, 3);
    }

    // ── LlmConfig::from_env paths ─────────────────────────────────────────────

    /// All `from_env` paths exercised sequentially to avoid races on shared
    /// env vars. Same pattern as cache.rs and crypto.rs.
    #[test]
    fn from_env_paths() {
        // — missing ANTHROPIC_API_KEY —
        std::env::remove_var("ANTHROPIC_API_KEY");
        std::env::remove_var("ANTHROPIC_MODEL");
        std::env::remove_var("ANTHROPIC_TIMEOUT_SECS");
        std::env::remove_var("ANTHROPIC_MAX_RETRIES");
        assert!(matches!(
            LlmConfig::from_env().unwrap_err(),
            LlmError::MissingEnv("ANTHROPIC_API_KEY")
        ));

        // — API key present, ANTHROPIC_MODEL unset → MissingEnv("ANTHROPIC_MODEL") —
        std::env::set_var("ANTHROPIC_API_KEY", "sk-ant-test");
        assert!(matches!(
            LlmConfig::from_env().unwrap_err(),
            LlmError::MissingEnv("ANTHROPIC_MODEL")
        ));

        // — both required vars present → Ok with those values —
        std::env::set_var("ANTHROPIC_MODEL", "claude-sonnet-4-6");
        let cfg = LlmConfig::from_env().unwrap();
        assert_eq!(cfg.api_key, "sk-ant-test");
        assert_eq!(cfg.model, "claude-sonnet-4-6");
        assert_eq!(cfg.timeout_secs, 30); // default
        assert_eq!(cfg.max_retries, 3); // default

        // — optional ANTHROPIC_TIMEOUT_SECS present —
        std::env::set_var("ANTHROPIC_TIMEOUT_SECS", "60");
        let cfg = LlmConfig::from_env().unwrap();
        assert_eq!(cfg.timeout_secs, 60);

        // — bad ANTHROPIC_TIMEOUT_SECS → InvalidEnv —
        std::env::set_var("ANTHROPIC_TIMEOUT_SECS", "notanumber");
        assert!(matches!(
            LlmConfig::from_env().unwrap_err(),
            LlmError::InvalidEnv("ANTHROPIC_TIMEOUT_SECS")
        ));
        std::env::remove_var("ANTHROPIC_TIMEOUT_SECS");

        // — bad ANTHROPIC_MAX_RETRIES → InvalidEnv —
        std::env::set_var("ANTHROPIC_MAX_RETRIES", "bad");
        assert!(matches!(
            LlmConfig::from_env().unwrap_err(),
            LlmError::InvalidEnv("ANTHROPIC_MAX_RETRIES")
        ));

        // cleanup
        std::env::remove_var("ANTHROPIC_API_KEY");
        std::env::remove_var("ANTHROPIC_MODEL");
        std::env::remove_var("ANTHROPIC_TIMEOUT_SECS");
        std::env::remove_var("ANTHROPIC_MAX_RETRIES");
    }

    // ── Serde shapes ──────────────────────────────────────────────────────────

    #[test]
    fn messages_request_omits_none_system_and_tools() {
        let req = MessagesRequest {
            model: "claude-opus-4-8".into(),
            max_tokens: 1024,
            system: None,
            messages: vec![Message {
                role: Role::User,
                content: "hello".into(),
            }],
            tools: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(
            !json.contains("\"system\""),
            "system key should be absent when None"
        );
        assert!(
            !json.contains("\"tools\""),
            "tools key should be absent when None"
        );
        assert!(
            json.contains("\"user\""),
            "role should serialise as \"user\""
        );
        assert!(json.contains("\"hello\""), "content should be present");
    }

    #[test]
    fn messages_request_includes_system_and_tools_when_some() {
        let req = MessagesRequest {
            model: "claude-opus-4-8".into(),
            max_tokens: 512,
            system: Some("You are helpful.".into()),
            messages: vec![Message {
                role: Role::Assistant,
                content: "Sure.".into(),
            }],
            tools: Some(serde_json::json!([])),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"system\""));
        assert!(json.contains("\"tools\""));
        assert!(json.contains("\"assistant\""));
    }

    #[test]
    fn messages_response_deserializes_sample() {
        let sample = r#"{
            "id": "msg_01XFDUDYJgAACzvnptvVoYEL",
            "type": "message",
            "role": "assistant",
            "content": [
                {"type": "text", "text": "Hello, world!"}
            ],
            "model": "claude-opus-4-8",
            "stop_reason": "end_turn",
            "stop_sequence": null,
            "usage": {
                "input_tokens": 10,
                "output_tokens": 25
            }
        }"#;
        let resp: MessagesResponse = serde_json::from_str(sample).unwrap();
        assert_eq!(resp.id, "msg_01XFDUDYJgAACzvnptvVoYEL");
        assert_eq!(resp.content.len(), 1);
        assert_eq!(resp.content[0].r#type, "text");
        assert_eq!(resp.content[0].text.as_deref(), Some("Hello, world!"));
        assert_eq!(resp.stop_reason.as_deref(), Some("end_turn"));
        assert_eq!(resp.usage.input_tokens, 10);
        assert_eq!(resp.usage.output_tokens, 25);
    }

    #[test]
    fn messages_response_handles_missing_text_in_content_block() {
        // Tool-use blocks have type="tool_use" and no "text" field.
        let sample = r#"{
            "id": "msg_tool",
            "content": [
                {"type": "tool_use", "id": "toolu_01", "name": "get_weather", "input": {}}
            ],
            "stop_reason": "tool_use",
            "usage": {"input_tokens": 5, "output_tokens": 3}
        }"#;
        let resp: MessagesResponse = serde_json::from_str(sample).unwrap();
        assert_eq!(resp.content[0].r#type, "tool_use");
        assert!(resp.content[0].text.is_none());
    }

    #[allow(dead_code)]
    fn existing_api_compiles() {
        fn _check(_cfg: LlmConfig, _client: LlmClient, req: MessagesRequest) {
            let _ = LlmClient::from_env();
            let _ = LlmClient::new(_cfg);
            let _: &MessagesRequest = &req;
        }
    }
}
