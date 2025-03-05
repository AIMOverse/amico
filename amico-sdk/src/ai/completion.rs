use serde::{Deserialize, Serialize};

use super::{chat::Message, provider::Provider, service::ServiceContext, tool::ToolDefinition};

/// Chat completion request schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompletionRequest {
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

impl Default for CompletionRequest {
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

/// Builder for `CompletionRequest`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CompletionRequestBuilder {
    /// The inner builder
    inner: CompletionRequest,
}

impl CompletionRequestBuilder {
    /// Creates a new `CompletionRequestBuilder` with default values
    pub fn new() -> Self {
        Self {
            inner: CompletionRequest::default(),
        }
    }

    /// Creates a `CompletionRequestBuilder` from a `ServiceContext`.
    /// Convinient for building requests inside a service.
    pub fn from_ctx<P: Provider>(ctx: &ServiceContext<P>) -> Self {
        Self::new()
            .model(ctx.model.clone())
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
    pub fn chat_history(mut self, chat_history: Vec<Message>) -> Self {
        self.inner.chat_history = chat_history;
        self
    }

    /// Sets the tools
    pub fn tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.inner.tools = tools;
        self
    }

    /// Builds the `CompletionRequest`
    pub fn build(self) -> CompletionRequest {
        self.inner.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_request_builder() {
        // Test building the request
        let request = CompletionRequestBuilder::new()
            .prompt("Hello, world!".to_string())
            .model("test".to_string())
            .system_prompt("You are a helpful assistant.".to_string())
            .temperature(0.2)
            .max_tokens(100)
            .chat_history(Vec::new())
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
