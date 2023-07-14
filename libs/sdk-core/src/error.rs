use thiserror::Error;

pub type SdkResult<T, E = SdkError> = Result<T, E>;

/// Type of error returned by the SDK
#[derive(Debug, Error)]
pub enum SdkError {
    #[error("Failed to connect to required services")]
    ConnectFailed,

    /// Generic error, that doesn't fit any of the other types
    #[error("Breez SDK error: {err}")]
    Generic { err: String },

    #[error("Failed to initialize the SDK")]
    InitFailed,

    #[error("Failed to use the local DB for persistence")]
    PersistenceFailure,

    #[error("Failed to receive payment")]
    ReceivePaymentFailure,
}

// TODO This won't be necessary when all service methods return SdkResult
impl From<anyhow::Error> for SdkError {
    fn from(_value: anyhow::Error) -> Self {
        Self::InitFailed
    }
}
