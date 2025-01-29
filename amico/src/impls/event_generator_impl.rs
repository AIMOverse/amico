use amico_core::entity::{Event, EventGenerator};
use std::any::Any;
use std::collections::HashMap;

/// Implementation of the EventGenerator trait.
pub struct EventGeneratorImpl;

impl EventGeneratorImpl {
    pub fn new() -> Self {
        EventGeneratorImpl
    }
}

impl EventGenerator for EventGeneratorImpl {
    fn generate_event(&self, params: HashMap<String, Box<dyn Any>>) -> Vec<Event> {
        // Generate and return a list of example events
        vec![
            Event {
                name: "example_event".to_string(),
                source: "example_source".to_string(),
                params,
            }
        ]
    }
}