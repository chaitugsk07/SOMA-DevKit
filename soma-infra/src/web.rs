//! Generic axum web plumbing for soma services.
//!
//! Pure plumbing — no service policy, no specific tokens, no specific routes.
//! Three helpers that vault and audit both hand-roll identically:
//!
//! - [`serve_spa`] — serve compile-time-embedded SPA assets with SPA fallback.
//! - [`extract_bearer`] — pull a bearer token from an `Authorization` header value.
//! - [`serve_with_shutdown`] — bind + serve + graceful shutdown via
//!   [`crate::signal::shutdown_signal`].
//!
//! Does not log — the caller logs bind address, shutdown events, etc.

use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::Response,
    Router,
};
use rust_embed::RustEmbed;

// ── SPA handler ───────────────────────────────────────────────────────────────

/// Serve a single-page app from compile-time-embedded assets.
///
/// Generic over a [`RustEmbed`] type `A` — the service declares its own:
/// ```ignore
/// #[derive(RustEmbed)]
/// #[folder = "../../dashboard/dist"]
/// struct Assets;
/// ```
/// and wires it as a fallback:
/// ```ignore
/// router.fallback(|uri: Uri| async move { soma_infra::web::serve_spa::<Assets>(&uri) })
/// ```
///
/// Behaviour:
/// 1. Serve the exact requested asset if present (MIME from `mime_guess`).
/// 2. Fall back to `index.html` for SPA client-side routing.
/// 3. If `index.html` is also absent (empty dist), return a generic stub so
///    the server is still usable without a bundled frontend.
// ponytail: mime_guess covers every common web type; hand-rolling a MIME table
// (as audit/portal.rs does today) is dead weight — mime_guess is already a dep.
pub fn serve_spa<A: RustEmbed>(uri: &Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    if let Some(f) = A::get(path) {
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();
        return build_response(StatusCode::OK, &mime, f.data.into_owned());
    }
    if let Some(index) = A::get("index.html") {
        return build_response(
            StatusCode::OK,
            "text/html; charset=utf-8",
            index.data.into_owned(),
        );
    }
    // ponytail: empty dist — generic stub so the server still boots. The service
    // ships its own index.html (via RustEmbed) to override this entirely.
    build_response(
        StatusCode::OK,
        "text/html; charset=utf-8",
        b"<!doctype html><html><head><meta charset=\"utf-8\"><title>soma</title></head>\
          <body><h1>Dashboard not bundled</h1><p>Build the frontend and rebuild the \
          server to embed it.</p></body></html>"
            .to_vec(),
    )
}

fn build_response(status: StatusCode, mime: &str, body: Vec<u8>) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, mime)
        .body(Body::from(body))
        .expect("static response is always valid")
}

// ── Bearer extraction ─────────────────────────────────────────────────────────

/// Extract the token from an `Authorization: Bearer <token>` header value.
///
/// Returns `None` if the value is absent or does not start with `"Bearer "`.
///
/// Usage:
/// ```ignore
/// let token = soma_infra::web::extract_bearer(
///     req.headers().get(AUTHORIZATION).and_then(|v| v.to_str().ok()),
/// );
/// ```
// ponytail: pure &str — no axum dependency needed; the caller does the header
// lookup, keeping this usable regardless of axum version.
pub fn extract_bearer(authorization_header: Option<&str>) -> Option<&str> {
    authorization_header?.strip_prefix("Bearer ")
}

// ── Graceful-serve loop ───────────────────────────────────────────────────────

/// Bind `addr`, serve `app`, and shut down gracefully on Ctrl-C / SIGTERM.
///
/// Calls `into_make_service()` on the router internally — if you need
/// `ConnectInfo` (e.g., to read the client IP), call `axum::serve` directly
/// with `app.into_make_service_with_connect_info::<SocketAddr>()`.
///
/// Does not log — the caller logs the bind address and the shutdown event:
/// ```ignore
/// tracing::info!("listening on {addr}");
/// soma_infra::web::serve_with_shutdown(&addr, app).await?;
/// tracing::info!("shutdown complete");
/// ```
pub async fn serve_with_shutdown(addr: &str, app: Router) -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(crate::signal::shutdown_signal())
        .await
}

// ── HTTP metrics middleware ────────────────────────────────────────────────────

/// Axum middleware that records `http.server.request.duration` (seconds) for every
/// request. Requires both the `web` and `otlp` features.
///
/// Wire it up with:
/// ```ignore
/// router.layer(axum::middleware::from_fn(soma_infra::web::track_http_metrics))
/// ```
///
/// Attributes per request:
/// - `http.request.method` — HTTP method (string)
/// - `http.route` — matched route template, or `"fallback"` if absent
/// - `http.response.status_code` — response status code (i64)
#[cfg(all(feature = "web", feature = "otlp"))]
pub async fn track_http_metrics(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::extract::MatchedPath;
    use opentelemetry::KeyValue;
    use std::sync::OnceLock;
    use std::time::Instant;

    static HISTOGRAM: OnceLock<opentelemetry::metrics::Histogram<f64>> = OnceLock::new();
    let histogram = HISTOGRAM.get_or_init(|| {
        opentelemetry::global::meter("soma-infra")
            .f64_histogram("http.server.request.duration")
            .with_unit("s")
            .with_boundaries(vec![
                0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0,
            ])
            .build()
    });

    let method = req.method().as_str().to_owned();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|m| m.as_str().to_owned())
        .unwrap_or_else(|| "fallback".to_owned());

    let start = Instant::now();
    let response = next.run(req).await;
    let elapsed = start.elapsed().as_secs_f64();

    let status = response.status().as_u16() as i64;
    histogram.record(
        elapsed,
        &[
            KeyValue::new("http.request.method", method),
            KeyValue::new("http.route", route),
            KeyValue::new("http.response.status_code", status),
        ],
    );

    response
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::extract_bearer;

    #[test]
    fn bearer_present() {
        assert_eq!(extract_bearer(Some("Bearer abc")), Some("abc"));
    }

    #[test]
    fn bearer_wrong_scheme() {
        assert_eq!(extract_bearer(Some("Basic xyz")), None);
    }

    #[test]
    fn bearer_absent() {
        assert_eq!(extract_bearer(None), None);
    }

    #[test]
    fn bearer_empty_token() {
        // "Bearer " with no token — strip_prefix succeeds, returns Some("")
        assert_eq!(extract_bearer(Some("Bearer ")), Some(""));
    }
}
