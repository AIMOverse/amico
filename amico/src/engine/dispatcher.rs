use amico_core::{traits::Strategy, types::AgentEvent, world::EventDelegate};

use super::events::{A2aMessageReceived, ConsoleInput, UserInput};

pub struct MatchDispatcher;

impl Strategy for MatchDispatcher {
    async fn dispatch(
        &mut self,
        agent_event: &AgentEvent,
        mut delegate: EventDelegate<'_>,
    ) -> anyhow::Result<()> {
        tracing::info!("Dispatching {:?}", agent_event);
        match agent_event.name {
            "ConsoleInput" => {
                let ConsoleInput(input) = agent_event.parse_content::<ConsoleInput>()?;
                delegate.send_event(UserInput(input));
            }
            "A2aMessageReceived" => {
                let A2aMessageReceived(message) =
                    agent_event.parse_content::<A2aMessageReceived>()?;

                delegate.send_event(UserInput(format!(
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
