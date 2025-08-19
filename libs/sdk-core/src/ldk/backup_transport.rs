use crate::backup::{BackupState, BackupTransport};
use crate::error::{SdkError, SdkResult};

pub(crate) struct LdkBackupTransport;

#[tonic::async_trait]
impl BackupTransport for LdkBackupTransport {
    async fn pull(&self) -> SdkResult<Option<BackupState>> {
        Err(SdkError::Generic {
            err: "LDK BackupTransport not implemented".into(),
        })
    }

    async fn push(&self, _version: Option<u64>, _data: Vec<u8>) -> SdkResult<u64> {
        Err(SdkError::Generic {
            err: "LDK BackupTransport not implemented".into(),
        })
    }
}
