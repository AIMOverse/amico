use amico_core::{traits::Strategy, types::AgentEvent, world::ActionSender};

use super::events::{A2aMessageReceived, ConsoleInput, UserInput};

/// A strategy that dispatches events to the ECS `World` directly.
pub struct DispatchStrategy;

impl Strategy for DispatchStrategy {
    async fn deliberate(
        &mut self,
        agent_event: &AgentEvent,
        mut sender: ActionSender<'_>,
    ) -> anyhow::Result<()> {
        tracing::info!("Dispatching {:?}", agent_event);
        match agent_event.name {
            "ConsoleInput" => {
                let ConsoleInput(input) = agent_event.parse_content::<ConsoleInput>()?;
                sender.send(UserInput(input));
            }
            "A2aMessageReceived" => {
                let A2aMessageReceived(message) =
                    agent_event.parse_content::<A2aMessageReceived>()?;

                sender.send(UserInput(format!(
                    "Just received a message from your agent frend: \"{}\"",
                    message,
                )));
            }
            _ => {
                tracing::warn!("Unknown event {:?} to dispatch", agent_event);
            }
        }

        Ok(())
    }
}
