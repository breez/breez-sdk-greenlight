use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// A time-based locking mechanism for coordinating access to shared resources across multiple instances.
///
/// `TimeLock` provides distributed locking functionality with automatic expiration. It allows
/// different instances of an application to coordinate access to shared resources by ensuring
/// only one instance can hold a lock at any given time.
///
/// The lock includes a configurable duration and accounts for clock skew between different nodes
/// to prevent race conditions. When a lock expires or is explicitly unlocked, any instance can acquire it.
#[derive(Debug)]
pub struct TimeLock {
    lock_duration: Duration,
    instance_id: String,
    locked_until: SystemTime,
}

/// Indicates whether the lock was previously held by the local or a remote instance.
#[derive(Debug, Eq, PartialEq)]
pub enum PreviousHolder {
    LocalInstance,
    RemoteInstance,
}

/// Represents the owner of a lock when a resource is currently locked by another instance.
#[derive(Debug, Eq, PartialEq)]
pub struct LockedBy(pub String);

impl TimeLock {
    // Allow for some leeway to account for clock skew between different nodes.
    const CLOCK_SKEW_LEEWAY: Duration = Duration::from_secs(3);

    /// Creates a new `TimeLock` instance.
    ///
    /// This method attempts to acquire a lock based on the current state. If the resource
    /// is already locked by another instance and the lock hasn't expired (accounting for
    /// clock skew), it returns `Err(LockedBy(instance_id))`. Otherwise, it creates a new
    /// lock instance.
    ///
    /// # Arguments
    ///
    /// * `lock_duration` - The duration for which the lock will be valid when acquired
    /// * `instance_id` - A unique identifier for this instance
    /// * `latest_lock_data` - The current lock state from persistent storage
    ///
    /// # Returns
    ///
    /// * `Ok((TimeLock, PreviousHolder))` - If the lock can be acquired or if it's already
    ///   owned by this instance, along with information about who previously held the lock
    /// * `Err(LockedBy(instance_id))` - If the resource is currently locked by another instance
    pub fn new(
        lock_duration: Duration,
        instance_id: String,
        latest_lock_data: LockData,
    ) -> Result<(Self, PreviousHolder), LockedBy> {
        debug_assert!(lock_duration > Self::CLOCK_SKEW_LEEWAY);
        debug_assert!(!instance_id.is_empty());
        if instance_id == latest_lock_data.instance_id {
            let tl = Self {
                lock_duration,
                instance_id,
                locked_until: latest_lock_data.locked_until,
            };
            Ok((tl, PreviousHolder::LocalInstance))
        } else if SystemTime::now() < latest_lock_data.locked_until + Self::CLOCK_SKEW_LEEWAY {
            Err(LockedBy(latest_lock_data.instance_id))
        } else {
            let tl = Self {
                lock_duration,
                instance_id,
                locked_until: UNIX_EPOCH,
            };
            Ok((tl, PreviousHolder::RemoteInstance))
        }
    }

    /// Checks if the current instance holds an active lock.
    pub fn is_locked(&self) -> bool {
        SystemTime::now() < self.locked_until - Self::CLOCK_SKEW_LEEWAY
    }

    /// Generates the next lock data for this instance.
    ///
    /// Creates a new `LockData` instance with the current time plus the lock duration
    /// as the expiration time. This is typically used to extend or refresh a lock.
    pub fn next_lock(&self) -> LockData {
        LockData {
            locked_until: SystemTime::now() + self.lock_duration,
            instance_id: self.instance_id.clone(),
        }
    }

    /// Attempts to unlock the resource.
    ///
    /// If this instance currently holds the lock, it releases it and returns the updated
    /// lock data. If this instance doesn't hold the lock, it returns `None`.
    pub fn unlock(&mut self) -> Option<LockData> {
        if self.is_locked() {
            self.locked_until = UNIX_EPOCH;
            let lock_data = LockData {
                locked_until: self.locked_until,
                instance_id: self.instance_id.clone(),
            };
            return Some(lock_data);
        }
        None
    }

    /// Updates the lock with new lock data.
    ///
    /// This method is used to synchronize the local lock state with data from persistent
    /// storage. It should only be called with lock data that belongs to this instance.
    ///
    /// # Returns
    ///
    /// The updated `locked_until` timestamp after applying the new lock data.
    pub fn update_lock(&mut self, lock_data: &LockData) -> SystemTime {
        debug_assert!(self.instance_id == lock_data.instance_id);
        self.locked_until = lock_data.locked_until;
        self.locked_until
    }
}

/// Represents the persistent state of a time-based lock.
///
/// `LockData` contains the information needed to track lock ownership and expiration
/// across different instances. It can be serialized to and from persistent storage
/// to maintain lock state across application restarts.
#[derive(Debug, Serialize, Deserialize)]
pub struct LockData {
    /// The timestamp when the lock expires.
    locked_until: SystemTime,

    /// The unique identifier of the instance that holds the lock.
    ///
    /// This field identifies which instance currently owns the lock. It's used
    /// to determine if a lock attempt should succeed (same instance) or fail
    /// (different instance with active lock).
    instance_id: String,
}

impl LockData {
    pub fn decode(data: &[u8]) -> Result<Self, serde_json::Error> {
        if data.is_empty() {
            return Ok(LockData {
                locked_until: UNIX_EPOCH,
                instance_id: String::new(),
            });
        }
        serde_json::from_slice(data)
    }

    pub fn encode(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn test_lock_data_encode_decode() {
        let timestamp = UNIX_EPOCH + Duration::from_secs(1234567890);
        let instance_id = "test-instance-123".to_string();

        let lock_data = LockData {
            locked_until: timestamp,
            instance_id: instance_id.clone(),
        };

        let encoded = lock_data.encode().unwrap();
        let decoded = LockData::decode(&encoded).unwrap();

        assert_eq!(decoded.locked_until, timestamp);
        assert_eq!(decoded.instance_id, instance_id);
    }

    #[test]
    fn test_lock_data_decode_empty_data() {
        let lock_data = LockData::decode(&[]).unwrap();
        assert_eq!(lock_data.locked_until, UNIX_EPOCH);
        assert_eq!(lock_data.instance_id, "");
    }

    #[test]
    fn test_lock_data_decode_invalid_json() {
        let invalid_json = "invalid json data".as_bytes();
        LockData::decode(invalid_json).unwrap_err();
    }

    #[test]
    fn test_lock_data_roundtrip_with_special_characters() {
        let timestamp = SystemTime::now();
        let instance_id = "instance-with-dashes_and_underscores.and.dots".to_string();

        let lock_data = LockData {
            locked_until: timestamp,
            instance_id: instance_id.clone(),
        };

        let encoded = lock_data.encode().unwrap();
        let decoded = LockData::decode(&encoded).unwrap();

        assert_eq!(decoded.locked_until, timestamp);
        assert_eq!(decoded.instance_id, instance_id);
    }

    // TimeLock tests
    #[test]
    fn test_time_lock_not_locked() {
        let lock_duration = Duration::from_secs(10);
        let instance_id = "test-instance".to_string();
        let latest_lock_data = LockData {
            locked_until: UNIX_EPOCH, // Expired lock
            instance_id: "other-instance".to_string(),
        };
        let (time_lock, previous_holder) =
            TimeLock::new(lock_duration, instance_id, latest_lock_data).unwrap();
        assert!(!time_lock.is_locked());
        assert_eq!(previous_holder, PreviousHolder::RemoteInstance);
    }

    #[test]
    fn test_time_lock_new_with_active_remote_lock() {
        let lock_duration = Duration::from_secs(10);
        let instance_id = "test-instance".to_string();
        let future_time = SystemTime::now() + Duration::from_secs(30);
        let latest_lock_data = LockData {
            locked_until: future_time,
            instance_id: "other-instance".to_string(),
        };
        let result = TimeLock::new(lock_duration, instance_id, latest_lock_data).unwrap_err();
        assert_eq!(result, LockedBy("other-instance".to_string()));
    }

    #[test]
    fn test_time_lock_new_with_own_expired_lock() {
        let lock_duration = Duration::from_secs(10);
        let instance_id = "test-instance".to_string();
        let past_time = SystemTime::now() - Duration::from_secs(30);
        let latest_lock_data = LockData {
            locked_until: past_time,
            instance_id: instance_id.clone(),
        };
        let (time_lock, previous_holder) =
            TimeLock::new(lock_duration, instance_id, latest_lock_data).unwrap();
        assert!(!time_lock.is_locked());
        assert_eq!(previous_holder, PreviousHolder::LocalInstance);
    }

    #[test]
    fn test_time_lock_is_locked() {
        let lock_duration = Duration::from_secs(10);
        let instance_id = "test-instance".to_string();
        let future_time = SystemTime::now() + Duration::from_secs(30);
        let latest_lock_data = LockData {
            locked_until: future_time,
            instance_id: instance_id.clone(),
        };
        let (time_lock, previous_holder) =
            TimeLock::new(lock_duration, instance_id, latest_lock_data).unwrap();
        assert!(time_lock.is_locked());
        assert_eq!(previous_holder, PreviousHolder::LocalInstance);
    }

    #[test]
    fn test_time_lock_lifecycle() {
        let lock_duration = Duration::from_secs(10);
        let instance_id = "test-instance".to_string();
        let latest_lock_data = LockData {
            locked_until: UNIX_EPOCH,
            instance_id: "other-instance".to_string(),
        };

        let (mut time_lock, previous_holder) =
            TimeLock::new(lock_duration, instance_id.clone(), latest_lock_data).unwrap();
        assert!(!time_lock.is_locked());
        assert_eq!(previous_holder, PreviousHolder::RemoteInstance);

        // Get next lock
        let lock_data = time_lock.next_lock();
        assert_eq!(lock_data.instance_id, instance_id);

        // Update lock with new data
        let updated_time = time_lock.update_lock(&lock_data);
        assert_eq!(updated_time, lock_data.locked_until);
        assert!(time_lock.is_locked());

        // Unlock
        let unlock_result = time_lock.unlock();
        assert!(unlock_result.is_some());
        assert!(!time_lock.is_locked());
    }

    #[test]
    fn test_time_lock_multiple_instances() {
        let lock_duration = Duration::from_secs(10);
        let instance_id_1 = "instance-1".to_string();
        let instance_id_2 = "instance-2".to_string();

        // Instance 1 creates a lock
        let (time_lock_1, previous_holder) = TimeLock::new(
            lock_duration,
            instance_id_1.clone(),
            LockData {
                locked_until: UNIX_EPOCH,
                instance_id: "other".to_string(),
            },
        )
        .unwrap();
        assert_eq!(previous_holder, PreviousHolder::RemoteInstance);

        let lock_data_1 = time_lock_1.next_lock();

        // Instance 2 tries to create a lock while instance 1's lock is active
        let result = TimeLock::new(lock_duration, instance_id_2, lock_data_1).unwrap_err();
        assert_eq!(result, LockedBy("instance-1".to_string()));
    }
}
