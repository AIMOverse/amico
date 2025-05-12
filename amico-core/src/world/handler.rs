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
