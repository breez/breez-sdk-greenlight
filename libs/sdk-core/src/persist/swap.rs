use rusqlite::{named_params, OptionalExtension, Params, Row, Transaction, TransactionBehavior};

use crate::{
    models::{OpeningFeeParams, SwapInfo, SwapStatus},
    swap_in::{SwapChainData, SwapChainInfo},
    ListSwapsRequest,
};

use super::{
    db::{SqliteStorage, StringArray},
    error::{PersistError, PersistResult},
};

#[cfg_attr(test, mockall::automock)]
pub(crate) trait SwapStorage: Send + Sync {
    fn get_swap_chain_data(&self, bitcoin_address: &str) -> PersistResult<Option<SwapChainData>>;
    fn set_swap_chain_data(
        &self,
        bitcoin_address: &str,
        chain_data: &SwapChainData,
        chain_info: &SwapChainInfo,
    ) -> PersistResult<()>;
    fn set_swap_status(&self, address: &str, status: &SwapStatus) -> PersistResult<()>;
    fn insert_swap(&self, swap_info: &SwapInfo) -> PersistResult<()>;
    fn update_swap_paid_amount(&self, bitcoin_address: &str, paid_msat: u64) -> PersistResult<()>;
    fn update_swap_max_allowed_deposit(
        &self,
        bitcoin_address: &str,
        max_allowed_deposit: i64,
    ) -> PersistResult<()>;
    fn update_swap_redeem_error(
        &self,
        bitcoin_address: String,
        redeem_err: String,
    ) -> PersistResult<()>;
    fn update_swap_bolt11(&self, bitcoin_address: String, bolt11: String) -> PersistResult<()>;
    fn update_swap_fees(
        &self,
        bitcoin_address: &str,
        channel_opening_fees: &OpeningFeeParams,
    ) -> PersistResult<()>;
    fn insert_swap_refund_tx_ids(
        &self,
        bitcoin_address: String,
        refund_tx_id: String,
    ) -> PersistResult<()>;
    fn get_swap_info_by_hash(&self, hash: &[u8]) -> PersistResult<Option<SwapInfo>>;
    fn get_swap_info_by_address(&self, address: &str) -> PersistResult<Option<SwapInfo>>;
    fn list_swaps(&self, req: ListSwapsRequest) -> PersistResult<Vec<SwapInfo>>;
}

impl SwapStorage for SqliteStorage {
    fn get_swap_chain_data(&self, bitcoin_address: &str) -> PersistResult<Option<SwapChainData>> {
        let con = self.get_connection()?;
        let mut stmt = con
            .prepare("SELECT chain_data FROM swaps_info WHERE bitcoin_address=:bitcoin_address")?;
        let rows: Vec<Option<String>> = stmt
            .query_map(
                named_params! {
                    ":bitcoin_address": bitcoin_address,
                },
                |row| row.get("chain_data"),
            )?
            .collect::<Result<_, _>>()?;

        let row = match rows.first() {
            Some(row) => row,
            None => return Ok(None),
        };

        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };

        Ok(serde_json::from_str(row)?)
    }

    fn set_swap_chain_data(
        &self,
        bitcoin_address: &str,
        chain_data: &SwapChainData,
        chain_info: &SwapChainInfo,
    ) -> PersistResult<()> {
        let chain_data = serde_json::to_string(chain_data)?;
        let con = self.get_connection()?;
        con.execute(
            "UPDATE swaps_info 
            SET total_incoming_txs=:total_incoming_txs
            ,   unconfirmed_sats=:unconfirmed_sats
            ,   unconfirmed_tx_ids=:unconfirmed_tx_ids
            ,   confirmed_sats=:confirmed_sats
            ,   confirmed_tx_ids=:confirmed_tx_ids
            ,   confirmed_at=:confirmed_at
            ,   chain_data=:chain_data
            WHERE bitcoin_address=:bitcoin_address",
            named_params! {
             ":unconfirmed_sats": chain_info.unconfirmed_sats,
             ":unconfirmed_tx_ids": StringArray(chain_info.unconfirmed_tx_ids.clone()),
             ":confirmed_sats": chain_info.confirmed_sats,
             ":bitcoin_address": bitcoin_address,
             ":confirmed_tx_ids": StringArray(chain_info.confirmed_tx_ids.clone()),
             ":confirmed_at": chain_info.confirmed_at,
             ":total_incoming_txs": chain_info.total_incoming_txs,
             ":chain_data": chain_data,
            },
        )?;
        Ok(())
    }

    fn set_swap_status(&self, address: &str, status: &SwapStatus) -> PersistResult<()> {
        let con = self.get_connection()?;
        con.execute(
            "UPDATE swaps_info SET status=:status WHERE bitcoin_address = :bitcoin_address",
            named_params! {
                ":status": *status as i32,
                ":bitcoin_address": address,
            },
        )?;
        Ok(())
    }

    fn insert_swap(&self, swap_info: &SwapInfo) -> PersistResult<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

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
           max_allowed_deposit,
           max_swapper_payable
         )
         VALUES (:bitcoin_address, :created_at, :lock_height, :payment_hash, :preimage, :private_key, :public_key, :swapper_public_key, :script, :min_allowed_deposit, :max_allowed_deposit, :max_swapper_payable)",
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
             ":max_allowed_deposit": swap_info.max_allowed_deposit,
             ":max_swapper_payable": swap_info.max_swapper_payable,
         },
        )?;

        tx.execute(
            "
        INSERT INTO swaps_info (
          bitcoin_address, 
          status,
          bolt11,
          paid_msat, 
          unconfirmed_sats, 
          unconfirmed_tx_ids, 
          confirmed_sats,
          confirmed_tx_ids,
          confirmed_at,
          total_incoming_txs
        ) VALUES (:bitcoin_address, :status, :bolt11, :paid_msat, :unconfirmed_sats, :unconfirmed_tx_ids, :confirmed_sats, :confirmed_tx_ids, :confirmed_at, :total_incoming_txs)",
            named_params! {
               ":bitcoin_address": swap_info.bitcoin_address,
               ":status": swap_info.status as i32,
               ":bolt11": None::<String>,
               ":paid_msat": swap_info.paid_msat,
               ":unconfirmed_sats": swap_info.unconfirmed_sats,
               ":unconfirmed_tx_ids": StringArray(swap_info.unconfirmed_tx_ids.clone()),
               ":confirmed_sats": swap_info.confirmed_sats,
               ":confirmed_tx_ids": StringArray(swap_info.confirmed_tx_ids.clone()),
               ":confirmed_at": swap_info.confirmed_at,
               ":total_incoming_txs": swap_info.total_incoming_txs,
            },
        )?;

        Self::insert_swaps_fees(
            &tx,
            &swap_info.bitcoin_address,
            swap_info.channel_opening_fees.as_ref().ok_or_else(|| {
                PersistError::generic("Dynamic fees must be set when creating a new swap")
            })?,
        )?;

        tx.commit()?;
        Ok(())
    }

    fn update_swap_paid_amount(&self, bitcoin_address: &str, paid_msat: u64) -> PersistResult<()> {
        self.get_connection()?.execute(
            "UPDATE swaps_info SET paid_msat=:paid_msat where bitcoin_address=:bitcoin_address",
            named_params! {
             ":paid_msat": paid_msat,
             ":bitcoin_address": bitcoin_address,
            },
        )?;
        Ok(())
    }

    fn update_swap_max_allowed_deposit(
        &self,
        bitcoin_address: &str,
        max_allowed_deposit: i64,
    ) -> PersistResult<()> {
        self.get_connection()?.execute(
            "UPDATE sync.swaps SET max_allowed_deposit=:max_allowed_deposit where bitcoin_address=:bitcoin_address",
            named_params! {
             ":max_allowed_deposit": max_allowed_deposit,
             ":bitcoin_address": bitcoin_address,
            },
        )?;

        Ok(())
    }

    fn update_swap_redeem_error(
        &self,
        bitcoin_address: String,
        redeem_err: String,
    ) -> PersistResult<()> {
        self.get_connection()?.execute(
            "UPDATE swaps_info SET last_redeem_error=:redeem_err where bitcoin_address=:bitcoin_address",
            named_params! {
             ":redeem_err": redeem_err,
             ":bitcoin_address": bitcoin_address,
            },
        )?;

        Ok(())
    }

    fn update_swap_bolt11(&self, bitcoin_address: String, bolt11: String) -> PersistResult<()> {
        self.get_connection()?.execute(
            "UPDATE swaps_info SET bolt11=:bolt11 where bitcoin_address=:bitcoin_address",
            named_params! {
             ":bolt11": bolt11,
             ":bitcoin_address": bitcoin_address,
            },
        )?;

        Ok(())
    }

    /// Update the dynamic fees associated with a swap
    fn update_swap_fees(
        &self,
        bitcoin_address: &str,
        channel_opening_fees: &OpeningFeeParams,
    ) -> PersistResult<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        Self::insert_swaps_fees(&tx, bitcoin_address, channel_opening_fees)?;

        tx.commit()?;
        Ok(())
    }

    fn insert_swap_refund_tx_ids(
        &self,
        bitcoin_address: String,
        refund_tx_id: String,
    ) -> PersistResult<()> {
        self.get_connection()?.execute(
            "INSERT OR IGNORE INTO sync.swap_refunds (bitcoin_address, refund_tx_id) VALUES(:bitcoin_address, :refund_tx_id)",
            named_params! {
             ":bitcoin_address": bitcoin_address,
             ":refund_tx_id": refund_tx_id,
            },
        )?;

        Ok(())
    }

    fn get_swap_info_by_hash(&self, hash: &[u8]) -> PersistResult<Option<SwapInfo>> {
        self.select_single_swap("payment_hash = ?1", [hash])
    }

    fn get_swap_info_by_address(&self, address: &str) -> PersistResult<Option<SwapInfo>> {
        self.select_single_swap("swaps.bitcoin_address = ?1", [address])
    }

    fn list_swaps(&self, req: ListSwapsRequest) -> PersistResult<Vec<SwapInfo>> {
        let con = self.get_connection()?;
        let mut where_clauses = Vec::new();
        if let Some(status) = req.status {
            if status.is_empty() {
                return Ok(Vec::new());
            }

            where_clauses.push(format!(
                "status in ({})",
                status
                    .into_iter()
                    .map(|s| (s as u32).to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        if let Some(from_timestamp) = req.from_timestamp {
            where_clauses.push(format!("swaps.created_at >= {from_timestamp}"));
        }

        if let Some(to_timestamp) = req.to_timestamp {
            where_clauses.push(format!("swaps.created_at < {to_timestamp}"));
        }

        let where_clause = match where_clauses.is_empty() {
            true => String::from("true"),
            false => where_clauses.join(" AND "),
        };

        let mut query = self.select_swap_query(&where_clause, "");

        match req.limit {
            Some(limit) => query.push_str(&format!("LIMIT {limit}\n")),
            None => query.push_str("LIMIT -1\n"),
        }

        if let Some(offset) = req.offset {
            query.push_str(&format!("OFFSET {offset}\n"));
        }

        let mut stmt = con.prepare(&query)?;

        let vec: Vec<SwapInfo> = stmt
            .query_map([], |row| self.sql_row_to_swap(row, ""))?
            .map(|i| i.unwrap())
            .collect();

        Ok(vec)
    }
}

impl SqliteStorage {
    fn insert_swaps_fees(
        tx: &Transaction,
        bitcoin_address: &str,
        channel_opening_fees: &OpeningFeeParams,
    ) -> PersistResult<()> {
        tx.execute(
            "INSERT OR REPLACE INTO sync.swaps_fees (bitcoin_address, created_at, channel_opening_fees) VALUES(:bitcoin_address, CURRENT_TIMESTAMP, :channel_opening_fees)",
            named_params! {
             ":bitcoin_address": bitcoin_address,
             ":channel_opening_fees": channel_opening_fees,
            },
        )?;

        Ok(())
    }

    fn select_single_swap<P>(
        &self,
        where_clause: &str,
        params: P,
    ) -> PersistResult<Option<SwapInfo>>
    where
        P: Params,
    {
        Ok(self
            .get_connection()?
            .query_row(&self.select_swap_query(where_clause, ""), params, |row| {
                self.sql_row_to_swap(row, "")
            })
            .optional()?)
    }

    pub(super) fn sql_row_to_swap(
        &self,
        row: &Row,
        prefix: &str,
    ) -> PersistResult<SwapInfo, rusqlite::Error> {
        let status: i32 = row
            .get::<&str, Option<i32>>(format!("{prefix}status").as_str())?
            .unwrap_or(SwapStatus::Initial as i32);
        let status: SwapStatus = status.try_into().unwrap_or(SwapStatus::Initial);
        let refund_txs_raw: String = row
            .get::<&str, Option<String>>(format!("{prefix}refund_tx_ids").as_str())?
            .unwrap_or("[]".to_string());
        let refund_tx_ids: Vec<String> = serde_json::from_str(refund_txs_raw.as_str()).unwrap();
        // let t: Vec<String> =
        //     serde_json::from_value(refund_txs_raw).map_err(|e| FromSqlError::InvalidType)?;

        let unconfirmed_tx_ids: StringArray = row
            .get::<&str, Option<StringArray>>(format!("{prefix}unconfirmed_tx_ids").as_str())?
            .unwrap_or(StringArray(vec![]));
        let confirmed_txs_raw: StringArray = row
            .get::<&str, Option<StringArray>>(format!("{prefix}confirmed_tx_ids").as_str())?
            .unwrap_or(StringArray(vec![]));
        let bitcoin_address = row.get(format!("{prefix}bitcoin_address").as_str())?;
        Ok(SwapInfo {
            bitcoin_address,
            created_at: row.get(format!("{prefix}created_at").as_str())?,
            lock_height: row.get(format!("{prefix}lock_height").as_str())?,
            payment_hash: row.get(format!("{prefix}payment_hash").as_str())?,
            preimage: row.get(format!("{prefix}preimage").as_str())?,
            private_key: row.get(format!("{prefix}private_key").as_str())?,
            public_key: row.get(format!("{prefix}public_key").as_str())?,
            swapper_public_key: row.get(format!("{prefix}swapper_public_key").as_str())?,
            script: row.get(format!("{prefix}script").as_str())?,
            bolt11: row.get(format!("{prefix}bolt11").as_str())?,
            paid_msat: row
                .get::<&str, Option<u64>>(format!("{prefix}paid_msat").as_str())?
                .unwrap_or_default(),
            unconfirmed_sats: row
                .get::<&str, Option<u64>>(format!("{prefix}unconfirmed_sats").as_str())?
                .unwrap_or_default(),
            confirmed_sats: row
                .get::<&str, Option<u64>>(format!("{prefix}confirmed_sats").as_str())?
                .unwrap_or_default(),
            total_incoming_txs: row
                .get::<&str, Option<u64>>(format!("{prefix}total_incoming_txs").as_str())?
                .unwrap_or_default(),
            status,
            refund_tx_ids,
            unconfirmed_tx_ids: unconfirmed_tx_ids.0,
            confirmed_tx_ids: confirmed_txs_raw.0,
            min_allowed_deposit: row.get(format!("{prefix}min_allowed_deposit").as_str())?,
            max_allowed_deposit: row.get(format!("{prefix}max_allowed_deposit").as_str())?,
            max_swapper_payable: row.get(format!("{prefix}max_swapper_payable").as_str())?,
            last_redeem_error: row.get(format!("{prefix}last_redeem_error").as_str())?,
            channel_opening_fees: row.get(format!("{prefix}channel_opening_fees").as_str())?,
            confirmed_at: row.get(format!("{prefix}confirmed_at").as_str())?,
        })
    }

    pub(super) fn select_swap_fields(&self, prefix: &str) -> String {
        format!(
            "        
          {prefix}bitcoin_address,
          {prefix}created_at,
          {prefix}lock_height,
          {prefix}payment_hash,
          {prefix}preimage,
          {prefix}private_key,
          {prefix}public_key,
          {prefix}swapper_public_key,
          {prefix}script,
          {prefix}min_allowed_deposit,
          {prefix}max_allowed_deposit,
          {prefix}max_swapper_payable,
          {prefix}bolt11,
          {prefix}paid_msat,
          {prefix}unconfirmed_sats,
          {prefix}confirmed_sats,
          {prefix}total_incoming_txs,
          {prefix}status,             
          {prefix}refund_tx_ids,
          {prefix}unconfirmed_tx_ids,
          {prefix}confirmed_tx_ids,
          {prefix}last_redeem_error,
          {prefix}channel_opening_fees,
          {prefix}confirmed_at          
          "
        )
    }

    pub(super) fn select_swap_query(&self, where_clause: &str, prefix: &str) -> String {
        let swap_fields = format!("        
          swaps.bitcoin_address  as {prefix}bitcoin_address,
          swaps.created_at as {prefix}created_at,
          lock_height as {prefix}lock_height,
          payment_hash as {prefix}payment_hash,
          preimage as {prefix}preimage,
          private_key as {prefix}private_key,
          public_key as {prefix}public_key,
          swapper_public_key as {prefix}swapper_public_key,
          script as {prefix}script,
          min_allowed_deposit as {prefix}min_allowed_deposit,
          max_allowed_deposit as {prefix}max_allowed_deposit,
          max_swapper_payable as {prefix}max_swapper_payable,
          bolt11 as {prefix}bolt11,
          paid_msat as {prefix}paid_msat,
          unconfirmed_sats as {prefix}unconfirmed_sats,
          confirmed_sats as {prefix}confirmed_sats,
          total_incoming_txs as {prefix}total_incoming_txs,
          status as {prefix}status,             
          (SELECT json_group_array(refund_tx_id) FROM sync.swap_refunds as swap_refunds where bitcoin_address = swaps.bitcoin_address) as {prefix}refund_tx_ids,
          unconfirmed_tx_ids as {prefix}unconfirmed_tx_ids,
          confirmed_tx_ids as {prefix}confirmed_tx_ids,
          last_redeem_error as {prefix}last_redeem_error,
          swaps_fees.channel_opening_fees as {prefix}channel_opening_fees,
          swaps_info.confirmed_at as {prefix}confirmed_at          
        ");

        format!(
            "
            SELECT
             {swap_fields}
            FROM sync.swaps as swaps
             LEFT JOIN swaps_info ON swaps.bitcoin_address = swaps_info.bitcoin_address
             LEFT JOIN sync.swaps_fees as swaps_fees ON swaps.bitcoin_address = swaps_fees.bitcoin_address
             LEFT JOIN sync.swap_refunds as swap_refunds ON swaps.bitcoin_address = swap_refunds.bitcoin_address
            WHERE {where_clause}
            "
        )
    }
}
