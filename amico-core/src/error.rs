//! Error types for the Amico library.
//!
//! This module defines the error types that can occur during task execution
//! and other operations in the Amico library. It uses the `thiserror` crate
//! to provide detailed error messages and proper error handling.
//!
//! # Examples
//!
//! ```
//! use amico_core::error::AmicoError;
//!
//! fn example_error() -> Result<(), AmicoError> {
//!     Err(AmicoError::TaskError(
//!         "my_task".to_string(),
//!         "connection failed".to_string(),
//!     ))
//! }
//! ```

use thiserror::Error;

/// Represents errors that can occur in the Amico library.
///
/// This enum implements the standard Error trait through thiserror's derive macro,
/// allowing it to be used with the standard Result type and error handling patterns.
#[derive(Debug, Error)]
pub enum AmicoError {
    /// Represents an unknown or unexpected error condition.
    #[error("Unknown error")]
    UnknownError,

    /// Represents an error that occurred during task execution.
    ///
    /// # Parameters
    ///
    /// * First String - The name of the task that failed
    /// * Second String - A description of the error that occurred
    #[error("Task {0} execution failed: {1}")]
    TaskError(String, String),
}
