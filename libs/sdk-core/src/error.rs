use thiserror::Error;

pub type SdkResult<T, E = SdkError> = Result<T, E>;

/// Type of error returned by the SDK
#[derive(Debug, Error)]
pub enum SdkError {
    /// Generic error, that doesn't fit any of the other types
    #[error("Breez SDK error: {err}")]
    Generic { err: String },

    #[error("Failed to initialize the SDK: {err}")]
    InitFailed { err: String },

    #[error("Failed to communicate with the LSP API: {err}")]
    LspConnectFailed { err: String },

    #[error("Failed to use the local DB for persistence: {err}")]
    PersistenceFailure { err: String },

    #[error("Failed to receive payment: {err}")]
    ReceivePaymentFailed { err: String },
}

impl From<rusqlite::Error> for SdkError {
    fn from(value: rusqlite::Error) -> Self {
        Self::PersistenceFailure {
            err: value.to_string(),
        }
    }
}

// TODO This won't be necessary when all service methods return SdkResult
impl From<anyhow::Error> for SdkError {
    fn from(value: anyhow::Error) -> Self {
        Self::Generic {
            err: value.to_string(),
        }
    }
}
