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

    pub fn get(&self, name: &str) -> Option<&AIAction> {
        self.actions.get(name)
    }

    pub fn describe(&self) -> String {
        let mut result = String::new();
        for (name, action) in &self.actions {
            result.push_str(&format!(
                "{}: {} parameters: {}\n",
                name, action.description, action.parameters_description
            ));
        }
        result
    }
}
