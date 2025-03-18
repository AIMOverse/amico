/// Resources, such as wallets, can be used in actions.
pub struct Resource<T> {
    /// The name of the resource
    name: String,

    /// The value of the resource
    value: T,
}

impl<T> Resource<T> {
    /// Create a new resource
    /// Arguments:
    ///    * `name` - The name of the resource.
    ///    * `value` - The value of the resource.
    ///    Returns:
    ///    * `Resource` - The new resource instance.
    pub fn new(name: String, value: T) -> Self {
        Self { name, value }
    }

    /// Get the name of the resource
    /// Returns:
    ///    * `&str` - The name of the resource.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the value of the resource
    /// Returns:
    ///    * `&T` - The value of the resource.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Borrow the value of the resource then apply a function to it.
    /// Arguments:
    ///    * `f` - A function to apply to the value.
    ///    Returns:
    ///    * `R` - The result of the function.
    pub fn borrow_then<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(&self.value)
    }

    /// Borrow the value of the resource mutably then apply a function to it.
    /// Arguments:
    ///    * `f` - A function to apply to the value.
    ///    Returns:
    ///    * `R` - The result of the function.
    pub fn mut_borrow_then<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        f(&mut self.value)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread,
    };

    use super::*;

    #[test]
    fn test_resource() {
        let mut resource = Resource::new("test".to_string(), 1);
        assert_eq!(resource.name(), "test");
        assert_eq!(resource.borrow_then(|v| *v), 1);
        assert_eq!(
            resource.mut_borrow_then(|v| {
                *v = 2;
                *v
            }),
            2
        );
    }

    #[test]
    fn test_boxed_resource() {
        let mut resource = Resource::new("test".to_string(), Box::new(1));
        assert_eq!(resource.name(), "test");
        assert_eq!(resource.borrow_then(|v| **v), 1);
        assert_eq!(
            resource.mut_borrow_then(|v| {
                **v = 2;
                **v
            }),
            2
        );
    }

    #[test]
    fn test_multithreaded_resource() {
        let resource = Resource::new("test".to_string(), 1);
        let boxed = Arc::new(Mutex::new(resource));

        let mut handles = vec![];
        for _ in 0..100 {
            let boxed = Arc::clone(&boxed);
            handles.push(thread::spawn(move || {
                let mut resource = boxed.lock().unwrap();
                resource.mut_borrow_then(|v| {
                    *v += 1;
                });
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(*boxed.lock().unwrap().value(), 101);
    }
}
