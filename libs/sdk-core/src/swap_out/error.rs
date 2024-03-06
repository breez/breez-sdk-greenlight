use anyhow::anyhow;

use crate::{
    bitcoin::{hashes, secp256k1},
    error::SdkError,
    node_api::NodeError,
    persist::error::PersistError,
};

pub type ReverseSwapResult<T, E = ReverseSwapError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum ReverseSwapError {
    #[error("Generic: {0}")]
    Generic(#[from] anyhow::Error),

    #[error("Claim tx feerate is too low")]
    ClaimFeerateTooLow,

    #[error("Invalid destination address: {0}")]
    InvalidDestinationAddress(anyhow::Error),

    #[error(transparent)]
    Node(#[from] NodeError),

    #[error("Service connectivity: {0}")]
    ServiceConnectivity(anyhow::Error),

    #[error("Unexpected invoice amount: {0}")]
    UnexpectedInvoiceAmount(anyhow::Error),

    #[error("Unexpected lockup address")]
    UnexpectedLockupAddress,

    #[error("Unexpected payment hash: {0}")]
    UnexpectedPaymentHash(anyhow::Error),

    #[error("Unexpected redeem script")]
    UnexpectedRedeemScript,

    #[error("Route not found: {0}")]
    RouteNotFound(anyhow::Error),
}
impl ReverseSwapError {
    pub(crate) fn generic(err: &str) -> Self {
        Self::Generic(anyhow!(err.to_string()))
    }
}

impl From<hashes::hex::Error> for ReverseSwapError {
    fn from(err: hashes::hex::Error) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<hex::FromHexError> for ReverseSwapError {
    fn from(err: hex::FromHexError) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<crate::lightning_invoice::ParseOrSemanticError> for ReverseSwapError {
    fn from(err: crate::lightning_invoice::ParseOrSemanticError) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<PersistError> for ReverseSwapError {
    fn from(err: PersistError) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<reqwest::Error> for ReverseSwapError {
    fn from(err: reqwest::Error) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<SdkError> for ReverseSwapError {
    fn from(value: SdkError) -> Self {
        match value {
            SdkError::Generic { err } => Self::Generic(anyhow!(err)),
            SdkError::ServiceConnectivity { err } => Self::ServiceConnectivity(anyhow!(err)),
        }
    }
}

impl From<secp256k1::Error> for ReverseSwapError {
    fn from(err: secp256k1::Error) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<serde_json::Error> for ReverseSwapError {
    fn from(err: serde_json::Error) -> Self {
        Self::ServiceConnectivity(anyhow::Error::new(err))
    }
}

impl From<tonic::Status> for ReverseSwapError {
    fn from(err: tonic::Status) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}
