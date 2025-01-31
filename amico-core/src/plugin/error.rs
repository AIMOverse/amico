use std::fmt::Debug;

#[derive(thiserror::Error, Debug)]
pub enum PluginError {
    #[error("Invalid config format")]
    InvalidConfigFormat,

    #[error("Invalid data format")]
    InvalidDataFormat,

    #[error("Execution error: {0}")]
    ExecutionError(String),
}
