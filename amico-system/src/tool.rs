//! Tool abstraction for agent capabilities.

use std::future::Future;

/// Core tool trait - all tools implement this.
///
/// Tools are the primary way agents interact with the real world.
/// Each tool has typed input/output and provides metadata for the
/// language model to understand when and how to use it.
pub trait Tool {
    /// Input type for the tool
    type Input;

    /// Output type produced by the tool
    type Output;

    /// Error type for tool execution
    type Error;

    /// Execute the tool with given input
    fn execute<'a>(
        &'a self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + 'a;

    /// Tool name (used for identification)
    fn name(&self) -> &str;

    /// Human-readable description of what the tool does
    fn description(&self) -> &str;

    /// JSON schema for the tool's input (optional)
    fn input_schema(&self) -> Option<&str> {
        None
    }
}
