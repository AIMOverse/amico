use amico_core::{Agent, traits::Strategy};

/// A module is a plugin that can be applied to an agent.
pub trait Module {
    /// Apply the module to an agent.
    fn apply<S: Strategy>(&self, agent: &mut Agent<S>);
}

#[cfg(test)]
mod tests {
    use amico_core::{
        Agent,
        traits::{PhantomEvent, Strategy, System},
    };

    use crate::Module;

    struct TestSystem;

    impl System for TestSystem {
        fn register_to(self, mut registry: amico_core::world::HandlerRegistry) {
            registry.register(|_: amico_core::ecs::Receiver<PhantomEvent>| {
                println!("inside TestSystem handler");
            });
        }
    }

    struct TestStrategy;

    impl Strategy for TestStrategy {
        async fn deliberate(
            &mut self,
            _agent_event: &amico_core::types::AgentEvent,
            _sender: amico_core::world::ActionSender<'_>,
        ) -> anyhow::Result<()> {
            Ok(())
        }
    }

    struct TestModule;

    impl Module for TestModule {
        fn apply<S: Strategy>(&self, agent: &mut amico_core::Agent<S>) {
            println!("Applying TestModule");
            agent.add_system(TestSystem);
        }
    }

    #[tokio::test]
    async fn test_build_agent_with_module() {
        let mut agent = Agent::new(TestStrategy);
        TestModule.apply(&mut agent);
    }
}
