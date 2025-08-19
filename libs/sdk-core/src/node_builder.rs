use std::sync::Arc;

use crate::backup::BackupTransport;
#[cfg(feature = "greenlight")]
use crate::greenlight::{GLBackupTransport, Greenlight};
#[cfg(feature = "ldk")]
use crate::ldk::{Ldk, LdkBackupTransport};
use crate::models::Config;
use crate::node_api::{NodeAPI, NodeResult};
use crate::persist::db::SqliteStorage;
use crate::NodeConfig;

#[allow(unused_variables)]
pub async fn build_node(
    config: Config,
    seed: Vec<u8>,
    restore_only: Option<bool>,
    persister: Arc<SqliteStorage>,
) -> NodeResult<(Arc<dyn NodeAPI>, Arc<dyn BackupTransport>)> {
    // TODO: Add NodeConfig::Ldk variant once the feature is developed.
    #[cfg(feature = "ldk")]
    {
        let ldk = Ldk::build(config, &seed, restore_only).await;
        let ldk = Arc::new(ldk);
        let backup_transport = Arc::new(LdkBackupTransport {});
        return Ok((ldk, backup_transport));
    }
    #[allow(unreachable_code)]
    match config.node_config {
        NodeConfig::Greenlight { .. } => {
            #[cfg(feature = "greenlight")]
            {
                let greenlight = Greenlight::connect(config, seed, restore_only, persister).await?;
                let greenlight = Arc::new(greenlight);
                let backup_transport = GLBackupTransport {
                    inner: greenlight.clone(),
                };
                let backup_transport = Arc::new(backup_transport);
                Ok((greenlight, backup_transport))
            }
            #[cfg(not(feature = "greenlight"))]
            Err(crate::node_api::NodeError::generic(
                "Misconfigration: `greenlight` feature must be enabled.",
            ))
        }
    }
}
