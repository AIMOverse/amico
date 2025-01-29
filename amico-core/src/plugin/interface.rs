use std::sync::Arc;

/// The base trait of a plugin.
pub trait Plugin: Send + Sync {
    /// The unique identifier of the plugin.
    fn name(&self) -> &'static str;

    /// Set up the plugin with the given context.
    fn setup() -> Self where Self: Sized;
}

// Plugins providing events
pub trait EventPlugin: Plugin {}

// Plugins providing task context & workflows
pub trait TaskPlugin: Plugin {}

// Plugins providing sensor to world environment
pub trait SensorPlugin: Plugin {}

// Plugins providing actuator control
pub trait ActuatorPlugin: Plugin {}

// The interface a plugin module exposes
pub enum PluginInterface {
    Event(Arc<dyn EventPlugin>),
    Task(Arc<dyn TaskPlugin>),
    Sensor(Arc<dyn SensorPlugin>),
    Actuator(Arc<dyn ActuatorPlugin>),
}
