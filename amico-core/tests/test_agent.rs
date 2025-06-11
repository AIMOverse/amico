use std::{future::Future, time::Duration};

use amico_core::{
    Agent, OnFinish, ecs,
    traits::{EventSource, Strategy, System},
    types::AgentEvent,
};
use serde::{Deserialize, Serialize};
use tokio::{task::JoinHandle, time::sleep};
use tokio_with_wasm::alias as tokio;

#[derive(Serialize, Deserialize)]
struct EventInner {
    message: String,
    value: i32,
}

struct TestEventSource;

impl EventSource for TestEventSource {
    fn spawn<F, Fut>(&self, on_event: F) -> JoinHandle<anyhow::Result<()>>
    where
        F: Fn(AgentEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Option<String>> + Send + 'static,
    {
        tokio::spawn(async move {
            for i in 1..10 {
                let event = AgentEvent::new("Tick", "TestEventSource")
                    .with_content(EventInner {
                        message: "tick".to_string(),
                        value: i,
                    })?
                    .lifetime(Duration::from_secs(10));

                on_event(event).await;

                // Simulate an asynchronous process.
                sleep(Duration::from_millis(50)).await;
            }

            Ok(())
        })
    }
}

struct TestStrategy;

impl Strategy for TestStrategy {
    async fn deliberate(
        &mut self,
        agent_event: &AgentEvent,
        mut sender: amico_core::world::ActionSender<'_>,
    ) -> anyhow::Result<Option<String>> {
        let EventInner { value, .. } = agent_event.parse_content::<EventInner>()?;
        sleep(Duration::from_millis(80)).await;

        sender.send(Tick(value));

        Ok(None)
    }
}

struct TestSystem;

#[derive(amico_core::ecs::GlobalEvent)]
struct Tick(pub i32);

impl System for TestSystem {
    fn register_to(self, mut registry: amico_core::world::HandlerRegistry) {
        registry.register(|r: ecs::Receiver<Tick>| {
            println!("Received Tick event seq. {}", r.event.0);
        });
    }
}

#[tokio::test]
async fn test_agent() {
    // tracing_subscriber::fmt::init();

    let mut agent = Agent::new(TestStrategy);
    agent.spawn_event_source(TestEventSource, OnFinish::Stop);
    agent.add_system(TestSystem);

    agent.run().await;
}
