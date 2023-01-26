use crate::models::{SwapInfo, SwapStatus};

use super::db::{SqliteStorage, StringArray};
use anyhow::{anyhow, Result};
use rusqlite::{named_params, OptionalExtension, Row};

impl SqliteStorage {
    pub(crate) fn insert_swap(&self, swap_info: SwapInfo) -> Result<()> {
        self.get_connection()?.execute(
         "INSERT INTO swaps (bitcoin_address, created_at, lock_height, payment_hash, preimage, private_key, public_key, swapper_public_key, bolt11, paid_sats, unconfirmed_sats, confirmed_sats, script, status, refund_tx_ids, unconfirmed_tx_ids, confirmed_tx_ids, min_allowed_deposit, max_allowed_deposit)
          VALUES (:bitcoin_address, :created_at, :lock_height, :payment_hash, :preimage, :private_key, :public_key, :swapper_public_key, :bolt11, :paid_sats, :unconfirmed_sats, :confirmed_sats, :script, :status, :refund_tx_ids, :unconfirmed_tx_ids, :confirmed_tx_ids, :min_allowed_deposit, :max_allowed_deposit)",
         named_params! {
             ":bitcoin_address": swap_info.bitcoin_address,
             ":created_at": swap_info.created_at,
             ":lock_height": swap_info.lock_height,
             ":payment_hash": swap_info.payment_hash,
             ":preimage": swap_info.preimage,
             ":private_key": swap_info.private_key,
             ":public_key": swap_info.public_key,
             ":swapper_public_key": swap_info.swapper_public_key,
             ":paid_sats": swap_info.paid_sats,
             ":unconfirmed_sats": swap_info.unconfirmed_sats,
             ":confirmed_sats": swap_info.confirmed_sats,
             ":script": swap_info.script,
             ":bolt11": None::<String>,
             ":status": swap_info.status as u32,
             ":refund_tx_ids": StringArray(swap_info.refund_tx_ids),
             ":unconfirmed_tx_ids": StringArray(swap_info.unconfirmed_tx_ids),
             ":confirmed_tx_ids": StringArray(swap_info.confirmed_tx_ids),
             ":min_allowed_deposit": swap_info.min_allowed_deposit,
             ":max_allowed_deposit": swap_info.max_allowed_deposit
         },
        )?;

        Ok(())
    }

    pub(crate) fn update_swap_paid_amount(
        &self,
        bitcoin_address: String,
        paid_sats: u32,
    ) -> Result<()> {
        self.get_connection()?.execute(
            "UPDATE swaps SET paid_sats=:paid_sats where bitcoin_address=:bitcoin_address",
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
            "UPDATE swaps SET last_redeem_error=:redeem_err where bitcoin_address=:bitcoin_address",
            named_params! {
             ":redeem_err": redeem_err,
             ":bitcoin_address": bitcoin_address,
            },
        )?;

        Ok(())
    }

    pub(crate) fn update_swap_bolt11(&self, bitcoin_address: String, bolt11: String) -> Result<()> {
        self.get_connection()?.execute(
            "UPDATE swaps SET bolt11=:bolt11 where bitcoin_address=:bitcoin_address",
            named_params! {
             ":bolt11": bolt11,
             ":bitcoin_address": bitcoin_address,
            },
        )?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn update_swap_chain_info(
        &self,
        bitcoin_address: String,
        unconfirmed_sats: u32,
        unconfirmed_tx_ids: Vec<String>,
        confirmed_sats: u32,
        confirmed_tx_ids: Vec<String>,
        refund_tx_ids: Vec<String>,
        status: SwapStatus,
    ) -> Result<SwapInfo> {
        self.get_connection()?.execute(
            "UPDATE swaps SET unconfirmed_sats=:unconfirmed_sats, unconfirmed_tx_ids=:unconfirmed_tx_ids, confirmed_sats=:confirmed_sats, refund_tx_ids=:refund_tx_ids, confirmed_tx_ids=:confirmed_tx_ids, status=:status where bitcoin_address=:bitcoin_address",
            named_params! {
             ":unconfirmed_sats": unconfirmed_sats,
             ":unconfirmed_tx_ids": StringArray(unconfirmed_tx_ids),
             ":confirmed_sats": confirmed_sats,
             ":bitcoin_address": bitcoin_address,
             ":refund_tx_ids": StringArray(refund_tx_ids),
             ":confirmed_tx_ids": StringArray(confirmed_tx_ids),
             ":status": status as u32
            },
        )?;
        Ok(self.get_swap_info_by_address(bitcoin_address)?.unwrap())
    }

    pub(crate) fn get_swap_info_by_hash(&self, hash: &Vec<u8>) -> Result<Option<SwapInfo>> {
        self.get_connection()?
            .query_row(
                "SELECT * FROM swaps where payment_hash = ?1",
                [hash],
                |row| self.sql_row_to_swap(row),
            )
            .optional()
            .map_err(|e| anyhow!(e))
    }

    pub(crate) fn get_swap_info_by_address(&self, address: String) -> Result<Option<SwapInfo>> {
        self.get_connection()?
            .query_row(
                "SELECT * FROM swaps where bitcoin_address = ?1",
                [address],
                |row| self.sql_row_to_swap(row),
            )
            .optional()
            .map_err(|e| anyhow!(e))
    }

    pub(crate) fn list_swaps_with_status(&self, status: SwapStatus) -> Result<Vec<SwapInfo>> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare(
            "
              SELECT * FROM swaps WHERE status=?         
            ",
        )?;

        let vec: Vec<SwapInfo> = stmt
            .query_map([status as u32], |row| self.sql_row_to_swap(row))?
            .map(|i| i.unwrap())
            .collect();

        Ok(vec)
    }

    fn sql_row_to_swap(&self, row: &Row) -> Result<SwapInfo, rusqlite::Error> {
        let status: i32 = row.get(13)?;
        let status: SwapStatus = status.try_into().map_or(SwapStatus::Initial, |v| v);
        let refund_txs_raw: StringArray = row.get(14)?;
        let unconfirmed_tx_ids: StringArray = row.get(15)?;
        let confirmed_txs_raw: StringArray = row.get(16)?;
        Ok(SwapInfo {
            bitcoin_address: row.get(0)?,
            created_at: row.get(1)?,
            lock_height: row.get(2)?,
            payment_hash: row.get(3)?,
            preimage: row.get(4)?,
            private_key: row.get(5)?,
            public_key: row.get(6)?,
            swapper_public_key: row.get(7)?,
            script: row.get(8)?,
            bolt11: row.get(9)?,
            paid_sats: row.get(10)?,
            unconfirmed_sats: row.get(11)?,
            confirmed_sats: row.get(12)?,
            status,
            refund_tx_ids: refund_txs_raw.0,
            unconfirmed_tx_ids: unconfirmed_tx_ids.0,
            confirmed_tx_ids: confirmed_txs_raw.0,
            min_allowed_deposit: row.get(17)?,
            max_allowed_deposit: row.get(18)?,
            last_redeem_error: row.get(19)?,
        })
    }
}

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

    let storage = SqliteStorage::from_file(test_utils::create_test_sql_file("swap".to_string()));

    println!("before init");
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
        status: crate::models::SwapStatus::Initial,
        refund_tx_ids: Vec::new(),
        unconfirmed_tx_ids: Vec::new(),
        confirmed_tx_ids: Vec::new(),
        min_allowed_deposit: 0,
        max_allowed_deposit: 100,
        last_redeem_error: None,
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
        vec![],
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
        vec![String::from("111"), String::from("222")],
        SwapStatus::Expired,
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

    storage.update_swap_paid_amount(tested_swap_info.bitcoin_address.clone(), 30)?;
    let updated_swap = storage
        .get_swap_info_by_address(tested_swap_info.bitcoin_address)?
        .unwrap();
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
