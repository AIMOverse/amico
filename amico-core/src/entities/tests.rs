use crate::entities::{Event, EventPool};
use crate::errors::EventPoolError;

#[test]
fn test_event_pool() -> Result<(), EventPoolError> {
    let mut event_pool = EventPool::new(5);
    event_pool.extend_events(vec![Event::new(
        "ExampleEvent".to_string(),
        "ExampleSource".to_string(),
        Default::default(),
        None,
    )])?;
    assert_eq!(event_pool.get_events().len(), 1);

    // Wait for 6 seconds
    std::thread::sleep(std::time::Duration::from_secs(6));

    assert_eq!(event_pool.get_events().len(), 0);

    Ok(())
}
