use std::sync::Arc;

use amico_mods::std::ai::{providers::rig::RigProvider, services::InMemoryService};
use evenio::prelude::*;
use tokio::sync::Mutex;

#[derive(Component)]
pub struct AiService(Arc<Mutex<InMemoryService<RigProvider>>>);

impl AiService {
    pub fn new(service: InMemoryService<RigProvider>) -> Self {
        Self(Arc::new(Mutex::new(service)))
    }

    pub fn get(&self) -> Arc<Mutex<InMemoryService<RigProvider>>> {
        self.0.clone()
    }
}
