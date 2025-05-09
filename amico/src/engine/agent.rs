use anyhow::{anyhow, Result};
use evenio::prelude::*;
use tokio::sync::mpsc::channel;

use crate::engine::events::UserInput;

use super::interaction::Stdio;

/// Send events from event generators to ECS World.
pub struct Agent<'world> {
    world: &'world mut World,
    interaction: EntityId,
}

impl<'world> Agent<'world> {
    pub fn new<'a>(world: &'a mut World, interaction: EntityId) -> Self
    where
        'a: 'world,
    {
        Self { world, interaction }
    }

    pub async fn run(self) -> Result<()> {
        let (tx, mut rx) = channel::<String>(4);

        let stdio_event_source = self
            .world
            .get::<Stdio>(self.interaction)
            .ok_or(anyhow!("Failed to find StdioEventSource Component"))?;

        let es_handle = stdio_event_source.spawn_event_source(move |content| {
            tx.blocking_send(content).unwrap();
        });

        while let Some(content) = rx.recv().await {
            tracing::info!("Received user input: {:?}", content);

            // Send to the chatbot system
            self.world.send(UserInput(content));
        }

        tracing::debug!("Channel closed");

        es_handle.await.unwrap();

        Ok(())
    }
}
