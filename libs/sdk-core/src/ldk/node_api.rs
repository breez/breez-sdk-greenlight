use core::str::FromStr;
use std::collections::HashSet;
use std::pin::Pin;
use std::sync::Arc;

use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::lightning::ln::msgs::SocketAddress;
use ldk_node::{Builder, Node};
use sdk_common::ensure_sdk;
use sdk_common::prelude::Network;
use serde_json::Value;
use tokio::sync::{mpsc, watch};
use tokio_stream::Stream;

use crate::bitcoin::secp256k1::Secp256k1;
use crate::bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey};
use crate::lightning_invoice::RawBolt11Invoice;
use crate::node_api::{
    CreateInvoiceRequest, FetchBolt11Result, IncomingPayment, NodeAPI, NodeError, NodeResult,
};
use crate::{
    CustomMessage, LspInformation, MaxChannelAmount, NodeCredentials, NodeState, Payment,
    PaymentResponse, PrepareRedeemOnchainFundsRequest, PrepareRedeemOnchainFundsResponse,
    RouteHint, RouteHintHop, SyncResponse, TlvEntry,
};

pub(crate) struct Ldk {
    network: Network,
    seed: [u8; 64],
    node: Arc<Node>,
}

impl Ldk {
    pub async fn build(
        config: crate::models::Config,
        seed: &[u8],
        _restore_only: Option<bool>,
    ) -> NodeResult<Self> {
        debug!("Building LDK Node");
        ensure_sdk!(
            config.network == Network::Regtest,
            NodeError::generic("Only Regtest mode is supported for now")
        );

        let mut builder = Builder::new();

        let mut bytes = [0u8; 64];
        bytes.copy_from_slice(seed);
        let seed = bytes;
        builder.set_entropy_seed_bytes(seed);
        builder.set_log_facade_logger();
        builder.set_network(to_ldk_network(&config.network));

        let ldk_config = crate::ldk::config::Config::regtest();
        builder.set_chain_source_esplora(ldk_config.esplora_url.to_string(), None);
        builder.set_gossip_source_rgs(ldk_config.rgs_url.to_string());

        let lsps2 = PublicKey::from_str(ldk_config.lsps2_id)
            .map_err(|e| NodeError::Generic(format!("Invalid public key for LSP: {e}")))?;
        let address = SocketAddress::from_str(ldk_config.lsps2_address)
            .map_err(|e| NodeError::Generic(format!("Invalid address for LSP: {e}")))?;
        builder.set_liquidity_source_lsps2(lsps2, address, None);

        // TODO: Use remote/local storage.
        builder.set_storage_dir_path(config.working_dir);

        let node = builder
            .build()
            .map_err(|e| NodeError::Generic(format!("Fail to build LDK Node: {e}")))?;
        let node = Arc::new(node);
        debug!("LDK Node was built");
        Ok(Self {
            network: config.network,
            seed,
            node,
        })
    }
}

#[tonic::async_trait]
impl NodeAPI for Ldk {
    async fn node_credentials(&self) -> NodeResult<Option<NodeCredentials>> {
        Ok(None)
    }

    async fn configure_node(&self, _close_to_address: Option<String>) -> NodeResult<()> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn create_invoice(&self, _request: CreateInvoiceRequest) -> NodeResult<String> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn delete_invoice(&self, _bolt11: String) -> NodeResult<()> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn fetch_bolt11(&self, _payment_hash: Vec<u8>) -> NodeResult<Option<FetchBolt11Result>> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn pull_changed(
        &self,
        _sync_state: Option<Value>,
        _match_local_balance: bool,
    ) -> NodeResult<SyncResponse> {
        // TODO: Implement.
        Ok(SyncResponse {
            sync_state: Value::Null,
            node_state: NodeState::default(),
            payments: Vec::new(),
            channels: Vec::new(),
        })
    }

    async fn send_payment(
        &self,
        _bolt11: String,
        _amount_msat: Option<u64>,
        _label: Option<String>,
    ) -> NodeResult<Payment> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn send_spontaneous_payment(
        &self,
        _node_id: String,
        _amount_msat: u64,
        _extra_tlvs: Option<Vec<TlvEntry>>,
        _label: Option<String>,
    ) -> NodeResult<Payment> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn send_trampoline_payment(
        &self,
        _bolt11: String,
        _amount_msat: u64,
        _label: Option<String>,
        _trampoline_node_id: Vec<u8>,
    ) -> NodeResult<Payment> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn node_id(&self) -> NodeResult<String> {
        Ok(self.node.node_id().to_string())
    }

    async fn send_pay(&self, _bolt11: String, _max_hops: u32) -> NodeResult<PaymentResponse> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn max_sendable_amount<'a>(
        &self,
        _payee_node_id: Option<Vec<u8>>,
        _max_hops: u32,
        _last_hop: Option<&'a RouteHintHop>,
    ) -> NodeResult<Vec<MaxChannelAmount>> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn redeem_onchain_funds(
        &self,
        _to_address: String,
        _sat_per_vbyte: u32,
    ) -> NodeResult<Vec<u8>> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn prepare_redeem_onchain_funds(
        &self,
        _req: PrepareRedeemOnchainFundsRequest,
    ) -> NodeResult<PrepareRedeemOnchainFundsResponse> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn start(&self, mut shutdown: mpsc::Receiver<()>) {
        debug!("Starting LDK Node");
        if let Err(e) = self.node.start() {
            error!("Failed to start LDK Node: {e}");
            return;
        }
        debug!("LDK Node started");

        _ = shutdown.recv().await;
        debug!("Stopping LDK Node");
        if let Err(e) = self.node.stop() {
            error!("Error on stopping LDK Node: {e}");
        }
        debug!("LDK Node stopped");
    }

    async fn start_keep_alive(&self, _shutdown: watch::Receiver<()>) {
        // No-op for LDK Node.
    }

    async fn connect_peer(&self, _node_id: String, _addr: String) -> NodeResult<()> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn sign_invoice(&self, _invoice: RawBolt11Invoice) -> NodeResult<String> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn close_peer_channels(&self, _node_id: String) -> NodeResult<Vec<String>> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn stream_incoming_payments(
        &self,
    ) -> NodeResult<Pin<Box<dyn Stream<Item = IncomingPayment> + Send>>> {
        // TODO: Implement.
        Ok(Box::pin(futures::stream::pending()))
    }

    async fn stream_log_messages(&self) -> NodeResult<Pin<Box<dyn Stream<Item = String> + Send>>> {
        // LDK Node is configured with facade logger.
        Ok(Box::pin(futures::stream::empty()))
    }

    async fn static_backup(&self) -> NodeResult<Vec<String>> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn execute_command(&self, _command: String) -> NodeResult<Value> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn generate_diagnostic_data(&self) -> NodeResult<Value> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn sign_message(&self, _message: &str) -> NodeResult<String> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn check_message(
        &self,
        _message: &str,
        _pubkey: &str,
        _signature: &str,
    ) -> NodeResult<bool> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn send_custom_message(&self, _message: CustomMessage) -> NodeResult<()> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn stream_custom_messages(
        &self,
    ) -> NodeResult<Pin<Box<dyn Stream<Item = anyhow::Result<CustomMessage>> + Send>>> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn derive_bip32_key(&self, path: Vec<ChildNumber>) -> NodeResult<ExtendedPrivKey> {
        Ok(
            ExtendedPrivKey::new_master(self.network.into(), &self.seed)?
                .derive_priv(&Secp256k1::new(), &path)?,
        )
    }

    async fn legacy_derive_bip32_key(&self, path: Vec<ChildNumber>) -> NodeResult<ExtendedPrivKey> {
        // Using the main implementation, because legacy way was never used for LDK.
        self.derive_bip32_key(path).await
    }

    async fn get_routing_hints(
        &self,
        _lsp_info: &LspInformation,
    ) -> NodeResult<(Vec<RouteHint>, bool)> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }

    async fn get_open_peers(&self) -> NodeResult<HashSet<Vec<u8>>> {
        Err(NodeError::generic("LDK implementation not yet available"))
    }
}

fn to_ldk_network(network: &Network) -> ldk_node::bitcoin::network::Network {
    match network {
        Network::Bitcoin => ldk_node::bitcoin::network::Network::Bitcoin,
        Network::Testnet => ldk_node::bitcoin::network::Network::Testnet,
        Network::Signet => ldk_node::bitcoin::network::Network::Signet,
        Network::Regtest => ldk_node::bitcoin::network::Network::Regtest,
    }
}
