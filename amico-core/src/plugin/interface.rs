use std::any::Any;

use crate::traits::{Action, ActionSelector, EventGenerator};

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
pub trait EventGeneratorPlugin: Plugin + EventGenerator {}

/// Plugins providing sensor to world environment
pub trait InputSource: Plugin {
    fn get_data(&self) -> Box<dyn Any>;
}

/// Plugins providing actuator control
pub trait ActionPlugin: Plugin + Action {}

/// Plugins selecting actions based on the current state and event pool
pub trait ActionSelectorPlugin: Plugin + ActionSelector {}
