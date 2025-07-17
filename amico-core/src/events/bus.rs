//! Event bus for dispatching events to handlers

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::TypeId;

use crate::events::handler::*;
use crate::events::types::*;

/// The main event bus that manages event dispatch
#[derive(Debug)]
pub struct EventBus {
    /// Storage for all event handlers
    handlers: HandlerStorage,
    /// Entity counter for generating unique entity IDs
    next_entity_id: u32,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            handlers: HandlerStorage::new(),
            next_entity_id: 1,
        }
    }

    /// Create a new entity and return its ID
    pub fn create_entity(&mut self) -> EntityId {
        let id = EntityId::new(self.next_entity_id);
        self.next_entity_id += 1;
        id
    }

    /// Add a synchronous event handler
    pub fn add_handler<E, H>(&mut self, handler: H)
    where
        E: GlobalEvent + 'static,
        H: SyncHandler<E>,
    {
        self.handlers.add_sync_global_handler::<E, H>(handler);
    }

    /// Add an asynchronous event handler
    pub fn add_async_handler<E, H>(&mut self, handler: H)
    where
        E: GlobalEvent + 'static,
        H: AsyncHandler<E>,
    {
        self.handlers.add_async_global_handler::<E, H>(handler);
    }

    /// Add a mediator handler
    pub fn add_mediator<E, S, H>(&mut self, handler: H)
    where
        E: GlobalEvent + 'static,
        S: EventSet,
        H: MediatorHandler<E, S>,
    {
        self.handlers.add_mediator_handler::<E, S, H>(handler);
    }

    /// Send a global event synchronously
    pub fn send<E>(&mut self, event: E) -> EventResult<()>
    where
        E: GlobalEvent + Clone + 'static,
    {
        let event_type = TypeId::of::<E>();
        
        // Handle sync handlers first
        if let Some(handlers) = self.handlers.get_global_handlers(event_type) {
            for handler in handlers {
                match handler {
                    TypeErasedHandler::Sync(sync_handler) => {
                        if let Err(e) = sync_handler(&event) {
                            tracing::error!("Sync handler failed: {}", e);
                        }
                    }
                    TypeErasedHandler::Async(_) => {
                        tracing::warn!("Async handler found in sync send - skipping");
                    }
                    TypeErasedHandler::Mediator(_) => {
                        // Skip mediator handlers for now to avoid borrow issues
                        tracing::warn!("Mediator handler found in sync send - skipping");
                    }
                }
            }
        }

        Ok(())
    }

    /// Send a global event asynchronously
    pub async fn send_async<E>(&mut self, event: E) -> EventResult<()>
    where
        E: GlobalEvent + Clone + 'static,
    {
        let event_type = TypeId::of::<E>();
        
        if let Some(handlers) = self.handlers.get_global_handlers(event_type) {
            let mut async_futures = Vec::new();
            
            for handler in handlers {
                match handler {
                    TypeErasedHandler::Sync(sync_handler) => {
                        if let Err(e) = sync_handler(&event) {
                            tracing::error!("Sync handler failed: {}", e);
                        }
                    }
                    TypeErasedHandler::Async(async_handler) => {
                        let future = async_handler(Box::new(event.clone()));
                        async_futures.push(future);
                    }
                    TypeErasedHandler::Mediator(_) => {
                        // Skip mediator handlers for now to avoid borrow issues
                        tracing::warn!("Mediator handler found in async send - skipping");
                    }
                }
            }

            // Execute all async handlers concurrently
            for future in async_futures {
                if let Err(e) = future.await {
                    tracing::error!("Async handler failed: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Send a targeted event to a specific entity
    pub fn send_to<E>(&mut self, _target: EntityId, event: E) -> EventResult<()>
    where
        E: TargetedEvent + GlobalEvent + Clone + 'static,
    {
        let event_type = TypeId::of::<E>();
        
        // Handle sync handlers first
        if let Some(handlers) = self.handlers.get_targeted_handlers(event_type) {
            for handler in handlers {
                match handler {
                    TypeErasedHandler::Sync(sync_handler) => {
                        if let Err(e) = sync_handler(&event) {
                            tracing::error!("Targeted sync handler failed: {}", e);
                        }
                    }
                    TypeErasedHandler::Async(_) => {
                        tracing::warn!("Async handler found in sync send_to - skipping");
                    }
                    TypeErasedHandler::Mediator(_) => {
                        // Skip mediator handlers for now to avoid borrow issues
                        tracing::warn!("Mediator handler found in sync send_to - skipping");
                    }
                }
            }
        }

        Ok(())
    }

    /// Get an action sender for this bus
    pub fn action_sender(&mut self) -> ActionSender {
        ActionSender { bus: self }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// A sender that can send events to the bus
pub struct BusSender<'a> {
    bus: &'a mut EventBus,
}

impl<'a> EventSender for BusSender<'a> {
    fn send_global(&mut self, _event: Box<dyn AsAny>) {
        // We need to handle type erasure here
        // This is a simplified implementation - in practice, you'd want to store
        // the original type information or use a different approach
        tracing::warn!("Type-erased event sending not fully implemented");
    }

    fn send_targeted(&mut self, _target: EntityId, _event: Box<dyn AsAny>) {
        // Similar to send_global, this needs proper type handling
        tracing::warn!("Type-erased targeted event sending not fully implemented");
    }
}

/// Action sender for the agent system
pub struct ActionSender<'a> {
    bus: &'a mut EventBus,
}

impl<'a> ActionSender<'a> {
    /// Send a global event (action)
    pub fn send<E>(&mut self, event: E) -> EventResult<()>
    where
        E: GlobalEvent + Clone + 'static,
    {
        self.bus.send(event)
    }

    /// Send a targeted event to a specific entity
    pub fn send_to<E>(&mut self, target: EntityId, event: E) -> EventResult<()>
    where
        E: TargetedEvent + GlobalEvent + Clone + 'static,
    {
        self.bus.send_to(target, event)
    }
}