use std::fmt::Debug;

pub trait PluginError: Debug {
    fn plugin_name(&self) -> &str;
    fn message(&self) -> &str;
}
