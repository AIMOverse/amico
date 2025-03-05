use amico::ai::{
    errors::{CompletionError, ServiceError, ToolCallError},
    message::Message,
    provider::{ModelChoice, Provider},
    service::ServiceContext,
};
use async_trait::async_trait;

use crate::interface::{Plugin, PluginCategory, PluginInfo};

pub struct InMemoryService<P>
where
    P: Provider,
{
    /// The context config for the service
    pub ctx: ServiceContext<P>,

    /// In-memory Chat history storage
    pub history: Vec<Message>,
}

impl<P: Provider> Plugin for InMemoryService<P> {
    fn info(&self) -> &'static PluginInfo {
        &PluginInfo {
            name: "StdInMemoryService",
            category: PluginCategory::Service,
        }
    }
}

// Tool call prompts

fn assistant_tool_call_prompt(function_name: &str, arguments: &str) -> String {
    format!("**Tool Call Request**\n\nI will call the tool funcion `{}` with arguments `{}`. Please tell me the result in your next message.", function_name, arguments)
}

fn user_tool_result_prompt(function_name: &str, result: &str) -> String {
    format!("**Tool Call Result**\n\nThe result of calling the tool `{}` is `{}`. With these extra information, please respond to the user again.", function_name, result)
}

fn user_tool_failed_prompt(function_name: &str, error: &str) -> String {
    format!("**Tool Call Failed**\n\nCalling the tool `{}` failed. The error is `{}`. Report the error to the user.", function_name, error)
}

#[async_trait]
impl<P> amico::ai::service::Service<P> for InMemoryService<P>
where
    P: Provider,
{
    fn from(context: ServiceContext<P>) -> Self
    where
        Self: Sized,
    {
        Self {
            ctx: context,
            history: Vec::new(),
        }
    }

    fn ctx(&self) -> &ServiceContext<P> {
        &self.ctx
    }

    fn mut_ctx(&mut self) -> &mut ServiceContext<P> {
        &mut self.ctx
    }

    async fn generate_text(&mut self, prompt: String) -> Result<String, ServiceError> {
        let request = amico::ai::completion::CompletionRequestBuilder::from_ctx(&self.ctx)
            .prompt(prompt.clone())
            .build();

        let response = self.ctx.provider.completion(&request).await;

        match response {
            Ok(choice) => match choice {
                ModelChoice::Message(msg) => {
                    tracing::debug!("Received message response: {}", msg);

                    // Add the new response to the history list
                    self.history.push(Message::user(prompt.clone()));
                    self.history.push(Message::assistant(msg.clone()));

                    tracing::debug!("Updated history: {:?}", self.history);

                    // Return the response message
                    Ok(msg)
                }
                ModelChoice::ToolCall(name, id, params) => {
                    tracing::debug!("Calling {} ({}) with params {}", name, id, params);

                    // Execute the tool
                    if let Some(tool) = self.ctx.tools.get(&name) {
                        match (tool.tool_call)(params.clone()) {
                            Ok(res) => {
                                // Successfully called the tool
                                tracing::debug!("Tool call succeeded with result: {}", res);

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

                                tracing::debug!("Updated history: {:?}", self.history);

                                tracing::debug!("Re-generating text");
                                // Re-generate the text with the prompt and the new information
                                self.generate_text(prompt).await
                            }
                            Err(err) => {
                                // Failed to call the tool
                                tracing::debug!("Tool call failed with error: {}", err);

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

                                tracing::debug!("Updated history: {:?}", self.history);

                                tracing::debug!("Re-generating text");
                                // Re-generate the text with the prompt and the new information
                                self.generate_text(prompt).await
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
}
