use super::db::SqliteStorage;
use crate::models::PaymentTypeFilter;
use crate::models::{Payment, PaymentDetails};
use crate::LnPaymentDetails;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput};

impl SqliteStorage {
    pub fn insert_payments(&self, transactions: &[Payment]) -> Result<()> {
        let con = self.get_connection()?;
        let mut prep_statment = con.prepare(
            "
         INSERT INTO payments (
           id,
           payment_type,                 
           payment_time,                                  
           amount_msat, 
           fee_msat,                 
           pending,
           description,
           details
         )
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8)
        ",
        )?;

        for ln_tx in transactions {
            _ = prep_statment.execute((
                &ln_tx.id,
                &ln_tx.payment_type,
                &ln_tx.payment_time,
                &ln_tx.amount_msat,
                &ln_tx.fee_msat,
                &ln_tx.pending,
                &ln_tx.description,
                &ln_tx.details,
            ))?;
        }
        Ok(())
    }

    pub fn last_payment_timestamp(&self) -> Result<i64> {
        self.get_connection()?
            .query_row("SELECT max(payment_time) FROM payments", [], |row| {
                row.get(0)
            })
            .map_err(anyhow::Error::msg)
    }

    pub fn list_payments(
        &self,
        type_filter: PaymentTypeFilter,
        from_timestamp: Option<i64>,
        to_timestamp: Option<i64>,
    ) -> Result<Vec<Payment>> {
        let where_clause = filter_to_where_clause(type_filter, from_timestamp, to_timestamp);
        let con = self.get_connection()?;
        let mut stmt = con.prepare(
            format!(
                "
            SELECT 
             id, 
             payment_type, 
             payment_time, 
             amount_msat, 
             fee_msat, 
             pending, 
             description,
             details
            FROM payments            
            {where_clause} ORDER BY payment_time DESC
          "
            )
            .as_str(),
        )?;

        let vec: Vec<Payment> = stmt
            .query_map([], |row| {
                Ok(Payment {
                    id: row.get(0)?,
                    payment_type: row.get(1)?,
                    payment_time: row.get(2)?,
                    amount_msat: row.get(3)?,
                    fee_msat: row.get(4)?,
                    pending: row.get(5)?,
                    description: row.get(6)?,
                    details: row.get(7)?,
                })
            })?
            .map(|i| i.unwrap())
            .collect();

        Ok(vec)
    }
}

fn filter_to_where_clause(
    type_filter: PaymentTypeFilter,
    from_timestamp: Option<i64>,
    to_timestamp: Option<i64>,
) -> String {
    let mut where_clause: Vec<String> = Vec::new();

    match from_timestamp {
        Some(t) => {
            where_clause.push(format!("payment_time >= {t}"));
        }
        None => (),
    };
    match to_timestamp {
        Some(t) => {
            where_clause.push(format!("payment_time <= {t}"));
        }
        None => (),
    };
    match type_filter {
        PaymentTypeFilter::Sent => {
            where_clause.push("payment_type = 'sent' ".to_string());
        }
        PaymentTypeFilter::Received => {
            where_clause.push("payment_type = 'received' ".to_string());
        }
        PaymentTypeFilter::All => (),
    }

    let mut where_clause_str = String::new();
    if where_clause.len() > 0 {
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

#[test]
fn test_ln_transactions() {
    use crate::models::{ClosesChannelPaymentDetails, LnPaymentDetails, Payment, PaymentDetails};
    use crate::persist::test_utils;

    let txs = [
        Payment {
            id: "123".to_string(),
            payment_type: "sent".to_string(),
            payment_time: 1001,
            amount_msat: 100,
            fee_msat: 20,
            pending: false,
            description: None,
            details: PaymentDetails::Ln {
                data: LnPaymentDetails {
                    payment_hash: "123".to_string(),
                    label: "label".to_string(),
                    destination_pubkey: "pubey".to_string(),
                    payment_preimage: "payment_preimage".to_string(),
                    keysend: true,
                    bolt11: "bolt11".to_string(),
                },
            },
        },
        Payment {
            id: "124".to_string(),
            payment_type: "received".to_string(),
            payment_time: 1000,
            amount_msat: 100,
            fee_msat: 20,
            pending: false,
            description: Some("desc".to_string()),
            details: PaymentDetails::Ln {
                data: LnPaymentDetails {
                    payment_hash: "124".to_string(),
                    label: "label".to_string(),
                    destination_pubkey: "pubey".to_string(),
                    payment_preimage: "payment_preimage".to_string(),
                    keysend: true,
                    bolt11: "bolt11".to_string(),
                },
            },
        },
    ];
    let storage =
        SqliteStorage::from_file(test_utils::create_test_sql_file("transactions".to_string()));
    storage.init().unwrap();
    storage.insert_payments(&txs).unwrap();

    // retrieve all
    let retrieve_txs = storage
        .list_payments(PaymentTypeFilter::All, None, None)
        .unwrap();
    assert_eq!(retrieve_txs.len(), 2);
    assert_eq!(retrieve_txs, txs);

    //test only sent
    let retrieve_txs = storage
        .list_payments(PaymentTypeFilter::Sent, None, None)
        .unwrap();
    assert_eq!(retrieve_txs.len(), 1);
    assert_eq!(retrieve_txs[0], txs[0]);

    //test only received
    let retrieve_txs = storage
        .list_payments(PaymentTypeFilter::Received, None, None)
        .unwrap();
    assert_eq!(retrieve_txs.len(), 1);
    assert_eq!(retrieve_txs[0], txs[1]);

    let max_ts = storage.last_payment_timestamp().unwrap();
    assert_eq!(max_ts, 1001);
}
