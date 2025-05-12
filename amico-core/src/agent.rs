use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
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
    // The JoinSet to store EventSource thread join handles.
    event_source_js: JoinSet<OnFinish>,

    // The mpsc channel to receive agent events from event sources.
    event_tx: Sender<AgentEvent>,
    event_rx: Receiver<AgentEvent>,

    // The ECS world manager.
    pub wm: WorldManager,

    // The event dispatcher
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
            let tx = tx.clone();
            event_source
                .run(move |event| {
                    let tx = tx.clone();
                    async move {
                        // TODO: Handle send errors.
                        tx.send(event).await.unwrap();
                    }
                })
                .await
                .unwrap();

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
            if let Some(EventContent::Instruction(instruction)) = event.content {
                // Received an instruction
                self.process_instruction(instruction);
            } else {
                // The event is not an instruction, dispatch the event to the `World`.
                if let Err(err) = self.dispatcher.dispatch(&mut self.wm, &event).await {
                    // Just report the error here.
                    tracing::error!("Error dispatching event {:?}: {}", event, err);
                }
            }
        }

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

    /// Processes an `AgentInstruction`
    ///
    /// TODO: Implement the actual instruction logic here.
    fn process_instruction(&mut self, instruction: AgentInstruction) {
        tracing::info!("Processing agent instruction {:?}", instruction);
    }
}
