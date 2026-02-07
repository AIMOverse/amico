//! Tool loop agent workflow.
//!
//! A tool loop agent repeatedly:
//! 1. Receives input from the user
//! 2. Uses a language model to decide the next action
//! 3. Executes a tool if needed
//! 4. Observes the result
//! 5. Repeats until the task is complete or max iterations are reached

use crate::{AgentFinishReason, AgentResponse, WorkflowError};
use amico_runtime::{ExecutionContext, Workflow};
use std::marker::PhantomData;

/// Tool loop agent - repeatedly calls tools until goal is met.
///
/// This is the most common agent pattern, equivalent to Vercel AI SDK's
/// `generateText` with `maxSteps`.
pub struct ToolLoopAgent<M, T, C> {
    model: M,
    tools: T,
    max_iterations: usize,
    _context: PhantomData<C>,
}

impl<M, T, C> ToolLoopAgent<M, T, C> {
    pub fn new(model: M, tools: T, max_iterations: usize) -> Self {
        Self {
            model,
            tools,
            max_iterations,
            _context: PhantomData,
        }
    }
}

impl<M, T, C> Workflow for ToolLoopAgent<M, T, C>
where
    M: Send + Sync,
    T: Send + Sync,
    C: ExecutionContext + Send + Sync,
{
    type Context = C;
    type Input = String;
    type Output = AgentResponse;
    type Error = WorkflowError;

    async fn execute<'a>(
        &'a self,
        _context: &'a Self::Context,
        input: Self::Input,
    ) -> Result<Self::Output, Self::Error> {
        // TODO: implement real tool loop — iterate up to max_iterations,
        // call model via AgentAction::step, execute tool, collect observations.
        let _ = &self.model;
        let _ = &self.tools;
        let _ = self.max_iterations;

        Ok(AgentResponse {
            content: format!("Response to: {}", input),
            steps: vec![],
            finish_reason: AgentFinishReason::Success,
        })
    }
}
