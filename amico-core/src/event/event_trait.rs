//! Event system for handling and managing various types of events in the AMICO framework.
//!
//! This module provides a flexible and thread-safe event system that allows for:
//! - Custom event types with specific trigger conditions
//! - Event handlers that can respond to events
//! - A global registry for managing events and handlers
//! - Integration with the AI prompt system
//!
//! # Examples
//!
//! ## Creating a Custom Event
//!
//! ```rust
//! use std::any::Any;
//! use std::sync::Arc;
//! use amico_core::prompt::Prompt;
//! use amico_core::event::{Event, EventHandler, EventRegistry};
//!
//! // Define a custom event for monitoring file changes
//! struct FileChangeEvent {
//!     path: String,
//!     operation: String,
//! }
//!
//! impl Event for FileChangeEvent {
//!     fn event_type(&self) -> &'static str {
//!         "file_change"
//!     }
//!
//!     fn should_trigger(&self) -> bool {
//!         true  // This event triggers immediately when created
//!     }
//!
//!     fn context(&self) -> Box<dyn Any + Send + Sync> {
//!         Box::new((self.path.clone(), self.operation.clone()))
//!     }
//!
//!     fn clone_box(&self) -> Box<dyn Event> {
//!         Box::new(Self {
//!             path: self.path.clone(),
//!             operation: self.operation.clone(),
//!         })
//!     }
//! }
//!
//! impl Prompt for FileChangeEvent {
//!     fn to_prompt(&self) -> String {
//!         format!("File {} was {}", self.path, self.operation)
//!     }
//! }
//!
//! // Create a handler
//! struct FileLogger;
//!
//! impl EventHandler for FileLogger {
//!     fn handle(&self, event: Box<dyn Event>) {
//!         if let Some((path, operation)) = event.context().downcast_ref::<(String, String)>() {
//!             println!("Logged: {} was {}", path, operation);
//!         }
//!     }
//! }
//!
//! // Create a registry and register the handler
//! let registry = EventRegistry::new();
//! registry.register_handler("file_change", Arc::new(FileLogger));
//!
//! // Create and trigger an event
//! let event = FileChangeEvent {
//!     path: "config.json".to_string(),
//!     operation: "modified".to_string(),
//! };
//!
//! registry.trigger_event(Box::new(event));
//! ```
//!
//! # Thread Safety
//!
//! The event system is designed to be thread-safe:
//! - Events must implement `Send + Sync`
//! - The registry uses `RwLock` for thread-safe access
//! - Event handlers are wrapped in `Arc` for safe sharing
//! - Broadcast channels are used for async event distribution
//!
//! # Integration with AI
//!
//! Events implement the `Prompt` trait, allowing them to be seamlessly integrated
//! with AI systems. The event context can be converted into natural language
//! prompts that describe the event in a way that AI models can understand.

use crate::prompt::Prompt;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

/// A trait for defining events that can be triggered and handled by the event system.
///
/// Events must be thread-safe (`Send + Sync`) and implement the `Prompt` trait for AI integration.
/// Each event has a unique type identifier, trigger conditions, and associated context data.
///
/// # Example
///
/// ```rust
/// use std::any::Any;
/// use amico_core::prompt::Prompt;
/// use amico_core::event::Event;
///
/// struct TemperatureEvent {
///     temperature: f64,
///     location: String,
/// }
///
/// impl Event for TemperatureEvent {
///     fn event_type(&self) -> &'static str {
///         "temperature_change"
///     }
///
///     fn should_trigger(&self) -> bool {
///         self.temperature > 30.0  // Trigger when temperature exceeds 30°C
///     }
///
///     fn context(&self) -> Box<dyn Any + Send + Sync> {
///         Box::new((self.temperature, self.location.clone()))
///     }
///
///     fn clone_box(&self) -> Box<dyn Event> {
///         Box::new(Self {
///             temperature: self.temperature,
///             location: self.location.clone(),
///         })
///     }
/// }
///
/// impl Prompt for TemperatureEvent {
///     fn to_prompt(&self) -> String {
///         format!("Temperature at {} is {}°C", self.location, self.temperature)
///     }
/// }
/// ```
pub trait Event: Send + Sync + Prompt {
    /// Returns a unique identifier for this event type.
    ///
    /// This identifier is used to match events with their handlers in the registry.
    fn event_type(&self) -> &'static str;

    /// Determines whether the event should be triggered based on its conditions.
    ///
    /// This method allows events to implement custom trigger logic. For example:
    /// - Time-based triggers (trigger after a specific time)
    /// - Threshold-based triggers (trigger when a value exceeds a threshold)
    /// - State-based triggers (trigger when a specific state is reached)
    fn should_trigger(&self) -> bool;

    /// Returns the event's context data as a type-erased object.
    ///
    /// The context can contain any data relevant to the event, which handlers
    /// can downcast to the appropriate type using `downcast_ref`.
    fn context(&self) -> Box<dyn Any + Send + Sync>;

    /// Creates a clone of the event as a boxed trait object.
    ///
    /// This is necessary because `Clone` cannot be implemented directly for
    /// trait objects. Each event implementation must provide its own cloning logic.
    fn clone_box(&self) -> Box<dyn Event>;
}

/// A trait for handling events registered in the event system.
///
/// Event handlers receive events when they are triggered and can perform
/// custom actions based on the event's context.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use amico_core::prompt::Prompt;
/// use amico_core::event::{Event, EventHandler, EventRegistry};
///
/// struct AlertHandler {
///     threshold: f64,
/// }
///
/// impl EventHandler for AlertHandler {
///     fn handle(&self, event: Box<dyn Event>) {
///         if let Some((temp, location)) = event.context().downcast_ref::<(f64, String)>() {
///             if *temp > self.threshold {
///                 println!("Alert: High temperature ({:.1}°C) at {}", temp, location);
///             }
///         }
///     }
/// }
///
/// // Create a registry and register the handler
/// let registry = EventRegistry::new();
/// registry.register_handler(
///     "temperature_change",
///     Arc::new(AlertHandler { threshold: 35.0 })
/// );
/// ```
pub trait EventHandler: Send + Sync {
    /// Handle an event that has been triggered.
    ///
    /// This method is called when an event matching the handler's registered
    /// event type is triggered. The handler can access the event's context
    /// data and perform appropriate actions.
    fn handle(&self, event: Box<dyn Event>);
}

/// A registry for managing events and their handlers.
///
/// The registry maintains thread-safe collections of event handlers and broadcast
/// channels for each event type. It provides methods for registering handlers
/// and triggering events.
///
/// # Thread Safety
///
/// The registry uses `RwLock` for thread-safe access to its internal collections
/// and `Arc` for sharing handlers across threads.
#[derive(Default)]
pub struct EventRegistry {
    handlers: RwLock<HashMap<&'static str, Vec<Arc<dyn EventHandler>>>>,
    event_channels: RwLock<HashMap<&'static str, broadcast::Sender<Box<dyn Event>>>>,
}

impl EventRegistry {
    /// Creates a new empty event registry.
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
            event_channels: RwLock::new(HashMap::new()),
        }
    }

    /// Registers a new event handler for a specific event type.
    ///
    /// # Arguments
    ///
    /// * `event_type` - The type identifier of events this handler should receive
    /// * `handler` - The handler implementation wrapped in an `Arc`
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::sync::Arc;
    /// use amico_core::prompt::Prompt;
    /// use amico_core::event::{Event, EventHandler, EventRegistry};
    ///
    /// struct LogHandler;
    /// impl EventHandler for LogHandler {
    ///     fn handle(&self, event: Box<dyn Event>) {
    ///         println!("Event occurred: {}", event.to_prompt());
    ///     }
    /// }
    ///
    /// let registry = EventRegistry::new();
    /// registry.register_handler("my_event", Arc::new(LogHandler));
    /// ```
    pub fn register_handler(&self, event_type: &'static str, handler: Arc<dyn EventHandler>) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.entry(event_type).or_default().push(handler);
    }

    /// Triggers an event and notifies all registered handlers.
    ///
    /// This method checks if the event should be triggered using its `should_trigger`
    /// method. If true, it:
    /// 1. Sends the event through the broadcast channel for async handling
    /// 2. Notifies all registered handlers for immediate handling
    ///
    /// # Arguments
    ///
    /// * `event` - The event to trigger
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::any::Any;
    /// use amico_core::prompt::Prompt;
    /// use amico_core::event::{Event, EventRegistry};
    /// struct MyEvent { message: String }
    /// impl Event for MyEvent {
    ///     fn event_type(&self) -> &'static str { "my_event" }
    ///     fn should_trigger(&self) -> bool { true }
    ///     fn context(&self) -> Box<dyn Any + Send + Sync> { Box::new(self.message.clone()) }
    ///     fn clone_box(&self) -> Box<dyn Event> { Box::new(Self { message: self.message.clone() }) }
    /// }
    /// impl Prompt for MyEvent {
    ///     fn to_prompt(&self) -> String { self.message.clone() }
    /// }
    /// let registry = EventRegistry::new();
    /// let event = MyEvent { message: "Something happened".to_string() };
    /// registry.trigger_event(Box::new(event));
    /// ```
    pub fn trigger_event(&self, event: Box<dyn Event>) {
        if !event.should_trigger() {
            return;
        }

        let event_type = event.event_type();

        // Send event through broadcast channel
        if let Some(sender) = self.event_channels.read().unwrap().get(event_type) {
            let _ = sender.send(event.clone_box());
        }

        // Notify handlers
        if let Some(handlers) = self.handlers.read().unwrap().get(event_type) {
            for handler in handlers {
                handler.handle(event.clone_box());
            }
        }
    }
}

// Create a global event registry
lazy_static::lazy_static! {
    /// Global instance of the event registry.
    ///
    /// This registry is available throughout the application and can be used
    /// to register handlers and trigger events from any context.
    pub static ref GLOBAL_REGISTRY: EventRegistry = EventRegistry::new();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    /// Example time-based event implementation for testing
    struct TimeEvent {
        trigger_time: SystemTime,
        context: String,
    }

    impl TimeEvent {
        fn new(trigger_after: Duration, context: String) -> Self {
            Self {
                trigger_time: SystemTime::now() + trigger_after,
                context,
            }
        }
    }

    impl Event for TimeEvent {
        fn event_type(&self) -> &'static str {
            "time_event"
        }

        fn should_trigger(&self) -> bool {
            SystemTime::now() >= self.trigger_time
        }

        fn context(&self) -> Box<dyn Any + Send + Sync> {
            Box::new(self.context.clone())
        }

        fn clone_box(&self) -> Box<dyn Event> {
            Box::new(Self {
                trigger_time: self.trigger_time,
                context: self.context.clone(),
            })
        }
    }

    impl Prompt for TimeEvent {
        fn to_prompt(&self) -> String {
            format!(
                "Time event triggered at {:?} with context: {}",
                self.trigger_time, self.context
            )
        }
    }

    #[test]
    fn test_time_event() {
        // Create an event that should trigger immediately
        let immediate_event = TimeEvent::new(Duration::from_secs(0), "immediate".to_string());
        assert!(immediate_event.should_trigger());

        // Create an event that should not trigger yet
        let future_event = TimeEvent::new(Duration::from_secs(60), "future".to_string());
        assert!(!future_event.should_trigger());

        // Test prompt formatting
        assert!(immediate_event.to_prompt().contains("immediate"));
    }

    #[test]
    fn test_event_registry_with_time_event() {
        struct TestHandler {
            received: std::sync::atomic::AtomicBool,
        }

        impl EventHandler for TestHandler {
            fn handle(&self, event: Box<dyn Event>) {
                if let Some(context) = event.context().downcast_ref::<String>() {
                    if context == "test_context" {
                        self.received
                            .store(true, std::sync::atomic::Ordering::SeqCst);
                    }
                }
            }
        }

        let registry = EventRegistry::new();
        let handler = Arc::new(TestHandler {
            received: std::sync::atomic::AtomicBool::new(false),
        });

        registry.register_handler("time_event", handler.clone());

        let event = TimeEvent::new(Duration::from_secs(0), "test_context".to_string());
        registry.trigger_event(Box::new(event));

        assert!(handler.received.load(std::sync::atomic::Ordering::SeqCst));
    }
}
