use std::{any::Any, collections::HashMap, sync::Arc};

use super::{ActionSelector, Actuator, EventSource, InputSource};

pub type AnyObject = Box<dyn Any>;
pub type EventSourceObject = Arc<dyn EventSource>;
pub type InputSourceObject = Arc<dyn InputSource>;
pub type ActionSelectorObject = Arc<dyn ActionSelector>;
pub type ActuatorObject = Arc<dyn Actuator>;

pub struct PluginPool {
    pub(crate) event_sources: HashMap<String, EventSourceObject>,
    pub(crate) inputs: HashMap<String, InputSourceObject>,
    pub(crate) action_selectors: HashMap<String, ActionSelectorObject>,
    pub(crate) actuators: HashMap<String, ActuatorObject>,
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

    pub fn add_event_source(&mut self, name: String, source: EventSourceObject) -> &mut Self {
        self.event_sources.insert(name, source);
        self
    }

    pub fn add_input(&mut self, name: String, source: InputSourceObject) -> &mut Self {
        self.inputs.insert(name, source);
        self
    }

    pub fn add_action_selector(&mut self, name: String, source: ActionSelectorObject) -> &mut Self {
        self.action_selectors.insert(name, source);
        self
    }

    pub fn add_actuator(&mut self, name: String, source: ActuatorObject) -> &mut Self {
        self.actuators.insert(name, source);
        self
    }
}
