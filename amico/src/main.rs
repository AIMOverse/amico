use std::process::exit;

use rig::{cli_chatbot::cli_chatbot, providers::openai};

fn print_demo_hint() {
    println!("THIS IS ONLY A DEMO VERSION OF AMICO");
    println!("CORE FEATURES ARE UNAVAILABLE IN THIS VERSION");
    println!();
    println!("CHECKOUT OUR DOCS FOR MORE INFORMATION:");
    println!("https://aimoverse.github.io/amico-docs/whitepaper/");
    println!();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print_demo_hint();

    // Check whether the OPENAI_API_KEY environment variable is set
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY environment variable not set");
        println!("Run the following command to set it:");
        println!("\nexport OPENAI_API_KEY=sk-...\n");
        exit(1);
    }

    // Initialize the OpenAI client using environment variables
    let client = openai::Client::from_env();

    // Create an agent instance
    let agent = client
        .agent("gpt-4o-mini")
        .preamble("You are a helpful assistant.")
        .temperature(0.7)
        .max_tokens(1000)
        .build();

    // Start the CLI chatbot
    cli_chatbot(agent).await?;

    Ok(())
}
