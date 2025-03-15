use amico::ai::{provider::Provider, service::Service};
use async_trait::async_trait;

pub struct TaskContext<S, P>
where
    S: Service<P>,
    P: Provider,
{
    pub service: S,

    phantom: std::marker::PhantomData<P>,
}

impl<S, P> TaskContext<S, P>
where
    S: Service<P>,
    P: Provider,
{
    pub fn new(service: S) -> Self {
        TaskContext { service, phantom: std::marker::PhantomData }
    }
}

#[async_trait]
pub trait Task<S, P>
where
    S: Service<P>,
    P: Provider,
{
    fn setup(context: TaskContext<S, P>) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
