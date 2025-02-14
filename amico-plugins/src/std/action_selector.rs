use crate::interface::{Plugin, PluginCategory, PluginInfo};
use amico::ai::service::Service;
use amico::core::action_map::ActionMap;
use amico_core::entities::Event;
use amico_core::errors::ActionSelectorError;
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
    fn select_action(
        &mut self,
        _events: Vec<Event>,
    ) -> Result<(Box<dyn Action>, Vec<u32>), ActionSelectorError> {
        // Update the prompt
        let prompt = "example prompt".to_string();

        // Call the asynchronous method to get the response text
        let response = match block_on(self.service.generate_text(prompt)) {
            Ok(res) => res,
            Err(e) => {
                return Err(ActionSelectorError::SelectingActionFailed(format!(
                    "Failed to generate text: {}",
                    e
                )))
            }
        };

        // Try to parse the response as JSON
        let json_response: serde_json::Value = match serde_json::from_str(&response) {
            Ok(json) => json,
            Err(e) => {
                return Err(ActionSelectorError::SelectingActionFailed(format!(
                    "Failed to parse the response as JSON: {}",
                    e
                )));
            }
        };

        // Extract the action name and parameters from the JSON response
        if let (Some(action_name), Some(parameters)) = (
            json_response["name"].as_str(),
            json_response["parameters"].as_object(),
        ) {
            // Check if the action exists in the actions_map
            if let Some(mut action) = self.actions_map.get(action_name) {
                action.set_parameters(serde_json::Value::Object(parameters.clone()));
                // TODO - Get the removing event IDs from the response

                // Return the action and the Vec<u32>
                return Ok((Box::new(action), vec![]));
            }
        }

        // Return a default action and an empty vector if the JSON response is not valid
        Err(ActionSelectorError::SelectingActionFailed(
            "Failed to select an action from the response".to_string(),
        ))
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
