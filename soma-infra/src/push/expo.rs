//! Expo Push API client.
//!
//! Mechanism only: POST a batch to exp.host's push/send endpoint and return
//! typed per-message results.  No device-token lifecycle, no fanout, no retry
//! policy — those are the consuming service's policy.
//!
//! No server-side credentials are required for Expo's basic tier; exp.host
//! handles APNs/FCM delivery transparently.

use std::time::Duration;

use serde::{Deserialize, Serialize};

// ── Request types ─────────────────────────────────────────────────────────────

/// A single push message to include in a batch request to exp.host.
#[derive(Debug, Serialize)]
pub struct ExpoPushMessage {
    /// Expo push token, e.g. `"ExponentPushToken[...]"`.
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Use `Some("default".to_string())` for the device's default notification sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
}

// ── Response types ────────────────────────────────────────────────────────────

/// Per-message delivery result from the Expo push API.
///
/// Returned in the `data` array of the response, one entry per input message
/// in the same order.
#[derive(Debug, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ExpoSendResult {
    Ok {
        id: String,
    },
    Error {
        message: String,
        #[serde(default)]
        details: Option<ExpoErrorDetails>,
    },
}

/// Structured error details returned by Expo for a failed message.
#[derive(Debug, Deserialize)]
pub struct ExpoErrorDetails {
    /// Expo error code string: `"DeviceNotRegistered"`, `"MessageTooBig"`,
    /// `"MessageRateExceeded"`, etc.
    pub error: Option<String>,
}

impl ExpoSendResult {
    /// `true` when Expo reports the device token is no longer registered and
    /// should be pruned.  The caller is responsible for deactivating the device.
    pub fn is_device_not_registered(&self) -> bool {
        matches!(
            self,
            ExpoSendResult::Error {
                details: Some(ExpoErrorDetails { error: Some(e) }),
                ..
            }
            if e == "DeviceNotRegistered"
        )
    }
}

// ── Error type ────────────────────────────────────────────────────────────────

/// Errors from Expo push operations.
#[derive(Debug, thiserror::Error)]
pub enum ExpoError {
    #[error("HTTP client build error: {0}")]
    HttpClient(String),
    #[error("request error: {0}")]
    Request(String),
    #[error("unexpected HTTP status {0}")]
    Status(u16),
    #[error("JSON parse error: {0}")]
    Json(String),
}

// ── Client ────────────────────────────────────────────────────────────────────

/// Expo push client. Clone-cheap (inner `reqwest::Client` is `Arc`-backed).
pub struct ExpoClient {
    http: reqwest::Client,
    /// Base URL — overridable in tests via `with_base_url`.
    base_url: String,
}

impl ExpoClient {
    /// Build a client pointing at the real Expo endpoint (`https://exp.host`).
    pub fn new() -> Result<Self, ExpoError> {
        Self::with_base_url("https://exp.host".to_string())
    }

    /// Build with an explicit base URL.  Used in tests to point at a mock server.
    pub fn with_base_url(base_url: String) -> Result<Self, ExpoError> {
        let http = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| ExpoError::HttpClient(e.to_string()))?;
        Ok(Self { http, base_url })
    }

    /// Send a batch of push messages (up to 100 per Expo recommendation).
    ///
    /// Returns one `ExpoSendResult` per input message in the same order.
    pub async fn send(
        &self,
        messages: &[ExpoPushMessage],
    ) -> Result<Vec<ExpoSendResult>, ExpoError> {
        let url = format!("{}/--/api/v2/push/send", self.base_url);

        let resp = self
            .http
            .post(&url)
            .json(messages)
            .send()
            .await
            .map_err(|e| ExpoError::Request(e.to_string()))?;

        let status = resp.status().as_u16();
        if status != 200 {
            return Err(ExpoError::Status(status));
        }

        #[derive(Deserialize)]
        struct ExpoResponse {
            data: Vec<ExpoSendResult>,
        }

        let body: ExpoResponse = resp
            .json()
            .await
            .map_err(|e| ExpoError::Json(e.to_string()))?;
        Ok(body.data)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn one_message() -> ExpoPushMessage {
        ExpoPushMessage {
            to: "ExponentPushToken[test]".into(),
            title: Some("Hello".into()),
            body: Some("World".into()),
            data: None,
            sound: Some("default".into()),
        }
    }

    #[tokio::test]
    async fn expo_send_ok_returns_ids() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/--/api/v2/push/send"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": [{"status": "ok", "id": "abc123"}]
            })))
            .mount(&server)
            .await;

        let client = ExpoClient::with_base_url(server.uri()).unwrap();
        let results = client.send(&[one_message()]).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(
            matches!(&results[0], ExpoSendResult::Ok { id } if id == "abc123"),
            "expected ok with id=abc123"
        );
    }

    #[tokio::test]
    async fn expo_device_not_registered_detected() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/--/api/v2/push/send"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "data": [{
                        "status":  "error",
                        "message": "The recipient device is no longer registered with the push notification service.",
                        "details": {"error": "DeviceNotRegistered"}
                    }]
                })),
            )
            .mount(&server)
            .await;

        let client = ExpoClient::with_base_url(server.uri()).unwrap();
        let results = client.send(&[one_message()]).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(
            results[0].is_device_not_registered(),
            "must detect DeviceNotRegistered"
        );
    }

    #[tokio::test]
    async fn expo_http_error_returns_err() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/--/api/v2/push/send"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = ExpoClient::with_base_url(server.uri()).unwrap();
        let err = client.send(&[one_message()]).await.unwrap_err();
        assert!(
            matches!(err, ExpoError::Status(500)),
            "expected Status(500), got {err:?}"
        );
    }
}
