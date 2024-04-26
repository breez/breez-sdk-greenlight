use crate::{
    bitcoin::{hashes, secp256k1},
    error::SdkError,
    node_api::NodeError,
    persist::error::PersistError,
};

pub type ReverseSwapResult<T, E = ReverseSwapError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum ReverseSwapError {
    #[error("{0}")]
    Generic(String),

    #[error("Claim tx feerate is too low")]
    ClaimFeerateTooLow,

    #[error("{0}")]
    InvalidDestinationAddress(String),

    #[error(transparent)]
    Node(#[from] NodeError),

    #[error("{0}")]
    RouteNotFound(String),

    #[error("{0}")]
    ServiceConnectivity(String),

    #[error("{0}")]
    UnexpectedInvoiceAmount(String),

    #[error("Unexpected lockup address")]
    UnexpectedLockupAddress,

    #[error("{0}")]
    UnexpectedPaymentHash(String),

    #[error("Unexpected redeem script")]
    UnexpectedRedeemScript,
}

impl ReverseSwapError {
    pub(crate) fn generic(err: &str) -> Self {
        Self::Generic(err.to_string())
    }

    pub(crate) fn unexpected_invoice_amount(err: &str) -> Self {
        Self::UnexpectedInvoiceAmount(err.to_string())
    }

    pub(crate) fn unexpected_payment_hash(err: &str) -> Self {
        Self::UnexpectedPaymentHash(err.to_string())
    }
}

impl From<anyhow::Error> for ReverseSwapError {
    fn from(err: anyhow::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<hashes::hex::Error> for ReverseSwapError {
    fn from(err: hashes::hex::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<hex::FromHexError> for ReverseSwapError {
    fn from(err: hex::FromHexError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<crate::lightning_invoice::ParseOrSemanticError> for ReverseSwapError {
    fn from(err: crate::lightning_invoice::ParseOrSemanticError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<PersistError> for ReverseSwapError {
    fn from(err: PersistError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<reqwest::Error> for ReverseSwapError {
    fn from(err: reqwest::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<SdkError> for ReverseSwapError {
    fn from(value: SdkError) -> Self {
        match value {
            SdkError::Generic { err } => Self::Generic(err),
            SdkError::ServiceConnectivity { err } => Self::ServiceConnectivity(err),
        }
    }
}

impl From<secp256k1::Error> for ReverseSwapError {
    fn from(err: secp256k1::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<serde_json::Error> for ReverseSwapError {
    fn from(err: serde_json::Error) -> Self {
        Self::ServiceConnectivity(err.to_string())
    }
}

impl From<tonic::Status> for ReverseSwapError {
    fn from(err: tonic::Status) -> Self {
        Self::Generic(err.to_string())
    }
}
