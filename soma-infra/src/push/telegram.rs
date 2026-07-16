//! Telegram Bot API client.
//!
//! Mechanism only: POST to the Bot API sendMessage endpoint and surface typed
//! errors.  No credential storage, no retry policy — those are the consuming
//! service's policy.

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Configuration for the Telegram Bot client.
pub struct TelegramConfig {
    /// Bot token from BotFather (e.g. `"123456789:AAF..."`).
    pub bot_token: String,
}

/// Errors from Telegram Bot API operations.
#[derive(Debug, thiserror::Error)]
pub enum TelegramError {
    /// `reqwest::Client` construction or network-level error.
    /// Bot token is scrubbed from the message.
    #[error("HTTP client error: {0}")]
    HttpClient(String),

    /// Telegram returned an `ok: false` response.
    #[error("Telegram API error {status}: {description}")]
    Api {
        status: u16,
        description: String,
        /// Present on 429 responses in the `parameters.retry_after` field.
        retry_after: Option<u64>,
    },

    /// Failed to parse the response body.
    #[error("response parse error: {0}")]
    Parse(String),
}

impl TelegramError {
    /// `true` if the error is permanent and should not be retried as-is.
    ///
    /// Covers 400 errors indicating a bad or non-existent chat, and 403 errors
    /// indicating the bot has been blocked or cannot initiate conversation.
    pub fn is_permanent(&self) -> bool {
        match self {
            Self::Api {
                status: 400,
                description,
                ..
            } => {
                let d = description.to_ascii_lowercase();
                d.contains("chat not found")
                    || d.contains("chat_id is empty")
                    || d.contains("wrong type of the web page content")
            }
            Self::Api {
                status: 403,
                description,
                ..
            } => {
                let d = description.to_ascii_lowercase();
                d.contains("bot was blocked")
                    || d.contains("user is deactivated")
                    || d.contains("bot can't initiate conversation")
            }
            _ => false,
        }
    }

    /// Seconds the caller must wait before retrying; present on 429 responses.
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            Self::Api { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}

// ── Internal response types ───────────────────────────────────────────────────

#[derive(Deserialize)]
struct TgResponse {
    ok: bool,
    #[serde(default)]
    result: Option<TgMessage>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    error_code: Option<u16>,
    #[serde(default)]
    parameters: Option<TgParameters>,
}

#[derive(Deserialize)]
struct TgMessage {
    message_id: i64,
}

#[derive(Deserialize)]
struct TgParameters {
    #[serde(default)]
    retry_after: Option<u64>,
}

// ── Request body ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendMessageBody<'a> {
    chat_id: &'a str,
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
}

// ── Client ────────────────────────────────────────────────────────────────────

/// Telegram Bot API client. Thread-safe; share via `Arc<TelegramClient>`.
pub struct TelegramClient {
    http: reqwest::Client,
    config: TelegramConfig,
    /// Base URL — overridable in tests via `with_base_url`.
    base_url: String,
}

impl TelegramClient {
    /// Build a client pointing at the real Telegram Bot API.
    pub fn new(cfg: TelegramConfig) -> Result<Self, TelegramError> {
        Self::with_base_url(cfg, "https://api.telegram.org".into())
    }

    /// Build with an explicit base URL. Used in tests to point at a mock server.
    pub fn with_base_url(cfg: TelegramConfig, base_url: String) -> Result<Self, TelegramError> {
        let http = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| TelegramError::HttpClient(scrub(&e.to_string(), &cfg.bot_token)))?;
        Ok(Self {
            http,
            config: cfg,
            base_url,
        })
    }

    /// Send a text message to `chat_id`.
    ///
    /// Returns the Telegram `message_id` on success. Does not truncate text;
    /// messages exceeding 4096 characters will be rejected by the API.
    pub async fn send_message(
        &self,
        chat_id: &str,
        text: &str,
        parse_mode: Option<&str>,
    ) -> Result<i64, TelegramError> {
        let url = format!("{}/bot{}/sendMessage", self.base_url, self.config.bot_token);
        let body = SendMessageBody {
            chat_id,
            text,
            parse_mode,
        };

        let resp = self.http.post(&url).json(&body).send().await.map_err(|e| {
            TelegramError::HttpClient(scrub(&e.to_string(), &self.config.bot_token))
        })?;

        let http_status = resp.status().as_u16();
        let tg: TgResponse = resp
            .json()
            .await
            .map_err(|e| TelegramError::Parse(e.to_string()))?;

        if tg.ok {
            return Ok(tg.result.map(|m| m.message_id).unwrap_or(0));
        }

        Err(TelegramError::Api {
            status: tg.error_code.unwrap_or(http_status),
            description: tg.description.unwrap_or_default(),
            retry_after: tg.parameters.and_then(|p| p.retry_after),
        })
    }
}

/// Replace all occurrences of `secret` in `s` with `"<redacted>"`.
fn scrub(s: &str, secret: &str) -> String {
    if secret.is_empty() {
        return s.to_string();
    }
    s.replace(secret, "<redacted>")
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn make_client(token: &str, base: &str) -> TelegramClient {
        TelegramClient::with_base_url(
            TelegramConfig {
                bot_token: token.into(),
            },
            base.into(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn telegram_send_message_ok() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/bottest_token/sendMessage"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "ok": true,
                "result": { "message_id": 42 }
            })))
            .mount(&server)
            .await;

        let client = make_client("test_token", &server.uri());
        let msg_id = client
            .send_message("-100123", "Hello!", None)
            .await
            .unwrap();
        assert_eq!(msg_id, 42);
    }

    #[tokio::test]
    async fn telegram_403_bot_blocked_is_permanent() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/bottest_token/sendMessage"))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "ok": false,
                "error_code": 403,
                "description": "Forbidden: bot was blocked by the user"
            })))
            .mount(&server)
            .await;

        let client = make_client("test_token", &server.uri());
        let err = client
            .send_message("-100123", "Hello!", None)
            .await
            .unwrap_err();
        assert!(err.is_permanent(), "bot blocked should be permanent");
        assert_eq!(err.retry_after(), None);
    }

    #[tokio::test]
    async fn telegram_429_retry_after() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/bottest_token/sendMessage"))
            .respond_with(ResponseTemplate::new(429).set_body_json(serde_json::json!({
                "ok": false,
                "error_code": 429,
                "description": "Too Many Requests: retry after 30",
                "parameters": { "retry_after": 30 }
            })))
            .mount(&server)
            .await;

        let client = make_client("test_token", &server.uri());
        let err = client
            .send_message("-100123", "Hello!", None)
            .await
            .unwrap_err();
        assert!(!err.is_permanent(), "rate limit is not permanent");
        assert_eq!(err.retry_after(), Some(30));
    }

    #[tokio::test]
    async fn telegram_token_not_in_http_client_error() {
        // Verify the scrub mechanism: a constructed HttpClient error that would
        // normally contain the bot token (e.g. from a URL in a reqwest error)
        // must not expose the secret in Display or Debug output.
        let secret = "super_secret_bot_token_123";
        let err = TelegramError::HttpClient(scrub(
            &format!("connection to https://api.telegram.org/bot{secret}/sendMessage refused"),
            secret,
        ));
        let display = format!("{err}");
        let debug = format!("{err:?}");
        assert!(
            !display.contains(secret),
            "token must not appear in Display: {display}"
        );
        assert!(
            !debug.contains(secret),
            "token must not appear in Debug: {debug}"
        );
    }
}
