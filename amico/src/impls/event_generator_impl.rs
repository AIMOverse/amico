use amico_core::entities::Event;
use amico_core::traits::EventGenerator;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Implementation of the EventGenerator trait.
#[derive(Default)]
pub struct EventGeneratorImpl;

impl EventGenerator for EventGeneratorImpl {
    fn generate_event(
        &self,
        source: String,
        params: HashMap<String, Arc<dyn Any + Send + Sync>>,
    ) -> Vec<Event> {
        // Generate and return a list of example events
        println!("Generating event with source: {}", source);
        // Simulate some processing time
        thread::sleep(Duration::from_millis(100));
        vec![Event::new("ExampleEvent".to_string(), source, params)]
    }
}
