use async_trait::async_trait;

use self::errors::GenerationError;
use super::provider::Provider;

/// An executor executes a certain agentic task based on a command prompt
/// using a series of model provider calls.
#[async_trait]
pub trait Service: Send + Sync {
    async fn generate_text(
        &mut self,
        provider: &dyn Provider,
        prompt: String,
    ) -> Result<String, GenerationError>;
}

pub mod errors {
    use crate::ai::{provider, tool};

    #[derive(Debug, thiserror::Error)]
    pub enum GenerationError {
        #[error("Provider error")]
        ProviderError(#[from] provider::errors::CompletionError),
        #[error("Tool error")]
        ToolError(#[from] tool::errors::ToolCallError),
    }
}
