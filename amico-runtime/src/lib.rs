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

use std::future::Future;

/// Workflow trait - defines a unit of work that can be executed
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

/// Execution context for workflows
pub trait ExecutionContext {
    /// State type managed by the context
    type State;
    
    /// Permission type for resource access
    type Permissions;
    
    /// Get immutable reference to state
    fn state(&self) -> &Self::State;
    
    /// Get mutable reference to state
    fn state_mut(&mut self) -> &mut Self::State;
    
    /// Get permissions
    fn permissions(&self) -> &Self::Permissions;
}

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

/// Runtime abstraction
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

/// Task scheduler trait
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

/// Long-lived runtime (e.g., OS processes, Cloudflare Workers)
/// Runtime persists across multiple workflow executions
pub trait LongLivedRuntime: Runtime {}

/// Runtime snapshot for state persistence
#[derive(Debug, Clone)]
pub struct RuntimeSnapshot {
    pub state_data: Vec<u8>,
    pub timestamp: u64,
}

/// Short-lived runtime (e.g., cloud functions, serverless)
/// Runtime exists for single workflow execution with state restoration
pub trait ShortLivedRuntime: Runtime {
    /// Create a snapshot of the current runtime state
    fn snapshot(&self) -> RuntimeSnapshot;
    
    /// Restore runtime from a snapshot
    fn restore(snapshot: RuntimeSnapshot) -> Self;
}

/// Simple execution context implementation
#[derive(Debug)]
pub struct SimpleContext<S, P> {
    state: S,
    permissions: P,
}

impl<S, P> SimpleContext<S, P> {
    pub fn new(state: S, permissions: P) -> Self {
        Self { state, permissions }
    }
}

impl<S, P> ExecutionContext for SimpleContext<S, P> {
    type State = S;
    type Permissions = P;
    
    fn state(&self) -> &Self::State {
        &self.state
    }
    
    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }
    
    fn permissions(&self) -> &Self::Permissions {
        &self.permissions
    }
}
