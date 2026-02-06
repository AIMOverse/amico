//! OpenAI-compatible chat model implementation.
//!
//! Implements `amico_models::ChatModel` and `amico_models::StreamingChatModel`
//! by calling any OpenAI-compatible `/v1/chat/completions` endpoint.

use amico_models::{
    ChatInput, ChatModel, ChatRole, FinishReason, LanguageOutput, Model, StreamChunk,
    StreamingChatModel, TokenUsage,
};
use futures::stream::{self, BoxStream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::future::Future;

/// An OpenAI-compatible chat model backed by an HTTP API.
pub struct OpenAiChatModel {
    client: Client,
    api_base: String,
    api_key: String,
    model: String,
}

impl OpenAiChatModel {
    pub fn new(api_base: &str, api_key: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            api_base: api_base.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
        }
    }
}

// -- OpenAI API request/response types --

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ApiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stream: bool,
}

#[derive(Serialize)]
struct ApiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
    usage: Option<ApiUsage>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ApiChoiceMessage,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct ApiChoiceMessage {
    content: Option<String>,
}

#[derive(Deserialize)]
struct ApiUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

// Streaming response types
#[derive(Deserialize)]
struct StreamChatCompletionChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct StreamDelta {
    content: Option<String>,
}

// -- helpers --

fn role_to_string(role: &ChatRole) -> &'static str {
    match role {
        ChatRole::System => "system",
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
        ChatRole::Tool => "tool",
    }
}

fn parse_finish_reason(s: Option<&str>) -> FinishReason {
    match s {
        Some("stop") => FinishReason::Stop,
        Some("length") => FinishReason::Length,
        Some("content_filter") => FinishReason::ContentFilter,
        Some("tool_calls") => FinishReason::ToolCalls,
        _ => FinishReason::Stop,
    }
}

fn build_api_messages(input: &ChatInput) -> Vec<ApiMessage> {
    input
        .messages
        .iter()
        .map(|m| ApiMessage {
            role: role_to_string(&m.role).to_string(),
            content: m.content.clone(),
        })
        .collect()
}

// -- Trait implementations --

/// Error type for the OpenAI model.
#[derive(Debug)]
pub struct OpenAiModelError(pub String);

impl std::fmt::Display for OpenAiModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpenAI model error: {}", self.0)
    }
}

impl std::error::Error for OpenAiModelError {}

/// The model does not require an external context — configuration lives
/// in the struct itself.
impl Model for OpenAiChatModel {
    type Context = ();
    type Input = ChatInput;
    type Output = LanguageOutput;
    type Error = OpenAiModelError;

    fn execute<'a>(
        &'a self,
        _context: &'a Self::Context,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + 'a {
        async move {
            let body = ChatCompletionRequest {
                model: self.model.clone(),
                messages: build_api_messages(&input),
                max_tokens: input.max_tokens,
                temperature: input.temperature,
                stream: false,
            };

            let resp = self
                .client
                .post(format!("{}/chat/completions", self.api_base))
                .bearer_auth(&self.api_key)
                .json(&body)
                .send()
                .await
                .map_err(|e| OpenAiModelError(e.to_string()))?;

            let status = resp.status();
            if !status.is_success() {
                let text = resp.text().await.unwrap_or_default();
                return Err(OpenAiModelError(format!(
                    "API returned {status}: {text}"
                )));
            }

            let data: ChatCompletionResponse = resp
                .json()
                .await
                .map_err(|e| OpenAiModelError(e.to_string()))?;

            let choice = data
                .choices
                .first()
                .ok_or_else(|| OpenAiModelError("No choices in response".into()))?;

            let usage = data.usage.as_ref();

            Ok(LanguageOutput {
                text: choice.message.content.clone().unwrap_or_default(),
                finish_reason: parse_finish_reason(choice.finish_reason.as_deref()),
                usage: TokenUsage {
                    prompt_tokens: usage.map_or(0, |u| u.prompt_tokens),
                    completion_tokens: usage.map_or(0, |u| u.completion_tokens),
                    total_tokens: usage.map_or(0, |u| u.total_tokens),
                },
            })
        }
    }
}

impl ChatModel for OpenAiChatModel {}

impl StreamingChatModel for OpenAiChatModel {
    type TokenStream = BoxStream<'static, Result<StreamChunk, OpenAiModelError>>;

    fn stream<'a>(
        &'a self,
        _context: &'a Self::Context,
        input: ChatInput,
    ) -> impl Future<Output = Result<Self::TokenStream, Self::Error>> + Send + 'a {
        async move {
            let body = ChatCompletionRequest {
                model: self.model.clone(),
                messages: build_api_messages(&input),
                max_tokens: input.max_tokens,
                temperature: input.temperature,
                stream: true,
            };

            let resp = self
                .client
                .post(format!("{}/chat/completions", self.api_base))
                .bearer_auth(&self.api_key)
                .json(&body)
                .send()
                .await
                .map_err(|e| OpenAiModelError(e.to_string()))?;

            let status = resp.status();
            if !status.is_success() {
                let text = resp.text().await.unwrap_or_default();
                return Err(OpenAiModelError(format!(
                    "API returned {status}: {text}"
                )));
            }

            // Read the SSE byte stream and parse into StreamChunks
            let byte_stream = resp.bytes_stream();

            let chunk_stream = byte_stream
                .map(|result| match result {
                    Err(e) => {
                        stream::iter(vec![Err(OpenAiModelError(e.to_string()))])
                    }
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        let chunks: Vec<Result<StreamChunk, OpenAiModelError>> = text
                            .lines()
                            .filter_map(|line| {
                                let line = line.strip_prefix("data: ")?;
                                if line == "[DONE]" {
                                    return Some(Ok(StreamChunk {
                                        delta: String::new(),
                                        done: true,
                                    }));
                                }
                                let parsed: StreamChatCompletionChunk =
                                    serde_json::from_str(line).ok()?;
                                let choice = parsed.choices.first()?;
                                let delta = choice.delta.content.clone().unwrap_or_default();
                                let done = choice.finish_reason.is_some();
                                Some(Ok(StreamChunk { delta, done }))
                            })
                            .collect();
                        stream::iter(chunks)
                    }
                })
                .flatten()
                .boxed();

            Ok(chunk_stream)
        }
    }
}
