use std::fmt;
use tonic::async_trait;

/// Errors that can occur during versioned store operations.
#[derive(Debug)]
pub enum Error {
    /// A conflict occurred, typically when trying to update a key with an outdated version.
    /// The string contains details about the conflict.
    Conflict(String),
    /// An internal error occurred during the operation.
    /// The string contains details about the internal error.
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

/// A trait for a versioned key-value store.
///
/// This store maintains version numbers for each key to detect concurrent modifications.
/// When updating a key, the version must match the current version in the store,
/// otherwise a conflict error is returned.
/// This trait provides a simplified abstraction over
/// <https://docs.rs/vss-client/latest/vss_client/client/struct.VssClient.html>.
#[async_trait]
pub trait VersionedStore {
    /// Retrieves a value and its version from the store.
    ///
    /// # Arguments
    /// * `key` - The key to retrieve
    ///
    /// # Returns
    /// * `Ok(Some((value, version)))` - The value and its version if the key exists
    /// * `Ok(None)` - If the key doesn't exist
    /// * `Err(Error)` - If an error occurred during retrieval
    async fn get(&self, key: String) -> Result<Option<(Vec<u8>, i64)>, Error>;

    /// Stores a value with a specific version in the store.
    ///
    /// This operation will fail with a `Conflict` error if the provided version
    /// doesn't match the current version in the store, indicating concurrent modification.
    ///
    /// # Arguments
    /// * `key` - The key to store
    /// * `value` - The value to store
    /// * `version` - The expected current version of the key
    ///
    /// # Returns
    /// * `Ok(())` - If the value was stored successfully
    /// * `Err(Error::Conflict)` - If the version doesn't match (concurrent modification)
    /// * `Err(Error::Internal)` - If an internal error occurred
    async fn put(&self, key: String, value: Vec<u8>, version: i64) -> Result<(), Error>;

    /// Deletes a key from the store.
    ///
    /// # Arguments
    /// * `key` - The key to delete
    ///
    /// # Returns
    /// * `Ok(())` - If the key was deleted successfully or didn't exist
    /// * `Err(Error)` - If an error occurred during deletion
    async fn delete(&self, key: String) -> Result<(), Error>;

    /// Lists all keys and their versions in the store.
    ///
    /// # Returns
    /// * `Ok(Vec<(String, i64)>)` - A list of (key, version) pairs
    /// * `Err(Error)` - If an error occurred during listing
    async fn list(&self) -> Result<Vec<(String, i64)>, Error>;
}
