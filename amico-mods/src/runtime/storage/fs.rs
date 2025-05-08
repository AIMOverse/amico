use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use amico_core::runtime::storage::{Namespace, RawData, Storage, StorageError};

/// A filesystem-based storage implementation where each namespace is a JSON file
/// and each file contains key-value pairs.
pub struct FsStorage {
    /// Root directory path for storage
    root_dir: PathBuf,
    /// Currently opened namespaces
    namespaces: HashMap<String, FsNamespace>,
}

/// A namespace implementation backed by a JSON file
pub struct FsNamespace {
    /// Path to the JSON file that stores the key-value pairs
    file_path: PathBuf,
    /// In-memory cache of the key-value pairs
    data: HashMap<String, Vec<u8>>,
    /// Flag to track if the data has been modified and needs to be saved
    modified: bool,
}

impl FsStorage {
    /// Create a new filesystem storage with the given root directory
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Result<Self, StorageError> {
        let root_dir = root_dir.as_ref().to_path_buf();

        // Create the root directory if it doesn't exist
        if !root_dir.exists() {
            fs::create_dir_all(&root_dir)?;
        }

        Ok(Self {
            root_dir,
            namespaces: HashMap::new(),
        })
    }

    /// Get the full path for a namespace
    fn namespace_path(&self, namespace: &str) -> PathBuf {
        self.root_dir.join(format!("{}.json", namespace))
    }
}

impl FsNamespace {
    /// Create a new namespace with the given file path
    fn new(file_path: PathBuf) -> Result<Self, StorageError> {
        let data = if file_path.exists() {
            // Load existing data from file
            let mut file = File::open(&file_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            if contents.is_empty() {
                HashMap::new()
            } else {
                // Deserialize JSON to HashMap<String, Vec<u8>>
                let map: HashMap<String, Vec<u8>> = serde_json::from_str(&contents)?;
                map
            }
        } else {
            // Create a new empty namespace
            HashMap::new()
        };

        Ok(Self {
            file_path,
            data,
            modified: false,
        })
    }

    /// Save the namespace data to disk if modified
    fn save(&mut self) -> Result<(), StorageError> {
        if self.modified {
            // Create parent directories if they don't exist
            if let Some(parent) = self.file_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }

            // Serialize data to JSON
            let json = serde_json::to_string(&self.data)?;

            // Write to file
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.file_path)?;

            file.write_all(json.as_bytes())?;

            self.modified = false;
        }

        Ok(())
    }
}

impl Storage<FsNamespace> for FsStorage {
    fn exist(&self, namespace: &str) -> bool {
        self.namespace_path(namespace).exists()
    }

    fn create(&mut self, namespace: &str) -> Result<String, StorageError> {
        let path = self.namespace_path(namespace);

        // If the namespace already exists, just return its name
        if path.exists() {
            return Ok(namespace.to_string());
        }

        // Create the parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        // Create an empty file
        let file = OpenOptions::new().write(true).create(true).open(&path)?;

        // Close the file immediately
        drop(file);

        Ok(namespace.to_string())
    }

    fn remove(&mut self, namespace: &str) -> Result<(), StorageError> {
        // Remove from in-memory cache if it exists
        self.namespaces.remove(namespace);

        // Remove the file if it exists
        let path = self.namespace_path(namespace);
        if path.exists() {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    fn list(&self, namespace: &str) -> Result<Vec<String>, StorageError> {
        if namespace.is_empty() {
            // List all namespaces at the root level
            let mut namespaces = Vec::new();

            for entry in fs::read_dir(&self.root_dir)? {
                let entry = entry?;
                let path = entry.path();

                // Only include .json files
                if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                    if let Some(file_stem) = path.file_stem() {
                        if let Some(name) = file_stem.to_str() {
                            namespaces.push(name.to_string());
                        }
                    }
                }
            }

            Ok(namespaces)
        } else {
            // This implementation doesn't support nested namespaces
            Err(StorageError::NoNamespace(namespace.to_string()))
        }
    }

    fn open(&mut self, namespace: &str) -> Result<&mut FsNamespace, StorageError> {
        // Check if the namespace is already open
        if !self.namespaces.contains_key(namespace) {
            let path = self.namespace_path(namespace);

            // Check if the namespace exists
            if !path.exists() {
                return Err(StorageError::NoNamespace(namespace.to_string()));
            }

            // Open the namespace
            let ns = FsNamespace::new(path)?;
            self.namespaces.insert(namespace.to_string(), ns);
        }

        // Return a mutable reference to the namespace
        self.namespaces
            .get_mut(namespace)
            .ok_or_else(|| StorageError::NoNamespace(namespace.to_string()))
    }
}

impl Namespace for FsNamespace {
    fn exist(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    fn get(&self, key: &str) -> Result<Option<RawData>, StorageError> {
        Ok(self.data.get(key).map(|data| RawData::from(data.clone())))
    }

    fn put(&mut self, key: &str, value: RawData) -> Result<(), StorageError> {
        let bytes: Vec<u8> = value.to_bytes();
        self.data.insert(key.to_string(), bytes);
        self.modified = true;
        self.save()?;
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<(), StorageError> {
        if self.data.remove(key).is_some() {
            self.modified = true;
            self.save()?;
        }
        Ok(())
    }

    fn keys(&self) -> Result<Vec<String>, StorageError> {
        Ok(self.data.keys().cloned().collect())
    }
}

impl Drop for FsNamespace {
    fn drop(&mut self) {
        // Save any pending changes when the namespace is dropped
        if self.modified {
            let _ = self.save();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_fs_storage() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        // Create a new storage
        let mut storage = FsStorage::new(temp_path).unwrap();

        // Test namespace creation
        let ns_name = "test_namespace";
        storage.create(ns_name).unwrap();
        assert!(storage.exist(ns_name));

        // Test namespace opening
        let namespace = storage.open(ns_name).unwrap();

        // Test key-value operations
        let key = "test_key";
        let value = RawData::from(b"test_value".to_vec());

        // Test put and get
        namespace.put(key, value).unwrap();
        assert!(namespace.exist(key));

        let retrieved = namespace.get(key).unwrap().unwrap();
        let retrieved_str = retrieved.to_string().unwrap();
        assert_eq!(retrieved_str, "test_value");

        // Test keys listing
        let keys = namespace.keys().unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], key);

        // Test key deletion
        namespace.delete(key).unwrap();
        assert!(!namespace.exist(key));

        // Test namespace listing
        let namespaces = storage.list("").unwrap();
        assert_eq!(namespaces.len(), 1);
        assert_eq!(namespaces[0], ns_name);

        // Test namespace removal
        storage.remove(ns_name).unwrap();
        assert!(!storage.exist(ns_name));
    }
}
