use super::db::SqliteStorage;
use anyhow::Result;
use rusqlite::Row;
use std::path::Path;

#[allow(dead_code)]
pub(crate) struct SyncVersion {
    pub created_at: String,
    pub last_version: u64,
    pub data: Vec<u8>,
}

impl SqliteStorage {
    pub(crate) fn backup<P: AsRef<Path>>(&self, dst_path: P) -> Result<()> {
        self.get_connection()?
            .backup(rusqlite::DatabaseName::Attached("sync"), dst_path, None)
            .map_err(anyhow::Error::msg)
    }

    pub(crate) fn get_last_sync_version(&self) -> Result<Option<u64>> {
        let res: rusqlite::Result<Option<u64>> = self.get_connection()?.query_row(
            "SELECT max(last_version) FROM sync_versions",
            [],
            |row| row.get::<usize, Option<u64>>(0),
        );
        res.map_err(anyhow::Error::msg)
    }

    pub(crate) fn set_last_sync_version(&self, last_version: u64, data: &Vec<u8>) -> Result<()> {
        let con = self.get_connection()?;

        // make sure we have no more than 20 history entries
        con.execute("Delete from sync_versions where last_version not in (select last_version from sync_versions order by created_at desc limit 19);", [])?;
        con.execute(
            "INSERT OR REPLACE INTO sync_versions (last_version, data) VALUES (?1, ?2);",
            (last_version, data),
        )?;

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn sync_versions_history(&self) -> Result<Vec<SyncVersion>> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare(
            "SELECT created_at, last_version, data FROM sync_versions ORDER BY created_at DESC;",
        )?;

        let vec: Vec<SyncVersion> = stmt
            .query_map([], |row| self.sql_row_to_sync_version(row))?
            .map(|i| i.unwrap())
            .collect();

        Ok(vec)
    }

    fn sql_row_to_sync_version(&self, row: &Row) -> Result<SyncVersion, rusqlite::Error> {
        let version = SyncVersion {
            created_at: row.get(0)?,
            last_version: row.get(1)?,
            data: row.get(2)?,
        };

        Ok(version)
    }

    pub fn get_last_sync_request(&self) -> Result<Option<u64>> {
        let res: rusqlite::Result<Option<u64>> =
            self.get_connection()?
                .query_row("SELECT max(id) FROM sync.sync_requests", [], |row| {
                    row.get::<usize, Option<u64>>(0)
                });
        res.map_err(anyhow::Error::msg)
    }

    pub(crate) fn delete_sync_requests_up_to(&self, request_id: u64) -> Result<()> {
        self.get_connection()?.execute(
            "DELETE FROM sync.sync_requests where id <= ?1",
            [request_id],
        )?;
        Ok(())
    }

    pub(crate) fn import_remote_changes(&self, remote_storage: &SqliteStorage) -> Result<()> {
        let sync_data_file = remote_storage.sync_db_path();
        match SqliteStorage::migrate_sync_db(sync_data_file.clone()) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to migrate sync db, probably local db is older than remote, skipping migration: {}", e);
            }
        }

        let mut con = self.get_connection()?;
        let tx = con.transaction()?;
        tx.execute("ATTACH DATABASE ? AS remote_sync;", [sync_data_file])?;

        // sync remote swaps table
        tx.execute(
            "
        INSERT INTO sync.swaps 
          SELECT
           bitcoin_address,
           created_at,
           lock_height,
           payment_hash,
           preimage,
           private_key,
           public_key,
           swapper_public_key,
           script,
           min_allowed_deposit,
           max_allowed_deposit
          FROM remote_sync.swaps
          WHERE bitcoin_address NOT IN (SELECT bitcoin_address FROM sync.swaps);",
            [],
        )?;

        // sync remote swap_refunds table
        tx.execute(
            "
        INSERT INTO sync.swap_refunds
         SELECT
          bitcoin_address,
          refund_tx_id
         FROM remote_sync.swap_refunds
         WHERE bitcoin_address NOT IN (SELECT bitcoin_address FROM sync.swap_refunds);",
            [],
        )?;

        // sync remote payments_external_info table
        tx.execute(
            "
         INSERT into sync.payments_external_info
         SELECT
          payment_id,
          lnurl_success_action,
          ln_address,
          lnurl_metadata
         FROM remote_sync.payments_external_info
         WHERE payment_id NOT IN (SELECT payment_id FROM sync.payments_external_info);",
            [],
        )?;

        // sync remote reverse_swaps table
        tx.execute(
            "
        INSERT into sync.reverse_swaps
        SELECT
         id,
         created_at_block_height,
         preimage,
         private_key,
         claim_pubkey,
         timeout_block_height,
         invoice,
         onchain_amount_sat,
         sat_per_vbyte,
         redeem_script
        FROM remote_sync.reverse_swaps
        WHERE id NOT IN (SELECT id FROM sync.reverse_swaps);",
            [],
        )?;

        // sync remote swap_refunds table
        tx.execute(
            "
        INSERT INTO sync.open_channel_payment_info
         SELECT
          payment_hash,
          payer_amount_msat
         FROM remote_sync.open_channel_payment_info
         WHERE payment_hash NOT IN (SELECT payment_hash FROM sync.open_channel_payment_info);",
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

    let mut remote_swap_info = local_swap_info;
    remote_swap_info.bitcoin_address = "2".into();
    remote_swap_info.script = vec![6];
    remote_swap_info.swapper_public_key = vec![6];
    remote_swap_info.public_key = vec![6];
    remote_swap_info.preimage = vec![6];
    remote_swap_info.payment_hash = vec![6];
    remote_swap_info.private_key = vec![6];

    let remote_storage = SqliteStorage::new(test_utils::create_test_sql_dir());
    remote_storage.init().unwrap();
    remote_storage.insert_swap(remote_swap_info).unwrap();

    remote_storage
        .insert_open_channel_payment_info("123", 100000)
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

    local_storage.set_last_sync_version(10, &vec![]).unwrap();
    let version = local_storage.get_last_sync_version().unwrap().unwrap();
    assert_eq!(version, 10);
}
