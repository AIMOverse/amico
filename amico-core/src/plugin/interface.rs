use std::any::Any;

use crate::entities::Event;
use crate::traits::Action;

use super::error::PluginError;

/// The base trait of a plugin.
pub trait Plugin {
    /// The unique identifier of the plugin.
    ///
    /// Not to be confused with plugin instance names
    /// in `PluginPool`s.
    fn name(&self) -> String;

    /// Set up the plugin with the given context.
    fn setup(config: &dyn Any) -> Result<Self, PluginError>
    where
        Self: Sized;
}

/// Plugins providing event sources
pub trait EventSource: Plugin {
    fn generate_event(&mut self) -> Event;
}

/// Plugins providing sensor to world environment
pub trait InputSource: Plugin {
    fn get_data(&self) -> Box<dyn Any>;
}

/// Plugins providing actuator control
pub trait Actuator: Plugin {
    fn execute(&mut self, data: &dyn Any) -> Result<Box<dyn Any>, PluginError>;
}

// TODO: Wait for Event Pool implementation
pub struct EventPool;

/// Plugins selecting actions based on the current state and event pool
pub trait ActionSelector: Plugin {
    fn select_action(&self, pool: &EventPool) -> Box<dyn Action>;
}
