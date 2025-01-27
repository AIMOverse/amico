use std::collections::HashMap;

pub type ParamValue = toml::Value;

pub type Params = HashMap<String, ParamValue>;
