use serde::{Deserialize, Serialize};

/// The schema representing a message in a chat
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Message {
    /// An message sent by AI assistant
    Assistant(String),

    /// An message sent by user
    User(String),

    /// A tool call request by AI assistant
    /// `ToolCall(name, id, params)`
    ToolCall(String, String, serde_json::Value),

    /// Result of the tool call
    /// `ToolResult(name, id, result)`
    ToolResult(String, String, serde_json::Value),
}

impl Message {
    /// Create a user message
    pub fn user(content: String) -> Self {
        Self::User(content)
    }

    /// Create an assistant message
    pub fn assistant(content: String) -> Self {
        Self::Assistant(content)
    }

    /// Create a tool call message
    pub fn tool_call(name: String, id: String, params: serde_json::Value) -> Self {
        Self::ToolCall(name, id, params)
    }

    /// Create a tool result message
    pub fn tool_result(name: String, id: String, result: serde_json::Value) -> Self {
        Self::ToolResult(name, id, result)
    }
}

/// Content of a message
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MessageContent {
    /// Text content
    Text(String),
}
