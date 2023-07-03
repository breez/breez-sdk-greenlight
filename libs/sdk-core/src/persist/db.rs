use super::migrations::{current_migrations, current_sync_migrations};
use anyhow::Result;
use rusqlite::{
    hooks::Action,
    types::{FromSql, FromSqlError, ToSqlOutput},
    Connection, ToSql,
};
use rusqlite_migration::{Migrations, M};
use tokio::sync::broadcast;

/// HookEvent is used to notify listeners about DB changes.
/// A listener can register to be notified about specific events that occurs as part of
/// modifications in the persistent storage.
#[derive(Debug, Clone)]
pub(crate) enum HookEvent {
    Insert { table: String },
}

pub(crate) struct SqliteStorage {
    /// Local DB. Exists only on this instance of the SDK.
    main_db_file: String,
    /// Sync DB. Gets synchronized across the different instances that connect to the same wallet.
    sync_db_file: String,
    /// Dispatch DB hook events.
    events_publisher: broadcast::Sender<HookEvent>,
}

impl SqliteStorage {
    pub fn new(working_dir: String) -> SqliteStorage {
        let main_db_file = format!("{}/storage.sql", working_dir);
        let sync_db_file = format!("{}/sync_storage.sql", working_dir);
        let (events_publisher, _) = broadcast::channel::<HookEvent>(100);

        SqliteStorage {
            main_db_file,
            sync_db_file,
            events_publisher,
        }
    }

    pub(crate) fn subscribe_hooks(&self) -> broadcast::Receiver<HookEvent> {
        self.events_publisher.subscribe()
    }

    pub(crate) fn init(&self) -> Result<()> {
        Self::migrate_sync_db(self.sync_db_file.clone())?;
        self.migrate_main_db()?;
        Ok(())
    }

    pub(crate) fn migrate_sync_db(sync_db_path: String) -> Result<()> {
        let mut sync_con = Connection::open(sync_db_path).map_err(anyhow::Error::msg)?;
        let sync_migrations =
            Migrations::new(current_sync_migrations().into_iter().map(M::up).collect());
        sync_migrations
            .to_latest(&mut sync_con)
            .map_err(anyhow::Error::msg)?;
        Ok(())
    }

    fn migrate_main_db(&self) -> Result<()> {
        let migrations = Migrations::new(current_migrations().into_iter().map(M::up).collect());
        let mut conn = self.get_connection()?;
        migrations
            .to_latest(&mut conn)
            .map_err(anyhow::Error::msg)?;
        Ok(())
    }

    pub(crate) fn get_connection(&self) -> Result<Connection> {
        let con = Connection::open(self.main_db_file.clone()).map_err(anyhow::Error::msg)?;
        let sql = "ATTACH DATABASE ? AS sync;";
        con.execute(sql, [self.sync_db_file.clone()])?;
        // We want to notify any subscribers with hook events.
        let events_publisher = self.events_publisher.clone();
        con.update_hook(Some(move |action, db: &str, t: &str, _| {
            if action == Action::SQLITE_INSERT && db == "sync" {
                _ = events_publisher.send(HookEvent::Insert { table: t.into() });
            }
        }));

        Ok(con)
    }

    pub(crate) fn sync_db_path(&self) -> String {
        self.sync_db_file.clone()
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
