use self::errors::*;
use crate::ai::model::{ChatResponse, Message};
use crate::ai::tool::ToolSet;
use async_trait::async_trait;

/// Trait for providers of AI models.
#[async_trait]
pub trait LLMProvider {
    /// Creates a new provider.
    fn new(
        base_url: &str,
        api_key: Option<&str>,
        toolset: Option<&ToolSet>,
    ) -> Result<Self, CreationError>
    where
        Self: Sized;

    /// Completes a prompt with the provider.
    async fn completion(
        &self,
        model: String,
        messages: Vec<Message>,
    ) -> Result<ChatResponse, CompletionError>;
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
        #[error("LLM API error")]
        ApiError,
    }
}
