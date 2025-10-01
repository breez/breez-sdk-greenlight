use std::ops::Deref;
use std::sync::{Arc, Mutex};

use ldk_node::bitcoin::io::ErrorKind;
use ldk_node::lightning::io;
use ldk_node::lightning::util::persist::KVStore;
use rusqlite::{params, Connection, Error as SqlError, OptionalExtension};
use tokio::runtime::Handle;

use crate::ldk::store::time_lock::PreviousHolder;
use crate::ldk::store::versioned_store::{Error as RemoteError, VersionedStore};
use crate::node_api::NodeError;
use crate::persist::error::PersistError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Local error: {0}")]
    Local(#[from] SqlError),
    #[error("Remote error: {0}")]
    Remote(#[from] RemoteError),
}

impl From<Error> for NodeError {
    fn from(err: Error) -> Self {
        match err {
            Error::Local(e) => {
                PersistError::Sql(format!("Mirroring store local error: {e}")).into()
            }
            Error::Remote(e) => {
                NodeError::ServiceConnectivity(format!("Mirroring store remote error: {e}"))
            }
        }
    }
}

pub struct MirroringStore<S: Deref<Target = T>, T: VersionedStore + Send + Sync> {
    handle: Handle,
    remote_client: S,
    conn: Arc<Mutex<Connection>>,
}

impl<S: Deref<Target = T>, T: VersionedStore + Send + Sync> MirroringStore<S, T> {
    pub async fn new(
        handle: Handle,
        conn: Connection,
        remote: S,
        previous_holder: PreviousHolder,
    ) -> Result<Self, Error> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS store (
                primary_ns TEXT NOT NULL,
                secondary_ns TEXT NOT NULL,
                key TEXT NOT NULL,
                value BLOB NOT NULL,
                local_version INTEGER NOT NULL,
                remote_version INTEGER NOT NULL DEFAULT -1,
                PRIMARY KEY (primary_ns, secondary_ns, key)
            )",
            [],
        )?;

        let is_dirty = is_dirty(&conn)?;
        match (previous_holder, is_dirty) {
            (PreviousHolder::LocalInstance, false) => {
                info!("Local store is clean, nothing new on remote. Skipping reconciliation.");
            }
            (PreviousHolder::LocalInstance, true) => {
                info!("Local store is *dirty*, nothing new on remote. Uploading to remote...");
                upload(&conn, &*remote).await?;
            }
            (PreviousHolder::RemoteInstance, false) => {
                info!("Local store is clean, something new on remote possible. Downloading from remote...");
                download(&conn, &*remote).await?;
            }
            (PreviousHolder::RemoteInstance, true) => {
                info!("Local store is *dirty*, something new on remote possible. Downloading from remote...");
                download(&conn, &*remote).await?;
            }
        };

        Ok(Self {
            handle,
            conn: Arc::new(Mutex::new(conn)),
            remote_client: remote,
        })
    }
}

impl<S: Deref<Target = T>, T: VersionedStore + Send + Sync> KVStore for MirroringStore<S, T> {
    fn read(&self, primary_ns: &str, secondary_ns: &str, key: &str) -> io::Result<Vec<u8>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT value FROM store WHERE primary_ns = ?1 AND secondary_ns = ?2 AND key = ?3",
            params![primary_ns, secondary_ns, key],
            |row| row.get(0),
        )
        .optional()
        .map_err(other)?
        .ok_or(io::Error::new(ErrorKind::NotFound, "Not Found"))
    }

    fn write(
        &self,
        primary_ns: &str,
        secondary_ns: &str,
        key: &str,
        value: &[u8],
    ) -> io::Result<()> {
        let full_key = format!("{primary_ns}/{secondary_ns}/{key}");
        debug!("Writing {full_key} {} bytes", value.len());
        let conn = self.conn.lock().unwrap();

        let local_data: Option<(i64, Vec<u8>)> = conn.query_row(
            "SELECT local_version, value FROM store WHERE primary_ns = ?1 AND secondary_ns = ?2 AND key = ?3",
            params![primary_ns, secondary_ns, key],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).optional().map_err(other)?;
        let next_version = match local_data {
            None => {
                let next_version = 0;
                conn.execute(
                    "INSERT INTO store (primary_ns, secondary_ns, key, value, local_version, remote_version) VALUES (?1, ?2, ?3, ?4, ?5, -1)",
                    params![primary_ns, secondary_ns, key, value, next_version],
                ).map_err(other)?;
                next_version
            }
            Some((_, local_value)) if local_value == value => {
                trace!("Local value is the same, skipping writing");
                return Ok(());
            }
            Some((local_version, _)) => {
                trace!("Local value is different, writing");
                let next_version = local_version + 1;
                conn.execute(
                    "UPDATE store SET value = ?1, local_version = ?2 WHERE primary_ns = ?3 AND secondary_ns = ?4 AND key = ?5",
                    params![value, next_version, primary_ns, secondary_ns, key],
                ).map_err(other)?;
                next_version
            }
        };

        tokio::task::block_in_place(|| {
            self.handle.block_on(
                self.remote_client
                    .put(full_key, value.to_vec(), next_version),
            )
        })
        .map_err(other)?;

        conn.execute(
            "UPDATE store SET remote_version = local_version WHERE primary_ns = ?1 AND secondary_ns = ?2 AND key = ?3",
            params![primary_ns, secondary_ns, key],
        ).map_err(other)?;

        Ok(())
    }

    fn remove(
        &self,
        primary_ns: &str,
        secondary_ns: &str,
        key: &str,
        _lazy: bool,
    ) -> io::Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "DELETE FROM store WHERE primary_ns = ?1 AND secondary_ns = ?2 AND key = ?3",
            params![primary_ns, secondary_ns, key],
        )
        .map_err(other)?;

        let full_key = format!("{primary_ns}/{secondary_ns}/{key}");
        tokio::task::block_in_place(|| self.handle.block_on(self.remote_client.delete(full_key)))
            .map_err(other)?;

        Ok(())
    }

    fn list(&self, primary_ns: &str, secondary_ns: &str) -> io::Result<Vec<String>> {
        self.conn
            .lock()
            .unwrap()
            .prepare("SELECT key FROM store WHERE primary_ns = ?1 AND secondary_ns = ?2")
            .map_err(other)?
            .query_map(params![primary_ns, secondary_ns], |row| row.get(0))
            .map_err(other)?
            .collect::<Result<Vec<String>, _>>()
            .map_err(other)
    }
}

fn is_dirty(conn: &Connection) -> rusqlite::Result<bool> {
    let dirty_rows: i64 = conn.query_row(
        "SELECT count(1) FROM store WHERE local_version != remote_version",
        [],
        |row| row.get(0),
    )?;
    Ok(dirty_rows > 0)
}

async fn download<S: VersionedStore>(conn: &Connection, remote: &S) -> Result<(), Error> {
    conn.execute("DELETE FROM store", [])?;

    for (full_key, version) in remote.list().await? {
        trace!("Downloading {full_key} @ {version} ...");
        let parts: Vec<&str> = full_key.splitn(3, '/').collect();
        let (primary, secondary, key) = match &parts[..] {
            [p, s, k] => (p.to_string(), s.to_string(), k.to_string()),
            _ => continue, // skip malformed keys
        };

        if let Some((value, version)) = remote.get(full_key).await? {
            trace!("Got {} bytes @ {version}", value.len());
            conn.execute(
                "INSERT INTO store (primary_ns, secondary_ns, key, value, local_version, remote_version) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
                params![primary, secondary, key, value, version - 1],
            )?;
        }
    }
    Ok(())
}

async fn upload<S: VersionedStore>(conn: &Connection, remote: &S) -> Result<(), Error> {
    let mut statement = conn.prepare("SELECT primary_ns, secondary_ns, key, value, local_version FROM store WHERE local_version != remote_version")?;
    let outdated_rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Vec<u8>>(3)?,
            row.get::<_, i64>(4)?,
        ))
    })?;

    for row in outdated_rows {
        let (primary_ns, secondary_ns, key, value, local_version) = row?;
        let full_key = format!("{primary_ns}/{secondary_ns}/{key}");
        trace!("Uploading {full_key} @ {local_version} ...");
        remote.put(full_key, value, local_version).await?;

        conn.execute(
            "UPDATE store SET remote_version = local_version WHERE primary_ns = ?1 AND secondary_ns = ?2 AND key = ?3",
            params![primary_ns, secondary_ns, key],
        )?;
    }
    Ok(())
}

fn other<E>(err: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    io::Error::new(ErrorKind::Other, err)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ldk::store::mock_versioned_store::MockVersionedStore;
    use crate::ldk::store::time_lock::PreviousHolder;
    use rusqlite::backup::Backup;
    use rusqlite::Connection;
    use std::time::Duration;
    use tokio::runtime::Handle;

    fn create_in_memory_db() -> Connection {
        Connection::open_in_memory().unwrap()
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_mirroring_store_normal_flow() {
        let mock_store = MockVersionedStore::default();
        let store = MirroringStore::new(
            Handle::current().clone(),
            create_in_memory_db(),
            &mock_store,
            PreviousHolder::RemoteInstance,
        )
        .await
        .unwrap();

        // Write, read, list values, remove.
        let list = store.list("ns", "sub").unwrap();
        assert!(list.is_empty());

        store.write("ns", "sub", "key", b"value").unwrap();
        store
            .write("ns", "sub", "to_remove", b"to_remove_value")
            .unwrap();
        store.remove("ns", "sub", "to_remove", false).unwrap();
        store.remove("ns", "sub", "does_not_exist", false).unwrap();

        let list = store.list("ns", "sub").unwrap();
        assert_eq!(list, vec!["key".to_string()]);
        let value = store.read("ns", "sub", "key").unwrap();
        assert_eq!(value, b"value");

        // Load a new instance.
        let store = MirroringStore::new(
            Handle::current().clone(),
            create_in_memory_db(),
            &mock_store,
            PreviousHolder::RemoteInstance,
        )
        .await
        .unwrap();
        // Data was loaded from remote.
        let list = store.list("ns", "sub").unwrap();
        assert_eq!(list, vec!["key".to_string()]);
        let value = store.read("ns", "sub", "key").unwrap();
        assert_eq!(value, b"value");

        // Update the value, write a new value..
        store.write("ns", "sub", "key", b"value2").unwrap();
        store.write("ns2", "sub2", "key2", b"value22").unwrap();
        let list = store.list("ns", "sub").unwrap();
        assert_eq!(list, vec!["key".to_string()]);
        let value = store.read("ns", "sub", "key").unwrap();
        assert_eq!(value, b"value2");
        let value = store.read("ns2", "sub2", "key2").unwrap();
        assert_eq!(value, b"value22");

        // No removed key.
        let err = store.read("ns", "sub", "to_remove").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_mirroring_store_remote_failure_handling() {
        let mut mock_store = MockVersionedStore::default();
        mock_store.should_fail_put = true; // Simulate remote failure

        let store = MirroringStore::new(
            Handle::current().clone(),
            create_in_memory_db(),
            &mock_store,
            PreviousHolder::LocalInstance,
        )
        .await
        .unwrap();

        // Try to write - should fail due to remote error.
        let err = store
            .write("ns", "sub", "key_dirty", b"value_dirty")
            .unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Other);
        // Dirty data is stored locally, though.
        let value = store.read("ns", "sub", "key_dirty").unwrap();
        assert_eq!(value, b"value_dirty");

        {
            // A new instance does not load this imformation.
            let store = MirroringStore::new(
                Handle::current().clone(),
                create_in_memory_db(),
                &mock_store,
                PreviousHolder::RemoteInstance,
            )
            .await
            .unwrap();
            let err = store.read("ns", "sub", "key_dirty").unwrap_err();
            assert_eq!(err.kind(), ErrorKind::NotFound);
        }

        {
            // Recovery of a dirty instance with another instance accessing the
            // store in between.
            let mut dirty_local_db = create_in_memory_db();
            clone_data(&store.conn.lock().unwrap(), &mut dirty_local_db);

            let store = MirroringStore::new(
                Handle::current().clone(),
                dirty_local_db,
                &mock_store,
                PreviousHolder::RemoteInstance,
            )
            .await
            .unwrap();
            let err = store.read("ns", "sub", "key_dirty").unwrap_err();
            assert_eq!(err.kind(), ErrorKind::NotFound);
        }

        {
            // Recovery of a dirty instance with *no* other instances accessing
            // the store in between.
            let mut dirty_local_db = create_in_memory_db();
            clone_data(&store.conn.lock().unwrap(), &mut dirty_local_db);
            mock_store.should_fail_put = false;

            let store = MirroringStore::new(
                Handle::current().clone(),
                dirty_local_db,
                &mock_store,
                PreviousHolder::LocalInstance,
            )
            .await
            .unwrap();
            let value = store.read("ns", "sub", "key_dirty").unwrap();
            assert_eq!(value, b"value_dirty");
            // Data was uploaded to remote.
            let data = mock_store.data.lock().unwrap();
            let value = data.get("ns/sub/key_dirty").unwrap().0.clone();
            assert_eq!(value, b"value_dirty");
        }
    }

    fn clone_data(src: &Connection, dst: &mut Connection) {
        Backup::new(src, dst)
            .unwrap()
            .run_to_completion(5, Duration::default(), None)
            .unwrap()
    }
}
