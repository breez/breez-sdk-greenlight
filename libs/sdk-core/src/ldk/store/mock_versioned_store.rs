use crate::ldk::store::versioned_store::{Error, VersionedStore};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tonic::async_trait;

/// A mock implementation of VersionedStore for testing purposes.
///
/// This mock store maintains an in-memory HashMap to simulate a versioned key-value store.
/// It can be configured to return specific errors or simulate various scenarios for testing.
#[derive(Default, Clone)]
pub struct MockVersionedStore {
    pub data: Arc<Mutex<HashMap<String, (Vec<u8>, i64)>>>,
    pub should_fail_get: bool,
    pub should_fail_put: bool,
    pub should_fail_delete: bool,
    pub should_fail_list: bool,
    pub conflict_on_put: bool,
}

#[async_trait]
impl VersionedStore for MockVersionedStore {
    async fn get(&self, key: String) -> Result<Option<(Vec<u8>, i64)>, Error> {
        if self.should_fail_get {
            return Err(Error::Internal("Mock get failure".to_string()));
        }

        let data = self.data.lock().unwrap();
        Ok(data.get(&key).cloned())
    }

    async fn put(&self, key: String, value: Vec<u8>, version: i64) -> Result<(), Error> {
        if self.should_fail_put {
            return Err(Error::Internal("Mock put failure".to_string()));
        }

        let mut data = self.data.lock().unwrap();

        if self.conflict_on_put {
            return Err(Error::Conflict("Mock version conflict".to_string()));
        }

        // Check if the version matches (simulate version conflict detection)
        if let Some((_, current_version)) = data.get(&key) {
            if *current_version != version {
                return Err(Error::Conflict(format!(
                    "Version mismatch: expected {current_version}, got {version}"
                )));
            }
        }

        data.insert(key, (value, version + 1));
        Ok(())
    }

    async fn delete(&self, key: String) -> Result<(), Error> {
        if self.should_fail_delete {
            return Err(Error::Internal("Mock delete failure".to_string()));
        }

        let mut data = self.data.lock().unwrap();
        data.remove(&key);
        Ok(())
    }

    async fn list(&self) -> Result<Vec<(String, i64)>, Error> {
        if self.should_fail_list {
            return Err(Error::Internal("Mock list failure".to_string()));
        }

        let data = self.data.lock().unwrap();
        let result: Vec<(String, i64)> = data
            .iter()
            .map(|(key, (_, version))| (key.clone(), *version))
            .collect();
        Ok(result)
    }
}
