use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::entity::Event;

use super::{error::PluginError, *};

// Event Source

#[derive(Debug, Deserialize, Serialize)]
struct TestEventSourceConfig {
    initial_state: i32,
}

impl PluginConfig for TestEventSourceConfig {
    fn toml_loader(&self) -> Option<fn(String) -> Self>
    where
        Self: Sized,
    {
        None
    }
}

struct TestEventSource {
    state: i32,
}

impl TestEventSource {
    fn state(&self) -> i32 {
        self.state
    }
}

impl Plugin<TestEventSourceConfig> for TestEventSource {
    fn name(&self) -> &str {
        "test_event_source"
    }

    fn setup(config: TestEventSourceConfig) -> Self
    where
        Self: Sized,
    {
        TestEventSource {
            state: config.initial_state,
        }
    }
}

impl EventSource<TestEventSourceConfig> for TestEventSource {
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
    let mut source = TestEventSource::setup(config);
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

impl PluginConfig for TestActuatorConfig {
    fn toml_loader(&self) -> Option<fn(String) -> Self>
    where
        Self: Sized,
    {
        None
    }
}

struct TestActuator {
    connected: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct TestActuatorError(String);

impl PluginError for TestActuatorError {
    fn message(&self) -> &str {
        self.0.as_str()
    }

    fn plugin_name(&self) -> &str {
        "TestActuator"
    }
}

impl TestActuator {
    fn new() -> Self {
        TestActuator { connected: false }
    }
    fn connect(&mut self, config: &str) {
        self.connected = config == "connected";
    }
}

impl Plugin<TestActuatorConfig> for TestActuator {
    fn name(&self) -> &str {
        "test_actuator"
    }

    fn setup(config: TestActuatorConfig) -> Self
    where
        Self: Sized,
    {
        let mut a = TestActuator::new();
        a.connect(config.connect_string.as_str());
        a
    }
}

impl Actuator<TestActuatorConfig, &str, (), TestActuatorError> for TestActuator {
    fn execute(&mut self, data: &str) -> Result<(), TestActuatorError> {
        if !self.connected {
            return Err(TestActuatorError("Not connected".to_string()));
        }

        println!("{}", data);

        Ok(())
    }
}

#[test]
fn test_actuator() {
    let disconnect_config = TestActuatorConfig {
        connect_string: "disconnected".to_string(),
    };
    let mut actuator = TestActuator::setup(disconnect_config);
    assert!(!actuator.connected);
    let result = actuator.execute("test_data");
    assert!(result.is_err());

    let connect_config = TestActuatorConfig {
        connect_string: "connected".to_string(),
    };
    let mut actuator = TestActuator::setup(connect_config);
    assert!(actuator.connected);
    let result = actuator.execute("test_data");
    assert!(result.is_ok());
}

// Plugin Pool

#[test]
fn test_plugin_pool() {}
