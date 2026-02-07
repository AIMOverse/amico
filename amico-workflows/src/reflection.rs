//! Reflection workflow.
//!
//! Uses self-critique to improve outputs:
//! 1. Generate initial response
//! 2. Critique the response
//! 3. Refine based on critique
//! 4. Repeat until satisfactory

use crate::{AgentFinishReason, AgentResponse, WorkflowError};
use amico_runtime::Workflow;

/// Reflection workflow.
pub struct ReflectionWorkflow<M> {
    model: M,
    critic: M,
    max_refinements: usize,
}

impl<M> ReflectionWorkflow<M> {
    pub fn new(model: M, critic: M, max_refinements: usize) -> Self {
        Self {
            model,
            critic,
            max_refinements,
        }
    }
}

impl<M> Workflow for ReflectionWorkflow<M>
where
    M: Send + Sync,
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
        // TODO: implement reflection loop — generate, critique, refine.
        let _ = &self.model;
        let _ = &self.critic;
        let _ = self.max_refinements;

        Ok(AgentResponse {
            content: format!("Reflection response to: {}", input),
            steps: vec![],
            finish_reason: AgentFinishReason::Success,
        })
    }
}
