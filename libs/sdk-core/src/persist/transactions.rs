use super::db::SqliteStorage;
use super::error::PersistResult;
use crate::lnurl::pay::model::SuccessActionProcessed;
use crate::models::*;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::Row;
use rusqlite::{params, OptionalExtension};
use std::collections::HashSet;

use std::str::FromStr;

impl SqliteStorage {
    /// Inserts payments into the payments table. These can be pending, completed and failed payments. Before
    /// persisting, it automatically deletes previously pending payments
    ///
    /// Note that, if a payment has details of type [LnPaymentDetails] which contain a [SuccessActionProcessed],
    /// then the [LnPaymentDetails] will NOT be persisted. In that case, the [SuccessActionProcessed]
    /// can be inserted separately via [SqliteStorage::insert_lnurl_payment_external_info].
    pub fn insert_or_update_payments(&self, transactions: &[Payment]) -> PersistResult<()> {
        let deleted = self.delete_pending_lightning_payments()?;
        debug!("Deleted {deleted} pending payments");

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

    /// Inserts LNURL-related metadata associated with this payment
    pub fn insert_lnurl_payment_external_info(
        &self,
        payment_hash: &str,
        lnurl_pay_success_action: Option<&SuccessActionProcessed>,
        lnurl_metadata: Option<String>,
        ln_address: Option<String>,
        lnurl_withdraw_endpoint: Option<String>,
    ) -> PersistResult<()> {
        let con = self.get_connection()?;
        let mut prep_statement = con.prepare(
            "
         INSERT OR REPLACE INTO sync.payments_external_info (
           payment_id,
           lnurl_success_action,
           lnurl_metadata,
           ln_address,
           lnurl_withdraw_endpoint
         )
         VALUES (?1,?2,?3,?4,?5)
        ",
        )?;

        _ = prep_statement.execute((
            payment_hash,
            &lnurl_pay_success_action,
            lnurl_metadata,
            ln_address,
            lnurl_withdraw_endpoint,
        ))?;

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
             o.payer_amount_msat
            FROM payments p
            LEFT JOIN sync.payments_external_info e
            ON
             p.id = e.payment_id
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

        let vec: Vec<Payment> = stmt
            .query_map([], |row| self.sql_row_to_payment(row))?
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
                 o.payer_amount_msat
                FROM payments p
                LEFT JOIN sync.payments_external_info e
                ON
                 p.id = e.payment_id
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
        let mut payment = Payment {
            id: row.get(0)?,
            payment_type: PaymentType::from_str(payment_type_str.as_str()).unwrap(),
            payment_time: row.get(2)?,
            amount_msat,
            fee_msat: row.get(4)?,
            status: row.get(5)?,
            description: row.get(6)?,
            details: row.get(7)?,
        };

        if let PaymentDetails::Ln { ref mut data } = payment.details {
            data.lnurl_success_action = row.get(8)?;
            data.lnurl_metadata = row.get(9)?;
            data.ln_address = row.get(10)?;
            data.lnurl_withdraw_endpoint = row.get(11)?;
        }

        // In case we have a record of the open channel fee, let's use it.
        let payer_amount_msat: Option<u64> = row.get(12)?;
        if let Some(payer_amount) = payer_amount_msat {
            payment.fee_msat = payer_amount - amount_msat;
        }

        Ok(payment)
    }
}

fn filter_to_where_clause(
    type_filters: Option<Vec<PaymentTypeFilter>>,
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
    let lnurl_withdraw_url = "https://test.lnurl.withdraw.link";
    let txs = [
        Payment {
            id: payment_hash_with_lnurl_success_action.to_string(),
            payment_type: PaymentType::Sent,
            payment_time: 1001,
            amount_msat: 100,
            fee_msat: 20,
            status: PaymentStatus::Complete,
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
                },
            },
        },
        Payment {
            id: payment_hash_with_lnurl_withdraw.to_string(),
            payment_type: PaymentType::Received,
            payment_time: 1000,
            amount_msat: 100,
            fee_msat: 20,
            status: PaymentStatus::Complete,
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
                },
            },
        },
    ];
    let failed_txs = [Payment {
        id: "125".to_string(),
        payment_type: PaymentType::Sent,
        payment_time: 2000,
        amount_msat: 1000,
        fee_msat: 0,
        status: PaymentStatus::Failed,
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
            },
        },
    }];
    let storage = SqliteStorage::new(test_utils::create_test_sql_dir());
    storage.init()?;
    storage.insert_or_update_payments(&txs)?;
    storage.insert_or_update_payments(&failed_txs)?;
    storage.insert_lnurl_payment_external_info(
        payment_hash_with_lnurl_success_action,
        Some(&sa),
        Some(lnurl_metadata.to_string()),
        Some(test_ln_address.to_string()),
        None,
    )?;
    storage.insert_lnurl_payment_external_info(
        payment_hash_with_lnurl_withdraw,
        None,
        None,
        None,
        Some(lnurl_withdraw_url.to_string()),
    )?;

    // retrieve all
    let retrieve_txs = storage.list_payments(ListPaymentsRequest::default())?;
    assert_eq!(retrieve_txs.len(), 2);
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
    assert_eq!(retrieve_txs.len(), 1);
    assert_eq!(retrieve_txs[0], txs[1]);

    let max_ts = storage.last_payment_timestamp()?;
    assert_eq!(max_ts, 2000);

    storage.insert_or_update_payments(&txs)?;
    let retrieve_txs = storage.list_payments(ListPaymentsRequest::default())?;
    assert_eq!(retrieve_txs.len(), 2);
    assert_eq!(retrieve_txs, txs);

    storage.insert_open_channel_payment_info("123", 150)?;
    let retrieve_txs = storage.list_payments(ListPaymentsRequest::default())?;
    assert_eq!(retrieve_txs[0].fee_msat, 50);

    // test all with failures
    let retrieve_txs = storage.list_payments(ListPaymentsRequest {
        include_failures: Some(true),
        ..Default::default()
    })?;
    assert_eq!(retrieve_txs.len(), 3);

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

    Ok(())
}
