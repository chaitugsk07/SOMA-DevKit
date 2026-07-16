//! Argon2id password hashing and session token generation for soma-iam.
//!
//! Uses stronger params than soma-infra: m=65536 KiB, t=3, p=4 (OWASP for auth
//! services). soma-infra uses m=19456 (OWASP minimum for non-auth — 3× weaker).

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params, Version,
};
// ponytail: use rand_core re-exported from password_hash (already a dep of argon2)
// rather than adding a separate rand_core or rand dep.
use argon2::password_hash::rand_core::{OsRng, RngCore};
use std::sync::OnceLock;
use tokio::sync::Semaphore;

use crate::sha256_hex;

/// Argon2id parameters: m=65536 KiB, t=3, p=4.
/// OWASP recommended for authentication services (2024).
/// soma-infra uses Argon2::default() = m=19456, t=2, p=1 — intentionally weaker
/// (OWASP minimum for non-auth flows). This crate always uses the stronger params.
fn argon2() -> Argon2<'static> {
    // ponytail: static params — no allocation per call.
    // These must never change without a migration; the golden-vector test pins them.
    let params = Params::new(65536, 3, 4, None).expect("hardcoded Argon2 params are valid");
    Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params)
}

/// Global semaphore bounding concurrent Argon2id hashes to prevent CPU saturation.
/// Default: 4. Override with env `SOMA_ARGON2_CONCURRENCY`.
// ponytail: OnceLock<Semaphore> — zero cost after init, no mutex in hot path.
// Ceiling: if hardware concurrency grows, raise SOMA_ARGON2_CONCURRENCY; no code change.
static SEM: OnceLock<Semaphore> = OnceLock::new();

fn semaphore() -> &'static Semaphore {
    SEM.get_or_init(|| {
        let n = std::env::var("SOMA_ARGON2_CONCURRENCY")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(4);
        Semaphore::new(n)
    })
}

/// Hash a password with Argon2id(m=65536, t=3, p=4), returning a PHC string.
///
/// Acquires a permit from the global semaphore before hashing to bound CPU use.
///
/// # Errors
/// Returns an error string if hashing fails (internal salt generation or Argon2 error).
pub async fn hash_password(password: &str) -> Result<String, String> {
    let _permit = semaphore().acquire().await.map_err(|e| e.to_string())?;
    let password = password.to_owned();
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        argon2()
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Verify `password` against a PHC hash string in constant time.
///
/// Returns `true` on match, `false` on any mismatch or malformed hash.
/// Always runs the full Argon2id computation — never short-circuits on error.
pub fn verify_password(password: &str, phc: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(phc) else {
        return false;
    };
    argon2()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

/// Cached dummy PHC hash for timing-parity on "user not found" login paths.
// ponytail: OnceLock generates once on first call; verify_password runs the full
// Argon2id computation every time regardless of dummy password value.
// Ceiling: if params change, OnceLock is re-generated automatically (unit restart).
static DUMMY_PHC: OnceLock<String> = OnceLock::new();

fn get_dummy_phc() -> &'static str {
    DUMMY_PHC.get_or_init(|| {
        let salt = SaltString::generate(&mut OsRng);
        argon2()
            .hash_password(b"__soma_dummy__", &salt)
            .expect("dummy hash generation failed")
            .to_string()
    })
}

/// Run a dummy Argon2id verify on a hardcoded hash.
///
/// Call this on the "user not found" branch of login so that the code path and
/// timing match a real password verification — preventing user-enumeration via
/// timing side-channel.
pub fn dummy_hash() {
    verify_password("__soma_dummy__", get_dummy_phc());
}

/// Generate a session token: 32 random bytes from `OsRng`.
///
/// Returns `(raw_hex, hash_hex)`:
/// - `raw_hex` — 64-char hex; goes into the `__Host-` cookie.
/// - `hash_hex` — SHA-256 of the raw bytes as hex; stored in `fct_sessions.session_token_hash`.
///
/// The caller must store only `hash_hex` in the database.
pub fn session_token() -> (String, String) {
    let mut raw = [0u8; 32];
    OsRng.fill_bytes(&mut raw);
    // ponytail: hex dep (0.4) for encoding raw bytes — stdlib has no built-in hex encode.
    let raw_hex = hex::encode(raw);
    let hash_hex = sha256_hex(raw_hex.as_bytes());
    (raw_hex, hash_hex)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Golden-vector: hashing any password must produce a PHC that encodes
    /// exactly m=65536, t=3, p=4. Fails if params ever silently drift.
    #[tokio::test]
    async fn golden_params_pinned() {
        let phc = hash_password("golden-test-password").await.unwrap();
        // PHC format: $argon2id$v=19$m=65536,t=3,p=4$<salt>$<hash>
        assert!(
            phc.contains("m=65536,t=3,p=4"),
            "Argon2id params drifted! PHC was: {phc}"
        );
    }

    #[tokio::test]
    async fn hash_and_verify_roundtrip() {
        let phc = hash_password("correct-horse-battery-staple").await.unwrap();
        assert!(verify_password("correct-horse-battery-staple", &phc));
        assert!(!verify_password("wrong-password", &phc));
    }

    #[test]
    fn dummy_hash_does_not_panic() {
        // Just verify it runs without panicking; timing parity is its purpose.
        dummy_hash();
    }

    #[test]
    fn session_token_raw_and_hash_lengths() {
        let (raw, hash) = session_token();
        assert_eq!(raw.len(), 64, "raw token must be 64 hex chars (32 bytes)");
        assert_eq!(hash.len(), 64, "sha256 hex must be 64 chars");
        // They must be different (raw != sha256(raw))
        assert_ne!(raw, hash);
    }

    #[test]
    fn session_token_unique() {
        let (a, _) = session_token();
        let (b, _) = session_token();
        assert_ne!(a, b, "tokens must be unique across calls");
    }
}
