use crate::{types::AgentEvent, world::ActionSender};

/// The Agent's action selection strategy.
///
/// Actions for the Agent is one or several ECS events sent to the ECS `World`.
pub trait Strategy {
    /// Responsible for selecting actions based on the `AgentEvent` received.
    ///
    /// The returned **reply** type `String` should not be confused with **the result**
    /// of the deliberation process. The agent may generate a reply message to the user,
    /// especially when the `AgentEvent` is a `Interaction`. But in other cases, the
    /// agent may not produce a reply message, unless it thinks replying to the user
    /// is necessary.
    ///
    /// The result of the deliberation process is the `Action`s sent to the ECS `World`.
    ///
    /// # Returns
    ///
    /// - `Ok(None)`: The strategy does not want to produce a reply message.
    /// - `Ok(Some(reply))`: The strategy wants to produce a reply message.
    /// - `Err(e)`: The strategy is unable to process the `AgentEvent`.
    fn deliberate(
        &mut self,
        agent_event: &AgentEvent,
        sender: ActionSender,
    ) -> impl Future<Output = anyhow::Result<Option<String>>>;
}
