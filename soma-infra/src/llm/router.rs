//! Thin router that delegates to a boxed `ProviderAdapter`.
//!
//! `LlmRouter` holds one adapter and exposes the same three operations.
//! No routing logic, no fallback, no provider selection — compose those
//! in the service layer.

use std::pin::Pin;

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
