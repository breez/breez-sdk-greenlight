use crate::backup::{BackupState, BackupTransport};
use crate::{log_info, Logger};

use super::node_api::Greenlight;
use anyhow::{anyhow, Result};
use gl_client::{node, pb};
use std::sync::Arc;

const BREEZ_SDK_DATASTORE_PATH: [&str; 2] = ["breez-sdk", "backup"];

pub(crate) struct GLBackupTransport {
    pub(crate) inner: Arc<Greenlight>,
    pub(crate) logger: Arc<Box<dyn Logger>>,
}

impl GLBackupTransport {
    fn gl_key(&self) -> Vec<String> {
        BREEZ_SDK_DATASTORE_PATH.map(|s| s.into()).to_vec()
    }
}
#[tonic::async_trait]
impl BackupTransport for GLBackupTransport {
    async fn pull(&self) -> Result<Option<BackupState>> {
        let key = self.gl_key();
        let mut c: node::ClnClient = self.inner.get_node_client().await?;
        let response: pb::cln::ListdatastoreResponse = c
            .list_datastore(pb::cln::ListdatastoreRequest { key })
            .await?
            .into_inner();
        let store = response.datastore;
        match store.len() {
            0 => Ok(None),
            1 => Ok(Some(BackupState {
                generation: store[0].generation.unwrap(),
                data: store[0].clone().hex.unwrap(),
            })),
            _ => Err(anyhow!("get returned multiple values")),
        }
    }

    async fn push(&self, version: Option<u64>, hex: Vec<u8>) -> Result<u64> {
        let key = self.gl_key();
        log_info!(
            self.logger,
            "set_value key = {:?} data length={:?}",
            key,
            hex.len()
        );
        let mut c: node::ClnClient = self.inner.get_node_client().await?;
        let mut mode = pb::cln::datastore_request::DatastoreMode::MustCreate;
        if version.is_some() {
            mode = pb::cln::datastore_request::DatastoreMode::MustReplace;
        }
        let response = c
            .datastore(pb::cln::DatastoreRequest {
                key,
                string: None,
                hex: Some(hex),
                generation: version,
                mode: Some(mode.into()),
            })
            .await?
            .into_inner();
        Ok(response.generation.unwrap())
    }
}
