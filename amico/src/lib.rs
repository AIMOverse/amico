//! # Amico V2 - Platform-Agnostic AI Agent Runtime
//!
//! Amico is a platform-agnostic runtime for AI agents built in Rust.
//! It provides a framework for developers to build AI agent business logic
//! similar to how web frameworks like Axum or Rocket enable web development.
//!
//! ## Architecture
//!
//! Amico V2 consists of five layers:
//!
//! 1. **Models Layer** (`amico-models`): Abstracts AI models by capability
//! 2. **System Layer** (`amico-system`): Tools and side-effects for interacting with the world
//! 3. **Runtime Layer** (`amico-runtime`): Workflow execution on different runtime types
//! 4. **Plugin Layer** (`amico-plugin`): Plugin architecture for extending agent capabilities
//! 5. **Workflows Layer** (`amico-workflows`): Preset workflow patterns
//!
//! ## Design Principles
//!
//! - **Traits + Generics**: No dynamic dispatch, compile-time polymorphism
//! - **Zero-cost Abstractions**: No runtime overhead
//! - **Platform Agnostic**: Works on OS, browsers, mobile, embedded devices
//! - **Type Safe**: Extensive compile-time verification
//! - **Extensible**: Plugin system covers all aspects of the agent lifecycle
//!
//! ## Event System
//!
//! Event sources are **streams** of events. Events carry context for agents
//! to inspect. An [`EventRouter`] dispatches received events to [`EventHandler`]s.
//! Handlers use a global context and event context to run agent workflows,
//! producing side-effects and a generation result wrapped in [`HandlerOutput`].
//!
//! ## Agent Workflows
//!
//! An atomic agent action step ([`runtime::AgentAction`]) takes a conversation
//! history and model parameters and produces an [`runtime::AgentChoice`]. A
//! workflow chains multiple steps. Observers can subscribe to the sequential
//! [`runtime::StepStream`] to inspect progress.
//!
//! ## System Side Effects
//!
//! The [`system::System`] trait bundles all platform capabilities (file I/O,
//! networking, process execution, clock, logging, entropy). Swapping the
//! `System` implementation ports an agent to a new platform without changing
//! business logic.
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

pub mod event;

// Re-export all layers
pub use amico_models as models;
pub use amico_plugin as plugin;
pub use amico_runtime as runtime;
pub use amico_system as system;
pub use amico_workflows as workflows;

// Re-export commonly used types
pub use amico_models::{LanguageInput, LanguageModel, LanguageOutput, Model};
pub use amico_plugin::{Plugin, PluginError, PluginRuntime, PluginSet, ToolPlugin};
pub use amico_runtime::{ExecutionContext, Runtime, Scheduler, Workflow};
pub use amico_system::{Observable, Permission, SystemEffect, Tool};
pub use amico_workflows::{AgentResponse, ToolLoopAgent, WorkflowError};

// Re-export event system types
pub use event::{
    BlockchainEvent, DispatchError, EventHandler, EventRouter, EventSource, HandlerOutput,
    MessageEvent, SensorEvent, TimerEvent,
};

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

/// Event trait - all events implement this.
///
/// Events carry context for agents to inspect. Each event has a type
/// identifier, a timestamp, and metadata describing its origin.
pub trait Event {
    /// Event type identifier
    fn event_type(&self) -> &str;

    /// Event timestamp
    fn timestamp(&self) -> Timestamp;

    /// Event metadata
    fn metadata(&self) -> &EventMetadata;
}

/// Plugin that provides event sources.
///
/// An `EventSourcePlugin` introduces new event streams into the runtime.
/// For example, an A2A connector plugin subscribes to an external agent
/// collaboration platform and surfaces inbound requests as events that the
/// agent developer can handle with an `EventHandler`.
pub trait EventSourcePlugin: Plugin {
    /// The event type produced by this plugin
    type ProvidedEvent: Event;

    /// The stream type that yields events
    type EventStream: amico_system::Stream<Item = Self::ProvidedEvent>;

    /// Subscribe to the plugin's event stream
    fn subscribe(&self) -> Self::EventStream;
}

/// Plugin that intercepts events before and after handling (middleware).
///
/// An `EventInterceptor` can observe or transform events at the boundary of
/// the event dispatch pipeline. Use cases include logging, authentication,
/// rate limiting, or metric collection.
pub trait EventInterceptor: Plugin {
    /// The event type this interceptor applies to
    type Event: Event;

    /// Called before the event handler processes the event
    fn before_handle<'a>(
        &'a self,
        event: &'a Self::Event,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send + 'a;

    /// Called after the event handler processes the event
    fn after_handle<'a>(
        &'a self,
        event: &'a Self::Event,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send + 'a;
}
