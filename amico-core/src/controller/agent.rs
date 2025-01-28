use crate::config::{Config, CoreConfig};

/// Struct representing an agent.
pub struct Agent {
    name: String,
    is_running: bool,
}

impl Agent {
    /// Creates a new agent with the given name.
    pub fn new(config_path: &str) -> Self {
        let config = Self::load_config(config_path);
        Self {
            name: config.name,
            is_running: false,
        }
    }

    /// Function to be called before starting the agent.
    pub fn before_start(&self) {
        println!("Loading objects for agent: {}", self.name);
        // Add any necessary pre-start logic here
    }

    /// Starts the agent.
    pub fn start(&mut self) {
        self.before_start();
        self.is_running = true;
        println!("Agent {} started.", self.name);
        // Add any necessary start logic here
    }

    /// Stops the agent.
    pub fn stop(&mut self) {
        self.is_running = false;
        println!("Agent {} stopped.", self.name);
        // Add any necessary stop logic here
    }

    fn load_config(config_path: &str) -> CoreConfig {
        let config_str = std::fs::read_to_string(&config_path).unwrap();
        CoreConfig::from_toml_str(&config_str).unwrap()
    }
}
