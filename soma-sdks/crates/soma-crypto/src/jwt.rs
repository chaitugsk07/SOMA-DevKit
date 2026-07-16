use std::collections::HashMap;

use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Claims for an ES256 access token.
/// `perms` are resolved at issue time from the RBAC engine.
///
/// # Reserved claims
/// `ver` is reserved for a future breaking-field version discriminator. Do NOT add `ver` until
/// a truly breaking change (new required field with no sensible default) makes it necessary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub iss: String,
    pub sub: String, // user_id as hyphenated UUID string
    pub aud: Vec<String>,
    pub exp: usize, // Unix timestamp seconds
    pub iat: usize,
    pub jti: String, // UUID v4 (mandatory for revocation)
    pub tid: Uuid,   // tenant_id
    pub mid: Uuid,   // membership_id (active at issue time)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wid: Option<Uuid>, // workspace_id (None for team members)
    pub perms: Vec<String>, // permission codes resolved at issue time
    /// E.164 phone number. Present when the login proved phone possession (phone-OTP flow).
    /// PII note: services MUST NOT log Authorization headers containing this claim.
    /// Survives token refresh by design — refresh handler re-reads fct_users.phone.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub phn: Option<String>,
    /// Global platform-admin flag, mirrored from fct_users.is_platform_admin.
    /// `serde(default)` ensures old tokens without this field deserialize to false.
    #[serde(default)]
    pub platform_admin: bool,
    /// OAuth2 client_id for machine tokens issued via the client_credentials grant.
    /// Present only when `sub` is a client_id (not a user UUID). Used by /oidc/introspect
    /// to check whether the issuing client is still active.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub client_id: Option<String>,
    /// Granted scope string (space-separated). Set for machine tokens (client_credentials);
    /// absent for user tokens. Echoed verbatim by /oidc/introspect (RFC 7662 §2.2).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scope: Option<String>,
    /// MFA enrollment is required but not yet completed for this session.
    /// When true, the session may only reach MFA-enrollment routes.
    /// `skip_serializing_if` ensures old tokens without this field work (defaults to None/false).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mfa_pending: Option<bool>,
}

/// Issue an ES256 access token.
/// `key_id` becomes the `kid` header (UUID of the signing key in fct_signing_keys).
pub fn issue_access_token(
    claims: &AccessTokenClaims,
    key_id: Uuid,
    encoding_key: &EncodingKey,
) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header {
        alg: Algorithm::ES256,
        kid: Some(key_id.to_string()),
        ..Default::default()
    };
    encode(&header, claims, encoding_key)
}

/// Issue an ES256 access token with extra custom claims injected.
///
/// Merge order: `extra` claims (lowest priority) → system `claims` (always win).
/// The caller is responsible for stripping RFC 9068 reserved names from `extra`
/// before passing them here — this function does not re-check.
///
/// Fast-paths to `issue_access_token` when `extra` is empty.
pub fn issue_access_token_with_extra(
    claims: &AccessTokenClaims,
    extra: &serde_json::Map<String, serde_json::Value>,
    key_id: Uuid,
    encoding_key: &EncodingKey,
) -> Result<String, jsonwebtoken::errors::Error> {
    if extra.is_empty() {
        return issue_access_token(claims, key_id, encoding_key);
    }
    let header = Header {
        alg: Algorithm::ES256,
        kid: Some(key_id.to_string()),
        ..Default::default()
    };
    // Build merged payload: insert extra first (low priority), then overlay system claims.
    let mut merged = extra.clone();
    if let serde_json::Value::Object(system) =
        serde_json::to_value(claims).expect("AccessTokenClaims always serializes")
    {
        // extend overwrites: system claims always win
        merged.extend(system);
    }
    encode(&header, &serde_json::Value::Object(merged), encoding_key)
}

/// Verify an ES256 access token.
///
/// Security invariants:
/// - ALWAYS uses `Validation::new(Algorithm::ES256)` — never `Validation::default()`
/// - The `alg` field in the token header is NEVER used to select the verification algorithm
/// - `kid` in the header is used only for key lookup; the algorithm is fixed to ES256
/// - `iss` and `aud` are validated; `exp` is checked with leeway=0
pub fn verify_access_token(
    token: &str,
    issuer: &str,
    audience: &str,
    decoding_keys: &HashMap<Uuid, DecodingKey>,
) -> Result<AccessTokenClaims, jsonwebtoken::errors::Error> {
    // Decode header unauthenticated to get kid. The alg field here is IGNORED for algorithm selection.
    let header = decode_header(token)?;
    let kid_str = header.kid.ok_or_else(|| {
        jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken)
    })?;
    let kid = kid_str.parse::<Uuid>().map_err(|_| {
        jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken)
    })?;
    let decoding_key = decoding_keys.get(&kid).ok_or_else(|| {
        jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidKeyFormat)
    })?;

    // SECURITY: Always use explicit ES256. Never trust the header's alg field.
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[audience]);
    validation.validate_exp = true;
    validation.leeway = 0;

    let data = decode::<AccessTokenClaims>(token, decoding_key, &validation)?;
    Ok(data.claims)
}

/// Claims for an OIDC ID token (distinct from the API access token).
/// `aud` is the OAuth client_id (not "soma-iam").
/// `nonce` must be reflected verbatim from the authorization request if provided.
/// `at_hash` is required when an access token is issued in the same response (OIDC Core §3.3.2.11).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdTokenClaims {
    pub iss: String,
    pub sub: String,      // user_id (stable per user)
    pub aud: Vec<String>, // client_id — NOT "soma-iam"
    pub exp: usize,
    pub iat: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_hash: Option<String>, // BASE64URL-NOPAD(SHA256(access_token)[0..16])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
}

/// Issue an ES256 ID token. Same key/kid as access tokens.
pub fn issue_id_token(
    claims: &IdTokenClaims,
    key_id: Uuid,
    encoding_key: &EncodingKey,
) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header {
        alg: Algorithm::ES256,
        kid: Some(key_id.to_string()),
        ..Default::default()
    };
    encode(&header, claims, encoding_key)
}

/// Compute `at_hash` for inclusion in the ID token.
/// Per OIDC Core §3.3.2.11: BASE64URL-NOPAD(SHA256(access_token)[0..16])
pub fn compute_at_hash(access_token: &str) -> String {
    use base64::Engine;
    let digest = sha256_bytes(access_token.as_bytes());
    let half = &digest[..16]; // left half of 32-byte SHA-256 digest
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(half)
}

use crate::signing::sha256_bytes;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signing::{generate_es256_keypair, signing_key_to_pem, verifying_key_to_pem};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_keys() -> (HashMap<Uuid, DecodingKey>, EncodingKey, Uuid) {
        let (ref sk, vk) = generate_es256_keypair();
        let kid = Uuid::new_v4();
        let ek = EncodingKey::from_ec_pem(&signing_key_to_pem(sk)).unwrap();
        let dk = DecodingKey::from_ec_pem(&verifying_key_to_pem(&vk)).unwrap();
        let mut map = HashMap::new();
        map.insert(kid, dk);
        (map, ek, kid)
    }

    fn future_exp() -> usize {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
            + 900
    }

    fn now_iat() -> usize {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
    }

    fn make_claims(exp: usize) -> AccessTokenClaims {
        AccessTokenClaims {
            iss: "https://iam.test".into(),
            sub: Uuid::new_v4().to_string(),
            aud: vec!["soma-iam".into()],
            exp,
            iat: now_iat(),
            jti: Uuid::new_v4().to_string(),
            tid: Uuid::new_v4(),
            mid: Uuid::new_v4(),
            wid: None,
            perms: vec![],
            phn: None,
            platform_admin: false,
            client_id: None,
            scope: None,
            mfa_pending: None,
        }
    }

    #[test]
    fn valid_token_roundtrip() {
        let (dks, ek, kid) = test_keys();
        let claims = make_claims(future_exp());
        let token = issue_access_token(&claims, kid, &ek).unwrap();
        let verified = verify_access_token(&token, "https://iam.test", "soma-iam", &dks).unwrap();
        assert_eq!(verified.sub, claims.sub);
        assert_eq!(verified.jti, claims.jti);
    }

    #[test]
    fn alg_none_rejected() {
        use base64::Engine;
        let (dks, _, kid) = test_keys();
        let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let header = engine.encode(
            serde_json::json!({"alg": "none", "typ": "JWT", "kid": kid.to_string()}).to_string(),
        );
        let payload = engine.encode(
            serde_json::json!({
                "iss": "https://iam.test",
                "sub": "user",
                "aud": ["soma-iam"],
                "exp": future_exp(),
                "iat": now_iat(),
                "jti": Uuid::new_v4().to_string(),
                "tid": Uuid::new_v4().to_string(),
                "mid": Uuid::new_v4().to_string(),
                "perms": []
            })
            .to_string(),
        );
        let token = format!("{}.{}.", header, payload);
        assert!(
            verify_access_token(&token, "https://iam.test", "soma-iam", &dks).is_err(),
            "alg=none token must be rejected"
        );
    }

    #[test]
    fn tampered_signature_rejected() {
        let (dks, ek, kid) = test_keys();
        let token = issue_access_token(&make_claims(future_exp()), kid, &ek).unwrap();
        let mut tampered = token.clone();
        let last = tampered.pop().unwrap();
        tampered.push(if last == 'a' { 'b' } else { 'a' });
        assert!(
            verify_access_token(&tampered, "https://iam.test", "soma-iam", &dks).is_err(),
            "tampered signature must be rejected"
        );
    }

    #[test]
    fn expired_token_rejected() {
        let (dks, ek, kid) = test_keys();
        let token = issue_access_token(&make_claims(1), kid, &ek).unwrap(); // exp=1 = past
        assert!(
            verify_access_token(&token, "https://iam.test", "soma-iam", &dks).is_err(),
            "expired token must be rejected"
        );
    }

    #[test]
    fn wrong_kid_rejected() {
        let (dks, ek, kid) = test_keys();
        let wrong_kid = Uuid::new_v4(); // not in decoding_keys
        let token = issue_access_token(&make_claims(future_exp()), wrong_kid, &ek).unwrap();
        // verify with dks that don't have wrong_kid
        assert!(
            verify_access_token(&token, "https://iam.test", "soma-iam", &dks).is_err(),
            "token with unknown kid must be rejected"
        );
        // also: a token issued with kid=kid but verified with map containing only wrong_kid
        let token2 = issue_access_token(&make_claims(future_exp()), kid, &ek).unwrap();
        let mut empty_dks = HashMap::new();
        empty_dks.insert(wrong_kid, dks.into_values().next().unwrap());
        assert!(
            verify_access_token(&token2, "https://iam.test", "soma-iam", &empty_dks).is_err(),
            "token whose kid is not in decoding_keys must be rejected"
        );
    }

    #[test]
    fn wrong_audience_rejected() {
        let (dks, ek, kid) = test_keys();
        let token = issue_access_token(&make_claims(future_exp()), kid, &ek).unwrap();
        assert!(
            verify_access_token(&token, "https://iam.test", "not-soma-iam", &dks).is_err(),
            "wrong audience must be rejected"
        );
    }

    #[test]
    fn wrong_issuer_rejected() {
        let (dks, ek, kid) = test_keys();
        let token = issue_access_token(&make_claims(future_exp()), kid, &ek).unwrap();
        assert!(
            verify_access_token(&token, "https://evil.example.com", "soma-iam", &dks).is_err(),
            "wrong issuer must be rejected"
        );
    }
}
