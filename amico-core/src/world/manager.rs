use evenio::prelude::Component;

use crate::{ecs, traits::System};

use super::{EventDelegate, HandlerRegistry};

/// The ECS World manager.
///
/// This type defines how to build the ECS `World` for our
/// framework to use.
#[derive(Debug)]
pub struct WorldManager {
    world: ecs::World,

    ai_layer: ecs::EntityId,
    env_layer: ecs::EntityId,
    int_layer: ecs::EntityId,
}

impl WorldManager {
    /// Creates a `World` and spawn all the entities.
    pub fn new() -> Self {
        let mut world = ecs::World::new();
        let ai_layer = world.spawn();
        let env_layer = world.spawn();
        let int_layer = world.spawn();

        Self {
            world,
            ai_layer,
            env_layer,
            int_layer,
        }
    }

    /// Gets AI Layer's EntityId
    pub fn ai_layer(&self) -> ecs::EntityId {
        self.ai_layer
    }

    /// Gets Environment Layer's EntityId
    pub fn env_layer(&self) -> ecs::EntityId {
        self.env_layer
    }

    /// Gets Interaction Layer's EntityId
    pub fn int_layer(&self) -> ecs::EntityId {
        self.int_layer
    }

    /// Register a `System` to the world.
    pub fn register_system<S: System>(&mut self, system: S) {
        system.register_to(HandlerRegistry {
            world: &mut self.world,
        });
    }

    /// Gets an immutable reference to component C on entity.
    /// Returns None if entity doesn't exist or doesn't have the requested component.
    pub fn get<C: ecs::Component>(&self, entity: ecs::EntityId) -> Option<&C> {
        self.world.get::<C>(entity)
    }

    /// Gets a mutable reference to component C on entity.
    /// Returns None if entity doesn't exist or doesn't have the requested component.
    pub fn get_mut<C: ecs::Component<Mutability = ecs::Mutable>>(
        &mut self,
        entity: ecs::EntityId,
    ) -> Option<&mut C> {
        self.world.get_mut::<C>(entity)
    }

    /// Adds a component `C` to an entity.
    pub fn add_component<C: Component>(&mut self, entity: ecs::EntityId, component: C) {
        self.world.insert(entity, component);
    }

    pub(crate) fn event_delegate(&mut self) -> EventDelegate {
        EventDelegate {
            world: &mut self.world,
        }
    }
}

impl Default for WorldManager {
    fn default() -> Self {
        Self::new()
    }
}
