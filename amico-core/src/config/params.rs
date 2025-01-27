use std::collections::HashMap;

pub use amico_macros::WithParams;

pub type ParamValue = toml::Value;

pub type Params = HashMap<String, ParamValue>;
