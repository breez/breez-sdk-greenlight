use rusqlite::named_params;
use strum_macros::FromRepr;

use super::{db::SqliteStorage, error::PersistResult};

#[derive(FromRepr)]
#[repr(i32)]
pub(crate) enum SendPayStatus {
    Pending = 0,
    Failed = 1,
    Complete = 2,
}

pub(crate) struct SendPay {
    pub created_index: u64,
    pub updated_index: Option<u64>,
    pub groupid: u64,
    pub partid: Option<u64>,
    pub payment_hash: Vec<u8>,
    pub status: SendPayStatus,
    pub amount_msat: Option<u64>,
    pub destination: Option<Vec<u8>>,
    pub created_at: u64,
    pub amount_sent_msat: Option<u64>,
    pub label: Option<String>,
    pub bolt11: Option<String>,
    pub description: Option<String>,
    pub bolt12: Option<String>,
    pub payment_preimage: Option<Vec<u8>>,
    pub erroronion: Option<Vec<u8>>,
}

impl SqliteStorage {
    pub(crate) fn insert_send_pays(&self, send_pays: &[SendPay]) -> PersistResult<()> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            r#"INSERT OR REPLACE INTO send_pays (created_index, updated_index, 
                groupid, partid, payment_hash, status, amount_msat, 
                destination, created_at, amount_sent_msat, label, bolt11, 
                description, bolt12, payment_preimage, erroronion) 
                VALUES (:created_index, :updated_index, 
                    :groupid, :partid, :payment_hash, :status, :amount_msat, 
                    :destination, :created_at, :amount_sent_msat, :label, :bolt11, 
                    :description, :bolt12, :payment_preimage, :erroronion)"#,
        )?;
        for send_pay in send_pays {
            let status: i32 = match send_pay.status {
                SendPayStatus::Pending => 0,
                SendPayStatus::Failed => 1,
                SendPayStatus::Complete => 2,
            };
            stmt.execute(named_params! {
                ":created_index": send_pay.created_index,
                ":updated_index": send_pay.updated_index,
                ":groupid": send_pay.groupid,
                ":partid": send_pay.partid,
                ":payment_hash": send_pay.payment_hash,
                ":status": status,
                ":amount_msat": send_pay.amount_msat,
                ":destination": send_pay.destination,
                ":created_at": send_pay.created_at,
                ":amount_sent_msat": send_pay.amount_sent_msat,
                ":label": send_pay.label,
                ":bolt11": send_pay.bolt11,
                ":description": send_pay.description,
                ":bolt12": send_pay.bolt12,
                ":payment_preimage": send_pay.payment_preimage,
                ":erroronion": send_pay.erroronion,
            })?;
        }

        Ok(())
    }

    pub(crate) fn list_send_pays(&self, hashes: &[Vec<u8>]) -> PersistResult<Vec<SendPay>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            r#"SELECT created_index, updated_index, groupid, partid, 
               payment_hash, status, amount_msat, destination, created_at, 
               amount_sent_msat, label, bolt11, description, bolt12, 
               payment_preimage, erroronion
               FROM send_pays
               WHERE payment_hash = :payment_hash"#,
        )?;
        let mut send_pays = Vec::new();
        for hash in hashes {
            let rows: Vec<_> = stmt
                .query_map(
                    named_params! {
                        ":payment_hash": hash
                    },
                    |row| {
                        let status: i32 = row.get("status")?;
                        Ok(SendPay {
                            amount_msat: row.get("amount_msat")?,
                            amount_sent_msat: row.get("amount_sent_msat")?,
                            created_index: row.get("created_index")?,
                            updated_index: row.get("updated_index")?,
                            groupid: row.get("groupid")?,
                            partid: row.get("partid")?,
                            payment_hash: row.get("payment_hash")?,
                            status: SendPayStatus::from_repr(status)
                                .ok_or(rusqlite::Error::IntegralValueOutOfRange(5, 2))?,
                            destination: row.get("destination")?,
                            created_at: row.get("created_at")?,
                            label: row.get("label")?,
                            bolt11: row.get("bolt11")?,
                            description: row.get("description")?,
                            bolt12: row.get("bolt12")?,
                            payment_preimage: row.get("payment_preimage")?,
                            erroronion: row.get("erroronion")?,
                        })
                    },
                )?
                .collect::<Result<Vec<SendPay>, _>>()?;
            for row in rows {
                send_pays.push(row);
            }
        }
        Ok(send_pays)
    }
}
