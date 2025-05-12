use crate::ecs;

/// The ECS World manager.
///
/// This type defines how to build the ECS `World` for our
/// framework to use.
pub struct WorldManager {
    world: ecs::World,

    ai_layer: ecs::EntityId,
    env_layer: ecs::EntityId,
    int_layer: ecs::EntityId,
}

impl WorldManager {
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

    pub fn ai_layer(&self) -> ecs::EntityId {
        self.ai_layer
    }

    pub fn env_layer(&self) -> ecs::EntityId {
        self.env_layer
    }

    pub fn int_layer(&self) -> ecs::EntityId {
        self.int_layer
    }
}
