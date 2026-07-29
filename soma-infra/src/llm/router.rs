//! Thin router that delegates to a boxed `ProviderAdapter`.
//!
//! `LlmRouter` holds one adapter and exposes the same three operations.
//! No routing logic, no fallback, no provider selection — compose those
//! in the service layer.

use std::pin::Pin;
use std::time::Duration;

use super::adapter::{
    CompletionRequest, CompletionResponse, EmbedRequest, EmbedResponse, ProviderAdapter,
    ProviderError, StreamEvent,
};

/// Thin holder that exposes the three `ProviderAdapter` operations via a
/// concrete struct (ergonomic for service state structs that don't want a
/// raw `Box<dyn ProviderAdapter>`).
pub struct LlmRouter {
    adapter: Box<dyn ProviderAdapter>,
}

impl LlmRouter {
    /// Wrap any adapter.
    pub fn new(adapter: Box<dyn ProviderAdapter>) -> Self {
        Self { adapter }
    }

    /// Non-streaming completion.
    pub async fn complete(
        &self,
        req: CompletionRequest,
    ) -> Result<CompletionResponse, ProviderError> {
        self.adapter.complete(req).await
    }

    /// Streaming completion.
    pub async fn complete_stream(
        &self,
        req: CompletionRequest,
    ) -> Result<
        Pin<
            Box<
                dyn futures_core::Stream<Item = Result<StreamEvent, ProviderError>>
                    + Send
                    + 'static,
            >,
        >,
        ProviderError,
    > {
        self.adapter.complete_stream(req).await
    }

    /// Embeddings.
    pub async fn embed(&self, req: EmbedRequest) -> Result<EmbedResponse, ProviderError> {
        self.adapter.embed(req).await
    }

    /// Non-streaming completion with bounded exponential backoff on transient failures.
    ///
    /// Retries on `RateLimited` (honouring `retry_after_secs`), `ProviderUnavailable`,
    /// and `Http` transport errors.  All other errors are returned immediately.
    ///
    // ponytail: 4 attempts, 20 s cap — tune per workload if the service's retry
    // tolerance differs (e.g. tighter latency budgets may prefer 2 attempts, 5 s cap).
    pub async fn complete_with_retry(
        &self,
        req: CompletionRequest,
    ) -> Result<CompletionResponse, ProviderError> {
        const MAX_ATTEMPTS: usize = 4;
        const BACKOFF_CAP_SECS: u64 = 20;

        let mut last_err: Option<ProviderError> = None;
        for attempt in 0..MAX_ATTEMPTS {
            if attempt > 0 {
                let retry_after =
                    if let Some(ProviderError::RateLimited { retry_after_secs }) = &last_err {
                        *retry_after_secs
                    } else {
                        None
                    };
                let secs = retry_after
                    .unwrap_or_else(|| 1u64 << (attempt - 1).min(6))
                    .min(BACKOFF_CAP_SECS);
                tracing::warn!(attempt, delay_secs = secs, "LLM transient error; retrying");
                tokio::time::sleep(Duration::from_secs(secs)).await;
            }
            match self.complete(req.clone()).await {
                Ok(resp) => return Ok(resp),
                Err(e) if is_retryable(&e) => last_err = Some(e),
                Err(e) => return Err(e),
            }
        }
        Err(last_err.expect("loop ran at least once"))
    }
}

/// Returns true when a `ProviderError` is transient and a retry may succeed.
///
/// Permanent errors (Auth, BadRequest, ContextOverflow, Malformed, NotSupported) are
/// returned without retry — they will not resolve on their own.
fn is_retryable(err: &ProviderError) -> bool {
    matches!(
        err,
        ProviderError::RateLimited { .. }
            | ProviderError::ProviderUnavailable
            | ProviderError::Http(_)
    )
}

impl std::fmt::Debug for LlmRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LlmRouter").finish_non_exhaustive()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-only: verify that LlmRouter is constructable with a boxed adapter.
    fn _assert_send_sync()
    where
        LlmRouter: Send + Sync,
    {
    }
}
