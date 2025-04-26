use amico::ai::services::ServiceBuilder;
use amico::resource::Resource;
use amico::task::Task;
use amico_mods::interface::Plugin;
use amico_mods::std::ai::providers::rig::{providers, RigProvider};
use amico_mods::std::ai::services::InMemoryService;
use amico_mods::std::ai::tasks::chatbot::cli::CliTask;
use amico_mods::web3::solana::balance::BalanceSensor;
use amico_mods::web3::solana::resources::SolanaClientResource;
use amico_mods::web3::solana::trade::TradeEffector;
use amico_mods::web3::wallet::Wallet;
use colored::Colorize;
use helpers::solana_rpc_url;
use prompt::AMICO_SYSTEM_PROMPT;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signer::Signer;
use std::process;
use tools::{balance_sensor_tool, trade_effector_tool};

mod helpers;
mod prompt;
mod tools;

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
        RpcClient::new(solana_rpc_url("devnet")),
    );

    // Create BalanceSensor instance
    let balance_sensor = Resource::new(
        "balance_sensor".to_string(),
        BalanceSensor::new(client.clone()),
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
        .tool(balance_sensor_tool(
            balance_sensor.clone(),
            &wallet.value().solana().pubkey(),
        ))
        .tool(trade_effector_tool(trade_effector.clone()))
        .build::<InMemoryService<RigProvider>>();

    println!();
    println!("Agent wallet addresses:");
    wallet.value().pubkey_list();

    println!();
    println!("Using service plugin: {}", service.info().name);
    println!("Tools enabled:\n{}", service.ctx.tools.describe());

    // Create a task
    let mut task = CliTask::new(service);

    // Run the task in execution order. If encounter error, re-run the task.
    task.before_run().await.unwrap_or_else(|err| {
        eprintln!("Error during {task:?}.before_run");
        tracing::error!("{err}");
    });

    while let Err(e) = task.run().await {
        eprintln!("Error running task. Re-running");
        tracing::error!("Error running task: {:?}", e);
        continue;
    }

    task.after_run().await.unwrap_or_else(|err| {
        eprintln!("Error during {task:?}.after_run");
        tracing::error!("{err}");
    });
}
