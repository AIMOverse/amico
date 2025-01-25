use schemars::JsonSchema;

#[derive(JsonSchema, serde::Serialize, serde::Deserialize)]
pub struct AgentConfig {
    name: String,
    system_prompt: String,
    provider: String,
    model: String,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
}
