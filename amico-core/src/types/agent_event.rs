use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::AgentInstruction;

/// Struct representing an event the agent receives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    /// The ID of the event.
    pub id: u32,

    /// The name of the event.
    pub name: &'static str,

    /// The event source information of the event.
    pub source: &'static str,

    /// The parameters of the event, stored as a HashMap.
    pub content: Option<EventContent>,

    /// The Expiry time of the event.
    pub expiry_time: Option<DateTime<Utc>>,
}

/// The content of an `AgentEvent`.
///
/// Either some content value, or an instruction for the agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventContent {
    Content(Value),
    Instruction(AgentInstruction),
}

impl AgentEvent {
    /// Creates a new Event instance with the given parameters.
    ///
    /// ## Arguments
    ///
    /// * `name` - The name of the event.
    /// * `source` - The source of the event.
    /// * `content` - The content of the event. (optional)
    /// * `lifetime` - How long the event should live. (optiona;)
    ///
    /// ## Returns
    ///
    /// * `Event` - The new Event instance.
    pub fn new(
        name: &'static str,
        source: &'static str,
        content: Option<EventContent>,
        lifetime: Option<Duration>,
    ) -> Self {
        // Calculate expiry time
        let expiry_time = lifetime.map(|lifetime| Utc::now() + lifetime);

        Self {
            id: 0,       // Placeholder value, will be set by the EventPool
            name,        // The name of the event
            source,      // The source of the event
            content,     // The content of the event
            expiry_time, // The expiry time of the event
        }
    }
}
