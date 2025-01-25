use std::io::Read;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    agent::AgentConfig, error::ConfigError, event::EventConfig, interface::Config,
    provider::ProvidersConfig, runtime::RuntimeConfig,
};

#[derive(JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BuiltinConfig {
    runtime: RuntimeConfig,
    agents: Vec<AgentConfig>,
    providers: ProvidersConfig,
    events: Vec<EventConfig>,
}

impl Config for BuiltinConfig {
    const VERSION: u32 = 1;

    fn load<R: Read>(reader: R) -> Result<Self, ConfigError> {
        serde_json::from_reader(reader).map_err(ConfigError::FailedToLoad)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        todo!()
    }
}
