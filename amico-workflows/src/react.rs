//! ReAct (Reasoning + Acting) workflow.
//!
//! Alternates between reasoning and acting:
//! 1. Reason about the current state
//! 2. Decide on an action
//! 3. Execute the action
//! 4. Observe the result
//! 5. Repeat

use crate::{AgentFinishReason, AgentResponse, WorkflowError};
use amico_runtime::Workflow;

/// ReAct (Reasoning + Acting) workflow.
pub struct ReActWorkflow<M, T> {
    model: M,
    tools: T,
    max_iterations: usize,
}

impl<M, T> ReActWorkflow<M, T> {
    pub fn new(model: M, tools: T, max_iterations: usize) -> Self {
        Self {
            model,
            tools,
            max_iterations,
        }
    }
}

impl<M, T> Workflow for ReActWorkflow<M, T>
where
    M: Send + Sync,
    T: Send + Sync,
{
    type Context = ();
    type Input = String;
    type Output = AgentResponse;
    type Error = WorkflowError;

    async fn execute<'a>(
        &'a self,
        _context: &'a Self::Context,
        input: Self::Input,
    ) -> Result<Self::Output, Self::Error> {
        // TODO: implement ReAct loop — alternate reasoning and acting.
        let _ = &self.model;
        let _ = &self.tools;
        let _ = self.max_iterations;

        Ok(AgentResponse {
            content: format!("ReAct response to: {}", input),
            steps: vec![],
            finish_reason: AgentFinishReason::Success,
        })
    }
}
