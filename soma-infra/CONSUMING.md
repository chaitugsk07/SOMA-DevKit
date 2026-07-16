# Using soma-infra in Another Repo

soma-infra is the shared backend-plumbing library for the soma-platform — the backend counterpart to `soma-ui`. It is a library, not a server; it does not run anything. You pick the features you need and get back configured clients and thin primitives. All logic stays in your application: soma-infra holds the mechanism, you hold the policy.

---

## Cargo dependency

```toml
# Active development (sibling clone) — select the features your service needs:
soma-infra = { path = "../soma-infra", features = ["db", "tracing"] }

# Stable pin via git tag:
soma-infra = { git = "https://github.com/chaitugsk07/soma-infra", tag = "v0.1.0", features = ["db", "tracing"] }

# Once published to crates.io:
soma-infra = { version = "0.1", features = ["db", "tracing"] }   # once published
```

For integration tests only (in `[dev-dependencies]`):

```toml
[dev-dependencies]
soma-infra = { path = "../soma-infra", features = ["testing"] }
```

`default = ["db", "tracing"]`. All other features are opt-in. Enabling only the features your service uses keeps compile times short and dependency graphs small — a db-only consumer never resolves the LLM, crypto, Redis, or object-store graphs.

---

## Feature → module table

| feature | module | what you get | key env vars |
|---------|--------|--------------|--------------|
| `db` *(default)* | `db` | `PoolConfig` + `connect` / `connect_from_env` — a configured `sqlx` Postgres pool | `DATABASE_URL` (req), `DB_MAX_CONNECTIONS`, `DB_MIN_CONNECTIONS`, `DB_ACQUIRE_TIMEOUT_SECS`, `DB_APP_NAME` |
| `tracing` *(default)* | `telemetry` | `init()` / `init_with(..)` — `tracing-subscriber` honoring `RUST_LOG`; human or JSON output | `RUST_LOG`, `LOG_FORMAT=json` |
| `config` | `config` | `require_env` / `env_or` / `env_parse` — tiny env-var helpers | — |
| `errors` | `errors` | `redact_db_error` — a client-safe message for a `sqlx::Error` (no SQL/PII leak) | — |
| `testing` | `testing` | `TestDb` — an isolated, auto-dropped Postgres database for integration tests | `TEST_DATABASE_URL` (req) |
| `cache` | `cache` | `CacheConfig` + `connect` / `connect_from_env` — a configured Redis `ConnectionManager` (multiplexed, auto-reconnecting); thin `get`/`set`/`set_ex`/`del`/`expire` helpers | `REDIS_URL` (req), `CACHE_RESPONSE_TIMEOUT_SECS`, `CACHE_CONNECTION_TIMEOUT_SECS` |
| `crypto` | `crypto` | `CryptoKey` + `encrypt`/`decrypt` (AES-256-GCM, version-prefixed) + `hash_password`/`verify_password` (Argon2id, PHC strings) + `hkdf_sha256` / `hmac_sha256_hex` / `sha256_hex` | *(key material comes from your chosen env var — see below)* |
| `storage-s3` | `storage` | `StorageConfig` + `StorageClient` — a configured AWS S3 (or S3-compatible) handle; thin `get`/`put`/`delete` | `STORAGE_BUCKET` (req), `STORAGE_S3_REGION` (req), `STORAGE_S3_ENDPOINT` (opt); AWS credentials picked up natively by the SDK |
| `storage-azure` | `storage` | `StorageConfig` + `StorageClient` — a configured Azure Blob Storage handle; same thin API as `storage-s3` | `STORAGE_AZURE_ACCOUNT` (req), `STORAGE_AZURE_CONTAINER` (req); `AZURE_STORAGE_ACCOUNT_KEY` picked up natively |
| `llm` | `llm` | `LlmConfig` + `LlmClient` — a configured Anthropic HTTP client; typed `MessagesRequest`/`MessagesResponse` + token usage; no retry, no prompt construction | `ANTHROPIC_API_KEY` (req), `ANTHROPIC_MODEL` (req — no default), `ANTHROPIC_TIMEOUT_SECS`, `ANTHROPIC_MAX_RETRIES` |
| `http` | `http` | `client()` / `client_with_timeouts(..)` — a `reqwest::Client` with rustls TLS and sane default timeouts (30s request, 10s connect) | — |
| `kg` | `kg` | Knowledge-graph query helpers: `upsert_node`/`upsert_edge`, `neighbors`, `vector_search_cosine` (pgvector cosine-distance); the caller supplies all vectors | *(rides on your `db` pool — no additional env vars)* |
| `signal` | `signal` | `shutdown_signal()` — await Ctrl-C (SIGINT) or, on Unix, SIGTERM | — |
| `web` | `web` | axum 0.8 helpers: `serve_spa::<A: RustEmbed>(uri)` (embedded SPA + index.html fallback), `extract_bearer(header)`, `serve_with_shutdown(addr, router)` | — |
| `iam-client` | `iam_client` | `JwksConfig` + `JwksVerifier` — ES256 JWKS fetch + in-memory cache (15-min TTL, 60-s cooldown, single-flight); `verify::<C: DeserializeOwned>(token)` returns caller-supplied claims; `prefetch()` for startup warm-up. With `web` also active: `axum_ext::extract_token(headers)` (Bearer → `__Host-soma_sso` → `soma_sso` cookie) and `axum_ext::IamClaims<C>` extractor. | `SOMA_IAM_ISSUER` (req), `SOMA_IAM_JWKS_URL` (opt; default `{issuer}/.well-known/jwks.json`), `SOMA_IAM_AUDIENCE` (opt; default `"soma-console"`) |

Both `storage-s3` and `storage-azure` may be enabled simultaneously. When both are active, `StorageConfig::from_env` reads `STORAGE_BACKEND` (`"s3"` or `"azure"`) to choose which backend to configure.

---

## Per-module quick start

### db

```rust
// Reads DATABASE_URL (required) + optional DB_* overrides.
let pool = soma_infra::connect_from_env().await?;

// Or build a pool from an explicit URL:
let pool = soma_infra::connect(&soma_infra::db::PoolConfig::new("postgres://...")).await?;
```

### tracing

```rust
// Initialize at info level; RUST_LOG overrides; LOG_FORMAT=json for structured output.
soma_infra::telemetry::init();

// Or pass a custom default directive:
soma_infra::telemetry::init_with("info,sqlx=warn");
```

### cache

```rust
use soma_infra::cache;

// Reads REDIS_URL (required) + optional CACHE_*_TIMEOUT_SECS.
let cm = cache::connect_from_env().await?;

// GET / SET / SETEX / DEL / EXPIRE
let val: Option<Vec<u8>> = cache::get(&cm, "my-key").await?;
cache::set(&cm, "my-key", b"value").await?;
cache::set_ex(&cm, "session:abc", token_bytes, 3600).await?;
cache::del(&cm, "my-key").await?;
cache::expire(&cm, "my-key", 300).await?;
```

The `ConnectionManager` is multiplexed and cheaply cloneable — clone it into each handler.

### crypto

```rust
use soma_infra::crypto;

// Load a 32-byte AES-256 key from a hex-encoded env var (64 hex chars).
// The variable name is YOUR policy — soma-infra takes a &'static str.
let key = crypto::CryptoKey::from_env("CRYPTO_KEY")?;

// AES-256-GCM encrypt/decrypt. AAD binds additional context into the tag.
let ciphertext = crypto::encrypt(&key, plaintext, aad)?;
let recovered  = crypto::decrypt(&key, &ciphertext, aad)?;

// Argon2id password hashing (PHC string format).
let hash = crypto::hash_password("hunter2")?;
crypto::verify_password("hunter2", &hash)?;

// HKDF-SHA256 — caller supplies salt/info strings (those are YOUR policy).
// Returns Zeroizing<Vec<u8>>; wiped on drop.
let derived = crypto::hkdf_sha256(master_key_bytes, Some(b"my-salt-v1"), info_bytes, 32)?;

// HMAC-SHA256 → lowercase hex string.
let tag = crypto::hmac_sha256_hex(key_bytes, message);

// SHA-256 → lowercase hex string (bare digest, NOT a MAC).
let fp = crypto::sha256_hex(data);
```

The KDF parameter strings (`"my-salt-v1"`, info bytes, etc.) are policy that belongs in your service. soma-infra exposes the primitive; you supply the parameters.

### storage

```rust
use soma_infra::storage::StorageClient;

// Reads STORAGE_BUCKET + STORAGE_S3_REGION (or STORAGE_AZURE_* for Azure).
let client = StorageClient::from_env()?;

let data: bytes::Bytes = client.get("path/to/object.json").await?;
client.put("path/to/object.json", data).await?;
client.delete("path/to/object.json").await?;
```

Credentials (`AWS_ACCESS_KEY_ID` / `AZURE_STORAGE_ACCOUNT_KEY` / instance metadata) are picked up natively by the underlying `object_store` builder — do not pass them through soma-infra.

### llm

```rust
use soma_infra::llm::{LlmClient, MessagesRequest, Message, Role};

// Both ANTHROPIC_API_KEY and ANTHROPIC_MODEL are required — there is no default model.
// Silently falling back to a different model tier would be a billing hazard.
let client = LlmClient::from_env()?;

let req = MessagesRequest {
    model: std::env::var("ANTHROPIC_MODEL").unwrap(),
    max_tokens: 1024,
    system: Some("You are a helpful assistant.".into()),
    messages: vec![Message { role: Role::User, content: "Hello".into() }],
    tools: None,
};

let resp = client.messages(&req).await?;
// resp.content[0].text, resp.usage.input_tokens, resp.usage.output_tokens
```

`messages()` returns `LlmError::RateLimited` on HTTP 429/529 — it does not retry. Your service owns the retry loop; read `retry_after_secs` from the error.

### kg

```rust
use soma_infra::kg::{self, KgNode, KgEdge, Direction};
use uuid::Uuid;

// kg rides on your existing db pool — no separate connection or config.
// The consumer owns the schema and migration; copy the SQL from README.md or
// src/kg.rs into your service's migrations/ directory (soma-schema format).

let node = KgNode { id: Uuid::new_v4(), kind: "document".into(), props: serde_json::json!({}) };
kg::upsert_node(&pool, &node).await?;

let edge = KgEdge { id: Uuid::new_v4(), src: node_a, dst: node_b, rel: "references".into(), props: serde_json::json!({}) };
kg::upsert_edge(&pool, &edge).await?;

// One-hop neighbors (outgoing edges, any relation, limit 50).
let neighbors = kg::neighbors(&pool, node.id, None, Direction::Outgoing, 50).await?;

// Top-10 similar nodes by cosine distance. YOU supply the embedding vector.
let matches = kg::vector_search_cosine(&pool, pgvector::Vector::from(my_embedding), 10).await?;
```

### signal

```rust
// Axum graceful shutdown — the shutdown_signal future completes on Ctrl-C or SIGTERM.
axum::serve(listener, router)
    .with_graceful_shutdown(soma_infra::signal::shutdown_signal())
    .await?;
```

### http

```rust
// Build a reqwest::Client with rustls TLS, 30s request timeout, 10s connect timeout.
let client = soma_infra::http::client()?;

// Or specify timeouts explicitly (seconds):
let client = soma_infra::http::client_with_timeouts(60, 15)?;
```

### config

```rust
use soma_infra::config::{require_env, env_or, env_parse};

let db_url: String    = require_env("DATABASE_URL")?;        // errors if unset
let log_level: String = env_or("LOG_LEVEL", "info");        // falls back to "info"
let port: Option<u16> = env_parse("PORT")?;                 // None if unset, Err if unparseable
```

### testing

```rust
// In a #[tokio::test] — reads TEST_DATABASE_URL from the environment.
#[tokio::test]
async fn my_integration_test() -> Result<(), Box<dyn std::error::Error>> {
    let db = soma_infra::TestDb::create_from_env().await?;
    // db.pool is a PgPool connected to a freshly-created isolated database.
    // The database is automatically dropped when `db` goes out of scope (even on panic).
    sqlx::query("CREATE TABLE foo (id SERIAL PRIMARY KEY)").execute(&db.pool).await?;
    Ok(())
}
```

---

## Env vars — consolidated list

| feature | var | required | default |
|---------|-----|----------|---------|
| `db` | `DATABASE_URL` | yes | — |
| `db` | `DB_MAX_CONNECTIONS` | no | 10 |
| `db` | `DB_MIN_CONNECTIONS` | no | 0 |
| `db` | `DB_ACQUIRE_TIMEOUT_SECS` | no | 30 |
| `db` | `DB_APP_NAME` | no | — |
| `tracing` | `RUST_LOG` | no | `"info"` |
| `tracing` | `LOG_FORMAT` | no | human (set to `json` for structured) |
| `testing` | `TEST_DATABASE_URL` | yes | — |
| `cache` | `REDIS_URL` | yes | — |
| `cache` | `CACHE_RESPONSE_TIMEOUT_SECS` | no | none (no timeout) |
| `cache` | `CACHE_CONNECTION_TIMEOUT_SECS` | no | none (no timeout) |
| `crypto` | *(your chosen var name)* | yes | — (pass name to `CryptoKey::from_env`) |
| `storage-s3` | `STORAGE_BUCKET` | yes | — |
| `storage-s3` | `STORAGE_S3_REGION` | yes | — |
| `storage-s3` | `STORAGE_S3_ENDPOINT` | no | — (for S3-compatible stores) |
| `storage-s3` | `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` | via SDK | — (read natively by object_store) |
| `storage-azure` | `STORAGE_AZURE_ACCOUNT` | yes | — |
| `storage-azure` | `STORAGE_AZURE_CONTAINER` | yes | — |
| `storage-azure` | `AZURE_STORAGE_ACCOUNT_KEY` | via SDK | — (read natively by object_store) |
| `storage-s3` + `storage-azure` | `STORAGE_BACKEND` | yes | — (`"s3"` or `"azure"`) |
| `llm` | `ANTHROPIC_API_KEY` | yes | — |
| `llm` | `ANTHROPIC_MODEL` | yes | — (no default — intentional) |
| `llm` | `ANTHROPIC_TIMEOUT_SECS` | no | 30 |
| `llm` | `ANTHROPIC_MAX_RETRIES` | no | 3 (stored in config; `messages()` does not retry) |

---

## The one rule

soma-infra ships the **mechanism**, never the **policy**. Before calling anything here, understand the split:

- soma-infra handles: pool builder, TLS, timeouts, AES-256-GCM wire format, Argon2id PHC strings, HKDF/HMAC/SHA-256 math, object-store auth wiring, HTTP client construction, Anthropic wire protocol, pgvector query helpers, SIGTERM plumbing.
- Your service handles: KDF salt/info strings, which fields to encrypt, SQLSTATE→domain-error mapping, migration schema and advisory lock key, prompt construction and retry logic, object key naming and content-type, rate-limit backoff strategy, what to log on shutdown, embedding generation.

If a function makes a decision the service should own, it does not belong in soma-infra. Do not re-implement a pool, telemetry init, crypto primitive, or HTTP client in your application.

---

## kg — consumer-owned migration

soma-infra ships only the Rust query helpers for the knowledge graph. The schema and migration belong to the consuming service. Copy the following SQL blocks into your service's `migrations/` directory (soma-schema UP/DOWN format) and adjust the `vector(1536)` dimension to match your embedding model:

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
-- This value cannot change after rows exist without dropping and recreating the table.
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

---

## License

Apache-2.0.
