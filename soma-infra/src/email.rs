//! Generic SMTP email client with optional DKIM signing.
//!
//! Pure mechanism: configured lettre SMTP transport (rustls, pooled) + a thin DKIM
//! signer wrapper.  No relay-selection, retry policy, suppression, or template
//! rendering — those are the consuming service's policy.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use soma_infra::email::{SmtpConfig, SmtpClient, DkimSigner, RawEmail, TlsMode};
//!
//! # async fn example() -> Result<(), soma_infra::email::EmailError> {
//! let client = SmtpClient::new(SmtpConfig {
//!     host: "smtp.relay.example".to_string(),
//!     port: 587,
//!     username: Some("user".to_string()),
//!     password: Some("secret".to_string()),
//!     tls: TlsMode::StartTls,
//!     timeout: None,
//! })?;
//!
//! let response = client.send(RawEmail {
//!     from: "noreply@example.com".to_string(),
//!     from_name: Some("Example".to_string()),
//!     to: "recipient@example.org".to_string(),
//!     reply_to: None,
//!     subject: "Hello".to_string(),
//!     text: Some("Hello, world!".to_string()),
//!     html: None,
//!     headers: vec![],
//! }).await?;
//! println!("SMTP response: {response}");
//! # Ok(())
//! # }
//! ```

use std::time::Duration;

use lettre::{
    message::{
        dkim::{
            DkimCanonicalization, DkimCanonicalizationType, DkimConfig, DkimSigningAlgorithm,
            DkimSigningKey,
        },
        header::{HeaderName, HeaderValue},
        Mailbox, Message, MultiPart, SinglePart,
    },
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
        AsyncSmtpTransport,
    },
    AsyncTransport, Tokio1Executor,
};

/// SMTP TLS mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsMode {
    /// Implicit TLS from the first byte (SMTPS, typically port 465).
    Tls,
    /// STARTTLS upgrade after plain-text greeting (typically port 587).
    StartTls,
    /// No TLS — plaintext only.  Use for local/testing relay on port 25.
    None,
}

/// Configuration for a single SMTP relay connection.
#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    /// SMTP AUTH username; `None` for anonymous relay.
    pub username: Option<String>,
    /// SMTP AUTH password; `None` for anonymous relay.
    pub password: Option<String>,
    pub tls: TlsMode,
    /// Per-connection timeout.  `None` uses the lettre default (60 s).
    pub timeout: Option<Duration>,
}

impl SmtpConfig {
    /// Build a `SmtpConfig` from environment variables, returning `None` when
    /// `SOMA_SMTP_HOST` is unset or empty.
    ///
    /// | Var | Default | Notes |
    /// |---|---|---|
    /// | `SOMA_SMTP_HOST` | — | Required. Empty → `None`. |
    /// | `SOMA_SMTP_PORT` | `587` | |
    /// | `SOMA_SMTP_USERNAME` | — | Optional. |
    /// | `SOMA_SMTP_PASSWORD_FILE` | — | File path; content used as password (leading/trailing whitespace stripped). |
    /// | `SOMA_SMTP_PASSWORD` | — | Direct value; used when `SOMA_SMTP_PASSWORD_FILE` is unset. |
    /// | `SOMA_SMTP_TLS` | `starttls` | `"tls"` / `"implicit"` → implicit TLS (port 465); `"none"` / `"plaintext"` → no TLS. |
    /// | `SOMA_SMTP_TIMEOUT_SECS` | — | Integer seconds; `None` uses lettre's 60 s default. |
    pub fn from_env() -> Option<Self> {
        let host = std::env::var("SOMA_SMTP_HOST")
            .ok()
            .filter(|v| !v.is_empty())?;
        let port = std::env::var("SOMA_SMTP_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(587u16);
        let username = std::env::var("SOMA_SMTP_USERNAME")
            .ok()
            .filter(|v| !v.is_empty());
        // File takes precedence over direct env var (matches secret-injection patterns).
        let password = std::env::var("SOMA_SMTP_PASSWORD_FILE")
            .ok()
            .filter(|v| !v.is_empty())
            .and_then(|f| std::fs::read_to_string(f).ok())
            .map(|s| s.trim().to_string())
            .or_else(|| {
                std::env::var("SOMA_SMTP_PASSWORD")
                    .ok()
                    .filter(|v| !v.is_empty())
            });
        let tls = match std::env::var("SOMA_SMTP_TLS").as_deref() {
            Ok("tls") | Ok("implicit") => TlsMode::Tls,
            Ok("none") | Ok("plaintext") => TlsMode::None,
            _ => TlsMode::StartTls,
        };
        let timeout = std::env::var("SOMA_SMTP_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_secs);
        Some(SmtpConfig {
            host,
            port,
            username,
            password,
            tls,
            timeout,
        })
    }
}

/// A raw email ready to hand to the SMTP transport.
///
/// At least one of [`text`][RawEmail::text] or [`html`][RawEmail::html] must be `Some`.
/// When both are provided the message is sent as `multipart/alternative`.
#[derive(Debug, Clone)]
pub struct RawEmail {
    /// RFC 5321 envelope / From address, e.g. `"noreply@example.com"`.
    pub from: String,
    /// Optional display name for the From header, e.g. `"Acme Notifications"`.
    pub from_name: Option<String>,
    /// Recipient address.
    pub to: String,
    /// Optional Reply-To address.
    pub reply_to: Option<String>,
    pub subject: String,
    /// Plain-text body.
    pub text: Option<String>,
    /// HTML body.
    pub html: Option<String>,
    /// Extra headers, e.g. `("List-Unsubscribe", "<https://...>")`.
    /// Values are assumed to be ASCII and already correctly formatted.
    pub headers: Vec<(String, String)>,
}

/// DKIM signer — a thin wrapper around lettre's `DkimConfig`.
///
/// The consuming service owns and decrypts the private key; this type is pure
/// mechanism: given a key, sign a `Message` before send.
///
/// # DKIM path chosen
/// lettre 0.11 includes a `dkim` feature (RSA-SHA256 via the `rsa` crate, no
/// external dependencies).  We use it directly.  RFC 6376 relaxed/relaxed
/// canonicalization; signed headers: From, Subject, To, Date.
pub struct DkimSigner {
    /// ponytail: hold lettre's DkimConfig directly — no re-wrapping
    config: DkimConfig,
}

impl DkimSigner {
    /// Build a DKIM signer for `domain` + DNS `selector` using an RSA private
    /// key in PKCS#1 PEM format.  The key material comes from the caller — the
    /// service decrypts it from its own store before passing it here.
    ///
    /// Canonicalization: `relaxed/relaxed` (RFC 6376 §3.4).
    /// Signed headers: From, Subject, To, Date.
    pub fn new(
        domain: impl Into<String>,
        selector: impl Into<String>,
        rsa_private_key_pem: &str,
    ) -> Result<Self, EmailError> {
        let signing_key = DkimSigningKey::new(rsa_private_key_pem, DkimSigningAlgorithm::Rsa)
            .map_err(|e| EmailError::DkimKey(e.to_string()))?;
        let config = DkimConfig::new(
            selector.into(),
            domain.into(),
            signing_key,
            vec![
                HeaderName::new_from_ascii_str("From"),
                HeaderName::new_from_ascii_str("Subject"),
                HeaderName::new_from_ascii_str("To"),
                HeaderName::new_from_ascii_str("Date"),
            ],
            DkimCanonicalization {
                header: DkimCanonicalizationType::Relaxed,
                body: DkimCanonicalizationType::Relaxed,
            },
        );
        Ok(Self { config })
    }
}

/// Pooled async SMTP client with optional DKIM signing.
///
/// Build with [`SmtpClient::new`]; attach DKIM via [`SmtpClient::with_dkim`].
/// The lettre connection pool is shared across all [`send`][SmtpClient::send] calls.
pub struct SmtpClient {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    dkim: Option<DkimSigner>,
}

impl SmtpClient {
    /// Build a pooled SMTP transport from `cfg`.
    pub fn new(cfg: SmtpConfig) -> Result<Self, EmailError> {
        let builder = match cfg.tls {
            TlsMode::Tls => {
                let tls = TlsParameters::new(cfg.host.clone())
                    .map_err(|e| EmailError::Config(e.to_string()))?;
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&cfg.host)
                    .port(cfg.port)
                    .tls(Tls::Wrapper(tls))
            }
            TlsMode::StartTls => {
                let tls = TlsParameters::new(cfg.host.clone())
                    .map_err(|e| EmailError::Config(e.to_string()))?;
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&cfg.host)
                    .port(cfg.port)
                    .tls(Tls::Required(tls))
            }
            TlsMode::None => {
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&cfg.host).port(cfg.port)
            }
        };

        let builder = match (cfg.username, cfg.password) {
            (Some(user), Some(pass)) => builder.credentials(Credentials::new(user, pass)),
            _ => builder,
        };

        let builder = if let Some(t) = cfg.timeout {
            builder.timeout(Some(t))
        } else {
            builder
        };

        Ok(Self {
            transport: builder.build(),
            dkim: None,
        })
    }

    /// Attach a [`DkimSigner`].  Chainable at construction time.
    ///
    /// ```rust,no_run
    /// # use soma_infra::email::{SmtpClient, SmtpConfig, DkimSigner, TlsMode};
    /// # let cfg = SmtpConfig { host: "h".into(), port: 25, username: None, password: None, tls: TlsMode::None, timeout: None };
    /// # let pem = "";
    /// let client = SmtpClient::new(cfg)
    ///     .unwrap()
    ///     .with_dkim(DkimSigner::new("example.com", "sel", pem).unwrap());
    /// ```
    pub fn with_dkim(mut self, dkim: DkimSigner) -> Self {
        self.dkim = Some(dkim);
        self
    }

    /// Send `msg` via SMTP, optionally applying DKIM signing.
    ///
    /// Returns the SMTP server response string (which typically embeds the
    /// message-id assigned by the relay).
    pub async fn send(&self, msg: RawEmail) -> Result<String, EmailError> {
        let mut message = build_message(&msg)?;

        // Add extra headers before DKIM so they are visible to the signer
        // (though we don't include them in the signed h= list by default —
        // List-Unsubscribe is typically not DKIM-signed in practice).
        for (name, value) in msg.headers {
            let header_name = HeaderName::new_from_ascii(name)
                .map_err(|_| EmailError::Build("header name contains non-ASCII bytes".into()))?;
            message
                .headers_mut()
                .insert_raw(HeaderValue::new(header_name, value));
        }

        if let Some(dkim) = &self.dkim {
            message.sign(&dkim.config);
        }

        let response = self.transport.send(message).await.map_err(map_smtp_error)?;

        // Collect response lines (the relay usually puts the message-id here)
        Ok(response.message().collect::<Vec<_>>().join(" "))
    }
}

/// Build a lettre [`Message`] from a [`RawEmail`] (custom headers and DKIM are
/// applied by the caller after this returns).
fn build_message(msg: &RawEmail) -> Result<Message, EmailError> {
    let from: Mailbox = match &msg.from_name {
        Some(name) => format!("{} <{}>", name, msg.from)
            .parse()
            .map_err(|e| EmailError::Build(format!("invalid from address: {e}")))?,
        None => msg
            .from
            .parse()
            .map_err(|e| EmailError::Build(format!("invalid from address: {e}")))?,
    };

    let to: Mailbox = msg
        .to
        .parse()
        .map_err(|e| EmailError::Build(format!("invalid to address: {e}")))?;

    let mut builder = Message::builder()
        .from(from)
        .to(to)
        .subject(msg.subject.clone());

    if let Some(rt) = &msg.reply_to {
        let reply_to: Mailbox = rt
            .parse()
            .map_err(|e| EmailError::Build(format!("invalid reply-to address: {e}")))?;
        builder = builder.reply_to(reply_to);
    }

    let message = match (&msg.text, &msg.html) {
        (Some(t), Some(h)) => builder
            .multipart(MultiPart::alternative_plain_html(t.clone(), h.clone()))
            .map_err(|e| EmailError::Build(e.to_string()))?,
        (Some(t), None) => builder
            .singlepart(SinglePart::plain(t.clone()))
            .map_err(|e| EmailError::Build(e.to_string()))?,
        (None, Some(h)) => builder
            .singlepart(SinglePart::html(h.clone()))
            .map_err(|e| EmailError::Build(e.to_string()))?,
        (None, None) => {
            return Err(EmailError::Build(
                "at least one of text or html is required".into(),
            ))
        }
    };

    Ok(message)
}

/// Map a lettre SMTP error to [`EmailError`], distinguishing transient from permanent.
///
/// lettre's `Error::is_transient()` covers 4xx responses; `is_permanent()` covers
/// 5xx.  Connection/IO/TLS errors are treated as transient (likely retryable).
fn map_smtp_error(e: lettre::transport::smtp::Error) -> EmailError {
    if e.is_permanent() {
        EmailError::Permanent(e.to_string())
    } else {
        // Covers: is_transient() (4xx), connection errors, IO errors, TLS errors.
        // ponytail: treat unknown categories as transient — the worst outcome is
        // an unnecessary retry, not a dropped message.
        EmailError::Transient(e.to_string())
    }
}

/// Errors from email operations.
#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    /// SMTP transport setup failure (invalid host, TLS configuration).  Not retryable.
    #[error("SMTP configuration error: {0}")]
    Config(String),

    /// Message construction failure (malformed address, missing body).  Not retryable.
    #[error("message build error: {0}")]
    Build(String),

    /// DKIM key parse or signing failure.  Not retryable.
    #[error("DKIM key error: {0}")]
    DkimKey(String),

    /// Transient SMTP error — connection failure, timeout, 4xx greylisting.  **Retryable.**
    #[error("transient SMTP error (retryable): {0}")]
    Transient(String),

    /// Permanent SMTP error — 5xx bad recipient, spam rejection.  Not retryable.
    #[error("permanent SMTP error: {0}")]
    Permanent(String),
}

impl EmailError {
    /// Returns `true` if the send should be retried (transient failure).
    pub fn is_transient(&self) -> bool {
        matches!(self, Self::Transient(_))
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── RSA test key (PKCS#1 PEM, 2048-bit) ─────────────────────────────────
    // This is the same key embedded in lettre's own DKIM test suite.  It is a
    // public test key, not a secret.
    const TEST_RSA_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAwOsW7UFcWn1ch3UM8Mll5qZH5hVHKJQ8Z0tUlebUECq0vjw6
VcsIucZ/B70VpCN63whyi7oApdCIS1o0zad7f0UaW/BfxXADqdcFL36uMaG0RHer
uSASjQGnsl9Kozt/dXiDZX5ngjr/arLJhNZSNR4/9VSwqbE2OPXaSaQ9BsqneD0P
8dCVSfkkDZCcfC2864z7hvC01lFzWQKF36ZAoGBERHScHtFMAzUOgGuqqPiP5khw
DQB3Ffccf+BsWLU2OOteshUwTGjpoangbPCYj6kckwNm440lQwuqTinpC92yyIE5
Ol8psNMW49DLowAeZb6JrjLhD+wY9bghTaOkcwIDAQABAoIBAHTZ8LkkrdvhsvoZ
XA088AwVC9fBa6iYoT2v0zw45JomQ/Q2Zt8wa8ibAradQU56byJI65jWwS2ucd+y
c+ldWOBt6tllb50XjCCDrRBnmvtVBuux0MIBOztNlVXlgj/8+ecdZ/lB51Bqi+sF
ACsF5iVmfTcMZTVjsYQu5llUseI6Lwgqpx6ktaXD2PVsVo9Gf01ssZ4GCy69wB/3
20CsOz4LEpSYkq1oE98lMMGCfD7py3L9kWHYNNisam78GM+1ynRxRGwEDUbz6pxs
fGPIAwHLaZsOmibPkBB0PJTW742w86qQ8KAqC6ZbRYOF19rSMj3oTfRnPMHn9Uu5
N8eQcoECgYEA97SMUrz2hqII5i8igKylO9kV8pjcIWKI0rdt8MKj4FXTNYjjO9I+
41ONOjhUOpFci/G3YRKi8UiwbKxIRTvIxNMh2xj6Ws3iO9gQHK1j8xTWxJdjEBEz
EuZI59Mi5H7fxSL1W+n8nS8JVsaH93rvQErngqTUAsihAzjxHWdFwm0CgYEAx2Dh
claESJP2cOKgYp+SUNwc26qMaqnl1f37Yn+AflrQOfgQqJe5TRbicEC+nFlm6XUt
3st1Nj29H0uOMmMZDmDCO+cOs5Qv5A9pG6jSC6wM+2KNHQDtrxlakBFygePEPVVy
GXaY9DRa9Q4/4ataxDR2/VvIAWfEEtMTJIBDtl8CgYAIXEuwLziS6r0qJ8UeWrVp
A7a97XLgnZbIpfBMBAXL+JmcYPZqenos6hEGOgh9wZJCFvJ9kEd3pWBvCpGV5KKu
IgIuhvVMQ06zfmNs1F1fQwDMud9aF3qF1Mf5KyMuWynqWXe2lns0QvYpu6GzNK8G
mICf5DhTr7nfhfh9aZLtMQKBgCxKsmqzG5n//MxhHB4sstVxwJtwDNeZPKzISnM8
PfBT/lQSbqj1Y73japRjXbTgC4Ore3A2JKjTGFN+dm1tJGDUT/H8x4BPWEBCyCfT
3i2noA6sewrJbQPsDvlYVubSEYNKmxlbBmmhw98StlBMv9I8kX6BSDI/uggwid0e
/WvjAoGBAKpZ0UOKQyrl9reBiUfrpRCvIMakBMd79kNiH+5y0Soq/wCAnAuABayj
XEIBhFv+HxeLEnT7YV+Zzqp5L9kKw/EU4ik3JX/XsEihdSxEuGX00ZYOw05FEfpW
cJ5Ku0OTwRtSMaseRPX+T4EfG1Caa/eunPPN4rh+CSup2BVVarOT
-----END RSA PRIVATE KEY-----";

    // ── Test helpers ─────────────────────────────────────────────────────────

    /// Parse all headers from a raw RFC 5322 email (handles folding).
    /// Returns `(name, raw_value)` pairs preserving fold characters.
    fn parse_headers(email_bytes: &[u8]) -> Vec<(String, String)> {
        let email = std::str::from_utf8(email_bytes).expect("email must be valid UTF-8");
        let header_block = email
            .split_once("\r\n\r\n")
            .map(|(h, _)| h)
            .unwrap_or(email);

        let mut headers: Vec<(String, String)> = Vec::new();
        let mut current: Option<(String, String)> = None;

        for line in header_block.split('\n') {
            // Trim trailing CR so we work with bare LF or CRLF
            let line = line.trim_end_matches('\r');
            if line.starts_with(' ') || line.starts_with('\t') {
                if let Some((_, ref mut val)) = current {
                    val.push_str("\r\n");
                    val.push_str(line);
                }
            } else if let Some(colon) = line.find(':') {
                if let Some(prev) = current.take() {
                    headers.push(prev);
                }
                let name = line[..colon].to_string();
                // Keep the raw value (may start with a space)
                let value = line[colon + 1..].to_string();
                current = Some((name, value));
            }
        }
        if let Some(last) = current {
            headers.push(last);
        }
        headers
    }

    /// Unfold a header value (replace CRLF+whitespace with a single space).
    fn unfold(value: &str) -> String {
        value.replace("\r\n ", " ").replace("\r\n\t", " ")
    }

    /// RFC 6376 §3.4.2 relaxed header canonicalization.
    /// `name` is the header field name (will be lowercased).
    /// `raw_value` is the raw value from the message (may include folding).
    fn relaxed_header(name: &str, raw_value: &str) -> String {
        let unfolded = unfold(raw_value);
        // Compress internal whitespace to a single space; strip leading/trailing.
        let value: String = unfolded.split_whitespace().collect::<Vec<_>>().join(" ");
        format!("{}:{}\r\n", name.to_lowercase(), value)
    }

    /// Extract a DKIM tag value from an unfolded DKIM-Signature value.
    /// Splits on `;`, trims, and finds `tag=value`.
    fn dkim_tag<'a>(sig_unfolded: &'a str, tag: &str) -> Option<&'a str> {
        for part in sig_unfolded.split(';') {
            let part = part.trim();
            let prefix = format!("{}=", tag);
            if part.starts_with(&prefix) && !part.starts_with(&format!("{}h=", tag)) {
                return Some(&part[prefix.len()..]);
            }
        }
        None
    }

    // ── Tests ─────────────────────────────────────────────────────────────────

    /// Verify that a plain-text RawEmail produces a correctly shaped lettre Message.
    #[test]
    fn build_message_plain_text() {
        let raw = RawEmail {
            from: "sender@example.com".to_string(),
            from_name: Some("Sender Name".to_string()),
            to: "recipient@example.org".to_string(),
            reply_to: None,
            subject: "Test subject".to_string(),
            text: Some("Hello, plain text!".to_string()),
            html: None,
            headers: vec![(
                "List-Unsubscribe".to_string(),
                "<https://example.com/unsub>".to_string(),
            )],
        };

        let mut msg = build_message(&raw).expect("plain-text message should build");

        // Add custom headers
        for (name, value) in raw.headers {
            let header_name = HeaderName::new_from_ascii(name).unwrap();
            msg.headers_mut()
                .insert_raw(HeaderValue::new(header_name, value));
        }

        let formatted = msg.formatted();
        let formatted_str = std::str::from_utf8(&formatted).unwrap();

        assert!(formatted_str.contains("From:"), "should have From header");
        assert!(formatted_str.contains("To:"), "should have To header");
        assert!(
            formatted_str.contains("Subject: Test subject"),
            "should have Subject header"
        );
        assert!(
            formatted_str.contains("List-Unsubscribe:"),
            "custom header should be present"
        );
        assert!(
            formatted_str.contains("Hello, plain text!"),
            "plain text body should be present"
        );
    }

    /// Verify that providing both text and html produces multipart/alternative.
    #[test]
    fn build_message_multipart() {
        let raw = RawEmail {
            from: "sender@example.com".to_string(),
            from_name: None,
            to: "recipient@example.org".to_string(),
            reply_to: Some("reply@example.com".to_string()),
            subject: "Multipart test".to_string(),
            text: Some("Plain text version".to_string()),
            html: Some("<p>HTML version</p>".to_string()),
            headers: vec![],
        };

        let msg = build_message(&raw).expect("multipart message should build");
        let formatted = msg.formatted();
        let formatted_str = std::str::from_utf8(&formatted).unwrap();

        assert!(
            formatted_str.contains("multipart/alternative"),
            "should be multipart/alternative"
        );
        assert!(
            formatted_str.contains("Plain text version"),
            "text part should be present"
        );
        assert!(
            formatted_str.contains("<p>HTML version</p>"),
            "html part should be present"
        );
        assert!(
            formatted_str.contains("Reply-To:"),
            "Reply-To header should be present"
        );
    }

    /// Verify that missing both text and html returns a Build error.
    #[test]
    fn build_message_no_body_returns_error() {
        let raw = RawEmail {
            from: "sender@example.com".to_string(),
            from_name: None,
            to: "recipient@example.org".to_string(),
            reply_to: None,
            subject: "No body".to_string(),
            text: None,
            html: None,
            headers: vec![],
        };
        assert!(
            matches!(build_message(&raw), Err(EmailError::Build(_))),
            "should fail with Build error when both text and html are None"
        );
    }

    /// Full DKIM round-trip: sign a message then verify the RSA-SHA256 signature
    /// using the matching public key (no DNS, no network).
    ///
    /// Algorithm: relaxed/relaxed canonicalization (RFC 6376 §3.4).
    /// We re-derive what was signed by inspecting the formatted email and applying
    /// the same canonicalization lettre used, then verify with `rsa::RsaPublicKey`.
    #[test]
    fn dkim_sign_and_verify_rsa_round_trip() {
        use base64::Engine as _;
        use rsa::{
            pkcs1::DecodeRsaPrivateKey, pkcs1v15::Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey,
        };
        use sha2::{Digest, Sha256};

        // ── 1. Build signer ───────────────────────────────────────────────────
        let signer = DkimSigner::new("example.com", "testsel", TEST_RSA_KEY)
            .expect("DkimSigner should parse the test RSA key");

        // ── 2. Build and sign a simple message ────────────────────────────────
        let raw = RawEmail {
            from: "sender@example.com".to_string(),
            from_name: None,
            to: "recipient@example.org".to_string(),
            reply_to: None,
            subject: "DKIM test".to_string(),
            text: Some("Hello DKIM world".to_string()),
            html: None,
            headers: vec![],
        };
        let mut msg = build_message(&raw).expect("message should build");
        msg.sign(&signer.config);

        // ── 3. Get formatted email bytes ──────────────────────────────────────
        let formatted = msg.formatted();
        let headers = parse_headers(&formatted);

        // ── 4. Locate and check DKIM-Signature header ─────────────────────────
        let dkim_header = headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("DKIM-Signature"))
            .expect("DKIM-Signature header must be present after signing");

        let raw_dkim_value = &dkim_header.1;
        let unfolded_value = unfold(raw_dkim_value);

        // Structural checks
        let a_tag = dkim_tag(&unfolded_value, "a").expect("a= tag");
        let c_tag = dkim_tag(&unfolded_value, "c").expect("c= tag");
        let d_tag = dkim_tag(&unfolded_value, "d").expect("d= tag");
        let s_tag = dkim_tag(&unfolded_value, "s").expect("s= tag");
        let h_tag = dkim_tag(&unfolded_value, "h").expect("h= tag");
        let b_tag = dkim_tag(&unfolded_value, "b").expect("b= tag");
        let bh_tag = dkim_tag(&unfolded_value, "bh").expect("bh= tag");

        assert_eq!(a_tag.trim(), "rsa-sha256", "algorithm must be rsa-sha256");
        assert_eq!(
            c_tag.trim(),
            "relaxed/relaxed",
            "canonicalization must be relaxed/relaxed"
        );
        assert_eq!(d_tag.trim(), "example.com", "domain must match");
        assert_eq!(s_tag.trim(), "testsel", "selector must match");

        // ── 5. Verify body hash (bh=) ─────────────────────────────────────────
        // Body starts after the first CRLF CRLF in the formatted email.
        let separator = b"\r\n\r\n";
        let body_start = formatted
            .windows(separator.len())
            .position(|w| w == separator)
            .expect("email must contain header/body separator")
            + separator.len();
        let body_bytes = &formatted[body_start..];

        // Relaxed body canonicalization (RFC 6376 §3.4.4):
        // - Strip trailing whitespace from each line
        // - Remove trailing blank lines
        // - Must end with CRLF
        let body_str = std::str::from_utf8(body_bytes).unwrap();
        let mut canon_body_lines: Vec<&str> =
            body_str.split("\r\n").map(|l| l.trim_end()).collect();
        // Remove trailing empty lines (RFC: body ends with one CRLF)
        while canon_body_lines.last().map_or(false, |l| l.is_empty()) {
            canon_body_lines.pop();
        }
        let mut canon_body = canon_body_lines.join("\r\n");
        canon_body.push_str("\r\n");

        let bh_computed =
            base64::engine::general_purpose::STANDARD.encode(Sha256::digest(canon_body.as_bytes()));
        assert_eq!(
            bh_computed.trim(),
            bh_tag.trim(),
            "body hash (bh=) must match re-computed SHA256 of relaxed-canonicalized body"
        );

        // ── 6. Reconstruct signing input and verify RSA signature (b=) ────────
        //
        // Signing input = relaxed-canonicalized signed headers (in h= order)
        //              ++ relaxed-canonicalized DKIM-Signature with b= emptied
        //                 (trimmed of trailing whitespace, per RFC 6376 §6.1.7)

        // 6a. Canonicalize the signed headers
        let mut signing_input = String::new();
        for header_name in h_tag.trim().split(':') {
            let header_name = header_name.trim();
            if let Some((_, val)) = headers
                .iter()
                .find(|(n, _)| n.eq_ignore_ascii_case(header_name))
            {
                signing_input.push_str(&relaxed_header(header_name, val));
            }
        }

        // 6b. Build DKIM-Signature value with b= stripped (value replaced by empty)
        // Strategy: split on semicolons, find the `b` tag (not `bh`), empty its value.
        let dkim_sig_stripped: String = unfolded_value
            .split(';')
            .map(|part| {
                let t = part.trim();
                if t.starts_with("b=") && !t.starts_with("bh=") {
                    // Preserve whitespace prefix of the segment, strip value
                    let prefix_spaces = part.len() - part.trim_start().len();
                    format!("{} b=", " ".repeat(prefix_spaces.saturating_sub(1)))
                } else {
                    part.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(";");

        // Apply relaxed canonicalization to the modified DKIM-Signature header,
        // then trim trailing whitespace (matching lettre's `.trim_end()` call).
        let dkim_canon = relaxed_header("DKIM-Signature", &dkim_sig_stripped);
        signing_input.push_str(dkim_canon.trim_end());

        // 6c. SHA256 hash of the signing input
        let digest = Sha256::digest(signing_input.as_bytes());

        // 6d. Decode the base64 RSA signature from b=
        let b_clean: String = b_tag
            .trim()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        let sig_bytes = base64::engine::general_purpose::STANDARD
            .decode(&b_clean)
            .expect("b= value must be valid base64");

        // 6e. Verify with RSA-PKCS1v15-SHA256 (same scheme as lettre uses)
        let private_key = RsaPrivateKey::from_pkcs1_pem(TEST_RSA_KEY).expect("test key must parse");
        let public_key = RsaPublicKey::from(&private_key);
        public_key
            .verify(Pkcs1v15Sign::new::<Sha256>(), &digest, &sig_bytes)
            .expect("DKIM RSA-SHA256 signature must verify with the matching public key");
    }

    /// EmailError::is_transient returns true only for Transient variant.
    #[test]
    fn email_error_is_transient_classification() {
        assert!(EmailError::Transient("4xx greylisted".into()).is_transient());
        assert!(!EmailError::Permanent("5xx bad rcpt".into()).is_transient());
        assert!(!EmailError::Config("tls error".into()).is_transient());
        assert!(!EmailError::Build("bad address".into()).is_transient());
        assert!(!EmailError::DkimKey("bad pem".into()).is_transient());
    }

    /// DkimSigner::new rejects an invalid PEM string.
    #[test]
    fn dkim_signer_rejects_invalid_key() {
        let result = DkimSigner::new("example.com", "sel", "not-a-pem-key");
        assert!(
            matches!(result, Err(EmailError::DkimKey(_))),
            "should fail with DkimKey error for invalid PEM"
        );
    }

    /// SmtpConfig::from_env returns None when SOMA_SMTP_HOST is absent.
    ///
    /// Safe to run in parallel: only removes a var that should not be set in CI.
    #[test]
    fn smtp_config_from_env_none_without_host() {
        // Temporarily unset to guarantee a clean state.
        let _prev = std::env::var("SOMA_SMTP_HOST").ok();
        std::env::remove_var("SOMA_SMTP_HOST");
        assert!(
            SmtpConfig::from_env().is_none(),
            "from_env() must return None when SOMA_SMTP_HOST is unset"
        );
    }

    /// SmtpConfig::from_env reads host + port + tls mode correctly.
    ///
    /// NOTE: modifies process-level env vars; run sequentially or accept rare
    /// flakiness if the CI environment already sets SOMA_SMTP_*.
    #[test]
    fn smtp_config_from_env_parses_basic_fields() {
        // Stash originals so we can restore after the test.
        let prev_host = std::env::var("SOMA_SMTP_HOST").ok();
        let prev_port = std::env::var("SOMA_SMTP_PORT").ok();
        let prev_tls = std::env::var("SOMA_SMTP_TLS").ok();

        std::env::set_var("SOMA_SMTP_HOST", "smtp.test.example");
        std::env::set_var("SOMA_SMTP_PORT", "465");
        std::env::set_var("SOMA_SMTP_TLS", "tls");
        std::env::remove_var("SOMA_SMTP_USERNAME");
        std::env::remove_var("SOMA_SMTP_PASSWORD");
        std::env::remove_var("SOMA_SMTP_PASSWORD_FILE");
        std::env::remove_var("SOMA_SMTP_TIMEOUT_SECS");

        let cfg = SmtpConfig::from_env().expect("should return Some when SOMA_SMTP_HOST is set");
        assert_eq!(cfg.host, "smtp.test.example");
        assert_eq!(cfg.port, 465);
        assert_eq!(
            cfg.tls,
            TlsMode::Tls,
            "tls string should map to TlsMode::Tls"
        );
        assert!(cfg.username.is_none());
        assert!(cfg.password.is_none());
        assert!(cfg.timeout.is_none());

        // Restore
        match prev_host {
            Some(v) => std::env::set_var("SOMA_SMTP_HOST", v),
            None => std::env::remove_var("SOMA_SMTP_HOST"),
        }
        match prev_port {
            Some(v) => std::env::set_var("SOMA_SMTP_PORT", v),
            None => std::env::remove_var("SOMA_SMTP_PORT"),
        }
        match prev_tls {
            Some(v) => std::env::set_var("SOMA_SMTP_TLS", v),
            None => std::env::remove_var("SOMA_SMTP_TLS"),
        }
    }
}
