use std::process;
use std::sync::mpsc::channel;
use std::sync::Arc;

use amico::ai::services::{CompletionServiceDyn, ServiceBuilder};
use amico::resource::Resource;
use amico_mods::interface::Plugin;
use amico_mods::std::ai::providers::rig::{providers, RigProvider};
use amico_mods::std::ai::services::InMemoryService;
use amico_mods::web3::solana::std::balance::BalanceSensor;
use amico_mods::web3::solana::std::client::{SolanaClient, SolanaClientResource};
use amico_mods::web3::solana::std::trade::TradeEffector;
use amico_mods::web3::wallet::Wallet;
use colored::Colorize;
use engine::actions::StdioOutputAction;
use engine::agent::EcsAgent;
use engine::components::AiService;
use engine::event_source::StdioEventSource;
use engine::events::UserContent;
use evenio::prelude::*;
use helpers::solana_rpc_url;
use prompt::AMICO_SYSTEM_PROMPT;
use tokio::sync::Mutex;

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
        .inspect(|_| println!("Found OPENAI_API_KEY"))
        .unwrap_or_else(|_| {
            eprintln!("Error: OPENAI_API_KEY is not set");
            process::exit(1);
        });

    // Read base url configuration
    let base_url = std::env::var("OPENAI_BASE_URL")
        .inspect(|_| println!("Found OPENAI_BASE_URL"))
        .unwrap_or_else(|_| {
            println!("Using default OPENAI_BASE_URL ({DEFAULT_OPENAI_BASE_URL})");
            DEFAULT_OPENAI_BASE_URL.to_string()
        });

    // Read Helius API key
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
    let wallet = Wallet::load_or_save_new("agent_wallet.txt")
        .inspect(|_| println!("Loaded agent wallet"))
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
        .build::<InMemoryService<RigProvider>>();

    println!();
    println!("Agent wallet addresses:");
    println!("{}", wallet.value().pubkey_list());

    println!();
    println!("Using service plugin: {}", service.info().name);
    println!("Tools enabled:\n{}", service.ctx.tools.describe());

    // Initialize ECS
    let mut world = World::new();

    let interaction = world.spawn();
    let ai_layer = world.spawn();

    world.insert(interaction, StdioEventSource);
    world.insert(interaction, StdioOutputAction);
    world.insert(ai_layer, AiService(Arc::new(Mutex::new(service))));

    world.add_handler(
        move |r: Receiver<UserContent>,
              it_fetcher: Fetcher<&StdioOutputAction>,
              ai_fetcher: Fetcher<&AiService>| {
            let output = it_fetcher.get(interaction).unwrap();
            let service = ai_fetcher.get(ai_layer).unwrap().0.clone();
            let UserContent(text) = r.event;
            let text = text.to_string();

            // Spawning an async task in tokio to run `generate_text`.
            // We can't await a JoinHandle in sync block, so use a channel instead.
            let (tx, rx) = channel::<String>();
            tokio::spawn(async move {
                let response = service
                    .lock()
                    .await
                    .generate_text_dyn(text)
                    .await
                    .unwrap_or_else(|err| format!("Service error: {:?}", err.to_string()));

                tx.send(response).unwrap();
            });

            let response = rx.recv().unwrap();
            output.print_message(response);
        },
    );

    let agent = EcsAgent::new(&mut world, interaction);
    if let Err(e) = agent.run().await {
        tracing::error!("{e}");
    }
}
