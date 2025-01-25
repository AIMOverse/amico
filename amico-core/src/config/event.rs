use std::collections::HashMap;

use schemars::JsonSchema;

#[derive(JsonSchema, serde::Serialize, serde::Deserialize)]
pub struct EventConfigSchema {
    name: String,
    source: String,
    params: Option<HashMap<String, EventParamType>>,
}

#[derive(JsonSchema, serde::Serialize, serde::Deserialize)]
pub enum EventParamType {
    String,
    Number,
    Boolean,
}
