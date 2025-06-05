use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard};
use tokio_with_wasm::alias as tokio;

/// `Resource<T>` represents a globally available resource instance that can be shared among agents.
///
/// The actual resource instance is stored in an `Arc` (Atomic Reference Counted pointer),
/// allowing the resource to be safely shared across multiple owners and threads without
/// copying the underlying data. This design enables `Resource` types to be efficiently cloned
/// wherever needed while still referring to the same underlying resource instance.
///
/// ## Example
///
/// ```rust
/// use amico::resource::Resource;
///
/// fn resource_consumer_one(resource: Resource<i32>) {
///     // Resource consumers don't need to consume
///     // a resource with references.
///     assert_eq!(*resource.get(), 1);
/// }
///
/// fn resource_consumer_two(resource: Resource<i32>) {
///     assert_eq!(*resource.get(), 1);
/// }
///
/// fn main() {
///     let resource = Resource::new("one", 1);
///
///     // Just clone the resource wherever needed.
///     resource_consumer_one(resource.clone());
///
///     // Just clone the resource wherever needed.
///     resource_consumer_two(resource.clone());
///
///     // The resource is still available here.
///     assert_eq!(*resource.get(), 1);
/// }
/// ```
#[derive(Debug)]
pub struct Resource<T> {
    /// The name of the resource.
    name: &'static str,

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
    pub fn new(name: &'static str, value: T) -> Self {
        Self {
            name,
            value: Arc::new(value),
        }
    }

    /// Get the name of the resource
    ///
    /// Returns:
    ///    * `&str` - The name of the resource.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get the value of the resource
    ///
    /// Returns:
    ///    * `&T` - The value of the resource.
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Get a clone of the `Arc` pointer to the value of the resource
    ///
    /// Returns:
    ///    * `Arc<T>` - A clone of the value of the resource.
    pub fn get_ptr(&self) -> Arc<T> {
        Arc::clone(&self.value)
    }
}

impl<T> Clone for Resource<T> {
    /// Clone the resource
    ///
    /// Returns:
    ///    * `Resource` - A clone of the resource.
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            value: Arc::clone(&self.value),
        }
    }
}

/// `IntoResource<T>` is a trait that allows types to be converted into a `Resource<T>`.
/// This trait is useful for converting types into a `Resource<T>`.
///
/// # Example
///
/// ```rust
/// use amico::resource::{IntoResource, Resource};
///
/// struct MyResource {
///     value: i32,
/// }
///
/// impl IntoResource<MyResource> for MyResource {
///     fn into_resource(self) -> Resource<MyResource> {
///         Resource::new("my_resource", self)
///     }
/// }
/// ```
pub trait IntoResource<T> {
    /// Convert the type into a `Resource<T>`.
    fn into_resource(self) -> Resource<T>;
}

/// `ResourceMut<T>` represents a globally available mutable resource instance that can be shared among agents.
///
/// The value of a `ResourceMut<T>` is stored inside a tokio Mutex.
#[derive(Debug)]
pub struct ResourceMut<T> {
    name: &'static str,
    value: Arc<Mutex<T>>,
}

impl<T> ResourceMut<T> {
    /// Create a new resource
    pub fn new(name: &'static str, value: T) -> Self {
        Self {
            name,
            value: Arc::new(Mutex::new(value)),
        }
    }

    /// Get the name of the resource
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Lock the resource
    pub async fn lock(&self) -> MutexGuard<T> {
        self.value.lock().await
    }

    /// Blockingly lock the resource
    pub fn blocking_lock(&self) -> MutexGuard<T> {
        self.value.blocking_lock()
    }

    /// Get a clone of the `Arc` pointer to the value of the resource
    pub fn get_ptr(&self) -> Arc<Mutex<T>> {
        Arc::clone(&self.value)
    }
}

impl<T> Clone for ResourceMut<T> {
    /// Clone the resource.
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            value: Arc::clone(&self.value),
        }
    }
}

/// `IntoResourceMut<T>` is a trait that allows types to be converted into a `ResourceMut<T>`.
/// This trait is useful for converting types into a `ResourceMut<T>`.
pub trait IntoResourceMut<T> {
    fn into_resource_mut(self) -> ResourceMut<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource() {
        let resource = Resource::new("test", 1);
        assert_eq!(resource.name(), "test");
        assert_eq!(*resource.get(), 1);
    }

    #[test]
    fn test_resource_mut() {
        let resource = ResourceMut::new("test", 1);
        assert_eq!(resource.name(), "test");
        assert_eq!(*resource.blocking_lock(), 1);
    }

    #[test]
    fn test_resource_mut_modify() {
        let resource = ResourceMut::new("test", 1);
        {
            let mut value = resource.blocking_lock();
            *value += 1;
        }
        assert_eq!(*resource.blocking_lock(), 2);
    }

    #[tokio::test]
    async fn test_async_resource_mut() {
        let resource = ResourceMut::new("test", 1);
        {
            let mut value = resource.lock().await;
            *value += 1;
        }
        assert_eq!(*resource.lock().await, 2);
    }

    #[tokio::test]
    async fn test_multithreaded_resource_mut() {
        let resource = ResourceMut::new("test", 1);

        let mut handles = vec![];
        for _ in 0..100 {
            let resource_clone = resource.clone();
            handles.push(tokio::spawn(async move {
                let mut value = resource_clone.lock().await;
                *value += 1;
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(*resource.lock().await, 101);
    }

    #[test]
    fn test_resource_mut_clone() {
        let resource = ResourceMut::new("test", 1);
        let cloned = resource.clone();

        {
            let mut value = resource.blocking_lock();
            *value += 1;
        }

        assert_eq!(*cloned.blocking_lock(), 2);
        assert_eq!(*resource.blocking_lock(), 2);
    }
}
