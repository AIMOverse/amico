use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

/// Struct representing an event in the system.

#[derive(Debug, Clone)]
pub struct Event {
    /// The ID of the event.
    pub id: u32,
    /// The name of the event.
    pub name: String,
    /// The source of the event.
    pub source: String,
    /// The parameters of the event, stored as a HashMap.
    pub params: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl Event {
    /// Creates a new Event instance with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the event.
    /// * `source` - The source of the event.
    /// * `params` - The parameters of the event.
    ///
    /// # Returns
    ///
    /// * `Event` - The new Event instance.
    pub fn new(
        name: String,
        source: String,
        params: HashMap<String, Arc<dyn Any + Send + Sync>>,
    ) -> Self {
        Self {
            id: 0,  // Placeholder value, will be set by the EventPool
            name,   // The name of the event
            source, // The source of the event
            params, // The parameters of the event
        }
    }
}
