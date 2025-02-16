use serde::{Deserialize, Serialize};

/// Every plugin must implement the `Plugin` trait to get necessary information.
pub trait Plugin {
    fn info(&self) -> &'static PluginInfo;
}

/// The information of a plugin
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct PluginInfo {
    pub name: &'static str,
    pub category: PluginCategory,
}

/// The category of a plugin
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PluginCategory {
    // High level
    Sensor,
    Effector,
    Service,
    Api,

    // Low level
    Embedding,
    EventGenerator,
    ActionSelector,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_plugin() {
        // Implement the interface
        struct TestPlugin;

        impl Plugin for TestPlugin {
            fn info(&self) -> &'static PluginInfo {
                &PluginInfo {
                    name: "TestPlugin",
                    category: PluginCategory::Sensor,
                }
            }
        }

        let plugin = TestPlugin;
        assert_eq!(plugin.info().name, "TestPlugin");
        assert_eq!(plugin.info().category, PluginCategory::Sensor);
    }
}
