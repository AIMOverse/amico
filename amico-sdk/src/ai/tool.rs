use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ai::errors::ToolCallError;

pub struct Tool {
    pub definition: ToolDefinition,
    pub tool_call:
        Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, ToolCallError> + Send + Sync>,
}

impl Tool {
    pub fn def(&self) -> &ToolDefinition {
        &self.definition
    }

    pub fn call(&self, args: serde_json::Value) -> Result<serde_json::Value, ToolCallError> {
        (self.tool_call)(args)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

pub struct ToolSet {
    pub tools: HashMap<String, Tool>,
}

impl From<Vec<Tool>> for ToolSet {
    fn from(tools: Vec<Tool>) -> Self {
        let mut tool_set = ToolSet::new();
        for tool in tools {
            tool_set.add_tool(tool);
        }
        tool_set
    }
}

impl Default for ToolSet {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolSet {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn add_tool(&mut self, tool: Tool) {
        self.tools.insert(tool.def().name.clone(), tool);
    }

    pub fn get<'a>(&'a self, name: &str) -> Option<&'a Tool> {
        self.tools.get(name)
    }

    pub fn describe(&self) -> String {
        let mut result = String::new();
        for tool in self.tools.values() {
            result.push_str(&format!(
                "- {}: {}\n",
                tool.def().name,
                tool.def().description
            ));
        }
        result
    }
}
