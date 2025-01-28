use crate::entities::{Action, Event};

/// Trait for selecting an action based on an event.
pub trait ActionSelector {
    /// Selects an action based on the given event.
    fn select_action(&self, event: &dyn Event) -> Box<dyn Action>;
}