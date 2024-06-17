use std::{array::TryFromSliceError, string::FromUtf8Error};

use bitcoin::{bech32, secp256k1, util::bip32};

use crate::prelude::InvoiceError;

pub type LnUrlResult<T, E = LnUrlError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum LnUrlError {
    #[error("{0}")]
    Generic(String),

    #[error("{0}")]
    InvalidInvoice(String),

    #[error("{0}")]
    InvalidUri(String),

    #[error("{0}")]
    ServiceConnectivity(String),
}

impl LnUrlError {
    pub fn generic(err: &str) -> Self {
        Self::Generic(err.to_string())
    }

    pub fn invalid_uri(err: &str) -> Self {
        Self::InvalidUri(err.to_string())
    }
}

impl From<aes::cipher::InvalidLength> for LnUrlError {
    fn from(err: aes::cipher::InvalidLength) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<aes::cipher::block_padding::UnpadError> for LnUrlError {
    fn from(err: aes::cipher::block_padding::UnpadError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<base64::DecodeError> for LnUrlError {
    fn from(err: base64::DecodeError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<bip32::Error> for LnUrlError {
    fn from(err: bip32::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<bech32::Error> for LnUrlError {
    fn from(err: bech32::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<FromUtf8Error> for LnUrlError {
    fn from(err: FromUtf8Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<secp256k1::Error> for LnUrlError {
    fn from(err: secp256k1::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<serde_json::Error> for LnUrlError {
    fn from(err: serde_json::Error) -> Self {
        Self::ServiceConnectivity(err.to_string())
    }
}

impl From<TryFromSliceError> for LnUrlError {
    fn from(err: TryFromSliceError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<InvoiceError> for LnUrlError {
    fn from(value: InvoiceError) -> Self {
        LnUrlError::InvalidInvoice(format!("{value}"))
    }
}
