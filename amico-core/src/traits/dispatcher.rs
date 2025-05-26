use std::future::Future;

use crate::{types::AgentEvent, world::EventDelegate};

/// `Dispatcher`s dispatches `AgentEvent`s into the ECS `World`.
///
/// Representing an Agent's action selection strategy.
pub trait Dispatcher {
    /// Dispatches an `AgentEvent` into the ECS `World`.
    fn dispatch(
        &mut self,
        agent_event: &AgentEvent,
        delegate: EventDelegate,
    ) -> impl Future<Output = anyhow::Result<()>>;
}
