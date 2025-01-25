use schemars::JsonSchema;

#[derive(JsonSchema, serde::Serialize, serde::Deserialize)]
pub struct AgentConfigSchema {
    name: String,
    system_prompt: String,
    provider: String,
    model: String,
    temperature: f64,
    max_tokens: u64,
}
