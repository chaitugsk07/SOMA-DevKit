//! Cryptographic primitives for soma-platform services.
//!
//! Pure plumbing: a typed key wrapper, AES-256-GCM encrypt/decrypt, and
//! Argon2id password hashing. No field selection, no key-rotation policy,
//! no envelope wrapping — those belong in the service or in soma-vault.
//!
//! Build a [`CryptoKey`] from raw bytes with [`CryptoKey::from_bytes`] or
//! from a hex-encoded env var with [`CryptoKey::from_env`], then call
//! [`encrypt`] / [`decrypt`] / [`hash_password`] / [`verify_password`].

use aes_gcm::{
    aead::{Aead, AeadCore, OsRng, Payload},
    Aes256Gcm, Key, KeyInit,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use zeroize::Zeroizing;

// Ciphertext wire format:
//   0x01         – version byte  (1 B)
//   nonce        – AES-GCM nonce (12 B)
//   ciphertext   – encrypted data + GCM auth tag (N + 16 B)
const VERSION: u8 = 0x01;
const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;
// Minimum: version(1) + nonce(12) + tag(16) = 29; zero plaintext allowed.
const MIN_CT_LEN: usize = 1 + NONCE_LEN + TAG_LEN;

/// Errors from crypto operations.
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    /// A required environment variable was missing.
    #[error("required env var {0} is not set")]
    MissingEnv(&'static str),
    /// An environment variable held an unparseable or wrong-length value.
    #[error("env var {0} has an invalid value")]
    InvalidEnv(&'static str),
    /// Key bytes were the wrong length.
    #[error("key must be {expected} bytes, got {got}")]
    KeyLength { expected: usize, got: usize },
    /// AES-GCM encryption failed.
    #[error("encryption failed")]
    Encrypt,
    /// AES-GCM decryption or authentication failed.
    #[error("decryption failed")]
    Decrypt,
    /// Password hashing failed or the hash string is malformed.
    #[error("password hash operation failed")]
    Hash,
    /// Password does not match the stored hash.
    #[error("invalid password")]
    InvalidPassword,
    /// Ciphertext begins with an unrecognised version byte.
    #[error("unsupported ciphertext version: {0:#04x}")]
    UnsupportedVersion(u8),
    /// HKDF expand failed (requested output length exceeds 255 * HashLen).
    #[error("HKDF expand failed (output too long)")]
    Hkdf,
}

/// A 256-bit AES-GCM key. Zeroized on drop; never prints its bytes.
pub struct CryptoKey(Zeroizing<[u8; 32]>);

impl std::fmt::Debug for CryptoKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CryptoKey(***)")
    }
}

impl CryptoKey {
    /// Build a key from exactly 32 raw bytes.
    pub fn from_bytes(raw: &[u8]) -> Result<Self, CryptoError> {
        if raw.len() != 32 {
            return Err(CryptoError::KeyLength {
                expected: 32,
                got: raw.len(),
            });
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(raw);
        Ok(Self(Zeroizing::new(arr)))
    }

    /// Return the raw key bytes. Use for deriving sub-keys (e.g. HKDF).
    /// The caller is responsible for not leaking the bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    /// Read a hex-encoded 32-byte key (exactly 64 hex chars) from `var`.
    ///
    /// Missing var → [`CryptoError::MissingEnv`].
    /// Bad hex or wrong decoded length → [`CryptoError::InvalidEnv`].
    pub fn from_env(var: &'static str) -> Result<Self, CryptoError> {
        let s = std::env::var(var).map_err(|_| CryptoError::MissingEnv(var))?;
        // ponytail: inline decode (~6 lines) rather than adding the `hex` dep.
        // `hex` is only transitive via the `cache` feature (redis), not guaranteed
        // when `crypto` is used alone.
        let bytes = decode_hex(s.trim()).ok_or(CryptoError::InvalidEnv(var))?;
        Self::from_bytes(&bytes).map_err(|_| CryptoError::InvalidEnv(var))
    }
}

/// Decode a hex string into bytes. Returns `None` on any decode error.
fn decode_hex(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    s.as_bytes()
        .chunks(2)
        .map(|pair| {
            let hi = char::from(pair[0]).to_digit(16)? as u8;
            let lo = char::from(pair[1]).to_digit(16)? as u8;
            Some((hi << 4) | lo)
        })
        .collect()
}

/// Encrypt `plaintext` with AES-256-GCM, binding `aad` into the tag.
///
/// Output format: `0x01 || nonce(12) || ciphertext || tag(16)`.
/// A fresh random nonce is generated per call.
pub fn encrypt(key: &CryptoKey, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let aes_key = Key::<Aes256Gcm>::from_slice(key.0.as_ref());
    let cipher = Aes256Gcm::new(aes_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let payload = Payload {
        msg: plaintext,
        aad,
    };
    let ct = cipher
        .encrypt(&nonce, payload)
        .map_err(|_| CryptoError::Encrypt)?;

    let mut out = Vec::with_capacity(1 + NONCE_LEN + ct.len());
    out.push(VERSION);
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ct);
    Ok(out)
}

/// Decrypt and authenticate a ciphertext produced by [`encrypt`].
///
/// Verifies the version byte, the nonce, the GCM tag, and the `aad`.
pub fn decrypt(key: &CryptoKey, ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if ciphertext.len() < MIN_CT_LEN {
        return Err(CryptoError::Decrypt);
    }
    let version = ciphertext[0];
    if version != VERSION {
        return Err(CryptoError::UnsupportedVersion(version));
    }
    let nonce_bytes = &ciphertext[1..1 + NONCE_LEN];
    let ct_and_tag = &ciphertext[1 + NONCE_LEN..];

    let aes_key = Key::<Aes256Gcm>::from_slice(key.0.as_ref());
    let cipher = Aes256Gcm::new(aes_key);
    let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);
    let payload = Payload {
        msg: ct_and_tag,
        aad,
    };
    cipher
        .decrypt(nonce, payload)
        .map_err(|_| CryptoError::Decrypt)
}

/// Hash `password` with Argon2id, returning a PHC-format string.
///
// ponytail: Argon2::default() is Argon2id with OWASP-recommended minimums
// (m=19456 KiB, t=2 iterations, p=1). Adjust only if profiling shows the
// latency is unacceptable for your auth path.
pub fn hash_password(password: &str) -> Result<String, CryptoError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| CryptoError::Hash)
}

/// Verify `password` against a PHC hash string.
///
/// Returns `Ok(())` on success, [`CryptoError::InvalidPassword`] on mismatch,
/// and [`CryptoError::Hash`] if `hash` is malformed (not a valid PHC string).
pub fn verify_password(password: &str, hash: &str) -> Result<(), CryptoError> {
    let parsed = PasswordHash::new(hash).map_err(|_| CryptoError::Hash)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|e| {
            use argon2::password_hash::Error;
            if matches!(e, Error::Password) {
                CryptoError::InvalidPassword
            } else {
                CryptoError::Hash
            }
        })
}

/// HKDF-SHA256 key derivation. Generic primitive: the caller supplies the
/// input key material, an optional salt, the context `info`, and the desired
/// output length. The KDF *parameters* (salt/info strings) are the caller's
/// policy and stay in the calling service.
///
/// Returns the derived bytes wrapped in `Zeroizing` (wiped on drop).
///
/// Fails only if `out_len` > 255 × 32 = 8 160 bytes (HKDF-SHA256 ceiling).
#[cfg(feature = "crypto")]
pub fn hkdf_sha256(
    ikm: &[u8],
    salt: Option<&[u8]>,
    info: &[u8],
    out_len: usize,
) -> Result<Zeroizing<Vec<u8>>, CryptoError> {
    use hkdf::Hkdf;
    use sha2::Sha256;
    let hk = Hkdf::<Sha256>::new(salt, ikm);
    let mut okm = Zeroizing::new(vec![0u8; out_len]);
    hk.expand(info, okm.as_mut())
        .map_err(|_| CryptoError::Hkdf)?; // fails only if out_len > 255*32
    Ok(okm)
}

/// HMAC-SHA256 of `msg` under `key`, returned as a lowercase hex string.
/// Accepts any key length (HMAC permits it).
///
/// The output is byte-for-byte identical to both `hex::encode(mac.finalize())`
/// and the `format!("{b:02x}")` fold used in soma-vault and soma-audit.
#[cfg(feature = "crypto")]
pub fn hmac_sha256_hex(key: &[u8], msg: &[u8]) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = <HmacSha256 as Mac>::new_from_slice(key).expect("HMAC accepts any key length");
    mac.update(msg);
    // ponytail: inline format fold — no `hex` dep needed, identical output.
    mac.finalize()
        .into_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// SHA-256 of `input`, rendered as a lowercase hex string (64 chars).
///
/// A bare digest — NOT a MAC. Use for content fingerprints and token-hash
/// storage where you store `sha256_hex(token)` and compare hex strings.
/// For authentication (verifying a value under a secret key) use
/// [`hmac_sha256_hex`] instead.
#[cfg(feature = "crypto")]
pub fn sha256_hex(input: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(input);
    // ponytail: inline format fold — no `hex` dep needed, identical output.
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> CryptoKey {
        CryptoKey::from_bytes(&[0x42u8; 32]).unwrap()
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let key = test_key();
        let plaintext = b"hello soma";
        let aad = b"context";
        let ct = encrypt(&key, plaintext, aad).unwrap();
        let recovered = decrypt(&key, &ct, aad).unwrap();
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn decrypt_wrong_key_fails() {
        let key = test_key();
        let ct = encrypt(&key, b"secret", b"aad").unwrap();
        let wrong_key = CryptoKey::from_bytes(&[0xFFu8; 32]).unwrap();
        assert!(matches!(
            decrypt(&wrong_key, &ct, b"aad"),
            Err(CryptoError::Decrypt)
        ));
    }

    #[test]
    fn decrypt_wrong_aad_fails() {
        let key = test_key();
        let ct = encrypt(&key, b"secret", b"correct-aad").unwrap();
        assert!(matches!(
            decrypt(&key, &ct, b"wrong-aad"),
            Err(CryptoError::Decrypt)
        ));
    }

    #[test]
    fn decrypt_tampered_ciphertext_fails() {
        let key = test_key();
        let mut ct = encrypt(&key, b"secret", b"aad").unwrap();
        // Flip a byte in the ciphertext region (after version + nonce)
        let flip_idx = 1 + NONCE_LEN + 1;
        ct[flip_idx] ^= 0xFF;
        assert!(matches!(
            decrypt(&key, &ct, b"aad"),
            Err(CryptoError::Decrypt)
        ));
    }

    #[test]
    fn encrypt_output_starts_with_version_byte() {
        let key = test_key();
        let ct = encrypt(&key, b"test", b"").unwrap();
        assert_eq!(ct[0], 0x01);
    }

    #[test]
    fn decrypt_bad_version_byte_returns_unsupported_version() {
        let key = test_key();
        let mut ct = encrypt(&key, b"test", b"").unwrap();
        ct[0] = 0x02;
        assert!(matches!(
            decrypt(&key, &ct, b""),
            Err(CryptoError::UnsupportedVersion(0x02))
        ));
    }

    #[test]
    fn decrypt_too_short_input_does_not_panic() {
        let key = test_key();
        // Way too short — no version + nonce + tag
        assert!(matches!(
            decrypt(&key, b"\x01short", b""),
            Err(CryptoError::Decrypt)
        ));
        // Empty input
        assert!(matches!(decrypt(&key, b"", b""), Err(CryptoError::Decrypt)));
    }

    #[test]
    fn from_bytes_wrong_length_returns_key_length_error() {
        let result = CryptoKey::from_bytes(&[0u8; 16]);
        assert!(matches!(
            result,
            Err(CryptoError::KeyLength { got: 16, .. })
        ));
    }

    #[test]
    fn hash_and_verify_password_ok() {
        let hash = hash_password("correct-horse").unwrap();
        assert!(verify_password("correct-horse", &hash).is_ok());
    }

    #[test]
    fn verify_password_wrong_returns_invalid_password() {
        let hash = hash_password("correct-horse").unwrap();
        assert!(matches!(
            verify_password("wrong-horse", &hash),
            Err(CryptoError::InvalidPassword)
        ));
    }

    #[test]
    fn verify_password_malformed_hash_returns_hash_error() {
        assert!(matches!(
            verify_password("any", "not-a-phc-hash"),
            Err(CryptoError::Hash)
        ));
    }

    /// All `from_env` paths exercised sequentially to avoid races on shared
    /// env vars. Same pattern as cache.rs.
    #[test]
    fn crypto_key_from_env_paths() {
        const VAR: &str = "SOMA_CRYPTO_TEST_KEY";

        // — missing var —
        std::env::remove_var(VAR);
        assert!(matches!(
            CryptoKey::from_env(VAR),
            Err(CryptoError::MissingEnv(VAR))
        ));

        // — bad hex (odd length) —
        std::env::set_var(VAR, "deadbeef0");
        assert!(matches!(
            CryptoKey::from_env(VAR),
            Err(CryptoError::InvalidEnv(VAR))
        ));

        // — wrong decoded length (not 32 bytes) —
        std::env::set_var(VAR, "deadbeef");
        assert!(matches!(
            CryptoKey::from_env(VAR),
            Err(CryptoError::InvalidEnv(VAR))
        ));

        // — valid 64-hex char (32 bytes) —
        std::env::set_var(
            VAR,
            "4242424242424242424242424242424242424242424242424242424242424242",
        );
        let key = CryptoKey::from_env(VAR).unwrap();
        assert_eq!(key.0.as_ref(), &[0x42u8; 32]);

        // cleanup
        std::env::remove_var(VAR);
    }

    // ── Golden-vector tests ──────────────────────────────────────────────────
    // Each test proves that the soma-infra primitive produces byte-for-byte
    // identical output to the raw crate call used in the originating service.
    // When these pass the service can safely delete its inline copy.

    /// VAULT: derive_tenant_kek
    /// salt = b"soma-vault-tenant-kek-v1", info = tenant_id bytes (16-byte fixed array)
    #[test]
    #[cfg(feature = "crypto")]
    fn golden_vault_tenant_kek() {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let master_kek = [0x01u8; 32];
        // Fixed tenant_id bytes — HKDF is indifferent to UUID structure.
        let tenant_id_bytes: [u8; 16] = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef,
        ];

        // (a) infra primitive
        let infra = hkdf_sha256(
            &master_kek,
            Some(b"soma-vault-tenant-kek-v1"),
            &tenant_id_bytes,
            32,
        )
        .unwrap();

        // (b) raw service code (verbatim)
        let hk = Hkdf::<Sha256>::new(Some(b"soma-vault-tenant-kek-v1"), &master_kek);
        let mut raw = [0u8; 32];
        hk.expand(&tenant_id_bytes, &mut raw).unwrap();

        assert_eq!(infra.as_slice(), raw.as_ref());
    }

    /// VAULT: derive_audit_hmac_key
    /// salt = b"soma-vault-audit-hmac-v1", info = b"audit"
    #[test]
    #[cfg(feature = "crypto")]
    fn golden_vault_audit_hmac_key() {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let master_kek = [0x01u8; 32];

        // (a) infra primitive
        let infra =
            hkdf_sha256(&master_kek, Some(b"soma-vault-audit-hmac-v1"), b"audit", 32).unwrap();

        // (b) raw service code (verbatim)
        let hk = Hkdf::<Sha256>::new(Some(b"soma-vault-audit-hmac-v1"), &master_kek);
        let mut raw = [0u8; 32];
        hk.expand(b"audit", &mut raw).unwrap();

        assert_eq!(infra.as_slice(), raw.as_ref());
    }

    /// AUDIT: derive_tenant_hmac_key
    /// salt = None, info = b"soma-audit-hmac-v1" ++ tenant_id_bytes (concatenated)
    #[test]
    #[cfg(feature = "crypto")]
    fn golden_audit_tenant_hmac_key() {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let master_secret = [0x01u8; 32];
        let tenant_id_bytes: [u8; 16] = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef,
        ];

        // Audit concatenates prefix + tenant bytes into one Vec (verbatim).
        let mut info: Vec<u8> = b"soma-audit-hmac-v1".to_vec();
        info.extend_from_slice(&tenant_id_bytes);

        // (a) infra primitive
        let infra = hkdf_sha256(&master_secret, None, &info, 32).unwrap();

        // (b) raw service code (verbatim)
        let hk = Hkdf::<Sha256>::new(None, &master_secret);
        let mut raw = [0u8; 32];
        hk.expand(&info, &mut raw).unwrap();

        assert_eq!(infra.as_slice(), raw.as_ref());
    }

    /// VAULT + AUDIT: hmac_sha256_hex equivalence
    /// Proves infra output == hex::encode path AND format!("{b:02x}") fold.
    #[test]
    #[cfg(feature = "crypto")]
    fn golden_hmac_hex_both_styles() {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        type HmacSha256 = Hmac<Sha256>;

        let key = [0x02u8; 32];
        let msg = b"soma golden vector test message";

        // (a) infra primitive
        let infra_hex = hmac_sha256_hex(&key, msg);

        // (b) vault path: hex::encode equivalent — build bytes then encode inline
        let mut mac_b = <HmacSha256 as Mac>::new_from_slice(&key).unwrap();
        mac_b.update(msg);
        let bytes_b = mac_b.finalize().into_bytes();
        // hex::encode produces lowercase hex — replicate without the dep:
        let vault_hex: String = bytes_b.iter().map(|b| format!("{b:02x}")).collect();

        // (c) audit path: format!("{b:02x}") fold (identical pattern, separate MAC instance)
        let mut mac_c = <HmacSha256 as Mac>::new_from_slice(&key).unwrap();
        mac_c.update(msg);
        let bytes_c = mac_c.finalize().into_bytes();
        let audit_hex: String = bytes_c.iter().map(|b| format!("{b:02x}")).collect();

        assert_eq!(infra_hex, vault_hex, "infra != vault hex");
        assert_eq!(infra_hex, audit_hex, "infra != audit hex");
        // Sanity: result is a 64-char lowercase hex string (32 bytes × 2)
        assert_eq!(infra_hex.len(), 64);
        assert!(infra_hex
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
    }

    /// Edge case: hkdf_sha256 returns Hkdf error when out_len exceeds ceiling.
    #[test]
    #[cfg(feature = "crypto")]
    fn hkdf_sha256_too_long_returns_error() {
        // SHA-256 ceiling: 255 * 32 = 8160 bytes; request one more.
        let result = hkdf_sha256(b"key", None, b"info", 8161);
        assert!(matches!(result, Err(CryptoError::Hkdf)));
    }

    /// NIST/RFC known-vector tests for sha256_hex.
    /// Pin the lowercase-hex output format so it cannot silently drift.
    #[test]
    #[cfg(feature = "crypto")]
    fn sha256_hex_known_vectors() {
        // Empty-string SHA-256 (well-known reference value).
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        // NIST FIPS 180-4 example: SHA-256("abc").
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
