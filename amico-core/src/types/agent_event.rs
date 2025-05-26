use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::errors::AgentEventError;

use super::AgentInstruction;

/// Struct representing an event the agent receives.
///
/// # Examples
///
/// ## Create an event with content and lifetime
///
/// ```
/// use std::time::Duration;
/// use amico_core::types::{AgentEvent, EventContent};
/// use serde_json::Value;
///
/// let event = AgentEvent::new("test", "TestSource")
///     .content(Value::String("test".to_string()))
///     .lifetime(Duration::from_secs(10));
///
/// assert_eq!(event.name, "test");
/// assert_eq!(event.source, "TestSource");
/// assert!(event.content.is_some());
/// assert!(event.expiry_time.is_some());
/// ```
///
/// ## Create an event with instruction
///
/// ```
/// use amico_core::types::{AgentEvent, AgentInstruction, EventContent};
///
/// let event = AgentEvent::new("test", "TestSource")
///     .instruction(AgentInstruction::Terminate);
///
/// assert_eq!(event.content, Some(EventContent::Instruction(AgentInstruction::Terminate)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    /// The ID of the event.
    pub id: u32,

    /// The name of the event.
    pub name: &'static str,

    /// The event source information of the event.
    pub source: &'static str,

    /// The parameters of the event, stored as a HashMap.
    pub content: Option<EventContent>,

    /// The Expiry time of the event.
    pub expiry_time: Option<DateTime<Utc>>,
}

/// The content of an `AgentEvent`.
///
/// Either some content value, or an instruction for the agent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventContent {
    Content(Value),
    Instruction(AgentInstruction),
}

impl AgentEvent {
    /// Creates a new empty Event instance without content or expiry time.
    ///
    /// # Examples
    ///
    /// ```
    /// use amico_core::types::AgentEvent;
    ///
    /// let event = AgentEvent::new("test", "TestSource");
    ///
    /// assert_eq!(event.name, "test");
    /// assert_eq!(event.source, "TestSource");
    /// assert_eq!(event.content, None);
    /// assert_eq!(event.expiry_time, None);
    /// ```
    pub fn new(name: &'static str, source: &'static str) -> Self {
        Self {
            id: 0,             // Placeholder value, will be set by the EventPool
            name,              // The name of the event
            source,            // The source of the event
            content: None,     // Default to None
            expiry_time: None, // Default to None
        }
    }

    /// Adds content with a specific serializable type to the event.
    ///
    /// # Examples
    ///
    /// ```
    /// use amico_core::types::AgentEvent;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct MyContent {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let event = AgentEvent::new("test", "TestSource").with_content(MyContent {
    ///     name: "test".to_string(),
    ///     age: 123,
    /// }).unwrap();
    ///
    /// assert!(event.content.is_some());
    /// ```
    pub fn with_content<T: Serialize>(self, content: T) -> Result<Self, AgentEventError> {
        Ok(Self {
            content: Some(EventContent::Content(serde_json::to_value(content)?)),
            ..self
        })
    }

    /// Adds content to the event.
    ///
    /// Setting `content` will override any existing content or instruction.
    ///
    /// # Examples
    ///
    /// ```
    /// use amico_core::types::{AgentEvent, EventContent};
    /// use serde_json::Value;
    ///
    /// let event = AgentEvent::new("test", "TestSource")
    ///     .content(Value::String("test".to_string()));
    ///
    /// assert_eq!(event.content, Some(EventContent::Content(Value::String("test".to_string()))));
    /// ```
    pub fn content(self, content: Value) -> Self {
        Self {
            content: Some(EventContent::Content(content)),
            ..self
        }
    }

    /// Adds an instruction to the event.
    ///
    /// Setting `instruction` will override any existing instruction or content.
    ///
    /// # Examples
    ///
    /// ```
    /// use amico_core::types::{AgentEvent, AgentInstruction, EventContent};
    ///
    /// let event = AgentEvent::new("test", "TestSource")
    ///     .instruction(AgentInstruction::Terminate);
    ///
    /// assert_eq!(event.content, Some(EventContent::Instruction(AgentInstruction::Terminate)));
    /// ```
    pub fn instruction(self, instruction: AgentInstruction) -> Self {
        Self {
            content: Some(EventContent::Instruction(instruction)),
            ..self
        }
    }

    /// Sets the expiry time of the event.
    ///
    /// Setting `lifetime` will override any existing expiry time or expiry_at.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use amico_core::types::AgentEvent;
    ///
    /// let event = AgentEvent::new("test", "TestSource")
    ///     .lifetime(Duration::from_secs(10));
    ///
    /// assert!(event.expiry_time.is_some());
    /// ```
    pub fn lifetime(self, lifetime: Duration) -> Self {
        Self {
            expiry_time: Some(Utc::now() + lifetime),
            ..self
        }
    }

    /// Sets the expiry time of the event to a specific time.
    ///
    /// Setting `expiry_time` will override any existing expiry time or lifetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use chrono::{DateTime, Utc};
    /// use amico_core::types::AgentEvent;
    ///
    /// let event = AgentEvent::new("test", "TestSource")
    ///     .expire_at(Utc::now() + Duration::from_secs(10));
    ///
    /// assert!(event.expiry_time.is_some());
    /// ```
    pub fn expire_at(self, expiry_time: DateTime<Utc>) -> Self {
        Self {
            expiry_time: Some(expiry_time),
            ..self
        }
    }

    /// Parses the content of the event as a specific type.
    ///
    /// # Examples
    ///
    /// ```
    /// use amico_core::types::AgentEvent;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct MyContent {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let event = AgentEvent::new("test", "TestSource")
    ///     .with_content(MyContent { name: "test".to_string(), age: 123 })
    ///     .unwrap();
    ///
    /// let content = event.parse_content::<MyContent>().unwrap();
    ///
    /// assert_eq!(content.name, "test");
    /// assert_eq!(content.age, 123);
    /// ```
    pub fn parse_content<T: DeserializeOwned>(&self) -> Result<T, AgentEventError> {
        match &self.content {
            Some(EventContent::Content(content)) => {
                let content = content.clone();
                serde_json::from_value::<T>(content).map_err(AgentEventError::SerdeJson)
            }
            Some(_) => Err(AgentEventError::ContentError(
                "Content is an AgentInstruction",
            )),
            None => Err(AgentEventError::ContentError("Content is None")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_event() {
        let expiry_time = Utc::now() + Duration::from_secs(10);
        let event = AgentEvent::new("test", "TestSource")
            .content(Value::String("test".to_string()))
            .expire_at(expiry_time);

        assert_eq!(event.name, "test");
        assert_eq!(event.source, "TestSource");
        assert_eq!(
            event.content,
            Some(EventContent::Content(Value::String("test".to_string())))
        );
        assert_eq!(event.expiry_time, Some(expiry_time));
    }

    #[test]
    fn test_content_with_type() {
        #[derive(Serialize, Deserialize)]
        struct MyContent {
            name: String,
            age: u32,
        }

        let event = AgentEvent::new("test", "TestSource")
            .with_content(MyContent {
                name: "test".to_string(),
                age: 123,
            })
            .unwrap();

        let content = event.parse_content::<MyContent>().unwrap();

        assert_eq!(content.name, "test");
        assert_eq!(content.age, 123);
    }
}
