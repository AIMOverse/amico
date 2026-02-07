//! # Amico Runtime Layer
//!
//! This crate provides the runtime layer for executing workflows in Amico V2.
//! The runtime abstracts over different execution environments (long-lived vs short-lived).
//!
//! ## Design Principles
//!
//! - **Runtime agnostic**: Workflows can run on different runtime types
//! - **Lifecycle management**: Clear start/shutdown semantics
//! - **Task scheduling**: Unified scheduler interface
//! - **State management**: Context and state handling for workflows
//! - **Observable workflows**: Agent workflows expose a sequential step stream
//!   that any client can observe for streaming UIs and diagnostics.
//!
//! ## Agent Step Model
//!
//! An atomic agent action step takes a conversation history + model parameters
//! and produces an `AgentChoice` (tool call, text response, or finish).
//!
//! An agent workflow chains multiple steps, yielding observable `StepItem`s
//! via a `StepStream`. The runtime defines how the stream is driven.
//!
//! ## Example
//!
//! ```rust,ignore
//! use amico_runtime::{Workflow, Runtime, ExecutionContext};
//!
//! struct MyWorkflow;
//!
//! impl Workflow for MyWorkflow {
//!     type Context = MyContext;
//!     type Input = String;
//!     type Output = String;
//!     type Error = Error;
//!
//!     async fn execute(&self, context: &Self::Context, input: String) -> Result<String, Error> {
//!         Ok(format!("Processed: {}", input))
//!     }
//! }
//! ```

pub mod context;
pub mod message;
pub mod runtime;
pub mod scheduler;
pub mod step;
pub mod workflow;

// Re-export all public items for backward compatibility
pub use context::{ExecutionContext, SimpleContext};
pub use message::{Message, Role, ToolCall, ToolResult};
pub use runtime::{LongLivedRuntime, Runtime, RuntimeError, RuntimeSnapshot, ShortLivedRuntime};
pub use scheduler::{Scheduler, SchedulerError, TaskHandle};
pub use step::{AgentAction, AgentChoice, StepItem, StepStream};
pub use workflow::Workflow;
