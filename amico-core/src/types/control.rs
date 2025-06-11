use serde::{Deserialize, Serialize};

/// A control instruction to the Agent, e.g. quit.
///
/// TODO: Define more instructions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Control {
    /// Signal to quit the agent event loop
    Quit,
}
