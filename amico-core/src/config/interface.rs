use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use std::io::Read;

use super::error::ConfigError;

pub trait Config: JsonSchema + DeserializeOwned + Serialize {
    const VERSION: u32;

    fn load<R: Read>(reader: R) -> Result<Self, ConfigError>;
    fn validate(&self) -> Result<(), ConfigError>;
}
