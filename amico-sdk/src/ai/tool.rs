use std::collections::HashMap;

use errors::ToolCallError;

pub struct Tool {
    pub name: String,                              // The name of the tool
    pub description: String,                       // A short description of the tool
    pub parameters_description: serde_json::Value, // Not the real parameters, just store the type of the parameters
    pub tool_call: Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, ToolCallError>>, // The function that will be called when the tool is used
}

pub mod errors {
    #[derive(Debug, thiserror::Error)]
    pub enum ToolCallError {
        #[error("Invalid param {name}: {value} ({reason})")]
        InvalidParam {
            name: String,
            value: serde_json::Value,
            reason: String,
        },
    }
}

pub struct ToolSet {
    tools: HashMap<String, Tool>,
}

impl ToolSet {
    pub fn new(tools: Vec<Tool>) -> Self {
        let mut map = HashMap::new();
        for tool in tools {
            map.insert(tool.name.clone(), tool);
        }
        Self { tools: map }
    }

    pub fn get(&self, name: &str) -> Option<&Tool> {
        self.tools.get(name)
    }

    pub fn describe(&self) -> String {
        let mut result = String::new();
        for (name, tool) in &self.tools {
            result.push_str(&format!("{}: {}\n", name, tool.description));
        }
        result
    }
}
