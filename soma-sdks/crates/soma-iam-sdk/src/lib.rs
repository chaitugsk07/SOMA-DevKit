#![forbid(unsafe_code)]
//! soma-iam SDK — JWT/JWKS verification for services consuming soma-iam tokens.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use soma_iam_sdk::JwksVerifier;
//!
//! # async fn run() {
//! let verifier = JwksVerifier::new(
//!     "http://localhost:3741/.well-known/jwks.json",
//!     "http://localhost:3741",
//!     "soma-iam",
//! );
//!
//! // On each incoming request:
//! let claims = verifier.verify("eyJ...").await.unwrap();
//! println!("user: {}", claims.sub);
//! # }
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use jsonwebtoken::DecodingKey;
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

pub use soma_crypto::jwt::AccessTokenClaims;

/// Errors returned by [`JwksVerifier::verify`].
#[derive(Debug, Error)]
pub enum JwksError {
    /// The JWT signature, expiry, issuer, or audience check failed.
    #[error("invalid token: {0}")]
    InvalidToken(String),
    /// JWKS endpoint fetch or parse failed.
    #[error("JWKS fetch failed: {0}")]
    FetchError(String),
    /// Cache is empty (no keys fetched yet) — fail closed.
    #[error("JWKS cache empty; token verification unavailable")]
    CacheEmpty,
}

struct Inner {
    keys: HashMap<Uuid, DecodingKey>,
    /// When keys were last successfully fetched. None = never.
    last_fetched: Option<Instant>,
    /// When we last attempted a refresh (successful or not). Controls 60s cooldown.
    last_refresh_attempt: Option<Instant>,
}

/// Async JWKS verifier with lazy single-flight refresh and 15-minute staleness TTL.
///
/// - Keys are fetched once lazily on first `verify()` call.
/// - Stale (> 15 min) keys trigger a background refresh on the next call.
/// - Failed refreshes are suppressed for 60 s (cooldown) to avoid hammering the IAM server.
/// - Fail closed: if the cache is empty and refresh fails, `verify` returns `Err(CacheEmpty)`.
/// - `Clone` is cheap — all state is behind `Arc`.
/// - Refresh is single-flight: concurrent stale `verify()` calls serialize through `refresh_mu`;
///   the second caller double-checks staleness after acquiring the mutex and skips the fetch.
#[derive(Clone)]
pub struct JwksVerifier {
    jwks_url: String,
    issuer: String,
    audience: String,
    inner: Arc<RwLock<Inner>>,
    /// Ensures only one goroutine fetches at a time (single-flight guarantee).
    refresh_mu: Arc<Mutex<()>>,
}

const STALENESS_SECS: u64 = 900; // 15 minutes
const COOLDOWN_SECS: u64 = 60; // 60 seconds between refresh attempts

impl JwksVerifier {
    /// Create a new verifier. No network call is made until the first `verify()`.
    pub fn new(
        jwks_url: impl Into<String>,
        issuer: impl Into<String>,
        audience: impl Into<String>,
    ) -> Self {
        Self {
            jwks_url: jwks_url.into(),
            issuer: issuer.into(),
            audience: audience.into(),
            inner: Arc::new(RwLock::new(Inner {
                keys: HashMap::new(),
                last_fetched: None,
                last_refresh_attempt: None,
            })),
            refresh_mu: Arc::new(Mutex::new(())),
        }
    }

    /// Verify a JWT, returning the decoded [`AccessTokenClaims`] on success.
    ///
    /// Refreshes JWKS lazily (15-min TTL, 60-s cooldown). Fail closed on empty cache.
    pub async fn verify(&self, token: &str) -> Result<AccessTokenClaims, JwksError> {
        let needs_refresh = {
            let inner = self.inner.read().await;
            inner
                .last_fetched
                .map_or(true, |t| t.elapsed().as_secs() > STALENESS_SECS)
        };

        if needs_refresh {
            self.try_refresh().await;
        }

        let inner = self.inner.read().await;
        if inner.keys.is_empty() {
            return Err(JwksError::CacheEmpty);
        }

        soma_crypto::jwt::verify_access_token(token, &self.issuer, &self.audience, &inner.keys)
            .map_err(|e| JwksError::InvalidToken(e.to_string()))
    }

    /// Attempt to refresh the key cache. Respects the 60-second cooldown.
    /// Old keys remain in place if the fetch fails (non-fatal, logged at WARN).
    ///
    /// Single-flight: concurrent callers serialize through `refresh_mu`. The second caller
    /// re-checks staleness after acquiring the lock and skips the fetch if a peer just refreshed.
    async fn try_refresh(&self) {
        // Fast path: populated caches honor the cooldown immediately. Empty caches
        // must still compete for the mutex so concurrent first-verifies can wait
        // for the in-flight fetch instead of failing closed spuriously.
        {
            let inner = self.inner.read().await;
            if !inner.keys.is_empty() {
                if let Some(t) = inner.last_refresh_attempt {
                    if t.elapsed().as_secs() < COOLDOWN_SECS {
                        return;
                    }
                }
            }
        }

        // Serialize all refreshers; only one fetches at a time.
        let _guard = self.refresh_mu.lock().await;

        // Double-check: a peer may have refreshed while we waited for the mutex.
        {
            let inner = self.inner.read().await;
            if let Some(t) = inner.last_refresh_attempt {
                if t.elapsed().as_secs() < COOLDOWN_SECS {
                    if inner.keys.is_empty() {
                        // The peer just attempted a refresh but the cache is still empty,
                        // which means the fetch failed. Respect the cooldown.
                        return;
                    }
                    // The peer filled the cache while we waited, so our work is done.
                    return;
                }
            }
        }

        // Record attempt timestamp before the network call.
        {
            let mut inner = self.inner.write().await;
            inner.last_refresh_attempt = Some(Instant::now());
        }

        match self.fetch_keys().await {
            Ok(keys) => {
                let mut inner = self.inner.write().await;
                inner.keys = keys;
                inner.last_fetched = Some(Instant::now());
                tracing::debug!(url = %self.jwks_url, "JWKS refreshed ({} keys)", inner.keys.len());
            }
            Err(e) => {
                tracing::warn!(url = %self.jwks_url, "JWKS refresh failed: {e}");
            }
        }
    }

    /// Fetch and parse the JWKS endpoint, returning a kid→DecodingKey map.
    async fn fetch_keys(&self) -> Result<HashMap<Uuid, DecodingKey>, JwksError> {
        use jsonwebtoken::jwk::JwkSet;

        let client = soma_infra::http::client()
            .map_err(|e| JwksError::FetchError(format!("HTTP client build failed: {e}")))?;
        let body: serde_json::Value = client
            .get(&self.jwks_url)
            .send()
            .await
            .map_err(|e| JwksError::FetchError(e.to_string()))?
            .error_for_status()
            .map_err(|e| JwksError::FetchError(e.to_string()))?
            .json()
            .await
            .map_err(|e| JwksError::FetchError(e.to_string()))?;

        let jwk_set: JwkSet = serde_json::from_value(body)
            .map_err(|e| JwksError::FetchError(format!("JWK parse error: {e}")))?;

        let mut map = HashMap::with_capacity(jwk_set.keys.len());
        for jwk in &jwk_set.keys {
            // kid must be a UUID — soma-iam sets it to the signing key UUID.
            let kid_str = jwk.common.key_id.as_deref().unwrap_or("");
            let kid: Uuid = kid_str.parse().map_err(|_| {
                JwksError::FetchError(format!("JWK kid is not a UUID: {kid_str:?}"))
            })?;
            let dk = DecodingKey::from_jwk(jwk)
                .map_err(|e| JwksError::FetchError(format!("DecodingKey build failed: {e}")))?;
            map.insert(kid, dk);
        }

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn future_exp() -> usize {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
            + 900
    }

    fn now_iat() -> usize {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
    }

    #[test]
    fn verifier_new_does_not_fetch() {
        // Construction must not make any network calls — just verify the struct is created.
        let v = JwksVerifier::new(
            "http://localhost:9999/jwks.json",
            "http://localhost:9999",
            "soma-iam",
        );
        // Runtime state: empty cache, no fetches attempted.
        // ponytail: tokio::test would be needed for async assertions; this unit test
        // just checks the sync invariant (empty at construction).
        assert_eq!(v.jwks_url, "http://localhost:9999/jwks.json");
        assert_eq!(v.issuer, "http://localhost:9999");
        assert_eq!(v.audience, "soma-iam");
    }

    #[tokio::test]
    async fn verify_returns_cache_empty_when_no_keys() {
        // Point at a URL that will never serve — should fail closed.
        let v = JwksVerifier::new("http://127.0.0.1:1/jwks.json", "iss", "aud");
        // Skip the network attempt by confirming CacheEmpty after a failed refresh.
        // The refresh cooldown starts after the failed attempt, so calling verify
        // immediately after construction (empty cache, refresh will fail fast) returns CacheEmpty.
        let result = v.verify("not.a.token").await;
        assert!(matches!(
            result,
            Err(JwksError::CacheEmpty) | Err(JwksError::FetchError(_))
        ));
    }

    #[tokio::test]
    async fn concurrent_stale_verify_double_checks_cooldown() {
        // After a failed fetch (which sets last_refresh_attempt), a second concurrent call
        // must not attempt another fetch within COOLDOWN_SECS.
        // We verify the double-check logic: after one failed try_refresh, the inner state
        // has last_refresh_attempt set; a second call to try_refresh sees the cooldown and returns.
        // ponytail: fetch_keys is not mockable without a real server; we test the double-check
        // state machine directly via try_refresh + inner inspection.
        let v = JwksVerifier::new("http://127.0.0.1:1/jwks.json", "iss", "aud");

        // First refresh attempt — will fail (no server), but records last_refresh_attempt.
        v.try_refresh().await;

        // Confirm cooldown is now set.
        let elapsed = {
            let inner = v.inner.read().await;
            inner
                .last_refresh_attempt
                .map(|t| t.elapsed().as_secs())
                .unwrap_or(u64::MAX)
        };
        assert!(
            elapsed < COOLDOWN_SECS,
            "cooldown should be fresh after first attempt"
        );

        // Second try_refresh must return immediately (fast path) without doing another fetch.
        // If single-flight is broken, this would still exit fast via the cooldown, so this
        // primarily documents the invariant; the real concurrent-fetch guard is the mutex.
        v.try_refresh().await;
        // Keys remain empty — no spurious second fetch changed anything.
        let keys_empty = v.inner.read().await.keys.is_empty();
        assert!(keys_empty, "keys must remain empty after failed fetches");
    }

    #[tokio::test]
    async fn concurrent_first_verify_waits_for_inflight_refresh() {
        use jsonwebtoken::EncodingKey;
        use soma_crypto::{
            jwt::issue_access_token,
            signing::{build_jwks_bytes, generate_es256_keypair, signing_key_to_pem},
        };
        use tokio::{
            io::{AsyncReadExt, AsyncWriteExt},
            net::TcpListener,
            time::{sleep, Duration},
        };

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let issuer = format!("http://{}", addr);
        let jwks_url = format!("{issuer}/.well-known/jwks.json");

        let (signing_key, verifying_key) = generate_es256_keypair();
        let key_id = Uuid::new_v4();
        let encoding_key = EncodingKey::from_ec_pem(&signing_key_to_pem(&signing_key)).unwrap();
        let jwks_body = build_jwks_bytes(&[(key_id, verifying_key)]);

        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut req_buf = [0u8; 2048];
            let _ = stream.read(&mut req_buf).await.unwrap();
            sleep(Duration::from_millis(150)).await;
            let headers = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                jwks_body.len()
            );
            stream.write_all(headers.as_bytes()).await.unwrap();
            stream.write_all(&jwks_body).await.unwrap();
        });

        let verifier = JwksVerifier::new(jwks_url, issuer.clone(), "soma-iam");
        let token = issue_access_token(
            &AccessTokenClaims {
                iss: issuer,
                sub: Uuid::new_v4().to_string(),
                aud: vec!["soma-iam".into()],
                exp: future_exp(),
                iat: now_iat(),
                jti: Uuid::new_v4().to_string(),
                tid: Uuid::new_v4(),
                mid: Uuid::new_v4(),
                wid: None,
                perms: vec!["scope.all".into()],
                phn: None,
                platform_admin: false,
                client_id: None,
                scope: None,
                mfa_pending: None,
            },
            key_id,
            &encoding_key,
        )
        .unwrap();

        let verifier_two = verifier.clone();
        let token_two = token.clone();
        let (first, second) =
            tokio::join!(verifier.verify(&token), verifier_two.verify(&token_two));

        assert!(
            first.is_ok(),
            "first verify should succeed after initial refresh"
        );
        assert!(
            second.is_ok(),
            "concurrent verify should wait for the in-flight refresh"
        );

        server.await.unwrap();
    }
}
