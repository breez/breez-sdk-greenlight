use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusqlite::{named_params, TransactionBehavior};

use super::{
    db::SqliteStorage,
    error::{PersistError, PersistResult},
};

impl SqliteStorage {
    pub(crate) fn get_current_tip(&self) -> PersistResult<Option<(u32, SystemTime)>> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare("SELECT tip, time FROM current_tip")?;
        let (tip, time) = stmt.query_row([], |row| {
            let tip: u32 = row.get("tip")?;
            let time: u64 = row.get("time")?;
            Ok((tip, time))
        })?;

        match tip {
            0 => Ok(None),
            _ => {
                let time = SystemTime::UNIX_EPOCH
                    .checked_add(Duration::from_secs(time))
                    .ok_or(PersistError::generic("invalid system time"))?;
                Ok(Some((tip, time)))
            }
        }
    }

    pub(crate) fn set_current_tip(&self, tip: u32) -> PersistResult<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        tx.execute(
            "UPDATE current_tip
             SET tip = :tip",
            named_params! {
                ":tip": tip,
                ":time": time
            },
        )?;

        tx.commit()?;
        Ok(())
    }
}
