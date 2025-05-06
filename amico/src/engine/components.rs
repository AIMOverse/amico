use std::sync::Arc;

use amico_mods::std::ai::{providers::rig::RigProvider, services::InMemoryService};
use evenio::prelude::*;
use tokio::sync::Mutex;

#[derive(Component)]
pub struct AiService(pub Arc<Mutex<InMemoryService<RigProvider>>>);
