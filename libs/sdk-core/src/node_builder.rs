use std::sync::Arc;

use crate::backup::BackupTransport;
use crate::breez_services::Receiver;
#[cfg(feature = "greenlight")]
use crate::greenlight::{GLBackupTransport, Greenlight};
#[cfg(feature = "ldk")]
use crate::ldk::{Ldk, LdkBackupTransport};
use crate::models::{Config, LspAPI};
use crate::node_api::{NodeAPI, NodeResult};
use crate::persist::db::SqliteStorage;
use crate::NodeConfig;

pub struct NodeImpls {
    pub node: Arc<dyn NodeAPI>,
    pub backup_transport: Arc<dyn BackupTransport>,
    pub lsp: Option<Arc<dyn LspAPI>>,
    pub receiver: Option<Arc<dyn Receiver>>,
}

#[allow(unused_variables)]
pub async fn build_node(
    config: Config,
    seed: Vec<u8>,
    restore_only: Option<bool>,
    persister: Arc<SqliteStorage>,
) -> NodeResult<NodeImpls> {
    // TODO: Add NodeConfig::Ldk variant once the feature is developed.
    #[cfg(feature = "ldk")]
    {
        let ldk = Ldk::build(config, &seed, restore_only).await?;
        let ldk = Arc::new(ldk);
        let backup_transport = Arc::new(LdkBackupTransport {});
        let lsp: Option<Arc<dyn LspAPI>> = Some(ldk.clone());
        let receiver: Option<Arc<dyn Receiver>> = Some(ldk.clone());
        return Ok(NodeImpls {
            node: ldk,
            backup_transport,
            lsp,
            receiver,
        });
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
                Ok(NodeImpls {
                    node: greenlight,
                    backup_transport,
                    lsp: None,
                    receiver: None,
                })
            }
            #[cfg(not(feature = "greenlight"))]
            Err(crate::node_api::NodeError::generic(
                "Misconfigration: `greenlight` feature must be enabled.",
            ))
        }
    }
}
