use amico::ai::service::Service;
use async_trait::async_trait;

pub struct TaskContext<S>
where
    S: Service,
{
    pub service: S,
}

impl<S> TaskContext<S>
where
    S: Service,
{
    pub fn new(service: S) -> Self {
        TaskContext { service }
    }
}

#[async_trait]
pub trait Task<S>
where
    S: Service,
{
    fn setup(context: TaskContext<S>) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
