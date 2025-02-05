use std::fmt::Debug;

/// Errors that can occur during executing actions.
#[derive(thiserror::Error, Debug)]
pub enum ActionError {
    /// Error when executing actions
    #[error("Executing Action Error: {0}")]
    ExecutingActionError(String),
}
