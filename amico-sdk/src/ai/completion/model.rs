use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::ai::{
    completion::{Error, SessionContext},
    message::Message,
    tool::ToolDefinition,
};

/// Trait for completion models.
pub trait Model {
    /// Completes a prompt with the provider.
    fn completion(
        &self,
        request: &Request,
    ) -> impl Future<Output = Result<ModelChoice, Error>> + Send;
}

#[async_trait]
pub trait ModelDyn {
    async fn completion_dyn(&self, request: &Request) -> Result<ModelChoice, Error>;
}

#[async_trait(?Send)]
pub trait ModelLocal {
    async fn completion_local(&self, request: &Request) -> Result<ModelChoice, Error>;
}

#[async_trait]
impl<T: Model + Sync> ModelDyn for T {
    async fn completion_dyn(&self, request: &Request) -> Result<ModelChoice, Error> {
        self.completion(request).await
    }
}

#[async_trait(?Send)]
impl<T: Model> ModelLocal for T {
    async fn completion_local(&self, request: &Request) -> Result<ModelChoice, Error> {
        self.completion(request).await
    }
}

/// Result of a model choice.
pub enum ModelChoice {
    Message(String),
    // ToolCall(name, id, params)
    ToolCall(String, String, serde_json::Value),
}

/// Chat completion request schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Request {
    /// The prompt to complete
    pub prompt: String,
    /// The model's name to use
    pub model: String,
    /// The system prompt to use
    pub system_prompt: Option<String>,
    /// The temperature to use
    pub temperature: Option<f64>,
    /// The maximum number of tokens to generate
    pub max_tokens: Option<u64>,
    /// The chat history
    pub chat_history: Vec<Message>,
    /// The tools to call
    pub tools: Vec<ToolDefinition>,
}

impl Default for Request {
    /// Creates a default `CompletionRequest` with empty fields
    fn default() -> Self {
        Self {
            prompt: String::new(),
            model: String::new(),
            system_prompt: None,
            temperature: None,
            max_tokens: None,
            chat_history: Vec::new(),
            tools: Vec::new(),
        }
    }
}

/// Builder for `Request`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RequestBuilder {
    /// The inner builder
    inner: Request,
}

impl RequestBuilder {
    /// Creates a new `CompletionRequestBuilder` with default values
    pub fn new() -> Self {
        Self {
            inner: Request::default(),
        }
    }

    /// Creates a `CompletionRequestBuilder` from a `ServiceContext`.
    /// Convinient for building requests inside a service.
    pub fn from_ctx<P: Model>(ctx: &SessionContext<P>) -> Self {
        Self::new()
            .model(ctx.model_name.clone())
            .system_prompt(ctx.system_prompt.clone())
            .tools(ctx.tools.iter_defs().cloned().collect())
            .temperature(ctx.temperature)
            .max_tokens(ctx.max_tokens)
    }

    /// Sets the prompt
    pub fn prompt(mut self, prompt: String) -> Self {
        self.inner.prompt = prompt;
        self
    }

    /// Sets the model
    pub fn model(mut self, model: String) -> Self {
        self.inner.model = model;
        self
    }

    /// Sets the system prompt
    pub fn system_prompt(mut self, system_prompt: String) -> Self {
        self.inner.system_prompt = Some(system_prompt);
        self
    }

    /// Sets the temperature
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.inner.temperature = Some(temperature);
        self
    }

    /// Sets the max tokens
    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.inner.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the chat history
    pub fn history(mut self, chat_history: Vec<Message>) -> Self {
        self.inner.chat_history = chat_history;
        self
    }

    /// Sets the tools
    pub fn tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.inner.tools = tools;
        self
    }

    /// Builds the `CompletionRequest`
    pub fn build(self) -> Request {
        self.inner.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_request_builder() {
        // Test building the request
        let request = RequestBuilder::new()
            .prompt("Hello, world!".to_string())
            .model("test".to_string())
            .system_prompt("You are a helpful assistant.".to_string())
            .temperature(0.2)
            .max_tokens(100)
            .history(Vec::new())
            .tools(Vec::new())
            .build();

        assert_eq!(request.prompt, "Hello, world!");
        assert_eq!(request.model, "test".to_string());
        assert_eq!(
            request.system_prompt,
            Some("You are a helpful assistant.".to_string())
        );
        assert_eq!(request.temperature, Some(0.2));
        assert_eq!(request.max_tokens, Some(100));
        assert_eq!(request.chat_history, Vec::new());
        assert_eq!(request.tools, Vec::new());
    }
}
