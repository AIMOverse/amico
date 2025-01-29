use crate::actions::PrintAction;
use amico_core::entity::{Action, ActionSelector, Event};

pub struct ActionSelectorImpl;

impl ActionSelectorImpl {
    pub fn new() -> Self {
        ActionSelectorImpl
    }
}

impl ActionSelector for ActionSelectorImpl {
    fn select_action(&self, event: Event) -> Box<dyn Action> {
        // Example logic to select an action based on the event
        Box::new(PrintAction::new(format!("Action for event: {}", event.name)))
    }
}