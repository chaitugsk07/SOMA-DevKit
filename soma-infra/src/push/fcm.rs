//! Firebase Cloud Messaging (FCM) HTTP v1 client.
//!
//! Mechanism only: RS256 JWT assertion → OAuth2 access token (with caching),
//! then POST to the FCM send endpoint.  No token lifecycle, no fanout, no
//! retry policy — those are the consuming service's policy.
//!
//! # Access token caching
//!
//! A fresh OAuth2 token is fetched on first use and cached in a `Mutex`.  The
//! cached token is reused until it is within 60 seconds of expiry, then
//! refreshed transparently.

use std::time::{Duration, Instant};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

const FCM_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";

/// Configuration for the FCM client.  Pass the raw contents of the Firebase
/// service account JSON file downloaded from the Firebase console.
pub struct FcmConfig {
    /// Complete service account JSON string (the `.json` file from Firebase console).
    pub service_account_json: String,
}

/// A message to send via FCM.
#[derive(Debug, Clone)]
pub struct FcmMessage {
    /// FCM registration token for the target device.
    pub token: String,
    /// Optional notification title (visible system notification).
    pub title: Option<String>,
    /// Optional notification body.
    pub body: Option<String>,
    /// Arbitrary data payload (string → string map).
    pub data: Option<std::collections::HashMap<String, String>>,
}

/// Errors from FCM operations.
#[derive(Debug, thiserror::Error)]
pub enum FcmError {
    /// Failed to parse the service account JSON.
    #[error("invalid FCM service account JSON: {0}")]
    InvalidConfig(String),

    /// Failed to parse or load the RSA private key.
    #[error("invalid FCM RSA private key: {0}")]
    InvalidKey(String),

    /// `reqwest::Client` construction failed.
    #[error("HTTP client error: {0}")]
    HttpClient(String),

    /// JWT mint or OAuth2 token exchange failed.
    #[error("FCM auth error: {0}")]
    Auth(String),

    /// FCM returned a provider error. Exposes the gRPC status string (e.g.
    /// `"UNREGISTERED"`, `"INVALID_ARGUMENT"`) and the HTTP status code.
    #[error("FCM provider error {http_status} ({status}): {message}")]
    Provider {
        http_status: u16,
        /// gRPC status string from FCM's error body (`error.status`).
        status: String,
        message: String,
    },

    /// Network or unexpected HTTP error.
    #[error("FCM request error: {0}")]
    Request(String),
}

impl FcmError {
    /// The gRPC status string from FCM's error body, e.g. `"UNREGISTERED"`.
    pub fn status(&self) -> Option<&str> {
        match self {
            Self::Provider { status, .. } => Some(status.as_str()),
            _ => None,
        }
    }

    /// `true` if FCM says the token is no longer valid — the consuming service
    /// should deactivate the device.
    pub fn is_unregistered(&self) -> bool {
        matches!(self.status(), Some("UNREGISTERED" | "NOT_FOUND"))
    }
}

// ── Internal SA JSON structure ────────────────────────────────────────────────

#[derive(Deserialize)]
struct ServiceAccount {
    project_id: String,
    client_email: String,
    private_key: String,
}

// ── JWT claims for the OAuth2 assertion ──────────────────────────────────────

#[derive(Serialize)]
struct AssertionClaims<'a> {
    iss: &'a str,
    sub: &'a str,
    aud: &'a str,
    scope: &'a str,
    iat: u64,
    exp: u64,
}

// ── Token endpoint response ───────────────────────────────────────────────────

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

// ── FCM error body ────────────────────────────────────────────────────────────

#[derive(Deserialize, Default)]
struct FcmErrorBody {
    error: Option<FcmErrorDetail>,
}

#[derive(Deserialize, Default)]
struct FcmErrorDetail {
    // ponytail: `code` is in FCM's error JSON but we use the HTTP status directly;
    // serde(default) silently ignores absent fields, so we skip storing it.
    #[serde(default)]
    status: String,
    #[serde(default)]
    message: String,
}

/// FCM client.  Thread-safe; share via `Arc<FcmClient>`.
pub struct FcmClient {
    http: reqwest::Client,
    sa: ServiceAccount,
    encoding_key: EncodingKey,
    /// Cached `(access_token, expiry_instant)`.
    token_cache: Mutex<Option<(String, Instant)>>,
    /// Token endpoint URL — overridable for tests.
    token_url: String,
    /// FCM send base URL — overridable for tests.
    fcm_base_url: String,
}

impl FcmClient {
    /// Build a client from the Firebase service account JSON.
    pub fn new(cfg: FcmConfig) -> Result<Self, FcmError> {
        Self::with_urls(
            cfg,
            "https://oauth2.googleapis.com/token".into(),
            "https://fcm.googleapis.com".into(),
        )
    }

    /// Build with explicit endpoint URLs.  Used in tests.
    pub fn with_urls(
        cfg: FcmConfig,
        token_url: String,
        fcm_base_url: String,
    ) -> Result<Self, FcmError> {
        let sa: ServiceAccount = serde_json::from_str(&cfg.service_account_json)
            .map_err(|e| FcmError::InvalidConfig(e.to_string()))?;

        let encoding_key = EncodingKey::from_rsa_pem(sa.private_key.as_bytes())
            .map_err(|e| FcmError::InvalidKey(e.to_string()))?;

        let http = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| FcmError::HttpClient(e.to_string()))?;

        Ok(Self {
            http,
            sa,
            encoding_key,
            token_cache: Mutex::new(None),
            token_url,
            fcm_base_url,
        })
    }

    /// Return a cached OAuth2 access token, fetching a fresh one if needed.
    async fn access_token(&self) -> Result<String, FcmError> {
        let mut cache = self.token_cache.lock().await;
        if let Some((token, expiry)) = cache.as_ref() {
            if Instant::now() + Duration::from_secs(60) < *expiry {
                return Ok(token.clone());
            }
        }
        let (token, expires_in) = self.fetch_token().await?;
        let expiry = Instant::now() + Duration::from_secs(expires_in);
        *cache = Some((token.clone(), expiry));
        Ok(token)
    }

    /// Mint an RS256 JWT assertion and exchange it for an OAuth2 access token.
    async fn fetch_token(&self) -> Result<(String, u64), FcmError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let claims = AssertionClaims {
            iss: &self.sa.client_email,
            sub: &self.sa.client_email,
            aud: &self.token_url,
            scope: FCM_SCOPE,
            iat: now,
            exp: now + 3600,
        };
        let header = Header::new(Algorithm::RS256);
        let assertion = encode(&header, &claims, &self.encoding_key)
            .map_err(|e| FcmError::Auth(format!("JWT sign error: {e}")))?;

        let form = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &assertion),
        ];
        let resp = self
            .http
            .post(&self.token_url)
            .form(&form)
            .send()
            .await
            .map_err(|e| FcmError::Auth(format!("token request error: {e}")))?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(FcmError::Auth(format!("token endpoint error: {text}")));
        }

        let tr: TokenResponse = resp
            .json()
            .await
            .map_err(|e| FcmError::Auth(format!("token parse error: {e}")))?;

        Ok((tr.access_token, tr.expires_in))
    }

    /// Send `msg` via FCM HTTP v1.
    ///
    /// Returns the FCM message `name` string on success (e.g.
    /// `"projects/my-project/messages/0:1234567890"`).
    pub async fn send(&self, msg: FcmMessage) -> Result<String, FcmError> {
        let token = self.access_token().await?;

        let mut message = serde_json::json!({ "token": msg.token });
        if msg.title.is_some() || msg.body.is_some() {
            message["notification"] = serde_json::json!({
                "title": msg.title,
                "body": msg.body,
            });
        }
        if let Some(data) = msg.data {
            message["data"] = Value::Object(
                data.into_iter()
                    .map(|(k, v)| (k, Value::String(v)))
                    .collect(),
            );
        }
        let body = serde_json::json!({ "message": message });

        let url = format!(
            "{}/v1/projects/{}/messages:send",
            self.fcm_base_url, self.sa.project_id
        );

        let resp = self
            .http
            .post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| FcmError::Request(e.to_string()))?;

        let http_status = resp.status().as_u16();
        if resp.status().is_success() {
            let result: Value = resp
                .json()
                .await
                .map_err(|e| FcmError::Request(e.to_string()))?;
            return Ok(result["name"].as_str().unwrap_or("").to_string());
        }

        let err_body: FcmErrorBody = resp.json().await.unwrap_or_default();
        let detail = err_body.error.unwrap_or_default();
        Err(FcmError::Provider {
            http_status,
            status: if detail.status.is_empty() {
                http_status.to_string()
            } else {
                detail.status
            },
            message: detail.message,
        })
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    // RSA PKCS#8 private key (test-only, not a secret).
    // Generated with: openssl genrsa 2048 | openssl pkcs8 -topk8 -nocrypt
    const TEST_RSA_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCoW4IevVyJCHAO
3hPbyRO9hd7qW4KVuJhNI4UrI0XVlzQ1BAJEVzwUZq7PNF1olsMfW+JN3XCxhq9M
nRF0QhTu7Nr5yTsxx/7WOBH57meSuX+8fQFs9Rm9u9DsfRk9F0z+wr6kTr5f9VYv
cqI+hMDhaN+LBc+lPdAgCJvF7ItuMc4PUo3FozKjlhl6I4Z9ftsHzRMUPmwXrNP7
NRB6t4x3aov/yKIpVwTADS6p5ksMQ18kpejHibS5YRKUY9ioi/LieBm9ZLYiTTqK
DROPAlRWSxjGW72FOiQACdCEho7+S7zMsFXpWViBsboze7A/RfKam8WCG7K/p3at
Ta7ZWQyZAgMBAAECggEACg/hCp8I6ig60Uqoz26Yu0+mU2Wpe/MKyDZ2oa3DoVi+
fiopjAXWMYZzDW984zC6g/PpBjwVPYj2DmtHRSZja6W0jy/9lZO1SX6TpXdAEbL8
PUrP1GAgPBie65ls8hujxoijXfzWUDQgpOiSJZAn2nj6/ks4PjEf8KAvFjDVYv4m
ZXMFxSTr2KC3nwiqKnhNitEbatpNs0lkhOAlMdfVvA+XieVbQ56Eidrv8ffVBMFg
/p1hIPTuK8uFBfVm369hziEbiL6pPumLTA91QV3IB1R0Ek3rSlBJov+lR8KkHfjh
pO2AxMD9L8BjMXXBDdANC+Jb7ykZ0LcNvo5qRd1goQKBgQDgZzYH0QSodoWNE425
T7tno0TwUixY/PWFMjwP0bg/wcWXNSbGu/aLqCPBpbLTgQmH7v1vSa/ktUXh53Kp
XezM3d31jkzUqDgfiS4Gd+bTAOvSOjDg04ySWWOBP70sxwzq1aZJOCAite9++6RT
MddJgqy1Vnbcqx1Yc2zg3jeP+QKBgQDAEBXXpo1/iz/3RP8x5SkjoaolyJV1IKyF
Hv1cU2iivGdoIhzodarGZ3ZojcD6AFaXEa/HXlP1b2uNQcSIERkO/qsbgla3FEBw
zcLFb7sVjO2Gqdc2tbVypwgRO8noB4SEYYO3qSjPIpcn24jNTxxE89y99wMLJZUC
01yMDIXJoQKBgQCArE37/2tnWcbMlTi/2KtCCwm9r7DEOhBiluJ/w6ad0HOHd6/L
Zv5BKGGQaX3371bSvkROKveT8imnFnrWCQh+lH3wju8ZRXghBR0CEoCl3zJZsopS
cJt7U4xOEYldAqoygd2+wFMU9DPnIU4hckPbX/W+aVzwvaGvFUiKTIGZqQKBgA1O
0ASiQLmur171w+z0IPacFAsMK/byraGHtSx5hw2Hmm6ntnMQ/CVFM/oooE9ySI2t
Jw3cZlriPoSKIxD+hkr4sGh2joWe/JwUVcOSa1ch7a9gA06CJrFsC24OQ341TyRR
EWxfZzl6/xg+6Oq46Y+JiikLfYV/NCT++jGmCHshAoGBANFmu4dS6ra7nlszRfhD
wE/fTRAkDr6kbzlMAUZaRZZ5AtrPpPot4WLLXss3DlLxggk/t12Rr2+aBJjzMsuu
//KvwsGXLtx4QY9rzgPh1NxurBvCgMUGXp8BBwhgHKpd62WPVt9cENUmgeEb5j2o
iux0bx1s4OlGb5GuPaYCICv4
-----END PRIVATE KEY-----";

    fn test_sa_json(token_url: &str) -> String {
        // The `aud` in our JWT claim is the token_url; we bake it in here so
        // the token-endpoint mock can validate it easily.
        let _ = token_url; // aud is set at request time, not here
        serde_json::json!({
            "type": "service_account",
            "project_id": "test-project",
            "client_email": "test@test-project.iam.gserviceaccount.com",
            "private_key": TEST_RSA_KEY
        })
        .to_string()
    }

    #[tokio::test]
    async fn fcm_success_and_token_cached() {
        let server = MockServer::start().await;

        // Token endpoint — expect exactly ONE call (second send should reuse cache)
        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "ya29.test_token",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .expect(1)
            .mount(&server)
            .await;

        // FCM send endpoint
        Mock::given(method("POST"))
            .and(path("/v1/projects/test-project/messages:send"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "projects/test-project/messages/0:1234567890"
            })))
            .expect(2) // two sends, one token fetch
            .mount(&server)
            .await;

        let token_url = format!("{}/token", server.uri());
        let fcm_base = server.uri();
        let client = FcmClient::with_urls(
            FcmConfig {
                service_account_json: test_sa_json(&token_url),
            },
            token_url,
            fcm_base,
        )
        .unwrap();

        // First send — fetches token
        let name = client
            .send(FcmMessage {
                token: "device1".into(),
                title: Some("Hello".into()),
                body: Some("World".into()),
                data: None,
            })
            .await
            .unwrap();
        assert_eq!(name, "projects/test-project/messages/0:1234567890");

        // Second send — must reuse cached token (mock expects exactly 1 token call)
        client
            .send(FcmMessage {
                token: "device2".into(),
                title: None,
                body: None,
                data: None,
            })
            .await
            .unwrap();

        server.verify().await;
    }

    #[tokio::test]
    async fn fcm_unregistered_returns_typed_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "ya29.test",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/v1/projects/test-project/messages:send"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "error": {
                    "code": 404,
                    "status": "UNREGISTERED",
                    "message": "Registration token is not a valid FCM registration token"
                }
            })))
            .mount(&server)
            .await;

        let token_url = format!("{}/token", server.uri());
        let fcm_base = server.uri();
        let client = FcmClient::with_urls(
            FcmConfig {
                service_account_json: test_sa_json(&token_url),
            },
            token_url,
            fcm_base,
        )
        .unwrap();

        let err = client
            .send(FcmMessage {
                token: "bad_token".into(),
                title: None,
                body: None,
                data: None,
            })
            .await
            .unwrap_err();

        assert!(
            matches!(&err, FcmError::Provider { http_status: 404, status, .. } if status == "UNREGISTERED")
        );
        assert_eq!(err.status(), Some("UNREGISTERED"));
        assert!(err.is_unregistered());
    }

    #[tokio::test]
    async fn fcm_invalid_argument_error_not_unregistered() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "ya29.test",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/v1/projects/test-project/messages:send"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "error": {
                    "code": 400,
                    "status": "INVALID_ARGUMENT",
                    "message": "Invalid value at 'message.token'"
                }
            })))
            .mount(&server)
            .await;

        let token_url = format!("{}/token", server.uri());
        let fcm_base = server.uri();
        let client = FcmClient::with_urls(
            FcmConfig {
                service_account_json: test_sa_json(&token_url),
            },
            token_url,
            fcm_base,
        )
        .unwrap();

        let err = client
            .send(FcmMessage {
                token: "bad".into(),
                title: None,
                body: None,
                data: None,
            })
            .await
            .unwrap_err();

        assert!(
            matches!(&err, FcmError::Provider { http_status: 400, status, .. } if status == "INVALID_ARGUMENT")
        );
        assert!(
            !err.is_unregistered(),
            "INVALID_ARGUMENT should not be classified as unregistered"
        );
    }
}
