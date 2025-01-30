use crate::entity::Event;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

/// Trait for generating events.
pub trait EventGenerator {
    /// Generates a list of events based on the given parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - A HashMap that holds the parameters for the events.
    ///
    /// # Returns
    ///
    /// * `Vec<Event>` - A vector of Event instances.
    fn generate_event(
        &self,
        source: String,
        params: HashMap<String, Arc<dyn Any + Send + Sync>>,
    ) -> Vec<Event>;
}
