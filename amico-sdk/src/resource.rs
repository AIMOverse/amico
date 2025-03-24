use std::sync::Arc;

/// Resources, such as wallets, can be used in actions.
#[derive(Debug, Clone)]
pub struct Resource<T> {
    /// The name of the resource.
    name: String,

    /// The value of the resource. Stored in an `Arc`.
    value: Arc<T>,
}

impl<T> Resource<T> {
    /// Create a new resource
    ///
    /// Arguments:
    ///    * `name` - The name of the resource.
    ///    * `value` - The value of the resource.
    ///
    /// Returns:
    ///    * `Resource` - The new resource instance.
    pub fn new(name: String, value: T) -> Self {
        Self {
            name,
            value: Arc::new(value),
        }
    }

    /// Get the name of the resource
    ///
    /// Returns:
    ///    * `&str` - The name of the resource.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the value of the resource
    ///
    /// Returns:
    ///    * `&T` - The value of the resource.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get a clone of the `Arc` pointer to the value of the resource
    ///
    /// Returns:
    ///    * `Arc<T>` - A clone of the value of the resource.
    pub fn value_ptr(&self) -> Arc<T> {
        Arc::clone(&self.value)
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Mutex, thread};

    use super::*;

    #[test]
    fn test_resource() {
        let resource = Resource::new("test".to_string(), 1);
        assert_eq!(resource.name(), "test");
        assert_eq!(*resource.value(), 1);
    }

    #[test]
    fn test_boxed_resource() {
        let resource = Resource::new("test".to_string(), Box::new(1));
        assert_eq!(resource.name(), "test");
        assert_eq!(**resource.value(), 1);
    }

    #[test]
    fn test_multithreaded_resource() {
        let resource = Resource::new("test".to_string(), Mutex::new(1));

        let mut handles = vec![];
        for _ in 0..100 {
            let ptr = resource.value_ptr();
            handles.push(thread::spawn(move || {
                let mut value = ptr.lock().unwrap();
                *value += 1;
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(*resource.value().lock().unwrap(), 101);
    }
}
