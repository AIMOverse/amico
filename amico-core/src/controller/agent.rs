use crate::entities::EventPool;
use crate::traits::{ActionSelector, EventGenerator};
use log::{error, info};
use serde_json::{Map, Value};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

/// The Core Agent program
/// Defines the workflow of the agent
pub struct Agent {
    name: String,                      // The name of the agent
    is_running: Arc<AtomicBool>,       // A flag to indicate if the agent is running
    event_pool: Arc<Mutex<EventPool>>, // The pool of events
    event_generator_factory: Arc<Box<dyn Fn() -> Box<dyn EventGenerator + Send> + Send + Sync>>, // The factory to create an event generator
    action_selector_factory: Arc<Box<dyn Fn() -> Box<dyn ActionSelector + Send> + Send + Sync>>, // The factory to create an action selector
    thread_handles: Mutex<Vec<JoinHandle<()>>>, // The handles of the threads
}

impl Agent {
    /// Create a new agent
    /// Arguments:
    ///    * `config_path` - The path to the configuration file.
    ///   * `event_generator_factory` - The factory to create an event generator.
    ///  * `action_selector_factory` - The factory to create an action selector.
    ///    Returns:
    ///   * `Agent` - The new agent instance.
    pub fn new(
        event_generator_factory: Box<dyn Fn() -> Box<dyn EventGenerator + Send> + Send + Sync>,
        action_selector_factory: Box<dyn Fn() -> Box<dyn ActionSelector + Send> + Send + Sync>,
    ) -> Self {
        Self {
            name: "Amico".to_string(),
            is_running: Arc::new(AtomicBool::new(false)),
            event_pool: Arc::new(Mutex::new(EventPool::new(
                Duration::from_secs(10).as_secs() as i64,
            ))),
            event_generator_factory: Arc::new(event_generator_factory),
            action_selector_factory: Arc::new(action_selector_factory),
            thread_handles: Mutex::new(Vec::new()),
        }
    }

    /// The function to be called before starting the agent
    /// This function initializes the logger
    /// and logs the objects that are loaded for the agent.
    /// This function is called by the start function.
    pub fn before_start(&self) {
        env_logger::init();
        info!("Loading objects for agent: {}", self.name);
    }

    /// The function to start the agent
    /// This function starts the agent by creating threads for the event generator and action selector.
    /// This function is called by the start function.
    /// The threads are stored in the thread_handles list.
    pub fn start(&self) {
        // Call the before_start function
        self.before_start();
        // Set the flag to indicate that the agent is running
        self.is_running.store(true, Ordering::SeqCst);
        info!("Agent {} started.", self.name);

        // Clone the variables to be used in the Event Generator threads
        let is_running = Arc::clone(&self.is_running);
        let event_pool_for_eg = Arc::clone(&self.event_pool);
        let event_generator_factory = Arc::clone(&self.event_generator_factory);

        let generator_handle = thread::spawn(move || {
            // The factory is called to create an event generator
            let event_generator = event_generator_factory();

            while is_running.load(Ordering::SeqCst) {
                // The event generator is used to generate events
                let new_events = event_generator
                    .generate_event("example_source".to_string(), Value::Object(Map::new()));
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

        // Clone the variables to be used in the Action Selector threads
        let is_running_action = Arc::clone(&self.is_running);
        let event_pool_for_as = Arc::clone(&self.event_pool);
        let action_selector_factory = Arc::clone(&self.action_selector_factory);

        let action_handle = thread::spawn(move || {
            // The factory is called to create an action selector
            let mut action_selector = action_selector_factory();

            while is_running_action.load(Ordering::SeqCst) {
                // The action selector is used to select an action
                let events;
                {
                    // The event pool is locked and checked for events
                    let mut event_pool_for_as = event_pool_for_as.lock().unwrap();
                    events = event_pool_for_as.get_events();
                    // The event pool list is unlocked
                }
                match action_selector.select_action(events) {
                    Ok((action, event_ids)) => {
                        info!("Removing {} events", event_ids.len());
                        {
                            // Lock the event pool
                            let mut event_pool_for_as = event_pool_for_as.lock().unwrap();

                            // Try to remove events from the pool
                            if let Err(e) = event_pool_for_as.remove_events(event_ids) {
                                error!("Failed to remove events: {}", e);
                            }
                        }
                        // The event pool is unlocked here automatically when the lock goes out of scope
                        // The action is executed
                        if let Err(e) = action.execute() {
                            error!("{}", e);
                        }
                    }
                    Err(e) => {
                        error!("{}", e);
                    }
                }
            }
        });

        // Store the thread handles for later use
        let mut handles = self.thread_handles.lock().unwrap();
        handles.push(generator_handle);
        handles.push(action_handle);
    }

    /// The function to stop the agent
    pub fn stop(&self) {
        info!("Agent {} stopping.", self.name);
        self.is_running.store(false, Ordering::SeqCst);
    }

    /// Wait for all threads to finish
    pub fn join(&self) {
        let mut handles = self.thread_handles.lock().unwrap();
        for handle in handles.drain(..) {
            let _ = handle.join();
        }
        info!("All threads have finished.");
    }
}
