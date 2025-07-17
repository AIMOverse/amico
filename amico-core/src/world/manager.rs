use crate::{events::EventBus, traits::System};

use super::{ActionSender, HandlerRegistry};

/// The Event World manager.
///
/// This type defines how to build the Event `World` for our
/// framework to use.
#[derive(Debug)]
pub struct WorldManager {
    event_bus: EventBus,
}

impl WorldManager {
    /// Creates an `EventBus` and spawn all the entities.
    pub fn new() -> Self {
        let event_bus = EventBus::new();

        // TODO: Spawn all the entities. (in the future)

        Self { event_bus }
    }

    /// Register a `System` to the world.
    pub fn add_system<S: System>(&mut self, system: S) {
        system.register_to(HandlerRegistry {
            event_bus: &mut self.event_bus,
        });
    }

    /// Gets the action sender.
    pub(crate) fn action_sender(&mut self) -> ActionSender {
        ActionSender {
            event_bus: &mut self.event_bus,
        }
    }
}

impl Default for WorldManager {
    fn default() -> Self {
        Self::new()
    }
}
