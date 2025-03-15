use amico::ai::service::{Service, ServiceBuilder};
use amico_mods::interface::Plugin;
use amico_mods::std::ai::providers::RigProvider;
use amico_mods::std::ai::services::InMemoryService;
use colored::Colorize;
use prompt::AMICO_SYSTEM_PROMPT;
use std::process;
use tasks::audio::AudioChatTask;
use tasks::interface::{Task, TaskContext};
use tools::{
    buy_solana_token_tool, check_ethereum_balance, check_solana_balance, create_asset_tool,
    search_jokes_tool,
};
use wallets::AgentWallet;

mod prompt;
mod tasks;
mod tools;
mod utils;
mod wallets;

const DEFAULT_OPENAI_BASE_URL: &str = "https://api.openai.com/v1";

fn print_demo_hint() {
    println!("{}", "This is only a PROTOTYPE VERSION of Amico.".yellow());
    println!("Check out our docs for more information:");
    println!("https://www.amico.dev");
    println!();
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
            println!("Using default OPENAI_BASE_URL ({DEFAULT_OPENAI_BASE_URL})");
            DEFAULT_OPENAI_BASE_URL.to_string()
        }
    };

    if std::env::var("HELIUS_API_KEY").is_ok() {
        println!("Found HELIUS_API_KEY");
    } else {
        println!("{}", "WARNING: Helius API key not found.".yellow());
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

    let provider = match RigProvider::new(&base_url, &openai_api_key) {
        Ok(provider) => provider,
        Err(err) => {
            eprintln!("Error creating provider: {err}");
            process::exit(1);
        }
    };

    let service = ServiceBuilder::new(provider)
        .model("gpt-4o".to_string())
        .system_prompt(AMICO_SYSTEM_PROMPT.to_string())
        .temperature(0.2)
        .max_tokens(1000)
        .tool(search_jokes_tool())
        .tool(check_solana_balance(wallet.solana_keypair().unwrap()))
        .tool(check_ethereum_balance(wallet.ethereum_wallet().unwrap()))
        .tool(create_asset_tool(wallet.solana_keypair().unwrap()))
        .tool(buy_solana_token_tool(wallet.solana_keypair().unwrap()))
        .build::<InMemoryService<RigProvider>>();

    println!();
    println!("Using service plugin: {}", service.info().name);
    println!("Tools enabled:\n{}", service.ctx().tools.describe());

    // Create a task
    //let mut task = CliTask::setup(TaskContext::new(service)).unwrap_or_else(|e| {
    let mut task = AudioChatTask::setup(TaskContext::new(service)).unwrap_or_else(|e| {
        eprintln!("Error creating task: {e}");
        process::exit(1);
    });

    // Run the task
    if let Err(e) = task.run().await {
        eprintln!("Error running task: {e}");
        process::exit(1);
    }
}
