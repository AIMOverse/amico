use anyhow::{anyhow, Result};
use evenio::prelude::*;
use tokio::sync::mpsc::channel;

use crate::engine::events::UserInput;

use super::{a2a::A2aModule, interaction::Stdio};

/// Send events from event generators to ECS World.
pub struct Agent<'world> {
    world: &'world mut World,
    interaction: EntityId,
    a2a: A2aModule,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AgentEvent {
    pub name: &'static str,
    pub value: String,
}

impl<'world> Agent<'world> {
    pub fn new<'a>(world: &'a mut World, interaction: EntityId, a2a: A2aModule) -> Self
    where
        'a: 'world,
    {
        Self {
            world,
            interaction,
            a2a,
        }
    }

    pub async fn run(self) -> Result<()> {
        let (tx, mut rx) = channel::<AgentEvent>(4);

        let mut es_join_handles = vec![];

        let stdio_event_source = self
            .world
            .get::<Stdio>(self.interaction)
            .ok_or(anyhow!("Failed to find StdioEventSource Component"))?;

        let tx_clone = tx.clone();
        es_join_handles.push(stdio_event_source.spawn_event_source(move |content| {
            tx_clone
                .blocking_send(AgentEvent {
                    name: "StdioInput",
                    value: content,
                })
                .unwrap();
        }));

        let tx_clone = tx.clone();
        es_join_handles.push(self.a2a.spawn_event_source(move |message| {
            let tx_clone = tx_clone.clone();
            async move {
                tx_clone
                    .send(AgentEvent {
                        name: "AgentMessage",
                        value: message,
                    })
                    .await
                    .unwrap();
            }
        }));

        while let Some(event) = rx.recv().await {
            tracing::info!("Received event: {:?}", event);

            match event.name {
                "StdioInput" => self.world.send(UserInput(event.value)),
                "AgentMessage" => self.world.send(UserInput(format!(
                    "Just received a message from your agent frend: \"{}\"",
                    event.value
                ))),
                _ => tracing::error!("Unknown event name: {}", event.name),
            }

            // Send to the chatbot system
        }

        tracing::debug!("Channel closed");

        for jh in es_join_handles {
            jh.await.unwrap();
        }

        Ok(())
    }
}
