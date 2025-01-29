use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Struct representing an event in the system.
pub struct Event {
    /// The name of the event.
    pub name: String,
    /// The source of the event.
    pub source: String,
    /// The parameters of the event, stored as a HashMap.
    pub params: HashMap<String, Arc<Mutex<dyn Any + Send + Sync>>>,
}
