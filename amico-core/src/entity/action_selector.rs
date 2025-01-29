use crate::entity::{Action, Event};

/// Trait for selecting an action based on an event.
pub trait ActionSelector {
    /// Selects an action based on the given event.
    fn select_action(&self, events: &mut Vec<Event>) -> Box<dyn Action>;
}
