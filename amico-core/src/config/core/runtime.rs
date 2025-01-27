use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeConfig {
    Standalone,
    // Farm,
}
