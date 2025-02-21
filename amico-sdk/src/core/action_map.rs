use crate::core::ai_action::AIAction;
use std::collections::HashMap;

/// A map of actions that can be understood by AI.
#[derive(Default)]
pub struct ActionMap {
    actions: HashMap<String, AIAction>,
}

/// Implementation of the ActionMap struct.
impl ActionMap {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    /// Adds an action to the map.
    pub fn add_action(&mut self, action: AIAction) {
        self.actions.insert(action.name.clone(), action);
    }

    /// Get a copy of the action from the map.
    pub fn get(&self, name: &str) -> Option<AIAction> {
        self.actions.get(name).cloned()
    }

    /// Get a description of the actions in the map.
    pub fn describe(&self) -> String {
        // Create a string to store the result
        let mut result = String::new();
        // Iterate over the actions in the map
        for (name, action) in &self.actions {
            result.push_str(&format!(
                "name: {} \n\
                description: {}\n\
                parameters description: {}\n\
                parameters types: {}\n\
                Is parameters mandatory: {}\n\
                \n\n
                ",
                name,
                action.description,
                action.parameters_description,
                action.parameters_types,
                action.parameters_requirements,
            ));
        }
        result
    }
}
