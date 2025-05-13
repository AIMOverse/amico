use crate::ecs;

/// A wrapper around `&mut World` to restrict the caller to
/// access only the `add_handler` method in `World`.
#[derive(Debug)]
pub struct HandlerRegistry<'world> {
    pub(crate) world: &'world mut ecs::World,
}

impl HandlerRegistry<'_> {
    /// Register a handler to `World`.
    pub fn register<M>(&mut self, handler: impl ecs::IntoHandler<M>) {
        self.world.add_handler(handler);
    }
}

/// A wrapper around `&mut World` to restrict the caller to
/// access only the `send` method in `World`.
pub struct EventDelegate<'world> {
    pub(crate) world: &'world mut ecs::World,
}

impl EventDelegate<'_> {
    pub fn send_event<E: ecs::GlobalEvent>(&mut self, event: E) {
        self.world.send(event);
    }

    pub fn send_event_to<E: ecs::TargetedEvent>(&mut self, target: ecs::EntityId, event: E) {
        self.world.send_to(target, event);
    }
}
