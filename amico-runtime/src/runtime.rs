//! Runtime abstraction and lifecycle.

use crate::context::ExecutionContext;
use crate::scheduler::Scheduler;
use std::future::Future;

/// Runtime error types
#[derive(Debug)]
pub enum RuntimeError {
    StartupFailed(String),
    ShutdownFailed(String),
    WorkflowExecutionFailed(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartupFailed(msg) => write!(f, "Runtime startup failed: {}", msg),
            Self::ShutdownFailed(msg) => write!(f, "Runtime shutdown failed: {}", msg),
            Self::WorkflowExecutionFailed(msg) => write!(f, "Workflow execution failed: {}", msg),
        }
    }
}

impl std::error::Error for RuntimeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_error_display() {
        let err = RuntimeError::StartupFailed("port in use".to_string());
        assert_eq!(err.to_string(), "Runtime startup failed: port in use");

        let err = RuntimeError::ShutdownFailed("timeout".to_string());
        assert_eq!(err.to_string(), "Runtime shutdown failed: timeout");

        let err = RuntimeError::WorkflowExecutionFailed("model error".to_string());
        assert_eq!(
            err.to_string(),
            "Workflow execution failed: model error"
        );
    }

    #[test]
    fn test_runtime_snapshot() {
        let snap = RuntimeSnapshot {
            state_data: vec![1, 2, 3],
            timestamp: 1234567890,
        };
        assert_eq!(snap.state_data, vec![1, 2, 3]);
        assert_eq!(snap.timestamp, 1234567890);
    }
}

/// Runtime abstraction.
///
/// A `Runtime` owns an execution context and a scheduler.  It manages
/// the lifecycle (start / shutdown) of the agent program.
pub trait Runtime {
    /// Execution context type
    type Context: ExecutionContext;

    /// Scheduler type
    type Scheduler: Scheduler;

    /// Get execution context
    fn context(&self) -> &Self::Context;

    /// Get mutable execution context
    fn context_mut(&mut self) -> &mut Self::Context;

    /// Get task scheduler
    fn scheduler(&self) -> &Self::Scheduler;

    /// Start the runtime
    fn start(&mut self) -> impl Future<Output = Result<(), RuntimeError>> + Send;

    /// Shutdown the runtime
    fn shutdown(&mut self) -> impl Future<Output = Result<(), RuntimeError>> + Send;
}

/// Long-lived runtime (e.g., OS processes, Cloudflare Workers).
///
/// Runtime persists across multiple workflow executions.
pub trait LongLivedRuntime: Runtime {}

/// Runtime snapshot for state persistence
#[derive(Debug, Clone)]
pub struct RuntimeSnapshot {
    pub state_data: Vec<u8>,
    pub timestamp: u64,
}

/// Short-lived runtime (e.g., cloud functions, serverless).
///
/// Runtime exists for single workflow execution with state restoration.
pub trait ShortLivedRuntime: Runtime {
    /// Create a snapshot of the current runtime state
    fn snapshot(&self) -> RuntimeSnapshot;

    /// Restore runtime from a snapshot
    fn restore(snapshot: RuntimeSnapshot) -> Self;
}
