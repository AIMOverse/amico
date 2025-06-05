use crate::{ecs, traits::System};

use super::{ActionSender, HandlerRegistry};

/// The ECS World manager.
///
/// This type defines how to build the ECS `World` for our
/// framework to use.
#[derive(Debug)]
pub struct WorldManager {
    world: ecs::World,
}

impl WorldManager {
    /// Creates a `World` and spawn all the entities.
    pub fn new() -> Self {
        let world = ecs::World::new();

        // TODO: Spawn all the entities. (in the future)

        Self { world }
    }

    /// Register a `System` to the world.
    pub fn add_system<S: System>(&mut self, system: S) {
        system.register_to(HandlerRegistry {
            world: &mut self.world,
        });
    }

    /// Gets the action sender.
    pub(crate) fn action_sender(&mut self) -> ActionSender {
        ActionSender {
            world: &mut self.world,
        }
    }
}

impl Default for WorldManager {
    fn default() -> Self {
        Self::new()
    }
}
