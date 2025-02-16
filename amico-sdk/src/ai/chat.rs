use serde::{Deserialize, Serialize};

/// The schema representing a message in a chat
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Message {
    pub role: String,
    pub content: String,
    pub name: Option<String>,              // Used in tool calls
    pub tool_calls: Option<Vec<ToolCall>>, // Used in tool calls
}

impl Message {
    /// Create a user message
    pub fn user(content: String) -> Self {
        Self {
            role: "user".to_string(),
            content,
            name: None,
            tool_calls: None,
        }
    }

    /// Create an assistant message
    pub fn assistant(content: String) -> Self {
        Self {
            role: "assistant".to_string(),
            content,
            name: None,
            tool_calls: None,
        }
    }

    /// Create an assistant message with a tool call
    pub fn assistant_tool_call(tool_calls: Vec<ToolCall>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: "".to_string(),
            name: None,
            tool_calls: Some(tool_calls),
        }
    }

    /// Create a tool message
    pub fn tool(name: String, content: String) -> Self {
        Self {
            role: "tool".to_string(),
            content,
            name: Some(name),
            tool_calls: None,
        }
    }
}

/// The schema representing a tool call
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

impl ToolCall {
    pub fn function(id: String, name: String, arguments: serde_json::Value) -> Self {
        Self {
            id,
            r#type: "function".to_string(),
            function: ToolCallFunction { name, arguments },
        }
    }
}

/// The schema representing a tool call function
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Type alias for a chat history
pub type ChatHistory = Vec<Message>;
