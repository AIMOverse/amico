use std::{future::Future, time::Duration};

use amico_core::{
    Agent, OnFinish,
    events::GlobalEvent,
    traits::{EventSource, Strategy, System},
    types::{AgentEvent, Chat, Interaction},
};
use tokio::{task::JoinHandle, time::sleep};
use tokio_with_wasm::alias as tokio;

struct InteractionSource;

impl EventSource for InteractionSource {
    fn spawn<F, Fut>(&self, on_event: F) -> JoinHandle<anyhow::Result<()>>
    where
        F: Fn(AgentEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Option<String>> + Send + 'static,
    {
        tokio::spawn(async move {
            for i in 1..10 {
                let event = AgentEvent::new("Chat", "InteractionSource")
                    .interaction(Chat::new().session_id(i).into_interaction());

                let result = on_event(event).await.unwrap();

                println!("Got reply: {}", result);

                // Simulate an asynchronous process.
                sleep(Duration::from_millis(30 + i * 5)).await;
            }

            Ok(())
        })
    }
}

struct InteractionStrategy;

impl Strategy for InteractionStrategy {
    async fn deliberate(
        &mut self,
        agent_event: &AgentEvent,
        mut sender: amico_core::world::ActionSender<'_>,
    ) -> anyhow::Result<Option<String>> {
        let interaction = agent_event.get_interaction().unwrap();

        match interaction {
            Interaction::Chat(chat) => {
                let _ = sender.send(Log(format!("Received chat interaction: {:?}", chat)));

                // Simulate an asynchronous reply generation process.
                sleep(Duration::from_millis(80)).await;
                Ok(Some(format!("Session ID: {}", chat.session_id)))
            } // _ => {}
        }
    }
}

struct LogSystem;

#[derive(Debug, Clone)]
struct Log(String);

impl GlobalEvent for Log {}

impl System for LogSystem {
    fn register_to(self, mut registry: amico_core::world::HandlerRegistry) {
        registry.register::<Log, _>(|event: &Log| {
            println!("log: {}", event.0);
            Ok(())
        });
    }
}

#[tokio::test]
async fn test_agent() {
    // tracing_subscriber::fmt::init();

    let mut agent = Agent::new(InteractionStrategy);
    agent.spawn_event_source(InteractionSource, OnFinish::Stop);
    agent.add_system(LogSystem);

    agent.run().await;
}
