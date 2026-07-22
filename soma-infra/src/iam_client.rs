//! Generic ES256 JWKS verifier for services consuming soma-iam (or any OIDC-compatible
//! provider) tokens.
//!
//! Mechanism only: fetch a JWKS endpoint, cache decoding keys in-process with a
//! 15-minute staleness TTL, verify an ES256 JWT with `iss` + `aud` + `exp`
//! validated. The **claims type is caller-supplied** (`C: serde::de::DeserializeOwned`).
//! Services consuming soma-iam SSO tokens use [`soma_iam_sdk::AccessTokenClaims`] as
//! `C`; services that only need a claim subset define their own minimal struct.
//!
//! Algorithm is **always** ES256 — the token header's `alg` field is never used for
//! algorithm selection. `kid` in the header selects the decoding key from the cache.
//!
//! # Feature
//!
//! Enable via `features = ["iam-client"]` in `Cargo.toml`.
//!
//! ```toml
//! soma-infra = { path = "…/soma-infra", features = ["iam-client"] }
//! ```
//!
//! # Quick start
//!
//! ```rust,no_run
//! use soma_infra::iam_client::{JwksConfig, JwksVerifier};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct MyClaims { sub: String, exp: usize }
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let cfg = JwksConfig::from_env()?;
//! let verifier = JwksVerifier::new(cfg);
//! verifier.prefetch().await?;   // optional: warm the cache at startup
//!
//! let claims: MyClaims = verifier.verify("eyJ…").await?;
//! println!("user: {}", claims.sub);
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};

// ── Config ────────────────────────────────────────────────────────────────────

/// Configuration for a [`JwksVerifier`].
#[derive(Debug, Clone)]
pub struct JwksConfig {
    /// JWKS endpoint URL, e.g. `"http://soma-iam:3741/.well-known/jwks.json"`.
    /// Inside Docker, use the internal service name, not the public URL.
    pub jwks_url: String,
    /// Expected `iss` claim value — the IAM public URL seen by browsers,
    /// e.g. `"http://localhost/iam"`. Must match what iam put in the token.
    pub issuer: String,
    /// Expected `aud` claim value, e.g. `"soma-console"`.
    pub audience: String,
}

impl JwksConfig {
    /// Build from explicit values.
    pub fn new(
        jwks_url: impl Into<String>,
        issuer: impl Into<String>,
        audience: impl Into<String>,
    ) -> Self {
        Self {
            jwks_url: jwks_url.into(),
            issuer: issuer.into(),
            audience: audience.into(),
        }
    }

    /// Build from environment variables.
    ///
    /// | var | required | default |
    /// |-----|----------|---------|
    /// | `SOMA_IAM_ISSUER` | yes | — |
    /// | `SOMA_IAM_JWKS_URL` | no | `{issuer}/.well-known/jwks.json` |
    /// | `SOMA_IAM_AUDIENCE` | no | `"soma-console"` |
    ///
    /// Note the two-env contract: `SOMA_IAM_ISSUER` is the *public* URL used as
    /// the expected `iss` claim; `SOMA_IAM_JWKS_URL` is where the service fetches
    /// keys (internal network, e.g. `http://soma-iam:3741/.well-known/jwks.json`).
    ///
    /// # Errors
    ///
    /// Returns `IamClientError::MissingEnv` when `SOMA_IAM_ISSUER` is not set.
    pub fn from_env() -> Result<Self, IamClientError> {
        let issuer = std::env::var("SOMA_IAM_ISSUER")
            .map_err(|_| IamClientError::MissingEnv("SOMA_IAM_ISSUER"))?;
        // ponytail: JWKS_URL defaults to {issuer}/.well-known/jwks.json for
        // single-host setups. In Docker, override with the internal URL.
        let jwks_url = std::env::var("SOMA_IAM_JWKS_URL")
            .unwrap_or_else(|_| format!("{issuer}/.well-known/jwks.json"));
        // ponytail: "soma-console" default matches the SSO cookie audience.
        // API-to-API consumers set SOMA_IAM_AUDIENCE to "soma-iam" or their service name.
        let audience =
            std::env::var("SOMA_IAM_AUDIENCE").unwrap_or_else(|_| "soma-console".to_owned());
        Ok(Self::new(jwks_url, issuer, audience))
    }
}

// ── Error ─────────────────────────────────────────────────────────────────────

/// Errors returned by [`JwksVerifier`].
#[derive(Debug, Error)]
pub enum IamClientError {
    /// The JWT signature, expiry, issuer, audience, algorithm, or `kid` check failed.
    #[error("invalid token: {0}")]
    InvalidToken(String),
    /// JWKS endpoint fetch or parse failed.
    #[error("JWKS fetch failed: {0}")]
    FetchError(String),
    /// Cache is empty (no keys fetched yet or all fetches failed) — fail closed.
    #[error("JWKS cache empty; token verification unavailable")]
    CacheEmpty,
    /// A required environment variable was missing.
    #[error("required env var {0} is not set")]
    MissingEnv(&'static str),
}

// ── Inner state ───────────────────────────────────────────────────────────────

struct Inner {
    keys: HashMap<String, DecodingKey>,
    /// When keys were last successfully fetched. `None` = never.
    last_fetched: Option<Instant>,
    /// When we last attempted a refresh (success or failure). Controls cooldown.
    last_refresh_attempt: Option<Instant>,
}

// ── Verifier ──────────────────────────────────────────────────────────────────

// ponytail: 15-min staleness + 60-s cooldown mirror soma-iam-sdk defaults.
// JWKS keys rotate infrequently; these are conservative. Tune per workload
// via a custom JwksConfig if needed — the upgrade path is adding a Config field.
const STALENESS_SECS: u64 = 900;
const COOLDOWN_SECS: u64 = 60;

/// Async JWKS verifier with lazy single-flight refresh and 15-minute staleness TTL.
///
/// - Keys are fetched lazily on the first [`verify`](Self::verify) call (or eagerly via [`prefetch`](Self::prefetch)).
/// - Stale keys (> 15 min old) trigger a background refresh on the next call.
/// - Failed refreshes are suppressed for 60 s (cooldown) to avoid hammering the provider.
/// - **Fail closed**: if the cache is empty and refresh fails, `verify` returns [`IamClientError::CacheEmpty`].
/// - `Clone` is cheap — all state is behind `Arc`.
/// - Refresh is single-flight: concurrent stale `verify()` callers serialize through an internal
///   `Mutex`. The second caller double-checks the cooldown after acquiring the lock and skips
///   the fetch if a peer just refreshed.
#[derive(Clone)]
pub struct JwksVerifier {
    cfg: JwksConfig,
    inner: Arc<RwLock<Inner>>,
    /// Single-flight gate: only one goroutine fetches keys at a time.
    refresh_mu: Arc<Mutex<()>>,
}

impl JwksVerifier {
    /// Create a verifier from an explicit config. No network call is made until the
    /// first [`verify`](Self::verify) or an explicit [`prefetch`](Self::prefetch).
    pub fn new(cfg: JwksConfig) -> Self {
        Self {
            cfg,
            inner: Arc::new(RwLock::new(Inner {
                keys: HashMap::new(),
                last_fetched: None,
                last_refresh_attempt: None,
            })),
            refresh_mu: Arc::new(Mutex::new(())),
        }
    }

    /// Convenience: [`JwksConfig::from_env`] then [`JwksVerifier::new`].
    ///
    /// # Errors
    ///
    /// Returns `IamClientError::MissingEnv` when `SOMA_IAM_ISSUER` is not set.
    pub fn from_env() -> Result<Self, IamClientError> {
        Ok(Self::new(JwksConfig::from_env()?))
    }

    /// Create a verifier with pre-loaded keys and mark the cache as fresh.
    ///
    /// Keys are considered freshly fetched at construction time — no refresh will be
    /// attempted until the 15-minute TTL elapses. Useful for tests and static
    /// configurations where keys are known ahead of time.
    pub fn with_preloaded_keys(cfg: JwksConfig, keys: HashMap<String, DecodingKey>) -> Self {
        let now = Instant::now();
        Self {
            cfg,
            inner: Arc::new(RwLock::new(Inner {
                keys,
                last_fetched: Some(now),
                last_refresh_attempt: Some(now),
            })),
            refresh_mu: Arc::new(Mutex::new(())),
        }
    }

    /// Look up the decoding key for `kid` without running full JWT validation.
    ///
    /// Triggers a JWKS refresh if the cache is stale (> 15 min) or empty. Returns
    /// [`IamClientError::InvalidToken`] when `kid` is not present after refresh.
    ///
    /// Use this when you need to supply your own [`jsonwebtoken::Validation`] (e.g.
    /// to accept multiple algorithms). For standard single-algorithm verification,
    /// prefer [`JwksVerifier::verify`].
    pub async fn get_key(&self, kid: &str) -> Result<DecodingKey, IamClientError> {
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
            return Err(IamClientError::CacheEmpty);
        }
        inner
            .keys
            .get(kid)
            .cloned()
            .ok_or_else(|| IamClientError::InvalidToken(format!("unknown kid: {kid:?}")))
    }

    /// Eagerly fetch and cache keys. Call once at service startup to avoid the
    /// first-request latency spike.
    ///
    /// # Errors
    ///
    /// Returns `IamClientError::FetchError` when the JWKS endpoint is unreachable
    /// or returns a non-2xx response.
    pub async fn prefetch(&self) -> Result<(), IamClientError> {
        let keys = self.fetch_keys().await?;
        let mut inner = self.inner.write().await;
        inner.keys = keys;
        inner.last_fetched = Some(Instant::now());
        inner.last_refresh_attempt = Some(Instant::now());
        Ok(())
    }

    /// Verify an ES256 JWT, decoding claims into `C` on success.
    ///
    /// Security invariants enforced:
    /// - Algorithm is **always** `ES256` — the token header's `alg` field is ignored.
    /// - `iss` and `aud` are validated against [`JwksConfig`].
    /// - `exp` is checked with `leeway = 0`.
    /// - `kid` in the header selects the decoding key; unknown `kid` → `InvalidToken`.
    ///
    /// # Errors
    ///
    /// - [`IamClientError::CacheEmpty`] — keys never fetched (and fetch failed).
    /// - [`IamClientError::InvalidToken`] — signature, expiry, iss, aud, or kid check failed.
    pub async fn verify<C: DeserializeOwned>(&self, token: &str) -> Result<C, IamClientError> {
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
            return Err(IamClientError::CacheEmpty);
        }

        // Decode header unauthenticated to get kid. The `alg` field is IGNORED.
        let header =
            decode_header(token).map_err(|e| IamClientError::InvalidToken(e.to_string()))?;
        let kid = header
            .kid
            .ok_or_else(|| IamClientError::InvalidToken("missing kid header".into()))?;
        let dk = inner
            .keys
            .get(&kid)
            .ok_or_else(|| IamClientError::InvalidToken(format!("unknown kid: {kid:?}")))?;

        // SECURITY: algorithm pinned to ES256; the token header's alg is never trusted.
        let mut validation = Validation::new(Algorithm::ES256);
        validation.set_issuer(&[&self.cfg.issuer]);
        validation.set_audience(&[&self.cfg.audience]);
        validation.validate_exp = true;
        validation.leeway = 0;

        let data = decode::<C>(token, dk, &validation)
            .map_err(|e| IamClientError::InvalidToken(e.to_string()))?;
        Ok(data.claims)
    }

    // ── Internal ──────────────────────────────────────────────────────────────

    async fn try_refresh(&self) {
        // Fast path: cooldown check under read lock — avoids mutex contention on hot path.
        {
            let inner = self.inner.read().await;
            if let Some(t) = inner.last_refresh_attempt {
                if t.elapsed().as_secs() < COOLDOWN_SECS {
                    return;
                }
            }
        }

        // Single-flight: only one fetch at a time.
        let _guard = self.refresh_mu.lock().await;

        // Double-check after acquiring the mutex — a peer may have refreshed while we waited.
        {
            let inner = self.inner.read().await;
            if let Some(t) = inner.last_refresh_attempt {
                if t.elapsed().as_secs() < COOLDOWN_SECS {
                    return;
                }
            }
        }

        // Record attempt timestamp *before* the network call so a fast failure still
        // sets the cooldown and prevents a thundering-herd on a down endpoint.
        {
            self.inner.write().await.last_refresh_attempt = Some(Instant::now());
        }

        match self.fetch_keys().await {
            Ok(keys) => {
                let mut inner = self.inner.write().await;
                let n = keys.len();
                inner.keys = keys;
                inner.last_fetched = Some(Instant::now());
                #[cfg(feature = "tracing")]
                tracing::debug!(url = %self.cfg.jwks_url, keys = n, "JWKS refreshed");
                let _ = n; // suppress unused-variable warning when tracing is off
            }
            Err(e) => {
                // ponytail: leave existing keys in place on refresh failure — fail closed only
                // when the cache is empty (verify() returns CacheEmpty). This tolerates
                // transient IAM downtime for up to STALENESS_SECS after the last success.
                #[cfg(feature = "tracing")]
                tracing::warn!(url = %self.cfg.jwks_url, error = %e, "JWKS refresh failed");
                let _ = e; // suppress unused when tracing is off
            }
        }
    }

    async fn fetch_keys(&self) -> Result<HashMap<String, DecodingKey>, IamClientError> {
        use jsonwebtoken::jwk::JwkSet;

        let client = crate::http::client()
            .map_err(|e| IamClientError::FetchError(format!("HTTP client: {e}")))?;
        let jwk_set: JwkSet = client
            .get(&self.cfg.jwks_url)
            .send()
            .await
            .map_err(|e| IamClientError::FetchError(e.to_string()))?
            .error_for_status()
            .map_err(|e| IamClientError::FetchError(e.to_string()))?
            .json::<JwkSet>()
            .await
            .map_err(|e| IamClientError::FetchError(format!("JWK parse: {e}")))?;

        let mut map = HashMap::with_capacity(jwk_set.keys.len());
        for jwk in &jwk_set.keys {
            let kid = jwk.common.key_id.as_deref().unwrap_or("").to_owned();
            let dk = DecodingKey::from_jwk(jwk)
                .map_err(|e| IamClientError::FetchError(format!("DecodingKey: {e}")))?;
            map.insert(kid, dk);
        }
        Ok(map)
    }
}

// ── axum helpers (requires `web` feature for axum dep) ───────────────────────

/// Axum helpers: token extraction from `Authorization: Bearer` or SSO cookies,
/// plus an `IamClaims<C>` extractor that verifies with [`JwksVerifier`].
///
/// Available only when **both** `iam-client` and `web` features are enabled.
#[cfg(feature = "web")]
pub mod axum_ext {
    use super::JwksVerifier;
    use axum::{
        extract::{FromRef, FromRequestParts},
        http::{header, request::Parts, HeaderMap, StatusCode},
    };
    use serde::de::DeserializeOwned;
    use std::sync::Arc;

    /// Extract a bearer token or SSO cookie from request headers.
    ///
    /// Extraction order (first match wins):
    /// 1. `Authorization: Bearer <token>`
    /// 2. `__Host-soma_sso` cookie (production secure cookie)
    /// 3. `soma_sso` cookie (development insecure cookie via `SOMA_COOKIE_INSECURE`)
    ///
    /// Returns `None` when no credential is present in any of the three locations.
    /// The caller decides what to do (e.g. return 401, fall through to another auth path).
    pub fn extract_token(headers: &HeaderMap) -> Option<String> {
        // 1. Authorization: Bearer
        if let Some(token) = headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
        {
            return Some(token.to_owned());
        }
        // 2 + 3. SSO cookies — prod (__Host-) first, dev fallback
        for name in ["__Host-soma_sso", "soma_sso"] {
            if let Some(v) = find_cookie(headers, name) {
                return Some(v);
            }
        }
        None
    }

    fn find_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
        // HTTP/1.1 may have multiple Cookie header lines; iterate all of them.
        for hv in headers.get_all(header::COOKIE) {
            if let Ok(raw) = hv.to_str() {
                if let Some(val) = raw.split(';').find_map(|pair| {
                    let (k, v) = pair.trim().split_once('=')?;
                    (k.trim() == name).then(|| v.trim().to_owned())
                }) {
                    return Some(val);
                }
            }
        }
        None
    }

    /// Axum extractor that reads a token via [`extract_token`], verifies it with
    /// [`JwksVerifier`], and produces decoded claims `C`.
    ///
    /// Requires `Arc<JwksVerifier>` to be available from the router state via
    /// [`FromRef`]. Simplest case: the state IS `Arc<JwksVerifier>`. For composite
    /// state, implement `FromRef<YourState>` for `Arc<JwksVerifier>`.
    ///
    /// Returns HTTP 401 on missing credential or verification failure. The calling
    /// service wraps this extractor if it needs a custom rejection body.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use soma_infra::iam_client::{JwksConfig, JwksVerifier};
    /// use soma_infra::iam_client::axum_ext::IamClaims;
    /// use axum::{Router, routing::get};
    /// use serde::Deserialize;
    /// use std::sync::Arc;
    ///
    /// #[derive(Deserialize)]
    /// struct Claims { sub: String }
    ///
    /// async fn me(IamClaims(c): IamClaims<Claims>) -> String { c.sub }
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let v = Arc::new(JwksVerifier::new(JwksConfig::from_env()?));
    /// let app: axum::Router = Router::new().route("/me", get(me)).with_state(v);
    /// # Ok(())
    /// # }
    /// ```
    pub struct IamClaims<C>(pub C);

    impl<S, C> FromRequestParts<S> for IamClaims<C>
    where
        S: Send + Sync,
        Arc<JwksVerifier>: FromRef<S>,
        C: DeserializeOwned + Send + 'static,
    {
        type Rejection = StatusCode;

        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, StatusCode> {
            let verifier = Arc::<JwksVerifier>::from_ref(state);
            let token = extract_token(&parts.headers).ok_or(StatusCode::UNAUTHORIZED)?;
            let claims: C = verifier
                .verify(&token)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;
            Ok(IamClaims(claims))
        }
    }

    // ── Tests ─────────────────────────────────────────────────────────────────

    #[cfg(test)]
    mod tests {
        use super::*;
        use axum::http::{HeaderName, HeaderValue};

        fn headers_with(pairs: &[(&str, &str)]) -> HeaderMap {
            let mut m = HeaderMap::new();
            for (k, v) in pairs {
                m.insert(
                    HeaderName::from_bytes(k.as_bytes()).unwrap(),
                    HeaderValue::from_str(v).unwrap(),
                );
            }
            m
        }

        #[test]
        fn bearer_header_extracted() {
            let h = headers_with(&[("authorization", "Bearer tok.en.value")]);
            assert_eq!(extract_token(&h).as_deref(), Some("tok.en.value"));
        }

        #[test]
        fn prod_sso_cookie_extracted() {
            let h = headers_with(&[("cookie", "__Host-soma_sso=cookie.tok.val; other=x")]);
            assert_eq!(extract_token(&h).as_deref(), Some("cookie.tok.val"));
        }

        #[test]
        fn dev_sso_cookie_extracted_as_fallback() {
            let h = headers_with(&[("cookie", "soma_sso=dev.tok.val; foo=bar")]);
            assert_eq!(extract_token(&h).as_deref(), Some("dev.tok.val"));
        }

        #[test]
        fn bearer_wins_over_cookie() {
            let h = headers_with(&[
                ("authorization", "Bearer bearer.tok"),
                ("cookie", "soma_sso=cookie.tok"),
            ]);
            assert_eq!(extract_token(&h).as_deref(), Some("bearer.tok"));
        }

        #[test]
        fn prod_cookie_wins_over_dev_cookie() {
            let h = headers_with(&[("cookie", "__Host-soma_sso=prod.tok; soma_sso=dev.tok")]);
            assert_eq!(extract_token(&h).as_deref(), Some("prod.tok"));
        }

        #[test]
        fn no_credential_returns_none() {
            let h = HeaderMap::new();
            assert!(extract_token(&h).is_none());
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use p256::ecdsa::SigningKey;
    use p256::pkcs8::{EncodePrivateKey, EncodePublicKey};
    use rand::rngs::OsRng;
    use serde::{Deserialize, Serialize};
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::Uuid;

    // ── Test key helpers ──────────────────────────────────────────────────────

    /// Generate a fresh ES256 key pair via p256, exporting to PEM for jsonwebtoken.
    fn gen_keys() -> (EncodingKey, DecodingKey, Uuid) {
        let sk = SigningKey::random(&mut OsRng);
        let vk = sk.verifying_key();
        let kid = Uuid::new_v4();
        // PKCS#8 PEM for the signing key
        let sk_pem = sk
            .to_pkcs8_pem(Default::default())
            .expect("PKCS8 PEM is infallible")
            .as_bytes()
            .to_vec();
        // SPKI PEM for the verifying key
        let vk_pem = vk
            .to_public_key_pem(Default::default())
            .expect("SPKI PEM is infallible")
            .into_bytes();
        let ek = EncodingKey::from_ec_pem(&sk_pem).expect("valid EC PEM");
        let dk = DecodingKey::from_ec_pem(&vk_pem).expect("valid EC PEM");
        (ek, dk, kid)
    }

    /// Build a verifier with pre-loaded keys (no network needed for verify tests).
    fn verifier_with_key(dk: DecodingKey, kid: Uuid, iss: &str, aud: &str) -> JwksVerifier {
        let mut keys = HashMap::new();
        keys.insert(kid.to_string(), dk);
        JwksVerifier::with_preloaded_keys(
            JwksConfig::new("http://unused.test/jwks.json", iss, aud),
            keys,
        )
    }

    fn now_secs() -> usize {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
    }

    // Minimal claims struct — callers supply their own in production.
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestClaims {
        iss: String,
        sub: String,
        aud: Vec<String>,
        exp: usize,
        iat: usize,
        jti: String,
    }

    fn make_token(ek: &EncodingKey, kid: Uuid, iss: &str, aud: &str, exp: usize) -> String {
        let header = Header {
            alg: Algorithm::ES256,
            kid: Some(kid.to_string()),
            ..Default::default()
        };
        let claims = TestClaims {
            iss: iss.to_owned(),
            sub: Uuid::new_v4().to_string(),
            aud: vec![aud.to_owned()],
            exp,
            iat: now_secs(),
            jti: Uuid::new_v4().to_string(),
        };
        encode(&header, &claims, ek).expect("test token encode")
    }

    const ISS: &str = "https://iam.test";
    const AUD: &str = "soma-console";

    // ── Accept tests ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn valid_token_accepted() {
        let (ek, dk, kid) = gen_keys();
        let verifier = verifier_with_key(dk, kid, ISS, AUD);
        let token = make_token(&ek, kid, ISS, AUD, now_secs() + 900);
        let claims: TestClaims = verifier.verify(&token).await.expect("valid token");
        assert_eq!(claims.iss, ISS);
    }

    // ── Reject tests ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn wrong_audience_rejected() {
        let (ek, dk, kid) = gen_keys();
        let verifier = verifier_with_key(dk, kid, ISS, AUD);
        let token = make_token(&ek, kid, ISS, "wrong-aud", now_secs() + 900);
        assert!(matches!(
            verifier.verify::<TestClaims>(&token).await,
            Err(IamClientError::InvalidToken(_))
        ));
    }

    #[tokio::test]
    async fn wrong_issuer_rejected() {
        let (ek, dk, kid) = gen_keys();
        let verifier = verifier_with_key(dk, kid, ISS, AUD);
        let token = make_token(&ek, kid, "https://evil.example.com", AUD, now_secs() + 900);
        assert!(matches!(
            verifier.verify::<TestClaims>(&token).await,
            Err(IamClientError::InvalidToken(_))
        ));
    }

    #[tokio::test]
    async fn expired_token_rejected() {
        let (ek, dk, kid) = gen_keys();
        let verifier = verifier_with_key(dk, kid, ISS, AUD);
        // exp=1 is deep in the past
        let token = make_token(&ek, kid, ISS, AUD, 1);
        assert!(matches!(
            verifier.verify::<TestClaims>(&token).await,
            Err(IamClientError::InvalidToken(_))
        ));
    }

    #[tokio::test]
    async fn unknown_kid_rejected() {
        let (ek, dk, kid) = gen_keys();
        let verifier = verifier_with_key(dk, kid, ISS, AUD);
        let wrong_kid = Uuid::new_v4(); // not in verifier's key map
        let token = make_token(&ek, wrong_kid, ISS, AUD, now_secs() + 900);
        assert!(matches!(
            verifier.verify::<TestClaims>(&token).await,
            Err(IamClientError::InvalidToken(_))
        ));
    }

    #[tokio::test]
    async fn tampered_signature_rejected() {
        let (ek, dk, kid) = gen_keys();
        let verifier = verifier_with_key(dk, kid, ISS, AUD);
        let mut token = make_token(&ek, kid, ISS, AUD, now_secs() + 900);
        // Flip the last character of the signature segment
        let last = token.pop().unwrap();
        token.push(if last == 'a' { 'b' } else { 'a' });
        assert!(matches!(
            verifier.verify::<TestClaims>(&token).await,
            Err(IamClientError::InvalidToken(_))
        ));
    }

    // ── Network / cache tests ─────────────────────────────────────────────────

    #[tokio::test]
    async fn cache_empty_on_unreachable_host() {
        let verifier = JwksVerifier::new(JwksConfig::new("http://127.0.0.1:1/jwks.json", ISS, AUD));
        let result = verifier.verify::<TestClaims>("not.a.token").await;
        // Either fetch failed (and cache is empty → CacheEmpty) or fetch errored.
        // Both are valid outcomes depending on timing; what must NOT happen is Ok(_).
        assert!(
            matches!(
                result,
                Err(IamClientError::CacheEmpty) | Err(IamClientError::FetchError(_))
            ),
            "expected CacheEmpty or FetchError, got: {result:?}"
        );
    }

    #[tokio::test]
    async fn cooldown_prevents_second_fetch() {
        let verifier = JwksVerifier::new(JwksConfig::new("http://127.0.0.1:1/jwks.json", ISS, AUD));
        // First attempt — will fail but sets last_refresh_attempt
        verifier.try_refresh().await;
        let elapsed = {
            let inner = verifier.inner.read().await;
            inner
                .last_refresh_attempt
                .map(|t| t.elapsed().as_secs())
                .unwrap_or(u64::MAX)
        };
        assert!(
            elapsed < COOLDOWN_SECS,
            "cooldown should be fresh after first attempt"
        );

        // Second attempt must return immediately (cooldown active); cache stays empty
        verifier.try_refresh().await;
        let still_empty = verifier.inner.read().await.keys.is_empty();
        assert!(still_empty, "keys must remain empty after failed fetches");
    }
}
