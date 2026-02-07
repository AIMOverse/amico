//! Workflow trait - a unit of work that can be executed.

use std::future::Future;

/// Workflow trait - defines a unit of work that can be executed.
///
/// A workflow takes a context and an input, then produces an output
/// asynchronously. Workflows are the primary building block for
/// agent business logic.
pub trait Workflow {
    /// Context type for workflow execution
    type Context;

    /// Input type for the workflow
    type Input;

    /// Output type produced by the workflow
    type Output;

    /// Error type for workflow execution
    type Error;

    /// Execute the workflow with given context and input
    fn execute<'a>(
        &'a self,
        context: &'a Self::Context,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + 'a;
}
