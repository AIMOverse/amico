use crate::types::AgentEvent;
use serde_json::Value;

/// Trait for generating events.
pub trait EventGenerator {
    /// Generates a list of events based on the given parameters.
    ///
    /// # Arguments
    ///
    /// * `source` - The source of the event.
    ///
    /// * `params` - A Json Object that holds the parameters from inputs.
    ///
    /// # Returns
    ///
    /// * `Vec<Event>` - A vector of Event instances that is going to be added into event pool.
    fn generate_event(&self, source: &'static str, params: Value) -> Vec<AgentEvent>;
}
