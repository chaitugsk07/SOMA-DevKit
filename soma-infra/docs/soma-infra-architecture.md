# soma-infra Architecture Decision Document

## TL;DR / Decision Summary

| decision | choice | one-line why |
|----------|--------|--------------|
| **One crate, not a workspace** | Single `soma-infra` package | Feature gates achieve module isolation without the coordination overhead of separate crates |
| **Feature per concern** | 13 opt-in features | Compile only what you use; db-only consumers never resolve LLM/crypto/Redis graphs |
| **No shared error type** | Each module defines its own `thiserror` enum | Flat, legible errors without a crate-wide `Error::Crypto(CryptoError)` chain |
| **No default LLM model** | `ANTHROPIC_MODEL` required at runtime | Silently falling back to a different model tier is a billing hazard |
| **Crypto golden vectors** | Tests pin byte-for-byte output | Any drift in HKDF/HMAC/SHA256 output breaks consuming services' stored ciphertexts and HMAC chains |
| **kg migration is consumer-owned** | SQL shipped as docs, not executed code | The advisory lock key and schema name are service policy, not plumbing |
| **Both storage features discriminant** | Enum variants, not struct fields | Invalid states (S3 config with Azure fields) are unrepresentable |
| **plumbing-vs-logic line** | Mechanism here, policy in service | Enforced by CLAUDE.md across repos; prevents logic creep into the shared library |

---

## 1. Governing principle: mechanism without policy

soma-infra is the backend counterpart to `soma-ui` (shared UI components). Every
soma service consumes it; nothing in this crate is service-specific.

The line: **soma-infra ships the mechanism, never the policy.**

- Mechanism: pool builder, AES-256-GCM wire format, HKDF expand, Redis
  `ConnectionManager`, Anthropic HTTP wire protocol, pgvector cosine query.
- Policy: which fields to encrypt, KDF salt/info strings, SQLSTATE→domain-error
  mapping, migration schema and advisory lock key, prompt construction, object
  key naming.

This line is enforced by `CLAUDE.md` files in each consuming service. If a
function makes a decision the service should own, it does not belong in
soma-infra.

Concrete examples of what correctly stays in each service:

| stays in service | reason |
|-----------------|--------|
| `b"soma-vault-tenant-kek-v1"` (HKDF salt) | Service's key-derivation policy |
| `b"soma-audit-hmac-v1"` (HKDF info prefix) | Service's chain-math policy |
| `"soma_audit"` schema name + advisory lock key | Service's migration contract |
| `Migrator` wiring and `include_dir!` SQL | Migration runner belongs to the service |
| `map_sqlx` (SQLSTATE → app error) | Domain error mapping is service logic |
| Retry loop for `LlmError::RateLimited` | Rate-limit strategy is service policy |
| Embedding generation before `vector_search_cosine` | ML pipeline is service logic |

---

## 2. One crate, not a workspace of crates

**Decision:** Single `soma-infra` package with 13 cargo features — not a
workspace of `soma-db`, `soma-crypto`, `soma-cache`, etc.

**Rationale:**

A workspace of separate crates would require managing independent version
numbers, publish order, and inter-crate path+version declarations for every
release. For a plumbing library where most consumers take several features
together, this overhead outweighs the isolation benefit.

Cargo feature gates achieve the same compile-time isolation:

- A db-only consumer enables `["db", "tracing"]` and never resolves the LLM,
  crypto, or Redis dependency graphs.
- A change to `llm.rs` does not force a rebuild of services that only use `db`.
- Each module has its own `#[cfg(feature = "X")]` guard; unused modules are
  never compiled.

The tradeoff accepted: a single crate version bumps everything together. At
0.1.x with a small number of consumers, this is the right tradeoff. If the
crate grows to the point where consumers want to pin different features at
different versions, split into crates then.

---

## 3. Feature design

### Default features

`default = ["db", "tracing"]` — the two features every backend service needs.
A consumer that wants only Redis or only crypto must explicitly disable defaults:

```toml
soma-infra = { version = "0.1", default-features = false, features = ["cache"] }
```

Or, more commonly, add the extra features on top of defaults:

```toml
soma-infra = { version = "0.1", features = ["cache", "crypto"] }
```

### Heavy features are opt-in

`crypto`, `cache`, `storage-s3`, `storage-azure`, `llm`, `kg`, `http`, `signal`
are all opt-in. A `db`-only consumer never pays for `aes-gcm`, `argon2`,
`pgvector`, `reqwest`, `redis`, or `object_store` compile time.

### Feature graph is flat

No feature depends on another feature except `testing` (which depends on `db`)
and `storage-s3`/`storage-azure` (which share `object_store` but each adds its
own backend feature). This flatness is intentional — it keeps the feature
selection model simple for consumers.

---

## 4. Per-module error enum convention

Each module defines its own `thiserror` error enum (`DbError`, `CacheError`,
`CryptoError`, `StorageError`, `LlmError`, `KgError`). There is no shared
crate-wide `InfraError`.

**Why no shared error type:**

A shared `InfraError::Crypto(CryptoError)` wrapping layer would force every
caller to unwrap one level to get at the actual error variant. Module-specific
errors are flat and legible:

```rust
match result {
    Err(CryptoError::Decrypt)               => { /* handle auth failure */ }
    Err(CryptoError::UnsupportedVersion(v)) => { /* handle version mismatch */ }
    Err(e)                                  => { /* propagate */ }
}
```

The conversion into the calling service's own error type (via `From` or `#[from]`)
is a one-liner at the service boundary and has no overhead.

---

## 5. Crypto golden-vector approach

The `crypto` module is consumed by soma-vault (AES-256-GCM encryption, HKDF
tenant KEK derivation) and soma-audit (HKDF per-tenant HMAC key, HMAC-SHA256
chain hashing). These services have existing stored ciphertexts and HMAC chains
in production databases.

Any change to `hkdf_sha256`, `hmac_sha256_hex`, or `sha256_hex` that alters the
output bytes would silently corrupt every stored ciphertext and audit chain.

**The golden-vector approach:** Tests in `crypto.rs` capture the exact byte
output of the infra primitive and compare it against the verbatim raw crate code
from the originating service:

```rust
// (a) infra primitive
let infra = hkdf_sha256(&master_kek, Some(b"soma-vault-tenant-kek-v1"), &tenant_id_bytes, 32)?;

// (b) raw service code (copied verbatim from soma-vault)
let hk = Hkdf::<Sha256>::new(Some(b"soma-vault-tenant-kek-v1"), &master_kek);
let mut raw = [0u8; 32];
hk.expand(&tenant_id_bytes, &mut raw).unwrap();

assert_eq!(infra.as_slice(), raw.as_ref());
```

These tests cannot be changed without simultaneously updating the consuming
service and confirming that its stored data is migrated. They act as a compile-
time contract between soma-infra and its consumers.

The AES-256-GCM wire format (`0x01 || nonce(12) || ciphertext || tag(16)`) is
similarly fixed by the format version byte. An unrecognised version byte returns
`CryptoError::UnsupportedVersion` rather than attempting to decrypt.

---

## 6. kg — consumer-owned migration

The knowledge-graph feature (`kg`) ships Rust query helpers only. The schema
(`kg_nodes`, `kg_edges`, `kg_node_embeddings`) is documented in `kg.rs`'s `//!`
header and in `CONSUMING.md`, but it is never executed by soma-infra.

**Why:** The advisory lock key and schema name are service-owned policy (per the
soma-schema contract: one schema per service, one advisory lock key per service).
If soma-infra ran its own migration, it would have to choose an advisory lock key
that all consumers share — violating the contract.

The consumer copies the migration SQL into its own `migrations/` directory and
runs it via its own soma-schema setup. The vector dimension (`vector(1536)`) is
a placeholder that the consumer must set to match its embedding model.

---

## 7. Storage: both features and the enum discriminant

`storage-s3` and `storage-azure` may be enabled simultaneously. When both are
active, `StorageConfig` is an enum:

```rust
pub enum StorageConfig {
    #[cfg(feature = "storage-s3")]
    S3 { bucket: String, region: String, endpoint: Option<String> },
    #[cfg(feature = "storage-azure")]
    Azure { account: String, container: String },
}
```

Each variant carries only the fields for its backend. The `STORAGE_BACKEND` env
var (`"s3"` or `"azure"`) selects the variant at runtime when both features are
enabled.

**Why an enum, not a struct with optional fields:** A struct would allow invalid
states (S3 config with empty Azure fields set, or vice versa). The enum makes
invalid states unrepresentable and the match in `StorageClient::new` exhaustive
and compiler-checked.

---

## 8. LLM: no default model

`LlmConfig::from_env` requires `ANTHROPIC_MODEL` explicitly and returns
`LlmError::MissingEnv("ANTHROPIC_MODEL")` if it is absent. There is no
hardcoded fallback.

**Why:** Silently defaulting to a model the service did not intend (e.g.
falling back from `claude-opus-4-8` to `claude-haiku-4-5`) could cause a
dramatic change in cost or capability. Billing surprises from an implicit
default are worse than a configuration error at startup.

The `max_retries` field in `LlmConfig` is a config knob stored for the caller's
use. `LlmClient::messages()` never reads it — retries are the caller's
responsibility (see `LlmError::RateLimited`).

---

## 9. The consume-via-soma-infra enforcement

Every soma service has a `CLAUDE.md` that references the platform-wide rule:
all reusable backend plumbing comes from soma-infra. Services must not re-
implement a Postgres pool, `tracing-subscriber` init, AES-256-GCM encrypt, HKDF
call, Redis `ConnectionManager`, or `reqwest::Client` builder.

This is enforced by code review and the `CLAUDE.md` files, not by the build
system. The practical enforcement is that any time a service hand-rolls something
soma-infra provides, the code review should reject it and point to soma-infra.

The benefit is that improvements to the plumbing (better pool config defaults,
a new crypto primitive, a timeout fix in the HTTP client) propagate to all
services at once by bumping the soma-infra version.

---

## 10. What soma-infra deliberately is not

- Not a framework. It does not own your request handling, routing, middleware,
  or dependency injection.
- Not a migration tool. Migration runner and schema ownership belong to
  soma-schema and the consuming service.
- Not an agent or orchestration layer. `LlmClient` sends a message and returns
  a response; tool dispatch, prompt templates, and retry loops are service logic.
- Not an embedding pipeline. `vector_search_cosine` takes a pre-computed vector;
  it does not call any embedding API.
- Not a metrics or tracing exporter. `telemetry::init()` sets up a local
  subscriber; distributed tracing and metrics exporters are out of scope.
