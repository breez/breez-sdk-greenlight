use crate::models::{SwapInfo, SwapStatus};

use super::db::{SqliteStorage, StringArray};
use crate::OpeningFeeParams;
use anyhow::{anyhow, Result};
use rusqlite::{named_params, OptionalExtension, Params, Row, Transaction};

impl SqliteStorage {
    pub(crate) fn insert_swap(&self, swap_info: SwapInfo) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction()?;

        tx.execute("
         INSERT INTO sync.swaps (
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
         )
         VALUES (:bitcoin_address, :created_at, :lock_height, :payment_hash, :preimage, :private_key, :public_key, :swapper_public_key, :script, :min_allowed_deposit, :max_allowed_deposit)",
         named_params! {
             ":bitcoin_address": swap_info.bitcoin_address,
             ":created_at": swap_info.created_at,
             ":lock_height": swap_info.lock_height,
             ":payment_hash": swap_info.payment_hash,
             ":preimage": swap_info.preimage,
             ":private_key": swap_info.private_key,
             ":public_key": swap_info.public_key,
             ":swapper_public_key": swap_info.swapper_public_key,            
             ":script": swap_info.script,             
             ":min_allowed_deposit": swap_info.min_allowed_deposit,
             ":max_allowed_deposit": swap_info.max_allowed_deposit
         },
        )?;

        tx.execute(
            "
        INSERT INTO swaps_info (
          bitcoin_address, 
          status,
          bolt11,
          paid_sats, 
          unconfirmed_sats, 
          unconfirmed_tx_ids, 
          confirmed_sats,
          confirmed_tx_ids
        ) VALUES (:bitcoin_address, :status, :bolt11, :paid_sats, :unconfirmed_sats, :unconfirmed_tx_ids, :confirmed_sats, :confirmed_tx_ids)",
            named_params! {
               ":bitcoin_address": swap_info.bitcoin_address,
               ":status": swap_info.status as i32,
               ":bolt11": None::<String>,
               ":paid_sats": swap_info.paid_sats,
               ":unconfirmed_sats": swap_info.unconfirmed_sats,
               ":unconfirmed_tx_ids": StringArray(swap_info.unconfirmed_tx_ids),
               ":confirmed_sats": swap_info.confirmed_sats,
               ":confirmed_tx_ids": StringArray(swap_info.confirmed_tx_ids),
            },
        )?;

        Self::insert_swaps_fees(
            &tx,
            swap_info.bitcoin_address,
            swap_info
                .channel_opening_fees
                .ok_or_else(|| anyhow!("Dynamic fees must be set when creating a new swap"))?,
        )?;

        tx.commit()?;
        Ok(())
    }

    pub(crate) fn update_swap_paid_amount(
        &self,
        bitcoin_address: String,
        paid_sats: u32,
    ) -> Result<()> {
        self.get_connection()?.execute(
            "UPDATE swaps_info SET paid_sats=:paid_sats where bitcoin_address=:bitcoin_address",
            named_params! {
             ":paid_sats": paid_sats,
             ":bitcoin_address": bitcoin_address,
            },
        )?;

        Ok(())
    }

    pub(crate) fn update_swap_redeem_error(
        &self,
        bitcoin_address: String,
        redeem_err: String,
    ) -> Result<()> {
        self.get_connection()?.execute(
            "UPDATE swaps_info SET last_redeem_error=:redeem_err where bitcoin_address=:bitcoin_address",
            named_params! {
             ":redeem_err": redeem_err,
             ":bitcoin_address": bitcoin_address,
            },
        )?;

        Ok(())
    }

    pub(crate) fn update_swap_bolt11(&self, bitcoin_address: String, bolt11: String) -> Result<()> {
        self.get_connection()?.execute(
            "UPDATE swaps_info SET bolt11=:bolt11 where bitcoin_address=:bitcoin_address",
            named_params! {
             ":bolt11": bolt11,
             ":bitcoin_address": bitcoin_address,
            },
        )?;

        Ok(())
    }

    fn insert_swaps_fees(
        tx: &Transaction,
        bitcoin_address: String,
        channel_opening_fees: OpeningFeeParams,
    ) -> Result<()> {
        tx.execute(
            "INSERT OR REPLACE INTO sync.swaps_fees (bitcoin_address, created_at, channel_opening_fees) VALUES(:bitcoin_address, CURRENT_TIMESTAMP, :channel_opening_fees)",
            named_params! {
             ":bitcoin_address": bitcoin_address,
             ":channel_opening_fees": channel_opening_fees,
            },
        )?;

        Ok(())
    }

    /// Update the dynamic fees associated with a swap
    pub(crate) fn update_swap_fees(
        &self,
        bitcoin_address: String,
        channel_opening_fees: OpeningFeeParams,
    ) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction()?;

        Self::insert_swaps_fees(&tx, bitcoin_address, channel_opening_fees)?;

        tx.commit()?;
        Ok(())
    }

    pub(crate) fn insert_swap_refund_tx_ids(
        &self,
        bitcoin_address: String,
        refund_tx_id: String,
    ) -> Result<()> {
        self.get_connection()?.execute(
            "INSERT INTO sync.swap_refunds (bitcoin_address, refund_tx_id) VALUES(:bitcoin_address, :refund_tx_id)",
            named_params! {
             ":bitcoin_address": bitcoin_address,
             ":refund_tx_id": refund_tx_id,
            },
        )?;

        Ok(())
    }

    pub(crate) fn update_swap_chain_info(
        &self,
        bitcoin_address: String,
        unconfirmed_sats: u64,
        unconfirmed_tx_ids: Vec<String>,
        confirmed_sats: u64,
        confirmed_tx_ids: Vec<String>,
        status: SwapStatus,
    ) -> Result<SwapInfo> {
        self.get_connection()?.execute(
            "UPDATE swaps_info SET unconfirmed_sats=:unconfirmed_sats, unconfirmed_tx_ids=:unconfirmed_tx_ids, confirmed_sats=:confirmed_sats, confirmed_tx_ids=:confirmed_tx_ids, status=:status where bitcoin_address=:bitcoin_address",
            named_params! {
             ":unconfirmed_sats": unconfirmed_sats,
             ":unconfirmed_tx_ids": StringArray(unconfirmed_tx_ids),
             ":confirmed_sats": confirmed_sats,
             ":bitcoin_address": bitcoin_address,             
             ":confirmed_tx_ids": StringArray(confirmed_tx_ids),
             ":status": status as u32
            },
        )?;
        Ok(self.get_swap_info_by_address(bitcoin_address)?.unwrap())
    }
    //(SELECT json_group_array(value) FROM json_each(json_group_array(refund_tx_id)) WHERE refund_tx_id is not null) as refund_tx_ids,
    fn select_swap_query(&self, where_clause: &str) -> String {
        format!(
            "
            SELECT
             swaps.bitcoin_address as bitcoin_address,
             swaps.created_at as created_at,
             lock_height as lock_height,
             payment_hash as payment_hash,
             preimage as preimage,
             private_key as private_key,
             public_key as public_key,
             swapper_public_key as swapper_public_key,
             script as script,
             min_allowed_deposit,
             max_allowed_deposit,
             bolt11 as bolt11,
             paid_sats as paid_sats,
             unconfirmed_sats as unconfirmed_sats,
             confirmed_sats as confirmed_sats,
             status as status,             
             (SELECT json_group_array(refund_tx_id) FROM sync.swap_refunds as swap_refunds where bitcoin_address = swaps.bitcoin_address) as refund_tx_ids,
             unconfirmed_tx_ids as unconfirmed_tx_ids,
             confirmed_tx_ids as confirmed_tx_ids,
             last_redeem_error as last_redeem_error,
             swaps_fees.channel_opening_fees as channel_opening_fees
            FROM sync.swaps as swaps
             LEFT JOIN swaps_info ON swaps.bitcoin_address = swaps_info.bitcoin_address
             LEFT JOIN sync.swaps_fees as swaps_fees ON swaps.bitcoin_address = swaps_fees.bitcoin_address
             LEFT JOIN sync.swap_refunds as swap_refunds ON swaps.bitcoin_address = swap_refunds.bitcoin_address
            WHERE {}
            ",
            where_clause
        )
    }

    fn select_single_swap<P>(&self, where_clause: &str, params: P) -> Result<Option<SwapInfo>>
    where
        P: Params,
    {
        self.get_connection()?
            .query_row(&self.select_swap_query(where_clause), params, |row| {
                self.sql_row_to_swap(row)
            })
            .optional()
            .map_err(|e| anyhow!(e))
    }

    pub(crate) fn get_swap_info_by_hash(&self, hash: &Vec<u8>) -> Result<Option<SwapInfo>> {
        self.select_single_swap("payment_hash = ?1", [hash])
    }

    pub(crate) fn get_swap_info_by_address(&self, address: String) -> Result<Option<SwapInfo>> {
        self.select_single_swap("swaps.bitcoin_address = ?1", [address])
    }

    pub(crate) fn list_swaps_with_status(&self, status: SwapStatus) -> Result<Vec<SwapInfo>> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare(&self.select_swap_query("status = ?1"))?;

        let vec: Vec<SwapInfo> = stmt
            .query_map([status as u32], |row| self.sql_row_to_swap(row))?
            .map(|i| i.unwrap())
            .collect();

        Ok(vec)
    }

    pub(crate) fn list_swaps(&self) -> Result<Vec<SwapInfo>> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare(&self.select_swap_query("true"))?;

        let vec: Vec<SwapInfo> = stmt
            .query_map([], |row| self.sql_row_to_swap(row))?
            .map(|i| i.unwrap())
            .collect();

        Ok(vec)
    }

    fn sql_row_to_swap(&self, row: &Row) -> Result<SwapInfo, rusqlite::Error> {
        let status: i32 = row
            .get::<&str, Option<i32>>("status")?
            .unwrap_or(SwapStatus::Initial as i32);
        let status: SwapStatus = status.try_into().map_or(SwapStatus::Initial, |v| v);
        let refund_txs_raw: String = row
            .get::<&str, Option<String>>("refund_tx_ids")?
            .unwrap_or("[]".to_string());
        let refund_tx_ids: Vec<String> = serde_json::from_str(refund_txs_raw.as_str()).unwrap();
        // let t: Vec<String> =
        //     serde_json::from_value(refund_txs_raw).map_err(|e| FromSqlError::InvalidType)?;

        let unconfirmed_tx_ids: StringArray = row
            .get::<&str, Option<StringArray>>("unconfirmed_tx_ids")?
            .unwrap_or(StringArray(vec![]));
        let confirmed_txs_raw: StringArray = row
            .get::<&str, Option<StringArray>>("confirmed_tx_ids")?
            .unwrap_or(StringArray(vec![]));
        Ok(SwapInfo {
            bitcoin_address: row.get("bitcoin_address")?,
            created_at: row.get("created_at")?,
            lock_height: row.get("lock_height")?,
            payment_hash: row.get("payment_hash")?,
            preimage: row.get("preimage")?,
            private_key: row.get("private_key")?,
            public_key: row.get("public_key")?,
            swapper_public_key: row.get("swapper_public_key")?,
            script: row.get("script")?,
            bolt11: row.get("bolt11")?,
            paid_sats: row
                .get::<&str, Option<u64>>("paid_sats")?
                .unwrap_or_default(),
            unconfirmed_sats: row
                .get::<&str, Option<u64>>("unconfirmed_sats")?
                .unwrap_or_default(),
            confirmed_sats: row
                .get::<&str, Option<u64>>("confirmed_sats")?
                .unwrap_or_default(),
            status,
            refund_tx_ids,
            unconfirmed_tx_ids: unconfirmed_tx_ids.0,
            confirmed_tx_ids: confirmed_txs_raw.0,
            min_allowed_deposit: row.get("min_allowed_deposit")?,
            max_allowed_deposit: row.get("max_allowed_deposit")?,
            last_redeem_error: row.get("last_redeem_error")?,
            channel_opening_fees: row.get("channel_opening_fees")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::persist::db::SqliteStorage;
    use crate::test_utils::get_test_ofp_48h;
    use crate::NopLogger;
    use crate::{OpeningFeeParams, SwapInfo, SwapStatus};
    use anyhow::Result;
    use rusqlite::{named_params, Connection};

    #[test]
    fn test_swaps() -> Result<(), Box<dyn std::error::Error>> {
        use crate::persist::test_utils;
        fn list_in_progress_swaps(storage: &SqliteStorage) -> Result<Vec<SwapInfo>> {
            Ok(storage
                .list_swaps_with_status(SwapStatus::Initial)?
                .into_iter()
                .filter(SwapInfo::in_progress)
                .collect())
        }

        let storage = SqliteStorage::new(
            test_utils::create_test_sql_dir(),
            Arc::new(Box::new(NopLogger {})),
        );

        storage.init()?;
        let tested_swap_info = SwapInfo {
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
            status: SwapStatus::Initial,
            refund_tx_ids: Vec::new(),
            unconfirmed_tx_ids: Vec::new(),
            confirmed_tx_ids: Vec::new(),
            min_allowed_deposit: 0,
            max_allowed_deposit: 100,
            last_redeem_error: None,
            channel_opening_fees: Some(get_test_ofp_48h(1, 1).into()),
        };
        storage.insert_swap(tested_swap_info.clone())?;
        let item_value = storage.get_swap_info_by_address("1".to_string())?.unwrap();
        assert_eq!(item_value, tested_swap_info);

        let in_progress = list_in_progress_swaps(&storage)?;
        assert_eq!(in_progress.len(), 0);

        let non_existent_swap = storage.get_swap_info_by_address("non-existent".to_string())?;
        assert!(non_existent_swap.is_none());

        let empty_swaps = storage.list_swaps_with_status(SwapStatus::Expired)?;
        assert_eq!(empty_swaps.len(), 0);

        let swaps = storage.list_swaps_with_status(SwapStatus::Initial)?;
        assert_eq!(swaps.len(), 1);

        let err = storage.insert_swap(tested_swap_info.clone());
        //assert_eq!(swaps.len(), 1);
        assert!(err.is_err());

        let swap_after_chain_update = storage.update_swap_chain_info(
            tested_swap_info.bitcoin_address.clone(),
            20,
            vec![String::from("333"), String::from("444")],
            0,
            vec![],
            SwapStatus::Initial,
        )?;
        let in_progress = list_in_progress_swaps(&storage)?;
        assert_eq!(in_progress[0], swap_after_chain_update);

        let swap_after_chain_update = storage.update_swap_chain_info(
            tested_swap_info.bitcoin_address.clone(),
            0,
            vec![],
            20,
            vec![String::from("333"), String::from("444")],
            SwapStatus::Initial,
        )?;
        let in_progress = list_in_progress_swaps(&storage)?;
        assert_eq!(in_progress[0], swap_after_chain_update);

        storage.update_swap_chain_info(
            tested_swap_info.bitcoin_address.clone(),
            0,
            vec![],
            20,
            vec![String::from("333"), String::from("444")],
            SwapStatus::Expired,
        )?;
        storage.insert_swap_refund_tx_ids(
            tested_swap_info.bitcoin_address.clone(),
            String::from("111"),
        )?;
        storage.insert_swap_refund_tx_ids(
            tested_swap_info.bitcoin_address.clone(),
            String::from("222"),
        )?;
        let in_progress = list_in_progress_swaps(&storage)?;
        assert_eq!(in_progress.len(), 0);

        storage.update_swap_redeem_error(
            tested_swap_info.bitcoin_address.clone(),
            String::from("test error"),
        )?;
        let updated_swap = storage
            .get_swap_info_by_address(tested_swap_info.bitcoin_address.clone())?
            .unwrap();
        assert_eq!(
            updated_swap.last_redeem_error.unwrap(),
            String::from("test error")
        );

        storage.update_swap_bolt11(tested_swap_info.bitcoin_address.clone(), "bolt11".into())?;
        storage.update_swap_paid_amount(tested_swap_info.bitcoin_address.clone(), 30)?;
        let updated_swap = storage
            .get_swap_info_by_address(tested_swap_info.bitcoin_address)?
            .unwrap();
        assert_eq!(updated_swap.bolt11.unwrap(), "bolt11".to_string());
        assert_eq!(updated_swap.paid_sats, 30);
        assert_eq!(updated_swap.confirmed_sats, 20);
        assert_eq!(
            updated_swap.refund_tx_ids,
            vec![String::from("111"), String::from("222")]
        );
        assert_eq!(
            updated_swap.confirmed_tx_ids,
            vec![String::from("333"), String::from("444")]
        );
        assert_eq!(updated_swap.status, SwapStatus::Expired);

        Ok(())
    }

    #[test]
    /// Checks if an empty column is converted to None
    fn test_rusqlite_empty_col_handling() -> Result<()> {
        let db = Connection::open_in_memory()?;

        // Insert a NULL
        db.execute_batch("CREATE TABLE foo (fees_optional TEXT)")?;
        db.execute(
            "
         INSERT INTO foo ( fees_optional )
         VALUES ( NULL )",
            named_params! {},
        )?;

        // Read the column, expect None
        let res = db.query_row("SELECT fees_optional FROM foo", [], |row| {
            row.get::<usize, Option<OpeningFeeParams>>(0)
        })?;
        assert_eq!(res, None);

        Ok(())
    }
}
