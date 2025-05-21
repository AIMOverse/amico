use std::{future::Future, time::Duration};

use amico_core::{
    Agent, OnFinish, ecs,
    traits::{Dispatcher, EventSource, System},
    types::{AgentEvent, EventContent},
};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

#[derive(Serialize, Deserialize)]
struct EventInner {
    message: String,
    value: i32,
}

struct TestEventSource;

impl EventSource for TestEventSource {
    async fn run<F, Fut>(&self, on_event: F) -> anyhow::Result<()>
    where
        F: Fn(AgentEvent) -> Fut + Send + Sync + 'static,
        Fut: Future + Send + 'static,
    {
        for i in 1..10 {
            let event = AgentEvent::new(
                "Tick",
                "TestEventSource",
                Some(EventContent::Content(serde_json::to_value(EventInner {
                    message: "tick".to_string(),
                    value: i,
                })?)),
                Some(Duration::from_secs(10)),
            );

            on_event(event).await;

            sleep(Duration::from_millis(50)).await;
        }

        Ok(())
    }
}

struct TestDispatcher;

impl Dispatcher for TestDispatcher {
    async fn dispatch(
        &mut self,
        agent_event: &AgentEvent,
        mut delegate: amico_core::world::EventDelegate<'_>,
    ) -> anyhow::Result<()> {
        let EventInner { value, .. } = agent_event.parse_content::<EventInner>()?;
        sleep(Duration::from_millis(80)).await;

        delegate.send_event(Tick(value));

        Ok(())
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
    tracing_subscriber::fmt::init();

    let mut agent = Agent::new(TestDispatcher);
    agent.spawn_event_source(TestEventSource, OnFinish::Stop);
    agent.wm.register_system(TestSystem);

    agent.run().await;
}
