use std::path::Path;

use anyhow::Result;
use rusqlite::{
    types::{FromSql, FromSqlError, ToSqlOutput},
    Connection, ToSql,
};

use rusqlite_migration::{Migrations, M};

use super::migrations::current_migrations;

pub struct SqliteStorage {
    file: String,
}

impl SqliteStorage {
    pub fn from_file(file: String) -> SqliteStorage {
        SqliteStorage { file }
    }

    pub fn init(&self) -> Result<()> {
        let migrations = Migrations::new(current_migrations().into_iter().map(M::up).collect());
        let mut conn = self.get_connection()?;
        migrations
            .to_latest(&mut conn)
            .map_err(anyhow::Error::msg)?;
        Ok(())
    }

    pub(crate) fn get_connection(&self) -> Result<Connection> {
        let con = Connection::open(self.file.clone()).map_err(anyhow::Error::msg)?;
        let sync_data_file = self.sync_file_path();
        let sql = "ATTACH DATABASE ? AS sync;";
        con.execute(sql, [sync_data_file])?;
        Ok(con)
    }

    pub(crate) fn sync_file_path(&self) -> String {
        let path: &Path = Path::new(self.file.as_str());
        let mut result = path.to_owned();
        let file_name = result.file_name().unwrap();
        result.set_file_name(format!("sync_{}", file_name.to_str().unwrap()));
        if let Some(ext) = path.extension() {
            result.set_extension(ext);
        }
        result.to_str().unwrap().to_string()
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
