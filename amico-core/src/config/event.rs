use serde::{Deserialize, Serialize};

use super::{params::Params, ConfigError, ParamValue};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventConfig {
    pub name: String,
    pub source: String,
    pub params: Option<Params>,
}

impl EventConfig {
    pub fn param(&self, key: &str) -> Result<&ParamValue, ConfigError> {
        let Some(params) = self.params.as_ref() else {
            return Err(ConfigError::ParamNotFound(key.to_string()));
        };

        params
            .get(key)
            .ok_or(ConfigError::ParamNotFound(key.to_string()))
    }
}
