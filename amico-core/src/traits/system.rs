use crate::world::HandlerRegistry;

/// An ECS System (also known as controller) to be registered
/// to an Agent.
pub trait System {
    /// Register one or some handler functions to the ECS World.
    fn register_to(&self, registry: HandlerRegistry);
}
