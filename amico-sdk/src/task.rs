use async_trait::async_trait;

/// An AI task uses a service to perform some work.
#[async_trait]
pub trait Task {
    /// The error type returned by the task.
    type Error: std::error::Error;

    /// Run once before the task is executed.
    async fn before_run(&mut self) -> Result<(), Self::Error>;

    /// Perform some AI task work.
    async fn run(&mut self) -> Result<(), Self::Error>;

    /// Run once after the task is executed.
    async fn after_run(&mut self) -> Result<(), Self::Error>;
}
