use serde::{Deserialize, Serialize};

use super::runtime::RuntimeConfig;
use crate::config::core::event_config::EventConfig;
use crate::config::{Config, ConfigError};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CoreConfig {
    pub name: String,
    pub version: u32,
    pub runtime: RuntimeConfig,
    pub plugins: Vec<String>,
    pub event_config: EventConfig,
}

impl Config for CoreConfig {
    const VERSION: u32 = 0;

    fn from_toml_str(s: &str) -> Result<Self, ConfigError> {
        toml::from_str(s).map_err(ConfigError::FailedToLoadToml)
    }
}
