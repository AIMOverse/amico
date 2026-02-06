//! # Amico Workflows Layer
//!
//! This crate provides preset workflow patterns for Amico V2, including:
//! - Tool loop agents
//! - Chain of thought workflows
//! - ReAct (Reasoning + Acting) workflows
//! - Reflection workflows
//! - Multi-agent coordination patterns
//!
//! ## Design Principles
//!
//! - **Preset patterns**: Common agent patterns ready to use
//! - **Composable**: Workflows can be composed to create complex behaviors
//! - **Generic**: Works with any model/tool implementations
//!
//! ## Example
//!
//! ```rust,ignore
//! use amico_workflows::ToolLoopAgent;
//! use amico_models::LanguageModel;
//! use amico_runtime::Workflow;
//!
//! let agent = ToolLoopAgent::new(model, tools, 10);
//! let result = agent.execute(&context, "What is 2+2?".to_string()).await?;
//! ```

use amico_runtime::{Workflow, ExecutionContext};
use std::marker::PhantomData;
use std::future::Future;

/// Agent response
#[derive(Debug, Clone)]
pub struct AgentResponse {
    pub content: String,
    pub steps: Vec<AgentStep>,
    pub finish_reason: AgentFinishReason,
}

/// Individual step in agent reasoning
#[derive(Debug, Clone)]
pub struct AgentStep {
    pub thought: String,
    pub action: Option<String>,
    pub observation: Option<String>,
}

/// Reason why agent finished
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentFinishReason {
    Success,
    MaxIterations,
    Error,
}

/// Workflow error
#[derive(Debug)]
pub enum WorkflowError {
    ModelError(String),
    ToolError(String),
    MaxIterationsReached,
    Other(String),
}

impl std::fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ModelError(msg) => write!(f, "Model error: {}", msg),
            Self::ToolError(msg) => write!(f, "Tool error: {}", msg),
            Self::MaxIterationsReached => write!(f, "Maximum iterations reached"),
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for WorkflowError {}

/// Tool registry trait
pub trait ToolRegistry {
    type Tool;
    type ToolName;
    
    fn get_tool(&self, name: &Self::ToolName) -> Option<&Self::Tool>;
    fn list_tools(&self) -> Vec<&Self::ToolName>;
}

/// Tool loop agent - repeatedly calls tools until goal is met
///
/// This workflow:
/// 1. Receives input from user
/// 2. Uses language model to decide next action
/// 3. Executes tool if needed
/// 4. Observes result
/// 5. Repeats until task is complete or max iterations reached
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
        // Placeholder implementation
        // In a real implementation, this would:
        // 1. Loop up to max_iterations
        // 2. Call model to decide action
        // 3. Execute tool if needed
        // 4. Collect observations
        // 5. Return when goal is met
        
        Ok(AgentResponse {
            content: format!("Response to: {}", input),
            steps: vec![],
            finish_reason: AgentFinishReason::Success,
        })
    }
}

/// Thought step in chain of thought
#[derive(Debug, Clone)]
pub struct ThoughtStep {
    pub description: String,
    pub reasoning: String,
}

/// Chain of thought workflow
///
/// This workflow breaks down complex problems into steps:
/// 1. Decompose problem into sub-problems
/// 2. Solve each sub-problem sequentially
/// 3. Combine results
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
        // Placeholder implementation
        Ok(AgentResponse {
            content: format!("Chain of thought response to: {}", input),
            steps: vec![],
            finish_reason: AgentFinishReason::Success,
        })
    }
}

/// ReAct (Reasoning + Acting) workflow
///
/// This workflow alternates between reasoning and acting:
/// 1. Reason about the current state
/// 2. Decide on an action
/// 3. Execute the action
/// 4. Observe the result
/// 5. Repeat
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
        // Placeholder implementation
        Ok(AgentResponse {
            content: format!("ReAct response to: {}", input),
            steps: vec![],
            finish_reason: AgentFinishReason::Success,
        })
    }
}

/// Reflection workflow
///
/// This workflow uses self-critique to improve outputs:
/// 1. Generate initial response
/// 2. Critique the response
/// 3. Refine based on critique
/// 4. Repeat until satisfactory
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
        // Placeholder implementation
        Ok(AgentResponse {
            content: format!("Reflection response to: {}", input),
            steps: vec![],
            finish_reason: AgentFinishReason::Success,
        })
    }
}

/// Multi-agent coordination strategy
pub enum CoordinationStrategy {
    /// Broadcast to all agents and aggregate responses
    Broadcast,
    /// Agents debate to reach consensus
    Debate,
    /// Sequential chain of agents
    Chain,
}

/// Multi-agent workflow
///
/// Coordinates multiple agents to solve complex tasks
pub trait MultiAgentWorkflow {
    type Agent: Workflow;
    type Coordination;
    
    fn agents(&self) -> &[Self::Agent];
    
    fn coordinate<'a>(
        &'a self,
        responses: Vec<AgentResponse>,
    ) -> impl Future<Output = Self::Coordination> + Send + 'a;
}
