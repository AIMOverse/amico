use amico::ai::{
    chat::{ChatHistory, Message, ToolCall},
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
                    self.history.push(Message::user(prompt.clone()));
                    self.history.push(Message::assistant(msg.clone()));

                    // Return the response message
                    Ok(msg)
                }
                ModelChoice::ToolCall(name, params) => {
                    tracing::debug!("Calling {} with params {}", name, params);

                    // Execute the tool
                    if let Some(tool) = self.tools.get(&name) {
                        match (tool.tool_call)(params.clone()) {
                            Ok(res) => {
                                tracing::debug!("Tool {} returned {}", name, res);
                                // Successfully called the tool
                                self.history.push(Message::user(prompt.clone()));
                                self.history.push(Message::assistant_tool_call(vec![
                                    ToolCall::function(
                                        "call_12345".to_string(),
                                        name.clone(),
                                        params.clone(),
                                    ),
                                ]));
                                self.history
                                    .push(Message::tool(name.clone(), res.to_string()));

                                tracing::debug!("History: {:#?}", self.history);
                                // Re-generate the text with the prompt and the new information
                                self.generate_text(prompt).await
                            }
                            Err(err) => Err(ServiceError::ToolError(err)),
                        }
                    } else {
                        Err(ServiceError::ToolError(ToolCallError::ToolUnavailable(
                            name,
                        )))
                    }
                }
            },
            Err(err) => {
                tracing::error!("Provider error: {}", err);
                Err(ServiceError::ProviderError(CompletionError::ApiError))
            }
        }
    }

    fn set_system_prompt(&mut self, prompt: String) {
        self.config.system_prompt = prompt;
    }

    fn set_provider(&mut self, provider: Box<dyn Provider>) {
        self.provider = provider;
    }
}
