use std::time::SystemTimeError;

use hex::FromHexError;
use secp256k1::musig::{MusigSignError, MusigTweakErr};
use thiserror::Error;

use crate::bitcoin;
use crate::lightning_invoice::ParseOrSemanticError;
use crate::{
    error::{ReceivePaymentError, SdkError},
    node_api::NodeError,
    persist::error::PersistError,
};

#[derive(Debug, Error)]
pub enum ReceiveSwapError {
    #[error("{0}")]
    Generic(String),
    #[error("Invalid address type")]
    InvalidAddressType,
    #[error("Node state not found")]
    NodeStateNotFound,
    #[error("No utxos found")]
    NoUtxos,
    #[error("Utxos are still timelocked")]
    UtxosTimelocked,
    #[error("Payment error: {0}")]
    PaymentError(String),
    #[error("Swap not found: {0}")]
    SwapNotFound(String),
    #[error("Output value for refund would be below dust")]
    OutputValueBelowDust,
    #[error("Persist error: {0}")]
    Persist(PersistError),
    #[error("Service connectivity error: {0}")]
    ServiceConnectivity(String),
    #[error("Missing opening fee params")]
    MissingOpeningFeeParams,
    #[error("Taproot error: {0}")]
    Taproot(String),
    #[error("Unsupported swap limits: {0}")]
    UnsupportedSwapLimits(String),
}

impl ReceiveSwapError {
    pub fn generic(msg: impl Into<String>) -> Self {
        Self::Generic(msg.into())
    }

    pub fn unsupported_swap_limits(msg: impl Into<String>) -> Self {
        Self::UnsupportedSwapLimits(msg.into())
    }
}
pub type ReceiveSwapResult<T, E = ReceiveSwapError> = Result<T, E>;

impl From<bitcoin::hashes::hex::Error> for ReceiveSwapError {
    fn from(e: bitcoin::hashes::hex::Error) -> Self {
        Self::Generic(e.to_string())
    }
}

impl From<bitcoin::util::address::Error> for ReceiveSwapError {
    fn from(e: bitcoin::util::address::Error) -> Self {
        Self::Generic(e.to_string())
    }
}

impl From<PersistError> for ReceiveSwapError {
    fn from(e: PersistError) -> Self {
        Self::Persist(e)
    }
}

impl From<SdkError> for ReceiveSwapError {
    fn from(e: SdkError) -> Self {
        match e {
            SdkError::Generic { err } => ReceiveSwapError::Generic(err),
            SdkError::ServiceConnectivity { err } => ReceiveSwapError::ServiceConnectivity(err),
        }
    }
}

impl From<bitcoin::util::sighash::Error> for ReceiveSwapError {
    fn from(e: bitcoin::util::sighash::Error) -> Self {
        Self::Generic(e.to_string())
    }
}

impl From<secp256k1::musig::ParseError> for ReceiveSwapError {
    fn from(e: secp256k1::musig::ParseError) -> Self {
        Self::Taproot(e.to_string())
    }
}

impl From<MusigSignError> for ReceiveSwapError {
    fn from(e: MusigSignError) -> Self {
        Self::Taproot(e.to_string())
    }
}

impl From<secp256k1::Error> for ReceiveSwapError {
    fn from(e: secp256k1::Error) -> Self {
        Self::Taproot(e.to_string())
    }
}

impl From<bitcoin::secp256k1::Error> for ReceiveSwapError {
    fn from(e: bitcoin::secp256k1::Error) -> Self {
        Self::Taproot(e.to_string())
    }
}

impl From<bitcoin::util::taproot::TaprootBuilderError> for ReceiveSwapError {
    fn from(e: bitcoin::util::taproot::TaprootBuilderError) -> Self {
        Self::Taproot(e.to_string())
    }
}

impl From<bitcoin::util::taproot::TaprootBuilder> for ReceiveSwapError {
    fn from(_e: bitcoin::util::taproot::TaprootBuilder) -> Self {
        Self::Taproot("Could not finalize taproot spend info".to_string())
    }
}

impl From<anyhow::Error> for ReceiveSwapError {
    fn from(e: anyhow::Error) -> Self {
        Self::Generic(e.to_string())
    }
}

impl From<SystemTimeError> for ReceiveSwapError {
    fn from(e: SystemTimeError) -> Self {
        Self::Generic(e.to_string())
    }
}

impl From<GetPaymentRequestError> for ReceiveSwapError {
    fn from(value: GetPaymentRequestError) -> Self {
        match value {
            GetPaymentRequestError::NeedsNewFeeParams => {
                ReceiveSwapError::generic("Opening fee params are no longer valid")
            }
            GetPaymentRequestError::InvoiceAlreadyExists => {
                ReceiveSwapError::generic("Failed to create invoice. Invoice already exists")
            }
            GetPaymentRequestError::ServiceConnectivity(msg) => {
                ReceiveSwapError::ServiceConnectivity(msg)
            }
            GetPaymentRequestError::Generic(msg) => ReceiveSwapError::Generic(msg),
            GetPaymentRequestError::MissingOpeningFeeParams => {
                ReceiveSwapError::MissingOpeningFeeParams
            }
        }
    }
}

impl From<FromHexError> for ReceiveSwapError {
    fn from(_value: FromHexError) -> Self {
        Self::generic("could not convert from hex")
    }
}

impl From<MusigTweakErr> for ReceiveSwapError {
    fn from(value: MusigTweakErr) -> Self {
        ReceiveSwapError::Taproot(value.to_string())
    }
}

impl From<secp256k1::scalar::OutOfRangeError> for ReceiveSwapError {
    fn from(value: secp256k1::scalar::OutOfRangeError) -> Self {
        ReceiveSwapError::Taproot(value.to_string())
    }
}

#[derive(Clone, Debug, Error)]
pub(super) enum GetPaymentRequestError {
    #[error("Needs new fee params")]
    NeedsNewFeeParams,
    #[error("Invoice already exists")]
    InvoiceAlreadyExists,
    #[error("Service connectivity error: {0}")]
    ServiceConnectivity(String),
    #[error("{0}")]
    Generic(String),
    #[error("Missing opening fee params")]
    MissingOpeningFeeParams,
}
impl GetPaymentRequestError {
    pub fn generic(msg: impl Into<String>) -> Self {
        Self::Generic(msg.into())
    }
}

impl From<ParseOrSemanticError> for GetPaymentRequestError {
    fn from(e: ParseOrSemanticError) -> Self {
        Self::Generic(e.to_string())
    }
}

impl From<NodeError> for GetPaymentRequestError {
    fn from(e: NodeError) -> Self {
        Self::Generic(e.to_string())
    }
}

impl From<ReceivePaymentError> for GetPaymentRequestError {
    fn from(e: ReceivePaymentError) -> Self {
        match e {
            ReceivePaymentError::InvoicePreimageAlreadyExists { err: _ } => {
                GetPaymentRequestError::InvoiceAlreadyExists
            }
            ReceivePaymentError::ServiceConnectivity { err } => {
                GetPaymentRequestError::ServiceConnectivity(err)
            }
            _ => GetPaymentRequestError::Generic(e.to_string()),
        }
    }
}

impl From<anyhow::Error> for GetPaymentRequestError {
    fn from(e: anyhow::Error) -> Self {
        Self::Generic(e.to_string())
    }
}

impl From<PersistError> for GetPaymentRequestError {
    fn from(e: PersistError) -> Self {
        Self::Generic(e.to_string())
    }
}
