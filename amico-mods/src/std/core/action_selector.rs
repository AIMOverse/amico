use crate::interface::{Plugin, PluginCategory, PluginInfo};
use amico::ai::provider::Provider;
use amico::ai::service::Service;
use amico::core::action_map::ActionMap;
use amico::core::model::Model;
use amico_core::entities::Event;
use amico_core::errors::ActionSelectorError;
use amico_core::traits::Action;
use futures::executor::block_on;
use std::marker::PhantomData;

/// A Standard Implementation of the ActionSelector Plugin.
pub struct ActionSelector<S, P>
where
    S: Service<P>,
    P: Provider,
{
    // Actions
    pub actions_map: ActionMap,
    pub service: S,
    pub model: Box<dyn Model>,

    // The PhantomData has zero runtime cost - it's
    // just a marker that helps the compiler understand
    // our intentions with the type parameter. This should
    // resolve the "type parameter P is never used" warning.
    // -- Claude 3.5 Sonnet
    _phantom: PhantomData<P>,
}

// Implement the Plugin trait for the ActionSelector struct
impl<S, P> Plugin for ActionSelector<S, P>
where
    S: Service<P>,
    P: Provider,
{
    fn info(&self) -> &'static PluginInfo {
        &PluginInfo {
            name: "StandardActionSelector",
            category: PluginCategory::ActionSelector,
        }
    }
}

// Implement the ActionSelector trait for the ActionSelector struct
impl<S, P> amico_core::traits::ActionSelector for ActionSelector<S, P>
where
    S: Service<P>,
    P: Provider,
{
    // Temporarily ignore the events
    fn select_action(
        &mut self,
        events: Vec<Event>,
    ) -> Result<(Box<dyn Action>, Vec<u32>), ActionSelectorError> {
        // Update the prompt
        let prompt = format!(
            "Instruction - Return the correct action for the current state \
                                     and return the id of the events that is going to be solved.\n\
                                     Context - {}\n\
                                     Received Events - {}",
            self.model.get_environment_description(),
            serde_json::to_string(&events).unwrap()
        )
        .to_string();

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
        if let (Some(action_name), Some(parameters), Some(event_ids)) = (
            json_response["name"].as_str(),
            json_response["parameters"].as_object(),
            json_response["event_ids"].as_array(),
        ) {
            // Check if the action exists in the actions_map
            if let Some(mut action) = self.actions_map.get(action_name) {
                action.set_parameters(serde_json::Value::Object(parameters.clone()));

                // Return the action and the Vec<u32>
                return Ok((
                    Box::new(action),
                    event_ids
                        .iter()
                        .map(|id| id.as_u64().unwrap() as u32)
                        .collect(),
                ));
            }
        }

        // Return a default action and an empty vector if the JSON response is not valid
        Err(ActionSelectorError::SelectingActionFailed(
            "Failed to select an action from the response".to_string(),
        ))
    }
}

/// Implement the ActionSelector struct
impl<S, P> ActionSelector<S, P>
where
    S: Service<P>,
    P: Provider,
{
    /// Create a new instance of the ActionSelector struct.
    pub fn new(actions_map: ActionMap, service: S, model: Box<dyn Model>) -> Self {
        Self {
            actions_map,
            service,
            model,
            _phantom: PhantomData,
        }
    }

    /// Update the system prompt.
    fn update_system_prompt(&mut self) {
        // Set the system prompt
        let prompt = r#"You are an Action Selector to select actions to execute in an agent.
            You will be provided with information of the environment, the state of the current agent
             and the events that are received. Make the best decision based on these information.
              Don't output the reason of choosing the action. Just output the
            name, the parameters of the action you choose and the event ids you solved."#;

        // An example output to be shown in the system prompt
        let example_output = r#"{
            "name": "clean",
            "parameters": {
                "room": "kitchen"
            },
            "event_ids": [1, 2, 3]
        }"#;

        // The final prompt to be set in the system
        let final_prompt = format!(
            "{}\n\
            Here is an example of the output:{}\n\
            Here are the available actions:{}",
            prompt.trim(),
            example_output,
            self.actions_map.describe()
        );

        // Set the system prompt
        self.service.mut_ctx().update(move |ctx| {
            ctx.system_prompt = final_prompt.to_string();
        });
    }

    /// Set the AI service for the ActionSelector.
    pub fn set_service(&mut self, service: S) {
        // Set the service
        self.service = service;
        // Update the system prompt
        self.update_system_prompt();
    }
}
