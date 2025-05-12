use serde::{Deserialize, Serialize};

/// An instruction to the Agent, e.g. quit.
///
/// TODO: Implement the instruction feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentInstruction {}
