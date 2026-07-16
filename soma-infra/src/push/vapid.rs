//! Web Push (RFC 8291 / RFC 8292) client with VAPID authentication.
//!
//! Mechanism only: aes128gcm payload encryption (RFC 8291) + VAPID JWT
//! (RFC 8292) + POST to the push endpoint.  No subscription lifecycle,
//! no fanout, no retry policy.
//!
//! # Encryption summary (RFC 8291 §3)
//!
//! 1. Ephemeral P-256 keypair.
//! 2. ECDH with the subscriber's `p256dh` key → 32-byte shared secret.
//! 3. HKDF-SHA-256 with `auth` secret → 32-byte IKM.
//! 4. HKDF-SHA-256 with random 16-byte salt → 16-byte CEK + 12-byte nonce.
//! 5. AES-128-GCM encrypt (`plaintext || 0x02`) with CEK/nonce.
//! 6. Prepend aes128gcm header: `salt(16) || rs(4) || keyid_len(1) || ephemeral_pub(65)`.
//!
//! # VAPID authentication (RFC 8292)
//!
//! An ES256 JWT with `aud` = origin of the push endpoint, `sub` = subject,
//! `exp` ≤ 24 h, sent as `Authorization: vapid t=<jwt>,k=<public_key_base64url>`.

use std::time::Duration;

use aes_gcm::{
    aead::{Aead, OsRng},
    Aes128Gcm, Key, KeyInit,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hkdf::Hkdf;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use p256::{
    ecdh::EphemeralSecret,
    elliptic_curve::sec1::ToEncodedPoint,
    pkcs8::{EncodePrivateKey, LineEnding},
    PublicKey, SecretKey,
};
use serde::Serialize;
use sha2::Sha256;

// ── Types ─────────────────────────────────────────────────────────────────────

/// A Web Push subscription as returned by the browser's `PushSubscription`.
#[derive(Debug, Clone)]
pub struct WebPushSubscription {
    /// The push endpoint URL (browser-provided).
    pub endpoint: String,
    /// The subscriber's P-256 public key, base64url-encoded (no padding).
    pub p256dh: String,
    /// The 16-byte auth secret, base64url-encoded (no padding).
    pub auth: String,
}

/// VAPID key pair — a P-256 private key + subject used to authenticate sends.
///
/// Construct with [`VapidKey::generate`] or [`VapidKey::from_bytes`].
/// Expose the application server key to browsers via [`VapidKey::application_server_key`].
pub struct VapidKey {
    secret: SecretKey,
    /// Uncompressed P-256 public key (65 bytes, `0x04` prefix).
    pub_bytes: [u8; 65],
    /// PKCS#8 PEM cached for JWT signing via `jsonwebtoken`.
    pem: String,
    /// VAPID subject (`mailto:` or `https:` URI, sent in JWT `sub` claim).
    pub subject: String,
}

impl VapidKey {
    /// Generate a fresh P-256 VAPID key pair.
    pub fn generate(subject: impl Into<String>) -> Result<Self, WebPushError> {
        let secret = SecretKey::random(&mut OsRng);
        Self::from_secret(secret, subject.into())
    }

    /// Load a VAPID key from raw 32-byte P-256 scalar bytes.
    pub fn from_bytes(
        private_key_bytes: &[u8],
        subject: impl Into<String>,
    ) -> Result<Self, WebPushError> {
        let secret = SecretKey::from_slice(private_key_bytes)
            .map_err(|e| WebPushError::InvalidKey(e.to_string()))?;
        Self::from_secret(secret, subject.into())
    }

    fn from_secret(secret: SecretKey, subject: String) -> Result<Self, WebPushError> {
        let encoded = secret.public_key().to_encoded_point(false);
        let mut pub_bytes = [0u8; 65];
        pub_bytes.copy_from_slice(encoded.as_bytes());

        let pem = secret
            .to_pkcs8_pem(LineEnding::LF)
            .map_err(|e| WebPushError::InvalidKey(e.to_string()))?
            .to_string();

        Ok(Self {
            secret,
            pub_bytes,
            pem,
            subject,
        })
    }

    /// The application server key as uncompressed P-256 base64url (no padding).
    ///
    /// Pass this to `navigator.serviceWorker.pushManager.subscribe({ applicationServerKey })`.
    pub fn application_server_key(&self) -> String {
        URL_SAFE_NO_PAD.encode(self.pub_bytes)
    }

    /// The raw 32-byte private key scalar.  The consuming service stores this
    /// (encrypted) and re-constructs the `VapidKey` via [`VapidKey::from_bytes`].
    pub fn private_key_bytes(&self) -> Vec<u8> {
        self.secret.to_bytes().to_vec()
    }
}

/// Errors from Web Push operations.
#[derive(Debug, thiserror::Error)]
pub enum WebPushError {
    /// Invalid VAPID key material.
    #[error("invalid VAPID key: {0}")]
    InvalidKey(String),

    /// Invalid push subscription (bad base64, wrong key size, …).
    #[error("invalid subscription: {0}")]
    InvalidSubscription(String),

    /// Payload encryption failed.
    #[error("encryption failed: {0}")]
    Encrypt(String),

    /// `reqwest::Client` construction failed.
    #[error("HTTP client error: {0}")]
    HttpClient(String),

    /// JWT signing failed.
    #[error("VAPID JWT error: {0}")]
    Jwt(String),

    /// The push service returned 404 or 410 — the subscription is gone.
    #[error("subscription gone (status {0})")]
    Gone(u16),

    /// Network or unexpected HTTP error.
    #[error("push request error: {0}")]
    Request(String),

    /// The push service returned an unexpected error status.
    #[error("push provider error {0}")]
    Provider(u16),
}

// ── VAPID JWT claims ──────────────────────────────────────────────────────────

#[derive(Serialize)]
struct VapidClaims<'a> {
    aud: &'a str,
    sub: &'a str,
    exp: u64,
}

// ── Web Push client (stateless — no caching needed for VAPID JWT) ─────────────

/// Web Push client.  Stateless; construct per-send or share via `Arc`.
pub struct WebPushClient {
    http: reqwest::Client,
}

impl WebPushClient {
    /// Build a client with rustls TLS and default timeouts.
    pub fn new() -> Result<Self, WebPushError> {
        let http = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| WebPushError::HttpClient(e.to_string()))?;
        Ok(Self { http })
    }

    /// Encrypt `payload`, sign a VAPID JWT, and POST to `sub.endpoint`.
    ///
    /// - `ttl`: time-to-live in seconds for the push message at the push service.
    /// - Returns `Ok(())` on 201/200.
    /// - Returns [`WebPushError::Gone`] on 404/410 (subscription expired — the
    ///   consuming service should delete it).
    pub async fn send(
        &self,
        sub: &WebPushSubscription,
        payload: &[u8],
        vapid: &VapidKey,
        ttl: u32,
    ) -> Result<(), WebPushError> {
        // ── 1. Decode subscription ───────────────────────────────────────────
        let receiver_pub_bytes = URL_SAFE_NO_PAD
            .decode(&sub.p256dh)
            .map_err(|e| WebPushError::InvalidSubscription(format!("bad p256dh: {e}")))?;
        let auth_bytes = URL_SAFE_NO_PAD
            .decode(&sub.auth)
            .map_err(|e| WebPushError::InvalidSubscription(format!("bad auth: {e}")))?;

        // ── 2. Encrypt (RFC 8291) ────────────────────────────────────────────
        let mut salt = [0u8; 16];
        use aes_gcm::aead::rand_core::RngCore;
        OsRng.fill_bytes(&mut salt);

        let ephemeral = EphemeralSecret::random(&mut OsRng);
        let ciphertext =
            encrypt_payload(&receiver_pub_bytes, &auth_bytes, payload, &salt, &ephemeral)?;

        // ── 3. VAPID JWT (RFC 8292) ──────────────────────────────────────────
        let origin = endpoint_origin(&sub.endpoint)?;
        let jwt = vapid_jwt(&origin, &vapid.subject, &vapid.pem)?;
        let vapid_auth = format!(
            "vapid t={},k={}",
            jwt,
            URL_SAFE_NO_PAD.encode(vapid.pub_bytes)
        );

        // ── 4. POST ──────────────────────────────────────────────────────────
        let resp = self
            .http
            .post(&sub.endpoint)
            .header("Content-Encoding", "aes128gcm")
            .header("Content-Type", "application/octet-stream")
            .header("Authorization", vapid_auth)
            .header("TTL", ttl.to_string())
            .body(ciphertext)
            .send()
            .await
            .map_err(|e| WebPushError::Request(e.to_string()))?;

        let status = resp.status().as_u16();
        match status {
            200 | 201 => Ok(()),
            404 | 410 => Err(WebPushError::Gone(status)),
            _ => Err(WebPushError::Provider(status)),
        }
    }
}

impl Default for WebPushClient {
    fn default() -> Self {
        Self::new().expect("WebPushClient construction should not fail")
    }
}

// ── RFC 8291 payload encryption ───────────────────────────────────────────────

/// Encrypt `plaintext` following RFC 8291 aes128gcm content encoding.
///
/// `ephemeral` is the sender's ephemeral P-256 key (consumed to derive ECDH).
/// `salt` is a random 16-byte value (caller-generated).
///
/// Exposed as `pub(crate)` so tests can call with known inputs for the
/// round-trip decryption test.
pub(crate) fn encrypt_payload(
    receiver_pub_bytes: &[u8], // 65-byte uncompressed P-256 point
    auth_bytes: &[u8],         // 16-byte auth secret
    plaintext: &[u8],
    salt: &[u8; 16],
    ephemeral: &EphemeralSecret,
) -> Result<Vec<u8>, WebPushError> {
    // Decode receiver's public key
    let receiver_pub = PublicKey::from_sec1_bytes(receiver_pub_bytes)
        .map_err(|e| WebPushError::InvalidSubscription(format!("bad p256dh: {e}")))?;

    // ECDH
    let shared = ephemeral.diffie_hellman(&receiver_pub);
    let shared_bytes = shared.raw_secret_bytes();

    // Sender's uncompressed public key (65 bytes)
    let sender_pub = ephemeral.public_key();
    let sender_pub_encoded = sender_pub.to_encoded_point(false);
    let sender_pub_bytes = sender_pub_encoded.as_bytes(); // 65 bytes

    // IKM = HKDF-SHA256(salt=auth_bytes, IKM=ecdh_secret,
    //                    info="WebPush: info\0" || receiver_pub || sender_pub, L=32)
    let mut info = Vec::with_capacity(14 + 65 + 65);
    info.extend_from_slice(b"WebPush: info\x00");
    info.extend_from_slice(receiver_pub_bytes);
    info.extend_from_slice(sender_pub_bytes);

    let ikm = hkdf_expand(shared_bytes.as_ref(), Some(auth_bytes), &info, 32)
        .map_err(|_| WebPushError::Encrypt("HKDF IKM derivation failed".into()))?;

    // CEK = HKDF-SHA256(salt=salt, IKM=ikm, info="Content-Encoding: aes128gcm\0\1", L=16)
    let cek_bytes = hkdf_expand(&ikm, Some(salt), b"Content-Encoding: aes128gcm\x00\x01", 16)
        .map_err(|_| WebPushError::Encrypt("HKDF CEK derivation failed".into()))?;

    // nonce = HKDF-SHA256(salt=salt, IKM=ikm, info="Content-Encoding: nonce\0\1", L=12)
    let nonce_bytes = hkdf_expand(&ikm, Some(salt), b"Content-Encoding: nonce\x00\x01", 12)
        .map_err(|_| WebPushError::Encrypt("HKDF nonce derivation failed".into()))?;

    // AES-128-GCM encrypt: plaintext || 0x02 (final-record delimiter, RFC 8188)
    let key = Key::<Aes128Gcm>::from_slice(&cek_bytes);
    let cipher = Aes128Gcm::new(key);
    let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);

    let mut padded = plaintext.to_vec();
    padded.push(0x02); // record delimiter

    let ct = cipher
        .encrypt(nonce, padded.as_slice())
        .map_err(|_| WebPushError::Encrypt("AES-128-GCM encrypt failed".into()))?;

    // aes128gcm header (RFC 8188 §2.1):
    // salt(16) || rs(4 BE) || idlen(1) || keyid(idlen)
    // keyid = sender's ephemeral public key (65 bytes uncompressed)
    let rs: u32 = 4096;
    let idlen: u8 = sender_pub_bytes.len() as u8; // 65

    let mut out = Vec::with_capacity(16 + 4 + 1 + 65 + ct.len());
    out.extend_from_slice(salt);
    out.extend_from_slice(&rs.to_be_bytes());
    out.push(idlen);
    out.extend_from_slice(sender_pub_bytes);
    out.extend_from_slice(&ct);

    Ok(out)
}

/// Thin HKDF-SHA-256 wrapper (extract + expand).
fn hkdf_expand(ikm: &[u8], salt: Option<&[u8]>, info: &[u8], len: usize) -> Result<Vec<u8>, ()> {
    let hk = Hkdf::<Sha256>::new(salt, ikm);
    let mut okm = vec![0u8; len];
    hk.expand(info, &mut okm).map_err(|_| ())?;
    Ok(okm)
}

/// Extract the origin (`scheme://host[:port]`) from a push endpoint URL.
fn endpoint_origin(endpoint: &str) -> Result<String, WebPushError> {
    // ponytail: parse just enough of the URL — no url crate dep needed.
    // Endpoints are always https:// push service URLs.
    let rest = endpoint
        .strip_prefix("https://")
        .or_else(|| endpoint.strip_prefix("http://"))
        .ok_or_else(|| {
            WebPushError::InvalidSubscription("endpoint must start with http(s)://".into())
        })?;
    let host_and_path = rest.split_once('/').map(|(h, _)| h).unwrap_or(rest);
    let scheme = if endpoint.starts_with("https") {
        "https"
    } else {
        "http"
    };
    Ok(format!("{}://{}", scheme, host_and_path))
}

/// Build and sign a VAPID ES256 JWT.
fn vapid_jwt(aud: &str, sub: &str, pem: &str) -> Result<String, WebPushError> {
    let exp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        + 12 * 3600; // 12 h ≤ 24 h max per RFC 8292

    let claims = VapidClaims { aud, sub, exp };
    let header = Header::new(Algorithm::ES256);
    let key =
        EncodingKey::from_ec_pem(pem.as_bytes()).map_err(|e| WebPushError::Jwt(e.to_string()))?;
    encode(&header, &claims, &key).map_err(|e| WebPushError::Jwt(e.to_string()))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use p256::{
        ecdh::diffie_hellman,
        pkcs8::{DecodePrivateKey, EncodePublicKey, LineEnding as PkcsLineEnding},
    };
    use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

    // P-256 PKCS#8 private key (test-only, not a secret).
    const TEST_VAPID_P8: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgCXAxOl2tTfYLXIIF
BBKmIBF1NbWoicWu/sOpYibOHvyhRANCAAT3PRG7cUm++NdSzIh9GzodnZyIsCEO
FDQboAwKzY/mLSNWyca7QLSYE5PM7hzceb/FBYfKRUYyS2TgGAozFsB1
-----END PRIVATE KEY-----";

    #[test]
    fn vapid_key_generate_roundtrip() {
        let key = VapidKey::generate("mailto:test@example.com").unwrap();
        let pub_key_str = key.application_server_key();

        // Re-construct from the raw bytes and verify the public key matches
        let bytes = key.private_key_bytes();
        let key2 = VapidKey::from_bytes(&bytes, "mailto:test@example.com").unwrap();
        assert_eq!(key2.application_server_key(), pub_key_str);
    }

    #[test]
    fn vapid_application_server_key_is_uncompressed_p256() {
        let key = VapidKey::generate("mailto:test@example.com").unwrap();
        let ask = key.application_server_key();
        let decoded = URL_SAFE_NO_PAD.decode(&ask).unwrap();
        assert_eq!(decoded.len(), 65, "uncompressed P-256 key must be 65 bytes");
        assert_eq!(
            decoded[0], 0x04,
            "uncompressed P-256 key must start with 0x04"
        );
    }

    #[test]
    fn vapid_jwt_header_alg_is_es256_and_verifies() {
        use jsonwebtoken::{Algorithm, DecodingKey, Validation};

        let secret = SecretKey::from_pkcs8_pem(TEST_VAPID_P8).unwrap();
        let pub_pem = secret
            .public_key()
            .to_public_key_pem(PkcsLineEnding::LF)
            .unwrap();

        // Build the key manually
        let key = VapidKey::from_secret(secret, "mailto:test@example.com".into()).unwrap();
        let jwt = vapid_jwt("https://push.example.com", &key.subject, &key.pem).unwrap();

        // Check header
        let header_b64 = jwt.split('.').next().unwrap();
        let header_bytes = URL_SAFE_NO_PAD.decode(header_b64).unwrap();
        let header: serde_json::Value = serde_json::from_slice(&header_bytes).unwrap();
        assert_eq!(header["alg"], "ES256");

        // Verify signature with public key
        let dk = DecodingKey::from_ec_pem(pub_pem.as_bytes()).unwrap();
        let mut v = Validation::new(Algorithm::ES256);
        v.required_spec_claims = std::collections::HashSet::new();
        v.validate_exp = false;
        v.validate_aud = false; // audience validation requires knowing the aud ahead of time
        jsonwebtoken::decode::<serde_json::Value>(&jwt, &dk, &v).unwrap();
    }

    #[test]
    fn encrypt_payload_decrypt_roundtrip() {
        // Generate a receiver P-256 key pair (as a persistent SecretKey for decryption).
        let receiver_secret = SecretKey::random(&mut OsRng);
        let receiver_pub_encoded = receiver_secret.public_key().to_encoded_point(false);
        let receiver_pub_bytes = receiver_pub_encoded.as_bytes(); // 65 bytes

        let auth = b"test_auth_secret"; // 16 bytes

        let plaintext = b"Hello, WebPush round-trip test!";
        let salt = [0xabu8; 16];

        // Encrypt using a known ephemeral key
        let ephemeral = EphemeralSecret::random(&mut OsRng);
        // Capture the sender public key BEFORE consuming ephemeral
        let sender_pub = ephemeral.public_key();
        let sender_pub_encoded = sender_pub.to_encoded_point(false);
        let sender_pub_bytes = sender_pub_encoded.as_bytes();

        let ciphertext =
            encrypt_payload(receiver_pub_bytes, auth, plaintext, &salt, &ephemeral).unwrap();

        // ── Decrypt manually (receiver side) ──────────────────────────────────
        // Parse the aes128gcm header
        assert!(ciphertext.len() > 86, "header is 86 bytes minimum");
        let salt_from_header = &ciphertext[..16];
        // rs: bytes 16-19 (big-endian u32) — not needed for single-record decrypt
        let idlen = ciphertext[20] as usize;
        assert_eq!(idlen, 65);
        let keyid_bytes = &ciphertext[21..21 + idlen]; // ephemeral sender pub key
        assert_eq!(
            keyid_bytes, sender_pub_bytes,
            "keyid in header must be the ephemeral pub key"
        );
        let encrypted_record = &ciphertext[21 + idlen..];

        // ECDH: receiver_secret × sender_pub
        let sender_pub_key = PublicKey::from_sec1_bytes(sender_pub_bytes).unwrap();
        let shared = diffie_hellman(
            receiver_secret.to_nonzero_scalar(),
            sender_pub_key.as_affine(),
        );
        let shared_bytes = shared.raw_secret_bytes();

        // Re-derive IKM
        let mut info = Vec::new();
        info.extend_from_slice(b"WebPush: info\x00");
        info.extend_from_slice(receiver_pub_bytes);
        info.extend_from_slice(sender_pub_bytes);
        let ikm = hkdf_expand(shared_bytes.as_ref(), Some(auth.as_ref()), &info, 32).unwrap();

        // Re-derive CEK + nonce
        let cek_bytes = hkdf_expand(
            &ikm,
            Some(salt_from_header),
            b"Content-Encoding: aes128gcm\x00\x01",
            16,
        )
        .unwrap();
        let nonce_bytes = hkdf_expand(
            &ikm,
            Some(salt_from_header),
            b"Content-Encoding: nonce\x00\x01",
            12,
        )
        .unwrap();

        // AES-128-GCM decrypt
        let key = Key::<Aes128Gcm>::from_slice(&cek_bytes);
        let cipher = Aes128Gcm::new(key);
        let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);
        let decrypted = cipher.decrypt(nonce, encrypted_record).unwrap();

        // Remove the 0x02 record delimiter
        assert_eq!(decrypted.last(), Some(&0x02u8));
        let recovered = &decrypted[..decrypted.len() - 1];
        assert_eq!(recovered, plaintext);
    }

    #[tokio::test]
    async fn webpush_send_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;

        let client = WebPushClient::new().unwrap();
        let vapid = VapidKey::generate("mailto:test@example.com").unwrap();

        // Use real receiver keys
        let receiver_secret = SecretKey::random(&mut OsRng);
        let auth = b"0123456789abcdef"; // 16 bytes
        let receiver_pub_encoded = receiver_secret.public_key().to_encoded_point(false);

        let sub = WebPushSubscription {
            endpoint: server.uri() + "/push",
            p256dh: URL_SAFE_NO_PAD.encode(receiver_pub_encoded.as_bytes()),
            auth: URL_SAFE_NO_PAD.encode(auth),
        };

        client
            .send(&sub, b"test payload", &vapid, 86400)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn webpush_send_410_returns_gone() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(410))
            .mount(&server)
            .await;

        let client = WebPushClient::new().unwrap();
        let vapid = VapidKey::generate("mailto:test@example.com").unwrap();
        let receiver_secret = SecretKey::random(&mut OsRng);
        let auth = b"0123456789abcdef";
        let receiver_pub_encoded = receiver_secret.public_key().to_encoded_point(false);

        let sub = WebPushSubscription {
            endpoint: server.uri() + "/push",
            p256dh: URL_SAFE_NO_PAD.encode(receiver_pub_encoded.as_bytes()),
            auth: URL_SAFE_NO_PAD.encode(auth),
        };

        let err = client
            .send(&sub, b"payload", &vapid, 3600)
            .await
            .unwrap_err();
        assert!(matches!(err, WebPushError::Gone(410)));
    }

    #[test]
    fn endpoint_origin_parses_correctly() {
        assert_eq!(
            endpoint_origin("https://fcm.googleapis.com/fcm/send/abc123").unwrap(),
            "https://fcm.googleapis.com"
        );
        assert_eq!(
            endpoint_origin("https://updates.push.services.mozilla.com/push/v1/abc").unwrap(),
            "https://updates.push.services.mozilla.com"
        );
    }
}
