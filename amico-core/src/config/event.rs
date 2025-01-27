use serde::{Deserialize, Serialize};

use super::params::{Params, WithParams};

#[derive(Serialize, Deserialize, WithParams)]
#[serde(rename_all = "snake_case")]
pub struct EventConfig {
    pub name: String,
    pub source: String,
    #[params]
    pub params: Option<Params>,
}
