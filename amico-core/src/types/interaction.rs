use serde::{Deserialize, Serialize};

/// An interaction with the Agent, e.g. a chat request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Interaction {
    /// A chat interaction.
    Chat(Chat),
}

/// The session ID type.
pub type SessionId = u64;

/// A chat interaction context data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Chat {
    messages: Vec<ChatMessage>,
    session_id: SessionId,
}

/// A interaction chat message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMessage {
    /// The message content.
    pub content: String,

    /// The message role.
    pub role: String,
}
