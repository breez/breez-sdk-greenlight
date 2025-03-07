use std::{collections::HashMap, rc::Rc};

use hex::FromHexError;
use rusqlite::{named_params, types::Value, Params, Row, Rows, TransactionBehavior};

use crate::swap_in_taproot::{
    FullTaprootSwapData, TaprootSwap, TaprootSwapOutput, TaprootSwapParameters, TaprootSwapRefund,
    TaprootSwapSpend,
};

use super::{db::SqliteStorage, error::PersistError};

impl SqliteStorage {
    pub(crate) fn add_taproot_swap(&self, swap: &TaprootSwap) -> Result<(), PersistError> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let opening_fee_params = serde_json::to_string(&swap.accepted_opening_fee_params)?;
        tx.execute(
            "INSERT INTO sync.taproot_swaps (
                address,
                claim_public_key,
                created_at,
                lock_time,
                payment_hash,
                preimage,
                refund_private_key,
                accepted_opening_fee_params)
            VALUES (
                :address,
                :claim_public_key,
                :created_at,
                :lock_time,
                :payment_hash,
                :preimage,
                :refund_private_key,
                :accepted_opening_fee_params)",
            named_params! {
                ":address": swap.address,
                ":claim_public_key": swap.claim_public_key,
                ":created_at": swap.created_at,
                ":lock_time": swap.lock_time,
                ":payment_hash": hex::encode(&swap.payment_hash),
                ":preimage": swap.preimage,
                ":refund_private_key": swap.refund_private_key,
                ":accepted_opening_fee_params": opening_fee_params,
            },
        )?;

        tx.commit()?;
        Ok(())
    }

    pub(crate) fn add_taproot_swap_outputs(
        &self,
        outputs: &[TaprootSwapOutput],
    ) -> Result<(), PersistError> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO taproot_swap_txos (
                    tx_id
                ,   output_index
                ,   address
                ,   amount_sat
                ,   confirmed_at_height
                ,   block_hash
                ) VALUES (:tx_id
                ,         :output_index
                ,         :address
                ,         :amount_sat
                ,         :confirmed_at_height
                ,         :block_hash
                )
                ",
            )?;

            for output in outputs {
                stmt.execute(named_params! {
                    ":tx_id": output.tx_id,
                    ":output_index": output.output_index,
                    ":address": output.address,
                    ":amount_sat": output.amount_sat,
                    ":confirmed_at_height": output.confirmed_at_height,
                    ":block_hash": output.block_hash,
                })?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    pub(crate) fn add_taproot_swap_refund(
        &self,
        refund: TaprootSwapRefund,
    ) -> Result<(), PersistError> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute(
            "INSERT OR IGNORE INTO taproot_swap_txos (
                refund_tx_id
            ,   spent_tx_id
            ,   spent_output_index
            ) VALUES (:refund_tx_id
            ,         :spent_tx_id
            ,         :spent_output_index
            )
            ",
            named_params! {
                ":refund_tx_id": refund.refund_tx_id,
                ":spent_tx_id": refund.spent_tx_id,
                ":spent_output_index": refund.spent_output_index,
            },
        )?;

        tx.commit()?;
        Ok(())
    }

    pub(crate) fn add_taproot_swap_spends(
        &self,
        spends: &[TaprootSwapSpend],
    ) -> Result<(), PersistError> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO taproot_swap_spends (
                    tx_id
                ,   output_index
                ,   spending_tx_id
                ,   spending_input_index
                ,   confirmed_at_height
                ,   block_hash
                ) VALUES (:tx_id
                ,         :output_index
                ,         :spending_tx_id
                ,         :spending_input_index
                ,         :confirmed_at_height
                ,         :block_hash
                )
                ",
            )?;

            for spend in spends {
                stmt.execute(named_params! {
                    ":tx_id": spend.tx_id,
                    ":output_index": spend.output_index,
                    ":spending_tx_id": spend.spending_tx_id,
                    ":spending_input_index": spend.spending_input_index,
                    ":confirmed_at_height": spend.confirmed_at_height,
                    ":block_hash": spend.block_hash,
                })?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    pub(crate) fn get_full_taproot_swap(
        &self,
        address: &str,
    ) -> Result<Option<FullTaprootSwapData>, PersistError> {
        let swaps = self.get_full_taproot_swaps(
            " WHERE s.address = :address",
            named_params! { ":address": address },
        )?;
        Ok(swaps.first().cloned())
    }

    pub(crate) fn get_full_taproot_swap_by_hash(
        &self,
        hash: &str,
    ) -> Result<Option<FullTaprootSwapData>, PersistError> {
        let hash = hex::decode(hash)?;
        let swaps = self.get_full_taproot_swaps(
            " WHERE s.payment_hash = :hash",
            named_params! { ":hash": hash },
        )?;
        Ok(swaps.first().cloned())
    }

    fn get_full_taproot_swaps<P: Params>(
        &self,
        where_clause: &str,
        params: P,
    ) -> Result<Vec<FullTaprootSwapData>, PersistError> {
        let conn = self.get_connection()?;
        let mut query = conn.prepare(
            &(String::from("SELECT s.address
            ,      s.claim_public_key
            ,      s.created_at
            ,      s.lock_time
            ,      s.payment_hash
            ,      s.preimage
            ,      s.refund_private_key
            ,      s.accepted_opening_fee_params
            ,      t.amount_sat
            ,      t.tx_id
            ,      t.output_index
            ,      t.confirmed_at_height
            ,      t.block_hash
            ,      sp.spending_tx_id
            ,      sp.spending_input_index
            ,      sp.confirmed_at_height AS spend_confirmed_at_height
            ,      sp.block_hash AS spend_block_hash
            ,      r.refund_tx_id
            ,      p.amount_msat AS paid_amount_msat
            ,      coalesce(o.open_channel_bolt11, json_extract(p.details, '$.bolt11'), c.bolt11) AS bolt11
            ,      c.max_swap_amount_sat
            ,      c.min_swap_amount_sat
            ,      c.min_utxo_amount_sat
            ,      c.last_payment_error
            FROM sync.taproot_swaps s
            LEFT JOIN taproot_swap_txos t ON s.address = t.address
            LEFT JOIN taproot_swap_spends sp ON t.tx_id = sp.tx_id AND t.output_index = sp.output_index
            LEFT JOIN sync.taproot_swap_refunds r 
                ON sp.spending_tx_id = r.refund_tx_id 
                AND sp.tx_id = r.spent_tx_id 
                AND sp.output_index = r.spent_output_index
            LEFT JOIN payments p ON s.payment_hash = p.id
            LEFT JOIN sync.open_channel_payment_info o ON s.payment_hash = o.payment_hash
            LEFT JOIN taproot_swap_cache c ON s.address = c.address
            ") + where_clause + 
            " ORDER BY created_at DESC"),
        )?;
        let mut rows = query.query(params)?;
        let swaps = self.map_full_taproot_swap_rows(&mut rows)?;
        Ok(swaps)
    }

    fn map_full_taproot_swap_rows(
        &self,
        rows: &mut Rows<'_>,
    ) -> Result<Vec<FullTaprootSwapData>, PersistError> {
        let mut result = HashMap::new();
        while let Some(row) = rows.next()? {
            let address: String = row.get("address")?;
            if !result.contains_key(&address) {
                let payment_hash: String = row.get("payment_hash")?;
                let payment_hash = hex::decode(payment_hash)?;
                let accepted_opening_fee_params: String = row.get("accepted_opening_fee_params")?;
                let accepted_opening_fee_params =
                    serde_json::from_str(&accepted_opening_fee_params)?;
                let max_swap_amount_sat: Option<u64> = row.get("max_swap_amount_sat")?;
                let parameters = match max_swap_amount_sat {
                    Some(max_swap_amount_sat) => Some(TaprootSwapParameters {
                        max_swap_amount_sat,
                        min_swap_amount_sat: row.get("min_swap_amount_sat")?,
                        min_utxo_amount_sat: row.get("min_utxo_amount_sat")?,
                    }),
                    None => None,
                };
                result.insert(
                    address.clone(),
                    FullTaprootSwapData {
                        swap: TaprootSwap {
                            address: address.clone(),
                            created_at: row.get("created_at")?,
                            lock_time: row.get("lock_time")?,
                            preimage: row.get("preimage")?,
                            payment_hash,
                            refund_private_key: row.get("refund_private_key")?,
                            claim_public_key: row.get("claim_public_key")?,
                            accepted_opening_fee_params,
                        },
                        outputs: Vec::new(),
                        paid_amount_msat: row.get("paid_amount_msat")?,
                        bolt11: row.get("bolt11")?,
                        last_payment_error: row.get("last_payment_error")?,
                        parameters,
                        refund_transactions: Vec::new(),
                    },
                );
            }

            let current_swap = result
                .get_mut(&address)
                .expect("just inserted key is not in hashmap");
            self.map_tx_info(current_swap, row)?;
            self.map_refund(current_swap, row)?;
        }

        Ok(result.into_values().collect())
    }

    fn map_tx_info(
        &self,
        swap: &mut FullTaprootSwapData,
        row: &Row<'_>,
    ) -> Result<(), PersistError> {
        let tx_id: Option<String> = row.get("tx_id")?;
        let tx_id = match tx_id {
            Some(tx_id) => tx_id,
            None => return Ok(()),
        };
        let output_index: u32 = row.get("output_index")?;

        if swap
            .outputs
            .iter()
            .any(|o| o.tx_id == tx_id && o.output_index == output_index)
        {
            return Ok(());
        }

        let mut output = TaprootSwapOutput {
            address: swap.swap.address.clone(),
            amount_sat: row.get("amount_sat")?,
            tx_id: tx_id.clone(),
            output_index,
            confirmed_at_height: row.get("confirmed_at_height")?,
            block_hash: row.get("block_hash")?,
            spend: None,
        };

        let spending_tx_id: Option<String> = row.get("spending_tx_id")?;
        if let Some(spending_tx_id) = spending_tx_id {
            output.spend = Some(TaprootSwapSpend {
                tx_id,
                output_index,
                spending_tx_id,
                spending_input_index: row.get("spending_input_index")?,
                block_hash: row.get("spend_block_hash")?,
                confirmed_at_height: row.get("spend_confirmed_at_height")?,
            })
        }

        swap.outputs.push(output);
        Ok(())
    }

    fn map_refund(
        &self,
        swap: &mut FullTaprootSwapData,
        row: &Row<'_>,
    ) -> Result<(), PersistError> {
        let refund_tx_id: Option<String> = row.get("refund_tx_id")?;
        let refund_tx_id = match refund_tx_id {
            Some(refund_tx_id) => refund_tx_id,
            None => return Ok(()),
        };

        if swap.refund_transactions.contains(&refund_tx_id) {
            return Ok(());
        }

        swap.refund_transactions.push(refund_tx_id);
        Ok(())
    }

    pub(crate) fn list_unused_taproot_swaps(
        &self,
    ) -> Result<Vec<FullTaprootSwapData>, PersistError> {
        self.get_full_taproot_swaps(" WHERE t.tx_id IS NULL", [])
    }

    pub(crate) fn list_taproot_swaps(&self) -> Result<Vec<FullTaprootSwapData>, PersistError> {
        self.get_full_taproot_swaps("", [])
    }

    pub(crate) fn list_taproot_swaps_by_hash(
        &self,
        hashes: &[String],
    ) -> Result<Vec<FullTaprootSwapData>, PersistError> {
        let hashes = Rc::new(
            hashes
                .iter()
                .map(hex::decode)
                .collect::<Result<Vec<_>, FromHexError>>()?
                .into_iter()
                .map(Value::from)
                .collect::<Vec<Value>>(),
        );
        self.get_full_taproot_swaps(
            " WHERE s.payment_hash IN rarray(:hashes)",
            named_params! {
                ":hashes": hashes
            },
        )
    }

    pub(crate) fn set_taproot_swap_bolt11(
        &self,
        address: &str,
        bolt11: &str,
    ) -> Result<(), PersistError> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute(
            "INSERT INTO taproot_swap_cache (
            address
        ,   bolt11
        ) VALUES (
            :address
        ,   :bolt11)
        ON CONFLICT (address) DO UPDATE
        SET bolt11=:bolt11",
            named_params! {
                ":address": address,
                ":bolt11": bolt11,
            },
        )?;
        tx.commit()?;
        Ok(())
    }

    pub(crate) fn set_taproot_swap_last_payment_error(
        &self,
        address: &str,
        last_payment_error: &str,
    ) -> Result<(), PersistError> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute(
            "INSERT INTO taproot_swap_cache (
            address
        ,   last_payment_error
        ) VALUES (
            :address
        ,   :last_payment_error)
        ON CONFLICT (address) DO UPDATE
        SET last_payment_error=:last_payment_error",
            named_params! {
                ":address": address,
                ":last_payment_error": last_payment_error,
            },
        )?;
        tx.commit()?;
        Ok(())
    }

    pub(crate) fn set_taproot_swap_parameters(
        &self,
        address: &str,
        parameters: &TaprootSwapParameters,
    ) -> Result<(), PersistError> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute(
            "INSERT INTO taproot_swap_cache (
            address
        ,   max_swap_amount_sat
        ,   min_swap_amount_sat
        ,   min_utxo_amount_sat
        ) VALUES (
            :address
        ,   :max_swap_amount_sat
        ,   :min_swap_amount_sat
        ,   :min_utxo_amount_sat)
        ON CONFLICT (address) DO UPDATE
        SET max_swap_amount_sat=:max_swap_amount_sat
        ,   min_swap_amount_sat=:min_swap_amount_sat
        ,   min_utxo_amount_sat=:min_utxo_amount_sat",
            named_params! {
                ":address": address,
                ":max_swap_amount_sat": parameters.max_swap_amount_sat,
                ":min_swap_amount_sat": parameters.min_swap_amount_sat,
                ":min_utxo_amount_sat": parameters.min_utxo_amount_sat,
            },
        )?;
        tx.commit()?;
        Ok(())
    }
}
