use crate::{error::SdkError, persist::error::PersistError};

pub type SwapResult<T, E = SwapError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum SwapError {
    #[error("{0}")]
    Generic(String),

    #[error(transparent)]
    Persistance(#[from] PersistError),

    #[error("{0}")]
    ServiceConnectivity(String),

    #[error("{0}")]
    UnsupportedSwapLimits(String),
}

impl SwapError {
    pub(crate) fn generic(err: &str) -> Self {
        Self::Generic(err.to_string())
    }

    pub(crate) fn unsupported_swap_limits(err: &str) -> Self {
        Self::UnsupportedSwapLimits(err.to_string())
    }
}

impl From<anyhow::Error> for SwapError {
    fn from(err: anyhow::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<SdkError> for SwapError {
    fn from(value: SdkError) -> Self {
        match value {
            SdkError::Generic { err } => Self::Generic(err),
            SdkError::ServiceConnectivity { err } => Self::ServiceConnectivity(err),
        }
    }
}

impl From<tonic::Status> for SwapError {
    fn from(status: tonic::Status) -> Self {
        Self::Generic(sdk_common::tonic_wrap::Status(status).to_string())
    }
}
