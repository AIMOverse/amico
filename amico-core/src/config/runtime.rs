use schemars::JsonSchema;

#[derive(JsonSchema, serde::Serialize, serde::Deserialize)]
pub enum RuntimeConfig {
    Standalone,
    // Farm,
}
