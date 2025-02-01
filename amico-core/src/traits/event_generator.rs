use crate::entities::Event;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

/// Trait for generating events.
pub trait EventGenerator {
    /// Generates a list of events based on the given parameters.
    ///
    /// # Arguments
    ///
    /// * `source` - The source of the event.
    ///
    /// * `params` - A HashMap that holds the parameters from inputs.
    ///
    /// # Returns
    ///
    /// * `Vec<Event>` - A vector of Event instances that is going to be added into event pool.
    fn generate_event(
        &self,
        source: String,
        params: HashMap<String, Arc<dyn Any + Send + Sync>>,
    ) -> Vec<Event>;
}
