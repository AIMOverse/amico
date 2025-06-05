use serde::{Deserialize, Serialize};

/// An instruction to the Agent, e.g. quit.
///
/// TODO: Define more instructions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Instruction {
    /// Signal to terminate the agent event loop
    Terminate,
}
