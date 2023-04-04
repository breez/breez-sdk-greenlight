use super::db::SqliteStorage;
use crate::boltzswap::BoltzApiReverseSwapStatus;
use crate::{ReverseSwapInfo, ReverseSwapInfoCached, ReverseSwapStatus};
use anyhow::Result;
use rusqlite::types::FromSqlError;
use rusqlite::{named_params, Row};

impl SqliteStorage {
    pub(crate) fn insert_reverse_swap(&self, rsi: &ReverseSwapInfo) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction()?;

        tx.execute(
            "INSERT INTO reverse_swaps (id, created_at, local_preimage, local_private_key, destination_address, timeout_block_height, hodl_bolt11, onchain_amount_sat, redeem_script)\
            VALUES (:id, :created_at, :local_preimage, :local_private_key, :destination_address, :timeout_block_height, :hodl_bolt11, :onchain_amount_sat, :redeem_script)",
            named_params! {
                ":id": rsi.id,
                ":created_at": rsi.created_at,
                ":local_preimage": rsi.local_preimage,
                ":local_private_key": rsi.local_private_key,
                ":destination_address": rsi.destination_address,
                ":timeout_block_height": rsi.timeout_block_height,
                ":hodl_bolt11": rsi.hodl_bolt11,
                ":onchain_amount_sat": rsi.onchain_amount_sat,
                ":redeem_script": rsi.redeem_script
            },
        )?;

        tx.execute(
            "INSERT INTO reverse_swaps_info (id, boltz_api_status, breez_status)\
            VALUES (:id, :boltz_api_status, :breez_status)",
            named_params! {
                ":id": rsi.id,
                ":boltz_api_status": serde_json::to_value(rsi.cache.boltz_api_status.clone())?,
                ":breez_status": serde_json::to_value(rsi.cache.breez_status)?
            },
        )?;

        tx.commit()?;
        Ok(())
    }

    pub(crate) fn update_reverse_swap_boltz_status(
        &self,
        id: &str,
        boltz_api_status: &BoltzApiReverseSwapStatus,
        breez_status: &ReverseSwapStatus,
    ) -> Result<()> {
        self.get_connection()?.execute(
            "UPDATE reverse_swaps_info SET boltz_api_status=:boltz_api_status,breez_status=:breez_status where id=:id",
            named_params! {
             ":boltz_api_status": serde_json::to_value(boltz_api_status)?,
             ":breez_status": serde_json::to_value(breez_status)?,
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
            created_at: row.get("created_at")?,
            local_preimage: row.get("local_preimage")?,
            local_private_key: row.get("local_private_key")?,
            timeout_block_height: row.get("timeout_block_height")?,
            destination_address: row.get("destination_address")?,
            hodl_bolt11: row.get("hodl_bolt11")?,
            onchain_amount_sat: row.get("onchain_amount_sat")?,
            redeem_script: row.get("redeem_script")?,
            cache: ReverseSwapInfoCached {
                boltz_api_status: serde_json::from_value(row.get("boltz_api_status")?)
                    .map_err(|_| FromSqlError::InvalidType)?,
                breez_status: serde_json::from_value(row.get("breez_status")?)
                    .map_err(|_| FromSqlError::InvalidType)?,
            },
        })
    }

    fn select_reverse_swap_query(&self) -> String {
        "
            SELECT
             *
            FROM reverse_swaps
             LEFT JOIN reverse_swaps_info ON reverse_swaps.id = reverse_swaps_info.id
            "
        .to_string()
    }
}
