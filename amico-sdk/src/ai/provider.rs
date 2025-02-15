use async_trait::async_trait;

use crate::ai::chat::ChatHistory;
use crate::ai::errors::{CompletionError, CreationError};

/// Trait for providers of AI models.
#[async_trait]
pub trait Provider: Send + Sync {
    /// Creates a new provider.
    fn new(base_url: Option<&str>, api_key: Option<&str>) -> Result<Self, CreationError>
    where
        Self: Sized;

    /// Completes a prompt with the provider.
    async fn completion(
        &self,
        prompt: &str,
        config: &CompletionConfig,
        chat_history: &ChatHistory,
    ) -> Result<ModelChoice, CompletionError>;

    /// Checks if a model name is available.
    async fn model_available(&self, model: &str) -> bool;
}

/// Result of a model choice.
pub enum ModelChoice {
    Message(String),
    ToolCall(String, serde_json::Value),
}

/// Configuration for the completion of a prompt.
#[derive(Debug, Clone, PartialEq)]
pub struct CompletionConfig {
    pub system_prompt: String,
    pub temperature: f64,
    pub max_tokens: u64,
    pub model: String,
}
