//! Graceful-shutdown signal handling.
//!
//! Pure plumbing: `shutdown_signal().await` returns when the process receives
//! Ctrl-C (SIGINT) or, on Unix, SIGTERM. The caller decides what to do on
//! shutdown — typically hand this future to `axum::serve(...).with_graceful_shutdown(...)`.
//!
//! Does not log — the caller logs on shutdown if desired.

/// Completes on Ctrl-C (SIGINT) or, on Unix, SIGTERM.
///
/// On non-Unix platforms only Ctrl-C is awaited (SIGTERM has no equivalent).
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    /// Verify `shutdown_signal` is pending (stays alive) when no signal arrives.
    /// A tiny timeout races the future; the timeout must win.
    #[tokio::test]
    async fn shutdown_signal_is_pending_without_a_signal() {
        let result =
            tokio::time::timeout(Duration::from_millis(50), super::shutdown_signal()).await;
        // If no signal was sent the timeout fires, giving Err(Elapsed).
        assert!(
            result.is_err(),
            "shutdown_signal() completed unexpectedly — a real signal must have arrived"
        );
    }
}
