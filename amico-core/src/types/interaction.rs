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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Chat {
    pub messages: Vec<ChatMessage>,
    pub session_id: SessionId,
}

/// A interaction chat message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ChatMessage {
    /// The message content.
    pub content: String,

    /// The message role.
    pub role: String,
}

impl Chat {
    /// Creates a new chat interaction.
    pub fn new() -> Self {
        Self {
            messages: vec![],
            session_id: 0,
        }
    }

    /// Sets the messages of the chat interaction.
    pub fn messages(self, messages: Vec<ChatMessage>) -> Self {
        Self { messages, ..self }
    }

    /// Sets the session ID of the chat interaction.
    pub fn session_id(self, session_id: SessionId) -> Self {
        Self { session_id, ..self }
    }

    /// Converts the chat interaction into an interaction.
    pub fn into_interaction(self) -> Interaction {
        Interaction::Chat(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_chat() {
        let chat = Chat::new()
            .messages(vec![ChatMessage {
                content: "Hello, world!".to_string(),
                role: "user".to_string(),
            }])
            .session_id(1);

        assert_eq!(
            chat.messages,
            vec![ChatMessage {
                content: "Hello, world!".to_string(),
                role: "user".to_string(),
            }]
        );
        assert_eq!(chat.session_id, 1);
    }
}
