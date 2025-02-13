use crate::ai::errors::{CompletionError, CreationError};
use crate::ai::model::{ChatResponse, Message};
use crate::ai::tool::ToolSet;
use async_trait::async_trait;

/// Trait for providers of AI models.
/// This trait should be used in AI services.
#[async_trait]
pub trait Provider {
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
