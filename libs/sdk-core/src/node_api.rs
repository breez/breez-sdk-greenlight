use std::collections::HashSet;
use std::pin::Pin;

use anyhow::Result;
use serde_json::Value;
use tokio::sync::{mpsc, watch};
use tokio_stream::Stream;

use sdk_common::prelude::*;

use crate::{
    bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey},
    lightning_invoice::RawBolt11Invoice,
    persist::error::PersistError,
    CustomMessage, LnUrlAuthError, LspInformation, MaxChannelAmount, NodeCredentials, Payment,
    PaymentResponse, PrepareRedeemOnchainFundsRequest, PrepareRedeemOnchainFundsResponse,
    RouteHint, RouteHintHop, SyncResponse, TlvEntry,
};

pub type NodeResult<T, E = NodeError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("{0}")]
    Credentials(String),

    #[error("{0}")]
    Generic(String),

    #[error(transparent)]
    InvalidInvoice(#[from] InvoiceError),

    #[error("{0}")]
    InvoiceExpired(String),

    #[error("{0}")]
    InvoiceNoDescription(String),

    #[error("{0}")]
    InvoicePreimageAlreadyExists(String),

    #[error("{0}")]
    PaymentFailed(String),

    #[error("{0}")]
    PaymentTimeout(String),

    #[error(transparent)]
    Persistance(#[from] PersistError),

    #[error("{0}")]
    RestoreOnly(String),

    #[error("{0}")]
    RouteTooExpensive(String),

    #[error("{0}")]
    RouteNotFound(String),

    #[error("{0}")]
    ServiceConnectivity(String),

    #[error("{0}")]
    InsufficientFunds(String),

    #[error("invoice already paid")]
    InvoiceAlreadyPaid,
}

impl NodeError {
    pub(crate) fn credentials(err: &str) -> Self {
        Self::Credentials(err.to_string())
    }

    pub(crate) fn generic(err: &str) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<NodeError> for sdk_common::prelude::LnUrlError {
    fn from(value: NodeError) -> Self {
        match value {
            NodeError::InvalidInvoice(err) => Self::InvalidInvoice(format!("{err}")),
            NodeError::ServiceConnectivity(err) => Self::ServiceConnectivity(err),
            _ => Self::Generic(value.to_string()),
        }
    }
}

impl From<NodeError> for LnUrlAuthError {
    fn from(value: NodeError) -> Self {
        match value {
            NodeError::ServiceConnectivity(err) => Self::ServiceConnectivity { err },
            _ => Self::Generic {
                err: value.to_string(),
            },
        }
    }
}

pub struct CreateInvoiceRequest {
    pub amount_msat: u64,
    pub description: String,
    pub payer_amount_msat: Option<u64>,
    pub preimage: Option<Vec<u8>>,
    pub use_description_hash: Option<bool>,
    pub expiry: Option<u32>,
    pub cltv: Option<u32>,
}

pub struct FetchBolt11Result {
    pub bolt11: String,
    pub payer_amount_msat: Option<u64>,
}

/// Trait covering functions affecting the LN node
#[cfg_attr(test, mockall::automock)]
#[tonic::async_trait]
pub trait NodeAPI: Send + Sync {
    async fn node_credentials(&self) -> NodeResult<Option<NodeCredentials>>;
    async fn configure_node(&self, close_to_address: Option<String>) -> NodeResult<()>;
    async fn create_invoice(&self, request: CreateInvoiceRequest) -> NodeResult<String>;
    async fn delete_invoice(&self, bolt11: String) -> NodeResult<()>;
    /// Fetches an existing BOLT11 invoice from the node
    async fn fetch_bolt11(&self, payment_hash: Vec<u8>) -> NodeResult<Option<FetchBolt11Result>>;
    async fn pull_changed(
        &self,
        sync_state: Option<Value>,
        match_local_balance: bool,
    ) -> NodeResult<SyncResponse>;
    /// As per the `pb::PayRequest` docs, `amount_msat` is only needed when the invoice doesn't specify an amount
    async fn send_payment(
        &self,
        bolt11: String,
        amount_msat: Option<u64>,
        label: Option<String>,
    ) -> NodeResult<Payment>;
    async fn send_spontaneous_payment(
        &self,
        node_id: String,
        amount_msat: u64,
        extra_tlvs: Option<Vec<TlvEntry>>,
        label: Option<String>,
    ) -> NodeResult<Payment>;
    async fn send_trampoline_payment(
        &self,
        bolt11: String,
        amount_msat: u64,
        label: Option<String>,
        trampoline_node_id: Vec<u8>,
    ) -> NodeResult<Payment>;
    async fn node_id(&self) -> NodeResult<String>;

    /// Attempts to find a payment path "manually" and send the htlcs in a way that will drain
    /// Large channels first.
    /// This is useful function to send the largest amount possible to a node.
    async fn send_pay(&self, bolt11: String, max_hops: u32) -> NodeResult<PaymentResponse>;

    /// Calculates the maximum amount that can be sent to a node.
    async fn max_sendable_amount<'a>(
        &self,
        payee_node_id: Option<Vec<u8>>,
        max_hops: u32,
        last_hop: Option<&'a RouteHintHop>,
    ) -> NodeResult<Vec<MaxChannelAmount>>;
    async fn redeem_onchain_funds(
        &self,
        to_address: String,
        sat_per_vbyte: u32,
    ) -> NodeResult<Vec<u8>>;
    async fn prepare_redeem_onchain_funds(
        &self,
        req: PrepareRedeemOnchainFundsRequest,
    ) -> NodeResult<PrepareRedeemOnchainFundsResponse>;
    async fn start_signer(&self, shutdown: mpsc::Receiver<()>);
    async fn start_keep_alive(&self, shutdown: watch::Receiver<()>);
    async fn connect_peer(&self, node_id: String, addr: String) -> NodeResult<()>;
    async fn sign_invoice(&self, invoice: RawBolt11Invoice) -> NodeResult<String>;
    async fn close_peer_channels(&self, node_id: String) -> NodeResult<Vec<String>>;
    async fn stream_incoming_payments(
        &self,
    ) -> NodeResult<Pin<Box<dyn Stream<Item = gl_client::signer::model::greenlight::IncomingPayment> + Send>>>;
    async fn stream_log_messages(
        &self,
    ) -> NodeResult<Pin<Box<dyn Stream<Item = gl_client::signer::model::greenlight::LogEntry> + Send>>>;
    async fn static_backup(&self) -> NodeResult<Vec<String>>;
    async fn execute_command(&self, command: String) -> NodeResult<Value>;
    async fn generate_diagnostic_data(&self) -> NodeResult<Value>;
    async fn sign_message(&self, message: &str) -> NodeResult<String>;
    async fn check_message(&self, message: &str, pubkey: &str, signature: &str)
        -> NodeResult<bool>;
    async fn send_custom_message(&self, message: CustomMessage) -> NodeResult<()>;
    async fn stream_custom_messages(
        &self,
    ) -> NodeResult<Pin<Box<dyn Stream<Item = Result<CustomMessage>> + Send>>>;

    /// Gets the private key at the path specified
    async fn derive_bip32_key(&self, path: Vec<ChildNumber>) -> NodeResult<ExtendedPrivKey>;
    async fn legacy_derive_bip32_key(&self, path: Vec<ChildNumber>) -> NodeResult<ExtendedPrivKey>;

    /// Gets the routing hints related to all private channels that the node has.
    /// Also returns a boolean indicating if the node has a public channel or not.
    async fn get_routing_hints(
        &self,
        lsp_info: &LspInformation,
    ) -> NodeResult<(Vec<RouteHint>, bool)>;
    /// Get peers with whom we have an open channel
    async fn get_open_peers(&self) -> NodeResult<HashSet<Vec<u8>>>;
}
