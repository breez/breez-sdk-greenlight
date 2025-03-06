use crate::ReverseSwapStatus;

use super::{db::SqliteStorage, error::PersistResult};
use rusqlite::{named_params, Row, Transaction, TransactionBehavior};
use std::path::Path;

#[allow(dead_code)]
pub(crate) struct SyncVersion {
    pub created_at: String,
    pub last_version: u64,
    pub data: Vec<u8>,
}

impl SqliteStorage {
    pub(crate) fn backup<P: AsRef<Path>>(&self, dst_path: P) -> PersistResult<()> {
        Ok(self.get_connection()?.backup(
            rusqlite::DatabaseName::Attached("sync"),
            dst_path,
            None,
        )?)
    }

    pub(crate) fn get_last_sync_version(&self) -> PersistResult<Option<u64>> {
        Ok(self.get_connection()?.query_row(
            "SELECT max(last_version) FROM sync_versions",
            [],
            |row| row.get::<usize, Option<u64>>(0),
        )?)
    }

    pub(crate) fn set_last_sync_version(
        &self,
        last_version: u64,
        data: &Vec<u8>,
    ) -> PersistResult<()> {
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
    pub(crate) fn sync_versions_history(&self) -> PersistResult<Vec<SyncVersion>> {
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

    fn sql_row_to_sync_version(&self, row: &Row) -> PersistResult<SyncVersion, rusqlite::Error> {
        let version = SyncVersion {
            created_at: row.get(0)?,
            last_version: row.get(1)?,
            data: row.get(2)?,
        };

        Ok(version)
    }

    pub fn get_last_sync_request(&self) -> PersistResult<Option<u64>> {
        let res: rusqlite::Result<Option<u64>> =
            self.get_connection()?
                .query_row("SELECT max(id) FROM sync.sync_requests", [], |row| {
                    row.get::<usize, Option<u64>>(0)
                });
        Ok(res?)
    }

    pub(crate) fn delete_sync_requests_up_to(&self, request_id: u64) -> PersistResult<()> {
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
    ) -> PersistResult<()> {
        let sync_data_file = remote_storage.sync_db_path();
        match SqliteStorage::migrate_sync_db(sync_data_file.clone()) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to migrate sync db, probably local db is older than remote, skipping migration: {}", e);
            }
        }

        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;
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
           max_allowed_deposit,
           max_swapper_payable
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
              lnurl_withdraw_endpoint,
              attempted_amount_msat,
              attempted_error,
              lnurl_pay_domain,
              lnurl_pay_comment
             FROM remote_sync.payments_external_info
             WHERE payment_id NOT IN (SELECT payment_id FROM sync.payments_external_info);",
            [],
        )?;

        // sync remote payments_metadata table
        tx.execute(
            "
             INSERT OR REPLACE INTO sync.payments_metadata
             SELECT
              remote_sync.payments_metadata.payment_id,
              remote_sync.payments_metadata.metadata,
              remote_sync.payments_metadata.updated_at
             FROM remote_sync.payments_metadata
             LEFT JOIN sync.payments_metadata 
             ON sync.payments_metadata.payment_id = remote_sync.payments_metadata.payment_id
             WHERE
              remote_sync.payments_metadata.updated_at > sync.payments_metadata.updated_at;",
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
         receive_amount_sat,
         redeem_script
        FROM remote_sync.reverse_swaps
        WHERE id NOT IN (SELECT id FROM sync.reverse_swaps);",
            [],
        )?;

        // sync remote swap_refunds table
        tx.execute(
            "
        INSERT OR REPLACE INTO sync.open_channel_payment_info
         SELECT
          payment_hash,
          payer_amount_msat,
          open_channel_bolt11
         FROM remote_sync.open_channel_payment_info
         WHERE payment_hash NOT IN (SELECT payment_hash FROM sync.open_channel_payment_info);",
            [],
        )?;

        // Sync remote swaps_fees table, which contains dynamic fees used in swaps
        // created_at is used to settle conflicts, since we assume small variations in the client local times
        Self::sync_swaps_fees_local(&tx)?;

        // Sync taproot swaps. This data is static/insert only.
        // TODO: What about the opening_fee_params? They could be updated in either the remote or local db.
        tx.execute(
            "
            INSERT OR IGNORE INTO sync.taproot_swaps
            SELECT address
            ,      claim_public_key
            ,      created_at
            ,      lock_time
            ,      payment_hash
            ,      preimage
            ,      refund_private_key
            ,      accepted_opening_fee_params
            FROM remote_sync.taproot_swaps;",
            [],
        )?;

        // Sync taproot swap refund transactions. As refund transactions cannot be distinguished from regular transactions onchain.
        tx.execute(
            "
            INSERT OR IGNORE INTO sync.taproot_swap_refunds
            SELECT refund_tx_id
            ,      spent_tx_id
            ,      spent_output_index
            FROM remote_sync.taproot_swap_refunds;",
            [],
        )?;

        tx.commit()?;
        con.execute("DETACH DATABASE remote_sync", [])?;

        Ok(())
    }

    /// Insert or update to local db all rows that have created_at larger than in the local
    fn sync_swaps_fees_local(tx: &Transaction) -> PersistResult<()> {
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
    use crate::persist::db::SqliteStorage;
    use crate::persist::error::PersistResult;
    use crate::persist::test_utils;

    #[test]
    fn test_sync() -> PersistResult<()> {
        let local_storage = SqliteStorage::new(test_utils::create_test_sql_dir());
        local_storage.init()?;

        let remote_storage = SqliteStorage::new(test_utils::create_test_sql_dir());
        remote_storage.init()?;
        remote_storage.insert_open_channel_payment_info("123", 100000, "")?;

        remote_storage.import_remote_changes(&local_storage, false)?;
        local_storage.import_remote_changes(&remote_storage, true)?;
        local_storage.set_last_sync_version(10, &vec![])?;
        let version = local_storage.get_last_sync_version()?.unwrap();
        assert_eq!(version, 10);

        Ok(())
    }
}
