use amico::ai::tool::Tool;
use serde_json::json;

pub fn search_jokes_tool() -> Tool {
    Tool {
        name: "search_for_jokes".to_string(),
        description: "Search for jokes".to_string(),
        parameters: json!({}),
        tool_call: Box::new(|_| {
            println!("Calling search_for_jokes tool");
            Ok(json!({
                "jokes": [
                    "Why don't scientists trust atoms?\nBecause they make up everything!",
                    "Why don't skeletons fight each other?\nBecause they don't have the guts!",
                ],
            }))
        }),
    }
}
