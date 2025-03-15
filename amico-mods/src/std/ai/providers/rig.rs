use amico::ai::{
    completion::CompletionRequest,
    errors::{CompletionError, CreationError},
    message::Message,
    provider::{ModelChoice, Provider},
    tool::ToolDefinition,
};
use async_trait::async_trait;
use lazy_static::lazy_static;
use rig::{
    completion::{self as rc, CompletionModel},
    message as rm,
    providers::openai,
    OneOrMany,
};

use crate::interface::{Plugin, PluginCategory, PluginInfo};

lazy_static! {
    /// List of available OpenAI models
    pub static ref OPENAI_MODELS: Vec<&'static str> = vec![
        openai::GPT_4,
        openai::GPT_4O,
        openai::GPT_4O_MINI,
        openai::GPT_4_TURBO,
        openai::GPT_35_TURBO,
    ];
}

// Implement type convertions

/// Convert `sdk`'s `Message` into `rig`'s `Message`
fn into_rig_message(message: &Message) -> rc::Message {
    match message {
        Message::Assistant(content) => rc::Message::Assistant {
            content: OneOrMany::one(rm::AssistantContent::text(content.clone())),
        },
        Message::User(content) => rc::Message::User {
            content: OneOrMany::one(rm::UserContent::text(content.clone())),
        },
        Message::ToolCall(name, id, params) => rc::Message::Assistant {
            content: OneOrMany::one(rm::AssistantContent::ToolCall(rm::ToolCall {
                id: id.clone(),
                function: rm::ToolFunction {
                    name: name.clone(),
                    arguments: params.clone(),
                },
            })),
        },
        Message::ToolResult(_, id, result) => rc::Message::User {
            content: OneOrMany::one(rm::UserContent::ToolResult(rm::ToolResult {
                id: id.clone(),
                content: OneOrMany::one(rm::ToolResultContent::text(result.to_string())),
            })),
        },
    }
}

/// Convert `rig`'s `ModelChoice` into `sdk`'s `ModelChoice`
fn from_rig_response(response: OneOrMany<rm::AssistantContent>) -> ModelChoice {
    match response.first() {
        rm::AssistantContent::ToolCall(tool_call) => ModelChoice::ToolCall(
            tool_call.function.name,
            tool_call.id,
            tool_call.function.arguments,
        ),
        rm::AssistantContent::Text(text) => ModelChoice::Message(text.text.clone()),
    }
}

/// Convert `sdk`'s `Tool` into `rig`'s `ToolDefinition`
fn into_rig_tool_def(def: &ToolDefinition) -> rig::completion::ToolDefinition {
    rig::completion::ToolDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        parameters: def.parameters.clone(),
    }
}

fn into_rig_request(request: CompletionRequest) -> rc::CompletionRequest {
    rc::CompletionRequest {
        chat_history: request.chat_history.iter().map(into_rig_message).collect(),
        prompt: rm::Message::User {
            content: OneOrMany::one(rm::UserContent::text(request.prompt.clone())),
        },
        preamble: request.system_prompt.clone(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        additional_params: None,
        tools: request.tools.iter().map(into_rig_tool_def).collect(),
        documents: Vec::new(),
    }
}

/// OpenAI provider using `rig-core`
pub struct RigProvider(openai::Client);

impl RigProvider {
    pub fn new(base_url: &str, api_key: &str) -> Result<Self, CreationError>
    where
        Self: Sized,
    {
        Ok(RigProvider(openai::Client::from_url(api_key, base_url)))
    }

    pub fn model_available(&self, model: &str) -> bool {
        // Check if the model name is available
        OPENAI_MODELS.contains(&model)
    }
}

#[async_trait]
impl Provider for RigProvider {
    #[doc = " Completes a prompt with the provider."]
    async fn completion(&self, req: &CompletionRequest) -> Result<ModelChoice, CompletionError> {
        let Self(client) = self;

        if !self.model_available(&req.model) {
            return Err(CompletionError::ModelUnavailable(req.model.clone()));
        }

        let model = client.completion_model(&req.model);

        // Build the rig completion request
        let request = into_rig_request(req.clone());

        // Perform request to the AI model API
        let response = model.completion(request).await;
        tracing::debug!("OpenAI response from rig: {:?}", response);

        // Convert the rig completion response to a ModelChoice
        match response {
            Ok(res) => Ok(from_rig_response(res.choice)),
            Err(err) => {
                tracing::error!("API error: {}", err);
                Err(CompletionError::ApiError)
            }
        }
    }
}

impl Plugin for RigProvider {
    fn info(&self) -> &'static PluginInfo {
        &PluginInfo {
            name: "StdOpenAIProvider",
            category: PluginCategory::Service,
        }
    }
}
