use crate::events::{EventBus, GlobalEvent, TargetedEvent, EntityId, EventResult};

/// A wrapper around `&mut EventBus` to restrict the caller to
/// access only the `add_handler` method in `EventBus`.
#[derive(Debug)]
pub struct HandlerRegistry<'world> {
    pub(crate) event_bus: &'world mut EventBus,
}

impl HandlerRegistry<'_> {
    /// Register a synchronous handler to `EventBus`.
    pub fn register<E, H>(&mut self, handler: H)
    where
        E: GlobalEvent + 'static,
        H: crate::events::SyncHandler<E>,
    {
        self.event_bus.add_handler(handler);
    }

    /// Register an asynchronous handler to `EventBus`.
    pub fn register_async<E, H>(&mut self, handler: H)
    where
        E: GlobalEvent + 'static,
        H: crate::events::AsyncHandler<E>,
    {
        self.event_bus.add_async_handler(handler);
    }

    /// Register a mediator handler to `EventBus`.
    pub fn register_mediator<E, S, H>(&mut self, handler: H)
    where
        E: GlobalEvent + 'static,
        S: crate::events::EventSet,
        H: crate::events::MediatorHandler<E, S>,
    {
        self.event_bus.add_mediator(handler);
    }
}

/// Sends events (Actions for the Agent) to the `EventBus`.
///
/// A wrapper around `&mut EventBus` to restrict the caller to
/// access only the `send` method in `EventBus`.
pub struct ActionSender<'world> {
    pub(crate) event_bus: &'world mut EventBus,
}

impl ActionSender<'_> {
    /// Send a global event to the `EventBus`.
    pub fn send<E>(&mut self, event: E) -> EventResult<()>
    where
        E: GlobalEvent + Clone + 'static,
    {
        self.event_bus.send(event)
    }

    /// Send an event to a specific entity in the `EventBus`.
    pub fn send_to<E>(&mut self, target: EntityId, event: E) -> EventResult<()>
    where
        E: TargetedEvent + GlobalEvent + Clone + 'static,
    {
        self.event_bus.send_to(target, event)
    }
}
