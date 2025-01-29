use amico::impls::ActionSelectorImpl;
use amico::impls::EventGeneratorImpl;
// Code for the main function of the amico agent
use amico_core::controller::Agent;


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

    // Create an agent from the configuration file
    let mut agent = Agent::new("src/config/config.toml", Box::new(EventGeneratorImpl), Box::new(ActionSelectorImpl::new()));

    // Start the agent
    agent.start();

    // Perform other tasks...

    // Stop the agent
    agent.stop();

    Ok(())
}
