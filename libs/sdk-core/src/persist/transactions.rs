use super::db::SqliteStorage;
use super::error::{PersistError, PersistResult};
use crate::lnurl::pay::model::SuccessActionProcessed;
use crate::{ensure_sdk, models::*};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::{Row, params_from_iter};
use rusqlite::{named_params, params, OptionalExtension};
use std::collections::HashSet;

use serde_json::{Map, Value};
use std::str::FromStr;

const METADATA_MAX_LEN: usize = 1000;

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
        delete_pending: bool,
    ) -> PersistResult<()> {
        if delete_pending {
            let deleted = self.delete_pending_lightning_payments()?;
            debug!("Deleted {deleted} pending payments");
        }

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
           details
        )
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8)
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
            ))?;
        }
        Ok(())
    }

    /// Deletes any pending sent payments and returns the deleted count
    fn delete_pending_lightning_payments(&self) -> PersistResult<usize> {
        Ok(self.get_connection()?.execute(
            "DELETE FROM payments WHERE payment_type = ?1 AND status = ?2",
            params![PaymentType::Sent.to_string(), PaymentStatus::Pending],
        )?)
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
           lnurl_metadata,
           ln_address,
           lnurl_withdraw_endpoint,
           attempted_amount_msat,
           attempted_error
         )
         VALUES (?1,?2,?3,?4,?5,?6,?7)
        ",
        )?;

        _ = prep_statement.execute((
            payment_hash,
            payment_external_info.lnurl_pay_success_action,
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
            PersistError::Generic(anyhow::anyhow!(
                "Max metadata size ({} characters) has been exceeded",
                METADATA_MAX_LEN
            ))
        );

        let _ = serde_json::from_str::<Map<String, Value>>(&new_metadata)?;

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
    ) -> PersistResult<()> {
        let con = self.get_connection()?;
        let mut prep_statement = con.prepare(
            "
        INSERT INTO sync.open_channel_payment_info (
          payment_hash,
          payer_amount_msat
        )
        VALUES (?1,?2)
       ",
        )?;

        _ = prep_statement.execute((payment_hash, payer_amount_msat))?;

        Ok(())
    }

    pub fn last_payment_timestamp(&self) -> PersistResult<u64> {
        Ok(self.get_connection()?.query_row(
            "SELECT max(payment_time) FROM payments where status != ?1",
            params![PaymentStatus::Pending],
            |row| row.get(0),
        )?)
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
        let mut stmt = con.prepare(
            format!(
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
             m.metadata
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
            {where_clause} ORDER BY payment_time DESC
            LIMIT {limit}
            OFFSET {offset}
          "
            )
            .as_str(),
        )?;

        let mut params: Vec<String> = vec![];
        if let Some(metadata_filters) = &req.metadata_filters {
            metadata_filters.iter().for_each(|PaymentMetadataFilter { search_path, search_value }| {
                params.push(search_path.clone());
                params.push(search_value.clone());
            })
        }

        let vec: Vec<Payment> = stmt
            .query_map(
                params_from_iter(params),
                |row| self.sql_row_to_payment(row))?
            .map(|i| i.unwrap())
            .collect();

        Ok(vec)
    }

    /// This queries a single payment by hash, which may be pending, completed or failed.
    ///
    /// To lookup a completed payment by hash, use [Self::get_completed_payment_by_hash]
    ///
    /// To query all payments, see [Self::list_payments]
    pub(crate) fn get_payment_by_hash(&self, hash: &String) -> PersistResult<Option<Payment>> {
        Ok(self
            .get_connection()?
            .query_row(
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
                 m.metadata
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
                WHERE
                 id = ?1",
                [hash],
                |row| self.sql_row_to_payment(row),
            )
            .optional()?)
    }

    /// Looks up a completed payment by hash.
    ///
    /// To include pending or failed payments in the lookup as well, use [Self::get_payment_by_hash]
    pub(crate) fn get_completed_payment_by_hash(
        &self,
        hash: &String,
    ) -> PersistResult<Option<Payment>> {
        let res = self
            .get_payment_by_hash(hash)?
            .filter(|p| p.status == PaymentStatus::Complete);
        Ok(res)
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
            metadata: row.get(15)?,
        };

        if let PaymentDetails::Ln { ref mut data } = payment.details {
            data.lnurl_success_action = row.get(8)?;
            data.lnurl_metadata = row.get(9)?;
            data.ln_address = row.get(10)?;
            data.lnurl_withdraw_endpoint = row.get(11)?;
            data.swap_info = self
                .get_swap_info_by_hash(&hex::decode(&payment.id).unwrap_or_default())
                .unwrap_or(None);
        }

        // In case we have a record of the open channel fee, let's use it.
        let payer_amount_msat: Option<u64> = row.get(14)?;
        if let Some(payer_amount) = payer_amount_msat {
            payment.fee_msat = payer_amount - amount_msat;
        }

        Ok(payment)
    }
}

fn filter_to_where_clause(
    type_filters: Option<Vec<PaymentTypeFilter>>,
    metadata_filters: &Option<Vec<PaymentMetadataFilter>>,
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
                    .map(|t| format!("'{}'", t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    if let Some(filters) = metadata_filters {
        filters.iter().for_each(|_| {
            where_clause.push("metadata->'$.?'='?'".to_string());
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

impl FromSql for SuccessActionProcessed {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        serde_json::from_str(value.as_str()?).map_err(|_| FromSqlError::InvalidType)
    }
}

impl ToSql for SuccessActionProcessed {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(
            serde_json::to_string(&self).map_err(|_| FromSqlError::InvalidType)?,
        ))
    }
}

#[test]
fn test_ln_transactions() -> PersistResult<(), Box<dyn std::error::Error>> {
    use crate::lnurl::pay::model::MessageSuccessActionData;
    use crate::lnurl::pay::model::SuccessActionProcessed;
    use crate::models::{LnPaymentDetails, Payment, PaymentDetails};
    use crate::persist::test_utils;

    let lnurl_metadata = "{'key': 'sample-metadata-val'}";
    let test_ln_address = "test@ln.adddress.com";
    let sa = SuccessActionProcessed::Message {
        data: MessageSuccessActionData {
            message: "test message".into(),
        },
    };

    let payment_hash_with_lnurl_success_action = "123";
    let payment_hash_with_lnurl_withdraw = "124";
    let payment_hash_with_swap_info: Vec<u8> = vec![234, 12, 53, 124];
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
        paid_sats: 50,
        confirmed_sats: 50,
        unconfirmed_sats: 0,
        status: SwapStatus::Expired,
        refund_tx_ids: vec![],
        unconfirmed_tx_ids: vec![],
        confirmed_tx_ids: vec![],
        min_allowed_deposit: 5_000,
        max_allowed_deposit: 1_000_000,
        last_redeem_error: None,
        channel_opening_fees: Some(OpeningFeeParams {
            min_msat: 5_000_000,
            proportional: 50,
            valid_until: "date".to_string(),
            max_idle_time: 12345,
            max_client_to_self_delay: 234,
            promise: "promise".to_string(),
        }),
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
                    payment_preimage: "payment_preimage".to_string(),
                    keysend: true,
                    bolt11: "bolt11".to_string(),
                    lnurl_success_action: Some(sa.clone()),
                    lnurl_metadata: Some(lnurl_metadata.to_string()),
                    ln_address: Some(test_ln_address.to_string()),
                    lnurl_withdraw_endpoint: None,
                    swap_info: None,
                    pending_expiration_block: None,
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
                    payment_preimage: "payment_preimage".to_string(),
                    keysend: true,
                    bolt11: "bolt11".to_string(),
                    lnurl_success_action: None,
                    lnurl_metadata: None,
                    ln_address: None,
                    lnurl_withdraw_endpoint: Some(lnurl_withdraw_url.to_string()),
                    swap_info: None,
                    pending_expiration_block: None,
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
                    payment_preimage: "payment_preimage".to_string(),
                    keysend: false,
                    bolt11: "swap_bolt11".to_string(),
                    lnurl_success_action: None,
                    lnurl_metadata: None,
                    ln_address: None,
                    lnurl_withdraw_endpoint: None,
                    swap_info: Some(swap_info.clone()),
                    pending_expiration_block: None,
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
                payment_preimage: "payment_preimage".to_string(),
                keysend: true,
                bolt11: "bolt11".to_string(),
                lnurl_success_action: None,
                lnurl_metadata: None,
                ln_address: None,
                lnurl_withdraw_endpoint: None,
                swap_info: None,
                pending_expiration_block: None,
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
            lnurl_metadata: None,
            ln_address: None,
            lnurl_withdraw_endpoint: Some(lnurl_withdraw_url.to_string()),
            attempted_amount_msat: None,
            attempted_error: None,
        },
    )?;
    storage.insert_swap(swap_info.clone())?;
    storage.update_swap_bolt11(
        swap_info.bitcoin_address.clone(),
        swap_info.bolt11.clone().unwrap(),
    )?;

    // retrieve all
    let retrieve_txs = storage.list_payments(ListPaymentsRequest::default())?;
    assert_eq!(retrieve_txs.len(), 3);
    assert_eq!(retrieve_txs, txs);

    //test only sent
    let retrieve_txs = storage.list_payments(ListPaymentsRequest {
        filters: Some(vec![
            PaymentTypeFilter::Sent,
            PaymentTypeFilter::ClosedChannel,
        ]),
        ..Default::default()
    })?;
    assert_eq!(retrieve_txs.len(), 1);
    assert_eq!(retrieve_txs[0], txs[0]);
    assert!(
        matches!( &retrieve_txs[0].details, PaymentDetails::Ln {data: LnPaymentDetails {lnurl_success_action, ..}} if lnurl_success_action == &Some(sa))
    );
    assert!(
        matches!( &retrieve_txs[0].details, PaymentDetails::Ln {data: LnPaymentDetails {ln_address, ..}} if ln_address == &Some(test_ln_address.to_string()))
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

    let max_ts = storage.last_payment_timestamp()?;
    assert_eq!(max_ts, 2000);

    storage.insert_or_update_payments(&txs, false)?;
    let retrieve_txs = storage.list_payments(ListPaymentsRequest::default())?;
    assert_eq!(retrieve_txs.len(), 3);
    assert_eq!(retrieve_txs, txs);

    storage.insert_open_channel_payment_info("123", 150)?;
    let retrieve_txs = storage.list_payments(ListPaymentsRequest::default())?;
    assert_eq!(retrieve_txs[0].fee_msat, 50);

    // test all with failures
    let retrieve_txs = storage.list_payments(ListPaymentsRequest {
        include_failures: Some(true),
        ..Default::default()
    })?;
    assert_eq!(retrieve_txs.len(), 4);

    // test sent with failures
    let retrieve_txs = storage.list_payments(ListPaymentsRequest {
        filters: Some(vec![
            PaymentTypeFilter::Sent,
            PaymentTypeFilter::ClosedChannel,
        ]),
        include_failures: Some(true),
        ..Default::default()
    })?;
    assert_eq!(retrieve_txs.len(), 2);

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
        PaymentMetadataFilter {
            search_path: "supportsBoolean".to_string(),
            search_value: "true".to_string(),
        },
        PaymentMetadataFilter {
            search_path: "supportsInt".to_string(),
            search_value: "10".to_string(),
        },
        PaymentMetadataFilter {
            search_path: "supportsString".to_string(),
            search_value: r#""supports string""#.to_string(),
        },
        PaymentMetadataFilter {
            search_path: "supportsNested.value".to_string(),
            search_value: "[1,2]".to_string(),
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

    Ok(())
}
