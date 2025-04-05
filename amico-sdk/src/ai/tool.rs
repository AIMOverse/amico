use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, future::Future, sync::Arc};

use crate::ai::errors::ToolCallError;

/// Definition of a tool in natural language
///
/// **TODO**: Restrict the parameters to be valid JSON Schema
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

/// Result type of tool call
pub type ToolResult = Result<serde_json::Value, ToolCallError>;

/// A tool that can be called by AI Agent.
#[derive(Clone)]
pub struct Tool {
    pub definition: ToolDefinition,
    tool_call: ToolCallFn,
}

impl Tool {
    /// Returns the definition of the tool.
    pub fn def(&self) -> &ToolDefinition {
        &self.definition
    }

    /// Calls the tool with the given arguments.
    pub async fn call(&self, args: serde_json::Value) -> ToolResult {
        match &self.tool_call {
            ToolCallFn::Sync(f) => (f)(args),
            ToolCallFn::Async(f) => {
                (f)(args.clone())
                    .await
                    .map_err(|err| ToolCallError::ExecutionError {
                        tool_name: self.definition.name.clone(),
                        params: args.clone(),
                        reason: err.to_string(),
                    })?
            }
        }
    }
}

/// Type of the tool call function
#[derive(Clone)]
pub enum ToolCallFn {
    /// Synchronous tool call function
    Sync(Arc<dyn Fn(serde_json::Value) -> ToolResult + Send + Sync>),
    /// Asynchronous tool call function
    Async(Arc<dyn Fn(serde_json::Value) -> tokio::task::JoinHandle<ToolResult> + Send + Sync>),
}

/// Builder for `Tool`
///
/// **TODO**: Restrict the parameters to be valid JSON Schema
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToolBuilder {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

impl ToolBuilder {
    /// Creates a new `ToolBuilder` with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name of the tool
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Sets the description of the tool
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Sets the parameters of the tool
    pub fn parameters(mut self, parameters: serde_json::Value) -> Self {
        self.parameters = parameters;
        self
    }

    /// Builds the `Tool` with tool call function from the builder
    pub fn build<F>(self, tool_call: F) -> Tool
    where
        F: Fn(serde_json::Value) -> ToolResult + Send + Sync + 'static,
    {
        Tool {
            definition: ToolDefinition {
                name: self.name,
                description: self.description,
                parameters: self.parameters,
            },
            tool_call: ToolCallFn::Sync(Arc::new(tool_call)),
        }
    }

    /// Builds the `Tool` with async tool call function from the builder
    pub fn build_async<F, Fut>(self, tool_call: F) -> Tool
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ToolResult> + Send + 'static,
    {
        Tool {
            definition: ToolDefinition {
                name: self.name,
                description: self.description,
                parameters: self.parameters,
            },
            tool_call: ToolCallFn::Async(Arc::new(move |args| tokio::task::spawn(tool_call(args)))),
        }
    }
}

/// A collection of tools.
#[derive(Default)]
pub struct ToolSet {
    pub tools: HashMap<String, Tool>,
}

impl Debug for ToolSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{:?}",
            self.tools
                .iter()
                .map(|(name, _)| format!("- {} \n", name))
                .collect::<Vec<_>>()
        ))
    }
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
    pub fn get(&self, name: &str) -> Option<&Tool> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_tool() -> Tool {
        ToolBuilder::new()
            .name("test")
            .description("test")
            .parameters(serde_json::json!({}))
            .build(|args| {
                Ok(serde_json::json!({
                    "message": "ok",
                    "args": args,
                }))
            })
    }

    fn build_test_async_tool() -> Tool {
        ToolBuilder::new()
            .name("test_async")
            .description("test_async")
            .parameters(serde_json::json!({}))
            .build_async(|args| async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                Ok(serde_json::json!({
                    "message": "ok",
                    "args": args,
                }))
            })
    }

    #[tokio::test]
    async fn test_tool_call() {
        let tool = build_test_tool();
        assert_eq!(tool.def().name, "test");
        assert_eq!(tool.def().description, "test");
        assert_eq!(tool.def().parameters, serde_json::json!({}));
        assert_eq!(
            tool.call(serde_json::json!({"a": "b"})).await.unwrap(),
            serde_json::json!({"message": "ok", "args": serde_json::json!({"a": "b"})})
        );
    }

    #[tokio::test]
    async fn test_tool_call_async() {
        let tool = build_test_async_tool();
        assert_eq!(tool.def().name, "test_async");
        assert_eq!(tool.def().description, "test_async");
        assert_eq!(tool.def().parameters, serde_json::json!({}));
        assert_eq!(
            tool.call(serde_json::json!({"a": "b"})).await.unwrap(),
            serde_json::json!({"message": "ok", "args": serde_json::json!({"a": "b"})})
        );
    }
}
