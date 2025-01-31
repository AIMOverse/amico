use serde::{de::DeserializeOwned, Serialize};

pub trait PluginError: Serialize + DeserializeOwned {
    fn plugin_name(&self) -> &str;
    fn message(&self) -> &str;
}
