use async_trait::async_trait;

use self::errors::ExecutorError;
use super::{provider::LLMProvider, tool::ToolSet};

/// An executor executes a certain agentic task based on a command prompt
/// using a series of model provider calls.
#[async_trait]
pub trait Executor {
    fn new(
        system_prompt: String,
        model: String,
        provider: Box<dyn LLMProvider>,
        tools: ToolSet,
    ) -> Self
    where
        Self: Sized;

    async fn execute(&mut self, prompt: String) -> Result<String, ExecutorError>;
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
