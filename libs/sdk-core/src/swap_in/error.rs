use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReceiveSwapError {
    Generic(String),
    InvalidAddressType,
    NoUtxos,
    PaymentError(String),
    SwapNotFound(String),
    OutputValueBelowDust,
}

impl ReceiveSwapError {
    pub fn generic(msg: impl Into<String>) -> Self {
        Self::Generic(msg.into())
    }
}
pub type ReceiveSwapResult<T, E = ReceiveSwapError> = Result<T, E>;

#[derive(Clone, Debug, Error)]
pub(super) enum GetPaymentRequestError {
    #[error("needs new fee params")]
    NeedsNewFeeParams,
    #[error("invoice already exists")]
    InvoiceAlreadyExists,
    #[error("{0}")]
    Generic(String),
}
impl GetPaymentRequestError {
    pub fn generic(msg: impl Into<String>) -> Self {
        Self::Generic(msg.into())
    }
}
