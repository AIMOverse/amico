use crate::entities::{Event, EventPool};
use crate::errors::EventPoolError;
use chrono::Duration;

#[test]
fn test_event_pool() -> Result<(), EventPoolError> {
    let mut event_pool = EventPool::new(5);
    event_pool.extend_events(vec![
        Event::new(
            "ExampleEvent".to_string(),
            "ExampleSource".to_string(),
            Default::default(),
            None,
        ),
        Event::new(
            "ExampleEvent2".to_string(),
            "ExampleSource2".to_string(),
            Default::default(),
            Some(Duration::seconds(10)),
        ),
    ])?;
    assert_eq!(event_pool.get_events().len(), 2);

    // Wait for 6 seconds
    std::thread::sleep(std::time::Duration::from_secs(6));

    assert_eq!(event_pool.get_events().len(), 1);

    // remove the last event
    event_pool.remove_events(vec![1])?;

    Ok(())
}
