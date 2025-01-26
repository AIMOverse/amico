use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    agent::AgentConfig, error::ConfigError, event::EventConfig, interface::Config,
    provider::ProvidersConfig, runtime::RuntimeConfig,
};

#[derive(JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CoreConfig {
    runtime: RuntimeConfig,
    agents: Vec<AgentConfig>,
    providers: ProvidersConfig,
    events: Vec<EventConfig>,
}

impl Config for CoreConfig {
    const VERSION: u32 = 1;

    fn from_toml_str(s: &str) -> Result<Self, ConfigError> {
        toml::from_str(s).map_err(ConfigError::FailedToLoadToml)
    }
}
