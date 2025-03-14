use amico::ai::tool::{Tool, ToolBuilder};
use serde_json::json;

pub fn search_jokes_tool() -> Tool {
    ToolBuilder::new()
        .name("search_for_jokes")
        .description("Search for jokes")
        .parameters(json!({}))
        .build(|_| {
            tracing::info!("Calling search_for_jokes tool");
            Ok(json!({
                "jokes": [
                    "Why don't scientists trust atoms?\nBecause they make up everything!",
                    "Why do programmers prefer dark mode?\nBecause the light attracts bugs!",
                    "Why did the TCP connection break up with UDP?\nBecause TCP wanted a reliable connection, but UDP just couldn't commit!",
                    "Why do UDP packets never get invited to parties?\nBecause they never respond to invites!",
                ],
            }))
        })
}
