//! Core event types and traits

use core::any::{Any, TypeId};
use core::fmt::Debug;

/// A unique identifier for an event type
pub type EventTypeId = TypeId;

/// A unique identifier for an entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

impl EntityId {
    /// Create a new entity ID
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
    
    /// Get the raw ID value
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Trait for events that can be sent globally
pub trait GlobalEvent: Any + Send + Sync + Debug {
    /// Get the type ID of this event
    fn type_id(&self) -> EventTypeId {
        TypeId::of::<Self>()
    }
}

/// Helper trait for downcasting
pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

/// Blanket implementation for all types that implement Any
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

/// Trait for events that can be sent to specific entities
pub trait TargetedEvent: Any + Send + Sync + Debug {
    /// Get the type ID of this event
    fn type_id(&self) -> EventTypeId {
        TypeId::of::<Self>()
    }
}

/// Trait for event sets (used for mediator output)
pub trait EventSet: Send + Sync + Debug {
    /// Send all events in this set to the event bus
    fn send_all(self, sender: &mut dyn EventSender);
}

/// Trait for sending events
pub trait EventSender {
    /// Send a global event
    fn send_global(&mut self, event: Box<dyn AsAny>);
    
    /// Send a targeted event to a specific entity
    fn send_targeted(&mut self, target: EntityId, event: Box<dyn AsAny>);
}

/// Empty event set implementation
impl EventSet for () {
    fn send_all(self, _sender: &mut dyn EventSender) {
        // No events to send
    }
}

/// Single event implementation
impl<T: GlobalEvent + 'static> EventSet for T {
    fn send_all(self, sender: &mut dyn EventSender) {
        sender.send_global(Box::new(self));
    }
}

/// Tuple implementation for multiple events
impl<T1, T2> EventSet for (T1, T2) 
where 
    T1: GlobalEvent + 'static,
    T2: GlobalEvent + 'static,
{
    fn send_all(self, sender: &mut dyn EventSender) {
        sender.send_global(Box::new(self.0));
        sender.send_global(Box::new(self.1));
    }
}

/// Result type for event handling
pub type EventResult<T = ()> = Result<T, EventError>;

/// Error type for event handling
#[derive(Debug)]
pub enum EventError {
    /// Handler execution failed
    HandlerFailed(String),
    /// Event type not found
    EventTypeNotFound,
    /// Entity not found
    EntityNotFound,
}

impl core::fmt::Display for EventError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EventError::HandlerFailed(msg) => write!(f, "Handler failed: {}", msg),
            EventError::EventTypeNotFound => write!(f, "Event type not found"),
            EventError::EntityNotFound => write!(f, "Entity not found"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EventError {}