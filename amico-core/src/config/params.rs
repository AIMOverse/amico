use std::collections::HashMap;

pub use core_macros::WithParams;

pub type ParamValue = toml::Value;

pub type Params = HashMap<String, ParamValue>;
