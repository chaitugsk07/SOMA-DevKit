# soma-infra Readiness Report

## 1. Bottom Line

soma-infra is ready to be consumed by soma-platform services and published to
crates.io. The fundamentals are solid: the feature-gate architecture works, the
crypto module has golden-vector coverage proving byte-identical output to
consuming services, 46 unit tests pass, and the crate has already been published
as v0.1.0 (v0.1.1 adds the docs.rs all-features fix).

Honest maturity caveat: this is a days-old API at 0.1.x. Three services are
consuming it (soma-audit, soma-vault, soma-iam) via path deps in the monorepo.
No external consumers have pinned the crates.io version yet. Expect iteration
on non-crypto APIs before 0.2. The crypto module is frozen — its output is
pinned by golden-vector tests that consuming services depend on.

---

## 2. Publish-Readiness Checklist

### Package metadata

| item | status | notes |
|------|--------|-------|
| `name`, `version`, `edition` | ✅ | `soma-infra`, `0.1.1`, `2021` |
| `license` | ✅ | `Apache-2.0`; `LICENSE` file present |
| `description` | ✅ | Present; describes all module areas |
| `repository`, `homepage` | ✅ | `github.com/chaitugsk07/soma-infra` |
| `readme` | ✅ | `README.md` present at crate root |
| `keywords` | ✅ | `["backend", "sqlx", "crypto", "redis", "llm"]` |
| `categories` | ✅ | `["database", "development-tools", "asynchronous"]` |
| `rust-version` | ✅ | `1.82` |
| `[package.metadata.docs.rs] all-features = true` | ✅ | Added in v0.1.1 |

### Code quality

| item | status | notes |
|------|--------|-------|
| `#![forbid(unsafe_code)]` at crate root | ✅ | Enforced in `lib.rs` |
| Compiles with `--all-features` | ✅ | Verified during publish dry-run |
| `cargo publish --dry-run` | ✅ | Passed before v0.1.0 publish |
| Unit tests (46 total) | ✅ | All pass; run with `cargo test --all-features` |
| No `path =` deps in `[dependencies]` | ✅ | All deps are registry-version deps |
| No `dev-dependencies` in the published surface | ✅ | `tokio` and `sqlx` in `[dev-dependencies]` only |

### Crypto golden-vector tests

These tests prove that soma-infra's HKDF/HMAC/SHA256 primitives produce
byte-for-byte identical output to the raw crate calls used in soma-vault and
soma-audit. If any of these fail, the consuming service's stored ciphertexts or
HMAC chains become invalid.

| test | service validated against | status |
|------|--------------------------|--------|
| `golden_vault_tenant_kek` | soma-vault `derive_tenant_kek` | ✅ |
| `golden_vault_audit_hmac_key` | soma-vault `derive_audit_hmac_key` | ✅ |
| `golden_audit_tenant_hmac_key` | soma-audit `derive_tenant_hmac_key` | ✅ |
| `golden_hmac_hex_both_styles` | soma-vault + soma-audit hex output | ✅ |
| `sha256_hex_known_vectors` | NIST FIPS 180-4 reference vectors | ✅ |
| `hkdf_sha256_too_long_returns_error` | ceiling enforcement | ✅ |

### Feature isolation

Each feature enables only its own dependencies. A `db`-only consumer (the most
common case) resolves zero crypto, Redis, or LLM dependencies.

| feature | exclusive deps | isolated | notes |
|---------|---------------|----------|-------|
| `db` | `sqlx`, `tokio`, `thiserror` | ✅ | Default on |
| `tracing` | `tracing`, `tracing-subscriber` | ✅ | Default on |
| `config` | `thiserror` | ✅ | — |
| `errors` | `sqlx` | ✅ | — |
| `testing` | `db` + `uuid` | ✅ | dev-dep use only |
| `cache` | `redis`, `tokio`, `thiserror` | ✅ | No db dep |
| `crypto` | `aes-gcm`, `argon2`, `zeroize`, `hkdf`, `hmac`, `sha2`, `thiserror` | ✅ | No network, no db |
| `storage-s3` | `object_store` (aws), `bytes`, `thiserror` | ✅ | — |
| `storage-azure` | `object_store` (azure), `bytes`, `thiserror` | ✅ | — |
| `llm` | `reqwest`, `serde`, `serde_json`, `tokio`, `thiserror` | ✅ | No db, no crypto |
| `http` | `reqwest` | ✅ | Subset of `llm` deps |
| `kg` | `sqlx`, `pgvector`, `serde_json`, `uuid`, `thiserror` | ✅ | Requires `db` pool but doesn't re-declare db dep |
| `signal` | `tokio` (signal feature) | ✅ | — |

### docs.rs

`[package.metadata.docs.rs] all-features = true` was added in v0.1.1. Before
this, docs.rs would only document the default features (`db`, `tracing`), making
`crypto`, `cache`, `storage`, `llm`, `kg`, `http`, and `signal` invisible in
the published docs.

---

## 3. Known Gaps (honest, not blockers)

| gap | severity | notes |
|-----|----------|-------|
| No CI badge | cosmetic | No CI pipeline configured yet; tests run locally |
| No published consumers pinned to crates.io version | tracking | All soma-platform services use path deps in monorepo |
| `kg` feature requires `db` pool but has no explicit `db` dep | minor gotcha | Consumer must enable both `kg` and `db` features; documented in CONSUMING.md |
| `object_store` pinned to `0.12.x` (rustc 1.82 compat) | known constraint | 0.13+ requires rustc ≥ 1.85; upgrade when rust-version is bumped |
| No streaming LLM support | deferred | `messages()` awaits full response; streaming deferred until consuming services need it |
| No external integration tests for storage/llm | expected | These require live credentials (AWS/Azure/Anthropic); not run in normal `cargo test` |
| `CACHE_RESPONSE_TIMEOUT_SECS` defaults to no timeout | known tradeoff | Documented; consumers should set it in production |

---

## 4. What Is Already Genuinely Solid

- **Feature-gate architecture is correct.** Thirteen features, each with exactly
  the deps it needs and nothing it doesn't. Verified at publish dry-run.
- **Crypto module is frozen and verified.** Golden-vector tests pin output to
  the consuming services' expectations. Any drift is caught at test time, not at
  runtime.
- **`#![forbid(unsafe_code)]`** is enforced crate-wide. No unsafe blocks anywhere.
- **46 unit tests all pass.** Covering every error path in crypto, all env-var
  fallback paths in CryptoKey/CacheConfig/PoolConfig, and the known NIST
  reference vectors.
- **The plumbing-vs-logic line is held.** No business logic, no policy
  decisions, no default LLM model, no prompt construction — every module is a
  configured client or a decision-free primitive.
- **Three active consumers** (soma-audit, soma-vault, soma-iam) are live proof
  that the API is coherent and usable from real service code.
- **CLAUDE.md enforcement.** Each consuming service's CLAUDE.md references
  soma-infra as the source of truth for plumbing, preventing services from
  re-implementing pools, crypto, or HTTP clients locally.
