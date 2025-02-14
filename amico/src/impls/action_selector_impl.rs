use crate::actions::PrintAction;
use amico_core::entities::Event;
use amico_core::errors::ActionSelectorError;
use amico_core::traits::{Action, ActionSelector};
use std::thread;
use std::time::Duration;

/// Implementation of the ActionSelector trait.
#[derive(Default)]
pub struct ActionSelectorImpl;

impl ActionSelector for ActionSelectorImpl {
    fn select_action(
        &mut self,
        events: Vec<Event>,
    ) -> Result<(Box<dyn Action>, Vec<u32>), ActionSelectorError> {
        println!("events: {:?}", events);
        if !events.is_empty() {
            // Simulate some processing time
            thread::sleep(Duration::from_millis(200));
            Ok((
                Box::new(PrintAction::new("Executing actions".to_string())),
                events.iter().map(|event| event.id).collect(),
            ))
        } else {
            Ok((
                Box::new(PrintAction::new("No events available".to_string())),
                vec![],
            ))
        }
    }
}
