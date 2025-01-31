use std::{any::Any, collections::HashMap, sync::Arc};

use super::{error::PluginError, ActionSelector, Actuator, EventSource, InputSource, PluginConfig};

pub type EventSourceObject = Arc<dyn EventSource<dyn PluginConfig>>;
pub type InputSourceObject = Arc<dyn InputSource<dyn PluginConfig, dyn Any>>;
pub type ActionSelectorObject = Arc<dyn ActionSelector<dyn PluginConfig>>;
pub type ActuatorObject = Arc<dyn Actuator<dyn PluginConfig, dyn Any, dyn Any, dyn PluginError>>;

pub struct PluginPool {
    event_sources: HashMap<String, EventSourceObject>,
    inputs: HashMap<String, InputSourceObject>,
    action_selectors: HashMap<String, ActionSelectorObject>,
    actuators: HashMap<String, ActuatorObject>,
}

impl Default for PluginPool {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginPool {
    pub fn new() -> Self {
        Self {
            event_sources: HashMap::new(),
            inputs: HashMap::new(),
            action_selectors: HashMap::new(),
            actuators: HashMap::new(),
        }
    }

    pub fn add_event_source(mut self, name: String, source: EventSourceObject) -> Self {
        self.event_sources.insert(name, source);
        self
    }

    pub fn add_input(mut self, name: String, source: InputSourceObject) -> Self {
        self.inputs.insert(name, source);
        self
    }

    pub fn add_action_selector(mut self, name: String, source: ActionSelectorObject) -> Self {
        self.action_selectors.insert(name, source);
        self
    }

    pub fn add_actuator(mut self, name: String, source: ActuatorObject) -> Self {
        self.actuators.insert(name, source);
        self
    }
}
