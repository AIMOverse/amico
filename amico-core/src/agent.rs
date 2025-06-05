use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio_with_wasm::alias as tokio;

use crate::{
    traits::{EventSource, Strategy},
    types::{AgentEvent, EventContent, Instruction},
    world::WorldManager,
};

/// The core event-driven Agent program. Defines the workflow of the agent.
///
/// The `Agent` creates and manages an ECS `World`, manages
/// `AgentEvent`s sent from `EventSource`s, and dispatches them.
///
/// ## Type parameters
///
/// - `S`: `Strategy` type, representing the Agent's action selection strategy.
///
/// ## Compatibility
///
/// - WASM: compatible.
pub struct Agent<S: Strategy> {
    /// The mpsc channel sender to send agent events to event sources.
    event_tx: Sender<AgentEvent>,

    /// The mpsc channel receiver to receive agent events from event sources.
    event_rx: Receiver<AgentEvent>,

    /// The ECS world manager.
    ///
    /// **NOTE**: This field will be private in the future, after
    /// we wrap the component / system registration into the `Agent`'s
    /// methods.
    pub wm: WorldManager,

    /// The action selection strategy.
    strategy: S,
}

impl<S: Strategy> Agent<S> {
    /// Create a new agent.
    pub fn new(strategy: S) -> Self {
        // Create an event channel.
        // TODO: make the channel size configurable.
        let (tx, rx) = channel(4);

        // Build the Agnet.
        Self {
            event_tx: tx,
            event_rx: rx,
            wm: WorldManager::new(),
            strategy,
        }
    }

    /// Spawn an event source for the agent.
    ///
    /// ## Spawns
    ///
    /// Spawns a new `tokio` thread for the event source.
    ///
    /// ## Compatibility
    ///
    /// - WASM: compatible with `tokio_with_wasm`
    pub fn spawn_event_source<E: EventSource + Send + 'static>(
        &mut self,
        event_source: E,
        on_finish: OnFinish,
    ) {
        let event_tx = self.event_tx.clone();
        // Spawn the thread.
        let jh = event_source.spawn(move |event| {
            tracing::debug!("On AgentEvent {:?}", event);
            let tx = event_tx.clone();

            async move {
                let name = event.name;

                if let Err(err) = tx.send(event).await {
                    tracing::warn!("Failed to send AgentEvent {}", err);
                } else {
                    tracing::info!("Sent AgentEvent {}", name);
                }
            }
        });

        // Wait for the event source to finish and send termination signal if needed.
        match &on_finish {
            OnFinish::Stop => {
                // Spawn a new thread to wait for the event source to finish.
                let event_tx = self.event_tx.clone();
                tokio::spawn(async move {
                    // Wait for the event source to finish.
                    if let Err(err) = jh.await.unwrap() {
                        tracing::error!("Event source JoinError: {}", err);
                        return;
                    }

                    // Send a termination instruction to signal the main loop to exit
                    let terminate_event = AgentEvent::new("Terminate", "spawn_event_source")
                        .instruction(Instruction::Terminate);

                    // Try to send the termination event, but don't panic if it fails
                    // (channel might already be closed)
                    if let Err(err) = event_tx.send(terminate_event).await {
                        tracing::warn!("Failed to send termination event: {}", err);
                    }
                });
            }
            OnFinish::Continue => {}
        }
    }

    /// The function to run the agent.
    ///
    /// `run` dispatches `AgentEvent`s into the ECS `World` based on the Agent's strategy.
    pub async fn run(&mut self) {
        // Listen for events sent by event sources.
        while let Some(event) = self.event_rx.recv().await {
            tracing::debug!("Received AgentEvent {:?}", event);

            if let Some(EventContent::Instruction(instruction)) = event.content {
                // Received an instruction
                tracing::debug!("Received instruction {:?}", instruction);
                match instruction {
                    // TODO: process other instructions
                    Instruction::Terminate => {
                        tracing::info!("Terminating event loop due to Terminate instruction");
                        break; // Exit the event loop immediately
                    }
                }
            } else {
                // The event is not an instruction, dispatch the event to the `World`.
                tracing::debug!("Dispatching event {:?}", event);
                if let Err(err) = self
                    .strategy
                    .deliberate(&event, self.wm.action_sender())
                    .await
                {
                    tracing::error!("Error dispatching event {:?}: {}", event, err);
                }
            }
        }

        tracing::info!("Exited event loop.");
    }
}

/// The behaviour to choose when event source thread finishes.
pub enum OnFinish {
    // Do nothing when the event source thread finishes.
    Continue,

    // Stop the Agent workflow when the thread finishes.
    Stop,
}
