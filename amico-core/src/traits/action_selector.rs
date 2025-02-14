use crate::entities::Event;
use crate::errors::ActionSelectorError;
use crate::traits::Action;

/// Trait for selecting an action based on an event.
pub trait ActionSelector {
    /// Selects an action based on the given event.
    /// Arguments:
    ///     * `events` - A vector of Event instances in the current event pool.
    /// Returns:
    ///     * A tuple containing the selected action
    ///     and the IDs of the events that is going to be removed from event pool.
    fn select_action(
        &mut self,
        events: Vec<Event>,
    ) -> Result<(Box<dyn Action>, Vec<u32>), ActionSelectorError>;
}
