//! Chain of thought workflow.
//!
//! Breaks down complex problems into sequential reasoning steps:
//! 1. Decompose problem into sub-problems
//! 2. Solve each sub-problem sequentially
//! 3. Combine results

use crate::{AgentFinishReason, AgentResponse, ThoughtStep, WorkflowError};
use amico_runtime::Workflow;

/// Chain of thought workflow.
pub struct ChainOfThought<M> {
    model: M,
    steps: Vec<ThoughtStep>,
}

impl<M> ChainOfThought<M> {
    pub fn new(model: M, steps: Vec<ThoughtStep>) -> Self {
        Self { model, steps }
    }
}

impl<M> Workflow for ChainOfThought<M>
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
        // TODO: implement chain-of-thought — walk through steps sequentially.
        let _ = &self.model;
        let _ = &self.steps;

        Ok(AgentResponse {
            content: format!("Chain of thought response to: {}", input),
            steps: vec![],
            finish_reason: AgentFinishReason::Success,
        })
    }
}
