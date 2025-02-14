use std::collections::HashMap;
use std::sync::Arc;

use crate::interface::ProviderPlugin;

/// This is where the plugin storage is stored.
pub struct PluginStorage {
    // Providers are immutable
    pub providers: HashMap<String, Arc<dyn ProviderPlugin>>,
}
