use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ai::errors::ToolCallError;

/// Definition of a tool in natural language
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToolDefinition {
    /// Name of the tool
    pub name: String,
    /// Description of the tool
    pub description: String,
    /// Parameter descriptions of the tool
    pub parameters: serde_json::Value,
}

/// A tool that can be called by AI Agent.
pub struct Tool {
    pub definition: ToolDefinition,
    pub tool_call:
        Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, ToolCallError> + Send + Sync>,
}

impl Tool {
    /// Returns the definition of the tool.
    pub fn def(&self) -> &ToolDefinition {
        &self.definition
    }

    /// Calls the tool with the given arguments.
    pub fn call(&self, args: serde_json::Value) -> Result<serde_json::Value, ToolCallError> {
        (self.tool_call)(args)
    }
}

/// A collection of tools.
#[derive(Default)]
pub struct ToolSet {
    pub tools: HashMap<String, Tool>,
}

impl From<Vec<Tool>> for ToolSet {
    /// Build a `ToolSet` from a vector of `Tool`s.
    fn from(tools: Vec<Tool>) -> Self {
        let mut tool_set = ToolSet::new();
        for tool in tools {
            tool_set.add_tool(tool);
        }
        tool_set
    }
}

impl ToolSet {
    /// Creates a new empty `ToolSet`.
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Adds a tool to the `ToolSet`.
    pub fn add_tool(&mut self, tool: Tool) {
        self.tools.insert(tool.def().name.clone(), tool);
    }

    /// Returns the tool with the given name.
    pub fn get<'a>(&'a self, name: &str) -> Option<&'a Tool> {
        self.tools.get(name)
    }

    /// Describes the tools in the set in natural language.
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

    /// Iterate over the tools in the set.
    pub fn iter(&self) -> impl Iterator<Item = &Tool> {
        self.tools.values()
    }

    /// Iterate over the tool definitions in the set.
    pub fn iter_defs(&self) -> impl Iterator<Item = &ToolDefinition> {
        self.tools.values().map(|t| t.def())
    }
}
