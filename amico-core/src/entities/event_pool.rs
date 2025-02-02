use crate::entities::Event;
use crate::errors::EventPoolError;
use chrono::{Duration, Utc};
use std::collections::HashMap;

/// Struct representing an event pool in the system.
#[derive(Debug)]
pub struct EventPool {
    /// Key is the Event ID, value is the Event itself.
    events_map: HashMap<u32, Event>,

    /// The next ID candidate if `free_list` is empty.
    next_id: u32,

    /// A list (stack) of IDs that were removed and can be reused.
    free_list: Vec<u32>,

    /// The default expiry time for events in seconds
    default_expiry_time: i64,
}

impl EventPool {
    /// Creates a new EventPool.
    pub fn new(default_expiry_time: i64) -> Self {
        Self {
            events_map: HashMap::new(),
            next_id: 0,
            free_list: Vec::new(),
            default_expiry_time,
        }
    }

    /// Retrieves all events as a cloned vector.
    /// (Depending on requirements, returning references or an iterator might be preferable.)
    pub fn get_events(&self) -> Vec<Event> {
        self.events_map.values().cloned().collect()
    }

    /// Inserts multiple new events.
    /// Each new Event will be assigned a unique ID from this pool.
    pub fn extend_events(&mut self, events: Vec<Event>) -> Result<(), EventPoolError> {
        for mut event in events {
            // Propagate the possible error from get_new_event_id using the `?` operator.
            let id = self.get_new_event_id()?;
            event.id = id;
            if event.expiry_time.is_none() {
                event.expiry_time = Some(Utc::now() + Duration::seconds(self.default_expiry_time));
            }
            self.events_map.insert(id, event);
        }

        Ok(())
    }

    /// Removes events corresponding to the given list of event IDs.
    /// Freed IDs are pushed back into `free_list`.
    pub fn remove_events(&mut self, event_ids: Vec<u32>) -> Result<(), EventPoolError> {
        for id in event_ids {
            if self.events_map.remove(&id).is_some() {
                // Only push back if the ID actually existed in the map
                // so we don't pollute `free_list` with unused IDs.
                self.free_list.push(id);
            } else {
                return Err(EventPoolError::EventIdNotFound(id));
            }
        }
        Ok(())
    }

    /// Generates a new unique event ID in O(1) time, reusing old IDs if available.
    fn get_new_event_id(&mut self) -> Result<u32, EventPoolError> {
        // If we have some freed IDs, reuse one.
        if let Some(reused_id) = self.free_list.pop() {
            return Ok(reused_id);
        }

        // Otherwise, use the next_id.
        let id = self.next_id;

        // If next_id is at max, check if we can still proceed.
        // If we've reached u32::MAX and there's no free ID left in free_list (already checked above),
        // then we've exhausted all possible IDs.
        if id == u32::MAX {
            // Choose what to do here: panic, return an error, etc.
            // For demonstration, we'll panic:
            return Err(EventPoolError::NoAvailableEventIds);
        }

        // Increment next_id for the next allocation.
        self.next_id = self.next_id.wrapping_add(1);

        Ok(id)
    }
}
