use super::db::SqliteStorage;
use anyhow::Result;
use std::path::Path;

impl SqliteStorage {
    pub(crate) fn backup<P: AsRef<Path>>(&self, dst_path: P) -> Result<()> {
        self.get_connection()?
            .backup(rusqlite::DatabaseName::Attached("sync"), dst_path, None)
            .map_err(anyhow::Error::msg)
    }

    pub fn get_last_sync_version(&self) -> Result<Option<u64>> {
        let res: rusqlite::Result<Option<u64>> = self.get_connection()?.query_row(
            "SELECT max(last_version) FROM sync_versions",
            [],
            |row| row.get::<usize, Option<u64>>(0),
        );
        res.map_err(anyhow::Error::msg)
    }

    pub fn set_last_sync_version(&self, last_version: u64) -> Result<()> {
        self.get_connection()?.execute(
            "INSERT OR REPLACE INTO sync_versions (last_version) VALUES (?1)",
            [last_version],
        )?;
        Ok(())
    }

    pub fn get_last_sync_request(&self) -> Result<Option<u64>> {
        let res: rusqlite::Result<Option<u64>> =
            self.get_connection()?
                .query_row("SELECT max(id) FROM sync.sync_requests", [], |row| {
                    row.get::<usize, Option<u64>>(0)
                });
        res.map_err(anyhow::Error::msg)
    }

    pub fn delete_sync_requests_up_to(&self, request_id: u64) -> Result<()> {
        self.get_connection()?.execute(
            "DELETE FROM sync.sync_requests where id <= ?1",
            [request_id],
        )?;
        Ok(())
    }

    pub fn add_sync_request(&self) -> Result<()> {
        self.get_connection()?.execute(
            " INSERT INTO sync_requests(changed_table) VALUES('user')",
            [],
        )?;
        Ok(())
    }

    pub fn import_remote_changes(&self, remote_storage: &SqliteStorage) -> Result<()> {
        let sync_data_file = remote_storage.sync_db_path();

        let mut con = self.get_connection()?;
        let tx = con.transaction()?;
        tx.execute("ATTACH DATABASE ? AS remote_sync;", [sync_data_file])?;
        tx.execute("insert into sync.swaps select * from remote_sync.swaps where bitcoin_address not in (select bitcoin_address from sync.swaps);", [])?;
        tx.execute(
            "insert into sync.swap_refunds select * from remote_sync.swap_refunds where bitcoin_address not in (select bitcoin_address from sync.swap_refunds);",
            [],
        )?;
        tx.execute(
            "insert into sync.payments_external_info select * from remote_sync.payments_external_info where payment_id not in (select payment_id from sync.payments_external_info);",
            [],
        )?;
        tx.commit()?;
        con.execute("DETACH DATABASE remote_sync", [])?;
        Ok(())
    }
}

#[test]
fn test_sync() {
    use crate::persist::test_utils;
    let local_storage = SqliteStorage::new(test_utils::create_test_sql_dir());
    local_storage.init().unwrap();

    let local_swap_info = crate::SwapInfo {
        bitcoin_address: String::from("1"),
        created_at: 0,
        lock_height: 100,
        payment_hash: vec![1],
        preimage: vec![2],
        private_key: vec![3],
        public_key: vec![4],
        swapper_public_key: vec![5],
        script: vec![5],
        bolt11: None,
        paid_sats: 0,
        unconfirmed_sats: 0,
        confirmed_sats: 0,
        status: crate::models::SwapStatus::Initial,
        refund_tx_ids: Vec::new(),
        unconfirmed_tx_ids: Vec::new(),
        confirmed_tx_ids: Vec::new(),
        min_allowed_deposit: 0,
        max_allowed_deposit: 100,
        last_redeem_error: None,
    };
    local_storage.insert_swap(local_swap_info.clone()).unwrap();

    let mut remote_swap_info = local_swap_info.clone();
    remote_swap_info.bitcoin_address = "2".into();
    remote_swap_info.script = vec![6];
    remote_swap_info.swapper_public_key = vec![6];
    remote_swap_info.public_key = vec![6];
    remote_swap_info.preimage = vec![6];
    remote_swap_info.payment_hash = vec![6];
    remote_swap_info.private_key = vec![6];

    let remote_storage = SqliteStorage::new(test_utils::create_test_sql_dir());
    remote_storage.init().unwrap();
    remote_storage
        .insert_swap(remote_swap_info.clone())
        .unwrap();
    remote_storage
        .import_remote_changes(&local_storage)
        .unwrap();
    local_storage
        .import_remote_changes(&remote_storage)
        .unwrap();

    let mut local_swaps = local_storage.list_swaps().unwrap();
    local_swaps.sort_by(|s1, s2| s1.bitcoin_address.cmp(&s2.bitcoin_address));

    let mut remote_swaps = local_storage.list_swaps().unwrap();
    remote_swaps.sort_by(|s1, s2| s1.bitcoin_address.cmp(&s2.bitcoin_address));

    assert_eq!(local_swaps, remote_swaps);
    assert_eq!(local_swaps.len(), 2);

    local_storage.set_last_sync_version(10).unwrap();
    let version = local_storage.get_last_sync_version().unwrap().unwrap();
    assert_eq!(version, 10);
}
