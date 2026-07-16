# CLAUDE.md — soma-infra

**soma-infra is the shared backend-plumbing library for the soma-platform** — the backend counterpart to `soma-ui` (UI components). Every soma service consumes it; nothing here is service-specific.

The platform-wide rule lives in `../CLAUDE.md` ("Shared components"). This file is the producer side of it.

## What this crate is

Pure **plumbing, no logic**. Each module is a configured client or a thin primitive behind its own cargo feature, so a consumer compiles only what it uses. A db-only consumer must never resolve the LLM/crypto/redis/storage/pgvector dependency graphs.

`default = ["db", "tracing"]`. All heavy features (`crypto`, `cache`, `storage-s3`, `storage-azure`, `llm`, `kg`, `http`, `signal`, `config`) are opt-in.

## The line you must hold when adding to this crate

soma-infra ships the **mechanism**, never the **policy**. Before adding anything here:

- ✅ Add it if it's a decision-free operation a service would otherwise hand-roll (a pool builder, an HKDF call, a SHA-256-to-hex, a configured HTTP client). Keep all parameters caller-supplied — e.g. `hkdf_sha256(ikm, salt: Option<&[u8]>, info, len)` takes the salt/info from the caller; the strings (`"soma-vault-tenant-kek-v1"`) stay in the service.
- ❌ Do NOT add policy: no agent loops, no prompt construction, no SQLSTATE→domain-error mapping, no migration schema/lock-key wiring, no "which fields to encrypt", no default LLM model, no ingestion pipelines. If a function makes a decision the service should own, it doesn't belong here.

## Module conventions (match exactly when adding a module)

- `#![forbid(unsafe_code)]` at crate root.
- One module per concern, behind one cargo feature: `#[cfg(feature = "X")] pub mod X;`.
- Each module defines its OWN `thiserror` error enum (e.g. `CacheError`, `LlmError`). No shared crate-wide error type. Env-var names are `&'static str`; wrapped lib errors use `#[error(transparent)]` + `#[from]`.
- `Config` pattern: `::new(..)` with sane defaults + `::from_env()` reading scoped env vars (mirror `db.rs`'s `PoolConfig`).
- `//!` module header in real prose; keep the feature/module table in `lib.rs` and `README.md` updated.
- Co-located `#[cfg(test)]` tests; integration tests in `tests/` (gate on the relevant env var).
- `ponytail:` comments mark deliberate hardcoded defaults with a "tune per workload" note.
- Thin functions over frameworks; smallest useful surface; no speculative abstraction.

## Security-critical: crypto changes

The `crypto` module is consumed by soma-vault and soma-audit for real encryption and key derivation. Any change to `hkdf_sha256`/`hmac_sha256_hex`/`sha256_hex`/`encrypt`/`decrypt` MUST keep byte-for-byte output stable — there are golden-vector tests pinning the output to the services' expected bytes. Never alter the wire format, hex casing, or KDF mechanics without updating those vectors and confirming every consuming service's crypto tests still pass.
