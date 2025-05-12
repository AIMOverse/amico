use std::future::Future;

use async_trait::async_trait;

use crate::{types::AgentEvent, world::WorldManager};

/// `Dispatcher`s dispatches `AgentEvent`s into the ECS `World`.
///
/// Currently, `Dispatcher` = `ActionSelector` + `Action` + ECS.
///
/// ## TODO
///
/// - Integrate `EventPool` to handle a list of events.
/// - Replace `ActionSelector`.
pub trait Dispatcher {
    /// Dispatches an `AgentEvent` into the ECS `World`.
    fn dispatch(
        &mut self,
        world: &mut WorldManager,
        agent_event: &AgentEvent,
    ) -> impl Future<Output = anyhow::Result<()>>;
}

/// The thread-local dyn-compatible `Dispatcher` trait.
///
/// `World` is not `Send`able, so the `DispatcherDyn` trait
/// won't be defined and implemented.
#[async_trait(?Send)]
pub trait DispatcherLocal {
    /// Dispatches an `AgentEvent` into the ECS `World`.
    async fn dispatch_local(
        &mut self,
        world: &mut WorldManager,
        agent_event: &AgentEvent,
    ) -> anyhow::Result<()>;
}

#[async_trait(?Send)]
impl<T: Dispatcher> DispatcherLocal for T {
    async fn dispatch_local(
        &mut self,
        world: &mut WorldManager,
        agent_event: &AgentEvent,
    ) -> anyhow::Result<()> {
        self.dispatch(world, agent_event).await
    }
}
