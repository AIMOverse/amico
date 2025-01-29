use amico_core::entity::{Event, EventGenerator};
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Implementation of the EventGenerator trait.
#[derive(Default)]
pub struct EventGeneratorImpl;

impl EventGenerator for EventGeneratorImpl {
    fn generate_event(
        &self,
        source: String,
        params: HashMap<String, Arc<Mutex<dyn Any + Send + Sync>>>,
    ) -> Vec<Event> {
        // Generate and return a list of example events
        println!("Generating event with source: {}", source);
        vec![Event {
            name: "example_event".to_string(),
            source,
            params,
        }]
    }
}
