use serde::{de::DeserializeOwned, Serialize};

use super::error::ConfigError;

pub trait Config: DeserializeOwned + Serialize {
    const VERSION: u32;
    fn from_toml_str(s: &str) -> Result<Self, ConfigError>;
}
