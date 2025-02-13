use crate::interface::{Plugin, PluginCategory, PluginInfo};
use amico_core::entities::Event;
use amico_core::traits::Action;

/// Implementation of the ActionSelector Plugin.
#[derive(Default)]
pub struct ActionSelector;

impl Plugin for ActionSelector {
    const INFO: &'static PluginInfo = &PluginInfo {
        name: "ActionSelector",
        category: PluginCategory::ActionSelector,
    };
}

impl amico_core::traits::ActionSelector for ActionSelector {
    fn select_action(&self, events: Vec<Event>) -> (Box<dyn Action>, Vec<u32>) {
        todo!()
    }
}
