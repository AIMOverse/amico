use amico_core::entities::Event;
use serde_json::Value;
use std::{thread, time::Duration};

/// Implementation of the EventGenerator trait.
#[derive(Default)]
pub struct EventGenerator;

impl amico_core::traits::EventGenerator for EventGenerator {
    fn generate_event(&self, source: String, params: Value) -> Vec<Event> {
        // TODO Implement the event generation logic here
        // Simulate some processing time
        thread::sleep(Duration::from_secs(30));
        vec![Event::new(
            "HalfMinuteEvent".to_string(),
            source,
            params,
            None,
        )]
    }
}
