//! Conversation model types.
//!
//! These types represent the conversation history that an agent
//! accumulates during a workflow. They are used as input to each
//! atomic agent step.

/// Role of a participant in a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// System instructions for the model.
    System,
    /// A message from the user / caller.
    User,
    /// A message produced by the assistant / agent.
    Assistant,
    /// Output from a tool execution.
    Tool,
}

/// A single message in a conversation.
#[derive(Debug, Clone)]
pub struct Message {
    /// Who produced this message.
    pub role: Role,
    /// Textual content of the message.
    pub content: String,
}

impl Message {
    /// Create a new message.
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    /// Create a system message.
    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }

    /// Create a user message.
    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }

    /// Create an assistant message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }

    /// Create a tool result message.
    pub fn tool(content: impl Into<String>) -> Self {
        Self::new(Role::Tool, content)
    }
}

/// A tool call request issued by the model.
#[derive(Debug, Clone)]
pub struct ToolCall {
    /// Unique identifier for this call (used to correlate with the result).
    pub id: String,
    /// Name of the tool to invoke.
    pub name: String,
    /// Serialized arguments for the tool (typically JSON).
    pub arguments: String,
}

/// The result of executing a tool call.
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// The `id` of the `ToolCall` this result corresponds to.
    pub call_id: String,
    /// Serialized output produced by the tool.
    pub output: String,
}
