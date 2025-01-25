use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct ProvidersConfig {
    openai: ProviderItem,
    custom: Vec<CustomProviderItem>,
}

#[derive(JsonSchema, Serialize, Deserialize)]
struct ProviderItem {
    api_key: Option<String>,
    base_url: Option<String>,
}

#[derive(JsonSchema, Serialize, Deserialize)]
struct CustomProviderItem {
    name: String,
    base_url: String,
    api_key: Option<String>,
    #[serde(default = "default_api_schema")]
    schema: ApiSchema,
}

#[derive(JsonSchema, Serialize, Deserialize)]
enum ApiSchema {
    Openai,
}

fn default_api_schema() -> ApiSchema {
    ApiSchema::Openai
}
