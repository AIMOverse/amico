use std::{collections::HashMap, sync::Arc};

use super::{ActionPlugin, ActionSelectorPlugin, EventGeneratorPlugin, InputSource};

// Type alias for any object implementing the `Any` trait.
pub type EventGeneratorObject = Arc<dyn EventGeneratorPlugin>;
pub type InputSourceObject = Arc<dyn InputSource>;
pub type ActionSelectorObject = Arc<dyn ActionSelectorPlugin>;
pub type ActionObject = Arc<dyn ActionPlugin>;

/// A struct representing a pool of plugins.
pub struct PluginPool {
    pub(crate) event_generators: HashMap<String, EventGeneratorObject>,
    pub(crate) input_sources: HashMap<String, InputSourceObject>,
    pub(crate) action_selectors: HashMap<String, ActionSelectorObject>,
    pub(crate) actions: HashMap<String, ActionObject>,
}

/// A default implementation of the `PluginPool` struct.
impl Default for PluginPool {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginPool {
    /// Creates a new empty `PluginPool`.
    pub fn new() -> Self {
        Self {
            event_generators: HashMap::new(),
            input_sources: HashMap::new(),
            action_selectors: HashMap::new(),
            actions: HashMap::new(),
        }
    }

    /// Adds an event generator plugin to the pool.
    pub fn add_event_generator(&mut self, name: String, source: EventGeneratorObject) -> &mut Self {
        self.event_generators.insert(name, source);
        self
    }

    /// Adds an input source plugin to the pool.
    pub fn add_input_source(&mut self, name: String, source: InputSourceObject) -> &mut Self {
        self.input_sources.insert(name, source);
        self
    }

    /// Adds an action selector plugin to the pool.
    pub fn add_action_selector(&mut self, name: String, source: ActionSelectorObject) -> &mut Self {
        self.action_selectors.insert(name, source);
        self
    }

    /// Adds an action plugin to the pool.
    pub fn add_action(&mut self, name: String, source: ActionObject) -> &mut Self {
        self.actions.insert(name, source);
        self
    }
}
