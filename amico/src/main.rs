use amico::ai::service::{Service, ServiceBuilder};
use amico::resource::Resource;
use amico_mods::interface::Plugin;
use amico_mods::std::ai::providers::RigProvider;
use amico_mods::std::ai::services::InMemoryService;
use amico_mods::web3::wallet::Wallet;
use colored::Colorize;
use prompt::AMICO_SYSTEM_PROMPT;
use std::process;
use std::sync::Arc;
use tasks::audio::AudioChatTask;
use tasks::interface::{Task, TaskContext};

mod prompt;
mod tasks;

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
    let base_url = match std::env::opvar("OPENAI_BASE_URL") {
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
    let wallet = Wallet::load_or_save_new("agent_wallet.txt").unwrap_or_else(|err| {
        eprintln!("Error loading wallet: {err}");
        process::exit(1);
    });
    // Make wallet a resource
    let wallet = Arc::new(Resource::new("wallet".to_string(), wallet));

    println!();
    println!("Agent wallet addresses:");
    wallet.value().print_all_pubkeys();

    let provider = RigProvider::new(&base_url, &openai_api_key).unwrap_or_else(|err| {
        eprintln!("Error creating provider: {err}");
        process::exit(1);
    });

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
    loop {
        if let Err(e) = task.run().await {
            eprintln!("Error running task. Re-running");
            tracing::error!("Error running task: {:?}", e);
            continue;
        } else {
            break;
        }
    }
}
