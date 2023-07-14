use anyhow::Error;
use std::fmt;

pub type SdkResult<T, E = NewSdkError> = Result<T, E>;

/// Type of error returned by the SDK
#[derive(Debug)]
pub enum NewSdkError {
    SdkFailedToRequestBackup,

    SdkFailedToStartBackupWatcher,

    SdkFailedToInitPersister,
    SdkFailedToConnectToGreenlight,

    SdkNoStateSynchronizerFound,

    SdkInitFailedNoNodeApiOrSeedFound,

    SdkCannotRetrievePersistedLastSyncVersion,

    SdkBip32ErrorIndexOutOfBounds,
    SdkBip32ErrorFailedToDeriveKey,

    SdkFailedToInsertOrUpdatePayments,
    SdkFailedToInsertLnurlExternalPaymentInfo,
    SdkFailedToReceivePayment,

    SdkFailedToListPayments,

    SdkFailedToSync,

    // NodeState
    SdkCannotRetrievePersistedNodeState,
    SdkNoNodeStateFound,

    // LSPs
    SdkFailedToRetrieveLsps,
    SdkFailedToGetLspId,
    SdkFailedToSetLspId,

    SdkServicesAlreadySet,
    SdkServicesAlreadyStarted,
    SdkServicesFailedToInit,
    SdkServicesWasNotInitialized,
}

impl From<NewSdkError> for anyhow::Error {
    fn from(value: NewSdkError) -> Self {
        anyhow::Error::msg(value.to_string())
    }
}

impl fmt::Display for NewSdkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<anyhow::Error> for NewSdkError {
    fn from(_value: Error) -> Self {
        Self::SdkServicesFailedToInit // TODO This conversion may not be possible (but is needed for uniffi?)
    }
}
