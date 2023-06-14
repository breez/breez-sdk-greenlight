use crate::{
    breez_services::BackupFailedData,
    persist::db::{HookEvent, SqliteStorage},
    BreezEvent,
};
use anyhow::{anyhow, Result};
use ecies::utils::{aes_decrypt, aes_encrypt};
use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec_with_limit};
use std::{
    env::temp_dir,
    fs::File,
    io::{Read, Write},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tempfile::tempdir_in;
use tokio::sync::broadcast::{self, error::RecvError};

// BackupState is the sdk state that requiers syncing between multiple apps.
/// It is just a blob of data marked with a specific version (generation).
/// The generation signals for the local state if the remote state is newer,
/// where in that case the local state should be updated with the remote state prior to pushing
/// any local changes.
pub struct BackupState {
    pub generation: u64,
    pub data: Vec<u8>,
}

/// BackupTransport is the interface for syncing the sdk state between multiple apps.
#[tonic::async_trait]
pub trait BackupTransport: Send + Sync {
    async fn pull(&self) -> Result<Option<BackupState>>;
    async fn push(&self, version: Option<u64>, data: Vec<u8>) -> Result<u64>;
}

pub(crate) struct BackupWatcher {
    started: AtomicBool,
    notifier: broadcast::Sender<BreezEvent>,
    worker: BackupWorker,
}

/// watches for sync requests and syncs the sdk state when a request is detected.
impl BackupWatcher {
    pub(crate) fn new(
        inner: Arc<dyn BackupTransport>,
        persister: Arc<SqliteStorage>,
        encryption_key: Vec<u8>,
    ) -> Self {
        let (notifier, _) = broadcast::channel::<BreezEvent>(1);
        let worker = BackupWorker::new(inner, persister, encryption_key, notifier.clone());

        Self {
            started: AtomicBool::new(false),
            notifier,
            worker,
        }
    }

    pub(crate) fn start(&self) -> Result<()> {
        // return error if we are already started
        if self.started.swap(true, Ordering::Relaxed) {
            return Err(anyhow!("Backup watcher already started"));
        }

        let worker = self.worker.clone();
        let mut hooks_subscription = worker.persister.subscribe_hooks();

        // spawn the background worker that handles backup requests
        tokio::spawn(async move {
            let res = worker.sync().await;
            info!("initial backup completed with result {res:?}");

            loop {
                tokio::select! {

                  // We spin the backup worker on every new entry to the sync_requests table.
                  event = hooks_subscription.recv() => {
                    match event {
                        Ok(event) => {
                            match event {
                                HookEvent::Insert { table } => {
                                    if table == "sync_requests" {
                                        info!("Sync request detected, starting sync");
                                        let res = worker.sync().await;
                                        info!("backup completed with result {res:?}");
                                    }
                                }
                            };
                        }
                        // If the channel is lagged we just continue
                        Err(RecvError::Lagged(_)) => {
                            continue;
                        }
                        // If the channel is closed we exit
                        Err(_) => {
                         return
                        }
                    }
                  }
                }
            }
        });

        Ok(())
    }

    pub(crate) fn subscribe_events(&self) -> broadcast::Receiver<BreezEvent> {
        self.notifier.subscribe()
    }
}

/// BackupWorker is a worker that bidirectionaly syncs the sdk state.
#[derive(Clone)]
struct BackupWorker {
    inner: Arc<dyn BackupTransport>,
    persister: Arc<SqliteStorage>,
    encryption_key: Vec<u8>,
    events_notifier: broadcast::Sender<BreezEvent>,
}

impl BackupWorker {
    pub(crate) fn new(
        inner: Arc<dyn BackupTransport>,
        persister: Arc<SqliteStorage>,
        encryption_key: Vec<u8>,
        events_notifier: broadcast::Sender<BreezEvent>,
    ) -> Self {
        Self {
            inner,
            persister,
            encryption_key,
            events_notifier,
        }
    }

    async fn notify(&self, e: BreezEvent) -> Result<()> {
        // we don't care for errors here as this happens if
        // there ar eno subscribers, just ignoring them.
        _ = self.events_notifier.send(e);
        Ok(())
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

        // In case we don't  have any sync requests the worker can exit
        if last_sync_request_id == 0 {
            return Ok(());
        }

        self.notify(BreezEvent::BackupStarted).await?;

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
        let optimistic_sync = self.push(last_version, data.clone()).await;

        let sync_result = match optimistic_sync {
            Ok(new_version) => {
                info!("Optimistic sync succeeded, new version = {new_version}");
                Ok(new_version)
            }
            Err(e) => {
                debug!("Optimistic sync failed, trying to sync remote changes {e}");
                // We need to sync remote changes and then retry the push
                self.sync_remote_and_push(data).await
            }
        };

        // In case we succeeded to push the local changes, we need to:
        // 1. Delete the sync requests so.
        // 2. Update the last sync version.
        match sync_result {
            Ok(new_version) => {
                let now = SystemTime::now();
                self.persister.set_last_sync_version(new_version)?;
                self.persister
                    .delete_sync_requests_up_to(last_sync_request_id)?;
                self.persister
                    .set_last_backup_time(now.duration_since(UNIX_EPOCH).unwrap().as_secs())?;
                info!("Sync succeeded");
                self.notify(BreezEvent::BackupSucceeded).await?;
                Ok(())
            }
            Err(e) => {
                error!("Sync failed: {}", e);
                self.notify(BreezEvent::BackupFailed {
                    details: BackupFailedData {
                        error: e.to_string(),
                    },
                })
                .await?;
                Err(e)
            }
        }
    }

    /// Syncs the remote changes into the local changes and then tries to push the local changes again.    
    async fn sync_remote_and_push(&self, local_data: Vec<u8>) -> Result<u64> {
        let remote_state = self.pull().await?;
        let tmp_dir = tempdir_in(temp_dir())?;
        let remote_storage_path = tmp_dir.path();
        let mut remote_storage_file = File::create(remote_storage_path.join("sync_storage.sql"))?;
        info!("remote_storage_path = {remote_storage_path:?}");
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
                    File::open(Path::new(remote_storage_path).join("sync_storage.sql"))?;
                remote_storage_file.read_to_end(&mut hex)?;

                // Push the local changes again
                let new_generation = self.push(Some(state.generation), hex).await?;
                Ok(new_generation)
            }

            // In case there is no remote state, we can just push the local changes
            None => {
                debug!("No remote state, pushing local changes");
                self.push(None, local_data).await
            }
        }
    }

    async fn pull(&self) -> Result<Option<BackupState>> {
        let state = self.inner.pull().await?;
        match state {
            Some(state) => {
                let decrypted_data =
                    aes_decrypt(self.encryption_key.as_slice(), state.data.as_slice())
                        .ok_or(anyhow!("Failed to decrypt backup"))?;
                match decompress_to_vec_with_limit(&decrypted_data, 4000000) {
                    Ok(decompressed) => Ok(Some(BackupState {
                        generation: state.generation,
                        data: decompressed,
                    })),
                    Err(e) => {
                        error!("Failed to decompress backup: {e}");
                        Ok(None)
                    }
                }
            }
            None => Ok(None),
        }
    }

    async fn push(&self, version: Option<u64>, data: Vec<u8>) -> Result<u64> {
        let compressed_data = compress_to_vec(&data, 10);
        info!(
            "Pushing compressed data with size = {}",
            compressed_data.len()
        );
        let encrypted_data =
            aes_encrypt(self.encryption_key.as_slice(), compressed_data.as_slice())
                .ok_or(anyhow!("Failed to encrypt backup"))?;
        self.inner.push(version, encrypted_data).await
    }
}
