use crate::persist::db::{HookEvent, HookListener, SqliteStorage};
use anyhow::{anyhow, Result};
use std::{
    env::temp_dir,
    fs::File,
    io::{Read, Write},
    path::Path,
    sync::Arc,
};
use tempfile::tempdir_in;

// SyncState is the sdk state that requiers syncing between multiple apps.
/// It is just a blob of data marked with a specific version (generation).
/// The generation signals for the local state if the remote state is newer,
/// where in that case the local state should be updated with the remote state prior to pushing
/// any local changes.
pub struct SyncState {
    pub generation: u64,
    pub data: Vec<u8>,
}

/// SyncTransport is the interface for syncing the sdk state between multiple apps.
#[tonic::async_trait]
pub trait SyncTransport: Send + Sync {
    async fn pull(&self) -> Result<Option<SyncState>>;
    async fn push(&self, version: Option<u64>, data: Vec<u8>) -> Result<u64>;
}

/// watches for sync requests and syncs the sdk state when a request is detected.
pub(crate) fn watch_and_sync(inner: Arc<dyn SyncTransport>, persister: Arc<SqliteStorage>) {
    persister.add_hook_listener(Arc::new(SyncStorageListener {
        inner: inner.clone(),
        persister: persister.clone(),
    }))
}

/// SyncStorageListener is a hook listener that syncs the sdk state when a sync request is detected.
/// It listens to events on the sync_requests table and starts a sync when a new request is detected.
struct SyncStorageListener {
    inner: Arc<dyn SyncTransport>,
    persister: Arc<SqliteStorage>,
}

impl HookListener for SyncStorageListener {
    fn notify(&self, event: &HookEvent) {
        match event {
            HookEvent::Insert { table } => {
                if table == "sync_requests" {
                    let transport = self.inner.clone();
                    let persister = self.persister.clone();
                    info!("Sync request detected, starting sync");

                    // spawn a new worker to sync the sdk state
                    tokio::spawn(async move {
                        if let Err(e) = SyncWorker::new(transport, persister).sync().await {
                            error!("Failed to sync sdk state: {}", e);
                        }
                    });
                }
            }
        }
    }
}

/// SyncWorker is a worker that bidirectionaly syncs the sdk state.
struct SyncWorker {
    inner: Arc<dyn SyncTransport>,
    persister: Arc<SqliteStorage>,
}

impl SyncWorker {
    pub(crate) fn new(inner: Arc<dyn SyncTransport>, persister: Arc<SqliteStorage>) -> Self {
        Self { inner, persister }
    }
    /// Syncs the sdk state with the remote state.
    /// The process is done in 3 steps:
    /// 1. Try to push the local state to the remote state using the current version (optimistic).
    /// 2. If the push fails, sync the remote changes into the local changes including the remote newer version.
    /// 3. Try to push the local state again with the new version.
    async fn sync(&self) -> Result<()> {
        // get the last local sync version and the last sync request id
        let last_version = self.persister.get_last_sync_version()?;
        let last_sync_request_id = self.persister.get_last_sync_request()?.unwrap_or_default();

        // Backup the local sdk state
        let local_storage_file = tempfile::NamedTempFile::new()?;
        self.persister.backup(local_storage_file.path())?;
        debug!(
            "syncing storge, last_version = {:?}, file = {:?}",
            last_version,
            local_storage_file.path()
        );

        // read the backed up local data
        let mut f = File::open(local_storage_file.path())?;
        let mut data = vec![];
        f.read_to_end(&mut data)?;

        // Try to push with the current version, if no one else has pushed then we will succeed
        let optimistic_sync = self.inner.push(last_version, data).await;

        let sync_result = match optimistic_sync {
            Ok(new_version) => {
                info!("Optimistic sync succeeded, new version = {:?}", new_version);
                Ok(new_version)
            }
            Err(e) => {
                debug!(
                    "Optimistic sync failed, trying to sync remote changes {}",
                    e
                );
                // We need to sync remote changes and then retry the push
                self.sync_remote_and_push().await
            }
        };

        // In case we succeeded to push the local changes, we need to:
        // 1. Delete the sync requests so.
        // 2. Update the last sync version.
        match sync_result {
            Ok(new_version) => {
                self.persister.set_last_sync_version(new_version)?;
                self.persister
                    .delete_sync_requests_up_to(last_sync_request_id)?;
                info!("Sync succeeded");
                Ok(())
            }
            Err(e) => {
                error!("Sync failed: {}", e);
                Err(e)
            }
        }
    }

    /// Syncs the remote changes into the local changes and then tries to push the local changes again.    
    async fn sync_remote_and_push(&self) -> Result<u64> {
        let remote_state = self.inner.pull().await?;
        let tmp_dir = tempdir_in(temp_dir())?;
        let remote_storage_path = tmp_dir.path();
        let mut remote_storage_file = File::create(&remote_storage_path.join("sync_storage.sql"))?;
        info!("remote_storage_path = {:?}", remote_storage_path);
        match remote_state {
            Some(state) => {
                // Write the remote state to a file
                remote_storage_file.write_all(&state.data[..])?;
                remote_storage_file.flush()?;
                let remote_storage = SqliteStorage::new(
                    remote_storage_path
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .to_string(),
                );

                // Bidirectionaly sync the local and remote changes
                self.persister.import_remote_changes(&remote_storage)?;
                remote_storage.import_remote_changes(self.persister.as_ref())?;
                let mut hex = vec![];
                remote_storage_file =
                    File::open(&Path::new(remote_storage_path).join("sync_storage.sql"))?;
                remote_storage_file.read_to_end(&mut hex)?;

                // Push the local changes again
                let new_generation = self.inner.push(Some(state.generation), hex).await?;
                Ok(new_generation)
            }
            None => Err(anyhow!("pull returned no values")),
        }
    }
}
