use std::sync::Arc;

use amico_mods::std::ai::{providers::rig::RigProvider, services::InMemoryService};
use anyhow::{Result, bail};
use evenio::prelude::*;
use tokio::sync::Mutex;

use crate::audio::{RecordSignal, play_blocking, spawn_record_task};

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

#[derive(Component)]
pub struct Recorder {
    stop_recording_tx: Option<std::sync::mpsc::Sender<RecordSignal>>,

    recording_task_rx: Option<std::sync::mpsc::Receiver<RecordSignal>>,
}

impl Recorder {
    pub fn new() -> Self {
        Self {
            stop_recording_tx: None,
            recording_task_rx: None,
        }
    }

    pub fn is_recording(&self) -> bool {
        self.stop_recording_tx.is_some() && self.recording_task_rx.is_some()
    }

    pub fn start_record(&mut self, filepath: &str) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.stop_recording_tx = Some(tx);
        let task_rx = spawn_record_task(filepath, rx);
        self.recording_task_rx = Some(task_rx);
    }

    pub fn finish_record(&mut self) -> Result<()> {
        if let Some(tx) = &self.stop_recording_tx {
            tx.send(RecordSignal)?;
        } else {
            bail!("Recorder not recording");
        }

        if let Some(rx) = &mut self.recording_task_rx {
            rx.recv()?;
        } else {
            bail!("Recorder not recording");
        }

        self.stop_recording_tx = None;
        self.recording_task_rx = None;

        Ok(())
    }
}

#[derive(Component, Clone, Copy)]
pub struct Player;

impl Player {
    pub fn play_blocking(&self, filepath: &str) -> Result<()> {
        play_blocking(filepath)
    }
}
