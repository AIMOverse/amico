use crate::config::{Config, CoreConfig};
use crate::entities::Event;
use crate::factory::{ActionSelectorFactory, EventGeneratorFactory};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;

pub struct Agent {
    name: String,
    is_running: Arc<AtomicBool>,
    events: Arc<Mutex<Vec<Event>>>,
    event_generator_factory: EventGeneratorFactory,
    action_selector_factory: ActionSelectorFactory,
}

impl Agent {
    pub fn new(
        config_path: &str,
        event_generator_factory: EventGeneratorFactory,
        action_selector_factory: ActionSelectorFactory,
    ) -> Self {
        let config = Self::load_config(config_path);
        Self {
            name: config.name,
            is_running: Arc::new(AtomicBool::new(false)),
            events: Arc::new(Mutex::new(Vec::new())),
            event_generator_factory,
            action_selector_factory,
        }
    }

    pub fn before_start(&self) {
        println!("Loading objects for agent: {}", self.name);
    }

    pub fn start(self) {
        self.before_start();
        self.is_running.store(true, Ordering::SeqCst);
        println!("Agent {} started.", self.name);

        let is_running = Arc::clone(&self.is_running);
        let events = Arc::clone(&self.events);

        let generator_handle = thread::spawn(move || {
            // The factory is called to create an event generator
            let event_generator = (self.event_generator_factory)();
            while is_running.load(Ordering::SeqCst) {
                // The event generator is used to generate events
                let new_events =
                    event_generator.generate_event("example_source".to_string(), HashMap::new());
                // The new events are added to the events list
                let mut events_lock = events.lock().unwrap();
                events_lock.extend(new_events);
            }
        });

        let is_running_action = Arc::clone(&self.is_running);
        let events_action = Arc::clone(&self.events);

        let action_handle = thread::spawn(move || {
            // The factory is called to create an action selector
            let action_selector = (self.action_selector_factory)();
            while is_running_action.load(Ordering::SeqCst) {
                // The action selector is used to select an action
                let mut events = events_action.lock().unwrap();
                if !events.is_empty() {
                    let action = action_selector.select_action(&mut events);
                    action.execute();
                }
            }
        });

        generator_handle.join().unwrap();
        action_handle.join().unwrap();
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
        println!("Agent {} stopped.", self.name);
    }

    fn load_config(config_path: &str) -> CoreConfig {
        let config_str = std::fs::read_to_string(config_path).unwrap();
        CoreConfig::from_toml_str(&config_str).unwrap()
    }
}
