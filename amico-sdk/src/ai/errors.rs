// Error during the creation of a new AI Provider
#[derive(Debug, thiserror::Error)]
pub enum CreationError {
    #[error("Invalid param")]
    InvalidParam,
}

// Error during the completion of a prompt in a provider
#[derive(Debug, thiserror::Error)]
pub enum CompletionError {
    #[error("LLM API error")]
    ApiError,
}

// Error during a tool call
#[derive(Debug, thiserror::Error)]
pub enum ToolCallError {
    #[error("Invalid param {name}: {value} ({reason})")]
    InvalidParam {
        name: String,
        value: serde_json::Value,
        reason: String,
    },
}

// Error during a service call
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Provider error")]
    ProviderError(#[from] CompletionError),
    #[error("Tool error")]
    ToolError(#[from] ToolCallError),
}
