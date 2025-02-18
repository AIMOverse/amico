use amico_core::entities::Event;
use std::{any::Any, collections::HashMap, sync::Arc, thread, time::Duration};

/// Implementation of the EventGenerator trait.
#[derive(Default)]
pub struct EventGenerator;

impl amico_core::traits::EventGenerator for EventGenerator {
    fn generate_event(
        &self,
        source: String,
        params: HashMap<String, Arc<dyn Any + Send + Sync>>,
    ) -> Vec<Event> {
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
