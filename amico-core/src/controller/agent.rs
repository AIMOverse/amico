use crate::config::{Config, CoreConfig};
use crate::entities::EventPool;
use crate::traits::{ActionSelector, EventGenerator};
use log::{error, info};
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
    event_pool: Arc<Mutex<EventPool>>,
    event_generator_factory: Arc<Box<dyn Fn() -> Box<dyn EventGenerator + Send> + Send + Sync>>,
    action_selector_factory: Arc<Box<dyn Fn() -> Box<dyn ActionSelector + Send> + Send + Sync>>,
    thread_handles: Mutex<Vec<JoinHandle<()>>>,
}

impl Agent {
    pub fn new(
        config_path: &str,
        event_generator_factory: Box<dyn Fn() -> Box<dyn EventGenerator + Send> + Send + Sync>,
        action_selector_factory: Box<dyn Fn() -> Box<dyn ActionSelector + Send> + Send + Sync>,
    ) -> Self {
        let config = Self::load_config(config_path);
        Self {
            name: config.name,
            is_running: Arc::new(AtomicBool::new(false)),
            event_pool: Arc::new(Mutex::new(EventPool::new())),
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
        let event_pool_for_eg = Arc::clone(&self.event_pool);
        let event_generator_factory = Arc::clone(&self.event_generator_factory);

        let generator_handle = thread::spawn(move || {
            // The factory is called to create an event generator
            let event_generator = event_generator_factory();

            while is_running.load(Ordering::SeqCst) {
                // The event generator is used to generate events
                let new_events =
                    event_generator.generate_event("example_source".to_string(), HashMap::new());
                // The new events are added to the events list
                {
                    info!("Extending {} events", new_events.len());
                    // The events pool is locked
                    let mut unlocked_event_pool = event_pool_for_eg.lock().unwrap();
                    if let Err(e) = unlocked_event_pool.extend_events(new_events) {
                        error!("Failed to extend events: {}", e);
                    }
                    // The events pool is unlocked
                }
            }
        });

        let is_running_action = Arc::clone(&self.is_running);
        let event_pool_for_as = Arc::clone(&self.event_pool);
        let action_selector_factory = Arc::clone(&self.action_selector_factory);

        let action_handle = thread::spawn(move || {
            // The factory is called to create an action selector
            let action_selector = action_selector_factory();

            while is_running_action.load(Ordering::SeqCst) {
                // The action selector is used to select an action
                let events;
                {
                    // The event pool is locked and checked for events
                    let event_pool_for_as = event_pool_for_as.lock().unwrap();
                    events = event_pool_for_as.get_events();
                    // The event pool list is unlocked
                }
                let (action, event_ids) = action_selector.select_action(events);
                {
                    info!("Removing {} events", event_ids.len());
                    // The events pool is locked
                    let mut event_pool_for_as = event_pool_for_as.lock().unwrap();
                    if let Err(e) = event_pool_for_as.remove_events(event_ids) {
                        error!("Failed to remove events: {}", e);
                    }
                    // The events pool is unlocked
                }
                // The action is executed
                action.execute();
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
