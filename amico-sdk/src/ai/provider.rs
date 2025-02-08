use async_trait::async_trait;

use self::errors::*;
use super::chat::ChatHistory;

/// Trait for providers of AI models.
#[async_trait]
pub trait Provider {
    /// Creates a new provider.
    fn new(base_url: Option<&str>, api_key: Option<&str>) -> Result<Self, CreationError>
    where
        Self: Sized;

    /// Completes a prompt with the provider.
    async fn completion(
        &self,
        model: String,
        prompt: String,
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

/// Errors related to AI providers.
pub mod errors {
    #[derive(Debug, thiserror::Error)]
    pub enum CreationError {
        #[error("Invalid param")]
        InvalidParam,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum CompletionError {
        #[error("API error")]
        ApiError,
    }
}
