use crate::config::{Config, CoreConfig};
use crate::entities::Event;
use crate::factory::{ActionSelectorFactory, EventGeneratorFactory};
use log::info;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::thread::JoinHandle;

pub struct Agent {
    name: String,
    is_running: Arc<AtomicBool>,
    events: Arc<Mutex<Vec<Event>>>,
    event_generator_factory: Arc<EventGeneratorFactory>,
    action_selector_factory: Arc<ActionSelectorFactory>,
    thread_handles: Mutex<Vec<JoinHandle<()>>>,
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
            event_generator_factory: Arc::new(event_generator_factory),
            action_selector_factory: Arc::new(action_selector_factory),
            thread_handles: Mutex::new(Vec::new()),
        }
    }

    pub fn before_start(&self) {
        env_logger::init();
        info!("Loading objects for agent: {}", self.name);
    }

    pub fn start(&self) {
        self.before_start();
        self.is_running.store(true, Ordering::SeqCst);
        info!("Agent {} started.", self.name);

        let is_running = Arc::clone(&self.is_running);
        let events = Arc::clone(&self.events);
        let event_generator_factory = Arc::clone(&self.event_generator_factory);

        let generator_handle = thread::spawn(move || {
            let mut counter = 0;
            // The factory is called to create an event generator
            let event_generator = event_generator_factory();
            while is_running.load(Ordering::SeqCst) {
                // The event generator is used to generate events
                let new_events =
                    event_generator.generate_event("example_source".to_string(), HashMap::new());
                // The new events are added to the events list
                let mut events_lock = events.lock().unwrap();
                events_lock.extend(new_events);
                counter += 1;
                info!("Generated {} events", counter);
            }
        });

        let is_running_action = Arc::clone(&self.is_running);
        let events_action = Arc::clone(&self.events);
        let action_selector_factory = Arc::clone(&self.action_selector_factory);

        let action_handle = thread::spawn(move || {
            // The factory is called to create an action selector
            let action_selector = action_selector_factory();
            let mut counter = 0;
            while is_running_action.load(Ordering::SeqCst) {
                // The action selector is used to select an action
                let mut events = events_action.lock().unwrap();
                if !events.is_empty() {
                    let action = action_selector.select_action(&mut events);
                    counter += 1;
                    info!("Processed {} events", counter);
                    action.execute();
                }
            }
        });

        // Store the thread handles for later use
        let mut handles = self.thread_handles.lock().unwrap();
        handles.push(generator_handle);
        handles.push(action_handle);
    }

    pub fn stop(&self) {
        info!("Agent {} stopping.", self.name);
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn join(&self) {
        let mut handles = self.thread_handles.lock().unwrap();
        for handle in handles.drain(..) {
            let _ = handle.join();
        }
        info!("All threads have finished.");
    }

    fn load_config(config_path: &str) -> CoreConfig {
        let config_str = std::fs::read_to_string(config_path).unwrap();
        CoreConfig::from_toml_str(&config_str).unwrap()
    }
}
