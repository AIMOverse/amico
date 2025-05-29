/// Error type for all storage operations.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Namespace not found: {0}")]
    NoNamespace(String),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Failed to convert from RawData: {0}")]
    FromBytes(String),
    // future: add backend-specific variants
}

pub trait Namespace {
    /// Check if a `key` exists
    fn exist(&self, key: &str) -> bool;

    /// Read raw bytes for `key` in `namespace`. Returns None if missing.
    fn get(&self, key: &str) -> Result<Option<RawData>, StorageError>;

    /// Write `value` bytes for `key` in `namespace`, overwriting if exists.
    fn put(&mut self, key: &str, value: RawData) -> Result<(), StorageError>;

    /// Delete `key` in `namespace`. No-op if missing.
    fn delete(&mut self, key: &str) -> Result<(), StorageError>;

    /// List all existing keys in `namespace`.
    fn keys(&self) -> Result<Vec<String>, StorageError>;
}

/// Abstract key/value storage where `namespace` groups tables or
/// directories, and each key maps to raw data.
pub trait Storage<N: Namespace> {
    /// Check if a `namespace` exists
    fn exist(&self, namespace: &str) -> bool;

    /// Create a `namespace`
    fn create(&mut self, namespace: &str) -> Result<String, StorageError>;

    /// Remove a `namespace`
    fn remove(&mut self, namespace: &str) -> Result<(), StorageError>;

    /// List all existing namespaces in `namespace`.
    fn list(&self, namespace: &str) -> Result<Vec<String>, StorageError>;

    /// Open a `namespace` for access
    fn open(&mut self, namespace: &str) -> Result<&mut N, StorageError>;

    // Automatically implemented methods

    /// Create and open a namespace if not exist
    fn open_or_create(&mut self, namespace: &str) -> Result<&mut N, StorageError> {
        if !self.exist(namespace) {
            self.create(namespace)?;
        }

        self.open(namespace)
    }
}

/// The data stored in the Storage in raw bytes
#[derive(Clone, Debug, PartialEq)]
pub struct RawData(Vec<u8>);

impl<T: Into<Vec<u8>>> From<T> for RawData {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl RawData {
    /// Convert the raw data to a String using utf-8 encoding
    pub fn to_string(self) -> Result<String, StorageError> {
        let Self(raw) = self;
        String::from_utf8(raw).map_err(|err| {
            StorageError::FromBytes(format!("Converting raw data to UTF-8 String: {}", err))
        })
    }

    /// Return the inner bytes of the raw data
    pub fn to_bytes(self) -> Vec<u8> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    /// Define a simple namespace
    struct TestNamespace {
        map: HashMap<String, Vec<u8>>,
    }

    impl TestNamespace {
        fn new() -> Self {
            Self {
                map: HashMap::new(),
            }
        }
    }

    impl Namespace for TestNamespace {
        fn exist(&self, key: &str) -> bool {
            self.map.contains_key(key)
        }

        fn get(&self, key: &str) -> Result<Option<RawData>, StorageError> {
            Ok(self.map.get(key).map(|data| RawData(data.clone())))
        }

        fn put(&mut self, key: &str, value: RawData) -> Result<(), StorageError> {
            let RawData(bytes) = value;
            self.map.insert(key.to_string(), bytes);
            Ok(())
        }

        fn delete(&mut self, key: &str) -> Result<(), StorageError> {
            self.map.remove(key);
            Ok(())
        }

        fn keys(&self) -> Result<Vec<String>, StorageError> {
            Ok(self.map.keys().cloned().collect())
        }
    }

    /// Define a simple storage
    struct TestStorage {
        files: HashMap<String, TestNamespace>,
    }

    impl TestStorage {
        fn new() -> Self {
            Self {
                files: HashMap::new(),
            }
        }
    }

    impl Storage<TestNamespace> for TestStorage {
        fn exist(&self, namespace: &str) -> bool {
            self.files.contains_key(namespace)
        }

        fn create(&mut self, namespace: &str) -> Result<String, StorageError> {
            if self.exist(namespace) {
                return Ok(namespace.to_string());
            }

            self.files
                .insert(namespace.to_string(), TestNamespace::new());
            Ok(namespace.to_string())
        }

        fn remove(&mut self, namespace: &str) -> Result<(), StorageError> {
            self.files.remove(namespace);
            Ok(())
        }

        fn list(&self, namespace: &str) -> Result<Vec<String>, StorageError> {
            if namespace.is_empty() {
                // Return all namespaces at the root level
                Ok(self.files.keys().cloned().collect())
            } else {
                // This simple implementation doesn't support nested namespaces
                Err(StorageError::NoNamespace(namespace.to_string()))
            }
        }

        fn open(&mut self, namespace: &str) -> Result<&mut TestNamespace, StorageError> {
            if let Some(ns) = self.files.get_mut(namespace) {
                Ok(ns)
            } else {
                Err(StorageError::NoNamespace(namespace.to_string()))
            }
        }
    }

    #[test]
    fn test_namespace_operations() {
        let mut ns = TestNamespace::new();

        // Test exist
        assert!(!ns.exist("key1"));

        // Test put
        let data = RawData(b"hello world".to_vec());
        ns.put("key1", data).unwrap();

        // Test exist after put
        assert!(ns.exist("key1"));

        // Test get
        let retrieved = ns.get("key1").unwrap().unwrap();
        let retrieved_str = retrieved.to_string().unwrap();
        assert_eq!(retrieved_str, "hello world");

        // Test keys
        let keys = ns.keys().unwrap();
        assert_eq!(keys.len(), 1);
        assert!(keys.contains(&"key1".to_string()));

        // Test delete
        ns.delete("key1").unwrap();
        assert!(!ns.exist("key1"));
        assert_eq!(ns.keys().unwrap().len(), 0);
    }

    #[test]
    fn test_rawdata_string_conversion() {
        // Test string to RawData conversion
        let original_str = "Hello, 你好, こんにちは!"; // Include multi-byte characters
        let raw_data: RawData = original_str.as_bytes().to_vec().into();

        // Test RawData to string conversion
        let converted_str = raw_data.to_string().unwrap();
        assert_eq!(converted_str, original_str);

        // Test invalid UTF-8 handling
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD]; // Invalid UTF-8 sequence
        let raw_data = RawData(invalid_utf8);
        assert!(raw_data.to_string().is_err());
    }

    #[test]
    fn test_storage_operations() {
        let mut storage = TestStorage::new();

        // Test exist
        assert!(!storage.exist("ns1"));

        // Test create
        storage.create("ns1").unwrap();
        assert!(storage.exist("ns1"));

        // Test list
        let namespaces = storage.list("").unwrap();
        assert_eq!(namespaces.len(), 1);
        assert!(namespaces.contains(&"ns1".to_string()));

        // Test open and namespace operations
        {
            let ns = storage.open("ns1").unwrap();
            assert!(!ns.exist("key1"));

            let data = RawData(b"test data".to_vec());
            ns.put("key1", data).unwrap();
            assert!(ns.exist("key1"));
        }

        // Test namespace persistence after reopening
        {
            let ns = storage.open("ns1").unwrap();
            assert!(ns.exist("key1"));

            let retrieved = ns.get("key1").unwrap().unwrap();
            let retrieved_str = retrieved.to_string().unwrap();
            assert_eq!(retrieved_str, "test data");
        }

        // Test open non-existent namespace
        assert!(storage.open("non_existent").is_err());

        // Test remove
        storage.remove("ns1").unwrap();
        assert!(!storage.exist("ns1"));
        assert_eq!(storage.list("").unwrap().len(), 0);
    }
}
