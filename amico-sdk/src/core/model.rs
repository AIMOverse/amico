use serde_json::Value;

// Use as a model of the current world in model-based Agent
pub trait Model {
    // Update the model with the new data
    fn update_model(&self, data: Value);

    // Get the current state of the model
    fn get_environment_description(&self) -> String;
}
