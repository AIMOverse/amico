use amico::ai::provider::Provider;
use amico::ai::service::Service;
use amico_plugins::interface::Plugin;
use amico_plugins::std::{providers::openai::OpenAI, service};
use std::io::{self, Write};
use std::process;

fn print_demo_hint() {
    println!("This is only a DEMO VERSION of Amico.");
    println!("Check out our docs for more information:");
    println!("https://www.amico.dev");
    println!();
}

#[tokio::main]
async fn main() {
    print_demo_hint();

    // Read `OPENAI_API_KEY` from environment variable
    let openai_api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: OPENAI_API_KEY is not set");
            process::exit(1);
        }
    };

    let provider = match OpenAI::new(None, Some(&openai_api_key)) {
        Ok(provider) => provider,
        Err(err) => {
            eprintln!("Error creating provider: {err}");
            process::exit(1);
        }
    };

    let mut service = service::Service {
        system_prompt: "You are a helpful assistant.".to_string(),
        temperature: 0.7,
        max_tokens: 1000,
        provider: Box::new(provider),
    };

    println!("Using service plugin: {}", service.info().name);

    loop {
        print!("Enter your message: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            println!("Exiting chatbot. Goodbye!");
            break;
        }

        // Mock response from AI service
        let response = match service.generate_text(input.to_string()).await {
            Ok(response) => response,
            Err(err) => {
                eprintln!("Error generating text: {err}");
                continue;
            }
        };
        println!("AI: {}", response);
    }
}
