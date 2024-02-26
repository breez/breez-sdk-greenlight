use super::{db::SqliteStorage, error::PersistResult};
use crate::{FullReverseSwapInfo, ReverseSwapInfoCached, ReverseSwapStatus};
use rusqlite::{named_params, OptionalExtension, Params, Row, TransactionBehavior};

impl SqliteStorage {
    pub(crate) fn insert_reverse_swap(&self, rsi: &FullReverseSwapInfo) -> PersistResult<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        tx.execute(
            "INSERT INTO sync.reverse_swaps (id, created_at_block_height, preimage, private_key, claim_pubkey, timeout_block_height, invoice, onchain_amount_sat, sat_per_vbyte, redeem_script)\
            VALUES (:id, :created_at_block_height, :preimage, :private_key, :claim_pubkey, :timeout_block_height, :invoice, :onchain_amount_sat, :sat_per_vbyte, :redeem_script)",
            named_params! {
                ":id": rsi.id,
                ":created_at_block_height": rsi.created_at_block_height,
                ":preimage": rsi.preimage,
                ":private_key": rsi.private_key,
                ":claim_pubkey": rsi.claim_pubkey,
                ":timeout_block_height": rsi.timeout_block_height,
                ":invoice": rsi.invoice,
                ":onchain_amount_sat": rsi.onchain_amount_sat,
                ":sat_per_vbyte": rsi.sat_per_vbyte,
                ":redeem_script": rsi.redeem_script
            },
        )?;

        tx.execute(
            "INSERT INTO reverse_swaps_info (id, status)\
            VALUES (:id, :status)",
            named_params! {
                ":id": rsi.id,
                ":status": serde_json::to_value(rsi.cache.status)?
            },
        )?;

        tx.commit()?;
        Ok(())
    }

    pub(crate) fn update_reverse_swap_status(
        &self,
        id: &str,
        status: &ReverseSwapStatus,
    ) -> PersistResult<()> {
        debug!("Persisting new status for reverse swap {id} to be {status:?}");

        self.get_connection()?.execute(
            "INSERT OR REPLACE INTO reverse_swaps_info(id, status) VALUES(:id, :status)",
            named_params! {
             ":status": serde_json::to_value(status)?,
             ":id": id,
            },
        )?;

        Ok(())
    }

    pub(crate) fn update_reverse_swap_lockup_txid(
        &self,
        id: &str,
        lockup_txid: Option<String>,
    ) -> PersistResult<()> {
        debug!("Updating lockup_txid for reverse swap {id} to be - lockup_txid: {lockup_txid:?}");

        self.get_connection()?.execute(
            "UPDATE reverse_swaps_info SET lockup_txid = :lockup_txid WHERE id = :id",
            named_params! {
             ":id": id,
             ":lockup_txid": lockup_txid,
            },
        )?;

        Ok(())
    }

    pub(crate) fn update_reverse_swap_claim_txid(
        &self,
        id: &str,
        claim_txid: Option<String>,
    ) -> PersistResult<()> {
        debug!("Updating claim_txid for reverse swap {id} to be - claim_txid: {claim_txid:?}");

        self.get_connection()?.execute(
            "UPDATE reverse_swaps_info SET claim_txid = :claim_txid WHERE id = :id",
            named_params! {
             ":id": id,
             ":claim_txid": claim_txid,
            },
        )?;

        Ok(())
    }

    pub(crate) fn list_reverse_swaps(&self) -> PersistResult<Vec<FullReverseSwapInfo>> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare(&self.select_reverse_swap_query("true"))?;

        let vec: Vec<FullReverseSwapInfo> = stmt
            .query_map([], |row| self.sql_row_to_reverse_swap(row))?
            .map(|i| i.unwrap())
            .collect();

        Ok(vec)
    }

    pub(crate) fn get_reverse_swap_by_preimage(
        &self,
        preimage: &Vec<u8>,
    ) -> PersistResult<Option<FullReverseSwapInfo>> {
        self.select_single_reverse_swap("preimage = ?1", [preimage])
    }

    fn select_single_reverse_swap<P>(
        &self,
        where_clause: &str,
        params: P,
    ) -> PersistResult<Option<FullReverseSwapInfo>>
    where
        P: Params,
    {
        Ok(self
            .get_connection()?
            .query_row(
                &self.select_reverse_swap_query(where_clause),
                params,
                |row| self.sql_row_to_reverse_swap(row),
            )
            .optional()?)
    }

    fn sql_row_to_reverse_swap(
        &self,
        row: &Row,
    ) -> PersistResult<FullReverseSwapInfo, rusqlite::Error> {
        Ok(FullReverseSwapInfo {
            id: row.get("id")?,
            created_at_block_height: row.get("created_at_block_height")?,
            preimage: row.get("preimage")?,
            private_key: row.get("private_key")?,
            timeout_block_height: row.get("timeout_block_height")?,
            claim_pubkey: row.get("claim_pubkey")?,
            invoice: row.get("invoice")?,
            onchain_amount_sat: row.get("onchain_amount_sat")?,
            sat_per_vbyte: row.get("sat_per_vbyte")?,
            redeem_script: row.get("redeem_script")?,
            cache: ReverseSwapInfoCached {
                // The status is stored in the main DB, which is empty when the node is restored.
                // We therefore default to the Initial state. This will be updated at the end of sync().
                status: serde_json::from_value(row.get("status")?)
                    .unwrap_or(ReverseSwapStatus::Initial),
                lockup_txid: row.get("lockup_txid")?,
                claim_txid: row.get("claim_txid")?,
            },
        })
    }

    fn select_reverse_swap_query(&self, where_clause: &str) -> String {
        format!(
            "
            SELECT
             *
            FROM sync.reverse_swaps
             LEFT JOIN reverse_swaps_info ON reverse_swaps.id = reverse_swaps_info.id
            WHERE {}
            ",
            where_clause
        )
    }
}
