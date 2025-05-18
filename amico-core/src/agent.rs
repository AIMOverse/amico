use tokio::{
    sync::mpsc::{Receiver, Sender, channel},
    task::JoinSet,
};

use crate::{
    traits::Dispatcher,
    types::{AgentInstruction, EventContent},
    world::WorldManager,
};
use crate::{traits::EventSource, types::AgentEvent};

/// The behaviour to choose when event source thread finishes.
///
/// TODO: Replace this with `AgentInstruction` after the
/// agent instruction feature is implemented.
pub enum OnFinish {
    // Do nothing when the event source thread finishes.
    Continue,

    // Stop the Agent workflow when the thread finishes.
    Stop,
}

/// The core event-driven Agent program. Defines the workflow of the agent.
///
/// The `Agent` creates and manages an ECS `World`, manages
/// `AgentEvent`s sent from `EventSource`s, and dispatches them.
///
/// ## Type parameters
///
/// - `D`: The Event `Dispatcher` type, representing the Agent's action selection strategy.
pub struct Agent<D: Dispatcher> {
    /// The JoinSet to store EventSource thread join handles.
    event_source_js: JoinSet<OnFinish>,

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

    /// The event dispatcher.
    dispatcher: D,
}

impl<D: Dispatcher> Agent<D> {
    /// Create a new agent.
    pub fn new(dispatcher: D) -> Self {
        // Create an event channel.
        // TODO: make the channel size configurable.
        let (tx, rx) = channel(4);

        // Build the Agnet.
        Self {
            event_source_js: JoinSet::new(),
            event_tx: tx,
            event_rx: rx,
            wm: WorldManager::new(),
            dispatcher,
        }
    }

    /// Spawn an event source for the agent.
    ///
    /// ## Spawns
    ///
    /// Spawns a new `tokio` thread for the event source.
    ///
    /// ## Panics
    ///
    /// Panics on `SendError`s.
    pub fn spawn_event_source<S: EventSource + Send + 'static>(
        &mut self,
        event_source: S,
        on_finish: OnFinish,
    ) {
        let js = &mut self.event_source_js;

        // Spawn the thread.
        let tx = self.event_tx.clone();
        js.spawn(async move {
            let event_tx = tx.clone();
            event_source
                .run(move |event| {
                    tracing::debug!("On AgentEvent {:?}", event);
                    let tx = event_tx.clone();

                    // Never enters this `async move` block
                    async move {
                        let name = event.name;
                        tracing::debug!("Sending Event to agent...");

                        if let Err(err) = tx.send(event).await {
                            tracing::warn!("Failed to send AgentEvent {}", err);
                        } else {
                            tracing::info!("Sent AgentEvent {}", name);
                        }
                    }
                })
                .await
                .unwrap();

            tracing::debug!("Event source ended.");

            // Send termination signal if needed
            match &on_finish {
                OnFinish::Stop => {
                    // Send a termination instruction to signal the main loop to exit
                    let terminate_event = AgentEvent::new(
                        "Terminate",
                        "EventSource",
                        Some(EventContent::Instruction(AgentInstruction::Terminate)),
                        None,
                    );

                    // Try to send the termination event, but don't panic if it fails
                    // (channel might already be closed)
                    if let Err(err) = tx.send(terminate_event).await {
                        tracing::warn!("Failed to send termination event: {}", err);
                    }
                }
                OnFinish::Continue => {}
            }

            // Yield the finish behaviour to the agent.
            // See `Agent::run`.
            on_finish
        });
    }

    /// The function to run the agent.
    ///
    /// `run` dispatches `AgentEvent`s into the ECS `World` and
    /// awaits all event sources to finish.
    pub async fn run(&mut self) {
        // Listen for events sent by event sources.
        while let Some(event) = self.event_rx.recv().await {
            tracing::debug!("Received AgentEvent {:?}", event);

            if let Some(EventContent::Instruction(instruction)) = event.content {
                // Received an instruction
                tracing::debug!("Received instruction {:?}", instruction);
                match instruction {
                    // TODO: process other instructions
                    AgentInstruction::Terminate => {
                        tracing::info!("Terminating event loop due to Terminate instruction");
                        break; // Exit the event loop immediately
                    }
                }
            } else {
                // The event is not an instruction, dispatch the event to the `World`.
                tracing::debug!("Dispatching event {:?}", event);
                if let Err(err) = self
                    .dispatcher
                    .dispatch(&event, self.wm.event_delegate())
                    .await
                {
                    // Just report the error here.
                    tracing::error!("Error dispatching event {:?}: {}", event, err);
                }
            }
        }

        tracing::info!("Exited event loop.");

        // Waits for event sources to finish.
        // If an event source choose to stop the agent workflow,
        while let Some(res) = self.event_source_js.join_next().await {
            match res {
                Ok(OnFinish::Continue) => continue,
                Ok(OnFinish::Stop) => return,
                Err(err) => {
                    tracing::error!("Event source JoinSet JoinError: {}", err);
                    panic!("Event source JoinSet JoinError: {}", err);
                }
            }
        }
    }
}
