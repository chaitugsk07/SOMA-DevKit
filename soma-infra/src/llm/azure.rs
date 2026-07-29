//! Azure OpenAI adapter — thin factory over [`OpenAICompatibleAdapter`].
//!
//! The only Azure delta vs the generic OpenAI-compatible adapter:
//! - Auth: `api-key: {key}` header instead of `Authorization: Bearer {key}`.
//! - URL: deployment-scoped path `/openai/deployments/{deployment}/{operation}`.
//!
//! Both deltas are handled by fields on [`OpenAICompatibleAdapter`]; this
//! module contributes the URL builder and a convenience factory function.

use super::{adapter::ProviderError, openai::OpenAICompatibleAdapter};

// ── URL helper ────────────────────────────────────────────────────────────────

/// Build a deployment-scoped Azure OpenAI operation URL.
///
/// ```text
/// {resource_endpoint}/openai/deployments/{deployment}/{operation}?api-version={api_version}
/// ```
///
/// A trailing slash on `resource_endpoint` is stripped.
pub(crate) fn azure_operation_url(
    resource_endpoint: &str,
    deployment: &str,
    operation: &str,
    api_version: &str,
) -> String {
    format!(
        "{}/openai/deployments/{deployment}/{operation}?api-version={api_version}",
        resource_endpoint.trim_end_matches('/')
    )
}

// ── Adapter alias + factory ───────────────────────────────────────────────────

/// Adapter for Azure OpenAI Service.
///
/// This is [`OpenAICompatibleAdapter`] pre-configured with Azure auth
/// (`api-key` header) and a deployment-scoped base URL. Construct with
/// [`new_azure_adapter`].
pub type AzureOpenAiAdapter = OpenAICompatibleAdapter;

/// Build an [`OpenAICompatibleAdapter`] targeting an Azure OpenAI deployment.
///
/// - `resource_endpoint`: base URL, e.g. `"https://acme.openai.azure.com"`.
///   A trailing slash is stripped automatically.
/// - `deployment`: the Azure deployment name (maps to the URL path segment).
/// - `api_version`: Azure API version string, e.g. `"2024-10-21"`.
/// - `api_key`: the Azure `api-key` credential.
/// - `timeout_secs`: per-request HTTP timeout.
pub fn new_azure_adapter(
    resource_endpoint: &str,
    deployment: &str,
    api_version: &str,
    api_key: &str,
    timeout_secs: u64,
) -> Result<OpenAICompatibleAdapter, ProviderError> {
    let api_base = format!(
        "{}/openai/deployments/{deployment}",
        resource_endpoint.trim_end_matches('/')
    );
    OpenAICompatibleAdapter::new(api_base, Some(api_key.to_owned()), timeout_secs)
        .map(|a| a.with_api_version(api_version).with_azure_auth(true))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::adapter::{ChatMessage, ChatRole, CompletionRequest, ProviderAdapter};

    #[test]
    fn builds_deployment_scoped_operation_urls() {
        assert_eq!(
            azure_operation_url(
                "https://acme.openai.azure.com/",
                "prod-gpt",
                "chat/completions",
                "2024-10-21"
            ),
            "https://acme.openai.azure.com/openai/deployments/prod-gpt/chat/completions?api-version=2024-10-21"
        );
        assert_eq!(
            azure_operation_url(
                "https://acme.openai.azure.com",
                "embed",
                "embeddings",
                "2024-10-21"
            ),
            "https://acme.openai.azure.com/openai/deployments/embed/embeddings?api-version=2024-10-21"
        );
    }

    #[test]
    fn debug_redacts_api_key() {
        let adapter = new_azure_adapter(
            "https://acme.openai.azure.com",
            "prod",
            "2024-10-21",
            "super-secret-key",
            30,
        )
        .unwrap();
        let debug = format!("{adapter:?}");
        assert!(!debug.contains("super-secret-key"));
        assert!(debug.contains("***"));
    }

    // Gate the integration test on the `web` feature since it boots an axum
    // server and axum is only available when `web` is active.
    #[cfg(feature = "web")]
    #[tokio::test]
    async fn completion_uses_deployment_url_api_key_and_azure_body() {
        use axum::{extract::OriginalUri, http::HeaderMap, routing::post, Json, Router};
        use std::sync::Arc;
        use tokio::sync::{oneshot, Mutex};

        let (sender, receiver) = oneshot::channel();
        let sender = Arc::new(Mutex::new(Some(sender)));
        let app = Router::new().route(
            "/openai/deployments/prod/chat/completions",
            post({
                let sender = Arc::clone(&sender);
                move |headers: HeaderMap,
                      OriginalUri(uri): OriginalUri,
                      Json(body): Json<serde_json::Value>| {
                    let sender = Arc::clone(&sender);
                    async move {
                        if let Some(sender) = sender.lock().await.take() {
                            let _ = sender.send((headers, uri, body));
                        }
                        Json(serde_json::json!({
                            "id": "azure-response",
                            "choices": [{
                                "message": {"content": "ok", "tool_calls": []},
                                "finish_reason": "stop"
                            }],
                            "usage": {"prompt_tokens": 3, "completion_tokens": 1}
                        }))
                    }
                }
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let adapter = new_azure_adapter(
            &format!("http://{address}"),
            "prod",
            "2024-10-21",
            "azure-secret",
            30,
        )
        .unwrap()
        .with_max_completion_tokens(true);
        let response = adapter
            .complete(CompletionRequest {
                model: "logical-model-is-not-used-in-url".into(),
                messages: vec![ChatMessage {
                    role: ChatRole::User,
                    content: "hello".into(),
                }],
                system: Some("system".into()),
                tools: None,
                temperature: Some(0.2),
                top_p: Some(0.9),
                max_tokens: 42,
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(response.content, "ok");
        let (headers, uri, body) = receiver.await.unwrap();
        assert_eq!(headers.get("api-key").unwrap(), "azure-secret");
        assert_eq!(
            uri.path_and_query().unwrap().as_str(),
            "/openai/deployments/prod/chat/completions?api-version=2024-10-21"
        );
        assert_eq!(body["max_completion_tokens"], 42);
        assert!(body.get("max_tokens").is_none());
        assert_eq!(body["messages"][0]["role"], "system");
        assert_eq!(body["messages"][1]["role"], "user");
    }
}
