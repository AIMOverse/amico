use std::any::Any;
use std::collections::HashMap;

/// Struct representing an event in the system.
pub struct Event {
    /// The name of the event.
    pub name: String,
    /// The source of the event.
    pub source: String,
    /// The parameters of the event, stored as a HashMap.
    pub params: HashMap<String, Box<dyn Any>>,
}