use std::collections::HashMap;

use serde_json::Value;

use crate::ai::errors::ToolCallError;

pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub tool_call: ToolCall,
}

pub enum ToolCall {
    Sync(Box<dyn Fn(Value) -> ToolResult + Send + Sync>),
    Async(Box<dyn Fn(Value) -> tokio::task::JoinHandle<ToolResult> + Send + Sync>),
}

pub type ToolResult = Result<Value, ToolCallError>;

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
        self.tools.insert(tool.name.clone(), tool);
    }

    pub fn get<'a>(&'a self, name: &str) -> Option<&'a Tool> {
        self.tools.get(name)
    }

    pub fn describe(&self) -> String {
        let mut result = String::new();
        for tool in self.tools.values() {
            result.push_str(&format!("- {}: {}\n", tool.name, tool.description));
        }
        result
    }
}
