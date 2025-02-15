use amico::ai::{
    errors::{CompletionError, ServiceError, ToolCallError},
    provider::{ModelChoice, Provider},
};
use async_trait::async_trait;

use crate::interface::{Plugin, PluginCategory, PluginInfo};

pub struct Service {
    /// The system prompt for the service
    pub system_prompt: String,

    /// The temperature to use
    pub temperature: f32,

    /// The maximum number of tokens to generate
    pub max_tokens: u32,

    /// The provider to use
    pub provider: Box<dyn Provider>,
}

impl Plugin for Service {
    fn info(&self) -> &'static PluginInfo {
        &PluginInfo {
            name: "StdService",
            category: PluginCategory::Service,
        }
    }
}

#[async_trait]
impl amico::ai::service::Service for Service {
    async fn generate_text(&mut self, prompt: String) -> Result<String, ServiceError> {
        let response = self
            .provider
            .completion(
                "gpt-4o".to_string(),
                prompt,
                &Vec::<amico::ai::chat::Message>::new(),
            )
            .await;

        match response {
            Ok(choice) => match choice {
                ModelChoice::Message(msg) => Ok(msg),
                ModelChoice::ToolCall(name, _) => {
                    Err(ServiceError::ToolError(ToolCallError::ToolUnavailable(name)))
                }
            },
            Err(_) => Err(ServiceError::ProviderError(CompletionError::ApiError)),
        }
    }

    fn set_system_prompt(&mut self, prompt: String) {
        self.system_prompt = prompt;
    }

    fn set_provider(&mut self, provider: Box<dyn Provider>) {
        self.provider = provider;
    }
}
