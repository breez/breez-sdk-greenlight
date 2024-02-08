use std::{array::TryFromSliceError, string::FromUtf8Error};

use anyhow::anyhow;

use crate::bitcoin::{bech32, secp256k1, util::bip32};
use crate::{invoice::InvoiceError, node_api::NodeError};

pub type LnUrlResult<T, E = LnUrlError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum LnUrlError {
    #[error("Generic: {0}")]
    Generic(#[from] anyhow::Error),

    #[error(transparent)]
    InvalidInvoice(#[from] InvoiceError),

    #[error("Invalid uri: {0}")]
    InvalidUri(anyhow::Error),

    #[error("Service connectivity: {0}")]
    ServiceConnectivity(anyhow::Error),
}

impl From<aes::cipher::InvalidLength> for LnUrlError {
    fn from(err: aes::cipher::InvalidLength) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<aes::cipher::block_padding::UnpadError> for LnUrlError {
    fn from(err: aes::cipher::block_padding::UnpadError) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<base64::DecodeError> for LnUrlError {
    fn from(err: base64::DecodeError) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<bip32::Error> for LnUrlError {
    fn from(err: bip32::Error) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<bech32::Error> for LnUrlError {
    fn from(err: bech32::Error) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<FromUtf8Error> for LnUrlError {
    fn from(err: FromUtf8Error) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<NodeError> for LnUrlError {
    fn from(err: NodeError) -> Self {
        match err {
            NodeError::InvalidInvoice(err) => Self::InvalidInvoice(err),
            NodeError::ServiceConnectivity(err) => Self::ServiceConnectivity(err),
            _ => Self::Generic(anyhow!(err.to_string())),
        }
    }
}

impl From<secp256k1::Error> for LnUrlError {
    fn from(err: secp256k1::Error) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<serde_json::Error> for LnUrlError {
    fn from(err: serde_json::Error) -> Self {
        Self::ServiceConnectivity(anyhow::Error::new(err))
    }
}

impl From<TryFromSliceError> for LnUrlError {
    fn from(err: TryFromSliceError) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}
