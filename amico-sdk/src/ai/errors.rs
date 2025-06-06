/// Errors during creation of AI Service
#[derive(Debug, thiserror::Error)]
pub enum CreationError {
    #[error("Invalid API key")]
    InvalidParam {
        name: String,
        value: serde_json::Value,
        reason: String,
    },
}

/// Errors during tool call
#[derive(Debug, Clone, thiserror::Error)]
pub enum ToolCallError {
    #[error("Tool {0} is unavailable")]
    ToolUnavailable(String),

    #[error("Invalid param {name} with value {value} for reason {reason}")]
    InvalidParam {
        name: String,
        value: serde_json::Value,
        reason: String,
    },

    #[error("Error executing {tool_name} with params {params} for reason {reason}")]
    ExecutionError {
        tool_name: String,
        params: serde_json::Value,
        reason: String,
    },
}
