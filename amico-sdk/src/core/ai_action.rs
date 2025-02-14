use amico_core::errors::ActionError;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct AIAction {
    // Name of the action
    pub name: String,

    // A short description of what the action does
    pub description: String,

    // The parameters passed to the action at runtime
    pub parameters: Value,

    // A description of each parameter
    pub parameters_description: Value,

    // The expected types for each parameter (e.g., string, number, boolean)
    pub parameters_types: Value,

    // The requirements for each parameter (e.g., "required" or "optional")
    pub parameters_requirements: Value,

    // The function representing the action to be executed
    // This is an asynchronous closure wrapped in an Arc to allow sharing between threads.
    pub action: Arc<dyn Fn(Value) -> Result<(), ActionError> + Send + Sync>,
}

impl amico_core::traits::Action for AIAction {
    /// Execute the action after validating the parameters.
    /// It first checks if all required parameters are provided and if each parameter is of the correct type.
    /// If validation passes, it executes the action with the provided parameters.
    fn execute(&self) -> Result<(), ActionError> {
        // Step 1: Validate the parameters
        if let Err(e) = self.validate_parameters() {
            return Err(e); // Return an error if parameter validation fails
        }

        // Step 2: Execute the action and return the result
        (self.action)(self.parameters.clone())
    }
}

impl AIAction {
    /// Create a new instance of AIAction.
    ///
    /// # Parameters
    /// - `name`: The name of the action.
    /// - `description`: A description of what the action does.
    /// - `parameters`: The initial parameters for the action.
    /// - `parameters_types`: The expected data types for each parameter.
    /// - `parameters_requirements`: Specifies which parameters are required or optional.
    /// - `parameters_description`: A description of each parameter.
    /// - `action`: The function that will be executed when this action is called.
    pub fn new<F>(
        name: String,
        description: String,
        parameters: Value,
        parameters_types: Value,
        parameters_requirements: Value,
        parameters_description: Value,
        action: F,
    ) -> Self
    where
        F: Fn(Value) -> Result<(), ActionError> + Send + Sync + 'static,
    {
        Self {
            name,
            description,
            parameters,
            parameters_types,
            parameters_requirements,
            parameters_description,
            action: Arc::new(action), // Wrap the function in an Arc for thread-safe usage
        }
    }

    /// Set new parameters for the action.
    /// This allows updating the action's parameters dynamically.
    ///
    /// # Parameters
    /// - `parameters`: The new parameters to set.
    pub fn set_parameters(&mut self, parameters: Value) {
        self.parameters = parameters;
    }

    /// Validate the parameters before executing the action.
    /// This method checks two main things:
    /// 1. All required parameters are present.
    /// 2. Each parameter is of the correct type.
    ///
    /// # Returns
    /// - `Ok(())` if all validations pass.
    /// - `Err(ActionError)` if a validation fails.
    fn validate_parameters(&self) -> Result<(), ActionError> {
        // Check if required parameters are present
        if let Some(requirements) = self.parameters_requirements.as_object() {
            for (param, requirement) in requirements {
                // If the requirement is "required" and the parameter is missing, return an error
                if requirement == "required" && !self.parameters.get(param).is_some() {
                    return Err(ActionError::MissingRequiredParameters(param.clone()));
                }
            }
        }

        // Check if the types of parameters match the expected types
        if let Some(types) = self.parameters_types.as_object() {
            for (param, expected_type) in types {
                if let Some(value) = self.parameters.get(param) {
                    // If the type does not match, return an error
                    if !self.check_type(value, expected_type.as_str().unwrap_or("")) {
                        return Err(ActionError::InvalidParameterType(
                            param.clone(),
                            expected_type.clone().to_string(),
                        ));
                    }
                }
            }
        }

        // If all validations pass, return Ok
        Ok(())
    }

    /// Helper function to check if a parameter matches the expected type.
    ///
    /// # Parameters
    /// - `value`: The parameter value to check.
    /// - `expected_type`: The expected type as a string (e.g., "string", "number").
    ///
    /// # Returns
    /// - `true` if the type matches.
    /// - `false` if the type does not match.
    fn check_type(&self, value: &Value, expected_type: &str) -> bool {
        match expected_type {
            "string" => value.is_string(),
            "number" => value.is_number(),
            "boolean" => value.is_boolean(),
            "object" => value.is_object(),
            "array" => value.is_array(),
            _ => false, // Return false for unknown types
        }
    }
}
