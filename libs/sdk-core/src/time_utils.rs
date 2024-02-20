use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) struct BreezTimeUtils;

pub(crate) trait TimeUtils: Send + Sync {
    fn get_current_time(&self) -> i64;
}

impl TimeUtils for BreezTimeUtils {
    fn get_current_time(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}

pub(crate) struct TestTimeUtils {
    pub current_time: i64,
}

impl TimeUtils for TestTimeUtils {
    fn get_current_time(&self) -> i64 {
        self.current_time
    }
}
