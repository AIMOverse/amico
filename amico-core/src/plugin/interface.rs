use crate::entity::{Action, Event};

use super::error::PluginError;

/// The base trait of a plugin.
pub trait Plugin {
    type Config: Sized;

    /// The unique identifier of the plugin.
    fn name(&self) -> String;

    /// Set up the plugin with the given context.
    fn setup(config: Self::Config) -> Self
    where
        Self: Sized;
}

/// Plugins providing event sources
pub trait EventSource: Plugin {
    fn generate_event(&mut self) -> Event;
}

/// Plugins providing sensor to world environment
pub trait InputSource: Plugin {
    type Data: Sized;
    fn get_data(&self) -> Self::Data;
}

/// Plugins providing actuator control
pub trait Actuator: Plugin {
    type Data: Sized;
    type Result: Sized;
    type Error: PluginError;
    fn execute(&mut self, data: Self::Data) -> Result<Self::Result, Self::Error>;
}

// TODO: Wait for Event Pool implementation
pub struct EventPool;

/// Plugins selecting actions based on the current state and event pool
pub trait ActionSelector: Plugin {
    fn select_action(&self, pool: &EventPool) -> Box<dyn Action>;
}
