use crate::config::{Config, CoreConfig};
use crate::entities::Event;
use crate::traits::{ActionSelector, EventGenerator};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

/// Struct representing an agent.
pub struct Agent {
    name: String,
    is_running: Arc<AtomicBool>,
    event_generator: Arc<Box<dyn EventGenerator>>,
    action_selector: Arc<Box<dyn ActionSelector>>,
    events: Arc<Mutex<Vec<Event>>>,
}

impl Agent {
    /// Creates a new agent with the given name.
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the configuration file.
    /// * `event_generator` - An instance of `EventGenerator`.
    /// * `action_selector` - An instance of `ActionSelector`.
    ///
    /// # Returns
    ///
    /// * `Agent` - A new instance of `Agent`.
    pub fn new(
        config_path: &str,
        event_generator: Box<dyn EventGenerator + Send>,
        action_selector: Box<dyn ActionSelector + Send>,
    ) -> Self {
        let config = Self::load_config(config_path);
        Self {
            name: config.name,
            is_running: Arc::new(AtomicBool::new(false)),
            event_generator: Arc::new(event_generator),
            action_selector: Arc::new(action_selector),
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Function to be called before starting the agent.
    pub fn before_start(&self) {
        println!("Loading objects for agent: {}", self.name);
    }

    /// Starts the agent.
    pub fn start(&self) {
        self.before_start();
        self.is_running.store(true, Ordering::SeqCst);
        println!("Agent {} started.", self.name);

        let is_running = Arc::clone(&self.is_running);
        let event_generator = Arc::clone(&self.event_generator);
        let events = Arc::clone(&self.events);

        // Event generation thread
        let generator_handle = thread::spawn(move || {
            while is_running.load(Ordering::SeqCst) {
                let new_events =
                    event_generator.generate_event("example_source".to_string(), HashMap::new());
                let mut events_lock = events.lock().unwrap();
                events_lock.extend(new_events);
                thread::sleep(Duration::from_millis(50)); // Event generation interval
            }
        });

        let is_running_action = Arc::clone(&self.is_running);
        let events_action = Arc::clone(&self.events);
        let action_selector_action = Arc::clone(&self.action_selector);

        // Event processing thread
        let action_handle = thread::spawn(move || {
            while is_running_action.load(Ordering::SeqCst) {
                let mut events = events_action.lock().unwrap();
                let action = action_selector_action.select_action(&mut events);
                action.execute();
                thread::sleep(Duration::from_millis(50)); // Event processing interval
            }
        });

        // Wait for threads to finish
        generator_handle.join().unwrap();
        action_handle.join().unwrap();
    }

    /// Stops the agent.
    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
        println!("Agent {} stopped.", self.name);
    }

    /// Loads the configuration from the given path.
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the configuration file.
    ///
    /// # Returns
    ///
    /// * `CoreConfig` - The loaded configuration.
    fn load_config(config_path: &str) -> CoreConfig {
        let config_str = std::fs::read_to_string(config_path).unwrap();
        CoreConfig::from_toml_str(&config_str).unwrap()
    }
}
