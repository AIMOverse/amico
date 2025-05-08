/// Error type for all storage operations.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Key not found: {0}")]
    NotFound(String),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Failed to convert from RawData: {0}")]
    FromBytes(String),
    // future: add backend-specific variants
}

/// Abstract key/value storage where `namespace` groups tables or
/// directories, and each key maps to raw data.
pub trait Storage {
    /// Read raw bytes for `key` in `namespace`. Returns None if missing.
    fn get(&self, namespace: &str, key: &str) -> Result<Option<RawData>, StorageError>;

    /// Write `value` bytes for `key` in `namespace`, overwriting if exists.
    fn put(&mut self, namespace: &str, key: &str, value: RawData) -> Result<(), StorageError>;

    /// Delete `key` in `namespace`. No-op if missing.
    fn delete(&mut self, namespace: &str, key: &str) -> Result<(), StorageError>;

    /// List all existing keys in `namespace`.
    fn list_keys(&self, namespace: &str) -> Result<Vec<String>, StorageError>;
}

/// The data stored in the Storage in raw bytes
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
}
