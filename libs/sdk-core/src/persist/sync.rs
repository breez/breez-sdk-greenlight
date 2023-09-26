use crate::{log_error, ReverseSwapStatus};

use super::db::SqliteStorage;
use anyhow::Result;
use rusqlite::{named_params, Row, Transaction};
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

    pub(crate) fn import_remote_changes(
        &self,
        remote_storage: &SqliteStorage,
        to_local: bool,
    ) -> Result<()> {
        let sync_data_file = remote_storage.sync_db_path();
        match SqliteStorage::migrate_sync_db(sync_data_file.clone()) {
            Ok(_) => {}
            Err(e) => {
                log_error!(self.logger, "Failed to migrate sync db, probably local db is older than remote, skipping migration: {}", e);
            }
        }

        let mut con = self.get_connection()?;
        let tx = con.transaction()?;
        tx.execute("ATTACH DATABASE ? AS remote_sync;", [sync_data_file])?;

        if to_local {
            tx.execute(
                "
            INSERT OR IGNORE INTO swaps_info (bitcoin_address, unconfirmed_tx_ids, confirmed_tx_ids)
                SELECT
                    bitcoin_address, '[]', '[]'
                FROM remote_sync.swaps;",
                [],
            )?;

            tx.execute(
                "
            INSERT OR IGNORE INTO reverse_swaps_info (id, status)
                SELECT
                    id, :status
                FROM remote_sync.reverse_swaps;",
                named_params! {
                    ":status": serde_json::to_value(ReverseSwapStatus::Initial)?
                },
            )?;
        }

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
          lnurl_metadata,
          lnurl_withdraw_endpoint
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

        // Sync remote swaps_fees table, which contains dynamic fees used in swaps
        // created_at is used to settle conflicts, since we assume small variations in the client local times
        Self::sync_swaps_fees_local(&tx)?;

        tx.commit()?;
        con.execute("DETACH DATABASE remote_sync", [])?;

        Ok(())
    }

    /// Insert or update to local db all rows that have created_at larger than in the local
    fn sync_swaps_fees_local(tx: &Transaction) -> Result<()> {
        // The WHERE clause covers both possible scenarios for the swaps_fees table:
        // - Local DB doesn't have a row matching a remote DB row with the same swap address
        //   - checked via `sync.swaps_fees.created_at IS NULL`
        //   - `created_at` is NOT NULL in the schema, so matching this means finding an address for which no local DB row exists
        // - Local and remote DBs have a row for the same swap address and remote crated_at > local created_at
        tx.execute(
            "
        INSERT OR REPLACE INTO sync.swaps_fees
         SELECT
          remote_sync.swaps_fees.bitcoin_address as bitcoin_address,
          remote_sync.swaps_fees.created_at as created_at,
          remote_sync.swaps_fees.channel_opening_fees as channel_opening_fees
         FROM remote_sync.swaps_fees
          LEFT JOIN sync.swaps_fees ON sync.swaps_fees.bitcoin_address = remote_sync.swaps_fees.bitcoin_address
         WHERE
          sync.swaps_fees.created_at IS NULL OR remote_sync.swaps_fees.created_at > sync.swaps_fees.created_at
         ;",
            [],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use rand::random;
    use std::sync::Arc;
    use std::time::Duration;

    use crate::persist::db::SqliteStorage;
    use crate::persist::test_utils;
    use crate::test_utils::{get_test_ofp_48h, rand_string, rand_vec_u8};
    use crate::NopLogger;
    use crate::SwapInfo;

    #[test]
    fn test_sync() -> Result<()> {
        let local_storage = SqliteStorage::new(
            test_utils::create_test_sql_dir(),
            Arc::new(Box::new(NopLogger {})),
        );
        local_storage.init()?;

        let local_swap_info = create_test_swap_info();
        local_storage.insert_swap(local_swap_info.clone())?;

        let mut remote_swap_info = local_swap_info;
        remote_swap_info.bitcoin_address = "2".into();
        remote_swap_info.script = vec![6];
        remote_swap_info.swapper_public_key = vec![6];
        remote_swap_info.public_key = vec![6];
        remote_swap_info.preimage = vec![6];
        remote_swap_info.payment_hash = vec![6];
        remote_swap_info.private_key = vec![6];

        let remote_storage = SqliteStorage::new(
            test_utils::create_test_sql_dir(),
            Arc::new(Box::new(NopLogger {})),
        );
        remote_storage.init()?;
        remote_storage.insert_swap(remote_swap_info)?;

        remote_storage.insert_open_channel_payment_info("123", 100000)?;

        remote_storage.import_remote_changes(&local_storage, false)?;
        local_storage.import_remote_changes(&remote_storage, true)?;

        let mut local_swaps = local_storage.list_swaps()?;
        local_swaps.sort_by(|s1, s2| s1.bitcoin_address.cmp(&s2.bitcoin_address));

        let mut remote_swaps = local_storage.list_swaps()?;
        remote_swaps.sort_by(|s1, s2| s1.bitcoin_address.cmp(&s2.bitcoin_address));

        assert_eq!(local_swaps, remote_swaps);
        assert_eq!(local_swaps.len(), 2);

        local_storage.set_last_sync_version(10, &vec![])?;
        let version = local_storage.get_last_sync_version()?.unwrap();
        assert_eq!(version, 10);

        Ok(())
    }

    #[tokio::test]
    async fn test_sync_swaps_update_swap_fees() -> Result<()> {
        let local_storage = SqliteStorage::new(
            test_utils::create_test_sql_dir(),
            Arc::new(Box::new(NopLogger {})),
        );
        local_storage.init()?;

        // Swap is created with initial dynamic fee
        let local_swap_info = create_test_swap_info();
        local_storage.insert_swap(local_swap_info.clone())?;

        // Sleep to cause a change in created_at
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Swap address is re-used later with different (newer) dynamic fee
        let new_fees: crate::OpeningFeeParams = get_test_ofp_48h(10, 10).into();
        local_storage.update_swap_fees(local_swap_info.bitcoin_address, new_fees.clone())?;

        let local_swaps = local_storage.list_swaps()?;
        assert_eq!(local_swaps.len(), 1);
        assert_eq!(
            local_swaps
                .first()
                .ok_or_else(|| anyhow!("No element found"))?
                .channel_opening_fees,
            Some(new_fees)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_sync_swaps_fees_local_vs_remote() -> Result<()> {
        let local_storage = SqliteStorage::new(
            test_utils::create_test_sql_dir(),
            Arc::new(Box::new(NopLogger {})),
        );
        local_storage.init()?;

        let remote_storage = SqliteStorage::new(
            test_utils::create_test_sql_dir(),
            Arc::new(Box::new(NopLogger {})),
        );
        remote_storage.init()?;

        // Created locally: Swaps L1, L2, L3
        let pre_sync_l1 = create_test_swap_info();
        let pre_sync_l2 = create_test_swap_info();
        let pre_sync_l3 = create_test_swap_info();

        // Create remote swaps R1 and R2 (clones of L1 and L2, but with different fees and created_at)
        let mut pre_sync_r1 = pre_sync_l1.clone();
        pre_sync_r1.channel_opening_fees = Some(get_test_ofp_48h(10, 10).into());
        let mut pre_sync_r2 = pre_sync_l2.clone();
        pre_sync_r2.channel_opening_fees = Some(get_test_ofp_48h(15, 15).into());

        // Swaps are inserted 1 second apart from each other, because the swaps_fees rows include a local timestamp on insertion
        // Swaps are inserted in this order:
        // - Remote swap R1
        // - Local swap L1  (created_at +1s)
        // - Local swap L2  (created_at +1s)
        // - Local swap L3  (created_at +1s)
        // - Remote swap R2 (created_at +1s)
        remote_storage.insert_swap(pre_sync_r1.clone())?; // R1
        tokio::time::sleep(Duration::from_secs(1)).await;
        local_storage.insert_swap(pre_sync_l1.clone())?; // L1
        tokio::time::sleep(Duration::from_secs(1)).await;
        local_storage.insert_swap(pre_sync_l2.clone())?; // L2
        tokio::time::sleep(Duration::from_secs(1)).await;
        local_storage.insert_swap(pre_sync_l3.clone())?; // L3
        tokio::time::sleep(Duration::from_secs(1)).await;
        remote_storage.insert_swap(pre_sync_r2.clone())?; // R2

        // The swap fees created_at are in this order: R1 < L1 < L2 < L3 < R2

        // As a result:
        // - R1 fee created_at < L1 fee created_at => sync should NOT replace the local version
        // - R2 fee created_at > L2 fee created_at => sync should replace the local version
        // - R3 should be created (mirror of L3) because it doesn't exist on remote

        let local_swaps = local_storage.list_swaps()?;
        let remote_swaps = remote_storage.list_swaps()?;
        assert_eq!(local_swaps.len(), 3);
        assert_eq!(remote_swaps.len(), 2); // Before the sync, only 2 swaps in remote

        // Update local DB based on remote (sync)
        remote_storage.import_remote_changes(&local_storage, false)?;
        local_storage.import_remote_changes(&remote_storage, true)?;

        let local_swaps = local_storage.list_swaps()?;
        let remote_swaps = remote_storage.list_swaps()?;
        assert_eq!(local_swaps.len(), 3);
        assert_eq!(remote_swaps.len(), 3); // After the sync, all 3 swaps in remote

        let post_sync_l1 = local_swaps
            .iter()
            .find(|s| s.bitcoin_address == pre_sync_l1.bitcoin_address)
            .ok_or_else(|| anyhow!("L1 swaps_fees row lost from local DB after sync"))?;
        let post_sync_l2 = local_swaps
            .iter()
            .find(|s| s.bitcoin_address == pre_sync_l2.bitcoin_address)
            .ok_or_else(|| anyhow!("L2 swaps_fees row lost from local DB after sync"))?;
        let post_sync_l3 = local_swaps
            .iter()
            .find(|s| s.bitcoin_address == pre_sync_l3.bitcoin_address)
            .ok_or_else(|| anyhow!("L3 swaps_fees row lost from local DB after sync"))?;
        let post_sync_r1 = remote_swaps
            .iter()
            .find(|s| s.bitcoin_address == pre_sync_r1.bitcoin_address)
            .ok_or_else(|| anyhow!("R1 swaps_fees row lost from remote DB after sync"))?;
        let post_sync_r2 = remote_swaps
            .iter()
            .find(|s| s.bitcoin_address == pre_sync_r2.bitcoin_address)
            .ok_or_else(|| anyhow!("R2 swaps_fees row lost from remote DB after sync"))?;
        let post_sync_r3 = remote_swaps
            .iter()
            .find(|s| s.bitcoin_address == pre_sync_l3.bitcoin_address)
            .ok_or_else(|| anyhow!("No R3 swap info found in remote DB after sync"))?;

        // L1 fees were NOT updated to R1 fees (R1 fees created_at < L1 fees created_at)
        assert_ne!(post_sync_l1, &pre_sync_r1);
        // L1 fees remain as they were before the sync
        assert_eq!(post_sync_l1, &pre_sync_l1);
        // L1 and R1 are in sync (L1 overwrote R1)
        assert_eq!(post_sync_l1, post_sync_r1);
        assert_ne!(post_sync_r1, &pre_sync_r1);

        // L2 fees were replaced by the R2 fees (R2 fees created_at > L2 fees created_at)
        assert_eq!(post_sync_l2, post_sync_r2);
        assert_ne!(post_sync_l2, &pre_sync_l2);

        // L3 and R3 are in sync (there was no R3 before the sync, now R3 = L3)
        assert_eq!(post_sync_l3, post_sync_r3);
        assert_eq!(post_sync_l3, &pre_sync_l3);

        Ok(())
    }

    fn create_test_swap_info() -> SwapInfo {
        SwapInfo {
            bitcoin_address: rand_string(10),
            created_at: 10,
            lock_height: random(),
            payment_hash: rand_vec_u8(10),
            preimage: rand_vec_u8(10),
            private_key: rand_vec_u8(10),
            public_key: rand_vec_u8(10),
            swapper_public_key: rand_vec_u8(10),
            script: rand_vec_u8(10),
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
            channel_opening_fees: Some(get_test_ofp_48h(random(), random()).into()),
        }
    }
}
