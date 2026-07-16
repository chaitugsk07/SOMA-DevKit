use base64::Engine;
use p256::{
    ecdsa::{SigningKey, VerifyingKey},
    pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey},
    EncodedPoint,
};
use rand::rngs::OsRng;
use serde_json::Value;
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::{decrypt, encrypt, CryptoError, CryptoKey};

pub const SIGNING_KEY_AAD: &[u8] = b"soma_iam:fct_signing_keys:private_key_enc";

/// Generate a fresh ES256 key pair.
/// The returned `SigningKey` implements `ZeroizeOnDrop` — key material is cleared on drop.
pub fn generate_es256_keypair() -> (SigningKey, VerifyingKey) {
    let sk = SigningKey::random(&mut OsRng);
    let vk = *sk.verifying_key();
    (sk, vk)
}

/// AES-256-GCM wrap the private key PKCS8 DER bytes.
/// AAD: `soma_iam:fct_signing_keys:private_key_enc`
pub fn wrap_signing_key(
    key: &SigningKey,
    wrapping_key: &CryptoKey,
) -> Result<Vec<u8>, CryptoError> {
    let der = key.to_pkcs8_der().map_err(|_| CryptoError::Encrypt)?;
    encrypt(wrapping_key, der.as_bytes(), SIGNING_KEY_AAD)
}

/// Unwrap and reconstruct a signing key from the encrypted DER bytes.
/// The plaintext DER bytes are wrapped in `Zeroizing` so they are cleared on drop.
pub fn unwrap_signing_key(enc: &[u8], wrapping_key: &CryptoKey) -> Result<SigningKey, CryptoError> {
    let der = Zeroizing::new(decrypt(wrapping_key, enc, SIGNING_KEY_AAD)?);
    let sk = SigningKey::from_pkcs8_der(&der).map_err(|_| CryptoError::Decrypt)?;
    Ok(sk)
}

/// Serialize a verifying key as a JWK (public key only; no secret material).
pub fn verifying_key_to_jwk(key_id: Uuid, key: &VerifyingKey) -> Value {
    let ep = key.to_encoded_point(false);
    let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let x = engine.encode(ep.x().expect("non-identity point"));
    let y = engine.encode(ep.y().expect("non-identity point"));
    serde_json::json!({
        "kty": "EC",
        "crv": "P-256",
        "alg": "ES256",
        "use": "sig",
        "kid": key_id.to_string(),
        "x": x,
        "y": y,
    })
}

/// Reconstruct a verifying key from a stored JWK value.
pub fn vk_from_jwk(jwk: &Value) -> anyhow::Result<VerifyingKey> {
    let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let x_str = jwk["x"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("JWK missing x"))?;
    let y_str = jwk["y"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("JWK missing y"))?;
    let x = engine.decode(x_str)?;
    let y = engine.decode(y_str)?;
    let mut point_bytes = vec![0x04u8];
    point_bytes.extend_from_slice(&x);
    point_bytes.extend_from_slice(&y);
    let ep = EncodedPoint::from_bytes(&point_bytes)
        .map_err(|_| anyhow::anyhow!("invalid EC point bytes"))?;
    VerifyingKey::from_encoded_point(&ep).map_err(|_| anyhow::anyhow!("invalid EC point"))
}

/// Serialize a set of keys as a JWKS JSON response body (bytes for serving directly).
pub fn build_jwks_bytes(keys: &[(Uuid, VerifyingKey)]) -> Vec<u8> {
    let jwks = serde_json::json!({
        "keys": keys.iter().map(|(id, k)| verifying_key_to_jwk(*id, k)).collect::<Vec<_>>()
    });
    serde_json::to_vec(&jwks).expect("jwks serialization is infallible")
}

/// Export verifying key as SPKI PEM for use with `jsonwebtoken::DecodingKey::from_ec_pem`.
pub fn verifying_key_to_pem(key: &VerifyingKey) -> Vec<u8> {
    key.to_public_key_pem(Default::default())
        .expect("PEM encoding is infallible")
        .into_bytes()
}

/// Export signing key as PKCS8 PEM for use with `jsonwebtoken::EncodingKey::from_ec_pem`.
/// The bytes are wrapped in `Zeroizing` so they are cleared on drop.
pub fn signing_key_to_pem(key: &SigningKey) -> Zeroizing<Vec<u8>> {
    Zeroizing::new(
        key.to_pkcs8_pem(Default::default())
            .expect("PEM encoding is infallible")
            .as_bytes()
            .to_vec(),
    )
}

/// SHA-256 of `data` as a raw 32-byte digest.
/// Use this for PKCE S256 verification and `at_hash` computation.
pub fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}
