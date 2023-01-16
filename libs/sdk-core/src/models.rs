use anyhow::{anyhow, Result};
use gl_client::pb::Peer;
use gl_client::pb::WithdrawResponse;
use gl_client::pb::{CloseChannelResponse, Invoice};
use lightning_invoice::RawInvoice;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;
use tokio::sync::mpsc;
use tonic::Streaming;

use crate::fiat::{FiatCurrency, Rate};
use crate::grpc::{PaymentInformation, RegisterPaymentReply};
use crate::lsp::LspInformation;
use crate::models::Network::*;

/// Different types of supported payments
#[derive(Clone, PartialEq, Eq, Debug, EnumString, Display, Deserialize, Serialize)]
pub enum PaymentType {
    Sent,
    Received,
    ClosedChannel,
}

#[tonic::async_trait]
pub trait NodeAPI: Send + Sync {
    async fn create_invoice(
        &self,
        amount_sats: u64,
        description: String,
        preimage: Option<Vec<u8>>,
    ) -> Result<Invoice>;
    async fn pull_changed(&self, since_timestamp: i64) -> Result<SyncResponse>;
    /// As per the `pb::PayRequest` docs, `amount_sats` is only needed when the invoice doesn't specify an amount
    async fn send_payment(
        &self,
        bolt11: String,
        amount_sats: Option<u64>,
    ) -> Result<gl_client::pb::Payment>;
    async fn send_spontaneous_payment(
        &self,
        node_id: String,
        amount_sats: u64,
    ) -> Result<gl_client::pb::Payment>;
    async fn start(&self) -> Result<()>;
    async fn sweep(
        &self,
        to_address: String,
        fee_rate_sats_per_byte: u64,
    ) -> Result<WithdrawResponse>;
    async fn start_signer(&self, shutdown: mpsc::Receiver<()>);
    async fn list_peers(&self) -> Result<Vec<Peer>>;
    async fn connect_peer(&self, node_id: String, addr: String) -> Result<()>;
    fn sign_invoice(&self, invoice: RawInvoice) -> Result<String>;
    async fn close_peer_channels(&self, node_id: String) -> Result<CloseChannelResponse>;
    async fn stream_incoming_payments(&self) -> Result<Streaming<gl_client::pb::IncomingPayment>>;
    async fn stream_log_messages(&self) -> Result<Streaming<gl_client::pb::LogEntry>>;
    async fn execute_command(&self, command: String) -> Result<String>;
}

#[tonic::async_trait]
pub trait LspAPI: Send + Sync {
    async fn list_lsps(&self, node_pubkey: String) -> Result<Vec<LspInformation>>;
    async fn register_payment(
        &self,
        lsp_id: String,
        lsp_pubkey: Vec<u8>,
        payment_info: PaymentInformation,
    ) -> Result<RegisterPaymentReply>;
}

#[tonic::async_trait]
pub trait FiatAPI: Send + Sync {
    fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>>;
    async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>>;
}

pub struct Swap {
    pub bitcoin_address: String,
    pub swapper_pubkey: Vec<u8>,
    pub lock_height: i64,
    pub max_allowed_deposit: i64,
    pub error_message: String,
    pub required_reserve: i64,
    pub min_allowed_deposit: i64,
}

#[tonic::async_trait]
pub trait SwapperAPI: Send + Sync {
    async fn create_swap(
        &self,
        hash: Vec<u8>,
        payer_pubkey: Vec<u8>,
        node_pubkey: String,
    ) -> Result<Swap>;

    async fn complete_swap(&self, bolt11: String) -> Result<()>;
}

/// Internal SDK log entry
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub line: String,
    pub level: String,
}

/// Configuration for the Breez Services.
///
/// Use [Config::production] or [Config::staging] for default configs of the different supported
/// environments.
#[derive(Clone)]
pub struct Config {
    pub breezserver: String,
    pub mempoolspace_url: String,
    pub working_dir: String,
    pub network: Network,
    pub payment_timeout_sec: u32,
    pub default_lsp_id: Option<String>,
    pub api_key: Option<String>,
}

impl Config {
    pub fn production() -> Self {
        Config {
            breezserver: "https://bs1-st.breez.technology:443".to_string(),
            mempoolspace_url: "https://mempool.space".to_string(),
            working_dir: ".".to_string(),
            network: Bitcoin,
            payment_timeout_sec: 30,
            default_lsp_id: Some(String::from("ea51d025-042d-456c-8325-63e430797481")),
            api_key: None,
        }
    }

    pub fn staging() -> Self {
        // TODO Update with staging values
        Config {
            breezserver: "https://bs1-st.breez.technology:443".to_string(),
            mempoolspace_url: "https://mempool.space".to_string(),
            working_dir: ".".to_string(),
            network: Bitcoin,
            payment_timeout_sec: 30,
            default_lsp_id: Some(String::from("ea51d025-042d-456c-8325-63e430797481")),
            api_key: None,
        }
    }
}

/// Indicates the different kinds of supported environments for [crate::BreezServices]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnvironmentType {
    Production,
    Staging,
}

/// Client-specific credentials to connect to and manage a Greenlight node in the cloud
#[derive(Clone, Serialize, Deserialize)]
pub struct GreenlightCredentials {
    pub device_key: Vec<u8>,
    pub device_cert: Vec<u8>,
}

/// The different supported bitcoin networks
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Network {
    /// Mainnet
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

impl From<bitcoin::network::constants::Network> for Network {
    fn from(network: bitcoin::network::constants::Network) -> Self {
        match network {
            bitcoin::network::constants::Network::Bitcoin => Bitcoin,
            bitcoin::network::constants::Network::Testnet => Testnet,
            bitcoin::network::constants::Network::Signet => Signet,
            bitcoin::network::constants::Network::Regtest => Regtest,
        }
    }
}

impl From<Network> for bitcoin::network::constants::Network {
    fn from(network: Network) -> Self {
        match network {
            Bitcoin => bitcoin::network::constants::Network::Bitcoin,
            Testnet => bitcoin::network::constants::Network::Testnet,
            Signet => bitcoin::network::constants::Network::Signet,
            Regtest => bitcoin::network::constants::Network::Regtest,
        }
    }
}

/// Different types of supported filters which can be applied when retrieving the transaction list
pub enum PaymentTypeFilter {
    Sent,
    Received,
    All,
}

/// Different types of supported feerates
pub enum FeeratePreset {
    Regular,
    Economy,
    Priority,
}

impl TryFrom<i32> for FeeratePreset {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(FeeratePreset::Regular),
            1 => Ok(FeeratePreset::Economy),
            2 => Ok(FeeratePreset::Priority),
            _ => Err(anyhow!("Unexpected feerate enum value")),
        }
    }
}

/// The node state of a Greenlight LN node running in the cloud
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct NodeState {
    pub id: String,
    pub block_height: u32,
    pub channels_balance_msat: u64,
    pub onchain_balance_msat: u64,

    #[serde(default)]
    pub utxos: Vec<UnspentTransactionOutput>,
    pub max_payable_msat: u64,
    pub max_receivable_msat: u64,
    pub max_single_payment_amount_msat: u64,
    pub max_chan_reserve_msats: u64,
    pub connected_peers: Vec<String>,
    pub inbound_liquidity_msats: u64,
}

pub(crate) struct SyncResponse {
    pub node_state: NodeState,
    pub payments: Vec<crate::models::Payment>,
    pub channels: Vec<crate::models::Channel>,
}

/// Represents a payment, including its [PaymentType] and [PaymentDetails]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Payment {
    pub id: String,
    pub payment_type: PaymentType,
    pub payment_time: i64,
    pub amount_msat: i64,
    pub fee_msat: i64,
    pub pending: bool,
    pub description: Option<String>,
    pub details: PaymentDetails,
}

/// Wrapper for the different types of payments
#[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PaymentDetails {
    Ln {
        #[serde(flatten)]
        data: LnPaymentDetails,
    },
    ClosedChannel {
        #[serde(flatten)]
        data: ClosesChannelPaymentDetails,
    },
}

/// Details of a LN payment, as included in a [Payment]
#[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
pub struct LnPaymentDetails {
    pub payment_hash: String,
    pub label: String,
    pub destination_pubkey: String,
    pub payment_preimage: String,
    pub keysend: bool,
    pub bolt11: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
pub struct ClosesChannelPaymentDetails {
    pub short_channel_id: String,
    pub state: ChannelState,
    pub funding_txid: String,
}

/// LN channel managed by the LSP
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Channel {
    pub funding_txid: String,
    pub short_channel_id: String,
    pub state: ChannelState,
    pub spendable_msat: u64,
    pub receivable_msat: u64,
    pub closed_at: Option<u64>,
}

/// State of a LN channel
#[derive(Clone, PartialEq, Eq, Debug, EnumString, Display, Deserialize, Serialize)]
pub enum ChannelState {
    PendingOpen,
    Opened,
    PendingClose,
    Closed,
}

/// The status of a swap
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SwapStatus {
    /// The swap address has been created and either there aren't any confirmed transactions associated with it
    /// or there are confirmed transactions that are bellow the lock timeout which means the funds are still
    /// eligible to be redeemed normally.
    Initial = 0,

    /// The swap address has confirmed transactions associated with it and the lock timeout has passed since
    /// the earliest confirmed transaction. This means the only way to spend the funds from this address is by
    /// broadcasting a refund transaction.
    Expired = 1,
}

impl TryFrom<i32> for SwapStatus {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SwapStatus::Initial),
            1 => Ok(SwapStatus::Expired),
            _ => Err(anyhow!("illegal value")),
        }
    }
}

/// Represents the details of an on-going swap.
///
/// Once this SwapInfo is created it will be monitored on-chain and its state is
/// saved to the persistent storage.
///
/// The SwapInfo has a status which changes accordingly, documented in [SwapStatus].
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwapInfo {
    //static immutable data
    pub bitcoin_address: String,
    pub created_at: i64,
    pub lock_height: i64,
    pub payment_hash: Vec<u8>,
    pub preimage: Vec<u8>,
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub swapper_public_key: Vec<u8>,
    pub script: Vec<u8>,

    // dynamic data
    pub bolt11: Option<String>,
    pub paid_sats: u32,
    pub confirmed_sats: u32,
    pub status: SwapStatus,
    pub refund_tx_ids: Vec<String>,
    pub confirmed_tx_ids: Vec<String>,
    pub min_allowed_deposit: i64,
    pub max_allowed_deposit: i64,
}

impl SwapInfo {
    pub(crate) fn redeemable(&self) -> bool {
        self.confirmed_sats > 0 && self.paid_sats == 0 && self.status != SwapStatus::Expired
    }
}

pub(crate) fn parse_short_channel_id(id_str: &str) -> Result<u64> {
    let parts: Vec<&str> = id_str.split('x').collect();
    if parts.len() != 3 {
        return Ok(0);
    }
    let block_num = parts[0].parse::<u64>()?;
    let tx_num = parts[1].parse::<u64>()?;
    let tx_out = parts[2].parse::<u64>()?;

    Ok((block_num & 0xFFFFFF) << 40 | (tx_num & 0xFFFFFF) << 16 | (tx_out & 0xFFFF))
}

/// UTXO known to the LN node
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct UnspentTransactionOutput {
    pub txid: Vec<u8>,
    pub outnum: u32,
    pub amount_millisatoshi: u64,
    pub address: String,
}

#[cfg(test)]
mod tests {
    use prost::Message;
    use rand::random;

    use crate::grpc::PaymentInformation;
    use crate::test_utils::rand_vec_u8;

    #[test]
    fn test_payment_information_ser_de() -> Result<(), Box<dyn std::error::Error>> {
        let dummy_payment_info = PaymentInformation {
            payment_hash: rand_vec_u8(10),
            payment_secret: rand_vec_u8(10),
            destination: rand_vec_u8(10),
            incoming_amount_msat: random(),
            outgoing_amount_msat: random(),
        };

        let mut buf = Vec::new();
        buf.reserve(dummy_payment_info.encoded_len());
        dummy_payment_info.encode(&mut buf)?;

        let decoded_payment_info: PaymentInformation = PaymentInformation::decode(&*buf)?;

        assert_eq!(dummy_payment_info, decoded_payment_info);

        Ok(())
    }
}
