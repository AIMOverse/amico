use anyhow::{anyhow, Result};
use evenio::prelude::*;
use tokio::sync::mpsc::channel;

use super::{events::UserContent, interaction::Stdio};

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
        let (tx, mut rx) = channel::<UserContent>(4);

        let stdio_event_source = self
            .world
            .get::<Stdio>(self.interaction)
            .ok_or(anyhow!("Failed to find StdioEventSource Component"))?;

        let handle = stdio_event_source.spawn_event_source(move |e| {
            tx.blocking_send(e).unwrap();
        });

        while let Some(e) = rx.recv().await {
            tracing::info!("Received user input event: {:?}", e);
            self.world.send(e);
        }

        tracing::debug!("Channel closed");

        handle.await.unwrap();

        Ok(())
    }
}
