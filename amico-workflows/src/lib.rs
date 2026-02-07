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

pub mod chain;
pub mod react;
pub mod reflection;
pub mod tool_loop;

use amico_runtime::Workflow;
use std::future::Future;

// Re-export all workflow types
pub use chain::ChainOfThought;
pub use react::ReActWorkflow;
pub use reflection::ReflectionWorkflow;
pub use tool_loop::ToolLoopAgent;

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

/// Thought step in chain of thought
#[derive(Debug, Clone)]
pub struct ThoughtStep {
    pub description: String,
    pub reasoning: String,
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
