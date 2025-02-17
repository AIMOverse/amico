use amico::ai::tool::Tool;
use serde_json::json;

pub fn trade_solana_token_tool() -> Tool {
    Tool {
        name: "trade_solana_token".to_string(),
        description: "Trade for a Solana token".to_string(),
        parameters: json!({}),
        tool_call: Box::new(|_| {
            tracing::info!("Calling trade_solana_token tool");
            Ok(json!({}))
        }),
    }
}
