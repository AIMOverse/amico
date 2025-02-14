use crate::interface::{Plugin, PluginCategory, PluginInfo};
use amico::core::action_map::ActionMap;
use amico_core::entities::Event;
use amico_core::traits::Action;

/// Implementation of the ActionSelector Plugin.
#[derive(Default)]
pub struct ActionSelector {
    // Actions
    pub actions_map: ActionMap,
}

impl Plugin for ActionSelector {
    fn info(&self) -> &'static PluginInfo {
        &PluginInfo {
            name: "ActionSelector",
            category: PluginCategory::ActionSelector,
        }
    }
}

impl amico_core::traits::ActionSelector for ActionSelector {
    // Temporarily ignore the events
    fn select_action(&self, _events: Vec<Event>) -> (Box<dyn Action>, Vec<u32>) {
        todo!()
    }
}
