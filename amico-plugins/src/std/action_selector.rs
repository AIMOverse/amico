use crate::interface::{Plugin, PluginCategory, PluginInfo};
use amico::ai::provider::Provider;
use amico::ai::service::Service;
use amico::core::action_map::ActionMap;
use amico_core::entities::Event;
use amico_core::traits::Action;

/// Implementation of the ActionSelector Plugin.
#[derive(Default)]
pub struct ActionSelector {
    // Actions
    pub actions_map: ActionMap,
    pub service: dyn Service,
    pub provider: dyn Provider,
}

impl Plugin for ActionSelector {
    const INFO: &'static PluginInfo = &PluginInfo {
        name: "ActionSelector",
        category: PluginCategory::ActionSelector,
    };
}

impl amico_core::traits::ActionSelector for ActionSelector {
    // Temporarily ignore the events
    fn select_action(&mut self, _events: Vec<Event>) -> (Box<dyn Action>, Vec<u32>) {
        // Prompt
        let prompt = "You are a Action Selector to select actions to execute in an agent.\
             You will be provided with information of the environment and the state of \
             the current agent and make the best decision. Don't output the reason of choosing the action.\
              Just output the name and the parameters of the action you choose instead.\
              Here is a example of the output:".to_string();
        let example_output = r#"{
            "name": "clean",
            "parameters": {
                "room": "kitchen"
            }
        }"#;
        let prompt = format!("{}{}", prompt, example_output);
        // Wait for AI Service to complete
        todo!()
    }
}
