use super::db::SqliteStorage;
use crate::{ReverseSwapInfo, ReverseSwapInfoCached, ReverseSwapStatus};
use anyhow::Result;
use rusqlite::types::FromSqlError;
use rusqlite::{named_params, Row};

impl SqliteStorage {
    pub(crate) fn insert_reverse_swap(&self, rsi: &ReverseSwapInfo) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction()?;

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
    ) -> Result<()> {
        debug!("Persisting new status for reverse swap {id} to be {status:?}");

        self.get_connection()?.execute(
            "UPDATE reverse_swaps_info SET status=:status where id=:id",
            named_params! {
             ":status": serde_json::to_value(status)?,
             ":id": id,
            },
        )?;

        Ok(())
    }

    pub(crate) fn list_reverse_swaps(&self) -> Result<Vec<ReverseSwapInfo>> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare(&self.select_reverse_swap_query())?;

        let vec: Vec<ReverseSwapInfo> = stmt
            .query_map([], |row| self.sql_row_to_reverse_swap(row))?
            .map(|i| i.unwrap())
            .collect();

        Ok(vec)
    }

    fn sql_row_to_reverse_swap(&self, row: &Row) -> Result<ReverseSwapInfo, rusqlite::Error> {
        Ok(ReverseSwapInfo {
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
                status: serde_json::from_value(row.get("status")?)
                    .map_err(|_| FromSqlError::InvalidType)?,
            },
        })
    }

    fn select_reverse_swap_query(&self) -> String {
        "
            SELECT
             *
            FROM sync.reverse_swaps
             LEFT JOIN reverse_swaps_info ON reverse_swaps.id = reverse_swaps_info.id
            "
        .to_string()
    }
}
