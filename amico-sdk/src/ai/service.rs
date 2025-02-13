use crate::ai::errors::ServiceError;
use crate::ai::model::{ChatResponse, Message};
use crate::ai::provider::Provider;
use async_trait::async_trait;

/// A Service provide a high-level interface to interact with AI models.
/// using a series of model provider calls.
#[async_trait]
pub trait Service: Send + Sync {
    async fn generate_text(
        &mut self,
        provider: &dyn Provider,
        model: String,
        messages: Vec<Message>,
    ) -> Result<ChatResponse, ServiceError>;
}
