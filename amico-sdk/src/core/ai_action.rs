use amico_core::errors::ActionError;
use std::sync::Arc;

#[derive(Clone)]
pub struct AIAction {
    pub name: String,                              // Name of the action
    pub description: String,                       // Description of the action
    pub parameters: serde_json::Value,             // Parameters of the action
    pub parameters_description: serde_json::Value, // Description of the parameters(types of the parameters)
    pub action: Arc<dyn Fn(serde_json::Value) -> Result<(), ActionError> + Send + Sync>, // Function to execute the action
}

impl amico_core::traits::Action for AIAction {
    // Execute the action
    fn execute(&self) -> Result<(), ActionError> {
        (self.action)(self.parameters.clone())
    }
}

impl AIAction {
    pub fn new<F>(
        name: String,
        description: String,
        parameters: serde_json::Value,
        parameters_description: serde_json::Value,
        action: F,
    ) -> Self
    where
        F: Fn(serde_json::Value) -> Result<(), ActionError> + Send + Sync + 'static,
    {
        Self {
            name,
            description,
            parameters,
            parameters_description,
            action: Arc::new(action), // Wrap the function in Arc
        }
    }

    pub fn set_parameters(&mut self, parameters: serde_json::Value) {
        self.parameters = parameters;
    }
}
