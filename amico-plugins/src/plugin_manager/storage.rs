use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::interface::{ActionSelectorPlugin, ProviderPlugin};

/// This is where the plugins are actually owned and stored.
pub struct PluginStorage {
    // Providers are immutable
    pub providers: HashMap<String, Arc<dyn ProviderPlugin>>,

    // Action selectors are mutable
    pub action_selectors: HashMap<String, Arc<Mutex<dyn ActionSelectorPlugin>>>,
}

impl Default for PluginStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginStorage {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            action_selectors: HashMap::new(),
        }
    }
}
