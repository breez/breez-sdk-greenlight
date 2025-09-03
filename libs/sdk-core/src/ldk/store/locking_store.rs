use crate::ldk::store::time_lock::{LockData, LockedBy, PreviousHolder, TimeLock};
use crate::ldk::store::versioned_store::{Error, VersionedStore};
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;
use tonic::async_trait;

struct VersionedTimeLock {
    tl: TimeLock,
    version: i64,
}

/// A wrapper around a `VersionedStore` that provides distributed locking capabilities.
///
/// `LockingStore` ensures that only one instance can access the underlying store at a time
/// by implementing a distributed locking mechanism. This is useful in scenarios where multiple
/// processes or instances might try to access the same storage concurrently.
/// It provides *exclusive* access to the store, meaning no one else can acquire/release
/// the lock while the `LockingStore` is managed.
/// *Warning:* If the lock is somehow lost, `LockingStore` cannot recover,
/// and all operations (except `unlock()`) will result in a `Conflict` error.
///
/// The locking mechanism works by:
/// 1. Acquiring a lock when the store is created
/// 2. Refreshing the lock periodically to maintain ownership
/// 3. Releasing the lock when explicitly unlocked
///
/// If another instance already holds the lock, attempting to create a new `LockingStore`
/// will result in a `Conflict` error.
pub struct LockingStore<S: VersionedStore + Send + Sync> {
    inner: S,
    versioned_tl: Mutex<VersionedTimeLock>,
}

#[allow(dead_code)]
impl<S: VersionedStore + Send + Sync> LockingStore<S> {
    const KEY: &str = "lock";
    const LOCK_DURATION: Duration = Duration::from_secs(60);
    const REFRESH_WINDOW: Duration = Duration::from_secs(30);

    /// Creates a new `LockingStore` and attempts to acquire the distributed lock.
    ///
    /// This method will:
    /// 1. Check if a lock already exists in the underlying store
    /// 2. If no lock exists, create one for this instance
    /// 3. If a lock exists, verify it's not held by another instance
    /// 4. If the lock is held by another instance, return a `Conflict` error
    pub async fn new(instance_id: String, store: S) -> Result<(Self, PreviousHolder), Error> {
        let (lock_data, version) = store.get(Self::KEY.to_string()).await?.unwrap_or_default();
        let lock_data = LockData::decode(&lock_data)
            .map_err(|e| Error::Internal(format!("Failed to decode lock_data: {e:?}")))?;
        let (tl, previous_holder) = TimeLock::new(Self::LOCK_DURATION, instance_id, lock_data)
            .map_err(|LockedBy(instance_id)| {
                Error::Conflict(format!("Remote lock aquired by `{instance_id}`"))
            })?;
        let versioned_tl = Mutex::new(VersionedTimeLock { tl, version });

        let locking_store = Self {
            inner: store,
            versioned_tl,
        };
        locking_store.lock().await?;

        Ok((locking_store, previous_holder))
    }

    /// Refreshes the distributed lock to extend its duration.
    ///
    /// This method should be called periodically to maintain ownership of the lock.
    ///
    /// # Returns
    /// * `Ok(SystemTime)` - The time until which the lock should be refreshed
    /// * `Err(Error::Conflict)` - If the lock was lost or cannot be refreshed
    /// * `Err(Error::Internal)` - If there was an internal error
    pub async fn refresh_lock(&self) -> Result<SystemTime, Error> {
        // Note: An optimization is possible here.
        // If we failed to lock here, we can still try to lock again,
        // asserting that the previous lock was acquired by us
        // (i.e., no one acquired/released the lock in between).
        let locked_until = self.lock().await?;
        Ok(locked_until - Self::REFRESH_WINDOW)
    }

    async fn lock(&self) -> Result<SystemTime, Error> {
        // Hold the lock on `self.versioned_tl` during the whole call to avoid
        // race conditions with `unlock()` method.
        let mut versioned_tl = self.versioned_tl.lock().await;
        let lock_data = versioned_tl.tl.next_lock();
        let value = lock_data
            .encode()
            .map_err(|e| Error::Internal(format!("Failed to encode lock_data: {e}")))?;
        self.inner
            .put(Self::KEY.to_string(), value, versioned_tl.version)
            .await?;
        versioned_tl.version += 1;
        Ok(versioned_tl.tl.update_lock(&lock_data))
    }

    /// Releases the distributed lock.
    ///
    /// This method should be called when the instance is done using the store
    /// to allow other instances to acquire the lock. If the lock has already
    /// expired or been lost, this method will still succeed.
    ///
    /// # Returns
    /// * `Ok(())` - If the lock was successfully released or was already expired
    /// * `Err(Error::Internal)` - If there was an internal error during release
    pub async fn unlock(&self) -> Result<(), Error> {
        // Hold the lock on `self.versioned_tl` during the whole call to avoid
        // race conditions with `lock()` method.
        let mut versioned_tl = self.versioned_tl.lock().await;
        if let Some(lock_data) = versioned_tl.tl.unlock() {
            let value = lock_data
                .encode()
                .map_err(|e| Error::Internal(format!("Failed to encode lock_data: {e}")))?;
            self.inner
                .put(Self::KEY.to_string(), value, versioned_tl.version)
                .await?
        }
        Ok(())
    }

    async fn ensure_locked(&self) -> Result<(), Error> {
        if !self.versioned_tl.lock().await.tl.is_locked() {
            return Err(Error::Conflict("Remote lock was not aquired".to_string()));
        }
        Ok(())
    }
}

/// Implementation of `VersionedStore` for `LockingStore`.
///
/// All store operations are wrapped with lock validation to ensure that
/// only the instance holding the lock can perform operations on the underlying store.
#[async_trait]
impl<S: VersionedStore + Send + Sync> VersionedStore for LockingStore<S> {
    async fn get(&self, key: String) -> Result<Option<(Vec<u8>, i64)>, Error> {
        self.ensure_locked().await?;
        self.inner.get(key).await
    }

    async fn put(&self, key: String, value: Vec<u8>, version: i64) -> Result<(), Error> {
        self.ensure_locked().await?;
        self.inner.put(key, value, version).await
    }

    async fn delete(&self, key: String) -> Result<(), Error> {
        self.ensure_locked().await?;
        self.inner.delete(key).await
    }

    async fn list(&self) -> Result<Vec<(String, i64)>, Error> {
        self.ensure_locked().await?;
        self.inner.list().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ldk::store::mock_versioned_store::MockVersionedStore;

    #[tokio::test]
    async fn test_locking_store() {
        let store = MockVersionedStore::default();
        let instance_id_1 = "instance_1".to_string();
        let (locking_store, previous_holder) = LockingStore::new(instance_id_1, store.clone())
            .await
            .unwrap();
        assert_eq!(previous_holder, PreviousHolder::RemoteInstance);

        // Lock was aquired, store can be accessed.
        locking_store
            .put("key".to_string(), "value".as_bytes().to_vec(), 1)
            .await
            .unwrap();
        let data = store.data.lock().unwrap().get("key").unwrap().clone();
        assert_eq!(data.0, "value".as_bytes());
        assert_eq!(data.1, 2);
        let next_lock = locking_store.refresh_lock().await.unwrap();
        assert!(SystemTime::now() < next_lock);

        // Another instance tries to acquire the lock.
        let instance_id_2 = "instance_2".to_string();
        let result = LockingStore::new(instance_id_2.clone(), store.clone()).await;
        assert!(matches!(result, Err(Error::Conflict(_))));

        // The first instance releases the lock.
        locking_store.unlock().await.unwrap();
        locking_store
            .put("key".to_string(), "value2".as_bytes().to_vec(), 2)
            .await
            .unwrap_err();

        // Another instance tries to acquire the lock again.
        let (locking_store2, previous_holder) =
            LockingStore::new(instance_id_2.clone(), store.clone())
                .await
                .unwrap();
        assert_eq!(previous_holder, PreviousHolder::RemoteInstance);
        locking_store2
            .put("key".to_string(), "value2".as_bytes().to_vec(), 2)
            .await
            .unwrap();
        let data = store.data.lock().unwrap().get("key").unwrap().clone();
        assert_eq!(data.0, "value2".as_bytes());
        assert_eq!(data.1, 3);

        // The instance crashed before releasing the lock.
        drop(locking_store2);
        // but it can instantly reaquire the lock.
        let _locking_store2 = LockingStore::new(instance_id_2, store).await.unwrap();
    }
}
