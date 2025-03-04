use serde_json::Value;
use std::collections::HashMap;

use crate::ai::errors::ToolCallError;

/// Struct for LLM Tool
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub tool_call: ToolCall,
}

/// The actual tool call closure
pub enum ToolCall {
    Sync(Box<dyn Fn(Value) -> ToolResult + Send + Sync>),
    Async(Box<dyn Fn(Value) -> tokio::task::JoinHandle<ToolResult> + Send + Sync>),
}

/// Result type for tool call executors
pub type ToolResult = Result<Value, ToolCallError>;

/// Struct for a set of tools
pub struct ToolSet {
    /// The tool collection
    pub tools: HashMap<String, Tool>,
}

impl From<Vec<Tool>> for ToolSet {
    /// Build the tool set from a vector of tools
    fn from(tools: Vec<Tool>) -> Self {
        let mut tool_set = ToolSet::new();
        for tool in tools {
            tool_set.add_tool(tool);
        }
        tool_set
    }
}

impl Default for ToolSet {
    /// Default value： build an empty tool set
    fn default() -> Self {
        Self::new()
    }
}

impl ToolSet {
    /// Build an empty tool set
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Add a tool to the set
    pub fn add_tool(&mut self, tool: Tool) {
        self.tools.insert(tool.name.clone(), tool);
    }

    /// Get a tool by name
    pub fn get<'a>(&'a self, name: &str) -> Option<&'a Tool> {
        self.tools.get(name)
    }

    /// Describe the tools in natural language in the set
    pub fn describe(&self) -> String {
        let mut result = String::new();
        for tool in self.tools.values() {
            result.push_str(&format!("- {}: {}\n", tool.name, tool.description));
        }
        result
    }
}
