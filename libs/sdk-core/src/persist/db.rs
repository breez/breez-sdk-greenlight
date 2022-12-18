use anyhow::{anyhow, Result};
use rusqlite::{
    types::{FromSql, FromSqlError, ToSqlOutput},
    Connection, ToSql,
};
use rusqlite_migration::{Migrations, M};

pub struct SqliteStorage {
    file: String,
}

impl SqliteStorage {
    pub fn from_file(file: String) -> SqliteStorage {
        SqliteStorage { file }
    }

    pub fn init(&self) -> Result<()> {
        let migrations = Migrations::new(vec![M::up(
            "CREATE TABLE IF NOT EXISTS payments (
               payment_type TEXT NOT NULL check( payment_type in('sent', 'received')),
               payment_hash TEXT NOT NULL PRIMARY KEY,
               payment_time INTEGER NOT NULL,
               label TEXT,
               destination_pubkey TEXT NOT NULL,
               amount_msats INTEGER NOT NULL,
               fee_msat INTEGER NOT NULL,
               payment_preimage TEXT,
               keysend INTEGER NOT NULL,                  
               bolt11 TEXT,
               pending INTEGER NOT NULL,
               description TEXT
             ) STRICT;  
             
             CREATE TABLE IF NOT EXISTS settings (
              key TEXT NOT NULL PRIMARY KEY,
              value TEXT NOT NULL
             ) STRICT;

             CREATE TABLE IF NOT EXISTS cached_items (
              key TEXT NOT NULL PRIMARY KEY,
              value TEXT NOT NULL
             ) STRICT;
             
             CREATE TABLE IF NOT EXISTS swaps (
               bitcoin_address TEXT PRIMARY KEY NOT NULL,
               created_at INTEGER DEFAULT CURRENT_TIMESTAMP,
               lock_height INTEGER NOT NULL,
               payment_hash BLOB NOT NULL UNIQUE,
               preimage BLOB NOT NULL UNIQUE,
               private_key BLOB NOT NULL UNIQUE,
               public_key BLOB NOT NULL UNIQUE,
               swapper_public_key BLOB NOT NULL UNIQUE,
               script BLOB NOT NULL UNIQUE,
               bolt11 TEXT,
               paid_sats INTEGER NOT NULL DEFAULT 0,
               confirmed_sats INTEGER NOT NULL DEFAULT 0,               
               status INTEGER NOT NULL DEFAULT 0,
               refund_tx_ids TEXT NOT NULL, 
               confirmed_tx_ids TEXT NOT NULL
             ) STRICT;
        ",
        )]);

        let mut conn = self.get_connection()?;
        migrations
            .to_latest(&mut conn)
            .map_err(anyhow::Error::msg)?;
        Ok(())
    }

    pub(crate) fn get_connection(&self) -> Result<Connection> {
        Connection::open(self.file.clone()).map_err(anyhow::Error::msg)
    }
}

pub(crate) struct StringArray(pub Vec<String>);

impl FromSql for StringArray {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let res: Result<Vec<String>, FromSqlError> =
            serde_json::from_str(value.as_str()?).map_err(|_| FromSqlError::InvalidType);
        Ok(StringArray(res?))
    }
}

impl ToSql for StringArray {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let res = serde_json::to_string(&self.0).map_err(|_| FromSqlError::InvalidType);
        Ok(ToSqlOutput::from(res?))
    }
}
