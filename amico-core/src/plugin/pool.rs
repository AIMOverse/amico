use std::{any::Any, collections::HashMap, error::Error, sync::Arc};

use super::{ActionSelector, Actuator, EventSource, InputSource};

pub struct PluginPool {
    pub event_sources: HashMap<String, Arc<dyn EventSource<dyn Any>>>,
    pub inputs: HashMap<String, Arc<dyn InputSource<dyn Any, dyn Any>>>,
    pub action_selectors: HashMap<String, Arc<dyn ActionSelector<dyn Any>>>,
    pub actuators: HashMap<String, Arc<dyn Actuator<dyn Any, dyn Any, dyn Any, dyn Error>>>,
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

    pub fn add_event_source(mut self, name: String, source: Arc<dyn EventSource<dyn Any>>) -> Self {
        self.event_sources.insert(name, source);
        self
    }

    pub fn add_input(
        mut self,
        name: String,
        source: Arc<dyn InputSource<dyn Any, dyn Any>>,
    ) -> Self {
        self.inputs.insert(name, source);
        self
    }

    pub fn add_action_selector(
        mut self,
        name: String,
        source: Arc<dyn ActionSelector<dyn Any>>,
    ) -> Self {
        self.action_selectors.insert(name, source);
        self
    }

    pub fn add_actuator(
        mut self,
        name: String,
        source: Arc<dyn Actuator<dyn Any, dyn Any, dyn Any, dyn Error>>,
    ) -> Self {
        self.actuators.insert(name, source);
        self
    }
}
