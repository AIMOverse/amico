//! Event handlers for both sync and async operations

use core::future::Future;
use core::pin::Pin;
use alloc::boxed::Box;
use alloc::vec::Vec;
use std::collections::HashMap;

use crate::events::types::*;

/// A boxed future for async event handling
pub type AsyncHandlerFuture = Pin<Box<dyn Future<Output = EventResult<()>> + Send + 'static>>;

/// Trait for synchronous event handlers
pub trait SyncHandler<E>: Send + Sync + 'static
where
    E: GlobalEvent,
{
    /// Handle the event synchronously
    fn handle(&self, event: &E) -> EventResult<()>;
}

/// Trait for asynchronous event handlers
pub trait AsyncHandler<E>: Send + Sync + 'static
where
    E: GlobalEvent,
{
    /// Handle the event asynchronously
    fn handle(&self, event: E) -> AsyncHandlerFuture;
}

/// Trait for mediator handlers that can consume events and produce new ones
pub trait MediatorHandler<E, S>: Send + Sync + 'static
where
    E: GlobalEvent,
    S: EventSet,
{
    /// Mediate the event, potentially producing new events
    fn mediate(&self, event: E, sender: &mut dyn EventSender) -> EventResult<()>;
}

/// A type-erased handler that can be stored in collections
pub enum TypeErasedHandler {
    /// Synchronous handler
    Sync(Box<dyn Fn(&dyn AsAny) -> EventResult<()> + Send + Sync + 'static>),
    /// Asynchronous handler
    Async(Box<dyn Fn(Box<dyn AsAny>) -> AsyncHandlerFuture + Send + Sync + 'static>),
    /// Mediator handler
    Mediator(Box<dyn Fn(Box<dyn AsAny>, &mut dyn EventSender) -> EventResult<()> + Send + Sync + 'static>),
}

impl std::fmt::Debug for TypeErasedHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeErasedHandler::Sync(_) => write!(f, "TypeErasedHandler::Sync"),
            TypeErasedHandler::Async(_) => write!(f, "TypeErasedHandler::Async"),
            TypeErasedHandler::Mediator(_) => write!(f, "TypeErasedHandler::Mediator"),
        }
    }
}

/// Storage for event handlers
#[derive(Debug)]
pub struct HandlerStorage {
    /// Global event handlers indexed by event type
    global_handlers: HashMap<EventTypeId, Vec<TypeErasedHandler>>,
    /// Targeted event handlers indexed by event type
    targeted_handlers: HashMap<EventTypeId, Vec<TypeErasedHandler>>,
}

impl HandlerStorage {
    /// Create a new handler storage
    pub fn new() -> Self {
        Self {
            global_handlers: HashMap::new(),
            targeted_handlers: HashMap::new(),
        }
    }

    /// Add a synchronous global event handler
    pub fn add_sync_global_handler<E, H>(&mut self, handler: H)
    where
        E: GlobalEvent + 'static,
        H: SyncHandler<E>,
    {
        let type_id = core::any::TypeId::of::<E>();
        let erased_handler = TypeErasedHandler::Sync(Box::new(move |event| {
            if let Some(concrete_event) = event.as_any().downcast_ref::<E>() {
                handler.handle(concrete_event)
            } else {
                Err(EventError::EventTypeNotFound)
            }
        }));

        self.global_handlers
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(erased_handler);
    }

    /// Add an asynchronous global event handler
    pub fn add_async_global_handler<E, H>(&mut self, handler: H)
    where
        E: GlobalEvent + 'static,
        H: AsyncHandler<E>,
    {
        let type_id = core::any::TypeId::of::<E>();
        let erased_handler = TypeErasedHandler::Async(Box::new(move |event| {
            if let Ok(concrete_event) = event.into_any().downcast::<E>() {
                handler.handle(*concrete_event)
            } else {
                Box::pin(async { Err(EventError::EventTypeNotFound) })
            }
        }));

        self.global_handlers
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(erased_handler);
    }

    /// Add a mediator handler
    pub fn add_mediator_handler<E, S, H>(&mut self, handler: H)
    where
        E: GlobalEvent + 'static,
        S: EventSet,
        H: MediatorHandler<E, S>,
    {
        let type_id = core::any::TypeId::of::<E>();
        let erased_handler = TypeErasedHandler::Mediator(Box::new(move |event, sender| {
            if let Ok(concrete_event) = event.into_any().downcast::<E>() {
                handler.mediate(*concrete_event, sender)
            } else {
                Err(EventError::EventTypeNotFound)
            }
        }));

        self.global_handlers
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(erased_handler);
    }

    /// Get handlers for a specific event type
    pub fn get_global_handlers(&self, event_type: EventTypeId) -> Option<&Vec<TypeErasedHandler>> {
        self.global_handlers.get(&event_type)
    }

    /// Get targeted handlers for a specific event type
    pub fn get_targeted_handlers(&self, event_type: EventTypeId) -> Option<&Vec<TypeErasedHandler>> {
        self.targeted_handlers.get(&event_type)
    }
}

impl Default for HandlerStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Implement SyncHandler for closures
impl<F, E> SyncHandler<E> for F
where
    F: Fn(&E) -> EventResult<()> + Send + Sync + 'static,
    E: GlobalEvent,
{
    fn handle(&self, event: &E) -> EventResult<()> {
        self(event)
    }
}

/// Implement AsyncHandler for closures that return futures
impl<F, Fut, E> AsyncHandler<E> for F
where
    F: Fn(E) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = EventResult<()>> + Send + 'static,
    E: GlobalEvent,
{
    fn handle(&self, event: E) -> AsyncHandlerFuture {
        Box::pin(self(event))
    }
}

/// Implement MediatorHandler for closures
impl<F, E, S> MediatorHandler<E, S> for F
where
    F: Fn(E, &mut dyn EventSender) -> EventResult<()> + Send + Sync + 'static,
    E: GlobalEvent,
    S: EventSet,
{
    fn mediate(&self, event: E, sender: &mut dyn EventSender) -> EventResult<()> {
        self(event, sender)
    }
}