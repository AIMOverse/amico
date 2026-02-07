//! Event system: sources, routing, and handling.
//!
//! Event sources are **streams** of events. Events carry context for
//! agents to inspect. An `EventRouter` dispatches received events to
//! the appropriate `EventHandler`. Handlers use the global context and
//! the event context to run an agent workflow, producing side-effects
//! and a generation result.

use crate::{Event, EventMetadata, Timestamp};
use std::future::Future;

// ---------------------------------------------------------------------------
// Event handler
// ---------------------------------------------------------------------------

/// Event handler trait - defines how to handle specific event types.
///
/// An event handler is similar to a route handler in a web framework
/// like Axum. It receives an event together with a shared context and
/// produces a typed response.
pub trait EventHandler<E: Event> {
    /// Shared global context for the handler (e.g., database pool, config).
    type Context;

    /// Response type produced by the handler.
    type Response;

    /// Error type for handler execution.
    type Error;

    /// Handle an event.
    fn handle<'a>(
        &'a self,
        event: E,
        context: &'a Self::Context,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a;
}

// ---------------------------------------------------------------------------
// Event dispatch
// ---------------------------------------------------------------------------

/// Event dispatch error.
#[derive(Debug)]
pub enum DispatchError {
    NoHandlerFound(String),
    HandlerFailed(String),
}

impl std::fmt::Display for DispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoHandlerFound(event_type) => {
                write!(f, "No handler found for event type: {}", event_type)
            }
            Self::HandlerFailed(msg) => write!(f, "Handler failed: {}", msg),
        }
    }
}

impl std::error::Error for DispatchError {}

/// Event router - registers and dispatches events to handlers.
///
/// The router is the central dispatcher of the event system. It maps
/// event type identifiers to handlers and drives the dispatch loop.
pub trait EventRouter {
    /// Event type that can be routed.
    type Event: Event;

    /// Handler type.
    type Handler;

    /// Register an event handler for a specific event type.
    fn register(&mut self, event_type: impl Into<String>, handler: Self::Handler);

    /// Dispatch a single event to the appropriate handler.
    fn dispatch<'a>(
        &'a self,
        event: Self::Event,
    ) -> impl Future<Output = Result<(), DispatchError>> + Send + 'a;
}

// ---------------------------------------------------------------------------
// Event source
// ---------------------------------------------------------------------------

/// An event source produces a stream of events.
///
/// Event sources connect the outside world to the agent runtime.
/// Examples include WebSocket connections, message queues, cron
/// timers, blockchain listeners, and sensor feeds. The router
/// consumes events from one or more sources and dispatches them.
pub trait EventSource {
    /// The event type produced by this source.
    type Event: Event;

    /// The stream type that yields events.
    type EventStream: amico_system::Stream<Item = Self::Event>;

    /// Subscribe to the event stream.
    fn subscribe(&self) -> Self::EventStream;
}

// ---------------------------------------------------------------------------
// Handler output
// ---------------------------------------------------------------------------

/// Output produced by an event handler invocation.
///
/// Wraps the handler's generation result so that the router (or
/// middleware) can inspect it uniformly.
#[derive(Debug, Clone)]
pub struct HandlerOutput<R> {
    /// The generation result produced by the handler.
    pub result: R,
    /// Timestamp when the handler finished.
    pub completed_at: Timestamp,
}

impl<R> HandlerOutput<R> {
    /// Create a new handler output.
    pub fn new(result: R, completed_at: Timestamp) -> Self {
        Self {
            result,
            completed_at,
        }
    }
}

// ---------------------------------------------------------------------------
// Common event types
// ---------------------------------------------------------------------------

/// Message event (e.g., from chat, social media, etc.)
#[derive(Debug, Clone)]
pub struct MessageEvent {
    pub content: String,
    pub sender: String,
    pub timestamp: Timestamp,
    pub metadata: EventMetadata,
}

impl Event for MessageEvent {
    fn event_type(&self) -> &str {
        "message"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
}

/// Timer event (scheduled execution)
#[derive(Debug, Clone)]
pub struct TimerEvent {
    pub timer_id: String,
    pub timestamp: Timestamp,
    pub metadata: EventMetadata,
}

impl Event for TimerEvent {
    fn event_type(&self) -> &str {
        "timer"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
}

/// Blockchain event (on-chain transaction or event)
#[derive(Debug, Clone)]
pub struct BlockchainEvent {
    pub chain: String,
    pub transaction_hash: String,
    pub event_data: Vec<u8>,
    pub timestamp: Timestamp,
    pub metadata: EventMetadata,
}

impl Event for BlockchainEvent {
    fn event_type(&self) -> &str {
        "blockchain"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
}

/// Sensor event (from physical or virtual sensors)
#[derive(Debug, Clone)]
pub struct SensorEvent {
    pub sensor_id: String,
    pub sensor_type: String,
    pub data: Vec<u8>,
    pub timestamp: Timestamp,
    pub metadata: EventMetadata,
}

impl Event for SensorEvent {
    fn event_type(&self) -> &str {
        "sensor"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
}
