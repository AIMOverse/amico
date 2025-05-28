use std::process;
use std::sync::Arc;

use amico::ai::services::ServiceBuilder;
use amico::resource::Resource;
use amico_core::{Agent, OnFinish};
use amico_mods::interface::Plugin;
use amico_mods::runtime::storage::fs::FsStorage;
use amico_mods::std::ai::providers::rig::{RigProvider, providers};
use amico_mods::std::ai::services::InMemoryService;
use amico_mods::web3::solana::balance::BalanceSensor;
use amico_mods::web3::solana::client::{SolanaClient, SolanaClientResource};
use amico_mods::web3::solana::trade::TradeEffector;
use amico_mods::web3::wallet::Wallet;
use colored::Colorize;
use engine::a2a::A2aModule;
use engine::components::{AiService, Recorder};
use engine::dispatcher::MatchDispatcher;
use engine::interaction::create_cli_client;
use engine::systems::{ChatbotSystem, CompletionSystem, SpeechSystem};
use helpers::solana_rpc_url;
use prompt::AMICO_SYSTEM_PROMPT;
use tokio::sync::Mutex;

mod audio;
mod engine;
mod helpers;
mod prompt;

const DEFAULT_OPENAI_BASE_URL: &str = "https://api.openai.com/v1";

fn print_demo_hint() {
    println!("{}", "This is only a PROTOTYPE VERSION of Amico.".yellow());
    print!("Check out our docs for more information: ");
    println!("{}", "https://amico.dev".blue());
    println!();
}

#[tokio::main]
async fn main() {
    print_demo_hint();

    // Initialize tracing
    // `export RUST_LOG=debug`
    if std::env::var("RUST_LOG").is_ok() {
        tracing_subscriber::fmt::init();
    }

    // Read `OPENAI_API_KEY` from environment variable
    let openai_api_key = std::env::var("OPENAI_API_KEY")
        .inspect(|_| println!("{}", "✔ Found OPENAI_API_KEY".green()))
        .unwrap_or_else(|_| {
            eprintln!("{}", "Error: OPENAI_API_KEY is not set".red());
            process::exit(1);
        });

    // Read base url configuration
    let base_url = std::env::var("OPENAI_BASE_URL")
        .inspect(|_| println!("{}", "✔ Found OPENAI_BASE_URL".green()))
        .unwrap_or_else(|_| {
            println!("{}", "✔ Using default OPENAI_BASE_URL".green());
            DEFAULT_OPENAI_BASE_URL.to_string()
        });

    // Read Helius API key
    if std::env::var("HELIUS_API_KEY").is_ok() {
        println!("{}", "✔ Found HELIUS_API_KEY".green());
    } else {
        println!("{}", "WARNING: Helius API key not found.".yellow());
        println!("We recommend you to use Helius API for on-chain actions.");
        println!("The default Solana RPC is not stable enough.");
        println!("Check out https://helius.dev for more information.");
        println!();
    }

    let fs_store = FsStorage::new(".amico/storage")
        .inspect_err(|err| {
            eprintln!("{}", "Error loading FS storage".red());
            tracing::error!("Error loading FS storage: {}", err);
            process::exit(1);
        })
        .unwrap();
    let storage_resource =
        Resource::new("Dev FS Storage".to_string(), Arc::new(Mutex::new(fs_store)));

    // Load agent wallet
    let wallet = Wallet::load_or_save_new("agent_wallet.txt")
        .inspect(|_| println!("{}", "✔ Loaded agent wallet".green()))
        .unwrap_or_else(|err| {
            eprintln!("Error loading wallet: {err}");
            process::exit(1);
        });
    // Make wallet a resource
    let wallet = Resource::new("wallet".to_string(), wallet);

    // Create Client resource
    let client = SolanaClientResource::new(
        "Client resource".to_string(),
        SolanaClient::new(solana_rpc_url("devnet").as_str()),
    );

    // Create BalanceSensor instance
    let balance_sensor = Resource::new(
        "balance_sensor".to_string(),
        BalanceSensor::new(client.clone(), wallet.clone()),
    );

    // Create TradeEffector instance
    let trade_effector = Resource::new(
        "TradeEffector".to_string(),
        TradeEffector::new(client.clone(), wallet.clone()),
    );

    // Initialize a2a module
    let a2a = A2aModule::new(wallet.clone(), storage_resource);

    // Connect to the network
    if let Err(e) = a2a.connect().await {
        tracing::error!("{}", e);
        eprintln!("Failed to connect to a2a network");
        process::exit(1);
    }

    // Create the Provider
    let provider = RigProvider::openai(providers::openai::Client::from_url(
        &openai_api_key,
        &base_url,
    ));

    // Create the Service
    let service = ServiceBuilder::new(provider)
        .model("gpt-4o".to_string())
        .system_prompt(AMICO_SYSTEM_PROMPT.to_string())
        .temperature(0.2)
        .max_tokens(1000)
        .tool(balance_sensor.value().agent_wallet_balance_tool())
        .tool(balance_sensor.value().account_balance_tool())
        .tool(trade_effector.value().tool())
        .tool(a2a.send_message_tool())
        .tool(a2a.contact_list_tool())
        .build::<InMemoryService<RigProvider>>();

    println!();
    println!("Agent wallet addresses:");
    println!("{}", wallet.value().pubkey_list());

    println!();
    println!("Using service plugin: {}", service.info().name);
    println!("Tools enabled:\n{}", service.ctx.tools.describe());

    // Initialize ECS
    let mut agent = Agent::new(MatchDispatcher);

    let int_layer = agent.wm.int_layer();
    let ai_layer = agent.wm.ai_layer();
    let env_layer = agent.wm.env_layer();

    let (cli_component, cli_event_source) = create_cli_client();

    agent.wm.add_component(int_layer, cli_component);
    agent.wm.add_component(ai_layer, AiService::new(service));
    agent.wm.add_component(env_layer, Recorder::new());

    agent.wm.register_system(CompletionSystem { ai_layer });
    agent.wm.register_system(SpeechSystem {
        env_layer,
        user_mp3_path: ".amico/cache/user.mp3",
        agent_mp3_path: ".amico/cache/agent.mp3",
    });
    agent.wm.register_system(ChatbotSystem {
        env_layer,
        int_layer,
    });

    agent.spawn_event_source(a2a, OnFinish::Continue);
    agent.spawn_event_source(cli_event_source, OnFinish::Stop);

    agent.run().await;
}
