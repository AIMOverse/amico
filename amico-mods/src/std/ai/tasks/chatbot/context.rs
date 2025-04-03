use amico::{ai::services::CompletionService, task::TaskContext};

/// The common context for chatbot tasks.
pub struct ChatbotContext<S>
where
    S: CompletionService,
{
    pub service: S,
}

impl<S> TaskContext for ChatbotContext<S>
where
    S: CompletionService,
{
    fn service(&self) -> &impl CompletionService
    where
        Self: Sized,
    {
        &self.service
    }
}
