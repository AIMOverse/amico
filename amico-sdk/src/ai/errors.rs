/// Errors during creation of AI service
#[derive(Debug, thiserror::Error)]
pub enum CreationError {
    #[error("Invalid param")]
    InvalidParam,
}

/// Errors during completion of chatting
#[derive(Debug, thiserror::Error)]
pub enum CompletionError {
    #[error("API error")]
    ApiError,

    #[error("Model {0} is unavailable")]
    ModelUnavailable(String),
}

/// Errors during tool call
#[derive(Debug, thiserror::Error)]
pub enum ToolCallError {
    #[error("Tool {0} is unavailable")]
    ToolUnavailable(String),

    #[error("Invalid param {name}: {value} ({reason})")]
    InvalidParam {
        name: String,
        value: serde_json::Value,
        reason: String,
    },
}

/// Errors during service call
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Provider error")]
    ProviderError(#[from] CompletionError),
    #[error("Tool error")]
    ToolError(#[from] ToolCallError),
}
