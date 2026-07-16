//! Configured `reqwest` HTTP client constructor.
//!
//! Pure plumbing: a `reqwest::Client` with rustls TLS and sane default
//! timeouts. The caller owns all request building, auth headers, retries, and
//! base URLs — this only standardises TLS + timeouts so services stop
//! hand-rolling slightly-different client builders.

use std::time::Duration;

/// Build a `reqwest::Client` with rustls TLS and default timeouts
/// (request 30s, connect 10s).
///
/// For a fully custom client, use `reqwest::Client::builder()` directly — this
/// is the common-case convenience.
// ponytail: 30s request / 10s connect are reasonable service-to-service
// defaults; a caller needing different values builds its own client.
pub fn client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .build()
}

/// Build a client with caller-specified timeouts (seconds).
pub fn client_with_timeouts(
    timeout_secs: u64,
    connect_timeout_secs: u64,
) -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(Duration::from_secs(timeout_secs))
        .connect_timeout(Duration::from_secs(connect_timeout_secs))
        .build()
}

/// Build a client that **never** follows redirects, with rustls TLS and
/// default timeouts (request 30s, connect 10s).
///
/// Use this wherever following a redirect would bypass a host allowlist — for
/// example, fetching SNS signing certs and SNS SubscribeURLs: a valid
/// `sns.<region>.amazonaws.com` host could 302 to `169.254.169.254` and the
/// default client would follow, leaking the SSRF guard.
pub fn client_no_redirect() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::none())
        .build()
}

/// Build a client that **never** follows redirects, with rustls TLS and
/// caller-specified timeouts (seconds).
///
/// Use this wherever following a redirect would bypass a host allowlist — for
/// example, SSRF-guarded HTTP dispatch where the validated URL must be the
/// same URL that is actually fetched: a 302 to `169.254.169.254` would
/// otherwise bypass the guard silently.
pub fn client_no_redirect_with_timeouts(
    timeout_secs: u64,
    connect_timeout_secs: u64,
) -> reqwest::Result<reqwest::Client> {
    reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(Duration::from_secs(timeout_secs))
        .connect_timeout(Duration::from_secs(connect_timeout_secs))
        .redirect(reqwest::redirect::Policy::none())
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_builds_ok() {
        assert!(client().is_ok());
    }

    #[test]
    fn client_with_timeouts_builds_ok() {
        assert!(client_with_timeouts(5, 2).is_ok());
    }

    #[test]
    fn client_no_redirect_builds_ok() {
        assert!(client_no_redirect().is_ok());
    }

    #[test]
    fn client_no_redirect_with_timeouts_builds_ok() {
        assert!(client_no_redirect_with_timeouts(5, 2).is_ok());
    }
}
