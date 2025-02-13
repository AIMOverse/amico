use amico_core::errors::ActionError;

pub struct AIAction {
    pub name: String,                              // The name of the action
    pub description: String,                       // The description of the action
    pub parameters: serde_json::Value,             // The actual parameters
    pub parameters_description: serde_json::Value, // The description of the parameters (The types of the parameters)
    pub action: Box<dyn Fn(serde_json::Value) -> Result<(), ActionError>>, // The actual action
}

impl amico_core::traits::Action for AIAction {
    // Execute the action
    fn execute(&self) -> Result<(), ActionError> {
        (self.action)(self.parameters.clone())
    }
}

impl AIAction {
    pub fn new(
        name: String,
        description: String,
        parameters: serde_json::Value,
        parameters_description: serde_json::Value,
        action: Box<dyn Fn(serde_json::Value) -> Result<(), ActionError>>,
    ) -> Self {
        Self {
            name,
            description,
            parameters,
            parameters_description,
            action,
        }
    }
}
