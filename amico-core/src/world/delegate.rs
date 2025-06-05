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

/// Sends ECS events (Actions for the Agent) to the ECS `World`.
///
/// A wrapper around `&mut World` to restrict the caller to
/// access only the `send` method in `World`.
pub struct ActionSender<'world> {
    pub(crate) world: &'world mut ecs::World,
}

impl ActionSender<'_> {
    /// Send an ECS event to the ECS `World`.
    pub fn send<E: ecs::GlobalEvent>(&mut self, event: E) {
        self.world.send(event);
    }

    /// Send an ECS event to a specific entity in the ECS `World`.
    pub fn send_to<E: ecs::TargetedEvent>(&mut self, target: ecs::EntityId, event: E) {
        self.world.send_to(target, event);
    }
}
