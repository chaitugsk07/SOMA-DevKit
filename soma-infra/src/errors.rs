//! Light error helpers and crash reporter for soma services.
//!
//! # DB-error redaction
//! [`redact_db_error`] renders a `sqlx::Error` safely — no SQL text, no PII.
//!
//! # Crash reporter
//! [`ErrorReporterConfig::from_env`] + [`install_panic_hook`] wire Rust panics
//! to soma-observe's `/v1/errors` endpoint.  The hook is a no-op when neither
//! `SOMA_OBSERVE_URL` nor `OTEL_EXPORTER_OTLP_ENDPOINT` is set, so it is safe
//! to call in every environment without any required configuration.

use serde::Serialize;

// ── DB-error redaction ────────────────────────────────────────────────────────

/// Render a `sqlx::Error` as a message safe to return to a client: it never
/// echoes SQL text, bound parameters, or row values (which can carry PII).
/// Only the variant and, for database errors, the SQLSTATE code are surfaced.
pub fn redact_db_error(err: &sqlx::Error) -> String {
    match err {
        sqlx::Error::RowNotFound => "row not found".to_string(),
        sqlx::Error::Database(db) => match db.code() {
            Some(code) => format!("database error (SQLSTATE {code})"),
            None => "database error".to_string(),
        },
        sqlx::Error::PoolTimedOut => "database pool timed out".to_string(),
        sqlx::Error::PoolClosed => "database pool closed".to_string(),
        _ => "database error".to_string(),
    }
}

// ── Crash reporter ────────────────────────────────────────────────────────────

/// Configuration for the soma-observe crash reporter.
///
/// Build with [`ErrorReporterConfig::from_env`] and pass to
/// [`install_panic_hook`].
pub struct ErrorReporterConfig {
    /// Base URL of the soma-observe instance (no trailing slash, no `/v1`).
    pub observe_url: String,
    /// Write-only ingest key (`ERROR_INGEST_KEY`).  `None` → header omitted.
    pub ingest_key: Option<String>,
    /// Short service name sent as `server_name` (e.g. `"soma-iam"`).
    pub service_name: String,
    /// Deployment environment (e.g. `"production"`, `"staging"`).
    pub environment: String,
    /// Release string — pass `env!("CARGO_PKG_VERSION")` from the binary crate.
    pub release: String,
}

impl ErrorReporterConfig {
    /// Build from environment variables.
    ///
    /// Returns `None` when neither `SOMA_OBSERVE_URL` nor
    /// `OTEL_EXPORTER_OTLP_ENDPOINT` is set — [`install_panic_hook`] then
    /// becomes a no-op and the default panic hook is unchanged.
    ///
    /// # Env vars read
    ///
    /// | Var | Priority | Notes |
    /// |-----|----------|-------|
    /// | `OTEL_EXPORTER_OTLP_ENDPOINT` | 1st | trailing `/v1` stripped |
    /// | `SOMA_OBSERVE_URL` | 2nd | direct base URL |
    /// | `ERROR_INGEST_KEY` | — | write-only ingest key (optional) |
    /// | `SOMA_ENV` | — | deployment env; falls back to `ENVIRONMENT` then `"production"` |
    ///
    /// `release` is caller-supplied — pass `env!("CARGO_PKG_VERSION")` in the
    /// binary crate so the version reflects the *service*, not soma-infra.
    pub fn from_env(service_name: &str, release: &str) -> Option<Self> {
        let observe_url = resolve_observe_url()?;
        let ingest_key = std::env::var("ERROR_INGEST_KEY").ok();
        let environment = std::env::var("SOMA_ENV")
            .or_else(|_| std::env::var("ENVIRONMENT"))
            .unwrap_or_else(|_| "production".to_string());
        Some(ErrorReporterConfig {
            observe_url,
            ingest_key,
            service_name: service_name.to_string(),
            environment,
            release: release.to_string(),
        })
    }
}

/// Install a panic hook that forwards panics to soma-observe as `fatal` events.
///
/// When `cfg` is `None` this is a no-op — the default panic hook is unchanged.
/// When `cfg` is `Some`, the *existing* hook is captured and always called
/// first (stderr output, tracing-panic integration, etc. continue to work),
/// then the event is sent best-effort to soma-observe.
///
/// # Ceiling
///
/// ponytail: delivery uses a detached `std::thread` with a blocking reqwest
/// client (2-second timeout).  With `panic = "abort"` profiles or an
/// immediate `std::process::exit`, the OS may reclaim the process before the
/// thread completes — the event will not be delivered.  `panic = "unwind"`
/// (the Cargo default) gives the thread sufficient time in practice.
/// Upgrade path: pre-spawn a reporter thread with a channel for
/// guaranteed-delivery semantics.
pub fn install_panic_hook(cfg: Option<ErrorReporterConfig>) {
    let Some(cfg) = cfg else { return };

    // Capture the current hook so we can call it first on every panic.
    let prev = std::panic::take_hook();

    // Arc so the closure satisfies `'static` without cloning on every panic.
    let cfg = std::sync::Arc::new(cfg);

    std::panic::set_hook(Box::new(move |info| {
        // Always call the previous hook first (stderr, tracing-panic, etc.).
        prev(info);

        let msg: String = if let Some(s) = info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "panic".to_string()
        };

        let location = info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "unknown".to_string());

        // Capture immediately — still in the panicking thread's context.
        let bt = std::backtrace::Backtrace::force_capture();
        let frames = parse_backtrace(&bt.to_string());

        let url = format!("{}/v1/errors", cfg.observe_url);
        let ingest_key = cfg.ingest_key.clone();
        let environment = cfg.environment.clone();
        let release = cfg.release.clone();
        let server_name = cfg.service_name.clone();

        let body = serde_json::json!({
            "level": "fatal",
            "platform": "rust",
            "environment": environment,
            "release": release,
            "server_name": server_name,
            "exception": {
                "type": "panic",
                "value": msg,
                "stacktrace": { "frames": frames }
            },
            "tags": { "location": location }
        });

        let Ok(body_str) = serde_json::to_string(&body) else {
            return;
        };

        // Spawn a detached OS thread — not a Tokio task — so the blocking
        // HTTP client creates its own runtime safely outside the async context.
        let _ = std::thread::Builder::new()
            .name("soma-panic-reporter".into())
            .spawn(move || {
                let Ok(client) = reqwest::blocking::Client::builder()
                    .timeout(std::time::Duration::from_secs(2))
                    .build()
                else {
                    return;
                };
                let mut req = client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .body(body_str);
                if let Some(key) = &ingest_key {
                    req = req.header("X-Soma-Error-Key", key);
                }
                let _ = req.send(); // best-effort; ignore network errors
            });
    }));
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Same env-var precedence as `telemetry::resolve_endpoint`:
/// `OTEL_EXPORTER_OTLP_ENDPOINT` > `SOMA_OBSERVE_URL`.
///
/// OTLP exporters append `/v1/{signals}` to the endpoint themselves, so the
/// OTEL env var typically carries a `/v1` suffix.  We strip it so that we can
/// append our own `/v1/errors` without doubling the path segment.
fn resolve_observe_url() -> Option<String> {
    let raw = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .or_else(|_| std::env::var("SOMA_OBSERVE_URL"))
        .ok()?;
    let url = raw.trim_end_matches('/');
    let url = url.strip_suffix("/v1").unwrap_or(url);
    if url.is_empty() {
        return None;
    }
    Some(url.to_string())
}

/// Per-frame entry in the `stacktrace.frames` payload.
#[derive(Serialize)]
struct BacktraceFrame {
    function: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lineno: Option<u32>,
    in_app: bool,
}

/// Parse the `Display` output of [`std::backtrace::Backtrace`] into frames.
///
/// The stdlib produces lines like:
/// ```text
///    0: std::panicking::begin_panic_handler
///              at /rustc/…/panicking.rs:645:5
///    1: myapp::handler
///              at src/main.rs:42:5
/// ```
///
/// The parser is intentionally lenient — it never panics and skips any line
/// that does not match the expected pattern.
fn parse_backtrace(bt: &str) -> Vec<BacktraceFrame> {
    let mut frames = Vec::new();
    let mut lines = bt.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        // Match "N: function_name"
        let Some(colon) = trimmed.find(':') else {
            continue;
        };
        let index_str = trimmed[..colon].trim();
        if index_str.parse::<u32>().is_err() {
            continue;
        }
        let function = trimmed[colon + 1..].trim();
        if function.is_empty() {
            continue;
        }
        let function = function.to_string();

        // The following line may be "at file:line:col"
        let (filename, lineno) = match lines.peek() {
            Some(next) => {
                let nt = next.trim();
                if let Some(loc) = nt.strip_prefix("at ") {
                    let result = parse_location(loc);
                    lines.next(); // consume the "at" line
                    result
                } else {
                    (None, None)
                }
            }
            None => (None, None),
        };

        let in_app = filename.as_deref().map(is_in_app).unwrap_or(false);
        frames.push(BacktraceFrame {
            function,
            filename,
            lineno,
            in_app,
        });
    }
    frames
}

/// Parse `"file:line:col"` or `"file:line"` — splitting from the right so
/// Windows drive-letter colons inside the path are not misinterpreted.
fn parse_location(loc: &str) -> (Option<String>, Option<u32>) {
    // rsplitn(3, ':') yields at most 3 parts from the right: [col, line, file].
    let mut parts: Vec<&str> = loc.rsplitn(3, ':').collect();
    parts.reverse(); // → [file, line, col] or [file, line] or [file]
    match parts.as_slice() {
        [file, line, _col] => (Some((*file).to_string()), line.parse().ok()),
        [file, line] => (Some((*file).to_string()), line.parse().ok()),
        _ => (None, None),
    }
}

/// `true` when the frame looks like application code rather than stdlib or
/// cargo-registry paths.
fn is_in_app(filename: &str) -> bool {
    !filename.starts_with("/rustc")
        && !filename.contains("/.cargo/")
        && !filename.contains("\\.cargo\\")
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── redact_db_error ───────────────────────────────────────────────────────

    #[test]
    fn redacts_row_not_found() {
        assert_eq!(redact_db_error(&sqlx::Error::RowNotFound), "row not found");
    }

    #[test]
    fn redacts_pool_timeout_without_detail() {
        let msg = redact_db_error(&sqlx::Error::PoolTimedOut);
        assert!(msg.contains("pool timed out"));
    }

    // ── parse_backtrace ───────────────────────────────────────────────────────

    #[test]
    fn parse_backtrace_extracts_function_and_location() {
        let bt = "   0: std::panicking::begin_panic\n\
                       at /rustc/abc/panicking.rs:10:5\n\
                  1: myapp::handler\n\
                       at src/main.rs:42:5\n";
        let frames = parse_backtrace(bt);
        assert_eq!(frames.len(), 2);

        assert_eq!(frames[0].function, "std::panicking::begin_panic");
        assert_eq!(
            frames[0].filename.as_deref(),
            Some("/rustc/abc/panicking.rs")
        );
        assert_eq!(frames[0].lineno, Some(10));
        assert!(!frames[0].in_app, "stdlib frame must not be in_app");

        assert_eq!(frames[1].function, "myapp::handler");
        assert_eq!(frames[1].filename.as_deref(), Some("src/main.rs"));
        assert_eq!(frames[1].lineno, Some(42));
        assert!(frames[1].in_app, "app frame must be in_app");
    }

    #[test]
    fn parse_backtrace_tolerates_missing_location() {
        let bt = "   0: some::func\n\
                  1: other::func\n\
                       at src/lib.rs:5:1\n";
        let frames = parse_backtrace(bt);
        assert_eq!(frames.len(), 2);
        assert!(frames[0].filename.is_none());
        assert!(frames[0].lineno.is_none());
        assert_eq!(frames[1].function, "other::func");
        assert_eq!(frames[1].lineno, Some(5));
    }

    #[test]
    fn parse_backtrace_empty_input_gives_no_frames() {
        assert!(parse_backtrace("").is_empty());
    }

    // ── envelope JSON ─────────────────────────────────────────────────────────

    #[test]
    fn envelope_json_contains_required_fields() {
        let frames: Vec<BacktraceFrame> = vec![];
        let body = serde_json::json!({
            "level": "fatal",
            "platform": "rust",
            "environment": "test",
            "release": "1.0.0",
            "server_name": "soma-test",
            "exception": {
                "type": "panic",
                "value": "explicit panic",
                "stacktrace": { "frames": frames }
            },
            "tags": { "location": "src/main.rs:10" }
        });
        let s = serde_json::to_string(&body).unwrap();
        assert!(s.contains("\"level\":\"fatal\""));
        assert!(s.contains("\"platform\":\"rust\""));
        assert!(s.contains("\"type\":\"panic\""));
        assert!(s.contains("soma-test"));
        assert!(s.contains("src/main.rs:10"));
    }

    // ── resolve_observe_url ───────────────────────────────────────────────────

    #[test]
    fn strips_v1_suffix_from_otel_endpoint() {
        // resolve_observe_url reads env vars — test the strip logic directly.
        let cases: &[(&str, &str)] = &[
            ("http://observe:4317/v1", "http://observe:4317"),
            ("http://observe:4317/v1/", "http://observe:4317"),
            ("http://observe:4317", "http://observe:4317"),
            ("http://observe:4317/", "http://observe:4317"),
        ];
        for (input, expected) in cases {
            let url = input.trim_end_matches('/');
            let url = url.strip_suffix("/v1").unwrap_or(url);
            assert_eq!(url, *expected, "input: {input}");
        }
    }
}
