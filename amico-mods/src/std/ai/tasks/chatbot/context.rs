use amico::{ai::service::Service, task::TaskContext};

/// The common context for chatbot tasks.
pub struct ChatbotContext<S>
where
    S: Service,
{
    pub service: S,
}

impl<S> TaskContext for ChatbotContext<S>
where
    S: Service,
{
    fn service(&self) -> &impl Service
    where
        Self: Sized,
    {
        &self.service
    }
}
