use std::{any::Any, collections::HashMap, sync::Arc};

use super::{error::PluginError, ActionSelector, Actuator, EventSource, InputSource};

pub type AnyObject = Box<dyn Any>;
pub type EventSourceObject = Arc<dyn EventSource<Config = AnyObject>>;
pub type InputSourceObject = Arc<dyn InputSource<Config = AnyObject, Data = AnyObject>>;
pub type ActionSelectorObject = Arc<dyn ActionSelector<Config = AnyObject>>;
pub type ActuatorObject = Arc<
    dyn Actuator<
        Config = AnyObject,
        Data = AnyObject,
        Result = AnyObject,
        Error = Box<dyn PluginError>,
    >,
>;

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

    pub fn add_event_source(mut self, source: EventSourceObject) -> Self {
        self.event_sources.insert(source.name(), source);
        self
    }

    pub fn add_input(mut self, source: InputSourceObject) -> Self {
        self.inputs.insert(source.name(), source);
        self
    }

    pub fn add_action_selector(mut self, source: ActionSelectorObject) -> Self {
        self.action_selectors.insert(source.name(), source);
        self
    }

    pub fn add_actuator(mut self, source: ActuatorObject) -> Self {
        self.actuators.insert(source.name(), source);
        self
    }
}
