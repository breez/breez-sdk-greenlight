use thiserror::Error;

pub type SdkResult<T, E = NewSdkError> = Result<T, E>;
pub type SdkResultDetailed<T, E = NewSdkErrorDetailed> = Result<T, E>;

#[derive(Error, Debug)]
pub enum NewSdkErrorDetailed {
    #[error("No state synchronizer found")]
    ConnectFailedNoStateSynchronizerFound,
    #[error("Failed to start backup watcher")]
    ConnectFailedFailedToStartBackupWatcher,
    #[error("No Node API or seed found")]
    ConnectFailedInitFailedNoNodeApiOrSeedFound,
    #[error("Failed to connect to Greenlight: `{0}`")]
    ConnectFailedFailedToConnectToGreenlight(String),

    #[error("Serialization or deserialization error: `{0}`")]
    PersistenceSerializationErr(serde_json::Error),
    #[error("Hex decoding error: `{0}`")]
    PersistenceHexDecodeErr(hex::FromHexError),
    #[error("Failed to get cached item")]
    PersistenceFailedToGetCachedItem,

    #[error("DB error: `{0}`")]
    PersistenceDbErr(rusqlite::Error),
    #[error("DB migration error: `{0}`")]
    PersistenceDbMigrationErr(rusqlite_migration::Error),

    #[error("BIP-32 derivation error: `{0}`")]
    SdkBip32Error(bitcoin::util::bip32::Error),

    #[error("Node is not initialized")]
    SdkServicesNodeNotInitialized,
}

impl From<rusqlite::Error> for NewSdkErrorDetailed {
    fn from(err: rusqlite::Error) -> Self {
        Self::PersistenceDbErr(err)
    }
}

impl From<rusqlite_migration::Error> for NewSdkErrorDetailed {
    fn from(err: rusqlite_migration::Error) -> Self {
        Self::PersistenceDbMigrationErr(err)
    }
}

impl From<hex::FromHexError> for NewSdkErrorDetailed {
    fn from(err: hex::FromHexError) -> Self {
        Self::PersistenceHexDecodeErr(err)
    }
}

impl From<serde_json::Error> for NewSdkErrorDetailed {
    fn from(err: serde_json::Error) -> Self {
        Self::PersistenceSerializationErr(err)
    }
}

impl From<bitcoin::util::bip32::Error> for NewSdkErrorDetailed {
    fn from(err: bitcoin::util::bip32::Error) -> Self {
        Self::SdkBip32Error(err)
    }
}

/// Type of error returned by the SDK
#[derive(Error, Debug)]
pub enum NewSdkError {
    #[error("Connect failed")]
    ConnectFailed,
    #[error("Persistence failure")]
    PersistenceFailure,

    // TODO Below errors still WIP
    #[error("err")]
    SdkFailedToRequestBackup,

    #[error("err")]
    SdkCannotRetrievePersistedLastSyncVersion,

    #[error("err")]
    SdkFailedToInsertLnurlExternalPaymentInfo,
    #[error("err")]
    SdkFailedToReceivePayment,

    #[error("err")]
    SdkFailedToSync,

    #[error("err")]
    SdkNoNodeStateFound,
    #[error("err")]
    SdkFailedToRetrieveLsps, // TODO Not a persistence failure, because it uses ChannelOpenerClient to lookup lsps
    #[error("err")]
    SdkServicesAlreadySet,
    #[error("err")]
    SdkServicesAlreadyStarted,
    #[error("err")]
    SdkServicesFailedToInit,

    #[error("err")]
    SdkServicesWasNotInitialized,
}

/// Converts and logs (where appropriate)
impl From<NewSdkErrorDetailed> for NewSdkError {
    fn from(err_detailed: NewSdkErrorDetailed) -> Self {
        error!("{err_detailed}");

        match err_detailed {
            NewSdkErrorDetailed::PersistenceSerializationErr(_)
            | NewSdkErrorDetailed::PersistenceHexDecodeErr(_)
            | NewSdkErrorDetailed::PersistenceFailedToGetCachedItem
            | NewSdkErrorDetailed::PersistenceDbErr(_)
            | NewSdkErrorDetailed::PersistenceDbMigrationErr(_) => Self::PersistenceFailure,

            NewSdkErrorDetailed::ConnectFailedInitFailedNoNodeApiOrSeedFound
            | NewSdkErrorDetailed::ConnectFailedFailedToStartBackupWatcher
            | NewSdkErrorDetailed::ConnectFailedNoStateSynchronizerFound
            | NewSdkErrorDetailed::ConnectFailedFailedToConnectToGreenlight(_)
            | NewSdkErrorDetailed::SdkBip32Error(_) => Self::ConnectFailed,

            NewSdkErrorDetailed::SdkServicesNodeNotInitialized => {
                Self::SdkServicesWasNotInitialized
            }
        }
    }
}

impl From<anyhow::Error> for NewSdkError {
    fn from(_value: anyhow::Error) -> Self {
        Self::SdkServicesFailedToInit // TODO This conversion may not be possible (but is needed for uniffi?)
    }
}
