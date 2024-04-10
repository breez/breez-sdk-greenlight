use anyhow::anyhow;

use crate::{error::SdkError, persist::error::PersistError};

pub type SwapResult<T, E = SwapError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum SwapError {
    #[error("Generic: {0}")]
    Generic(#[from] anyhow::Error),

    #[error(transparent)]
    Persistance(#[from] PersistError),

    #[error("Service connectivity: {0}")]
    ServiceConnectivity(anyhow::Error),

    #[error("Unsupported swap limits: {0}")]
    UnsupportedSwapLimits(anyhow::Error),
}
impl SwapError {
    pub(crate) fn unsupported_swap_limits(err: &str) -> Self {
        Self::UnsupportedSwapLimits(anyhow!(err.to_string()))
    }
}

impl From<SdkError> for SwapError {
    fn from(value: SdkError) -> Self {
        match value {
            SdkError::Generic { err } => Self::Generic(anyhow!(err)),
            SdkError::ServiceConnectivity { err } => Self::ServiceConnectivity(anyhow!(err)),
        }
    }
}

impl From<tonic::Status> for SwapError {
    fn from(err: tonic::Status) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}
