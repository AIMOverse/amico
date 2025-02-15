use std::fmt::Debug;

/// Errors that can occur during executing actions.
#[derive(thiserror::Error, Debug)]
pub enum ActionError {
    /// Error when executing actions
    #[error("Executing Action Error: {0}")]
    ExecutingActionError(String),

    /// Error when missing required parameters
    #[error("Missing required parameters: {0}")]
    MissingRequiredParameters(String),

    /// Error when invalid parameter type
    #[error("Invalid parameter type: {0}, expected: {1}")]
    InvalidParameterType(String, String),
}
