//! WhatsApp Business Cloud API client.
//!
//! Mechanism only: POST text and template messages to the Graph API and surface
//! typed errors.  No credential storage, 24-hour messaging window enforcement,
//! template approval, or retry policy — those are the consuming service's policy.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Configuration for the WhatsApp Business Cloud client.
pub struct WhatsAppConfig {
    /// Graph API access token (system user or app token).
    pub access_token: String,
    /// WhatsApp Business phone number ID.
    pub phone_number_id: String,
    /// Graph API version, e.g. `"v19.0"`. Supplied by the caller so the client
    /// is not pinned to a specific version.
    pub api_version: String,
}

/// Errors from WhatsApp Cloud API operations.
#[derive(Debug, thiserror::Error)]
pub enum WhatsAppError {
    /// `reqwest::Client` construction or network-level error.
    /// Access token is scrubbed from the message.
    #[error("HTTP client error: {0}")]
    HttpClient(String),

    /// The Graph API returned an error envelope.
    #[error("WhatsApp API error {status} (code {code}): {message}")]
    Api {
        status: u16,
        /// Graph error code, e.g. 131026, 190.
        code: i64,
        message: String,
        /// Optional sub-code for more precise classification.
        error_subcode: Option<i64>,
    },

    /// Failed to parse the response body.
    #[error("response parse error: {0}")]
    Parse(String),
}

impl WhatsAppError {
    /// `true` if the error indicates the recipient cannot receive this message
    /// and the request should not be retried as-is.
    ///
    /// Covers codes for non-WhatsApp recipients, re-engagement window violations,
    /// invalid parameters, unsupported message types, and invalid parameter values.
    pub fn is_permanent_recipient(&self) -> bool {
        matches!(
            self,
            // 131026: recipient not on WhatsApp / message undeliverable
            // 131047: outside 24h re-engagement window
            // 100:    invalid parameter
            // 131051: unsupported message type
            // 131009: parameter value is invalid
            Self::Api {
                code: 131026 | 131047 | 100 | 131051 | 131009,
                ..
            }
        )
    }

    /// `true` if the error is an authentication failure (expired or invalid token).
    ///
    /// Covers Graph error code 190 and any HTTP 401 response.
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            Self::Api { code: 190, .. } | Self::Api { status: 401, .. }
        )
    }
}

// ── Internal response types ───────────────────────────────────────────────────

#[derive(Deserialize)]
struct WaMessageId {
    id: String,
}

#[derive(Deserialize)]
struct WaSuccess {
    messages: Vec<WaMessageId>,
}

#[derive(Deserialize)]
struct WaErrorEnvelope {
    error: WaErrorDetail,
}

#[derive(Deserialize)]
struct WaErrorDetail {
    code: i64,
    message: String,
    #[serde(default)]
    error_subcode: Option<i64>,
}

// ── Request body helpers ──────────────────────────────────────────────────────

#[derive(Serialize)]
struct TextPayload<'a> {
    messaging_product: &'static str,
    to: &'a str,
    #[serde(rename = "type")]
    msg_type: &'static str,
    text: TextBody<'a>,
}

#[derive(Serialize)]
struct TextBody<'a> {
    body: &'a str,
}

#[derive(Serialize)]
struct TemplatePayload<'a> {
    messaging_product: &'static str,
    to: &'a str,
    #[serde(rename = "type")]
    msg_type: &'static str,
    template: TemplateBody<'a>,
}

#[derive(Serialize)]
struct TemplateBody<'a> {
    name: &'a str,
    language: TemplateLanguage<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Value>,
}

#[derive(Serialize)]
struct TemplateLanguage<'a> {
    code: &'a str,
}

// ── Client ────────────────────────────────────────────────────────────────────

/// WhatsApp Business Cloud API client. Thread-safe; share via `Arc<WhatsAppClient>`.
pub struct WhatsAppClient {
    http: reqwest::Client,
    config: WhatsAppConfig,
    /// Base URL — overridable in tests via `with_base_url`.
    base_url: String,
}

impl WhatsAppClient {
    /// Build a client pointing at the real Graph API (`https://graph.facebook.com`).
    pub fn new(cfg: WhatsAppConfig) -> Result<Self, WhatsAppError> {
        Self::with_base_url(cfg, "https://graph.facebook.com".into())
    }

    /// Build with an explicit base URL. Used in tests to point at a mock server.
    pub fn with_base_url(cfg: WhatsAppConfig, base_url: String) -> Result<Self, WhatsAppError> {
        let http = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| WhatsAppError::HttpClient(scrub(&e.to_string(), &cfg.access_token)))?;
        Ok(Self {
            http,
            config: cfg,
            base_url,
        })
    }

    /// Send a plain-text message to `to` (E.164 format).
    ///
    /// Returns the WhatsApp message ID on success.
    pub async fn send_text(&self, to: &str, body: &str) -> Result<String, WhatsAppError> {
        self.post_json(&TextPayload {
            messaging_product: "whatsapp",
            to,
            msg_type: "text",
            text: TextBody { body },
        })
        .await
    }

    /// Send a pre-approved template message to `to` (E.164 format).
    ///
    /// `components` is an optional JSON array of template component objects
    /// (headers, bodies, buttons). Returns the WhatsApp message ID on success.
    pub async fn send_template(
        &self,
        to: &str,
        template_name: &str,
        language_code: &str,
        components: Option<Value>,
    ) -> Result<String, WhatsAppError> {
        self.post_json(&TemplatePayload {
            messaging_product: "whatsapp",
            to,
            msg_type: "template",
            template: TemplateBody {
                name: template_name,
                language: TemplateLanguage {
                    code: language_code,
                },
                components,
            },
        })
        .await
    }

    async fn post_json<T: Serialize>(&self, body: &T) -> Result<String, WhatsAppError> {
        let url = format!(
            "{}/{}/{}/messages",
            self.base_url, self.config.api_version, self.config.phone_number_id
        );

        let resp = self
            .http
            .post(&url)
            .bearer_auth(&self.config.access_token)
            .json(body)
            .send()
            .await
            .map_err(|e| {
                WhatsAppError::HttpClient(scrub(&e.to_string(), &self.config.access_token))
            })?;

        let http_status = resp.status().as_u16();

        if resp.status().is_success() {
            let success: WaSuccess = resp
                .json()
                .await
                .map_err(|e| WhatsAppError::Parse(e.to_string()))?;
            return Ok(success
                .messages
                .into_iter()
                .next()
                .map(|m| m.id)
                .unwrap_or_default());
        }

        let envelope: WaErrorEnvelope = resp
            .json()
            .await
            .map_err(|e| WhatsAppError::Parse(e.to_string()))?;

        Err(WhatsAppError::Api {
            status: http_status,
            code: envelope.error.code,
            message: envelope.error.message,
            error_subcode: envelope.error.error_subcode,
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
        matchers::{body_json, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn make_client(server_uri: &str) -> WhatsAppClient {
        WhatsAppClient::with_base_url(
            WhatsAppConfig {
                access_token: "test_token".into(),
                phone_number_id: "123456789".into(),
                api_version: "v19.0".into(),
            },
            server_uri.into(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn whatsapp_send_text_ok() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v19.0/123456789/messages"))
            .and(body_json(serde_json::json!({
                "messaging_product": "whatsapp",
                "to": "+14155552671",
                "type": "text",
                "text": { "body": "Hello!" }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "messaging_product": "whatsapp",
                "contacts": [{ "input": "+14155552671", "wa_id": "14155552671" }],
                "messages": [{ "id": "wamid.abc123" }]
            })))
            .mount(&server)
            .await;

        let c = make_client(&server.uri());
        let msg_id = c.send_text("+14155552671", "Hello!").await.unwrap();
        assert_eq!(msg_id, "wamid.abc123");
    }

    #[tokio::test]
    async fn whatsapp_send_template_ok_with_components() {
        let server = MockServer::start().await;
        let components = serde_json::json!([
            { "type": "body", "parameters": [{ "type": "text", "text": "World" }] }
        ]);
        Mock::given(method("POST"))
            .and(path("/v19.0/123456789/messages"))
            .and(body_json(serde_json::json!({
                "messaging_product": "whatsapp",
                "to": "+14155552671",
                "type": "template",
                "template": {
                    "name": "hello_world",
                    "language": { "code": "en_US" },
                    "components": components
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "messaging_product": "whatsapp",
                "messages": [{ "id": "wamid.tmpl456" }]
            })))
            .mount(&server)
            .await;

        let c = make_client(&server.uri());
        let msg_id = c
            .send_template("+14155552671", "hello_world", "en_US", Some(components))
            .await
            .unwrap();
        assert_eq!(msg_id, "wamid.tmpl456");
    }

    #[tokio::test]
    async fn whatsapp_131026_is_permanent_recipient() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v19.0/123456789/messages"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "error": {
                    "message": "Recipient phone number not in allowed list",
                    "type": "OAuthException",
                    "code": 131026,
                    "error_subcode": 2655007
                }
            })))
            .mount(&server)
            .await;

        let c = make_client(&server.uri());
        let err = c.send_text("+14155552671", "Hi").await.unwrap_err();
        assert!(err.is_permanent_recipient(), "131026 must be permanent");
        assert!(!err.is_auth_error());
    }

    #[tokio::test]
    async fn whatsapp_190_is_auth_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v19.0/123456789/messages"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": {
                    "message": "Invalid OAuth access token",
                    "type": "OAuthException",
                    "code": 190
                }
            })))
            .mount(&server)
            .await;

        let c = make_client(&server.uri());
        let err = c.send_text("+14155552671", "Hi").await.unwrap_err();
        assert!(err.is_auth_error(), "code 190 must be auth error");
        assert!(!err.is_permanent_recipient());
    }

    #[tokio::test]
    async fn whatsapp_500_is_transient() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v19.0/123456789/messages"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "error": {
                    "message": "Internal server error",
                    "type": "OAuthException",
                    "code": 2
                }
            })))
            .mount(&server)
            .await;

        let c = make_client(&server.uri());
        let err = c.send_text("+14155552671", "Hi").await.unwrap_err();
        assert!(!err.is_permanent_recipient(), "500 must not be permanent");
        assert!(!err.is_auth_error(), "500 must not be auth error");
    }

    #[tokio::test]
    async fn whatsapp_token_not_in_http_client_error() {
        // Verify the scrub mechanism: a constructed HttpClient error that would
        // normally contain the access token must not expose the secret in
        // Display or Debug output.
        let secret = "super_secret_access_token_abc";
        let err = WhatsAppError::HttpClient(scrub(
            &format!("connection error: bearer {secret} rejected"),
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
