use std::collections::HashMap;

use crate::entity::Event;

use super::*;

// Event Source

struct TestEventSource {
    state: i32,
}

impl TestEventSource {
    fn state(&self) -> i32 {
        self.state
    }
}

impl Plugin<i32> for TestEventSource {
    fn name(&self) -> String {
        "TestEventSource".to_string()
    }

    fn setup(config: i32) -> Self
    where
        Self: Sized,
    {
        TestEventSource { state: config }
    }
}

impl EventSource<i32> for TestEventSource {
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
    let mut source = TestEventSource::setup(0);
    let event = source.generate_event();
    assert_eq!(event.name, "test_event");
    assert_eq!(event.source, "test_source");
    assert_eq!(event.params.len(), 0);
    assert_eq!(source.state(), 1);
}

// Actuator

struct TestActuator {
    connected: bool,
}

impl TestActuator {
    fn new() -> Self {
        TestActuator { connected: false }
    }
    fn connect(&mut self, config: &str) {
        self.connected = config == "connected";
    }
}

impl Plugin<&str> for TestActuator {
    fn name(&self) -> String {
        "TestActuator".to_string()
    }

    fn setup(config: &str) -> Self
    where
        Self: Sized,
    {
        let mut a = TestActuator::new();
        a.connect(config);
        a
    }
}

impl Actuator<&str, &str, (), ()> for TestActuator {
    fn execute(&mut self, data: &str) -> Result<(), ()> {
        if !self.connected {
            return Err(());
        }

        println!("{}", data);

        Ok(())
    }
}

#[test]
fn test_actuator() {
    let mut actuator = TestActuator::setup("disconnected");
    assert!(!actuator.connected);
    let result = actuator.execute("test_data");
    assert!(result.is_err());

    let mut actuator = TestActuator::setup("connected");
    assert!(actuator.connected);
    let result = actuator.execute("test_data");
    assert!(result.is_ok());
}

// Plugin Pool
