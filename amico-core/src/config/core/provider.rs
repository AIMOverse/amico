use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProvidersConfig {
    pub openai: ProviderItem,
    pub custom: Option<Vec<CustomProviderItem>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProviderItem {
    pub api_key: String,
    pub base_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CustomProviderItem {
    pub name: String,
    #[serde(default = "default_api_schema")]
    pub schema: ApiSchema,
    pub base_url: String,
    pub api_key: Option<String>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ApiSchema {
    Openai,
}

fn default_api_schema() -> ApiSchema {
    ApiSchema::Openai
}
