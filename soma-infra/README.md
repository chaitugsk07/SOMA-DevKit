# soma-infra

Shared utilities for the [soma-platform](https://github.com/chaitugsk07) Rust repos. One crate, but every concern sits behind its own cargo feature, so a repo compiles only what it uses and a change in one module never forces unrelated consumers to rebuild.

| feature | module | what you get |
|---------|--------|--------------|
| `db` *(default)* | `db` | `PoolConfig` + `connect`/`connect_from_env` — a configured `sqlx` Postgres pool (size, timeouts, `application_name`, `sslmode=require` for Azure). |
| `tracing` *(default)* | `telemetry` | `init()` / `init_with(..)` — `tracing-subscriber` honoring `RUST_LOG`, human or `LOG_FORMAT=json`. |
| `config` | `config` | `require_env` / `env_or` / `env_parse` — tiny env helpers. |
| `errors` | `errors` | `redact_db_error` — a client-safe message for a `sqlx::Error` (no SQL/PII leak). |
| `testing` | `testing` | `TestDb` — an isolated, auto-dropped Postgres database for integration tests. |
| `cache` | `cache` | `CacheConfig` + `connect`/`connect_from_env` — a configured Redis `ConnectionManager` (multiplexed, auto-reconnecting). Thin GET/SET/SETEX/DEL/EXPIRE helpers; caller owns all cache logic. |
| `crypto` | `crypto` | `CryptoKey` + `encrypt`/`decrypt` (AES-256-GCM, version-prefixed wire format) + `hash_password`/`verify_password` (Argon2id, PHC strings). Primitives only — no key rotation, no envelope wrapping. |
| `storage-s3` | `storage` | `StorageConfig` + `StorageClient` — a configured AWS S3 (or S3-compatible) object-store handle. Thin `get`/`put`/`delete`; caller owns path naming and content-type. |
| `storage-azure` | `storage` | `StorageConfig` + `StorageClient` — a configured Azure Blob Storage handle. Same thin API as `storage-s3`. |
| `llm` | `llm` | `LlmConfig` + `LlmClient` — a configured Anthropic HTTP client. Typed `MessagesRequest`/`MessagesResponse` + token usage. No retry, no prompt construction, no agent logic. |
| `http` | `http` | `client()` / `client_with_timeouts(..)` — a `reqwest::Client` with rustls TLS and sane default timeouts (30s request, 10s connect). Caller owns all request building, auth, retries, and base URLs. |
| `kg` | `kg` | Knowledge-graph query helpers: upsert nodes/edges, one-hop neighbor traversal, pgvector cosine-distance similarity search. The caller supplies all vectors; no ingestion or embedding generation here. |
| `signal` | `signal` | `shutdown_signal()` — await Ctrl-C (SIGINT) or, on Unix, SIGTERM. Pure plumbing for graceful shutdown; does not log. |
| `web` | `web` | `serve_spa::<A>(&uri)` (embedded SPA fallback, generic over `RustEmbed` type), `extract_bearer(header)` (strip `"Bearer "` prefix), `serve_with_shutdown(addr, router)` (bind + graceful serve). No service policy. |

## Use it

```toml
# runtime
soma-infra = { path = "../soma-infra", features = ["db", "tracing"] }

# integration tests
[dev-dependencies]
soma-infra = { path = "../soma-infra", features = ["testing"] }
```

```rust
let pool = soma_infra::connect_from_env().await?;   // reads DATABASE_URL
soma_infra::telemetry::init();                       // RUST_LOG-aware
```

```rust
// in a #[tokio::test]
let db = soma_infra::TestDb::create_from_env().await?; // reads TEST_DATABASE_URL
// ... use db.pool ... database is dropped when `db` goes out of scope
```

## Env vars

- `DATABASE_URL` (required for `db`), plus optional `DB_MAX_CONNECTIONS`, `DB_MIN_CONNECTIONS`, `DB_ACQUIRE_TIMEOUT_SECS`, `DB_APP_NAME`.
- `RUST_LOG`, `LOG_FORMAT=json` (for `tracing`).
- `TEST_DATABASE_URL` (for `testing`).
- `REDIS_URL` (required for `cache`), plus optional `CACHE_RESPONSE_TIMEOUT_SECS`, `CACHE_CONNECTION_TIMEOUT_SECS`.
- `CRYPTO_KEY` (or any name you choose) for `crypto` — a hex-encoded 32-byte AES-256 key (exactly 64 hex characters). Load with `CryptoKey::from_env("CRYPTO_KEY")`.

### kg

Thin query helpers for a property graph (nodes + directed edges) stored in Postgres, with cosine-distance vector similarity search via pgvector. No ingestion, no embedding generation — the caller supplies all vectors.

The consuming service owns the schema and migration — copy the SQL blocks below into your service's `migrations/` directory (soma-schema UP/DOWN format). soma-infra ships only the Rust query helpers.

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
-- This value cannot change after rows exist without dropping and recreating this table.
CREATE TABLE kg_node_embeddings (
    node_id   uuid PRIMARY KEY REFERENCES kg_nodes(id) ON DELETE CASCADE,
    embedding vector(1536) NOT NULL
);

-- operator class MUST match <=> (cosine distance) used by vector_search_cosine.
-- lists is a tuning parameter; adjust for dataset size.
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

## License

Apache-2.0.
