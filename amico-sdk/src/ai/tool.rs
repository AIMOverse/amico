use std::collections::HashMap;

use crate::ai::errors::ToolCallError;

pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub tool_call: Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, ToolCallError>>,
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
