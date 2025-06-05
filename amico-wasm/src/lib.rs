use std::{future::Future, time::Duration};

use amico_core::{
    Agent, OnFinish, ecs,
    traits::{EventSource, Strategy, System},
    types::AgentEvent,
};
use serde::{Deserialize, Serialize};
use tokio::{spawn, task::JoinHandle, time::sleep};
use tokio_with_wasm::alias as tokio;
use wasm_bindgen::prelude::wasm_bindgen;

mod log;

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
        Fut: Future + Send + 'static,
    {
        spawn(async move {
            for i in 1..10 {
                let event = AgentEvent::new("Tick", "TestEventSource")
                    .with_content(EventInner {
                        message: "tick".to_string(),
                        value: i,
                    })?
                    .lifetime(Duration::from_secs(10));

                on_event(event).await;

                sleep(Duration::from_millis(50)).await;
            }

            Ok(())
        })
    }
}

struct TestDispatcher;

impl Strategy for TestDispatcher {
    async fn deliberate(
        &mut self,
        agent_event: &AgentEvent,
        mut delegate: amico_core::world::ActionSender<'_>,
    ) -> anyhow::Result<()> {
        let EventInner { value, .. } = agent_event.parse_content::<EventInner>()?;
        sleep(Duration::from_millis(80)).await;

        delegate.send(Tick(value));

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

#[tokio::main(flavor = "current_thread")]
async fn test_agent() {
    let mut agent = Agent::new(TestDispatcher);
    agent.spawn_event_source(TestEventSource, OnFinish::Stop);
    agent.add_system(TestSystem);

    agent.run().await;
}

#[wasm_bindgen(start)]
pub fn start() {
    log::init();
    test_agent();
}
