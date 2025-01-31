use std::fmt::Debug;

/// Errors that can occur while working with plugins.
#[derive(thiserror::Error, Debug)]
pub enum PluginError {
    /// Error when the config format is invalid.
    #[error("Invalid config format")]
    InvalidConfigFormat,

    /// Error when the data format is invalid.
    #[error("Invalid data format")]
    InvalidDataFormat,

    /// Error when an execution error occurs.
    #[error("Execution error: {0}")]
    ExecutionError(String),
}
