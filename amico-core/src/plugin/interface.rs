use std::fmt::Debug;

use crate::entity::{Action, Event};

use super::error::PluginError;

/// The base trait of a plugin.
pub trait Plugin<C>: Send + Sync
where
    C: PluginConfig,
{
    /// The unique identifier of the plugin.
    fn name(&self) -> &str;

    /// Set up the plugin with the given context.
    fn setup(config: C) -> Self
    where
        Self: Sized;
}

/// The config type used to setup a plugin.
pub trait PluginConfig: Debug {
    fn toml_loader(&self) -> Option<fn(String) -> Self>
    where
        Self: Sized;
}

/// Plugins providing event sources
pub trait EventSource<C>: Plugin<C>
where
    C: PluginConfig,
{
    fn generate_event(&mut self) -> Event;
}

/// Plugins providing sensor to world environment
pub trait InputSource<C, D>: Plugin<C>
where
    C: PluginConfig,
{
    fn get_data(&self) -> D
    where
        D: Sized;
}

/// Plugins providing actuator control
pub trait Actuator<C, D, R, E>: Plugin<C>
where
    C: PluginConfig,
    E: PluginError,
{
    fn execute(&mut self, data: D) -> Result<R, E>;
}

// TODO: Wait for Event Pool implementation
pub struct EventPool;

/// Plugins selecting actions based on the current state and event pool
pub trait ActionSelector<C>: Plugin<C>
where
    C: PluginConfig,
{
    fn select_action(&self, pool: &EventPool) -> Box<dyn Action>;
}
