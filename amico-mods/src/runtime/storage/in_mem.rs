use std::collections::HashMap;

use amico_core::runtime::storage::{Namespace, RawData, Storage, StorageError};

/// An in-memory storage implementation where each namespace is a collection of key-value pairs
/// held in memory using a HashMap.
#[derive(Default)]
pub struct InMemStorage {
    /// Currently opened namespaces
    namespaces: HashMap<String, InMemNamespace>,
}

/// A namespace implementation backed by an in-memory HashMap
pub struct InMemNamespace {
    /// In-memory storage of the key-value pairs
    data: HashMap<String, RawData>,
}

impl InMemStorage {
    /// Create a new in-memory storage
    pub fn new() -> Self {
        Self {
            namespaces: HashMap::new(),
        }
    }
}

impl InMemNamespace {
    /// Create a new namespace
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Storage<InMemNamespace> for InMemStorage {
    fn exist(&self, namespace: &str) -> bool {
        self.namespaces.contains_key(namespace)
    }

    fn create(&mut self, namespace: &str) -> Result<String, StorageError> {
        // If the namespace already exists, just return its name
        if self.namespaces.contains_key(namespace) {
            return Ok(namespace.to_string());
        }

        // Create a new empty namespace
        self.namespaces
            .insert(namespace.to_string(), InMemNamespace::new());
        Ok(namespace.to_string())
    }

    fn remove(&mut self, namespace: &str) -> Result<(), StorageError> {
        // Remove the namespace if it exists
        self.namespaces.remove(namespace);
        Ok(())
    }

    fn list(&self, namespace: &str) -> Result<Vec<String>, StorageError> {
        if namespace.is_empty() {
            // List all namespaces at the root level
            let namespaces = self.namespaces.keys().cloned().collect();
            Ok(namespaces)
        } else {
            // This implementation does not support nested namespaces
            Err(StorageError::NoNamespace(namespace.to_string()))
        }
    }

    fn open(&mut self, namespace: &str) -> Result<&mut InMemNamespace, StorageError> {
        // Check if the namespace exists
        if !self.namespaces.contains_key(namespace) {
            return Err(StorageError::NoNamespace(namespace.to_string()));
        }

        // Return a mutable reference to the namespace
        self.namespaces
            .get_mut(namespace)
            .ok_or_else(|| StorageError::NoNamespace(namespace.to_string()))
    }
}

impl Namespace for InMemNamespace {
    fn exist(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    fn get(&self, key: &str) -> Result<Option<RawData>, StorageError> {
        Ok(self.data.get(key).cloned())
    }

    fn put(&mut self, key: &str, value: RawData) -> Result<(), StorageError> {
        self.data.insert(key.to_string(), value);
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<(), StorageError> {
        self.data.remove(key);
        Ok(())
    }

    fn keys(&self) -> Result<Vec<String>, StorageError> {
        Ok(self.data.keys().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_mem_storage() {
        // Create a new in-memory storage
        let mut storage = InMemStorage::new();

        // Test namespace creation
        let ns_name = "test_namespace";
        let result = storage.create(ns_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ns_name);

        // Test namespace existence
        assert!(storage.exist(ns_name));

        // Test opening a namespace
        let namespace = storage.open(ns_name);
        assert!(namespace.is_ok());

        // Test namespace operations
        let namespace = namespace.unwrap();

        // Put data
        let key = "test_key";
        let value = "Test value".to_string();
        let result = namespace.put(key, value.clone().into());
        assert!(result.is_ok());

        // Check key existence
        assert!(namespace.exist(key));

        // Get data
        let result = namespace.get(key);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.is_some());
        assert_eq!(data.unwrap(), value.into());

        // List keys
        let result = namespace.keys();
        assert!(result.is_ok());
        let keys = result.unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], key);

        // Delete key
        let result = namespace.delete(key);
        assert!(result.is_ok());
        assert!(!namespace.exist(key));

        // Test namespace listing
        let result = storage.list("");
        assert!(result.is_ok());
        let namespaces = result.unwrap();
        assert_eq!(namespaces.len(), 1);
        assert_eq!(namespaces[0], ns_name);

        // Test namespace removal
        let result = storage.remove(ns_name);
        assert!(result.is_ok());
        assert!(!storage.exist(ns_name));
    }
}
