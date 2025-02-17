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

// Tool call prompts

fn assistant_tool_call_prompt(function_name: &str, arguments: &str) -> String {
    format!("I will call the tool funcion `{}` with arguments `{}`. Please tell me the result and ask me again.", function_name, arguments)
}

fn user_tool_result_prompt(function_name: &str, result: &str) -> String {
    format!("I just called the tool `{}` for you and the result was `{}`. With these information, please respond again.", function_name, result)
}

fn user_tool_failed_prompt(function_name: &str, error: &str) -> String {
    format!("I just called the tool `{}` for you, but there was an error: `{}`. If you cannot proceed, please let me know.", function_name, error)
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
                ModelChoice::ToolCall(name, id, params) => {
                    tracing::debug!("Calling {} ({}) with params {}", name, id, params);

                    // Execute the tool
                    if let Some(tool) = self.tools.get(&name) {
                        match (tool.tool_call)(params.clone()) {
                            Ok(res) => {
                                // Successfully called the tool
                                // TODO: Use actual tool call format
                                self.history.push(Message::user(prompt.clone()));
                                self.history
                                    .push(Message::assistant(assistant_tool_call_prompt(
                                        name.as_str(),
                                        params.to_string().as_str(),
                                    )));
                                self.history.push(Message::user(user_tool_result_prompt(
                                    name.as_str(),
                                    res.to_string().as_str(),
                                )));
                                // Re-generate the text with the prompt and the new information
                                self.generate_text(prompt).await
                            }
                            Err(err) => {
                                // Failed to call the tool
                                self.history.push(Message::user(prompt.clone()));
                                self.history
                                    .push(Message::assistant(assistant_tool_call_prompt(
                                        name.as_str(),
                                        params.to_string().as_str(),
                                    )));
                                self.history.push(Message::user(user_tool_failed_prompt(
                                    name.as_str(),
                                    err.to_string().as_str(),
                                )));
                                Err(ServiceError::ToolError(err))
                            }
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
