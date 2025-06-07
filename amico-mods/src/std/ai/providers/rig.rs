use amico::ai::completion::{Error as CompletionError, Model, ModelChoice, Request};
use rig::{
    completion::{self as rc, CompletionModel as _},
    providers as rp,
};
use std::fmt::Debug;

use super::rig_helpers::*;

/// Re-export providers from rig-core
/// so that SDK users do not need to add `rig-core` as a dependency
pub use rig::providers;

// Implement type convertions

/// The uniform completion wrapper for all rig providers
async fn provider_completion(
    provider: &RigProvider,
    model_name: &str,
    request: rc::CompletionRequest,
) -> Result<ModelChoice, CompletionError> {
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
            "RigProvider::{}",
            match self {
                Self::Anthropic(_) => "Anthropic",
                Self::Deepseek(_) => "Deepseek",
                Self::Gemini(_) => "Gemini",
                Self::Openai(_) => "Openai",
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

impl Model for RigProvider {
    #[doc = " Completes a prompt with the provider."]
    async fn completion(&self, req: &Request) -> Result<ModelChoice, CompletionError> {
        provider_completion(self, &req.model, into_rig_request(req)).await
    }
}
