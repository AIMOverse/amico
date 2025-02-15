use crate::ai::errors::ToolCallError;

pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub tool_call:
        Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, ToolCallError> + Send + Sync>,
}

pub type ToolSet = Vec<Tool>;

pub fn describe_tool_set(tool_set: &ToolSet) -> String {
    let mut result = String::new();
    for tool in tool_set {
        result.push_str(&format!("{}: {}\n", tool.name, tool.description));
    }
    result
}
