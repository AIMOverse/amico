//! Task scheduler trait.

use std::future::Future;

/// Scheduler error types
#[derive(Debug)]
pub enum SchedulerError {
    TaskSchedulingFailed(String),
    TaskCancellationFailed(String),
}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TaskSchedulingFailed(msg) => write!(f, "Task scheduling failed: {}", msg),
            Self::TaskCancellationFailed(msg) => write!(f, "Task cancellation failed: {}", msg),
        }
    }
}

impl std::error::Error for SchedulerError {}

/// Task handle for tracking scheduled tasks
pub struct TaskHandle {
    id: u64,
}

impl TaskHandle {
    pub fn new(id: u64) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

/// Task scheduler trait.
///
/// A scheduler accepts tasks and arranges their execution. The
/// concrete implementation may use tokio, embassy, or any other
/// async executor.
pub trait Scheduler {
    /// Task type that can be scheduled
    type Task;

    /// Schedule a task for execution
    fn schedule<'a>(
        &'a self,
        task: Self::Task,
    ) -> impl Future<Output = Result<TaskHandle, SchedulerError>> + Send + 'a;

    /// Cancel a scheduled task
    fn cancel(&self, handle: TaskHandle) -> Result<(), SchedulerError>;
}
