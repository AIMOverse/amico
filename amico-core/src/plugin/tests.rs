use std::{any::Any, collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{
    entities::Event, errors::ActionError, traits::{Action, EventGenerator}
};

use super::*;

// Event Source

#[derive(Debug, Deserialize, Serialize)]
struct TestEventSourceConfig {
    initial_state: i32,
}

struct TestEventSource {
    state: i32,
}

impl TestEventSource {
    fn state(&self) -> i32 {
        self.state
    }
}

impl Plugin for TestEventSource {
    fn name(&self) -> String {
        "test_event_source".to_string()
    }

    fn setup(config: &dyn Any) -> Result<Self, PluginError>
    where
        Self: Sized,
    {
        if let Some(config) = config.downcast_ref::<TestEventSourceConfig>() {
            Ok(TestEventSource {
                state: config.initial_state,
            })
        } else {
            Err(PluginError::InvalidConfigFormat)
        }
    }
}

impl EventGenerator for TestEventSource {
    fn generate_event(
        &self,
        source: String,
        params: HashMap<String, Arc<dyn Any + Send + Sync>>,
    ) -> Vec<Event> {
        vec![Event::new("ExampleEvent".to_string(), source, params, None)]
    }
}

impl EventGeneratorPlugin for TestEventSource {}

#[test]
fn test_event_source() {
    let config = TestEventSourceConfig { initial_state: 0 };
    let source = TestEventSource::setup(&config).unwrap();
    let events = source.generate_event("test source".to_string(), HashMap::new());
    assert_eq!(events.len(), 1);
    let event = &events[0];
    assert_eq!(event.name, "ExampleEvent");
    assert_eq!(event.source, "test source");
    assert_eq!(event.params.len(), 0);
    assert_eq!(source.state(), 0);
}

// Actuator

#[derive(Debug, Deserialize, Serialize)]
struct TestActuatorConfig {
    connect_string: String,
}

struct TestActuator {
    connected: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct TestActuatorError(String);

impl TestActuator {
    fn new() -> Self {
        TestActuator { connected: false }
    }
    fn connect(&mut self, config: &str) {
        self.connected = config == "connected";
    }
}

impl Plugin for TestActuator {
    fn name(&self) -> String {
        "test_actuator".to_string()
    }

    fn setup(config: &dyn Any) -> Result<Self, PluginError>
    where
        Self: Sized,
    {
        if let Some(config) = config.downcast_ref::<TestActuatorConfig>() {
            let mut a = TestActuator::new();
            a.connect(config.connect_string.as_str());
            Ok(a)
        } else {
            Err(PluginError::InvalidConfigFormat)
        }
    }
}

impl Action for TestActuator {
    fn execute(&self) -> Result<(), ActionError> {
        if !self.connected {
            return Err(ActionError::ExecutingActionError("Not connected".to_string()));
        }
        Ok(())
    }
}

impl ActionPlugin for TestActuator {}

#[test]
fn test_actuator() {
    let disconnect_config = TestActuatorConfig {
        connect_string: "disconnected".to_string(),
    };
    let actuator = TestActuator::setup(&disconnect_config).unwrap();
    assert!(!actuator.connected);
    let result = actuator.execute();
    assert!(result.is_err());
    let connect_config = TestActuatorConfig {
        connect_string: "connected".to_string(),
    };
    let actuator = TestActuator::setup(&connect_config).unwrap();
    assert!(actuator.connected);
    let result = actuator.execute();
    assert!(result.is_ok());
    let result = actuator.execute();
    assert!(result.is_ok());
}

// Plugin Pool

#[test]
fn test_plugin_pool() {
    let event_source_config = TestEventSourceConfig { initial_state: 0 };
    let event_source_config_1 = TestEventSourceConfig { initial_state: 1 };
    let actuator_config = TestActuatorConfig {
        connect_string: "connected".to_string(),
    };

    let mut pool = PluginPool::new();
    pool.add_event_generator(
        "event_source_0".to_string(),
        Arc::new(TestEventSource::setup(&event_source_config).unwrap()),
    );
    pool.add_action(
        "actuator_0".to_string(),
        Arc::new(TestActuator::setup(&actuator_config).unwrap()),
    );
    pool.add_event_generator(
        "event_source_1".to_string(),
        Arc::new(TestEventSource::setup(&event_source_config_1).unwrap()),
    );

    assert!(pool.event_generators.len() == 2);
    assert!(pool.actions.len() == 1);
    assert!(pool.input_sources.len() == 0);
    assert!(pool.action_selectors.len() == 0);
}
