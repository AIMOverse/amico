use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{agent::AgentConfigSchema, event::EventConfigSchema, provider::ProvidersConfigSchema};

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct AmicoConfigSchema {
    version: u32,
    runtime: String,
    agents: Vec<AgentConfigSchema>,
    providers: ProvidersConfigSchema,
    events: Vec<EventConfigSchema>,
}
