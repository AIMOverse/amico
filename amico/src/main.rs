use amico::ai::service::{Service, ServiceBuilder};
use amico_plugins::interface::Plugin;
use amico_plugins::std::providers::openai::OpenAI;
use amico_plugins::std::service::InMemoryService;
use colored::Colorize;
use prompt::AMICO_SYSTEM_PROMPT;
use std::io::{self, Write};
use std::process;
use tools::{
    buy_solana_token_tool, check_ethereum_balance, check_solana_balance, create_asset_tool,
    search_jokes_tool,
};
use wallets::AgentWallet;

mod prompt;
mod tools;
mod utils;
mod wallets;

fn print_demo_hint() {
    println!("This is only a DEMO VERSION of Amico.");
    println!("Check out our docs for more information:");
    println!("https://www.amico.dev");
    println!();
}

fn print_message_separator() {
    println!("--------------------");
}

#[tokio::main]
async fn main() {
    print_demo_hint();

    // Initialize tracing
    // `export RUST_LOG=debug`
    tracing_subscriber::fmt::init();

    // Read `OPENAI_API_KEY` from environment variable
    let openai_api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(key) => {
            println!("Found OPENAI_API_KEY");
            key
        }
        Err(_) => {
            eprintln!("Error: OPENAI_API_KEY is not set");
            process::exit(1);
        }
    };

    // Read base url configuration
    let base_url = match std::env::var("OPENAI_BASE_URL") {
        Ok(key) => {
            println!("Found OPENAI_BASE_URL");
            key
        }
        Err(_) => {
            eprintln!("Error: OPENAI_BASE_URL is not set");
            process::exit(1);
        }
    };

    if let Ok(proxy) = std::env::var("HTTP_PROXY") {
        tracing::debug!("Using HTTP proxy: {proxy}");
    }

    if let Ok(proxy) = std::env::var("HTTPS_PROXY") {
        tracing::debug!("Using HTTPS proxy: {proxy}");
    }

    if std::env::var("HELIUS_API_KEY").is_ok() {
        println!("Found HELIUS_API_KEY");
    } else {
        println!("WARNING: Helius API key not found.");
        println!("We recommend you to use Helius API for on-chain actions.");
        println!("The default Solana RPC is not stable enough.");
        println!("Check out https://helius.dev for more information.");
        println!();
    }

    // Load agent wallet
    let wallet = match AgentWallet::load_or_save_new("agent_wallet.txt") {
        Ok(wallet) => wallet,
        Err(err) => {
            eprintln!("Error loading agent wallet: {err}");
            process::exit(1);
        }
    };

    println!();
    println!("Agent wallet addresses:");
    if let Err(e) = wallet.print_all_pubkeys() {
        eprintln!("Error printing public keys: {e}");
        process::exit(1);
    }

    let provider = match OpenAI::new(&base_url, &openai_api_key) {
        Ok(provider) => provider,
        Err(err) => {
            eprintln!("Error creating provider: {err}");
            process::exit(1);
        }
    };

    let mut service = ServiceBuilder::new(provider)
        .model("gpt-4o".to_string())
        .system_prompt(AMICO_SYSTEM_PROMPT.to_string())
        .temperature(0.2)
        .max_tokens(1000)
        .tool(search_jokes_tool())
        .tool(check_solana_balance(wallet.solana_keypair().unwrap()))
        .tool(check_ethereum_balance(wallet.ethereum_wallet().unwrap()))
        .tool(create_asset_tool(wallet.solana_keypair().unwrap()))
        .tool(buy_solana_token_tool(wallet.solana_keypair().unwrap()))
        .build::<InMemoryService<OpenAI>>();

    println!();
    println!("Using service plugin: {}", service.info().name);
    println!("Tools enabled:\n{}", service.ctx().tools.describe());

    // Print global prompt
    println!();
    println!(
        "{}",
        "I'm Amico, your personal AI assistant. How can I assist you today?".green()
    );
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
        println!("{}", "[Amico]".yellow());
        println!("{}", response.green());
        print_message_separator();
    }
}
