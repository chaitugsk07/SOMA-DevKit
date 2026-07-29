//! OpenAI-compatible adapter.
//!
//! Covers: OpenAI, Azure OpenAI, Groq, DeepSeek, vLLM, Ollama — any endpoint
//! that speaks the OpenAI `/v1/chat/completions` wire format.
//!
//! Hand-rolled on reqwest; no `async-openai` dependency (ponytail: avoids ~10
//! transitive deps for a thin adapter that fits in ~200 lines).

use std::pin::Pin;
use std::time::Duration;

use super::adapter::{
    drain_lines, is_false, ChatRole, CompletionRequest, CompletionResponse, EmbedRequest,
    EmbedResponse, ProviderAdapter, ProviderError, ReasoningEffort, StreamEvent, ToolCall, Usage,
};

// ── Wire types (OpenAI request) ───────────────────────────────────────────────

#[derive(serde::Serialize, Default)]
struct OaiRequest {
    model: String,
    messages: Vec<OaiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OaiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    /// Standard field for most endpoints (vLLM, Ollama, Groq, OpenAI).
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    /// o-series / Azure convention; set instead of `max_tokens` when the
    /// adapter flag `use_max_completion_tokens` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_completion_tokens: Option<u32>,
    #[serde(skip_serializing_if = "is_false")]
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_options: Option<OaiStreamOptions>,
    /// Structured JSON output schema. Built from `CompletionRequest::json_schema`.
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<serde_json::Value>,
    /// Stop sequences. Maps from `CompletionRequest::stop_sequences`.
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    /// Reasoning effort for o-series models. Maps from `CompletionRequest::reasoning_effort`.
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_effort: Option<&'static str>,
}

#[derive(serde::Serialize)]
struct OaiMessage {
    role: String,
    content: String,
}

#[derive(serde::Serialize)]
struct OaiTool {
    #[serde(rename = "type")]
    ty: &'static str,
    function: OaiFunction,
}

#[derive(serde::Serialize)]
struct OaiFunction {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    parameters: serde_json::Value,
}

#[derive(serde::Serialize)]
struct OaiStreamOptions {
    include_usage: bool,
}

#[derive(serde::Serialize)]
struct OaiEmbedRequest {
    model: String,
    input: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<usize>,
}

// ── Wire types (OpenAI response) ──────────────────────────────────────────────

#[derive(serde::Deserialize)]
struct OaiResponse {
    id: String,
    choices: Vec<OaiChoice>,
    usage: Option<OaiUsage>,
}

#[derive(serde::Deserialize)]
struct OaiChoice {
    message: OaiChoiceMsg,
    finish_reason: Option<String>,
}

#[derive(serde::Deserialize)]
struct OaiChoiceMsg {
    content: Option<String>,
    tool_calls: Option<Vec<OaiToolCall>>,
}

#[derive(serde::Deserialize)]
struct OaiToolCall {
    id: String,
    function: OaiFunctionCall,
}

#[derive(serde::Deserialize)]
struct OaiFunctionCall {
    name: String,
    arguments: String,
}

#[derive(serde::Deserialize)]
struct OaiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

#[derive(serde::Deserialize)]
struct OaiEmbedResponse {
    data: Vec<OaiEmbedding>,
    usage: Option<OaiUsage>,
}

#[derive(serde::Deserialize)]
struct OaiEmbedding {
    embedding: Vec<f32>,
}

// ── SSE chunk types ───────────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
struct OaiChunk {
    #[allow(dead_code)]
    id: Option<String>,
    choices: Vec<OaiChunkChoice>,
    usage: Option<OaiUsage>,
}

#[derive(serde::Deserialize)]
struct OaiChunkChoice {
    delta: OaiDelta,
    finish_reason: Option<String>,
}

#[derive(serde::Deserialize, Default)]
struct OaiDelta {
    content: Option<String>,
    tool_calls: Option<Vec<OaiDeltaToolCall>>,
}

#[derive(serde::Deserialize)]
struct OaiDeltaToolCall {
    id: Option<String>,
    function: Option<OaiDeltaFunction>,
}

#[derive(serde::Deserialize)]
struct OaiDeltaFunction {
    name: Option<String>,
    arguments: Option<String>,
}

// ── OpenAICompatibleAdapter ───────────────────────────────────────────────────

/// Adapter for any OpenAI-compatible endpoint.
///
/// `api_key` is `None` for local endpoints (Ollama) that don't require auth.
#[derive(Clone)]
pub struct OpenAICompatibleAdapter {
    pub(crate) http: reqwest::Client,
    pub(crate) api_base: String,
    pub(crate) api_key: Option<String>,
    /// Azure OpenAI API version (e.g. `"2024-02-01"`).
    pub(crate) api_version: Option<String>,
    /// When true, sends `max_completion_tokens` instead of `max_tokens`.
    ///
    /// Required for o-series / Azure AI Foundry models that reject the
    /// standard `max_tokens` field (e.g. gpt-5.6-sol).  Default: false.
    pub(crate) use_max_completion_tokens: bool,
    /// When true, sends `api-key: {key}` instead of `Authorization: Bearer {key}`.
    ///
    /// Required for Azure OpenAI endpoints, which expect the credential in the
    /// `api-key` header rather than the standard `Authorization: Bearer` scheme.
    /// Default: false.
    pub(crate) use_azure_auth: bool,
    /// When true, omits `temperature`, `top_p`, and `stop` from the serialised
    /// request entirely (not sent as null — not sent at all).
    ///
    /// Required for newer reasoning models (gpt-5.x, o-series) that reject any
    /// non-default `temperature`/`top_p` with HTTP 400 and reject `stop`
    /// entirely. `reasoning_effort` and `response_format` are left untouched.
    /// Default: false.
    pub(crate) omit_sampling_params: bool,
}

impl std::fmt::Debug for OpenAICompatibleAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAICompatibleAdapter")
            .field("api_base", &self.api_base)
            .field("api_key", &self.api_key.as_ref().map(|_| "***"))
            .field("api_version", &self.api_version)
            .field("use_max_completion_tokens", &self.use_max_completion_tokens)
            .field("use_azure_auth", &self.use_azure_auth)
            .field("omit_sampling_params", &self.omit_sampling_params)
            .finish()
    }
}

impl OpenAICompatibleAdapter {
    /// Create an adapter.
    ///
    /// - `api_base`: base URL, e.g. `"https://api.openai.com/v1"` or `"http://localhost:11434/v1"`.
    /// - `api_key`: `None` for Ollama or other unauthenticated local endpoints.
    /// - `timeout_secs`: per-request HTTP timeout.
    pub fn new(
        api_base: impl Into<String>,
        api_key: Option<String>,
        timeout_secs: u64,
    ) -> Result<Self, ProviderError> {
        let http = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .map_err(ProviderError::Http)?;
        Ok(Self {
            http,
            api_base: api_base.into(),
            api_key,
            api_version: None,
            use_max_completion_tokens: false,
            use_azure_auth: false,
            omit_sampling_params: false,
        })
    }

    /// Set the Azure OpenAI API version query parameter.
    pub fn with_api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = Some(version.into());
        self
    }

    /// Send `max_completion_tokens` instead of `max_tokens`.
    ///
    /// Set to `true` for o-series / Azure AI Foundry models (e.g. `gpt-5.6-sol`)
    /// that reject the standard `max_tokens` field.  Default: `false`.
    pub fn with_max_completion_tokens(mut self, yes: bool) -> Self {
        self.use_max_completion_tokens = yes;
        self
    }

    /// Use Azure OpenAI auth (`api-key: {key}` header instead of `Authorization: Bearer {key}`).
    ///
    /// Set to `true` when targeting an Azure OpenAI deployment URL. Default: `false`.
    /// The adapter must be constructed with a non-`None` `api_key` when Azure auth is enabled.
    pub fn with_azure_auth(mut self, yes: bool) -> Self {
        self.use_azure_auth = yes;
        self
    }

    /// Omit `temperature`, `top_p`, and `stop` from the serialised request entirely.
    ///
    /// Set to `true` for newer reasoning models (gpt-5.x, o-series / Azure AI Foundry)
    /// that reject any non-default `temperature`/`top_p` with HTTP 400 and reject
    /// `stop` entirely. `reasoning_effort` and `response_format` are left untouched.
    /// Default: `false`.
    pub fn with_omit_sampling_params(mut self, yes: bool) -> Self {
        self.omit_sampling_params = yes;
        self
    }

    fn chat_url(&self) -> String {
        let base = format!("{}/chat/completions", self.api_base);
        if let Some(ref v) = self.api_version {
            format!("{base}?api-version={v}")
        } else {
            base
        }
    }

    fn embed_url(&self) -> String {
        let base = format!("{}/embeddings", self.api_base);
        if let Some(ref v) = self.api_version {
            format!("{base}?api-version={v}")
        } else {
            base
        }
    }

    fn auth_header(&self) -> String {
        match &self.api_key {
            Some(k) => format!("Bearer {k}"),
            None => "Bearer ollama".to_string(),
        }
    }

    fn apply_auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if self.use_azure_auth {
            req.header("api-key", self.api_key.as_deref().unwrap_or(""))
        } else {
            req.header("Authorization", &self.auth_header())
        }
    }

    fn build_messages(&self, req: &CompletionRequest) -> Vec<OaiMessage> {
        let mut msgs: Vec<OaiMessage> = Vec::new();

        // System prompt: top-level `system` field takes precedence, else first System message.
        if let Some(ref s) = req.system {
            msgs.push(OaiMessage {
                role: "system".to_string(),
                content: s.clone(),
            });
        } else if let Some(sys_msg) = req.messages.iter().find(|m| m.role == ChatRole::System) {
            msgs.push(OaiMessage {
                role: "system".to_string(),
                content: sys_msg.content.clone(),
            });
        }

        for m in req.messages.iter().filter(|m| m.role != ChatRole::System) {
            msgs.push(OaiMessage {
                role: match m.role {
                    ChatRole::User => "user".to_string(),
                    ChatRole::Assistant => "assistant".to_string(),
                    ChatRole::System => "system".to_string(),
                },
                content: m.content.clone(),
            });
        }
        msgs
    }

    fn map_error(status: u16, body: String) -> ProviderError {
        match status {
            401 | 403 => ProviderError::Auth,
            429 => ProviderError::RateLimited {
                retry_after_secs: None,
            },
            // Include the provider's error body so callers can log the real reason.
            // Body is the API error message — headers (where keys live) are never included.
            400..=499 => ProviderError::BadRequest { status, body },
            500..=599 => ProviderError::ProviderUnavailable,
            _ => ProviderError::Malformed(format!("HTTP {status}")),
        }
    }
}

#[async_trait::async_trait]
impl ProviderAdapter for OpenAICompatibleAdapter {
    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, ProviderError> {
        let messages = self.build_messages(&req);
        let tools: Option<Vec<OaiTool>> = req.tools.as_ref().map(|ts| {
            ts.iter()
                .map(|t| OaiTool {
                    ty: "function",
                    function: OaiFunction {
                        name: t.name.clone(),
                        description: t.description.clone(),
                        parameters: t.parameters.clone(),
                    },
                })
                .collect()
        });

        let (max_tokens, max_completion_tokens) = if self.use_max_completion_tokens {
            (None, Some(req.max_tokens))
        } else {
            (Some(req.max_tokens), None)
        };

        // ponytail: cache_system_prompt and adaptive_thinking are Anthropic-only; silently ignored.
        let response_format = req.json_schema.as_ref().map(|schema| {
            let name = schema["title"].as_str().unwrap_or("output").to_owned();
            serde_json::json!({
                "type": "json_schema",
                "json_schema": {"name": name, "strict": true, "schema": schema}
            })
        });
        // Newer reasoning models (gpt-5.x, o-series) reject non-default temperature/top_p
        // and reject stop entirely. Omit when omit_sampling_params is set.
        let (temperature, top_p, stop) = if self.omit_sampling_params {
            (None, None, None)
        } else {
            (req.temperature, req.top_p, req.stop_sequences.clone())
        };
        let reasoning_effort = req.reasoning_effort.map(|e| match e {
            ReasoningEffort::Low => "low",
            ReasoningEffort::Medium => "medium",
            ReasoningEffort::High => "high",
            // ponytail: OpenAI reasoning_effort does not accept "xhigh"/"max"; clamp to
            // "high" so callers (including Azure) setting XHigh/Max don't get a runtime error.
            ReasoningEffort::XHigh | ReasoningEffort::Max => "high",
        });

        let body = OaiRequest {
            model: req.model.clone(),
            messages,
            tools,
            temperature,
            top_p,
            max_tokens,
            max_completion_tokens,
            stream: false,
            stream_options: None,
            response_format,
            stop,
            reasoning_effort,
        };

        let resp = self
            .apply_auth(self.http.post(self.chat_url()))
            .json(&body)
            .send()
            .await
            .map_err(ProviderError::Http)?;

        let status = resp.status().as_u16();
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Self::map_error(status, body));
        }

        let parsed: OaiResponse = resp
            .json()
            .await
            .map_err(|e| ProviderError::Malformed(e.to_string()))?;

        let choice = parsed
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| ProviderError::Malformed("empty choices".into()))?;

        let content = choice.message.content.unwrap_or_default();
        let stop_reason = choice.finish_reason;

        let tool_calls = choice
            .message
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .map(|tc| {
                let arguments =
                    serde_json::from_str(&tc.function.arguments).unwrap_or(serde_json::Value::Null);
                ToolCall {
                    id: tc.id,
                    name: tc.function.name,
                    arguments,
                }
            })
            .collect();

        let usage = parsed
            .usage
            .map(|u| Usage {
                input_tokens: u.prompt_tokens,
                output_tokens: u.completion_tokens,
                ..Default::default()
            })
            .unwrap_or_default();

        Ok(CompletionResponse {
            id: parsed.id,
            content,
            tool_calls,
            stop_reason,
            usage,
        })
    }

    async fn complete_stream(
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
        let messages = self.build_messages(&req);
        let tools: Option<Vec<OaiTool>> = req.tools.as_ref().map(|ts| {
            ts.iter()
                .map(|t| OaiTool {
                    ty: "function",
                    function: OaiFunction {
                        name: t.name.clone(),
                        description: t.description.clone(),
                        parameters: t.parameters.clone(),
                    },
                })
                .collect()
        });

        let (max_tokens, max_completion_tokens) = if self.use_max_completion_tokens {
            (None, Some(req.max_tokens))
        } else {
            (Some(req.max_tokens), None)
        };

        // ponytail: cache_system_prompt and adaptive_thinking are Anthropic-only; silently ignored.
        let response_format = req.json_schema.as_ref().map(|schema| {
            let name = schema["title"].as_str().unwrap_or("output").to_owned();
            serde_json::json!({
                "type": "json_schema",
                "json_schema": {"name": name, "strict": true, "schema": schema}
            })
        });
        // Newer reasoning models (gpt-5.x, o-series) reject non-default temperature/top_p
        // and reject stop entirely. Omit when omit_sampling_params is set.
        let (temperature, top_p, stop) = if self.omit_sampling_params {
            (None, None, None)
        } else {
            (req.temperature, req.top_p, req.stop_sequences.clone())
        };
        let reasoning_effort = req.reasoning_effort.map(|e| match e {
            ReasoningEffort::Low => "low",
            ReasoningEffort::Medium => "medium",
            ReasoningEffort::High => "high",
            // ponytail: OpenAI reasoning_effort does not accept "xhigh"/"max"; clamp to
            // "high" so callers (including Azure) setting XHigh/Max don't get a runtime error.
            ReasoningEffort::XHigh | ReasoningEffort::Max => "high",
        });

        // Azure api-versions before 2024-02-01 reject stream_options with HTTP 400.
        // Omit it when use_azure_auth is true to restore the exact pre-migration behavior.
        let stream_options = if self.use_azure_auth {
            None
        } else {
            Some(OaiStreamOptions {
                include_usage: true,
            })
        };
        let body = OaiRequest {
            model: req.model.clone(),
            messages,
            tools,
            temperature,
            top_p,
            max_tokens,
            max_completion_tokens,
            stream: true,
            stream_options,
            response_format,
            stop,
            reasoning_effort,
        };

        let resp = self
            .apply_auth(self.http.post(self.chat_url()))
            .json(&body)
            .send()
            .await
            .map_err(ProviderError::Http)?;

        let status = resp.status().as_u16();
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Self::map_error(status, body));
        }

        // Switch to the byte stream for genuinely incremental SSE parsing.
        let byte_stream = resp.bytes_stream();

        Ok(Box::pin(async_stream::stream! {
            tokio::pin!(byte_stream);
            // Byte-level line buffer — accumulates bytes across chunk boundaries,
            // including mid-line splits and mid-UTF-8-codepoint splits.
            let mut buf: Vec<u8> = Vec::new();
            let mut done = false;
            // OpenAI with stream_options.include_usage sends finish_reason in one
            // chunk and usage in a SEPARATE trailing chunk, then [DONE]. Buffer
            // both and emit a single Done at [DONE] so consumers see correct usage.
            let mut buffered_stop: Option<String> = None;
            let mut buffered_usage: Option<Usage> = None;

            loop {
                if done { break; }

                let item = std::future::poll_fn(|cx| {
                    futures_core::Stream::poll_next(byte_stream.as_mut(), cx)
                }).await;

                match item {
                    None => {
                        // Clean EOF — process any remaining bytes as a final line.
                        if !buf.is_empty() && !done {
                            let line = std::mem::take(&mut buf);
                            let s = String::from_utf8_lossy(&line);
                            if let Some(data) = s.strip_prefix("data: ") {
                                if data != "[DONE]" {
                                    for ev in dispatch_openai_data(data) {
                                        match ev {
                                            Ok(StreamEvent::Done { stop_reason, usage }) => {
                                                if stop_reason.is_some() { buffered_stop = stop_reason; }
                                                if usage.is_some() { buffered_usage = usage; }
                                            }
                                            other => yield other,
                                        }
                                    }
                                }
                            }
                        }
                        // Emit any buffered Done at stream end (no [DONE] line received).
                        if buffered_stop.is_some() || buffered_usage.is_some() {
                            yield Ok(StreamEvent::Done {
                                stop_reason: buffered_stop.take(),
                                usage: buffered_usage.take(),
                            });
                        }
                        break;
                    }
                    Some(Err(e)) => {
                        // Transport error mid-stream surfaces as an Err item.
                        yield Err(ProviderError::Http(e));
                        return;
                    }
                    Some(Ok(chunk)) => {
                        buf.extend_from_slice(&chunk);
                        let lines = drain_lines(&mut buf);
                        for line_bytes in lines {
                            if done { break; }
                            // Skip blank lines and SSE comment lines.
                            if line_bytes.is_empty() || line_bytes.starts_with(b":") {
                                continue;
                            }
                            let line = String::from_utf8_lossy(&line_bytes);
                            let data = match line.strip_prefix("data: ") {
                                Some(d) => d,
                                None => continue,
                            };
                            if data == "[DONE]" {
                                // Emit the single Done with all accumulated state.
                                yield Ok(StreamEvent::Done {
                                    stop_reason: buffered_stop.take(),
                                    usage: buffered_usage.take(),
                                });
                                done = true;
                                break;
                            }
                            for ev in dispatch_openai_data(data) {
                                match ev {
                                    Ok(StreamEvent::Done { stop_reason, usage }) => {
                                        if stop_reason.is_some() { buffered_stop = stop_reason; }
                                        if usage.is_some() { buffered_usage = usage; }
                                    }
                                    other => yield other,
                                }
                            }
                        }
                    }
                }
            }
        }))
    }

    async fn embed(&self, req: EmbedRequest) -> Result<EmbedResponse, ProviderError> {
        let body = OaiEmbedRequest {
            model: req.model.clone(),
            input: req.texts.clone(),
            dimensions: req.dimensions,
        };

        let resp = self
            .apply_auth(self.http.post(self.embed_url()))
            .json(&body)
            .send()
            .await
            .map_err(ProviderError::Http)?;

        let status = resp.status().as_u16();
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Self::map_error(status, body));
        }

        let parsed: OaiEmbedResponse = resp
            .json()
            .await
            .map_err(|e| ProviderError::Malformed(e.to_string()))?;

        let usage = parsed.usage.map(|u| Usage {
            input_tokens: u.prompt_tokens,
            output_tokens: u.completion_tokens,
            ..Default::default()
        });

        Ok(EmbedResponse {
            embeddings: parsed.data.into_iter().map(|e| e.embedding).collect(),
            usage,
        })
    }
}

// ── SSE event dispatcher + batch parser ──────────────────────────────────────

/// Dispatch one OpenAI-compatible SSE `data:` payload into zero or more
/// [`StreamEvent`]s.
///
/// OpenAI events are always single-line (`data: <json>`), so this takes the
/// value after the `"data: "` prefix already stripped. Returns an error item
/// on parse failure rather than panicking.
pub(crate) fn dispatch_openai_data(data: &str) -> Vec<Result<StreamEvent, ProviderError>> {
    let mut out = Vec::new();

    let chunk: OaiChunk = match serde_json::from_str(data) {
        Ok(c) => c,
        Err(e) => return vec![Err(ProviderError::Malformed(e.to_string()))],
    };

    let mut last_finish_reason: Option<String> = None;

    for choice in &chunk.choices {
        last_finish_reason = choice.finish_reason.clone();

        if let Some(ref text) = choice.delta.content {
            if !text.is_empty() {
                out.push(Ok(StreamEvent::Token(text.clone())));
            }
        }

        if let Some(ref tcs) = choice.delta.tool_calls {
            for tc in tcs {
                let id = tc.id.clone().unwrap_or_default();
                let name = tc
                    .function
                    .as_ref()
                    .and_then(|f| f.name.clone())
                    .unwrap_or_default();
                let args_delta = tc
                    .function
                    .as_ref()
                    .and_then(|f| f.arguments.clone())
                    .unwrap_or_default();
                out.push(Ok(StreamEvent::ToolCallDelta {
                    id,
                    name,
                    arguments_delta: args_delta,
                }));
            }
        }
    }

    // If last chunk carried usage and/or finish_reason, emit Done.
    if last_finish_reason.is_some() || chunk.usage.is_some() {
        let usage = chunk.usage.map(|u| Usage {
            input_tokens: u.prompt_tokens,
            output_tokens: u.completion_tokens,
            ..Default::default()
        });
        out.push(Ok(StreamEvent::Done {
            stop_reason: last_finish_reason,
            usage,
        }));
    }

    out
}

/// Parse a complete OpenAI-compatible SSE response body into a vec of events.
///
/// Uses the same [`dispatch_openai_data`] helper as `complete_stream`.
#[cfg(test)]
pub(crate) fn parse_openai_sse(body: String) -> Vec<Result<StreamEvent, ProviderError>> {
    let mut events = Vec::new();
    let mut buf: Vec<u8> = body.into_bytes();
    let lines = drain_lines(&mut buf);

    for line_bytes in lines {
        if line_bytes.is_empty() || line_bytes.starts_with(b":") {
            continue;
        }
        let line = String::from_utf8_lossy(&line_bytes);
        let data = match line.strip_prefix("data: ") {
            Some(d) => d,
            None => continue,
        };
        if data == "[DONE]" {
            break;
        }
        events.extend(dispatch_openai_data(data));
    }

    events
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::adapter::{ChatMessage, ChatRole, CompletionRequest, ToolDefinition};

    fn sample_request() -> CompletionRequest {
        CompletionRequest {
            model: "gpt-4o".to_string(),
            messages: vec![ChatMessage {
                role: ChatRole::User,
                content: "Hello".to_string(),
            }],
            system: Some("You are helpful.".to_string()),
            tools: None,
            temperature: None,
            top_p: None,
            max_tokens: 256,
            ..Default::default()
        }
    }

    fn make_adapter() -> OpenAICompatibleAdapter {
        OpenAICompatibleAdapter {
            http: reqwest::Client::new(),
            api_base: "https://api.openai.com/v1".to_string(),
            api_key: Some("sk-test".to_string()),
            api_version: None,
            use_max_completion_tokens: false,
            use_azure_auth: false,
            omit_sampling_params: false,
        }
    }

    #[test]
    fn openai_complete_request_serializes() {
        let adapter = make_adapter();
        let req = sample_request();
        let messages = adapter.build_messages(&req);

        // System message should be first
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[0].content, "You are helpful.");
        assert_eq!(messages[1].role, "user");
        assert_eq!(messages[1].content, "Hello");

        let body = OaiRequest {
            model: req.model.clone(),
            messages,
            tools: None,
            temperature: req.temperature,
            top_p: req.top_p,
            max_tokens: Some(req.max_tokens),
            max_completion_tokens: None,
            stream: false,
            stream_options: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"gpt-4o\""), "model");
        assert!(json.contains("\"max_tokens\":256"), "max_tokens present");
        assert!(
            !json.contains("\"max_completion_tokens\""),
            "max_completion_tokens absent by default"
        );
        assert!(!json.contains("\"stream\""), "stream absent when false");
    }

    #[test]
    fn openai_complete_request_with_tools_serializes() {
        let adapter = make_adapter();
        let mut req = sample_request();
        req.tools = Some(vec![ToolDefinition {
            name: "search".to_string(),
            description: Some("Search the web".to_string()),
            parameters: serde_json::json!({"type": "object"}),
        }]);
        let messages = adapter.build_messages(&req);
        let tools: Option<Vec<OaiTool>> = req.tools.as_ref().map(|ts| {
            ts.iter()
                .map(|t| OaiTool {
                    ty: "function",
                    function: OaiFunction {
                        name: t.name.clone(),
                        description: t.description.clone(),
                        parameters: t.parameters.clone(),
                    },
                })
                .collect()
        });
        let body = OaiRequest {
            model: req.model.clone(),
            messages,
            tools,
            temperature: req.temperature,
            top_p: req.top_p,
            max_tokens: Some(req.max_tokens),
            max_completion_tokens: None,
            stream: false,
            stream_options: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"type\":\"function\""), "tool type");
        assert!(json.contains("\"search\""), "tool name");
    }

    #[test]
    fn openai_sse_parsing() {
        let sse = "\
data: {\"id\":\"chatcmpl-1\",\"object\":\"chat.completion.chunk\",\"choices\":[{\"delta\":{\"content\":\"Hello\"},\"finish_reason\":null}]}\n\
data: {\"id\":\"chatcmpl-1\",\"object\":\"chat.completion.chunk\",\"choices\":[{\"delta\":{\"content\":\" world\"},\"finish_reason\":null}]}\n\
data: {\"id\":\"chatcmpl-1\",\"object\":\"chat.completion.chunk\",\"choices\":[{\"delta\":{},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":5}}\n\
data: [DONE]\n\
";
        let events = parse_openai_sse(sse.to_string());
        let ok_events: Vec<&StreamEvent> = events.iter().filter_map(|e| e.as_ref().ok()).collect();

        assert!(ok_events.len() >= 3, "expected at least 3 events");
        assert!(matches!(ok_events[0], StreamEvent::Token(t) if t == "Hello"));
        assert!(matches!(ok_events[1], StreamEvent::Token(t) if t == " world"));
        assert!(
            matches!(ok_events[2], StreamEvent::Done { stop_reason: Some(r), .. } if r == "stop")
        );
    }

    #[test]
    fn openai_embed_request_serializes() {
        let texts = vec!["hello world".to_string(), "foo bar".to_string()];
        let body = OaiEmbedRequest {
            model: "text-embedding-3-small".to_string(),
            input: texts,
            dimensions: Some(768),
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"text-embedding-3-small\""), "model");
        assert!(json.contains("\"hello world\""), "first text");
        assert!(json.contains("\"foo bar\""), "second text");
        assert!(json.contains("\"dimensions\":768"), "dimensions");
    }

    // ── max_tokens vs max_completion_tokens serialization ────────────────────

    #[test]
    fn request_uses_max_tokens_by_default() {
        // Default adapter (use_max_completion_tokens = false) must send
        // max_tokens and must NOT send max_completion_tokens.
        let body = OaiRequest {
            model: "gpt-4o".to_string(),
            messages: vec![],
            tools: None,
            temperature: None,
            top_p: None,
            max_tokens: Some(512),
            max_completion_tokens: None,
            stream: false,
            stream_options: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"max_tokens\":512"), "must have max_tokens");
        assert!(
            !json.contains("max_completion_tokens"),
            "must NOT have max_completion_tokens"
        );
    }

    #[test]
    fn request_uses_max_completion_tokens_when_flag_set() {
        // When use_max_completion_tokens = true the adapter passes None to
        // max_tokens and Some(n) to max_completion_tokens.
        let body = OaiRequest {
            model: "gpt-5.6-sol".to_string(),
            messages: vec![],
            tools: None,
            temperature: None,
            top_p: None,
            max_tokens: None,
            max_completion_tokens: Some(512),
            stream: false,
            stream_options: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(
            json.contains("\"max_completion_tokens\":512"),
            "must have max_completion_tokens"
        );
        assert!(!json.contains("\"max_tokens\""), "must NOT have max_tokens");
    }

    #[test]
    fn with_max_completion_tokens_builder_toggles_flag() {
        let adapter = make_adapter();
        assert!(!adapter.use_max_completion_tokens, "default is false");
        let adapter2 = adapter.with_max_completion_tokens(true);
        assert!(adapter2.use_max_completion_tokens, "flag set via builder");
        let adapter3 = adapter2.with_max_completion_tokens(false);
        assert!(
            !adapter3.use_max_completion_tokens,
            "flag cleared via builder"
        );
    }

    // ── incremental parser boundary tests ────────────────────────────────────
    //
    // Exercise `dispatch_openai_data` + `drain_lines` by feeding byte slices
    // split at awkward positions.

    /// Run the same incremental logic used in `complete_stream` over an
    /// arbitrary sequence of byte slices (simulating reqwest chunk delivery).
    ///
    /// Mirrors the Done-buffering in `complete_stream`: Done events from
    /// `dispatch_openai_data` are buffered; a single Done is emitted at [DONE].
    fn parse_chunks(chunks: &[&[u8]]) -> Vec<Result<StreamEvent, ProviderError>> {
        let mut events = Vec::new();
        let mut buf: Vec<u8> = Vec::new();
        let mut done = false;
        let mut buffered_stop: Option<String> = None;
        let mut buffered_usage: Option<Usage> = None;

        for chunk in chunks {
            if done {
                break;
            }
            buf.extend_from_slice(chunk);
            let lines = drain_lines(&mut buf);
            for line_bytes in lines {
                if done {
                    break;
                }
                if line_bytes.is_empty() || line_bytes.starts_with(b":") {
                    continue;
                }
                let line = String::from_utf8_lossy(&line_bytes);
                let data = match line.strip_prefix("data: ") {
                    Some(d) => d,
                    None => continue,
                };
                if data == "[DONE]" {
                    events.push(Ok(StreamEvent::Done {
                        stop_reason: buffered_stop.take(),
                        usage: buffered_usage.take(),
                    }));
                    done = true;
                    break;
                }
                for ev in dispatch_openai_data(data) {
                    match ev {
                        Ok(StreamEvent::Done { stop_reason, usage }) => {
                            if stop_reason.is_some() {
                                buffered_stop = stop_reason;
                            }
                            if usage.is_some() {
                                buffered_usage = usage;
                            }
                        }
                        other => events.push(other),
                    }
                }
            }
        }
        events
    }

    #[test]
    fn openai_incremental_mid_line_split() {
        // Split mid-way through the "data: " prefix of the first chunk line.
        let full: &[u8] = b"data: {\"id\":\"c1\",\"choices\":[{\"delta\":{\"content\":\"Hi\"},\"finish_reason\":null}]}\ndata: [DONE]\n";
        // Split at byte 8 — inside "data: {..."
        let (a, b) = full.split_at(8);
        let events = parse_chunks(&[a, b]);
        // parse_chunks now emits Done at [DONE] — expect Token + Done.
        assert!(events.len() >= 1, "expected at least 1 event");
        assert!(
            matches!(&events[0], Ok(StreamEvent::Token(t)) if t == "Hi"),
            "first event should be Token(Hi), got {:?}",
            events[0]
        );
    }

    #[test]
    fn openai_incremental_mid_data_line_split() {
        // Two tokens, second chunk starts mid-way through the second data line.
        let chunk1: &[u8] =
            b"data: {\"id\":\"c1\",\"choices\":[{\"delta\":{\"content\":\"Hello\"},\"finish_reason\":null}]}\n";
        let chunk2_part1: &[u8] =
            b"data: {\"id\":\"c1\",\"choices\":[{\"delta\":{\"content\":\" world\"";
        let chunk2_part2: &[u8] = b"},\"finish_reason\":null}]}\ndata: [DONE]\n";
        let events = parse_chunks(&[chunk1, chunk2_part1, chunk2_part2]);
        // parse_chunks now emits Done at [DONE] — expect Token + Token + Done.
        assert!(
            events.len() >= 2,
            "expected at least 2 Token events, got {:?}",
            events
        );
        assert!(matches!(&events[0], Ok(StreamEvent::Token(t)) if t == "Hello"));
        assert!(matches!(&events[1], Ok(StreamEvent::Token(t)) if t == " world"));
    }

    #[test]
    fn openai_incremental_done_with_usage() {
        // Verify Done event carries usage when stream_options include_usage is active.
        let chunks: &[&[u8]] = &[
            b"data: {\"id\":\"c1\",\"choices\":[{\"delta\":{},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":5}}\n",
            b"data: [DONE]\n",
        ];
        let events = parse_chunks(chunks);
        assert_eq!(events.len(), 1, "expected 1 Done event");
        match &events[0] {
            Ok(StreamEvent::Done { stop_reason, usage }) => {
                assert_eq!(stop_reason.as_deref(), Some("stop"));
                let u = usage.expect("usage should be present");
                assert_eq!(u.input_tokens, 10);
                assert_eq!(u.output_tokens, 5);
            }
            other => panic!("expected Done, got {:?}", other),
        }
    }

    #[test]
    fn openai_incremental_mid_utf8_split() {
        // "é" = 0xC3 0xA9 — split the raw bytes mid-codepoint.
        let prefix: &[u8] = b"data: {\"id\":\"c1\",\"choices\":[{\"delta\":{\"content\":\"Hi\xC3";
        let suffix: &[u8] = b"\xA9\"},\"finish_reason\":null}]}\ndata: [DONE]\n";
        let events = parse_chunks(&[prefix, suffix]);
        // parse_chunks now emits Done at [DONE] — expect Token + Done.
        assert!(events.len() >= 1, "expected at least 1 event");
        match &events[0] {
            Ok(StreamEvent::Token(t)) => {
                assert!(t.contains("Hi"), "token should start with 'Hi': {:?}", t);
            }
            other => panic!("expected Token, got {:?}", other),
        }
    }

    // ── map_error body-surfacing test ────────────────────────────────────────

    /// Verify that a 400 response body is threaded into `ProviderError` and that
    /// no API key material leaks into the error's Display string.
    #[test]
    fn map_error_400_surfaces_body_and_no_key_material() {
        let body = r#"{"error":{"message":"temperature is not supported, only the default (1) value is supported"}}"#.to_string();
        let err = OpenAICompatibleAdapter::map_error(400, body);
        let msg = err.to_string();
        assert!(
            msg.contains("temperature"),
            "error Display must contain 'temperature' from the provider body: {msg}"
        );
        assert!(
            !msg.contains("Bearer"),
            "error Display must not contain 'Bearer' (auth header value): {msg}"
        );
        assert!(
            !msg.contains("sk-"),
            "error Display must not contain 'sk-' (API key prefix): {msg}"
        );
        // Confirm the variant is BadRequest, not Malformed.
        assert!(
            matches!(err, ProviderError::BadRequest { status: 400, .. }),
            "expected BadRequest variant for HTTP 400"
        );
    }

    #[test]
    fn map_error_401_is_auth() {
        let err = OpenAICompatibleAdapter::map_error(401, String::new());
        assert!(matches!(err, ProviderError::Auth));
    }

    #[test]
    fn map_error_429_is_rate_limited() {
        let err = OpenAICompatibleAdapter::map_error(429, String::new());
        assert!(matches!(err, ProviderError::RateLimited { .. }));
    }

    #[test]
    fn map_error_500_is_provider_unavailable() {
        let err = OpenAICompatibleAdapter::map_error(500, String::new());
        assert!(matches!(err, ProviderError::ProviderUnavailable));
    }

    #[test]
    fn openai_stream_usage_in_trailing_chunk() {
        // OpenAI with stream_options.include_usage sends finish_reason in one
        // chunk and usage in a SEPARATE trailing chunk, then [DONE].
        // The single Done event emitted at [DONE] must carry both stop_reason
        // and non-zero usage — not the usage=None from the finish_reason chunk.
        let chunks: &[&[u8]] = &[
            b"data: {\"id\":\"c1\",\"choices\":[{\"delta\":{\"content\":\"Hi\"},\"finish_reason\":null}]}\n",
            // finish_reason chunk — usage is null (usage arrives separately)
            b"data: {\"id\":\"c1\",\"choices\":[{\"delta\":{},\"finish_reason\":\"stop\"}]}\n",
            // trailing usage-only chunk — choices is empty, usage is populated
            b"data: {\"id\":\"c1\",\"choices\":[],\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":5}}\n",
            b"data: [DONE]\n",
        ];
        let events = parse_chunks(chunks);
        let ok_events: Vec<&StreamEvent> = events.iter().filter_map(|e| e.as_ref().ok()).collect();

        // Expected: Token("Hi"), Done(stop_reason="stop", usage=(10,5))
        assert!(
            ok_events.len() >= 2,
            "expected at least Token + Done, got {} events: {:?}",
            ok_events.len(),
            events
        );
        assert!(
            matches!(ok_events[0], StreamEvent::Token(t) if t == "Hi"),
            "first event must be Token(Hi)"
        );
        match ok_events.last().expect("no events") {
            StreamEvent::Done { stop_reason, usage } => {
                assert_eq!(
                    stop_reason.as_deref(),
                    Some("stop"),
                    "stop_reason must be populated from finish_reason chunk"
                );
                let u = usage.expect("usage must be present (from trailing chunk)");
                assert_eq!(u.input_tokens, 10, "input_tokens from trailing chunk");
                assert_eq!(u.output_tokens, 5, "output_tokens from trailing chunk");
            }
            other => panic!("last event should be Done, got {:?}", other),
        }
    }

    // ── stream_options / Azure conditional ───────────────────────────────────

    #[test]
    fn stream_options_absent_in_azure_mode() {
        // When use_azure_auth=true the serialized body must NOT contain
        // stream_options — Azure api-versions < 2024-02-01 reject the field.
        let body = OaiRequest {
            model: "gpt-4o".to_string(),
            messages: vec![],
            tools: None,
            temperature: None,
            top_p: None,
            max_tokens: Some(256),
            max_completion_tokens: None,
            stream: true,
            stream_options: None, // azure path sets None
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(
            !json.contains("stream_options"),
            "azure mode must not serialize stream_options: {json}"
        );
    }

    // ── New capability wire-mapping tests ────────────────────────────────────

    #[test]
    fn openai_json_schema_serializes_response_format() {
        let schema = serde_json::json!({
            "title": "MyOutput",
            "type": "object",
            "properties": {"value": {"type": "string"}}
        });
        let req = CompletionRequest {
            json_schema: Some(schema),
            ..sample_request()
        };
        let adapter = make_adapter();
        let messages = adapter.build_messages(&req);
        let response_format = req.json_schema.as_ref().map(|s| {
            let name = s["title"].as_str().unwrap_or("output").to_owned();
            serde_json::json!({
                "type": "json_schema",
                "json_schema": {"name": name, "strict": true, "schema": s}
            })
        });
        let body = OaiRequest {
            model: req.model.clone(),
            messages,
            max_tokens: Some(req.max_tokens),
            response_format,
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(
            json.contains("\"response_format\""),
            "response_format must be present: {json}"
        );
        assert!(
            json.contains("\"json_schema\""),
            "response_format.type must be json_schema: {json}"
        );
        assert!(
            json.contains("\"MyOutput\""),
            "schema title must appear as name: {json}"
        );
        assert!(
            json.contains("\"strict\":true"),
            "strict must be true: {json}"
        );
    }

    #[test]
    fn openai_stop_sequences_serializes_as_stop() {
        let req = CompletionRequest {
            stop_sequences: Some(vec!["<END>".to_string()]),
            ..sample_request()
        };
        let body = OaiRequest {
            stop: req.stop_sequences.clone(),
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(
            json.contains("\"stop\""),
            "stop field must be present: {json}"
        );
        assert!(json.contains("\"<END>\""), "stop value must appear: {json}");
    }

    #[test]
    fn openai_reasoning_effort_serializes() {
        // OaiRequest.reasoning_effort is &'static str — no enum conversion needed.
        let body = OaiRequest {
            reasoning_effort: Some("medium"),
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(
            json.contains("\"reasoning_effort\":\"medium\""),
            "reasoning_effort must serialize: {json}"
        );
    }

    #[test]
    fn openai_reasoning_effort_xhigh_clamps_to_high() {
        // OpenAI does not accept "xhigh"; the adapter must clamp to "high" silently.
        use crate::llm::adapter::ReasoningEffort;
        let req = CompletionRequest {
            reasoning_effort: Some(ReasoningEffort::XHigh),
            ..sample_request()
        };
        let effort = req.reasoning_effort.map(|e| match e {
            ReasoningEffort::Low => "low",
            ReasoningEffort::Medium => "medium",
            ReasoningEffort::High => "high",
            ReasoningEffort::XHigh | ReasoningEffort::Max => "high",
        });
        let body = OaiRequest {
            reasoning_effort: effort,
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(
            json.contains("\"reasoning_effort\":\"high\""),
            "XHigh must clamp to 'high' on OpenAI: {json}"
        );
    }

    #[test]
    fn openai_reasoning_effort_max_clamps_to_high() {
        // OpenAI does not accept "max"; the adapter must clamp to "high" silently.
        use crate::llm::adapter::ReasoningEffort;
        let req = CompletionRequest {
            reasoning_effort: Some(ReasoningEffort::Max),
            ..sample_request()
        };
        let effort = req.reasoning_effort.map(|e| match e {
            ReasoningEffort::Low => "low",
            ReasoningEffort::Medium => "medium",
            ReasoningEffort::High => "high",
            ReasoningEffort::XHigh | ReasoningEffort::Max => "high",
        });
        let body = OaiRequest {
            reasoning_effort: effort,
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(
            json.contains("\"reasoning_effort\":\"high\""),
            "Max must clamp to 'high' on OpenAI: {json}"
        );
    }

    #[test]
    fn openai_cache_and_adaptive_thinking_silently_ignored() {
        // Verify that setting Anthropic-only fields on a CompletionRequest does
        // NOT produce an error when passed through the OpenAI adapter path.
        // We exercise the wire-building logic (response_format / stop / reasoning_effort)
        // to confirm none of the Anthropic-only fields surface as errors.
        use crate::llm::adapter::ReasoningEffort;
        let req = CompletionRequest {
            cache_system_prompt: true,
            adaptive_thinking: true,
            reasoning_effort: Some(ReasoningEffort::Low),
            ..sample_request()
        };
        // ponytail: cache_system_prompt and adaptive_thinking are Anthropic-only; silently ignored.
        let reasoning_effort = req.reasoning_effort.map(|e| match e {
            ReasoningEffort::Low => "low",
            ReasoningEffort::Medium => "medium",
            ReasoningEffort::High => "high",
            // ponytail: OpenAI reasoning_effort does not accept "xhigh"/"max"; clamp to
            // "high" so callers (including Azure) setting XHigh/Max don't get a runtime error.
            ReasoningEffort::XHigh | ReasoningEffort::Max => "high",
        });
        let body = OaiRequest {
            reasoning_effort,
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        // Should not error and should NOT emit Anthropic-specific fields
        assert!(
            !json.contains("cache_control"),
            "cache_control must not appear in OAI body: {json}"
        );
        assert!(
            !json.contains("adaptive"),
            "adaptive thinking must not appear in OAI body: {json}"
        );
        assert!(
            json.contains("\"reasoning_effort\":\"low\""),
            "reasoning_effort low must serialize: {json}"
        );
    }

    #[test]
    fn stream_options_present_in_non_azure_mode() {
        // Non-azure path keeps include_usage=true for accurate billing telemetry.
        let body = OaiRequest {
            model: "gpt-4o".to_string(),
            messages: vec![],
            tools: None,
            temperature: None,
            top_p: None,
            max_tokens: Some(256),
            max_completion_tokens: None,
            stream: true,
            stream_options: Some(OaiStreamOptions {
                include_usage: true,
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(
            json.contains("\"stream_options\""),
            "non-azure mode must serialize stream_options: {json}"
        );
        assert!(
            json.contains("\"include_usage\":true"),
            "stream_options.include_usage must be true: {json}"
        );
    }

    // ── omit_sampling_params flag ────────────────────────────────────────────

    #[test]
    fn with_omit_sampling_params_builder_toggles_flag() {
        let adapter = make_adapter();
        assert!(!adapter.omit_sampling_params, "default is false");
        let adapter2 = adapter.with_omit_sampling_params(true);
        assert!(adapter2.omit_sampling_params, "flag set via builder");
        let adapter3 = adapter2.with_omit_sampling_params(false);
        assert!(!adapter3.omit_sampling_params, "flag cleared via builder");
    }

    #[test]
    fn omit_sampling_params_removes_temperature_top_p_stop_from_body() {
        // Simulate the adapter's body-building logic for a request that has all
        // three sampling params set. With the flag on, none must appear in the
        // serialised JSON; with it off, all three must be present.
        let req = CompletionRequest {
            temperature: Some(0.0),
            top_p: Some(0.9),
            stop_sequences: Some(vec!["<END>".to_string()]),
            ..sample_request()
        };

        // flag ON ─ temperature, top_p, stop must be absent entirely (not null).
        let (temperature, top_p, stop) = (None::<f32>, None::<f32>, None::<Vec<String>>);
        let body = OaiRequest {
            model: req.model.clone(),
            messages: vec![],
            temperature,
            top_p,
            max_tokens: Some(req.max_tokens),
            stop,
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(
            !json.contains("\"temperature\""),
            "temperature must be absent when omit_sampling_params=true: {json}"
        );
        assert!(
            !json.contains("\"top_p\""),
            "top_p must be absent when omit_sampling_params=true: {json}"
        );
        assert!(
            !json.contains("\"stop\""),
            "stop must be absent when omit_sampling_params=true: {json}"
        );

        // flag OFF ─ temperature, top_p, stop must be present.
        let (temperature, top_p, stop) = (req.temperature, req.top_p, req.stop_sequences.clone());
        let body = OaiRequest {
            model: req.model.clone(),
            messages: vec![],
            temperature,
            top_p,
            max_tokens: Some(req.max_tokens),
            stop,
            ..Default::default()
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(
            json.contains("\"temperature\""),
            "temperature must be present when omit_sampling_params=false: {json}"
        );
        assert!(
            json.contains("\"top_p\""),
            "top_p must be present when omit_sampling_params=false: {json}"
        );
        assert!(
            json.contains("\"stop\""),
            "stop must be present when omit_sampling_params=false: {json}"
        );
    }
}
