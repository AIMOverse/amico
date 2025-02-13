use async_trait::async_trait;

use super::provider::Provider;
use crate::ai::errors::ServiceError;

/// A Service executes a certain AI task, such as generating text.
/// using a series of model provider calls.
#[async_trait]
pub trait Service: Send + Sync {
    async fn generate_text(
        &mut self,
        provider: &dyn Provider,
        prompt: String,
    ) -> Result<String, ServiceError>;

    fn set_system_prompt(&mut self, prompt: String);
}
