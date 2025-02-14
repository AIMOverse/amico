use crate::interface::{Plugin, PluginCategory, PluginInfo};
use amico::ai::service::Service;
use amico::core::action_map::ActionMap;
use amico_core::entities::Event;
use amico_core::traits::Action;
use futures::executor::block_on;

/// Implementation of the ActionSelector Plugin.
#[derive(Default)]
pub struct ActionSelector {
    // Actions
    pub actions_map: ActionMap,
    pub service: Box<dyn Service>,
}

// Implement the Plugin trait for the ActionSelector struct
impl Plugin for ActionSelector {
    const INFO: &'static PluginInfo = &PluginInfo {
        name: "ActionSelector",
        category: PluginCategory::ActionSelector,
    };
}

// Implement the ActionSelector trait for the ActionSelector struct
impl amico_core::traits::ActionSelector for ActionSelector {
    // Temporarily ignore the events
    fn select_action(&mut self, _events: Vec<Event>) -> (Box<dyn Action>, Vec<u32>) {
        // TODO: Update the prompt
        let prompt = "Example prompt".to_string();

        // Get Response
        let response =
            block_on(self.service.generate_text(prompt)).expect("Failed to generate text");
        // Parse the response to JSON
        let json_response: serde_json::Value =
            serde_json::from_str(&response).expect("Failed to parse JSON");
        let mut action = self
            .actions_map
            .get(&json_response["name"].as_str().unwrap())
            .unwrap()
            .clone();
        action.set_parameters(serde_json::Value::Object(
            json_response["parameters"].as_object().unwrap().clone(),
        ));

        // Return the action
        todo!()
    }
}

impl ActionSelector {
    pub fn new(actions_map: ActionMap, service: Box<dyn Service>) -> Self {
        let mut instance = Self {
            actions_map,
            service,
        };
        // Update the system prompt
        instance.update_system_prompt();
        // Return the instance
        instance
    }

    fn update_system_prompt(&mut self) {
        // Set the system prompt
        let prompt = r#"You are an Action Selector to select actions to execute in an agent.
            You will be provided with information of the environment and the state of the current agent
            and make the best decision. Don't output the reason of choosing the action. Just output the
            name and the parameters of the action you choose instead."#;

        let example_output = r#"{
            "name": "clean",
            "parameters": {
                "room": "kitchen"
            }
        }"#;

        let final_prompt = format!(
            "{}\nHere is an example of the output:{}\nHere are the available actions:{}",
            prompt.trim(),
            example_output,
            self.actions_map.describe()
        );

        self.service.set_system_prompt(final_prompt);
    }

    pub fn set_service(&mut self, service: Box<dyn Service>) {
        // Set the service
        self.service = service;
        // Update the system prompt
        self.update_system_prompt();
    }
}
