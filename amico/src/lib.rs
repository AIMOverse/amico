//! # Amico V2 - Platform-Agnostic AI Agent Runtime
//!
//! Amico is a platform-agnostic runtime for AI agents built in Rust.
//! It provides a framework for developers to build AI agent business logic
//! similar to how web frameworks like Axum or Rocket enable web development.
//!
//! ## Architecture
//!
//! Amico V2 consists of four layers:
//!
//! 1. **Models Layer** (`amico-models`): Abstracts AI models by capability
//! 2. **System Layer** (`amico-system`): Tools and side-effects for interacting with the world
//! 3. **Runtime Layer** (`amico-runtime`): Workflow execution on different runtime types
//! 4. **Workflows Layer** (`amico-workflows`): Preset workflow patterns
//!
//! ## Design Principles
//!
//! - **Traits + Generics**: No dynamic dispatch, compile-time polymorphism
//! - **Zero-cost Abstractions**: No runtime overhead
//! - **Platform Agnostic**: Works on OS, browsers, mobile, embedded devices
//! - **Type Safe**: Extensive compile-time verification
//!
//! ## Example
//!
//! ```rust,ignore
//! use amico::{
//!     EventHandler, EventRouter,
//!     runtime::Runtime,
//!     workflows::ToolLoopAgent,
//! };
//!
//! struct MyAgentHandler {
//!     agent: ToolLoopAgent<MyModel, MyTools, MyContext>,
//! }
//!
//! impl EventHandler<MessageEvent> for MyAgentHandler {
//!     type Context = AgentContext;
//!     type Response = MessageResponse;
//!     type Error = HandlerError;
//!     
//!     async fn handle(&self, event: MessageEvent, context: &Self::Context)
//!         -> Result<Self::Response, Self::Error>
//!     {
//!         let response = self.agent.execute(context, event.content).await?;
//!         Ok(MessageResponse::from(response))
//!     }
//! }
//! ```

use std::future::Future;

// Re-export all layers
pub use amico_models as models;
pub use amico_system as system;
pub use amico_runtime as runtime;
pub use amico_workflows as workflows;

// Re-export commonly used types
pub use amico_models::{Model, LanguageModel, LanguageInput, LanguageOutput};
pub use amico_system::{Tool, SystemEffect, Permission, Observable};
pub use amico_runtime::{Workflow, ExecutionContext, Runtime, Scheduler};
pub use amico_workflows::{ToolLoopAgent, AgentResponse, WorkflowError};

/// Timestamp in milliseconds since epoch
pub type Timestamp = u64;

/// Event metadata
#[derive(Debug, Clone)]
pub struct EventMetadata {
    pub source: String,
    pub tags: Vec<String>,
}

impl EventMetadata {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            tags: Vec::new(),
        }
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

/// Event trait - all events implement this
pub trait Event {
    /// Event type identifier
    fn event_type(&self) -> &str;
    
    /// Event timestamp
    fn timestamp(&self) -> Timestamp;
    
    /// Event metadata
    fn metadata(&self) -> &EventMetadata;
}

/// Event handler trait - defines how to handle specific event types
pub trait EventHandler<E: Event> {
    /// Context type for the handler
    type Context;
    
    /// Response type produced by the handler
    type Response;
    
    /// Error type for handler execution
    type Error;
    
    /// Handle an event
    fn handle<'a>(
        &'a self,
        event: E,
        context: &'a Self::Context,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a;
}

/// Event dispatch error
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

/// Event router - registers and dispatches events to handlers
pub trait EventRouter {
    /// Event type that can be routed
    type Event: Event;
    
    /// Handler type
    type Handler;
    
    /// Register an event handler for a specific event type
    fn register(&mut self, event_type: impl Into<String>, handler: Self::Handler);
    
    /// Dispatch event to appropriate handler
    fn dispatch<'a>(
        &'a self,
        event: Self::Event,
    ) -> impl Future<Output = Result<(), DispatchError>> + Send + 'a;
}

/// Common event types

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
