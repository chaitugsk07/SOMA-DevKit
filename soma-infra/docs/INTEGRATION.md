# Add soma-infra to your Rust application

soma-infra is the shared backend-plumbing library for the soma-platform. It is a
**library, not a server** — it does not listen, spawn background tasks, or own
any business logic. You pick the features your service uses and get back
configured clients and thin primitives. All logic stays in your application;
soma-infra holds the mechanism.

This guide is the canonical how-to for integrating soma-infra into a Rust
service. It covers every feature, every real API call, and the invariants you
must hold. See [CONSUMING.md](../CONSUMING.md) for the condensed quick-reference
table; this document goes deeper on per-feature gotchas, env vars, and the line
between plumbing and policy.

---

## 1. Adding the dependency

soma-infra v0.1.1 is published to [crates.io](https://crates.io/crates/soma-infra).

```toml
# Cargo.toml — stable consumers
[dependencies]
soma-infra = { version = "0.1", features = ["db", "tracing"] }
```

For integration tests only:

```toml
[dev-dependencies]
soma-infra = { version = "0.1", features = ["testing"] }
```

**Monorepo / active development** — use a path dep instead; the version dep
form is only needed by external consumers or when pinning a tag:

```toml
soma-infra = { path = "../soma-infra", features = ["db", "tracing"] }
```

Git tag form (pinned, not yet on crates.io or wanting a specific patch):

```toml
soma-infra = { git = "https://github.com/chaitugsk07/soma-infra", tag = "v0.1.1", features = ["db", "tracing"] }
```

The `version` and `path` (or `git`) forms may coexist in the same declaration:
the path wins in local dev, the version is used by published downstream
consumers.

---

## 2. Choosing features

`default = ["db", "tracing"]`. Every other feature is opt-in.

| feature | module | what you get |
|---------|--------|--------------|
| `db` *(default)* | `db` | `PoolConfig` + `connect` / `connect_from_env` — configured `sqlx` Postgres pool |
| `tracing` *(default)* | `telemetry` | `init()` / `init_with(..)` — `tracing-subscriber` with `RUST_LOG`, human or JSON |
| `config` | `config` | `require_env` / `env_or` / `env_parse` — tiny env-var helpers |
| `errors` | `errors` | `redact_db_error` — client-safe message from a `sqlx::Error` |
| `testing` | `testing` | `TestDb` — isolated, auto-dropped Postgres DB for integration tests |
| `cache` | `cache` | `CacheConfig` + `connect` / `connect_from_env` — Redis `ConnectionManager`; thin `get`/`set`/`set_ex`/`del`/`expire` |
| `crypto` | `crypto` | `CryptoKey` + `encrypt`/`decrypt` (AES-256-GCM) + `hash_password`/`verify_password` (Argon2id) + `hkdf_sha256` / `hmac_sha256_hex` / `sha256_hex` |
| `storage-s3` | `storage` | `StorageConfig` + `StorageClient` — AWS S3 (or S3-compatible); thin `get`/`put`/`delete` |
| `storage-azure` | `storage` | `StorageConfig` + `StorageClient` — Azure Blob Storage; same thin API |
| `llm` | `llm` | `LlmConfig` + `LlmClient` — Anthropic HTTP client; typed request/response + token usage |
| `http` | `http` | `client()` / `client_with_timeouts(..)` — `reqwest::Client` with rustls TLS and sane timeouts |
| `kg` | `kg` | Knowledge-graph query helpers: `upsert_node`/`upsert_edge`, `neighbors`, `vector_search_cosine` |
| `signal` | `signal` | `shutdown_signal()` — await Ctrl-C or SIGTERM |

**Rule:** enable only what you use. A db-only consumer must never resolve the
LLM, crypto, Redis, or object-store dependency graphs. Every feature gates its
dependencies independently.

---

## 3. db — Postgres pool

### Environment variables

| var | required | default |
|-----|----------|---------|
| `DATABASE_URL` | yes | — |
| `DB_MAX_CONNECTIONS` | no | 10 |
| `DB_MIN_CONNECTIONS` | no | 0 |
| `DB_ACQUIRE_TIMEOUT_SECS` | no | 30 |
| `DB_APP_NAME` | no | — (sets `application_name` in `pg_stat_activity`) |

### Quick start

```rust
// Reads DATABASE_URL (required) + optional DB_* overrides.
let pool = soma_infra::connect_from_env().await?;

// Or build from an explicit URL with sane defaults, then override:
use soma_infra::db::PoolConfig;
let mut cfg = PoolConfig::new("postgres://user:pass@host/db?sslmode=require");
cfg.max_connections = 20;
let pool = soma_infra::connect(&cfg).await?;
```

### Gotchas

- **Pool size must be ≥ 2** when you also use soma-schema's migration runner
  (which holds an advisory lock for the duration of each migration run). One
  connection holds the lock; at least one more is needed for actual queries.
  This is checked by soma-schema, not soma-infra — but size your pool
  accordingly at startup.
- `PoolConfig::new` defaults to 10 connections. Raise `DB_MAX_CONNECTIONS` for
  high-concurrency services; lower it for services sharing a small database
  cluster.
- `application_name` (`DB_APP_NAME`) shows up in `pg_stat_activity`. Set it to
  the service name to make slow-query diagnosis less painful.

---

## 4. tracing — telemetry init

### Environment variables

| var | required | default |
|-----|----------|---------|
| `RUST_LOG` | no | `"info"` |
| `LOG_FORMAT` | no | human-readable (set to `json` for structured output) |

### Quick start

```rust
// Initialize at info level; RUST_LOG overrides; LOG_FORMAT=json for structured.
soma_infra::telemetry::init();

// Or pass a custom default directive (overridden by RUST_LOG at runtime):
soma_infra::telemetry::init_with("info,sqlx=warn,hyper=warn");
```

Call this once, early in `main`, before any `tracing::info!` / `warn!` / etc.
calls. Calling it a second time is a no-op (subscriber is already set globally).

### Gotchas

- `LOG_FORMAT=json` emits each log line as a JSON object — pipe it to your log
  aggregator. The human format is readable in a terminal but unsuitable for
  structured log parsing.
- `RUST_LOG` follows the standard `tracing-subscriber` env-filter syntax:
  `info,soma_infra::db=debug,sqlx=warn` is a valid value.
- `telemetry::init()` does NOT set up metrics, distributed tracing spans, or
  OpenTelemetry exporters. It is a local subscriber only (stderr or stdout).

---

## 5. config — env helpers

No env vars. No external dependencies. Import the three functions you need:

```rust
use soma_infra::config::{require_env, env_or, env_parse};

// Fails with ConfigError if DATABASE_URL is not set.
let db_url: String = require_env("DATABASE_URL")?;

// Falls back to "info" if LOG_LEVEL is not set.
let level: String = env_or("LOG_LEVEL", "info");

// Returns None if PORT is not set; Err if it's set but not parseable as u16.
let port: Option<u16> = env_parse("PORT")?;
```

Use `require_env` at startup for mandatory config — you want a clear error
message before the service starts doing any work. Use `env_parse` for optional
numeric values.

---

## 6. errors — safe DB error redaction

```rust
use soma_infra::errors::redact_db_error;
use axum::http::StatusCode;

let result = sqlx::query("...").execute(&pool).await;
match result {
    Ok(_) => { /* proceed */ }
    Err(e) => {
        // Returns a client-safe string — no SQL fragment, no column name, no PII.
        let safe_msg = redact_db_error(&e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, safe_msg));
    }
}
```

`redact_db_error` maps the `sqlx::Error` variants to short, generic strings
(`"not found"`, `"conflict"`, `"database error"`, etc.). It never passes
through the raw database error message, which may contain SQL fragments or
column names.

---

## 7. testing — integration test harness

### Environment variables

| var | required | default |
|-----|----------|---------|
| `TEST_DATABASE_URL` | yes | — |

```rust
use soma_infra::TestDb;

#[tokio::test]
async fn my_integration_test() -> Result<(), Box<dyn std::error::Error>> {
    // Creates a uniquely-named Postgres database; db.pool is connected to it.
    let db = TestDb::create_from_env().await?;

    // Run your service's migrations against the test database.
    // db.pool is a standard PgPool.
    sqlx::query("CREATE TABLE documents (id UUID PRIMARY KEY)")
        .execute(&db.pool)
        .await?;

    // The database is automatically dropped when `db` goes out of scope,
    // even on panic. No manual cleanup needed.
    Ok(())
}
```

`TestDb::create` accepts an explicit URL if you want to use a different base
URL than `TEST_DATABASE_URL`:

```rust
let db = TestDb::create("postgres://user:pass@localhost/postgres").await?;
```

### Gotchas

- `TestDb` only creates the database and hands you a pool. Running your
  service's migrations against `db.pool` is your responsibility — soma-infra
  does not know your schema.
- `TEST_DATABASE_URL` should point to any accessible database on the target
  server (e.g. `postgres://user:pass@localhost/postgres`). The test harness
  swaps the database component to create and drop the throwaway database.
- Multiple `TestDb` instances in one test run are fine — each gets a unique
  name (`soma_test_<uuid>`).
- `Drop` runs `DROP DATABASE` synchronously. In async tests with `tokio::test`,
  this works correctly because `Drop` is called before the executor shuts down.

---

## 8. cache — Redis

### Environment variables

| var | required | default |
|-----|----------|---------|
| `REDIS_URL` | yes | — |
| `CACHE_RESPONSE_TIMEOUT_SECS` | no | none (no timeout) |
| `CACHE_CONNECTION_TIMEOUT_SECS` | no | none (no timeout) |

```rust
use soma_infra::cache;

// Reads REDIS_URL + optional timeout vars.
let cm = cache::connect_from_env().await?;

// Or explicit URL with a custom config:
let cfg = cache::CacheConfig::new("redis://127.0.0.1:6379");
let cm = cache::connect(&cfg).await?;

// Operations — all values are raw bytes; serialization belongs in your service.
let val: Option<Vec<u8>> = cache::get(&cm, "session:abc").await?;
cache::set(&cm, "session:abc", b"token_bytes").await?;
cache::set_ex(&cm, "session:abc", b"token_bytes", 3600).await?;  // TTL in seconds
cache::del(&cm, "session:abc").await?;
cache::expire(&cm, "session:abc", 300).await?;   // update TTL in seconds
```

The `ConnectionManager` is multiplexed and cheaply cloneable — clone it into
each request handler:

```rust
// In your app state:
#[derive(Clone)]
struct AppState {
    cm: redis::aio::ConnectionManager,
}

// In a handler:
async fn my_handler(State(state): State<AppState>) {
    let val = cache::get(&state.cm, "key").await?;
}
```

### Gotchas

- The cache layer stores and returns raw bytes (`Vec<u8>`). Serialization
  (JSON, MessagePack, etc.) and key prefixing are your responsibility.
- Neither `CACHE_RESPONSE_TIMEOUT_SECS` nor `CACHE_CONNECTION_TIMEOUT_SECS`
  default to a timeout. In production, set at least `CACHE_RESPONSE_TIMEOUT_SECS`
  to avoid unbounded blocking on a dead Redis node.
- `ConnectionManager` auto-reconnects on transient failures. You do not need to
  rebuild it; just clone the handle and call the operation again.
- There is no built-in key expiry default. Always pass a TTL via `set_ex` for
  session and token keys.

---

## 9. crypto — AES-256-GCM, Argon2id, and KDF primitives

The crypto module is consumed by soma-vault and soma-audit for real encryption
and key derivation. It has **golden-vector tests** pinning its output to the
services' expected bytes. The wire format must not change.

### Loading a key

```rust
use soma_infra::crypto::{self, CryptoKey};

// Load a 32-byte AES-256 key from a hex-encoded env var (exactly 64 hex chars).
// The variable name is YOUR policy — soma-infra takes a &'static str.
let key = CryptoKey::from_env("MY_ENCRYPTION_KEY")?;

// Or from raw bytes (e.g. a vault-sourced secret):
let key = CryptoKey::from_bytes(&my_32_bytes)?;
```

Generate a key with: `openssl rand -hex 32`

`CryptoKey` is zeroized on drop and prints `CryptoKey(***)` via `Debug`.

### AES-256-GCM encrypt / decrypt

```rust
// AAD binds additional context into the GCM tag (e.g. tenant_id, resource_type).
// Decryption fails if AAD does not match exactly.
let ciphertext = crypto::encrypt(&key, plaintext_bytes, aad)?;
let plaintext   = crypto::decrypt(&key, &ciphertext, aad)?;
```

Wire format: `0x01 (version byte) || nonce (12 bytes) || ciphertext || GCM tag (16 bytes)`.

The version byte is checked on decrypt — `CryptoError::UnsupportedVersion` is
returned for anything other than `0x01`.

### Argon2id password hashing

```rust
// Hash — returns a PHC-format string safe to store verbatim.
let hash = crypto::hash_password("hunter2")?;

// Verify — Ok(()) on match, CryptoError::InvalidPassword on mismatch.
crypto::verify_password("hunter2", &hash)?;
```

Argon2::default() uses OWASP-recommended minimums (m=19456 KiB, t=2, p=1).
Adjust only if profiling shows the latency is unacceptable for your auth path.

### HKDF-SHA256 key derivation

```rust
// Derive a 32-byte subkey. Returns Zeroizing<Vec<u8>> (wiped on drop).
// The salt/info strings are YOUR policy — keep them in your service.
let derived = crypto::hkdf_sha256(
    master_key_bytes,
    Some(b"my-service-kdf-v1"),  // salt — None means no salt
    info_bytes,                   // context (e.g. tenant_id bytes)
    32,
)?;
```

**Critical:** The salt and info strings (e.g. `b"soma-vault-tenant-kek-v1"`,
`b"soma-audit-hmac-v1"`) are domain policy that belongs in the calling service,
not in soma-infra. soma-infra runs the math; you supply the parameters.

### HMAC-SHA256

```rust
// Returns a lowercase 64-char hex string.
let tag = crypto::hmac_sha256_hex(key_bytes, message_bytes);
```

### SHA-256

```rust
// Returns a lowercase 64-char hex string. A bare digest, NOT a MAC.
// Use for content fingerprinting and token-hash storage.
let fp = crypto::sha256_hex(data);
```

Use `hmac_sha256_hex` when authenticating a value under a secret key.
Use `sha256_hex` for content fingerprinting only.

### Gotchas

- **Do not change the wire format.** Any change to `encrypt`/`decrypt` or the
  KDF parameters breaks soma-vault's and soma-audit's stored ciphertexts.
  Golden-vector tests in the test suite pin the output — they must pass before
  any crypto change ships.
- **`hkdf_sha256` ceiling.** The HKDF-SHA256 ceiling is 255 × 32 = 8160 bytes.
  Requesting more returns `CryptoError::Hkdf`. In practice you'll never hit
  this for key derivation.
- **No key rotation, no envelope wrapping.** If you need key rotation, build a
  version envelope in your service (store the key version alongside the
  ciphertext and dispatch on it before calling `decrypt`).

---

## 10. storage — S3 and Azure Blob

### Environment variables

| var | feature | required | notes |
|-----|---------|----------|-------|
| `STORAGE_BUCKET` | `storage-s3` | yes | S3 bucket name |
| `STORAGE_S3_REGION` | `storage-s3` | yes | e.g. `us-east-1` |
| `STORAGE_S3_ENDPOINT` | `storage-s3` | no | For S3-compatible stores (MinIO, etc.) |
| `STORAGE_AZURE_ACCOUNT` | `storage-azure` | yes | Azure storage account name |
| `STORAGE_AZURE_CONTAINER` | `storage-azure` | yes | Blob container name |
| `STORAGE_BACKEND` | both enabled | yes | `"s3"` or `"azure"` |

Credentials (`AWS_ACCESS_KEY_ID`/`AWS_SECRET_ACCESS_KEY` for S3,
`AZURE_STORAGE_ACCOUNT_KEY` for Azure) are read natively by the `object_store`
builders — do not pass them through soma-infra.

```rust
use soma_infra::storage::StorageClient;

// Reads env vars for the configured backend.
let client = StorageClient::from_env()?;

let data: bytes::Bytes = client.get("objects/doc-123.json").await?;
client.put("objects/doc-123.json", data).await?;
client.delete("objects/doc-123.json").await?;
```

Or build from an explicit config:

```rust
use soma_infra::storage::StorageConfig;

let cfg = StorageConfig::new_s3("my-bucket", "us-east-1", None);
let client = StorageClient::new(cfg)?;
```

### When both features are enabled

`StorageConfig` is an enum with one variant per backend (`S3` / `Azure`). Each
variant carries only the fields for its backend — no zero-filling. Use the
`STORAGE_BACKEND` env var to select which backend `from_env` constructs:

```sh
STORAGE_BACKEND=s3    # → StorageConfig::S3 { bucket, region, endpoint }
STORAGE_BACKEND=azure # → StorageConfig::Azure { account, container }
```

### Gotchas

- **Object keys are paths, not URLs.** Pass `"prefix/object.json"`, not
  `"https://my-bucket.s3.amazonaws.com/prefix/object.json"`. Key naming and
  prefix conventions belong in your service.
- **No retry policy exposed.** `object_store` retries internally (exponential
  backoff on transient errors). You do not need to add a retry layer on top
  for typical usage.
- **No presigned URLs, no listing, no multipart.** These belong in your
  service using `object_store` directly if needed.
- **`object_store` is pinned to `0.12.x`** in this crate (rustc 1.82
  compatibility). If you need a newer version's API, depend on `object_store`
  directly alongside soma-infra.

---

## 11. llm — Anthropic client

### Environment variables

| var | required | default | notes |
|-----|----------|---------|-------|
| `ANTHROPIC_API_KEY` | yes | — | Bearer credential |
| `ANTHROPIC_MODEL` | yes | — | **No default — intentional** |
| `ANTHROPIC_TIMEOUT_SECS` | no | 30 | Per-request HTTP timeout |
| `ANTHROPIC_MAX_RETRIES` | no | 3 | Config knob only; `messages()` does not retry |

**There is no default model.** `LlmConfig::from_env` requires `ANTHROPIC_MODEL`
explicitly. Silently falling back to a different model tier would be a billing
hazard — you must name the model in your service's configuration.

```rust
use soma_infra::llm::{LlmClient, LlmConfig, MessagesRequest, Message, Role};

// Both ANTHROPIC_API_KEY and ANTHROPIC_MODEL are required.
let client = LlmClient::from_env()?;

// Or build from an explicit config:
let cfg = LlmConfig::new("sk-ant-...", "claude-sonnet-4-6");
let client = LlmClient::new(cfg)?;

let req = MessagesRequest {
    model: std::env::var("ANTHROPIC_MODEL").unwrap(),
    max_tokens: 1024,
    system: Some("You are a helpful assistant.".into()),
    messages: vec![Message { role: Role::User, content: "Summarize this document.".into() }],
    tools: None,
};

let resp = client.messages(&req).await?;
// resp.content[0].text  — the assistant's response text
// resp.usage.input_tokens, resp.usage.output_tokens — token counts for billing
```

### Rate limiting

`messages()` returns `LlmError::RateLimited { retry_after_secs: Option<u64> }`
on HTTP 429 or 529. It does **not** sleep or retry. Your service owns the retry
loop:

```rust
use soma_infra::llm::LlmError;

loop {
    match client.messages(&req).await {
        Ok(resp) => break resp,
        Err(LlmError::RateLimited { retry_after_secs }) => {
            let wait = retry_after_secs.unwrap_or(5);
            tokio::time::sleep(std::time::Duration::from_secs(wait)).await;
        }
        Err(e) => return Err(e.into()),
    }
}
```

### Gotchas

- **ANTHROPIC_MODEL is required, no default.** Set it in your service's env
  or deployment config. Using the wrong model tier can have significant billing
  impact; soma-infra forces you to be explicit.
- **No streaming.** `messages()` awaits the complete response. Streaming is
  deferred until consuming services have a concrete need for it.
- **No prompt construction, no tool dispatch, no agent loops.** Those belong in
  your service. soma-infra sends the request you build and hands back the
  typed response.
- **`max_retries` in `LlmConfig` is a config knob only.** Store it in your app
  state and read it in your retry loop; `messages()` itself never uses it.

---

## 12. http — reqwest client

```rust
// Build a reqwest::Client with rustls TLS, 30s request timeout, 10s connect timeout.
let client = soma_infra::http::client()?;

// Or specify explicit timeouts (seconds):
let client = soma_infra::http::client_with_timeouts(60, 15)?;
```

Use this when you need to call external HTTP APIs from your service. The client
uses rustls (no OpenSSL dependency). All request building, auth headers, base
URLs, retry logic, and response parsing belong in your service.

---

## 13. kg — knowledge-graph query helpers

### Consumer-owned migration

soma-infra ships only the Rust query helpers. The schema and migration belong
to the consuming service. Copy the SQL below into your service's `migrations/`
directory (soma-schema UP/DOWN format). Adjust `vector(1536)` to match your
embedding model's output dimension — **this value cannot change after rows
exist** without dropping and recreating the table.

```sql
-- UP
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE kg_nodes (
    id    uuid    PRIMARY KEY,
    kind  text    NOT NULL,
    props jsonb   NOT NULL DEFAULT '{}'
);

CREATE TABLE kg_edges (
    id    uuid PRIMARY KEY,
    src   uuid NOT NULL REFERENCES kg_nodes(id) ON DELETE CASCADE,
    dst   uuid NOT NULL REFERENCES kg_nodes(id) ON DELETE CASCADE,
    rel   text NOT NULL,
    props jsonb NOT NULL DEFAULT '{}'
);

CREATE INDEX kg_edges_src_idx ON kg_edges(src);
CREATE INDEX kg_edges_dst_idx ON kg_edges(dst);

-- 1536 is a PLACEHOLDER — choose the dimension your embedding model produces.
CREATE TABLE kg_node_embeddings (
    node_id   uuid PRIMARY KEY REFERENCES kg_nodes(id) ON DELETE CASCADE,
    embedding vector(1536) NOT NULL
);

CREATE INDEX kg_node_embeddings_cosine_idx
    ON kg_node_embeddings USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
```

```sql
-- DOWN
DROP INDEX IF EXISTS kg_node_embeddings_cosine_idx;
DROP TABLE IF EXISTS kg_node_embeddings;
DROP INDEX IF EXISTS kg_edges_dst_idx;
DROP INDEX IF EXISTS kg_edges_src_idx;
DROP TABLE IF EXISTS kg_edges;
DROP TABLE IF EXISTS kg_nodes;
-- Intentionally NOT dropping the vector extension (destructive across a shared DB).
```

### Using the helpers

```rust
use soma_infra::kg::{self, KgNode, KgEdge, Direction};
use uuid::Uuid;
use serde_json::json;

// kg rides on your existing db pool — no separate connection or config.
let node_a = KgNode {
    id: Uuid::new_v4(),
    kind: "document".into(),
    props: json!({ "title": "Introduction" }),
};
kg::upsert_node(&pool, &node_a).await?;

let node_b = KgNode { id: Uuid::new_v4(), kind: "section".into(), props: json!({}) };
kg::upsert_node(&pool, &node_b).await?;

let edge = KgEdge {
    id: Uuid::new_v4(),
    src: node_a.id,
    dst: node_b.id,
    rel: "contains".into(),
    props: json!({}),
};
kg::upsert_edge(&pool, &edge).await?;

// One-hop neighbors in the outgoing direction, any relation, limit 50.
let neighbors = kg::neighbors(&pool, node_a.id, None, Direction::Outgoing, 50).await?;
// Filter by relation:
let children = kg::neighbors(&pool, node_a.id, Some("contains"), Direction::Outgoing, 50).await?;

// Top-10 similar nodes by cosine distance. YOU supply the embedding vector.
// Embeddings must already be stored in kg_node_embeddings.
let my_embedding: Vec<f32> = embed_text("search query").await?;
let matches = kg::vector_search_cosine(&pool, pgvector::Vector::from(my_embedding), 10).await?;
```

### Gotchas

- **Consumer owns the migration.** soma-infra ships no migration SQL as
  executable code. The advisory lock key and migration runner belong to your
  service. Run the UP migration block above via your own soma-schema setup.
- **Embedding dimension is baked in at migration time.** `vector(1536)` must
  match your embedding model exactly. Changing it later requires dropping and
  recreating `kg_node_embeddings`.
- **You supply all vectors.** soma-infra does not generate embeddings, call any
  embedding API, or run any ingestion pipeline. The `vector_search_cosine`
  function takes a pre-computed `pgvector::Vector` and queries the index.
- **IVFFlat index requires training.** An empty table will not benefit from
  the ivfflat index until enough rows exist. For small datasets (< 1000 rows),
  the index may perform worse than a sequential scan — this is expected behavior
  from pgvector.

---

## 14. signal — graceful shutdown

```rust
// Await Ctrl-C (SIGINT) or, on Unix, SIGTERM.
// Returns when either signal is received.
soma_infra::signal::shutdown_signal().await;

// Typical usage with axum:
axum::serve(listener, router)
    .with_graceful_shutdown(soma_infra::signal::shutdown_signal())
    .await?;
```

`shutdown_signal()` does not log anything. If you want to log on shutdown, do
it in your service after the future resolves:

```rust
soma_infra::signal::shutdown_signal().await;
tracing::info!("shutdown signal received, draining connections");
```

---

## 15. Complete startup example

A service using db, tracing, config, errors, cache, crypto, and signal:

```rust
use soma_infra::{self as infra, telemetry, cache, crypto};
use soma_infra::config::require_env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Telemetry first — so every subsequent log call is captured.
    telemetry::init_with("info,sqlx=warn");

    // 2. Database pool.
    let pool = infra::connect_from_env().await?;

    // 3. Redis cache.
    let cache_mgr = cache::connect_from_env().await?;

    // 4. Encryption key (variable name is YOUR policy).
    let crypto_key = Arc::new(crypto::CryptoKey::from_env("MY_ENCRYPTION_KEY")?);

    // 5. Any service-specific config.
    let service_name: String = require_env("SERVICE_NAME")?;

    tracing::info!(service = %service_name, "startup complete");

    // ... build your router, run your server ...

    // 6. Graceful shutdown.
    infra::signal::shutdown_signal().await;
    tracing::info!("shutting down");

    Ok(())
}
```

---

## 16. The plumbing-vs-logic line

soma-infra holds the **mechanism**. Your service holds the **policy**.

**soma-infra handles:**
- Pool builder (connection string, TLS, timeouts, `application_name`)
- `tracing-subscriber` init (RUST_LOG, JSON format switch)
- AES-256-GCM wire format, nonce generation, version prefix
- Argon2id PHC string format and OWASP default parameters
- HKDF-SHA256 expand, HMAC-SHA256 tag, SHA-256 digest → hex
- Redis `ConnectionManager` multiplexing and reconnection
- `object_store` builder for S3 and Azure (auth wiring, endpoint config)
- Anthropic HTTP wire protocol, typed request/response structs
- `reqwest::Client` construction (rustls, default timeouts)
- pgvector cosine-distance query helpers
- SIGTERM / SIGINT signal handling

**Your service handles:**
- KDF salt and info strings (e.g. `b"my-service-kdf-v1"`, `b"tenant-kek"`)
- Which fields to encrypt and what to use as AAD
- SQLSTATE → domain error mapping (`redact_db_error` gives you the safe string;
  your service maps it to an HTTP status)
- Migration schema name and advisory lock key (soma-schema wiring)
- Prompt construction, tool dispatch, retry/backoff logic for LLM calls
- Object key naming and prefix conventions
- Rate-limit backoff strategy
- What to log on shutdown
- Embedding generation

If a function makes a decision the service should own, it does not belong in
soma-infra. Do not re-implement a pool, telemetry init, crypto primitive, or
HTTP client in your application.
