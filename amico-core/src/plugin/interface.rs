use crate::entity::{Action, Event};

/// The base trait of a plugin.
pub trait Plugin<C>: Send + Sync {
    /// The unique identifier of the plugin.
    fn name(&self) -> String;

    /// Set up the plugin with the given context.
    fn setup(config: C) -> Self
    where
        Self: Sized;
}

/// Plugins providing event sources
pub trait EventSource<C>: Plugin<C> {
    fn generate_event(&mut self) -> Event;
}

/// Plugins providing sensor to world environment
pub trait InputSource<C, D>: Plugin<C> {
    fn get_data(&self) -> D
    where
        D: Sized;
}

/// Plugins providing actuator control
pub trait Actuator<C, D, R, E>: Plugin<C> {
    fn execute(&mut self, data: D) -> Result<R, E>;
}

// TODO: Wait for Event Pool implementation
pub struct EventPool;

/// Plugins selecting actions based on the current state and event pool
pub trait ActionSelector<C>: Plugin<C> {
    fn select_action(&self, pool: &EventPool) -> Box<dyn Action>;
}
