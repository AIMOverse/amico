//! Agent step primitives and observable step streams.
//!
//! An atomic agent action step takes a conversation history and model
//! parameters, and produces an `AgentChoice` — for example a tool call,
//! a text response, or a finish signal.
//!
//! An agent workflow consists of multiple steps. To observers, the
//! workflow is an **observable sequential stream** of `StepItem`s.
//! Any client may attach to the stream to inspect progress, e.g. to
//! render a streaming UI.

use crate::message::{Message, ToolCall};
use std::future::Future;

/// The choice made by an agent in a single step.
#[derive(Debug, Clone)]
pub enum AgentChoice {
    /// The agent wants to invoke a tool.
    ToolCall(ToolCall),
    /// The agent produced a (partial) text response.
    TextResponse(String),
    /// The agent is done and produced a final answer.
    Finish(String),
}

/// An atomic agent action step.
///
/// Using a conversation history and model parameters, an `AgentAction`
/// produces a single `AgentChoice`. This is the smallest unit of agent
/// reasoning — analogous to a single `generateText` or `streamResponse`
/// call.
pub trait AgentAction {
    /// Model parameters type (temperature, max tokens, etc.)
    type Params;

    /// Error type for step execution
    type Error;

    /// Execute a single agent step.
    fn step<'a>(
        &'a self,
        messages: &'a [Message],
        params: &'a Self::Params,
    ) -> impl Future<Output = Result<AgentChoice, Self::Error>> + Send + 'a;
}

/// Items yielded by an observable workflow step stream.
///
/// Clients subscribe to a `StepStream` and receive these items in
/// sequential order. The whole stream is sequential — there is no
/// concurrent interleaving of steps.
#[derive(Debug, Clone)]
pub enum StepItem {
    /// A new reasoning step has started.
    StepStarted {
        /// Zero-based index of the step within the workflow run.
        step_index: usize,
    },
    /// The agent made a choice (tool call, text, or finish).
    Choice(AgentChoice),
    /// A streaming text chunk within the current step.
    TextDelta(String),
    /// The current step completed.
    StepCompleted {
        /// Zero-based index of the completed step.
        step_index: usize,
    },
    /// The entire workflow finished successfully.
    Finished(String),
}

/// An observable step stream for workflow execution.
///
/// Clients (or any observers) may attach to this stream to inspect
/// the agent's progress in real-time. The stream is sequential: items
/// are yielded one at a time in order.
///
/// The workflow runs on a `Runtime`. The runtime defines how the
/// stream is driven — for example, on a local tokio executor, on a
/// serverless cloud function, or on an embedded event loop.
pub trait StepStream {
    /// Error type
    type Error;

    /// Poll the next item from the step stream.
    ///
    /// Returns `None` when the stream is exhausted (workflow complete).
    fn poll_next<'a>(
        &'a mut self,
    ) -> impl Future<Output = Option<Result<StepItem, Self::Error>>> + Send + 'a;
}
