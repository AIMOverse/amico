// Models may be used to serialize and deserialize data from the LLM Service API.
use serde::{Deserialize, Serialize};

// Response from LLM API
#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
}

// Tool Calls
#[derive(Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub args: serde_json::Value, // validate before calling
}
