use std::fmt::Debug;

/// Errors that can occur in action selector.
#[derive(thiserror::Error, Debug)]
pub enum ActionSelectorError {
    /// Error when selecting action.
    #[error("Selecting action failed: {0}")]
    SelectingActionFailed(String),
}
