use schemars::JsonSchema;

#[derive(JsonSchema, serde::Serialize, serde::Deserialize)]
pub struct ProvidersConfigSchema {
    openai: ProviderItemSchema,
    custom: Vec<CustomProviderItemSchema>,
}

#[derive(JsonSchema, serde::Serialize, serde::Deserialize)]
struct ProviderItemSchema {
    api_key: Option<String>,
    base_url: Option<String>,
}

#[derive(JsonSchema, serde::Serialize, serde::Deserialize)]
struct CustomProviderItemSchema {
    name: String,
    api_key: Option<String>,
    base_url: String,
}
