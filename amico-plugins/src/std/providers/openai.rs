use amico::ai::{
    chat::{ChatHistory, Message},
    errors::{CompletionError, CreationError},
    provider::{CompletionConfig, ModelChoice, Provider},
    tool::{Tool, ToolSet},
};
use async_trait::async_trait;
use lazy_static::lazy_static;
use rig::{
    completion::{CompletionModel, CompletionRequest},
    providers::openai,
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
fn into_rig_message(message: &Message) -> rig::completion::Message {
    rig::completion::Message {
        role: message.role.clone(),
        content: message.content(),
    }
}

/// Convert `rig`'s `ModelChoice` into `sdk`'s `ModelChoice`
fn from_rig_choice(choice: rig::completion::ModelChoice) -> ModelChoice {
    match choice {
        rig::completion::ModelChoice::ToolCall(name, id, params) => {
            ModelChoice::ToolCall(name, id, params)
        }
        rig::completion::ModelChoice::Message(msg) => ModelChoice::Message(msg),
    }
}

/// Convert `sdk`'s `Tool` into `rig`'s `ToolDefinition`
fn into_rig_tool_def(tool: &Tool) -> rig::completion::ToolDefinition {
    rig::completion::ToolDefinition {
        name: tool.name.clone(),
        description: tool.description.clone(),
        parameters: tool.parameters.clone(),
    }
}

/// OpenAI provider using `rig-core`
pub struct OpenAI(openai::Client);

#[async_trait]
impl Provider for OpenAI {
    #[doc = " Creates a new provider."]
    fn new(base_url: &str, api_key: &str) -> Result<Self, CreationError>
    where
        Self: Sized,
    {
        Ok(OpenAI(openai::Client::from_url(api_key, base_url)))
    }

    #[doc = " Completes a prompt with the provider."]
    async fn completion(
        &self,
        prompt: &str,
        config: &CompletionConfig,
        chat_history: &ChatHistory,
        tools: &ToolSet,
    ) -> Result<ModelChoice, CompletionError> {
        let Self(client) = self;

        if !self.model_available(&config.model).await {
            return Err(CompletionError::ModelUnavailable(config.model.clone()));
        }

        let model = client.completion_model(&config.model);

        // Build the rig completion request
        let request = CompletionRequest {
            chat_history: chat_history.iter().map(into_rig_message).collect(),
            prompt: prompt.to_string(),
            preamble: Some(config.system_prompt.clone()),
            temperature: Some(config.temperature),
            max_tokens: Some(config.max_tokens),
            additional_params: None,
            tools: tools.tools.values().map(into_rig_tool_def).collect(),
            documents: Vec::new(),
        };

        // Perform request to the AI model API
        let response = model.completion(request).await;
        // tracing::debug!("OpenAI response: {:?}", response);

        // Convert the rig completion response to a ModelChoice
        match response {
            Ok(res) => Ok(from_rig_choice(res.choice)),
            Err(err) => {
                tracing::error!("API error: {}", err);
                Err(CompletionError::ApiError)
            }
        }
    }

    #[doc = " Checks if a model name is available."]
    async fn model_available(&self, model: &str) -> bool {
        // Check if the model name is available
        OPENAI_MODELS.contains(&model)
    }
}

impl Plugin for OpenAI {
    fn info(&self) -> &'static PluginInfo {
        &PluginInfo {
            name: "StdOpenAIProvider",
            category: PluginCategory::Service,
        }
    }
}
