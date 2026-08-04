//! Postgres connection pooling on `sqlx`.

use std::str::FromStr;
use std::time::Duration;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;

/// Tunables for a Postgres connection pool. Build from an explicit URL with
/// [`PoolConfig::new`] or from the environment with [`PoolConfig::from_env`].
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Connection URL, e.g. `postgres://user:pass@host:5432/db?sslmode=require`.
    pub url: String,
    /// Max pooled connections. Raised to at least 2 on connect (the migration
    /// runner needs a spare connection for its advisory lock).
    pub max_connections: u32,
    /// Min idle connections kept warm.
    pub min_connections: u32,
    /// How long to wait for a free connection before erroring.
    pub acquire_timeout: Duration,
    /// Close a connection after this much idle time.
    pub idle_timeout: Option<Duration>,
    /// Recycle a connection after this much wall-clock age.
    pub max_lifetime: Option<Duration>,
    /// `application_name` reported to Postgres (shows up in `pg_stat_activity`).
    pub application_name: Option<String>,
}

impl PoolConfig {
    /// A config with sane defaults for the given URL.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            max_connections: 10,
            min_connections: 0,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Some(Duration::from_secs(600)),
            max_lifetime: Some(Duration::from_secs(1800)),
            application_name: None,
        }
    }

    /// Read `DATABASE_URL` (required) plus optional overrides
    /// `DB_MAX_CONNECTIONS`, `DB_MIN_CONNECTIONS`, `DB_ACQUIRE_TIMEOUT_SECS`,
    /// `DB_APP_NAME`.
    pub fn from_env() -> Result<Self, DbError> {
        let url = std::env::var("DATABASE_URL").map_err(|_| DbError::MissingEnv("DATABASE_URL"))?;
        let mut cfg = Self::new(url);
        if let Ok(v) = std::env::var("DB_MAX_CONNECTIONS") {
            cfg.max_connections = v
                .parse()
                .map_err(|_| DbError::InvalidEnv("DB_MAX_CONNECTIONS"))?;
        }
        if let Ok(v) = std::env::var("DB_MIN_CONNECTIONS") {
            cfg.min_connections = v
                .parse()
                .map_err(|_| DbError::InvalidEnv("DB_MIN_CONNECTIONS"))?;
        }
        if let Ok(v) = std::env::var("DB_ACQUIRE_TIMEOUT_SECS") {
            let secs: u64 = v
                .parse()
                .map_err(|_| DbError::InvalidEnv("DB_ACQUIRE_TIMEOUT_SECS"))?;
            cfg.acquire_timeout = Duration::from_secs(secs);
        }
        if let Ok(v) = std::env::var("DB_APP_NAME") {
            cfg.application_name = Some(v);
        }
        Ok(cfg)
    }
}

/// Errors from building or connecting a pool.
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    /// A required environment variable was missing.
    #[error("required env var {0} is not set")]
    MissingEnv(&'static str),
    /// An environment variable held an unparseable value.
    #[error("env var {0} has an invalid value")]
    InvalidEnv(&'static str),
    /// The underlying sqlx error.
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

// ponytail: these pool sizes are a sane service default. Tune `max_connections`
// per workload and keep the fleet-wide total under Postgres `max_connections`
// minus headroom. `sslmode=require` (Azure) works as-is via the URL + tls-rustls.
/// Build a **lazy** pool from a [`PoolConfig`].
///
/// No TCP connection is opened — the first actual query opens the connection.
/// This removes all Postgres network I/O from the boot path, so the process
/// can listen for requests before the DB round-trip completes.
///
/// Migration runners need a direct (non-lazy) connection; call
/// `connect_direct` or `PgPoolOptions::connect_with` explicitly for that.
pub async fn connect(cfg: &PoolConfig) -> Result<PgPool, DbError> {
    let mut opts = PgConnectOptions::from_str(&cfg.url)?;
    if let Some(app) = &cfg.application_name {
        opts = opts.application_name(app);
    }
    // connect_lazy_with: validates options but opens no TCP connection.
    // The pool acquires a real connection on the first query.
    let pool = PgPoolOptions::new()
        .max_connections(cfg.max_connections.max(2))
        .min_connections(cfg.min_connections)
        .acquire_timeout(cfg.acquire_timeout)
        .idle_timeout(cfg.idle_timeout)
        .max_lifetime(cfg.max_lifetime)
        .connect_lazy_with(opts);
    Ok(pool)
}

/// Convenience: [`PoolConfig::from_env`] then [`connect`].
pub async fn connect_from_env() -> Result<PgPool, DbError> {
    connect(&PoolConfig::from_env()?).await
}

/// Return `true` when `schema."00_schema_migrations"` exists and has at least
/// one row — a quick, DDL-free readiness check.
///
/// Returns `false` when the table does not exist (migrations not yet applied)
/// or when the table is empty.  Returns `Err` on unexpected DB errors.
pub async fn schema_migrations_applied(pool: &PgPool, schema: &str) -> Result<bool, sqlx::Error> {
    let sql = format!(
        r#"SELECT COUNT(*) FROM "{schema}"."00_schema_migrations""#,
        schema = schema
    );
    match sqlx::query_scalar::<_, i64>(&sql).fetch_one(pool).await {
        Ok(n) => Ok(n > 0),
        Err(e) => {
            // 42P01 = "undefined_table" — migrations not applied yet, not an error
            if let sqlx::Error::Database(ref de) = e {
                if de.code().as_deref() == Some("42P01") {
                    return Ok(false);
                }
            }
            Err(e)
        }
    }
}
