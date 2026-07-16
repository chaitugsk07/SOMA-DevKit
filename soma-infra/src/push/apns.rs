//! Apple Push Notification service (APNs) HTTP/2 client.
//!
//! Mechanism only: ES256 provider JWT generation + caching, raw HTTP/2 POST to
//! `/3/device/{token}`.  No device-token lifecycle, no fanout, no retry policy.
//!
//! # Provider JWT
//!
//! Apple requires an ES256 JWT signed with the `.p8` private key.  The JWT is valid
//! for up to one hour; this client caches it and re-mints when it is older than
//! 50 minutes.  On `ExpiredProviderToken` (our own stale token — not the device's),
//! the JWT is force-refreshed and the request retried exactly once.

use std::time::{Duration, Instant};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

/// Configuration for the APNs client.
#[derive(Debug, Clone)]
pub struct ApnsConfig {
    /// 10-character APNs key ID (from Apple Developer portal).
    pub key_id: String,
    /// 10-character Apple Team ID.
    pub team_id: String,
    /// PKCS#8 EC private key in PEM format (the `.p8` file Apple provides,
    /// `-----BEGIN PRIVATE KEY-----`).
    pub private_key_p8: String,
    /// `true` → connect to `api.sandbox.push.apple.com`.
    pub use_sandbox: bool,
}

/// Successful APNs send response.
#[derive(Debug, Clone)]
pub struct ApnsResponse {
    /// `apns-id` UUID returned by Apple (useful for debugging / idempotency).
    pub apns_id: String,
}

/// Errors from APNs operations.
#[derive(Debug, thiserror::Error)]
pub enum ApnsError {
    /// Failed to parse the private key PEM.
    #[error("invalid APNs private key: {0}")]
    InvalidKey(String),

    /// `reqwest::Client` construction failed.
    #[error("HTTP client error: {0}")]
    HttpClient(String),

    /// JWT mint or signing failed.
    #[error("APNs JWT error: {0}")]
    Jwt(String),

    /// APNs returned a provider error (4xx/5xx with a JSON reason body).
    #[error("APNs provider error {status}: {reason}")]
    Provider { status: u16, reason: String },

    /// Network or unexpected HTTP error.
    #[error("APNs request error: {0}")]
    Request(String),
}

impl ApnsError {
    /// The reason string from Apple's JSON body, e.g. `"BadDeviceToken"`.
    /// `None` for non-provider errors.
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Provider { reason, .. } => Some(reason.as_str()),
            _ => None,
        }
    }

    /// `true` if the error means the device token is invalid and should be
    /// deactivated.  The service, not this library, decides what to do.
    pub fn is_bad_token(&self) -> bool {
        matches!(
            self.reason(),
            Some("BadDeviceToken" | "Unregistered" | "DeviceTokenNotForTopic")
        )
    }
}

#[derive(Serialize)]
struct ApnsClaims<'a> {
    iss: &'a str,
    iat: u64,
}

/// Unused but satisfies serde_json deserialization of Apple's error body.
#[derive(Deserialize, Default)]
struct ApnsErrorBody {
    reason: Option<String>,
}

/// APNs client.  Clone-safe (inner `Arc` via `Mutex`); share via `Arc<ApnsClient>`.
pub struct ApnsClient {
    http: reqwest::Client,
    cfg: ApnsConfig,
    encoding_key: EncodingKey,
    /// Cached `(jwt, issued_at)`.  Refreshed when the JWT is older than 50 min.
    token_cache: Mutex<Option<(String, Instant)>>,
    /// Base URL — overridable for tests.
    base_url: String,
}

impl ApnsClient {
    /// Build a client from `cfg`.  Validates and loads the private key eagerly.
    pub fn new(cfg: ApnsConfig) -> Result<Self, ApnsError> {
        let base_url = if cfg.use_sandbox {
            "https://api.sandbox.push.apple.com"
        } else {
            "https://api.push.apple.com"
        }
        .to_string();
        Self::with_base_url(cfg, base_url)
    }

    /// Build with an explicit base URL.  Used in tests to point at a mock server.
    pub fn with_base_url(cfg: ApnsConfig, base_url: String) -> Result<Self, ApnsError> {
        let encoding_key = EncodingKey::from_ec_pem(cfg.private_key_p8.as_bytes())
            .map_err(|e| ApnsError::InvalidKey(e.to_string()))?;

        // ponytail: no .http2_prior_knowledge() here — reqwest 0.12 will negotiate
        // HTTP/2 via TLS ALPN on production APNs connections (port 443).  The mock
        // server in tests speaks HTTP/1.1 and falls back gracefully.
        let http = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| ApnsError::HttpClient(e.to_string()))?;

        Ok(Self {
            http,
            cfg,
            encoding_key,
            token_cache: Mutex::new(None),
            base_url,
        })
    }

    /// Return a cached provider JWT, minting a fresh one if needed.
    async fn provider_jwt(&self) -> Result<String, ApnsError> {
        let mut cache = self.token_cache.lock().await;
        if let Some((token, issued_at)) = cache.as_ref() {
            if issued_at.elapsed() < Duration::from_secs(50 * 60) {
                return Ok(token.clone());
            }
        }
        let jwt = self.mint_jwt()?;
        *cache = Some((jwt.clone(), Instant::now()));
        Ok(jwt)
    }

    fn mint_jwt(&self) -> Result<String, ApnsError> {
        let iat = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(self.cfg.key_id.clone());

        let claims = ApnsClaims {
            iss: &self.cfg.team_id,
            iat,
        };
        encode(&header, &claims, &self.encoding_key).map_err(|e| ApnsError::Jwt(e.to_string()))
    }

    /// Send a push notification to `device_token`.
    ///
    /// `payload_json` is the complete APS payload, e.g.
    /// `serde_json::json!({"aps":{"alert":"Hello"}})`.
    ///
    /// On `ExpiredProviderToken` the provider JWT is force-refreshed and the
    /// request retried exactly once (our own stale JWT — not the device's token).
    /// All other provider errors are returned immediately without retry.
    pub async fn send(
        &self,
        device_token: &str,
        topic: &str,
        payload_json: Value,
        push_type: &str,
        priority: u8,
    ) -> Result<ApnsResponse, ApnsError> {
        let jwt = self.provider_jwt().await?;
        let result = self
            .send_once(
                device_token,
                topic,
                &payload_json,
                push_type,
                priority,
                &jwt,
            )
            .await;

        // Force-refresh the provider JWT and retry once on ExpiredProviderToken.
        // This is a mechanism concern: the stale token is ours, not the device's.
        let is_expired = matches!(
            &result,
            Err(ApnsError::Provider { reason, .. }) if reason == "ExpiredProviderToken"
        );
        if is_expired {
            let new_jwt = {
                let mut cache = self.token_cache.lock().await;
                let j = self.mint_jwt()?;
                *cache = Some((j.clone(), Instant::now()));
                j
            };
            return self
                .send_once(
                    device_token,
                    topic,
                    &payload_json,
                    push_type,
                    priority,
                    &new_jwt,
                )
                .await;
        }
        result
    }

    async fn send_once(
        &self,
        device_token: &str,
        topic: &str,
        payload_json: &Value,
        push_type: &str,
        priority: u8,
        jwt: &str,
    ) -> Result<ApnsResponse, ApnsError> {
        let url = format!("{}/3/device/{}", self.base_url, device_token);

        let resp = self
            .http
            .post(&url)
            .header("authorization", format!("bearer {}", jwt))
            .header("apns-topic", topic)
            .header("apns-push-type", push_type)
            .header("apns-priority", priority.to_string())
            .json(payload_json)
            .send()
            .await
            .map_err(|e| ApnsError::Request(e.to_string()))?;

        let status = resp.status().as_u16();
        let apns_id = resp
            .headers()
            .get("apns-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        if status == 200 {
            return Ok(ApnsResponse { apns_id });
        }

        let body: ApnsErrorBody = resp.json().await.unwrap_or_default();
        let reason = body.reason.unwrap_or_else(|| "Unknown".to_string());
        Err(ApnsError::Provider { status, reason })
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::{method, path_regex},
        Mock, MockServer, ResponseTemplate,
    };

    // P-256 PKCS#8 private key (test-only, not a secret).
    // Generated with: openssl ecparam -genkey -name prime256v1 -noout | openssl pkcs8 -topk8 -nocrypt
    const TEST_P8_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgCXAxOl2tTfYLXIIF
BBKmIBF1NbWoicWu/sOpYibOHvyhRANCAAT3PRG7cUm++NdSzIh9GzodnZyIsCEO
FDQboAwKzY/mLSNWyca7QLSYE5PM7hzceb/FBYfKRUYyS2TgGAozFsB1
-----END PRIVATE KEY-----";

    fn test_cfg(base_url: &str) -> (ApnsConfig, String) {
        let cfg = ApnsConfig {
            key_id: "TESTKEY1234".to_string(),
            team_id: "TEAMID1234".to_string(),
            private_key_p8: TEST_P8_KEY.to_string(),
            use_sandbox: false,
        };
        (cfg, base_url.to_string())
    }

    #[tokio::test]
    async fn apns_success_returns_apns_id() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/3/device/.*"))
            .respond_with(ResponseTemplate::new(200).append_header("apns-id", "test-apns-id-uuid"))
            .mount(&server)
            .await;

        let (cfg, base_url) = test_cfg(&server.uri());
        let client = ApnsClient::with_base_url(cfg, base_url).unwrap();
        let resp = client
            .send(
                "device_token_abc",
                "com.example.app",
                serde_json::json!({"aps":{"alert":"Hello"}}),
                "alert",
                10,
            )
            .await
            .unwrap();

        assert_eq!(resp.apns_id, "test-apns-id-uuid");
    }

    #[tokio::test]
    async fn apns_bad_device_token_returns_typed_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/3/device/.*"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_json(serde_json::json!({"reason":"BadDeviceToken"})),
            )
            .mount(&server)
            .await;

        let (cfg, base_url) = test_cfg(&server.uri());
        let client = ApnsClient::with_base_url(cfg, base_url).unwrap();
        let err = client
            .send(
                "bad_device_token",
                "com.example.app",
                serde_json::json!({"aps":{}}),
                "alert",
                10,
            )
            .await
            .unwrap_err();

        assert!(
            matches!(&err, ApnsError::Provider { status: 400, reason, .. } if reason == "BadDeviceToken")
        );
        assert_eq!(err.reason(), Some("BadDeviceToken"));
        assert!(err.is_bad_token());
    }

    #[tokio::test]
    async fn apns_expired_provider_token_retries_once() {
        let server = MockServer::start().await;

        // Register success mock FIRST (lower LIFO priority) — matches the retry.
        Mock::given(method("POST"))
            .and(path_regex(r"/3/device/.*"))
            .respond_with(ResponseTemplate::new(200).append_header("apns-id", "retry-apns-id"))
            .mount(&server)
            .await;

        // Register error mock SECOND (higher LIFO priority) limited to 1 response.
        // After one use it is exhausted and the success mock handles the retry.
        Mock::given(method("POST"))
            .and(path_regex(r"/3/device/.*"))
            .respond_with(
                ResponseTemplate::new(403)
                    .set_body_json(serde_json::json!({"reason":"ExpiredProviderToken"})),
            )
            .up_to_n_times(1)
            .mount(&server)
            .await;

        let (cfg, base_url) = test_cfg(&server.uri());
        let client = ApnsClient::with_base_url(cfg, base_url).unwrap();
        let resp = client
            .send(
                "device_token",
                "com.example.app",
                serde_json::json!({"aps":{}}),
                "alert",
                10,
            )
            .await
            .unwrap();

        assert_eq!(resp.apns_id, "retry-apns-id");
    }

    #[test]
    fn apns_jwt_header_has_correct_kid_and_algorithm() {
        let cfg = ApnsConfig {
            key_id: "MYKEYID1234".to_string(),
            team_id: "MYTEAMID12".to_string(),
            private_key_p8: TEST_P8_KEY.to_string(),
            use_sandbox: false,
        };
        let client = ApnsClient::with_base_url(cfg, "https://api.push.apple.com".into()).unwrap();
        let jwt = client.mint_jwt().unwrap();

        // Decode the header (base64url-encoded JSON, first segment)
        let header_b64 = jwt.split('.').next().unwrap();
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        let header_bytes = URL_SAFE_NO_PAD.decode(header_b64).unwrap();
        let header: serde_json::Value = serde_json::from_slice(&header_bytes).unwrap();

        assert_eq!(header["alg"], "ES256", "APNs JWT must use ES256");
        assert_eq!(
            header["kid"], "MYKEYID1234",
            "APNs JWT kid must match key_id"
        );

        // Verify with the matching P-256 public key using jsonwebtoken
        use jsonwebtoken::{Algorithm, DecodingKey, Validation};
        let public_pem = extract_p256_public_pem_for_test(TEST_P8_KEY);
        let dk = DecodingKey::from_ec_pem(public_pem.as_bytes()).unwrap();
        let mut v = Validation::new(Algorithm::ES256);
        v.required_spec_claims = std::collections::HashSet::new(); // relax required claims
        v.validate_exp = false;
        v.validate_aud = false;
        let decoded = jsonwebtoken::decode::<serde_json::Value>(&jwt, &dk, &v).unwrap();
        assert_eq!(decoded.claims["iss"], "MYTEAMID12");
    }

    /// Extract the P-256 public key as PEM from a PKCS#8 private key PEM.
    fn extract_p256_public_pem_for_test(private_pem: &str) -> String {
        use p256::{
            pkcs8::{DecodePrivateKey, EncodePublicKey, LineEnding},
            SecretKey,
        };
        let sk = SecretKey::from_pkcs8_pem(private_pem).unwrap();
        sk.public_key().to_public_key_pem(LineEnding::LF).unwrap()
    }
}
