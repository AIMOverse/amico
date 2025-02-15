use amico::ai::provider::{CompletionConfig, Provider};
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

fn print_message_separator() {
    println!("--------------------");
}

const AMICO_SYSTEM_PROMPT: &str = "You are Amico, a virtual assistant.";

#[tokio::main]
async fn main() {
    print_demo_hint();

    // Read `OPENAI_API_KEY` from environment variable
    let openai_api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(key) => {
            println!("Found $OPENAI_API_KEY");
            key
        }
        Err(_) => {
            eprintln!("Error: OPENAI_API_KEY is not set");
            process::exit(1);
        }
    };

    match std::env::var("HTTP_PROXY") {
        Ok(proxy) => {
            println!("Using HTTP proxy: {proxy}");
        }
        Err(_) => (),
    }

    match std::env::var("HTTPS_PROXY") {
        Ok(proxy) => {
            println!("Using HTTPS proxy: {proxy}");
        }
        Err(_) => (),
    }

    let provider = match OpenAI::new(None, Some(&openai_api_key)) {
        Ok(provider) => provider,
        Err(err) => {
            eprintln!("Error creating provider: {err}");
            process::exit(1);
        }
    };

    let mut service = service::InMemoryService::new(
        CompletionConfig {
            system_prompt: AMICO_SYSTEM_PROMPT.to_string(),
            temperature: 0.7,
            max_tokens: 1000,
            model: "gpt-4o".to_string(),
        },
        Box::new(provider),
    );

    println!("Using service plugin: {}", service.info().name);

    // Print global prompt
    println!();
    println!("Hi! I'm Amico, your personal AI assistant. How can I assist you today?");
    print_message_separator();

    loop {
        println!("Enter your message");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            println!("Exiting chatbot. Goodbye!");
            break;
        }

        print_message_separator();

        // Get response from AI service
        let response = match service.generate_text(input.to_string()).await {
            Ok(response) => response,
            Err(err) => {
                eprintln!("Error generating text: {err}");
                continue;
            }
        };
        println!("[Amico]\n{}", response);
        print_message_separator();
    }
}
