use serde::{Deserialize, Serialize};

use crate::config::{Config, ConfigError};

use super::{
    agent::AgentConfig, event::EventConfig, provider::ProvidersConfig, runtime::RuntimeConfig,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CoreConfig {
    pub version: u32,
    pub runtime: RuntimeConfig,
    pub agents: Vec<AgentConfig>,
    pub providers: ProvidersConfig,
    pub events: Vec<EventConfig>,
}

impl Config for CoreConfig {
    const VERSION: u32 = 0;

    fn from_toml_str(s: &str) -> Result<Self, ConfigError> {
        toml::from_str(s).map_err(ConfigError::FailedToLoadToml)
    }
}
