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

    /// Updates the events map and retrieves all events that hasn't expired as a cloned vector and .
    /// (Depending on requirements, returning references or an iterator might be preferable.)
    pub fn get_events(&mut self) -> Vec<Event> {
        let now = Utc::now();

        // Use drain_filter() to remove expired events and recycle IDs
        let mut expired_ids = Vec::new();
        self.events_map.retain(|id, event| {
            if let Some(expiry_time) = event.expiry_time {
                if expiry_time < now {
                    expired_ids.push(*id);
                    return false;
                }
            }
            true
        });
        self.free_list.extend(expired_ids);

        // return all non-expired events
        self.events_map.values().cloned().collect()
    }

    /// Inserts multiple new events into the event pool.
    /// Each new Event will be assigned a unique ID from this pool.
    ///
    /// # Arguments
    ///
    /// * `events` - A vector of events to be inserted.
    ///
    /// # Returns
    ///
    /// * `Result<(), EventPoolError>` - Returns `Ok(())` if all events are successfully inserted,
    ///   otherwise returns an `EventPoolError`.
    pub fn extend_events(&mut self, events: Vec<Event>) -> Result<(), EventPoolError> {
        let now = Utc::now();
        let default_expiry = now + Duration::seconds(self.default_expiry_time);

        // Optimization: Pre-allocate capacity for the HashMap to reduce reallocations
        self.events_map.reserve(events.len());

        for mut event in events {
            let id = self.get_new_event_id()?;
            event.id = id;
            if event.expiry_time.is_none() {
                event.expiry_time = Some(default_expiry);
            }
            self.events_map.insert(id, event);
        }

        Ok(())
    }

    /// Removes events corresponding to the given list of event IDs.
    /// Freed IDs are pushed back into `free_list`.
    pub fn remove_events(&mut self, event_ids: Vec<u32>) -> Result<(), EventPoolError> {
        let mut not_found_ids = Vec::new();

        for id in event_ids {
            if self.events_map.remove(&id).is_some() {
                self.free_list.push(id);
            } else {
                not_found_ids.push(id);
            }
        }

        if !not_found_ids.is_empty() {
            return Err(EventPoolError::SomeEventIdsNotFound(not_found_ids));
        }

        Ok(())
    }

    /// Generates a new unique event ID in O(1) time, reusing old IDs if available.
    fn get_new_event_id(&mut self) -> Result<u32, EventPoolError> {
        // If we have some freed IDs, reuse one.
        if let Some(reused_id) = self.free_list.pop() {
            return Ok(reused_id);
        }

        // Ensure we don't overflow the ID space.
        if self.next_id == u32::MAX {
            return Err(EventPoolError::NoAvailableEventIds);
        }

        // Otherwise, use the next_id.
        let id = self.next_id;
        // Increment next_id for the next allocation.
        self.next_id += 1;
        Ok(id)
    }
}
