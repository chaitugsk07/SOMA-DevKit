#![forbid(unsafe_code)]
//! # soma-infra
//!
//! Shared utilities for the soma-platform. Every concern lives behind its own
//! cargo feature so a consumer compiles only what it uses.
//!
//! | feature | module | provides |
//! |---------|--------|----------|
//! | `db` (default) | [`db`] | Postgres `PoolConfig` + [`connect`] |
//! | `tracing` (default) | [`telemetry`] | `tracing-subscriber` setup |
//! | `config` | [`config`] | env-var helpers |
//! | `errors` | [`errors`] | safe DB-error redaction |
//! | `testing` | [`testing`] | throwaway-DB harness for integration tests |
//! | `cache` | [`cache`] | Redis `CacheConfig` + `ConnectionManager` connect/ops |
//! | `crypto` | [`crypto`] | AES-256-GCM encrypt/decrypt + Argon2id password hashing |
//! | `storage-s3` | [`storage`] | AWS S3 (or S3-compatible) `StorageConfig` + `StorageClient` get/put/delete |
//! | `storage-azure` | [`storage`] | Azure Blob Storage `StorageConfig` + `StorageClient` get/put/delete |
//! | `llm` | [`llm`] | Anthropic `LlmConfig` + `LlmClient::messages` — typed request/response, token usage; no retry, no prompt logic |
//! | `http` | [`http`] | `client()` / `client_with_timeouts(..)` — `reqwest::Client` with rustls TLS and sane default timeouts |
//! | `kg` | [`kg`] | Knowledge-graph nodes/edges + pgvector cosine search | `pgvector` |
//! | `signal` | [`signal`] | `shutdown_signal()` — await Ctrl-C (SIGINT) or, on Unix, SIGTERM; pure plumbing for graceful shutdown |
//! | `web` | [`web`] | `serve_spa` (embedded-SPA fallback handler), `extract_bearer` (Authorization header), `serve_with_shutdown` (bind + graceful-serve loop) |
//! | `email` | [`email`] | `SmtpClient` (pooled SMTP, rustls), `DkimSigner` (RSA-SHA256 relaxed/relaxed), `RawEmail` builder |
//! | `push` | [`push`] | `ApnsClient` (APNs HTTP/2, ES256 JWT), `FcmClient` (FCM v1, RS256 OAuth2), `WebPushClient` (RFC 8291/8292 VAPID) |
//! | `iam-client` | [`iam_client`] | `JwksConfig` + `JwksVerifier` — ES256 JWKS fetch + in-memory cache (15-min TTL, 60-s cooldown, single-flight); `verify::<C: DeserializeOwned>(token)` returns caller-supplied claims; `prefetch()` for startup warm-up; `axum_ext::extract_token` + `IamClaims<C>` extractor when `web` is also active |
//!
//! Adopt in another repo:
//! ```toml
//! soma-infra = { path = "../soma-infra", features = ["db", "tracing"] }
//! # tests only:
//! soma-infra = { path = "../soma-infra", features = ["testing"] }
//! ```

#[cfg(feature = "db")]
pub mod db;
#[cfg(feature = "db")]
pub use db::{connect, connect_from_env, PoolConfig};

#[cfg(feature = "tracing")]
pub mod telemetry;

#[cfg(feature = "config")]
pub mod config;

#[cfg(feature = "errors")]
pub mod errors;

#[cfg(feature = "testing")]
pub mod testing;
#[cfg(feature = "testing")]
pub use testing::TestDb;

#[cfg(feature = "cache")]
pub mod cache;

#[cfg(feature = "crypto")]
pub mod crypto;

#[cfg(any(feature = "storage-s3", feature = "storage-azure"))]
pub mod storage;

#[cfg(feature = "llm")]
pub mod llm;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "kg")]
pub mod kg;

#[cfg(feature = "signal")]
pub mod signal;

#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "email")]
pub mod email;

#[cfg(feature = "push")]
pub mod push;

#[cfg(feature = "iam-client")]
pub mod iam_client;
