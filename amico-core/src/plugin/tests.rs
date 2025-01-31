use std::{any::Any, collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::entity::Event;

use super::{error::PluginError, *};

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

impl EventSource for TestEventSource {
    fn generate_event(&mut self) -> crate::entity::Event {
        self.state += 1;

        Event {
            name: "test_event".to_string(),
            source: "test_source".to_string(),
            params: HashMap::new(),
        }
    }
}

#[test]
fn test_event_source() {
    let config = TestEventSourceConfig { initial_state: 0 };
    let mut source = TestEventSource::setup(&config).unwrap();
    let event = source.generate_event();
    assert_eq!(event.name, "test_event");
    assert_eq!(event.source, "test_source");
    assert_eq!(event.params.len(), 0);
    assert_eq!(source.state(), 1);
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

impl Actuator for TestActuator {
    fn execute(&mut self, data: &dyn Any) -> Result<Box<dyn Any>, PluginError> {
        if !self.connected {
            return Err(PluginError::ExecutionError("Not connected".to_string()));
        }

        if let Some(data) = data.downcast_ref::<String>() {
            if data == "ping" {
                return Ok(Box::new("ok".to_string()));
            }
        } else if let Some(data) = data.downcast_ref::<&str>() {
            if data.to_string() == "ping".to_string() {
                return Ok(Box::new("ok".to_string()));
            }
        }

        Err(PluginError::InvalidDataFormat)
    }
}

#[test]
fn test_actuator() {
    let disconnect_config = TestActuatorConfig {
        connect_string: "disconnected".to_string(),
    };
    let mut actuator = TestActuator::setup(&disconnect_config).unwrap();
    assert!(!actuator.connected);
    let result = actuator.execute(&"ping".to_string());
    assert!(result.is_err());

    let connect_config = TestActuatorConfig {
        connect_string: "connected".to_string(),
    };
    let mut actuator = TestActuator::setup(&connect_config).unwrap();
    assert!(actuator.connected);
    let result = actuator.execute(&"ping".to_string());
    assert!(result.is_ok());
    let result = actuator.execute(&"ping");
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
    pool.add_event_source(Arc::new(
        TestEventSource::setup(&event_source_config).unwrap(),
    ));
    pool.add_actuator(Arc::new(TestActuator::setup(&actuator_config).unwrap()));
    pool.add_event_source(Arc::new(
        TestEventSource::setup(&event_source_config_1).unwrap(),
    ));

    assert!(pool.event_sources.len() == 2);
    assert!(pool.actuators.len() == 1);
    assert!(pool.inputs.len() == 0);
    assert!(pool.action_selectors.len() == 0);
}
