use crate::actions::PrintAction;
use amico_core::entity::{Action, ActionSelector, Event};

/// Implementation of the ActionSelector trait.
#[derive(Default)]
pub struct ActionSelectorImpl;

impl ActionSelector for ActionSelectorImpl {
    fn select_action(&self, events: &mut Vec<Event>) -> Box<dyn Action> {
        if !events.is_empty() {
            let event = events.remove(0); // 移除并返回第一个事件
            Box::new(PrintAction::new(format!(
                "Processing event: {}",
                event.name
            )))
        } else {
            Box::new(PrintAction::new("No events available".to_string()))
        }
    }
}
