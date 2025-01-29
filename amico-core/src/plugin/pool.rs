use std::{collections::HashMap, sync::Arc};

use super::{ActuatorPlugin, EventPlugin, SensorPlugin, TaskPlugin};

pub struct PluginPool {
    pub events: HashMap<String, Arc<Box<dyn EventPlugin>>>,
    pub tasks: HashMap<String, Arc<Box<dyn TaskPlugin>>>,
    pub sensors: HashMap<String, Arc<Box<dyn SensorPlugin>>>,
    pub actuators: HashMap<String, Arc<Box<dyn ActuatorPlugin>>>,
}

impl PluginPool {
    pub fn new(plugin_names: Vec<&str>) -> Self {
        Self {
            events: HashMap::new(),
            tasks: HashMap::new(),
            sensors: HashMap::new(),
            actuators: HashMap::new(),
        }
    }
}
