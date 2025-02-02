use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventConfig {
    pub expiry_time: u32, // The expiry time of the event in seconds
}
