use std::fmt::Debug;

use amico::ai::{
    errors::CompletionModelError,
    message::Message,
    models::{CompletionModel, CompletionRequest, ModelChoice},
    tool::ToolDefinition,
};
use async_trait::async_trait;
use rig::{
    completion::{self as rc, CompletionModel as _},
    message as rm, providers as rp, OneOrMany,
};

use crate::interface::{Plugin, PluginCategory, PluginInfo};

/// Re-export providers from rig-core
pub use rig::providers;

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

/// Convert `rig`'s `CompletionResponse` into `amico`'s `ModelChoice`
fn into_amico_choice<T>(response: rc::CompletionResponse<T>) -> ModelChoice {
    match response.choice.first() {
        rm::AssistantContent::ToolCall(tool_call) => ModelChoice::ToolCall(
            tool_call.function.name,
            tool_call.id,
            tool_call.function.arguments,
        ),
        rm::AssistantContent::Text(text) => ModelChoice::Message(text.text.clone()),
    }
}

/// Convert `rig`'s `CompletionError` into `amico`'s `CompletionModelError`
fn into_amico_err(error: rc::CompletionError) -> CompletionModelError {
    CompletionModelError::ProviderError(error.to_string())
}

/// Convert `amico`'s `Tool` into `rig`'s `ToolDefinition`
fn into_rig_tool_def(def: &ToolDefinition) -> rig::completion::ToolDefinition {
    rig::completion::ToolDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        parameters: def.parameters.clone(),
    }
}

/// Convert `amico`'s `CompletionRequest` into `rig`'s
fn into_rig_request(request: &CompletionRequest) -> rc::CompletionRequest {
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

/// The uniform completion wrapper for all rig providers
async fn provider_completion(
    provider: &RigProvider,
    model_name: &str,
    request: rc::CompletionRequest,
) -> Result<ModelChoice, CompletionModelError> {
    match provider {
        RigProvider::Anthropic(client) => client
            .completion_model(model_name)
            .completion(request)
            .await
            .map(into_amico_choice)
            .map_err(into_amico_err),
        RigProvider::Deepseek(client) => client
            .completion_model(model_name)
            .completion(request)
            .await
            .map(into_amico_choice)
            .map_err(into_amico_err),
        RigProvider::Gemini(client) => client
            .completion_model(model_name)
            .completion(request)
            .await
            .map(into_amico_choice)
            .map_err(into_amico_err),
        RigProvider::Openai(client) => client
            .completion_model(model_name)
            .completion(request)
            .await
            .map(into_amico_choice)
            .map_err(into_amico_err),
    }
}

/// OpenAI provider using `rig-core`
pub enum RigProvider {
    Anthropic(rp::anthropic::Client),
    Deepseek(rp::deepseek::Client),
    Gemini(rp::gemini::Client),
    Openai(rp::openai::Client),
}

impl Debug for RigProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}",
            match self {
                Self::Anthropic(_) => "anthropic",
                Self::Deepseek(_) => "deepseek",
                Self::Gemini(_) => "gemini",
                Self::Openai(_) => "openai",
            }
        ))
    }
}

impl RigProvider {
    pub fn anthropic(client: rp::anthropic::Client) -> Self {
        Self::Anthropic(client)
    }

    pub fn deepseek(client: rp::deepseek::Client) -> Self {
        Self::Deepseek(client)
    }

    pub fn gemini(client: rp::gemini::Client) -> Self {
        Self::Gemini(client)
    }

    pub fn openai(client: rp::openai::Client) -> Self {
        Self::Openai(client)
    }
}

#[async_trait]
impl CompletionModel for RigProvider {
    #[doc = " Completes a prompt with the provider."]
    async fn completion(
        &self,
        req: &CompletionRequest,
    ) -> Result<ModelChoice, CompletionModelError> {
        provider_completion(self, &req.model, into_rig_request(req)).await
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
