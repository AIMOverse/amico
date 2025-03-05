use async_trait::async_trait;

use crate::ai::completion::CompletionRequest;
use crate::ai::errors::CompletionError;

/// Trait for providers of AI models.
#[async_trait]
pub trait Provider: Send + Sync {
    /// Completes a prompt with the provider.
    async fn completion(&self, request: &CompletionRequest)
        -> Result<ModelChoice, CompletionError>;
}

/// Result of a model choice.
pub enum ModelChoice {
    Message(String),
    // ToolCall(name, id, params)
    ToolCall(String, String, serde_json::Value),
}
