use async_trait::async_trait;

use crate::ai::service::Service;

/// A context that provides access to the service.
pub trait TaskContext {
    /// Get the service.
    fn service(&self) -> &impl Service
    where
        Self: Sized;
}

/// An AI task uses a service to perform some work.
#[async_trait]
pub trait Task {
    /// The context type used by the task.
    type Context: TaskContext;

    /// The error type returned by the task.
    type Error: std::error::Error;

    /// Perform some AI task work.
    async fn run(&mut self, context: &Self::Context) -> Result<(), Self::Error>;
}
