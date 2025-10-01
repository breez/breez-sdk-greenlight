use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rand::distributions::Alphanumeric;
use rand::Rng;
use tokio::runtime::Handle;
use tokio::sync::mpsc;
use vss_client::client::VssClient;
use vss_client::error::VssError;
use vss_client::util::retry::{ExponentialBackoffRetryPolicy, MaxAttemptsRetryPolicy, RetryPolicy};

use crate::ldk::store::{PreviousHolder, VssStore};
use crate::node_api::NodeResult;
use crate::persist::error::PersistError;

pub(crate) type CustomRetryPolicy = MaxAttemptsRetryPolicy<ExponentialBackoffRetryPolicy<VssError>>;
pub(crate) type LockingStore = crate::ldk::store::LockingStore<VssStore<CustomRetryPolicy>>;
pub(crate) type MirroringStore = crate::ldk::store::MirroringStore<Arc<LockingStore>, LockingStore>;

pub(crate) fn build_vss_store(url: String, store_id: String) -> VssStore<CustomRetryPolicy> {
    let vss_client = VssClient::new(
        url,
        ExponentialBackoffRetryPolicy::<VssError>::new(Duration::from_secs(1)).with_max_attempts(5),
    );
    VssStore::new(vss_client, store_id)
}

pub(crate) async fn build_mirroring_store(
    working_dir: &str,
    vss_store: VssStore<CustomRetryPolicy>,
    remote_lock_shutdown_rx: mpsc::Receiver<()>,
) -> NodeResult<MirroringStore> {
    let (locking_store, previous_holder) =
        build_locking_store(working_dir, vss_store, remote_lock_shutdown_rx).await?;

    let sqlite_file_path = Path::new(working_dir).join("ldk_node_storage.sql");
    let manager = SqliteConnectionManager::file(sqlite_file_path);
    let pool = Pool::new(manager).unwrap();
    MirroringStore::new(Handle::current(), pool, locking_store, previous_holder)
        .await
        .map_err(Into::into)
}

async fn build_locking_store(
    working_dir: &str,
    vss_store: VssStore<CustomRetryPolicy>,
    remote_lock_shutdown_rx: mpsc::Receiver<()>,
) -> NodeResult<(Arc<LockingStore>, PreviousHolder)> {
    let instance_id = read_or_generate_instance_id(working_dir)?;
    let (locking_store, previous_holder) = LockingStore::new(instance_id, vss_store)
        .await
        .map_err(|e| PersistError::Generic(format!("Failed to build locking store: {e}")))?;
    let locking_store = Arc::new(locking_store);
    tokio::task::spawn(start_refreshing(
        Arc::clone(&locking_store),
        remote_lock_shutdown_rx,
    ));
    Ok((locking_store, previous_holder))
}

fn read_or_generate_instance_id(working_dir: &str) -> Result<String, PersistError> {
    let filepath = Path::new(working_dir).join("instance_id");
    match fs::read_to_string(&filepath) {
        Ok(instance_id) => Ok(instance_id.trim().to_string()),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let instance_id = generate_instance_id();
            fs::write(&filepath, &instance_id).map_err(|e| {
                PersistError::Generic(format!(
                    "Failed to create file {}: {e}",
                    filepath.to_string_lossy()
                ))
            })?;
            Ok(instance_id)
        }
        Err(e) => Err(PersistError::Generic(format!(
            "Failed to read file {}: {e}",
            filepath.to_string_lossy()
        ))),
    }
}

fn generate_instance_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

async fn start_refreshing(locking_store: Arc<LockingStore>, mut shutdown_rx: mpsc::Receiver<()>) {
    loop {
        let duration = match locking_store.refresh_lock().await {
            Ok(until) => {
                trace!("Remote lock was refreshed");
                until.duration_since(SystemTime::now()).unwrap_or_default()
            }
            Err(e) => {
                warn!("Failed to refresh remote lock: {e:?}");
                Duration::from_secs(5)
            }
        };
        tokio::select! {
            biased; // Prioritise shutdown event.
            _ = shutdown_rx.recv() => break,
            _ = tokio::time::sleep(duration) => (),
        }
    }

    info!("Releasing remote lock");
    match locking_store.unlock().await {
        Ok(()) => info!("Remote lock was released"),
        Err(e) => error!("Failed to release remote lock: {e}"),
    };
    // Explicitly drop the receiver to let the sender know we are done with releasing the lock.
    drop(shutdown_rx);
}
