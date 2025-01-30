use crate::actions::PrintAction;
use amico_core::entities::Event;
use amico_core::traits::{Action, ActionSelector};

/// Implementation of the ActionSelector trait.
#[derive(Default)]
pub struct ActionSelectorImpl;

impl ActionSelector for ActionSelectorImpl {
    fn select_action(&self, events: &mut Vec<Event>) -> Box<dyn Action> {
        if !events.is_empty() {
            let event = events.remove(0); // Remove the first event from the list
            Box::new(PrintAction::new(format!(
                "Processing event: {}",
                event.name
            )))
        } else {
            Box::new(PrintAction::new("No events available".to_string()))
        }
    }
}
