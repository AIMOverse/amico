/*
use amico_core::errors::ActionSelectorError;

#[tokio::test]
async fn test_action_selector() -> Result<(), ActionSelectorError> {

    // Read `OPENAI_API_KEY` from environment variable
    let openai_api_key = "OPENAI_API_KEY";

    // Read base url configuration
    let base_url = "https://api.openai.com/v1/chat/completions";

    // Create a new OpenAI provider
    let provider = OpenAI::new(Option::from(base_url), Some(&openai_api_key)).unwrap();

    // Create a new Service
    let service = service::InMemoryService::new(
        CompletionConfig {
            system_prompt: "".to_string(),
            temperature: 0.2,
            max_tokens: 1000,
            model: "gpt-4o".to_string(),
        },
        Box::new(provider),
        ToolSet::from(vec![]),
    );

    // create a clean action
    let clean_action = AIAction::new(
        "clean".to_string(),
        "Clean the current room".to_string(),
        Value::Object(Default::default()),
        serde_json::json!({}),
        serde_json::json!({}),
        serde_json::json!({}),
        |_params: Value| -> Result<(), ActionError> {
            println!("Cleaning the room");
            Ok(())
        },
    );

    // create a move action
    let move_action = AIAction::new(
        "move".to_string(),
        "Move to the next room".to_string(),
        Value::Object(Default::default()),
        serde_json::json!({}),
        serde_json::json!({}),
        serde_json::json!({}),
        |_params: Value| -> Result<(), ActionError> {
            println!("Moving to the next room");
            Ok(())
        },
    );


    // create an action map
    let mut action_map = ActionMap::new();

    action_map.add_action(clean_action);
    action_map.add_action(move_action);

    // create a new Model
    let model = Model::default();

    // create a new action selector
    let mut action_selector = ActionSelector::new(action_map,
                                                  Box::new(service),
                                                  Box::new(model));
    let (action, event_ids) = action_selector.select_action(vec![])?;

    let _ = action.execute();
    assert_eq!(event_ids.len(), 0);
    // TODO make the test works
    Ok(())
}
*/
