use async_trait::async_trait;

use self::errors::ExecutorError;
use super::{provider::AIProvider, tool::ToolSet};

/// The trait for AI services.
/// using a series of model provider calls.
#[async_trait]
pub trait AIService {
    fn new(
        system_prompt: String,
        model: String,
        provider: Box<dyn AIProvider>,
        tools: ToolSet,
    ) -> Self
    where
        Self: Sized;

    async fn get_response(&mut self, prompt: String) -> Result<String, ExecutorError>;
}

pub mod errors {
    use crate::ai::{provider, tool};

    #[derive(Debug, thiserror::Error)]
    pub enum CreationError {
        #[error("Provider error")]
        ProviderError(#[from] provider::errors::CreationError),
    }

    #[derive(Debug, thiserror::Error)]
    pub enum ExecutorError {
        #[error("Provider error")]
        ProviderError(#[from] provider::errors::CompletionError),
        #[error("Tool error")]
        ToolError(#[from] tool::errors::ToolCallError),
    }
}
