use crate::core::ai_action::AIAction;
use std::collections::HashMap;

#[derive(Default)]
pub struct ActionMap {
    actions: HashMap<String, AIAction>,
}

impl ActionMap {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    pub fn add_action(&mut self, action: AIAction) {
        self.actions.insert(action.name.clone(), action);
    }

    pub fn get(&self, name: &str) -> Option<AIAction> {
        self.actions.get(name).cloned()
    }

    pub fn describe(&self) -> String {
        let mut result = String::new();
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
