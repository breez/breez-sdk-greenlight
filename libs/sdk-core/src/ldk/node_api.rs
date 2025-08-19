use std::collections::HashSet;
use std::pin::Pin;

use anyhow::Result;
use serde_json::Value;
use tokio::sync::{mpsc, watch};
use tokio_stream::Stream;

use crate::bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey};
use crate::lightning_invoice::RawBolt11Invoice;
use crate::models::Config;
use crate::node_api::{
    CreateInvoiceRequest, FetchBolt11Result, IncomingPayment, NodeAPI, NodeError, NodeResult,
};
use crate::{
    CustomMessage, LspInformation, MaxChannelAmount, NodeCredentials, Payment, PaymentResponse,
    PrepareRedeemOnchainFundsRequest, PrepareRedeemOnchainFundsResponse, RouteHint, RouteHintHop,
    SyncResponse, TlvEntry,
};

/// Stub implementation of NodeAPI for LDK
/// This is a placeholder implementation that returns errors for all methods
/// TODO: Implement actual LDK functionality
pub(crate) struct Ldk {
    // TODO: Add actual LDK node fields
}

impl Ldk {
    pub async fn build(_config: Config, _seed: &[u8], _restore_only: Option<bool>) -> Self {
        // TODO: Implement actual LDK node building
        Self {}
    }
}

#[tonic::async_trait]
impl NodeAPI for Ldk {
    async fn node_credentials(&self) -> NodeResult<Option<NodeCredentials>> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn configure_node(&self, _close_to_address: Option<String>) -> NodeResult<()> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn create_invoice(&self, _request: CreateInvoiceRequest) -> NodeResult<String> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn delete_invoice(&self, _bolt11: String) -> NodeResult<()> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn fetch_bolt11(&self, _payment_hash: Vec<u8>) -> NodeResult<Option<FetchBolt11Result>> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn pull_changed(
        &self,
        _sync_state: Option<Value>,
        _match_local_balance: bool,
    ) -> NodeResult<SyncResponse> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn send_payment(
        &self,
        _bolt11: String,
        _amount_msat: Option<u64>,
        _label: Option<String>,
    ) -> NodeResult<Payment> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn send_spontaneous_payment(
        &self,
        _node_id: String,
        _amount_msat: u64,
        _extra_tlvs: Option<Vec<TlvEntry>>,
        _label: Option<String>,
    ) -> NodeResult<Payment> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn send_trampoline_payment(
        &self,
        _bolt11: String,
        _amount_msat: u64,
        _label: Option<String>,
        _trampoline_node_id: Vec<u8>,
    ) -> NodeResult<Payment> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn node_id(&self) -> NodeResult<String> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn send_pay(&self, _bolt11: String, _max_hops: u32) -> NodeResult<PaymentResponse> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn max_sendable_amount<'a>(
        &self,
        _payee_node_id: Option<Vec<u8>>,
        _max_hops: u32,
        _last_hop: Option<&'a RouteHintHop>,
    ) -> NodeResult<Vec<MaxChannelAmount>> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn redeem_onchain_funds(
        &self,
        _to_address: String,
        _sat_per_vbyte: u32,
    ) -> NodeResult<Vec<u8>> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn prepare_redeem_onchain_funds(
        &self,
        _req: PrepareRedeemOnchainFundsRequest,
    ) -> NodeResult<PrepareRedeemOnchainFundsResponse> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn start_signer(&self, _shutdown: mpsc::Receiver<()>) {
        // No-op for stub implementation
    }

    async fn start_keep_alive(&self, _shutdown: watch::Receiver<()>) {
        // No-op for stub implementation
    }

    async fn connect_peer(&self, _node_id: String, _addr: String) -> NodeResult<()> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn sign_invoice(&self, _invoice: RawBolt11Invoice) -> NodeResult<String> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn close_peer_channels(&self, _node_id: String) -> NodeResult<Vec<String>> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn stream_incoming_payments(
        &self,
    ) -> NodeResult<Pin<Box<dyn Stream<Item = IncomingPayment> + Send>>> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn stream_log_messages(&self) -> NodeResult<Pin<Box<dyn Stream<Item = String> + Send>>> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn static_backup(&self) -> NodeResult<Vec<String>> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn execute_command(&self, _command: String) -> NodeResult<Value> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn generate_diagnostic_data(&self) -> NodeResult<Value> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn sign_message(&self, _message: &str) -> NodeResult<String> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn check_message(
        &self,
        _message: &str,
        _pubkey: &str,
        _signature: &str,
    ) -> NodeResult<bool> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn send_custom_message(&self, _message: CustomMessage) -> NodeResult<()> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn stream_custom_messages(
        &self,
    ) -> NodeResult<Pin<Box<dyn Stream<Item = Result<CustomMessage>> + Send>>> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn derive_bip32_key(&self, _path: Vec<ChildNumber>) -> NodeResult<ExtendedPrivKey> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn legacy_derive_bip32_key(
        &self,
        _path: Vec<ChildNumber>,
    ) -> NodeResult<ExtendedPrivKey> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn get_routing_hints(
        &self,
        _lsp_info: &LspInformation,
    ) -> NodeResult<(Vec<RouteHint>, bool)> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }

    async fn get_open_peers(&self) -> NodeResult<HashSet<Vec<u8>>> {
        Err(NodeError::Generic(
            "LDK implementation not yet available".to_string(),
        ))
    }
}
