use std::future::Future;

use crate::{types::AgentEvent, world::ActionSender};

/// The Agent's action selection strategy.
///
/// Actions for the Agent is one or several ECS events sent to the ECS `World`.
pub trait Strategy {
    /// Responsible for selecting actions based on the `AgentEvent` received.
    fn deliberate(
        &mut self,
        agent_event: &AgentEvent,
        sender: ActionSender,
    ) -> impl Future<Output = anyhow::Result<()>>;
}
