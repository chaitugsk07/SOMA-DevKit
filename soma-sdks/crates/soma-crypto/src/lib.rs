#![forbid(unsafe_code)]
//! Cryptographic primitives for soma-iam auth paths.
//!
//! Key distinction from `soma-infra::crypto`: [`password::hash_password`] uses
//! `Argon2id(m=65536, t=3, p=4)` — OWASP recommended for auth services.
//! `soma-infra` uses `m=19456` (OWASP minimum for non-auth). Never swap them.

pub mod jwt;
pub mod password;
pub mod signing;

// Re-export infra primitives needed at M3 envelope/token-hash call sites.
// ponytail: re-export not re-implement — infra already owns AES-GCM, SHA-256, and HKDF.
pub use crate::signing::sha256_bytes;
pub use soma_infra::crypto::{decrypt, encrypt, hkdf_sha256, sha256_hex, CryptoError, CryptoKey};
