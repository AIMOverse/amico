use amico::ai::{
    chat::{ChatHistory, Message},
    errors::{CompletionError, ServiceError, ToolCallError},
    provider::{CompletionConfig, ModelChoice, Provider},
    tool::ToolSet,
};
use async_trait::async_trait;

use crate::interface::{Plugin, PluginCategory, PluginInfo};

pub struct InMemoryService {
    /// The configuration for the service
    pub config: CompletionConfig,

    /// The provider to use
    pub provider: Box<dyn Provider>,

    /// Chat history
    pub history: ChatHistory,

    /// Tools to use
    pub tools: ToolSet,
}

impl InMemoryService {
    pub fn new(config: CompletionConfig, provider: Box<dyn Provider>, tools: ToolSet) -> Self {
        Self {
            config,
            provider,
            history: ChatHistory::new(),
            tools,
        }
    }
}

impl Plugin for InMemoryService {
    fn info(&self) -> &'static PluginInfo {
        &PluginInfo {
            name: "StdInMemoryService",
            category: PluginCategory::Service,
        }
    }
}

#[async_trait]
impl amico::ai::service::Service for InMemoryService {
    async fn generate_text(&mut self, prompt: String) -> Result<String, ServiceError> {
        let response = self
            .provider
            .completion(&prompt, &self.config, &self.history, &self.tools)
            .await;

        match response {
            Ok(choice) => match choice {
                ModelChoice::Message(msg) => {
                    // Add the new response to the history list
                    self.history.push(Message {
                        role: "user".to_string(),
                        content: prompt.clone(),
                    });
                    self.history.push(Message {
                        role: "assistant".to_string(),
                        content: msg.clone(),
                    });

                    // Return the response message
                    Ok(msg)
                }
                ModelChoice::ToolCall(name, _) => Err(ServiceError::ToolError(
                    ToolCallError::ToolUnavailable(name),
                )),
            },
            Err(_) => Err(ServiceError::ProviderError(CompletionError::ApiError)),
        }
    }

    fn set_system_prompt(&mut self, prompt: String) {
        self.config.system_prompt = prompt;
    }

    fn set_provider(&mut self, provider: Box<dyn Provider>) {
        self.provider = provider;
    }
}
