#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AgentConfig {
    pub name: String,
    pub system_prompt: String,
    pub provider: String,
    pub model: String,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
}
