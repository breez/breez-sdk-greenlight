use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, Type, ValueRef};
use rusqlite::Row;
use rusqlite::{named_params, params, OptionalExtension};
use sdk_common::prelude::*;
use serde_json::{Map, Value};

use super::db::SqliteStorage;
use super::error::{PersistError, PersistResult};
use crate::models::*;

const METADATA_MAX_LEN: usize = 1000;

#[cfg_attr(test, mockall::automock)]
pub(crate) trait PaymentStorage: Send + Sync {
    fn get_completed_payment_by_hash(&self, hash: &str) -> PersistResult<Option<Payment>>;
    fn get_open_channel_bolt11_by_hash(&self, hash: &str) -> PersistResult<Option<String>>;
}

impl PaymentStorage for SqliteStorage {
    /// Looks up a completed payment by hash.
    ///
    /// To include pending or failed payments in the lookup as well, use [Self::get_payment_by_hash]
    fn get_completed_payment_by_hash(&self, hash: &str) -> PersistResult<Option<Payment>> {
        let res = self
            .get_payment_by_hash(hash)?
            .filter(|p| p.status == PaymentStatus::Complete);
        Ok(res)
    }

    /// Look up a modified open channel bolt11 by hash.
    fn get_open_channel_bolt11_by_hash(&self, hash: &str) -> PersistResult<Option<String>> {
        Ok(self
            .get_connection()?
            .query_row(
                "
          SELECT o.open_channel_bolt11           
          FROM sync.open_channel_payment_info o        
          WHERE
           payment_hash = ?1",
                [hash],
                |row| row.get(0),
            )
            .optional()?)
    }
}

impl SqliteStorage {
    /// Inserts payments into the payments table. These can be pending, completed and failed payments. Before
    /// persisting, it automatically deletes previously pending payments
    ///
    /// Note that, if a payment has details of type [LnPaymentDetails] which contain a [SuccessActionProcessed],
    /// then the [LnPaymentDetails] will NOT be persisted. In that case, the [SuccessActionProcessed]
    /// can be inserted separately via [SqliteStorage::insert_payment_external_info].
    pub fn insert_or_update_payments(
        &self,
        transactions: &[Payment],
        is_pseudo: bool,
    ) -> PersistResult<()> {
        let con = self.get_connection()?;
        let mut prep_statement = con.prepare(
            "
         INSERT OR REPLACE INTO payments (
           id,
           payment_type,                 
           payment_time,                                  
           amount_msat, 
           fee_msat,                 
           status,
           description,
           details,
           is_pseudo
        )
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)
        ",
        )?;

        for ln_tx in transactions {
            _ = prep_statement.execute((
                &ln_tx.id,
                &ln_tx.payment_type.to_string(),
                &ln_tx.payment_time,
                &ln_tx.amount_msat,
                &ln_tx.fee_msat,
                &ln_tx.status,
                &ln_tx.description,
                &ln_tx.details,
                &is_pseudo,
            ))?;
        }
        Ok(())
    }

    pub fn delete_pseudo_payments(&self) -> PersistResult<()> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare("DELETE FROM payments where is_pseudo=1")?;
        let res = stmt.execute([])?;
        if res > 0 {
            debug!("deleted {} pseudo-payments", res);
        }

        Ok(())
    }

    /// Inserts metadata associated with this payment
    pub fn insert_payment_external_info(
        &self,
        payment_hash: &str,
        payment_external_info: PaymentExternalInfo,
    ) -> PersistResult<()> {
        let con = self.get_connection()?;
        let mut prep_statement = con.prepare(
            "
         INSERT OR REPLACE INTO sync.payments_external_info (
           payment_id,
           lnurl_success_action,
           lnurl_pay_domain,
           lnurl_pay_comment,
           lnurl_metadata,
           ln_address,
           lnurl_withdraw_endpoint,
           attempted_amount_msat,
           attempted_error
         )
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)
        ",
        )?;

        _ = prep_statement.execute((
            payment_hash,
            serde_json::to_string(&payment_external_info.lnurl_pay_success_action)?,
            payment_external_info.lnurl_pay_domain,
            payment_external_info.lnurl_pay_comment,
            payment_external_info.lnurl_metadata,
            payment_external_info.ln_address,
            payment_external_info.lnurl_withdraw_endpoint,
            payment_external_info.attempted_amount_msat,
            payment_external_info.attempted_error,
        ))?;

        Ok(())
    }

    /// Updates the metadata object associated to a payment
    pub fn set_payment_external_metadata(
        &self,
        payment_hash: String,
        new_metadata: String,
    ) -> PersistResult<()> {
        ensure_sdk!(
            new_metadata.len() <= METADATA_MAX_LEN,
            PersistError::Generic(format!(
                "Max metadata size ({METADATA_MAX_LEN} characters) has been exceeded"
            ))
        );

        let _ = serde_json::from_str::<Map<String, Value>>(&new_metadata)?;

        // Check if the payment exists
        let payment_exists = self
            .get_connection()?
            .prepare("SELECT 1 FROM payments WHERE id = ?1;")?
            .exists(params![payment_hash])?;

        if !payment_exists {
            return Err(PersistError::generic("Payment not found"));
        }

        self.get_connection()?.execute(
            "
             INSERT OR REPLACE INTO sync.payments_metadata(
                payment_id,
                metadata,
                updated_at
             )
             VALUES (
                ?1,
                json(?2),
                CURRENT_TIMESTAMP
             );",
            params![payment_hash, new_metadata],
        )?;

        Ok(())
    }

    /// Updates attempted error data associated with this payment
    pub fn update_payment_attempted_error(
        &self,
        payment_hash: &str,
        attempted_error: Option<String>,
    ) -> PersistResult<()> {
        self.get_connection()?.execute(
            "UPDATE sync.payments_external_info SET attempted_error=:attempted_error WHERE payment_id=:payment_id",
            named_params! {
             ":payment_id": payment_hash,
             ":attempted_error": attempted_error,
            },
        )?;

        Ok(())
    }

    /// Inserts payer amount for invoices that require opening a channel.
    pub fn insert_open_channel_payment_info(
        &self,
        payment_hash: &str,
        payer_amount_msat: u64,
        open_channel_bolt11: &str,
    ) -> PersistResult<()> {
        let con = self.get_connection()?;
        let mut prep_statement = con.prepare(
            "
        INSERT OR IGNORE INTO sync.open_channel_payment_info (
          payment_hash,
          payer_amount_msat,
          open_channel_bolt11
        )
        VALUES (?1,?2,?3)
       ",
        )?;

        _ = prep_statement.execute((payment_hash, payer_amount_msat, open_channel_bolt11))?;

        Ok(())
    }

    /// Constructs [Payment] by joining data in the `payment` and `payments_external_info` tables
    ///
    /// This queries all payments. To query a single payment, see [Self::get_payment_by_hash]
    /// or [Self::get_completed_payment_by_hash]
    pub fn list_payments(&self, req: ListPaymentsRequest) -> PersistResult<Vec<Payment>> {
        let where_clause = filter_to_where_clause(
            req.filters,
            &req.metadata_filters,
            req.from_timestamp,
            req.to_timestamp,
            req.include_failures,
        );
        let offset = req.offset.unwrap_or(0u32);
        let limit = req.limit.unwrap_or(u32::MAX);
        let con = self.get_connection()?;
        let query = self.select_payments_query(where_clause.as_str(), offset, limit)?;
        let mut stmt = con.prepare(query.as_str())?;

        let mut params: HashMap<String, String> = HashMap::new();

        if let Some(metadata_filters) = &req.metadata_filters {
            metadata_filters.iter().enumerate().for_each(
                |(
                    i,
                    MetadataFilter {
                        json_path,
                        json_value,
                    },
                )| {
                    params.insert(format!(":json_path_{i}"), format!("$.{json_path}"));
                    params.insert(format!(":json_value_{i}"), json_value.clone());
                },
            )
        }

        let vec: Vec<Payment> = stmt
            .query_map(
                params
                    .iter()
                    .map(|(k, v)| (k.as_str(), v as &dyn ToSql))
                    .collect::<Vec<(&str, &dyn ToSql)>>()
                    .as_slice(),
                |row| self.sql_row_to_payment(row),
            )?
            .map(|i| i.unwrap())
            .collect();
        Ok(vec)
    }

    pub fn select_payments_query(
        &self,
        where_clause: &str,
        offset: u32,
        limit: u32,
    ) -> PersistResult<String> {
        let swap_fields = self.select_swap_fields("swaps_");
        let swap_query = self.select_swap_query("true", "swaps_");
        let rev_swap_fields = self.select_reverse_swap_fields("revswaps_");
        let rev_swap_query = self.select_reverse_swap_query("true", "revswaps_");
        let query = format!(
            "
          SELECT 
           p.id,
           p.payment_type,
           p.payment_time,
           p.amount_msat,
           p.fee_msat,
           p.status,
           p.description,
           p.details,
           e.lnurl_success_action,
           e.lnurl_metadata,
           e.ln_address,
           e.lnurl_withdraw_endpoint,
           e.attempted_amount_msat,
           e.attempted_error,
           o.payer_amount_msat,
           o.open_channel_bolt11,
           m.metadata,
           e.lnurl_pay_domain,
           e.lnurl_pay_comment,
           {swap_fields},
           {rev_swap_fields}
          FROM payments p
          LEFT JOIN sync.payments_external_info e
          ON
           p.id = e.payment_id
          LEFT JOIN sync.payments_metadata m
          ON
            p.id = m.payment_id
          LEFT JOIN sync.open_channel_payment_info o
           ON
            p.id = o.payment_hash
          LEFT JOIN ({swap_query}) as swaps
           ON
            p.id = hex(swaps_payment_hash) COLLATE NOCASE
          LEFT JOIN ({rev_swap_query}) as revswaps
           ON
            json_extract(p.details, '$.payment_preimage') = hex(revswaps_preimage) COLLATE NOCASE
          {where_clause}
          ORDER BY payment_time DESC
          LIMIT {limit}
          OFFSET {offset}
        "
        );

        Ok(query)
    }

    /// This queries a single payment by hash, which may be pending, completed or failed.
    ///
    /// To lookup a completed payment by hash, use [Self::get_completed_payment_by_hash]
    ///
    /// To query all payments, see [Self::list_payments]
    pub(crate) fn get_payment_by_hash(&self, hash: &str) -> PersistResult<Option<Payment>> {
        let query = self.select_payments_query("where id = ?1", 0, 1)?;
        Ok(self
            .get_connection()?
            .query_row(query.as_str(), [hash], |row| self.sql_row_to_payment(row))
            .optional()?)
    }

    /// Look up a modified open channel bolt11 by hash.
    #[allow(unused)]
    pub(crate) fn get_open_channel_bolt11_by_hash(
        &self,
        hash: &str,
    ) -> PersistResult<Option<String>> {
        Ok(self
            .get_connection()?
            .query_row(
                "
          SELECT
           o.open_channel_bolt11           
          FROM sync.open_channel_payment_info o        
          WHERE
           payment_hash = ?1",
                [hash],
                |row| row.get(0),
            )
            .optional()?)
    }

    fn sql_row_to_payment(&self, row: &Row) -> PersistResult<Payment, rusqlite::Error> {
        let payment_type_str: String = row.get(1)?;
        let amount_msat = row.get(3)?;
        let status: PaymentStatus = row.get(5)?;
        let attempted_amount_msat: Option<u64> = row.get(12)?;
        let mut payment = Payment {
            id: row.get(0)?,
            payment_type: PaymentType::from_str(payment_type_str.as_str()).unwrap(),
            payment_time: row.get(2)?,
            amount_msat: match status {
                PaymentStatus::Complete => amount_msat,
                _ => attempted_amount_msat.unwrap_or(amount_msat),
            },
            fee_msat: row.get(4)?,
            status,
            description: row.get(6)?,
            details: row.get(7)?,
            error: row.get(13)?,
            metadata: row.get(16)?,
        };

        if let PaymentDetails::Ln { ref mut data } = payment.details {
            let lnurl_success_action_str: Option<String> = row.get(8)?;
            data.lnurl_success_action = match lnurl_success_action_str {
                None => None,
                Some(s) => serde_json::from_str(&s).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(8, Type::Text, Box::new(e))
                })?,
            };

            data.lnurl_pay_domain = row.get(17)?;
            data.lnurl_pay_comment = row.get(18)?;
            data.lnurl_metadata = row.get(9)?;
            data.ln_address = row.get(10)?;
            data.lnurl_withdraw_endpoint = row.get(11)?;
            data.swap_info = self.sql_row_to_swap(row, "swaps_").ok();
            if let Ok(fr) = self.sql_row_to_reverse_swap(row, "revswaps_") {
                data.reverse_swap_info = Some(fr.get_reverse_swap_info_using_cached_values());
            }
        }

        // In case we have a record of the open channel fee, let's use it.
        let payer_amount_msat: Option<u64> = row.get(14)?;
        if let Some(payer_amount) = payer_amount_msat {
            payment.fee_msat = payer_amount - amount_msat;
        }

        // Add the payer invoice if it exists, in case of a received payment
        if let Some(open_channel_bolt11) = row.get(15)? {
            if let PaymentDetails::Ln { data } = &mut payment.details {
                data.open_channel_bolt11 = Some(open_channel_bolt11);
            }
        }

        Ok(payment)
    }
}

fn filter_to_where_clause(
    type_filters: Option<Vec<PaymentTypeFilter>>,
    metadata_filters: &Option<Vec<MetadataFilter>>,
    from_timestamp: Option<i64>,
    to_timestamp: Option<i64>,
    include_failures: Option<bool>,
) -> String {
    let mut where_clause: Vec<String> = Vec::new();
    let with_failures = include_failures.unwrap_or(false);

    if let Some(t) = from_timestamp {
        where_clause.push(format!("payment_time >= {t}"));
    };
    if let Some(t) = to_timestamp {
        where_clause.push(format!("payment_time <= {t}"));
    };
    if !with_failures {
        where_clause.push(format!("status != {}", PaymentStatus::Failed as i64));
    };

    if let Some(filters) = type_filters {
        if !filters.is_empty() {
            let mut type_filter_clause: HashSet<PaymentType> = HashSet::new();
            for type_filter in filters {
                match type_filter {
                    PaymentTypeFilter::Sent => {
                        type_filter_clause.insert(PaymentType::Sent);
                    }
                    PaymentTypeFilter::Received => {
                        type_filter_clause.insert(PaymentType::Received);
                    }
                    PaymentTypeFilter::ClosedChannel => {
                        type_filter_clause.insert(PaymentType::ClosedChannel);
                    }
                }
            }

            where_clause.push(format!(
                "payment_type in ({})",
                type_filter_clause
                    .iter()
                    .map(|t| format!("'{t}'"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    if let Some(filters) = metadata_filters {
        filters.iter().enumerate().for_each(|(i, _)| {
            where_clause.push(format!("metadata->:json_path_{i} = :json_value_{i}"));
        });
    }

    let mut where_clause_str = String::new();
    if !where_clause.is_empty() {
        where_clause_str = String::from("where ");
        where_clause_str.push_str(where_clause.join(" and ").as_str());
    }
    where_clause_str
}

impl FromSql for PaymentDetails {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        serde_json::from_str(value.as_str()?).map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for PaymentDetails {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(
            serde_json::to_string(&self).map_err(|_| FromSqlError::InvalidType)?,
        ))
    }
}

impl FromSql for PaymentStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => match i as u8 {
                0 => Ok(PaymentStatus::Pending),
                1 => Ok(PaymentStatus::Complete),
                2 => Ok(PaymentStatus::Failed),
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for PaymentStatus {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(*self as i64))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        persist::{db::SqliteStorage, error::PersistResult, swap::SwapStorage},
        FullReverseSwapInfo, ListPaymentsRequest, MetadataFilter, OpeningFeeParams,
        PaymentExternalInfo, PaymentStatus, PaymentType, PaymentTypeFilter, ReverseSwapInfo,
        ReverseSwapInfoCached, ReverseSwapStatus, SwapInfo, SwapStatus,
    };

    #[test]
    fn test_ln_transactions() -> PersistResult<(), Box<dyn std::error::Error>> {
        use sdk_common::prelude::*;

        use crate::models::{LnPaymentDetails, Payment, PaymentDetails};
        use crate::persist::test_utils;

        let lnurl_metadata = "{'key': 'sample-metadata-val'}";
        let test_ln_address = "test@ln.adddress.com";
        let test_lnurl_pay_domain = "example.com";
        let test_lnurl_pay_comment = "Thank you Satoshi!";
        let sa = SuccessActionProcessed::Message {
            data: MessageSuccessActionData {
                message: "test message".into(),
            },
        };

        let payment_hash_with_lnurl_success_action = "123";
        let payment_hash_with_lnurl_withdraw = "124";
        let payment_hash_with_swap_info: Vec<u8> = vec![234, 12, 53, 124];
        let payment_hash_with_lnurl_domain = "126";
        let payment_hash_with_rev_swap_info: Vec<u8> = vec![8, 7, 6, 5, 4, 3, 2, 1];
        let lnurl_withdraw_url = "https://test.lnurl.withdraw.link";
        let swap_info = SwapInfo {
            bitcoin_address: "123".to_string(),
            created_at: 1234567,
            lock_height: 7654321,
            payment_hash: payment_hash_with_swap_info.clone(),
            preimage: vec![1, 2, 3],
            private_key: vec![3, 2, 1],
            public_key: vec![1, 3, 2],
            swapper_public_key: vec![2, 1, 3],
            script: vec![2, 3, 1],
            bolt11: Some("swap_bolt11".into()),
            paid_msat: 50_000,
            confirmed_sats: 50,
            unconfirmed_sats: 0,
            total_incoming_txs: 1,
            status: SwapStatus::Refundable,
            refund_tx_ids: vec![],
            unconfirmed_tx_ids: vec![],
            confirmed_tx_ids: vec![],
            min_allowed_deposit: 5_000,
            max_allowed_deposit: 1_000_000,
            max_swapper_payable: 2_000_000,
            last_redeem_error: None,
            channel_opening_fees: Some(OpeningFeeParams {
                min_msat: 5_000_000,
                proportional: 50,
                valid_until: "date".to_string(),
                max_idle_time: 12345,
                max_client_to_self_delay: 234,
                promise: "promise".to_string(),
            }),
            confirmed_at: Some(555),
        };
        let rev_swap_preimage = vec![4, 4, 4, 4];
        let full_ref_swap_info = FullReverseSwapInfo {
            id: "rev_swap_id".to_string(),
            created_at_block_height: 0,
            preimage: rev_swap_preimage.clone(),
            private_key: vec![],
            claim_pubkey: "claim_pubkey".to_string(),
            timeout_block_height: 600_000,
            invoice: "645".to_string(),
            redeem_script: "redeem_script".to_string(),
            onchain_amount_sat: 250,
            sat_per_vbyte: Some(50),
            receive_amount_sat: None,
            cache: ReverseSwapInfoCached {
                status: ReverseSwapStatus::CompletedConfirmed,
                lockup_txid: Some("lockup_txid".to_string()),
                claim_txid: Some("claim_txid".to_string()),
            },
        };
        let rev_swap_info = ReverseSwapInfo {
            id: "rev_swap_id".to_string(),
            claim_pubkey: "claim_pubkey".to_string(),
            lockup_txid: Some("lockup_txid".to_string()),
            claim_txid: Some("claim_txid".to_string()),
            onchain_amount_sat: 250,
            status: ReverseSwapStatus::CompletedConfirmed,
        };
        let txs = [
            Payment {
                id: payment_hash_with_lnurl_success_action.to_string(),
                payment_type: PaymentType::Sent,
                payment_time: 1001,
                amount_msat: 100,
                fee_msat: 20,
                status: PaymentStatus::Complete,
                error: None,
                description: None,
                details: PaymentDetails::Ln {
                    data: LnPaymentDetails {
                        payment_hash: payment_hash_with_lnurl_success_action.to_string(),
                        label: "label".to_string(),
                        destination_pubkey: "pubey".to_string(),
                        payment_preimage: "1111".to_string(),
                        keysend: true,
                        bolt11: "bolt11".to_string(),
                        lnurl_success_action: Some(sa.clone()),
                        lnurl_pay_domain: None,
                        lnurl_pay_comment: None,
                        lnurl_metadata: Some(lnurl_metadata.to_string()),
                        ln_address: Some(test_ln_address.to_string()),
                        lnurl_withdraw_endpoint: None,
                        swap_info: None,
                        reverse_swap_info: None,
                        pending_expiration_block: None,
                        open_channel_bolt11: None,
                    },
                },
                metadata: None,
            },
            Payment {
                id: payment_hash_with_lnurl_withdraw.to_string(),
                payment_type: PaymentType::Received,
                payment_time: 1000,
                amount_msat: 100,
                fee_msat: 20,
                status: PaymentStatus::Complete,
                error: None,
                description: Some("desc".to_string()),
                details: PaymentDetails::Ln {
                    data: LnPaymentDetails {
                        payment_hash: payment_hash_with_lnurl_withdraw.to_string(),
                        label: "label".to_string(),
                        destination_pubkey: "pubey".to_string(),
                        payment_preimage: "2222".to_string(),
                        keysend: true,
                        bolt11: "bolt11".to_string(),
                        lnurl_success_action: None,
                        lnurl_pay_domain: None,
                        lnurl_pay_comment: None,
                        lnurl_metadata: None,
                        ln_address: None,
                        lnurl_withdraw_endpoint: Some(lnurl_withdraw_url.to_string()),
                        swap_info: None,
                        reverse_swap_info: None,
                        pending_expiration_block: None,
                        open_channel_bolt11: None,
                    },
                },
                metadata: None,
            },
            Payment {
                id: hex::encode(payment_hash_with_swap_info.clone()),
                payment_type: PaymentType::Received,
                payment_time: 999,
                amount_msat: 50_000,
                fee_msat: 20,
                status: PaymentStatus::Complete,
                error: None,
                description: Some("desc".to_string()),
                details: PaymentDetails::Ln {
                    data: LnPaymentDetails {
                        payment_hash: hex::encode(payment_hash_with_swap_info),
                        label: "label".to_string(),
                        destination_pubkey: "pubkey".to_string(),
                        payment_preimage: "3333".to_string(),
                        keysend: false,
                        bolt11: "swap_bolt11".to_string(),
                        lnurl_success_action: None,
                        lnurl_pay_domain: None,
                        lnurl_pay_comment: None,
                        lnurl_metadata: None,
                        ln_address: None,
                        lnurl_withdraw_endpoint: None,
                        swap_info: Some(swap_info.clone()),
                        reverse_swap_info: None,
                        pending_expiration_block: None,
                        open_channel_bolt11: None,
                    },
                },
                metadata: None,
            },
            Payment {
                id: hex::encode(payment_hash_with_rev_swap_info.clone()),
                payment_type: PaymentType::Sent,
                payment_time: 998,
                amount_msat: 100_000,
                fee_msat: 200,
                status: PaymentStatus::Complete,
                error: None,
                description: Some("desc".to_string()),
                details: PaymentDetails::Ln {
                    data: LnPaymentDetails {
                        payment_hash: hex::encode(payment_hash_with_rev_swap_info),
                        label: "label".to_string(),
                        destination_pubkey: "pubkey".to_string(),
                        payment_preimage: hex::encode(rev_swap_preimage),
                        keysend: false,
                        bolt11: "swap_bolt11".to_string(),
                        lnurl_success_action: None,
                        lnurl_metadata: None,
                        lnurl_pay_domain: None,
                        lnurl_pay_comment: None,
                        ln_address: None,
                        lnurl_withdraw_endpoint: None,
                        swap_info: None,
                        reverse_swap_info: Some(rev_swap_info.clone()),
                        pending_expiration_block: None,
                        open_channel_bolt11: None,
                    },
                },
                metadata: None,
            },
            Payment {
                id: payment_hash_with_lnurl_domain.to_string(),
                payment_type: PaymentType::Sent,
                payment_time: 998,
                amount_msat: 100,
                fee_msat: 20,
                status: PaymentStatus::Complete,
                error: None,
                description: None,
                details: PaymentDetails::Ln {
                    data: LnPaymentDetails {
                        payment_hash: payment_hash_with_lnurl_domain.to_string(),
                        label: "label".to_string(),
                        destination_pubkey: "pubey".to_string(),
                        payment_preimage: "payment_preimage".to_string(),
                        keysend: true,
                        bolt11: "bolt11".to_string(),
                        lnurl_success_action: None,
                        lnurl_pay_domain: Some(test_lnurl_pay_domain.to_string()),
                        lnurl_pay_comment: Some(test_lnurl_pay_comment.to_string()),
                        lnurl_metadata: Some(lnurl_metadata.to_string()),
                        ln_address: None,
                        lnurl_withdraw_endpoint: None,
                        swap_info: None,
                        reverse_swap_info: None,
                        pending_expiration_block: None,
                        open_channel_bolt11: None,
                    },
                },
                metadata: None,
            },
        ];
        let failed_txs = [Payment {
            id: "125".to_string(),
            payment_type: PaymentType::Sent,
            payment_time: 2000,
            amount_msat: 1000,
            fee_msat: 0,
            status: PaymentStatus::Failed,
            error: None,
            description: Some("desc".to_string()),
            details: PaymentDetails::Ln {
                data: LnPaymentDetails {
                    payment_hash: "125".to_string(),
                    label: "label".to_string(),
                    destination_pubkey: "pubey".to_string(),
                    payment_preimage: "4444".to_string(),
                    keysend: true,
                    bolt11: "bolt11".to_string(),
                    lnurl_success_action: None,
                    lnurl_pay_domain: None,
                    lnurl_pay_comment: None,
                    lnurl_metadata: None,
                    ln_address: None,
                    lnurl_withdraw_endpoint: None,
                    swap_info: None,
                    reverse_swap_info: None,
                    pending_expiration_block: None,
                    open_channel_bolt11: None,
                },
            },
            metadata: None,
        }];
        let storage = SqliteStorage::new(test_utils::create_test_sql_dir());
        storage.init()?;
        storage.insert_or_update_payments(&txs, false)?;
        storage.insert_or_update_payments(&failed_txs, false)?;
        storage.insert_payment_external_info(
            payment_hash_with_lnurl_success_action,
            PaymentExternalInfo {
                lnurl_pay_success_action: Some(sa.clone()),
                lnurl_pay_domain: None,
                lnurl_pay_comment: None,
                lnurl_metadata: Some(lnurl_metadata.to_string()),
                ln_address: Some(test_ln_address.to_string()),
                lnurl_withdraw_endpoint: None,
                attempted_amount_msat: None,
                attempted_error: None,
            },
        )?;
        storage.insert_payment_external_info(
            payment_hash_with_lnurl_withdraw,
            PaymentExternalInfo {
                lnurl_pay_success_action: None,
                lnurl_pay_domain: None,
                lnurl_pay_comment: None,
                lnurl_metadata: None,
                ln_address: None,
                lnurl_withdraw_endpoint: Some(lnurl_withdraw_url.to_string()),
                attempted_amount_msat: None,
                attempted_error: None,
            },
        )?;
        storage.insert_swap(&swap_info)?;
        storage.update_swap_bolt11(
            swap_info.bitcoin_address.clone(),
            swap_info.bolt11.clone().unwrap(),
        )?;
        storage.insert_payment_external_info(
            payment_hash_with_lnurl_domain,
            PaymentExternalInfo {
                lnurl_pay_success_action: None,
                lnurl_pay_domain: Some(test_lnurl_pay_domain.to_string()),
                lnurl_pay_comment: Some(test_lnurl_pay_comment.to_string()),
                lnurl_metadata: Some(lnurl_metadata.to_string()),
                ln_address: None,
                lnurl_withdraw_endpoint: None,
                attempted_amount_msat: None,
                attempted_error: None,
            },
        )?;
        storage.insert_reverse_swap(&full_ref_swap_info)?;
        storage
            .update_reverse_swap_status("rev_swap_id", &ReverseSwapStatus::CompletedConfirmed)?;
        storage.update_reverse_swap_lockup_txid("rev_swap_id", Some("lockup_txid".to_string()))?;
        storage.update_reverse_swap_claim_txid("rev_swap_id", Some("claim_txid".to_string()))?;

        // retrieve all
        let retrieve_txs = storage.list_payments(ListPaymentsRequest::default())?;
        assert_eq!(retrieve_txs.len(), 5);
        assert_eq!(retrieve_txs, txs);

        //test only sent
        let retrieve_txs = storage.list_payments(ListPaymentsRequest {
            filters: Some(vec![
                PaymentTypeFilter::Sent,
                PaymentTypeFilter::ClosedChannel,
            ]),
            ..Default::default()
        })?;
        assert_eq!(retrieve_txs.len(), 3);
        assert_eq!(retrieve_txs[0], txs[0]);
        assert_eq!(retrieve_txs[1], txs[3]);
        assert!(
            matches!( &retrieve_txs[0].details, PaymentDetails::Ln {data: LnPaymentDetails {lnurl_success_action, ..}} if lnurl_success_action == &Some(sa))
        );
        assert!(
            matches!( &retrieve_txs[0].details, PaymentDetails::Ln {data: LnPaymentDetails {lnurl_pay_domain, ln_address, ..}} if lnurl_pay_domain.is_none() && ln_address == &Some(test_ln_address.to_string()))
        );
        assert!(
            matches!( &retrieve_txs[2].details, PaymentDetails::Ln {data: LnPaymentDetails {lnurl_pay_domain, ln_address, ..}} if lnurl_pay_domain == &Some(test_lnurl_pay_domain.to_string()) && ln_address.is_none())
        );
        assert!(
            matches!( &retrieve_txs[1].details, PaymentDetails::Ln {data: LnPaymentDetails {reverse_swap_info: rev_swap, ..}} if rev_swap == &Some(rev_swap_info))
        );

        //test only received
        let retrieve_txs = storage.list_payments(ListPaymentsRequest {
            filters: Some(vec![PaymentTypeFilter::Received]),
            ..Default::default()
        })?;
        assert_eq!(retrieve_txs.len(), 2);
        assert_eq!(retrieve_txs[0], txs[1]);
        assert_eq!(retrieve_txs[1], txs[2]);
        assert!(
            matches!( &retrieve_txs[1].details, PaymentDetails::Ln {data: LnPaymentDetails {swap_info: swap, ..}} if swap == &Some(swap_info))
        );

        storage.insert_or_update_payments(&txs, false)?;
        let retrieve_txs = storage.list_payments(ListPaymentsRequest::default())?;
        assert_eq!(retrieve_txs.len(), 5);
        assert_eq!(retrieve_txs, txs);

        storage.insert_open_channel_payment_info("123", 150, "")?;
        let retrieve_txs = storage.list_payments(ListPaymentsRequest::default())?;
        assert_eq!(retrieve_txs[0].fee_msat, 50);

        // test all with failures
        let retrieve_txs = storage.list_payments(ListPaymentsRequest {
            include_failures: Some(true),
            ..Default::default()
        })?;
        assert_eq!(retrieve_txs.len(), 6);

        // test sent with failures
        let retrieve_txs = storage.list_payments(ListPaymentsRequest {
            filters: Some(vec![
                PaymentTypeFilter::Sent,
                PaymentTypeFilter::ClosedChannel,
            ]),
            include_failures: Some(true),
            ..Default::default()
        })?;
        assert_eq!(retrieve_txs.len(), 4);

        // test limit
        let retrieve_txs = storage.list_payments(ListPaymentsRequest {
            include_failures: Some(false),
            limit: Some(1),
            ..Default::default()
        })?;
        assert_eq!(retrieve_txs.len(), 1);

        // test offset
        let retrieve_txs = storage.list_payments(ListPaymentsRequest {
            include_failures: Some(false),
            offset: Some(1),
            limit: Some(1),
            ..Default::default()
        })?;
        assert_eq!(retrieve_txs.len(), 1);
        assert_eq!(retrieve_txs[0].id, payment_hash_with_lnurl_withdraw);

        // test json metadata validation
        assert!(storage
            .set_payment_external_metadata(
                payment_hash_with_lnurl_withdraw.to_string(),
                r#"{ "malformed: true }"#.to_string()
            )
            .is_err());

        // test metadata set and filter
        let test_json = r#"{"supportsBoolean":true,"supportsInt":10,"supportsString":"supports string","supportsNested":{"value":[1,2]}}"#;
        let test_json_filters = Some(vec![
            MetadataFilter {
                json_path: "supportsBoolean".to_string(),
                json_value: "true".to_string(),
            },
            MetadataFilter {
                json_path: "supportsInt".to_string(),
                json_value: "10".to_string(),
            },
            MetadataFilter {
                json_path: "supportsString".to_string(),
                json_value: r#""supports string""#.to_string(),
            },
            MetadataFilter {
                json_path: "supportsNested.value".to_string(),
                json_value: "[1,2]".to_string(),
            },
        ]);

        storage.set_payment_external_metadata(
            payment_hash_with_lnurl_withdraw.to_string(),
            test_json.to_string(),
        )?;

        let retrieve_txs = storage.list_payments(ListPaymentsRequest {
            metadata_filters: test_json_filters,
            ..Default::default()
        })?;
        assert_eq!(retrieve_txs.len(), 1);
        assert_eq!(retrieve_txs[0].id, payment_hash_with_lnurl_withdraw);
        assert_eq!(retrieve_txs[0].metadata, Some(test_json.to_string()),);

        // test open_channel_bolt11
        storage.insert_open_channel_payment_info(
            payment_hash_with_lnurl_withdraw,
            150,
            "original_invoice",
        )?;

        let open_channel_bolt11 = storage
            .get_open_channel_bolt11_by_hash(payment_hash_with_lnurl_withdraw)?
            .unwrap();
        assert_eq!(open_channel_bolt11, "original_invoice");

        let open_channel_bolt11 = storage.get_open_channel_bolt11_by_hash("non existing hash")?;
        assert_eq!(open_channel_bolt11, None);

        let retrieve_txs = storage.list_payments(ListPaymentsRequest {
            filters: Some(vec![PaymentTypeFilter::Received]),
            ..Default::default()
        })?;

        let filtered_txs: Vec<&Payment> = retrieve_txs
            .iter()
            .filter(|p| {
                if let PaymentDetails::Ln { data } = &p.details {
                    return data.open_channel_bolt11 == Some("original_invoice".to_string());
                }
                false
            })
            .collect();

        assert_eq!(filtered_txs.len(), 1);
        assert_eq!(filtered_txs[0].id, payment_hash_with_lnurl_withdraw);
        assert!(matches!(filtered_txs[0].details, PaymentDetails::Ln { .. }));
        if let PaymentDetails::Ln { data } = &filtered_txs[0].details {
            assert_eq!(
                data.open_channel_bolt11,
                Some("original_invoice".to_string())
            );
        }

        Ok(())
    }
}
