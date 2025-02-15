use amico_core::controller::Agent;
use amico_core::traits::{ActionSelector, EventGenerator};
use amico_lib::impls::ActionSelectorImpl;
use amico_lib::impls::EventGeneratorImpl;

fn print_demo_hint() {
    println!("THIS IS ONLY A DEMO VERSION OF AMICO");
    println!();
    println!("CHECKOUT OUR DOCS FOR MORE INFORMATION:");
    println!("https://www.amico.dev");
    println!();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print_demo_hint();

    let eg_factory = Box::new(|| Box::new(EventGeneratorImpl) as Box<dyn EventGenerator + Send>);
    let as_factory = Box::new(|| Box::new(ActionSelectorImpl) as Box<dyn ActionSelector + Send>);

    // Create an agent from the configuration file
    let agent = Agent::new("src/config/config.toml", eg_factory, as_factory);

    // Start the agent
    agent.start();
    agent.join();

    // Stop the agent
    // agent.stop();

    Ok(())
}
