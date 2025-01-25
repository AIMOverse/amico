use std::collections::HashMap;

use schemars::JsonSchema;

pub type Params = HashMap<String, ParamValue>;

#[derive(JsonSchema, serde::Serialize, serde::Deserialize)]
pub enum ParamValue {
    Int(i64),
    String(String),
    Number(f64),
    Boolean(bool),
}
