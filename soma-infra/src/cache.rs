//! Redis connection management via a multiplexed `ConnectionManager`.
//!
//! Pure plumbing: configure and connect, then hand the manager to the caller.
//! No key prefixing, no default TTL, no serialization, no cache-aside logic —
//! those belong in the service that knows what it's caching.
//!
//! Build a manager from an explicit config with [`connect`] or pull it
//! straight from the environment with [`connect_from_env`].

use std::time::Duration;

use redis::AsyncCommands as _;

/// Re-export for consumers that need to name the type without a direct redis dep.
pub use redis::aio::ConnectionManager;

/// Tunables for the Redis `ConnectionManager`. Build from an explicit URL with
/// [`CacheConfig::new`] or from the environment with [`CacheConfig::from_env`].
///
/// `ConnectionManager` is a multiplexed, internally-reconnecting manager:
/// there is no pool-size knob, only per-operation and per-connection timeouts.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Redis connection URL, e.g. `redis://127.0.0.1:6379`.
    pub url: String,
    // ponytail: Duration::MAX means "no timeout" — tune per workload.
    /// Timeout applied to each Redis command response. `None` → no timeout.
    pub response_timeout: Option<Duration>,
    /// Timeout for establishing a new connection to the server. `None` → no timeout.
    pub connection_timeout: Option<Duration>,
}

impl CacheConfig {
    /// A config with sane defaults for the given URL.
    ///
    /// Both timeouts default to `None` (no timeout). Set them explicitly for
    /// latency-sensitive paths.
    // ponytail: no-timeout default is safe for a start; add CACHE_*_TIMEOUT_SECS
    // in production to avoid unbounded blocking on a dead Redis.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            response_timeout: None,
            connection_timeout: None,
        }
    }

    /// Read `REDIS_URL` (required) plus optional overrides
    /// `CACHE_RESPONSE_TIMEOUT_SECS`, `CACHE_CONNECTION_TIMEOUT_SECS`.
    pub fn from_env() -> Result<Self, CacheError> {
        let url = std::env::var("REDIS_URL").map_err(|_| CacheError::MissingEnv("REDIS_URL"))?;
        let mut cfg = Self::new(url);
        if let Ok(v) = std::env::var("CACHE_RESPONSE_TIMEOUT_SECS") {
            let secs: u64 = v
                .parse()
                .map_err(|_| CacheError::InvalidEnv("CACHE_RESPONSE_TIMEOUT_SECS"))?;
            cfg.response_timeout = Some(Duration::from_secs(secs));
        }
        if let Ok(v) = std::env::var("CACHE_CONNECTION_TIMEOUT_SECS") {
            let secs: u64 = v
                .parse()
                .map_err(|_| CacheError::InvalidEnv("CACHE_CONNECTION_TIMEOUT_SECS"))?;
            cfg.connection_timeout = Some(Duration::from_secs(secs));
        }
        Ok(cfg)
    }
}

/// Errors from building a cache connection or executing a command.
///
/// Note: `PartialEq` is intentionally NOT derived — `redis::RedisError` does
/// not implement it.
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    /// A required environment variable was missing.
    #[error("required env var {0} is not set")]
    MissingEnv(&'static str),
    /// An environment variable held an unparseable value.
    #[error("env var {0} has an invalid value")]
    InvalidEnv(&'static str),
    /// An error returned by the Redis driver.
    #[error(transparent)]
    Redis(#[from] redis::RedisError),
}

/// Build and connect a `ConnectionManager` from a [`CacheConfig`].
///
/// Does NOT ping the server — health-checking is the caller's responsibility.
pub async fn connect(cfg: &CacheConfig) -> Result<redis::aio::ConnectionManager, CacheError> {
    let client = redis::Client::open(cfg.url.as_str())?;
    // ponytail: retry backoff constants mirror ConnectionManager::new defaults
    // (base=2, factor=100ms, 6 retries). Tune per workload if needed.
    let manager = if cfg.response_timeout.is_some() || cfg.connection_timeout.is_some() {
        redis::aio::ConnectionManager::new_with_backoff_and_timeouts(
            client,
            2,   // exponent_base — matches ConnectionManager::new default
            100, // factor (ms)  — matches ConnectionManager::new default
            6,   // retries      — matches ConnectionManager::new default
            cfg.response_timeout.unwrap_or(Duration::MAX),
            cfg.connection_timeout.unwrap_or(Duration::MAX),
        )
        .await?
    } else {
        redis::aio::ConnectionManager::new(client).await?
    };
    Ok(manager)
}

/// Convenience: [`CacheConfig::from_env`] then [`connect`].
pub async fn connect_from_env() -> Result<redis::aio::ConnectionManager, CacheError> {
    connect(&CacheConfig::from_env()?).await
}

/// GET a key. Returns `None` on a cache miss.
pub async fn get(
    cm: &redis::aio::ConnectionManager,
    key: &str,
) -> Result<Option<Vec<u8>>, CacheError> {
    let mut cm = cm.clone();
    let value: Option<Vec<u8>> = cm.get(key).await?;
    Ok(value)
}

/// SET a key with no expiry.
pub async fn set(
    cm: &redis::aio::ConnectionManager,
    key: &str,
    value: &[u8],
) -> Result<(), CacheError> {
    let mut cm = cm.clone();
    cm.set::<_, _, ()>(key, value).await?;
    Ok(())
}

/// SETEX — SET with a TTL in seconds. Redis rejects `ttl_secs = 0` itself.
pub async fn set_ex(
    cm: &redis::aio::ConnectionManager,
    key: &str,
    value: &[u8],
    ttl_secs: u64,
) -> Result<(), CacheError> {
    let mut cm = cm.clone();
    cm.set_ex::<_, _, ()>(key, value, ttl_secs).await?;
    Ok(())
}

/// DEL a key. Returns `true` if the key existed.
pub async fn del(cm: &redis::aio::ConnectionManager, key: &str) -> Result<bool, CacheError> {
    let mut cm = cm.clone();
    let removed: u64 = cm.del(key).await?;
    Ok(removed > 0)
}

/// EXPIRE — set a TTL on an existing key. Returns `true` if the key existed.
pub async fn expire(
    cm: &redis::aio::ConnectionManager,
    key: &str,
    ttl_secs: u64,
) -> Result<bool, CacheError> {
    let mut cm = cm.clone();
    let existed: bool = cm.expire(key, ttl_secs as i64).await?;
    Ok(existed)
}

/// PUBLISH a message to a Redis channel.
///
/// Takes the `ConnectionManager` by value (callers clone before passing).
/// Use the existing `ConnectionManager` for publishing; subscribing requires a
/// dedicated connection via [`open_subscriber`].
pub async fn publish(
    mut cm: redis::aio::ConnectionManager,
    channel: &str,
    message: &[u8],
) -> Result<(), CacheError> {
    let _: i64 = redis::cmd("PUBLISH")
        .arg(channel)
        .arg(message)
        .query_async(&mut cm)
        .await?;
    Ok(())
}

/// Open a dedicated async pub/sub connection and subscribe to `channel`.
///
/// The returned `PubSub` is in subscribe mode — it cannot issue other Redis
/// commands. Drive it with `into_on_message()` to receive messages, and
/// reconnect by calling [`open_subscriber`] again after the stream ends.
///
/// Use [`publish`] with the shared `ConnectionManager` for the publishing side.
pub async fn open_subscriber(
    redis_url: &str,
    channel: &str,
) -> Result<redis::aio::PubSub, CacheError> {
    let client = redis::Client::open(redis_url)?;
    let mut pubsub = client.get_async_pubsub().await?;
    pubsub.subscribe(channel).await?;
    Ok(pubsub)
}

/// `SET key value EX ttl_secs NX` — atomically set if not exists, with a TTL.
///
/// Returns `true` when the key was newly written (first hit); `false` when it
/// already existed (duplicate).  Use for sliding-window dedup where the first
/// writer wins and all later callers within the window see `false`.
pub async fn set_nx_ex(
    cm: &redis::aio::ConnectionManager,
    key: &str,
    value: &[u8],
    ttl_secs: u64,
) -> Result<bool, CacheError> {
    let mut cm = cm.clone();
    // Redis SET NX EX returns "OK" (Some) on new key, nil (None) on existing key.
    let result: Option<String> = redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("EX")
        .arg(ttl_secs)
        .arg("NX")
        .query_async(&mut cm)
        .await?;
    Ok(result.is_some())
}

// ── Rate-limit primitive ──────────────────────────────────────────────────────

/// Lua script: atomic INCR + conditional EXPIRE for fixed-window rate limiting.
///
/// **Algorithm: fixed window.**  The window starts on first hit and lasts
/// `ARGV[1]` seconds.  A caller that times requests to straddle a window
/// boundary can pass up to `2 * limit` in the worst case (e.g. limit requests
/// at the very end of one window, limit more at the very start of the next).
/// For a brute-force guard this is an acceptable trade-off: the window boundary
/// is anchored to the first hit, not a clock second, so an adversary cannot
/// predict when the window resets.  The absolute maximum across any two adjacent
/// windows is `2 * limit`, not unbounded.
///
/// **Atomicity**: INCR + EXPIRE are run together inside a single Lua eval so a
/// crash between them cannot leave a key without a TTL (which would make the
/// rate limit permanent).
const RATE_LIMIT_SCRIPT: &str = r#"
local count = redis.call('INCR', KEYS[1])
if count == 1 then
  redis.call('EXPIRE', KEYS[1], ARGV[1])
end
return count
"#;

/// Outcome of a single [`rate_limit`] call.
pub struct RateLimitResult {
    /// `true` when the caller is within the limit and the request should proceed.
    pub allowed: bool,
    /// Count after this increment (1-based).
    pub count: u64,
    /// The configured per-window limit supplied by the caller.
    pub limit: u64,
    /// Conservative upper bound on seconds until the window resets.
    /// Use as the value of a `Retry-After` response header.
    pub retry_after_secs: u64,
}

/// Fixed-window rate-limit check-and-increment.
///
/// Atomically increments `key` in Redis and, on the first hit in a window,
/// sets its TTL to `window_secs`.  Returns [`RateLimitResult`] where
/// `allowed = count <= limit`.
///
/// # Fail-open contract
/// This function returns `Err(CacheError::Redis(_))` when Redis is unreachable
/// or times out.  **Callers must decide** whether to allow or deny on error:
/// log at `warn` and allow (fail-open) to keep auth available during a Redis
/// blip, or deny to stay safe at the cost of availability.  The recommended
/// choice for an auth brute-force guard is **fail-open** — see the
/// `rate_limit_middleware` in soma-vault-api for the rationale.
pub async fn rate_limit(
    cm: &redis::aio::ConnectionManager,
    key: &str,
    limit: u64,
    window_secs: u64,
) -> Result<RateLimitResult, CacheError> {
    let mut cm = cm.clone();
    let count: i64 = redis::cmd("EVAL")
        .arg(RATE_LIMIT_SCRIPT)
        .arg(1) // numkeys
        .arg(key)
        .arg(window_secs)
        .query_async(&mut cm)
        .await?;
    let count = count.max(0) as u64;
    Ok(RateLimitResult {
        allowed: count <= limit,
        count,
        limit,
        retry_after_secs: window_secs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_url_and_no_timeouts() {
        let cfg = CacheConfig::new("redis://127.0.0.1:6379");
        assert_eq!(cfg.url, "redis://127.0.0.1:6379");
        assert!(cfg.response_timeout.is_none());
        assert!(cfg.connection_timeout.is_none());
    }

    /// All `from_env` paths are exercised in a single sequential test to avoid
    /// races on the shared `REDIS_URL` / `CACHE_*` env vars. This mirrors how
    /// config.rs avoids parallel env-var conflicts.
    #[test]
    fn from_env_paths() {
        // — missing REDIS_URL —
        std::env::remove_var("REDIS_URL");
        std::env::remove_var("CACHE_RESPONSE_TIMEOUT_SECS");
        std::env::remove_var("CACHE_CONNECTION_TIMEOUT_SECS");
        assert!(matches!(
            CacheConfig::from_env().unwrap_err(),
            CacheError::MissingEnv("REDIS_URL")
        ));

        // — URL present, no optional vars → defaults —
        std::env::set_var("REDIS_URL", "redis://localhost:6379");
        let cfg = CacheConfig::from_env().unwrap();
        assert_eq!(cfg.url, "redis://localhost:6379");
        assert!(cfg.response_timeout.is_none());
        assert!(cfg.connection_timeout.is_none());

        // — both timeout vars present and valid —
        std::env::set_var("CACHE_RESPONSE_TIMEOUT_SECS", "5");
        std::env::set_var("CACHE_CONNECTION_TIMEOUT_SECS", "3");
        let cfg = CacheConfig::from_env().unwrap();
        assert_eq!(cfg.response_timeout, Some(Duration::from_secs(5)));
        assert_eq!(cfg.connection_timeout, Some(Duration::from_secs(3)));

        // — invalid CACHE_RESPONSE_TIMEOUT_SECS —
        std::env::set_var("CACHE_RESPONSE_TIMEOUT_SECS", "notanumber");
        assert!(matches!(
            CacheConfig::from_env().unwrap_err(),
            CacheError::InvalidEnv("CACHE_RESPONSE_TIMEOUT_SECS")
        ));
        std::env::remove_var("CACHE_RESPONSE_TIMEOUT_SECS");

        // — invalid CACHE_CONNECTION_TIMEOUT_SECS —
        std::env::set_var("CACHE_CONNECTION_TIMEOUT_SECS", "bad");
        assert!(matches!(
            CacheConfig::from_env().unwrap_err(),
            CacheError::InvalidEnv("CACHE_CONNECTION_TIMEOUT_SECS")
        ));

        // cleanup
        std::env::remove_var("REDIS_URL");
        std::env::remove_var("CACHE_RESPONSE_TIMEOUT_SECS");
        std::env::remove_var("CACHE_CONNECTION_TIMEOUT_SECS");
    }

    // ── rate_limit tests — require a live Redis (REDIS_URL) ──────────────────
    // These tests are skipped when REDIS_URL is not set so they never break
    // the CI unit-test pass; set REDIS_URL to run them against a real server.

    async fn maybe_connect() -> Option<redis::aio::ConnectionManager> {
        let url = std::env::var("REDIS_URL").ok()?;
        connect(&CacheConfig::new(url)).await.ok()
    }

    #[tokio::test]
    async fn rate_limit_under_limit_allows() {
        let Some(cm) = maybe_connect().await else {
            return;
        };
        // Use a unique key to avoid test cross-contamination.
        let key = format!("test:rate_limit:under:{}", uuid_key());
        for i in 1..=5u64 {
            let r = rate_limit(&cm, &key, 10, 60).await.unwrap();
            assert!(r.allowed, "request {i} should be allowed");
            assert_eq!(r.count, i);
        }
        cleanup(&cm, &key).await;
    }

    #[tokio::test]
    async fn rate_limit_over_limit_denies() {
        let Some(cm) = maybe_connect().await else {
            return;
        };
        let key = format!("test:rate_limit:over:{}", uuid_key());
        // Fill to limit.
        for _ in 0..10u64 {
            rate_limit(&cm, &key, 10, 60).await.unwrap();
        }
        // 11th request must be denied.
        let r = rate_limit(&cm, &key, 10, 60).await.unwrap();
        assert!(!r.allowed, "11th request should be denied");
        assert_eq!(r.count, 11);
        assert_eq!(r.limit, 10);
        assert_eq!(r.retry_after_secs, 60);
        cleanup(&cm, &key).await;
    }

    #[tokio::test]
    async fn rate_limit_window_expiry_resets() {
        let Some(cm) = maybe_connect().await else {
            return;
        };
        let key = format!("test:rate_limit:expiry:{}", uuid_key());
        // Fill a 1-second window.
        for _ in 0..10u64 {
            rate_limit(&cm, &key, 10, 1).await.unwrap();
        }
        let denied = rate_limit(&cm, &key, 10, 1).await.unwrap();
        assert!(!denied.allowed, "should be denied before window resets");
        // Wait for the window to expire.
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        // New window — counter resets.
        let r = rate_limit(&cm, &key, 10, 1).await.unwrap();
        assert!(r.allowed, "should be allowed after window reset");
        assert_eq!(r.count, 1, "count should reset to 1 in new window");
        cleanup(&cm, &key).await;
    }

    // Unique suffix to keep tests isolated across concurrent runs.
    fn uuid_key() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos();
        format!("{:x}", ns)
    }

    async fn cleanup(cm: &redis::aio::ConnectionManager, key: &str) {
        let _ = del(cm, key).await;
    }
}
