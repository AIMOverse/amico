use serde::{Deserialize, Serialize};

/// An instruction to the Agent, e.g. quit.
///
/// TODO: Implement the instruction feature.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentInstruction {
    /// Signal to terminate the agent event loop
    Terminate,
}
