use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};

use super::error::ConfigError;

pub trait Config: JsonSchema + DeserializeOwned + Serialize {
    const VERSION: u32;

    fn from_toml_str(s: &str) -> Result<Self, ConfigError>;
}
