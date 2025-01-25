use schemars::JsonSchema;

use super::params::Params;

#[derive(JsonSchema, serde::Serialize, serde::Deserialize)]
pub struct EventConfig {
    name: String,
    source: String,
    params: Option<Params>,
}
