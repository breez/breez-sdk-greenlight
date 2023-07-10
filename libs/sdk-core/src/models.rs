use std::str::FromStr;

use crate::boltzswap::{BoltzApiCreateReverseSwapResponse, BoltzApiReverseSwapStatus};
use anyhow::{anyhow, Result};
use bitcoin::blockdata::opcodes;
use bitcoin::blockdata::script::Builder;
use bitcoin::hashes::hex::{FromHex, ToHex};
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::{PublicKey, Secp256k1, SecretKey};
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey};
use bitcoin::{Address, Script};
use gl_client::pb::Invoice;
use gl_client::pb::Peer;
use gl_client::pb::WithdrawResponse;
use lightning_invoice::RawInvoice;
use ripemd::Digest;
use ripemd::Ripemd160;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;
use tokio::sync::mpsc;
use tonic::Streaming;

use crate::fiat::{FiatCurrency, Rate};
use crate::grpc::{PaymentInformation, RegisterPaymentReply};
use crate::lnurl::pay::model::SuccessActionProcessed;
use crate::lsp::LspInformation;
use crate::models::Network::*;
use crate::LnUrlErrorData;

/// Different types of supported payments
#[derive(Clone, PartialEq, Eq, Debug, EnumString, Display, Deserialize, Serialize)]
pub enum PaymentType {
    Sent,
    Received,
    ClosedChannel,
}

/// Trait covering functions affecting the LN node
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
    ) -> Result<crate::models::PaymentResponse>;
    async fn send_spontaneous_payment(
        &self,
        node_id: String,
        amount_sats: u64,
    ) -> Result<crate::models::PaymentResponse>;
    async fn start(&self) -> Result<()>;
    async fn sweep(
        &self,
        to_address: String,
        fee_rate_sats_per_vbyte: u64,
    ) -> Result<WithdrawResponse>;
    async fn start_signer(&self, shutdown: mpsc::Receiver<()>);
    async fn list_peers(&self) -> Result<Vec<Peer>>;
    async fn connect_peer(&self, node_id: String, addr: String) -> Result<()>;
    fn sign_invoice(&self, invoice: RawInvoice) -> Result<String>;
    async fn close_peer_channels(&self, node_id: String) -> Result<Vec<String>>;
    async fn stream_incoming_payments(&self) -> Result<Streaming<gl_client::pb::IncomingPayment>>;
    async fn stream_log_messages(&self) -> Result<Streaming<gl_client::pb::LogEntry>>;
    async fn execute_command(&self, command: String) -> Result<String>;

    /// Gets the private key at the path specified
    fn derive_bip32_key(&self, path: Vec<ChildNumber>) -> Result<ExtendedPrivKey>;
}

/// Trait covering LSP-related functionality
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

/// Trait covering fiat-related functionality
#[tonic::async_trait]
pub trait FiatAPI: Send + Sync {
    /// List all supported fiat currencies for which there is a known exchange rate.
    async fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>>;

    /// Get the live rates from the server.
    async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>>;
}

/// Summary of an ongoing swap
pub struct Swap {
    pub bitcoin_address: String,
    pub swapper_pubkey: Vec<u8>,
    pub lock_height: i64,
    pub max_allowed_deposit: i64,
    pub error_message: String,
    pub required_reserve: i64,
    pub min_allowed_deposit: i64,
}

/// Trait covering functionality involving swaps
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

/// Details about the BTC/BTC reverse swap pair, at this point in time
///
/// Maps the result of https://docs.boltz.exchange/en/latest/api/#getting-pairs for the BTC/BTC pair
#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct ReverseSwapPairInfo {
    /// Minimum amount of sats a reverse swap is allowed to have on this endpoint
    pub min: u64,
    /// Maximum amount of sats a reverse swap is allowed to have on this endpoint
    pub max: u64,
    /// Hash of the pair info JSON
    pub fees_hash: String,
    /// Percentage fee for the reverse swap service
    pub fees_percentage: f64,
    /// Estimated miner fees in sats for locking up funds
    pub fees_lockup: u64,
    /// Estimated miner fees in sats for claiming funds
    pub fees_claim: u64,
}

/// Details of past or ongoing reverse swaps, as stored in the Breez local DB
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FullReverseSwapInfo {
    /// The reverse swap ID, as reported by the Boltz API in case of a successful creation
    pub id: String,

    /// The blockheight at the moment the reverse swap was created
    pub created_at_block_height: u32,

    /// Locally generated preimage, revealed in the last step of the reverse swap
    pub preimage: Vec<u8>,

    /// Locally generated private key, used to sign the claim tx
    pub private_key: Vec<u8>,

    /// On-chain destination address, to which the reverse swap will finally send funds to
    pub claim_pubkey: String,

    pub timeout_block_height: u32,

    /// The HODL invoice
    pub invoice: String,
    pub redeem_script: String,

    /// Amount of sats that will be locked.
    ///
    /// The final amount sent will be this value minus the claim tx fees.
    pub onchain_amount_sat: u64,

    /// User-specified feerate for the claim tx
    pub sat_per_vbyte: u64,

    pub cache: ReverseSwapInfoCached,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReverseSwapInfoCached {
    pub status: ReverseSwapStatus,
}

impl FullReverseSwapInfo {
    /// Builds the expected redeem script
    fn build_expected_reverse_swap_script(
        preimage_hash: Vec<u8>,
        compressed_pub_key: Vec<u8>,
        sig: Vec<u8>,
        lock_height: u32,
    ) -> Result<Script> {
        let mut ripemd160_hasher = Ripemd160::new();
        ripemd160_hasher.update(preimage_hash);
        let ripemd160_hash = ripemd160_hasher.finalize();

        // Remove empty non-significant bytes
        let timeout_height_le_hex = lock_height.to_le_bytes().to_hex();
        let timeout_height_le_hex_trimmed = timeout_height_le_hex.trim_end_matches("00");
        let timeout_height_le_bytes = hex::decode(timeout_height_le_hex_trimmed)?;

        Ok(Builder::new()
            .push_opcode(opcodes::all::OP_SIZE)
            .push_slice(&[0x20])
            .push_opcode(opcodes::all::OP_EQUAL)
            .push_opcode(opcodes::all::OP_IF)
            .push_opcode(opcodes::all::OP_HASH160)
            .push_slice(&ripemd160_hash[..])
            .push_opcode(opcodes::all::OP_EQUALVERIFY)
            .push_slice(&compressed_pub_key[..])
            .push_opcode(opcodes::all::OP_ELSE)
            .push_opcode(opcodes::all::OP_DROP)
            .push_slice(&timeout_height_le_bytes)
            .push_opcode(opcodes::all::OP_CLTV)
            .push_opcode(opcodes::all::OP_DROP)
            .push_slice(&sig[..])
            .push_opcode(opcodes::all::OP_ENDIF)
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script())
    }

    /// Validates the redeem script and the lockup address
    ///
    /// ### Arguments
    ///
    /// * `received_lockup_address` - The lockup address, as received from Boltz in the create rev swap API response
    /// * `network` - The network type which is one of (Bitcoin, Testnet, Signet, Regtest)
    pub(crate) fn validate_redeem_script(
        &self,
        received_lockup_address: String,
        network: Network,
    ) -> Result<()> {
        let redeem_script_received = Script::from_hex(&self.redeem_script)?;
        let asm = redeem_script_received.asm();
        debug!("received asm = {asm:?}");

        let sk = SecretKey::from_slice(&self.private_key)?;
        let pk = PublicKey::from_secret_key(&Secp256k1::new(), &sk);

        // The 18th asm element is the refund address, provided by the Boltz service
        let asm_elements: Vec<&str> = asm.split(' ').collect();
        let refund_address = asm_elements.get(18).unwrap_or(&"").to_string();
        let refund_address_bytes = hex::decode(refund_address)?;

        let redeem_script_expected = Self::build_expected_reverse_swap_script(
            self.get_preimage_hash().to_vec(), // Preimage hash
            pk.serialize().to_vec(),           // Compressed pubkey
            refund_address_bytes,
            self.timeout_block_height,
        )?;
        debug!("expected asm = {:?}", redeem_script_expected.asm());

        match redeem_script_received.eq(&redeem_script_expected) {
            true => {
                let lockup_addr_expected = &received_lockup_address;
                let lockup_addr_from_script =
                    &Address::p2wsh(&redeem_script_received, network.into()).to_string();

                match lockup_addr_from_script == lockup_addr_expected {
                    true => Ok(()),
                    false => Err(anyhow!("Unexpected lockup address")),
                }
            }
            false => Err(anyhow!("Unexpected redeem script")),
        }
    }

    /// Validates the received HODL invoice:
    ///
    /// - checks if amount matches the amount requested by the user
    /// - checks if the payment hash is the same preimage hash (derived from local secret bytes)
    /// included in the create request
    pub(crate) fn validate_hodl_invoice(&self, amount_req_msat: u64) -> Result<()> {
        let inv: lightning_invoice::Invoice = self.invoice.parse()?;

        // Validate if received invoice has the same amount as requested by the user
        let amount_from_invoice_msat = inv.amount_milli_satoshis().unwrap_or_default();
        match amount_from_invoice_msat == amount_req_msat {
            false => Err(anyhow!("Invoice amount doesn't match the request")),
            true => {
                // Validate if received invoice has the same payment hash as the preimage hash in the request
                let preimage_hash_from_invoice = inv.payment_hash();
                let preimage_hash_from_req = &self.get_preimage_hash();
                match preimage_hash_from_invoice == preimage_hash_from_req {
                    false => Err(anyhow!("Invoice payment hash doesn't match the request")),
                    true => Ok(()),
                }
            }
        }
    }

    /// Derives the lockup address from the redeem script
    pub(crate) fn get_lockup_address(&self, network: Network) -> Result<Address> {
        let redeem_script = Script::from_hex(&self.redeem_script)?;
        Ok(Address::p2wsh(&redeem_script, network.into()))
    }

    /// Get the preimage hash sent in the create request
    pub(crate) fn get_preimage_hash(&self) -> bitcoin::hashes::sha256::Hash {
        bitcoin::hashes::sha256::Hash::hash(&self.preimage)
    }
}

/// Simplified version of [FullReverseSwapInfo], containing only the user-relevant fields
#[derive(Serialize)]
pub struct ReverseSwapInfo {
    pub id: String,
    pub claim_pubkey: String,
    pub onchain_amount_sat: u64,
    pub status: ReverseSwapStatus,
}

impl From<FullReverseSwapInfo> for ReverseSwapInfo {
    fn from(rsi: FullReverseSwapInfo) -> Self {
        Self {
            id: rsi.id,
            claim_pubkey: rsi.claim_pubkey,
            onchain_amount_sat: rsi.onchain_amount_sat,
            status: rsi.cache.status,
        }
    }
}

/// The possible statuses of a reverse swap, from the Breez SDK perspective.
///
/// See [BoltzApiReverseSwapStatus] for the reverse swap status from the Breez endpoint point of view.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ReverseSwapStatus {
    /// HODL invoice payment is not completed yet
    Initial = 0,

    /// HODL invoice payment was successfully triggered and confirmed by Boltz, but the reverse swap
    /// is not yet complete
    InProgress = 1,

    /// An explicit error occurs (validation error, failure reported by Boltz, expiration, etc) and
    /// the initial invoice funds are returned to the sender (invoice is cancelled or payment failed)
    Cancelled = 2,

    /// Successfully completed (claim tx has been seen in the mempool)
    CompletedSeen = 3,

    /// Successfully completed (claim tx has at least one confirmation)
    CompletedConfirmed = 4,
}

impl ReverseSwapStatus {
    pub(crate) fn is_monitored_state(&self) -> bool {
        matches!(
            self,
            ReverseSwapStatus::Initial
                | ReverseSwapStatus::InProgress
                | ReverseSwapStatus::CompletedSeen
        )
    }

    pub(crate) fn is_blocking_state(&self) -> bool {
        matches!(
            self,
            ReverseSwapStatus::Initial | ReverseSwapStatus::InProgress
        )
    }
}

impl TryFrom<i32> for ReverseSwapStatus {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ReverseSwapStatus::Initial),
            1 => Ok(ReverseSwapStatus::InProgress),
            2 => Ok(ReverseSwapStatus::Cancelled),
            3 => Ok(ReverseSwapStatus::CompletedSeen),
            4 => Ok(ReverseSwapStatus::CompletedConfirmed),
            _ => Err(anyhow!("illegal value")),
        }
    }
}

/// Trait covering functionality involving swaps
#[tonic::async_trait]
pub(crate) trait ReverseSwapperAPI: Send + Sync {
    /// Lookup the most recent reverse swap pair info using the Boltz API
    async fn fetch_reverse_swap_fees(&self) -> Result<ReverseSwapPairInfo>;

    /// Creates a reverse submarine swap on the remote service (Boltz).
    ///
    /// # Arguments
    ///
    /// * `amount_sat` - Amount that is to be swapped
    /// * `preimage_hash_hex` - Hex of preimage hash
    /// * `claim_pubkey` - Pubkey of a keypair that can allow the SDK to claim the locked funds
    /// * `pair_hash` - The hash of the exchange rate, looked-up before this call
    /// * `routing_node` - Pubkey of a LN node used as routing hint
    async fn create_reverse_swap_on_remote(
        &self,
        amount_sat: u64,
        preimage_hash_hex: String,
        claim_pubkey: String,
        pair_hash: String,
        routing_node: String,
    ) -> Result<BoltzApiCreateReverseSwapResponse>;

    /// Performs a live lookup of the reverse swap's status on the Boltz API
    async fn get_boltz_status(&self, id: String) -> Result<BoltzApiReverseSwapStatus>;
}

/// Internal SDK log entry
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub line: String,
    pub level: String,
}

/// Configuration for the Breez Services
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
    pub maxfee_percent: f64,
    pub node_config: NodeConfig,
}

impl Config {
    pub fn production(api_key: String, node_config: NodeConfig) -> Self {
        Config {
            breezserver: "https://bs1.breez.technology:443".to_string(),
            mempoolspace_url: "https://mempool.space".to_string(),
            working_dir: ".".to_string(),
            network: Bitcoin,
            payment_timeout_sec: 60,
            default_lsp_id: Some(String::from("03cea51f-b654-4fb0-8e82-eca137f236a0")),
            api_key: Some(api_key),
            maxfee_percent: 0.5,
            node_config,
        }
    }

    pub fn staging(api_key: String, node_config: NodeConfig) -> Self {
        Config {
            breezserver: "https://bs1-st.breez.technology:443".to_string(),
            mempoolspace_url: "https://mempool.space".to_string(),
            working_dir: ".".to_string(),
            network: Bitcoin,
            payment_timeout_sec: 60,
            default_lsp_id: Some(String::from("ea51d025-042d-456c-8325-63e430797481")),
            api_key: Some(api_key),
            maxfee_percent: 0.5,
            node_config,
        }
    }
}

#[derive(Clone)]
pub enum NodeConfig {
    Greenlight { config: GreenlightNodeConfig },
}

#[derive(Clone)]
pub struct GreenlightNodeConfig {
    pub partner_credentials: Option<GreenlightCredentials>,
    pub invite_code: Option<String>,
}

/// Indicates the different kinds of supported environments for [crate::BreezServices].
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumString)]
pub enum EnvironmentType {
    #[strum(serialize = "production")]
    Production,
    #[strum(serialize = "staging")]
    Staging,
}

/// Client-specific credentials to connect to and manage a Greenlight node in the cloud
#[derive(Clone, Serialize, Deserialize)]
pub struct GreenlightCredentials {
    pub device_key: Vec<u8>,
    pub device_cert: Vec<u8>,
}

/// The different supported bitcoin networks
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct BackupStatus {
    pub backed_up: bool,
    pub last_backup_time: Option<u64>,
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

/// Internal response to a [NodeAPI::pull_changed] call
pub struct SyncResponse {
    pub node_state: NodeState,
    pub payments: Vec<crate::models::Payment>,
    pub channels: Vec<crate::models::Channel>,
}

/// Represents a payment, including its [PaymentType] and [PaymentDetails].
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub payment_type: PaymentType,
    pub payment_time: i64,
    pub amount_msat: u64,
    pub fee_msat: u64,
    pub pending: bool,
    pub description: Option<String>,
    pub details: PaymentDetails,
}

/// Represents a payment response.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub payment_time: i64,
    pub amount_msat: u64,
    pub fee_msat: u64,
    pub payment_hash: String,
    pub payment_preimage: String,
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
        data: ClosedChannelPaymentDetails,
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

    /// Only set for [PaymentType::Sent] payments that are part of a LNURL-pay workflow where
    /// the endpoint returns a success action
    pub lnurl_success_action: Option<SuccessActionProcessed>,

    /// Only set for [PaymentType::Sent] payments that are sent to a Lightning Address
    pub ln_address: Option<String>,

    /// Only set for [PaymentType::Sent] payments where the receiver endpoint returned LNURL metadata
    pub lnurl_metadata: Option<String>,
}

/// Represents the funds that were on the user side of the channel at the time it was closed.
#[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
pub struct ClosedChannelPaymentDetails {
    pub short_channel_id: String,
    pub state: ChannelState,
    pub funding_txid: String,
}

/// Lightning channel
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Channel {
    pub funding_txid: String,
    pub short_channel_id: String,
    pub state: ChannelState,
    pub spendable_msat: u64,
    pub receivable_msat: u64,
    pub closed_at: Option<u64>,
}

/// State of a Lightning channel
#[derive(Clone, PartialEq, Eq, Debug, EnumString, Display, Deserialize, Serialize)]
pub enum ChannelState {
    PendingOpen,
    Opened,
    PendingClose,
    Closed,
}

/// The status of a swap
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
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
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
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
    pub paid_sats: u64,
    pub confirmed_sats: u64,
    pub unconfirmed_sats: u64,
    pub status: SwapStatus,
    pub refund_tx_ids: Vec<String>,
    pub unconfirmed_tx_ids: Vec<String>,
    pub confirmed_tx_ids: Vec<String>,
    pub min_allowed_deposit: i64,
    pub max_allowed_deposit: i64,
    pub last_redeem_error: Option<String>,
}

impl SwapInfo {
    pub(crate) fn unused(&self) -> bool {
        self.confirmed_sats == 0
            && self.unconfirmed_sats == 0
            && self.paid_sats == 0
            && self.status != SwapStatus::Expired
    }

    pub(crate) fn in_progress(&self) -> bool {
        (self.confirmed_sats > 0 || self.unconfirmed_sats > 0)
            && self.paid_sats == 0
            && self.status != SwapStatus::Expired
    }

    pub(crate) fn redeemable(&self) -> bool {
        self.unconfirmed_sats == 0
            && self.confirmed_sats > 0
            && self.paid_sats == 0
            && self.status != SwapStatus::Expired
    }

    pub(crate) fn refundable(&self) -> bool {
        self.confirmed_sats > self.paid_sats && self.status == SwapStatus::Expired
    }

    pub(crate) fn monitored(&self) -> bool {
        self.unused() || self.in_progress() || self.refundable()
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
    #[serde(default)]
    pub reserved: bool,
    #[serde(default)]
    pub reserved_to_block: u32,
}

//// Contains the result of the entire LNURL interaction, as reported by the LNURL endpoint.
///
/// * `Ok` indicates the interaction with the endpoint was valid, and the endpoint
///  - started to pay the invoice asynchronously in the case of LNURL-withdraw,
///  - verified the client signature in the case of LNURL-auth,////// * `Error` indicates a generic issue the LNURL endpoint encountered, including a freetext
/// description of the reason.
///
/// Both cases are described in LUD-03 <https://github.com/lnurl/luds/blob/luds/03.md> & LUD-04: <https://github.com/lnurl/luds/blob/luds/04.md>
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(tag = "status")]
pub enum LnUrlCallbackStatus {
    /// On-wire format is: `{"status": "OK"}`
    Ok,
    /// On-wire format is: `{"status": "ERROR", "reason": "error details..."}`
    #[serde(rename = "ERROR")]
    ErrorStatus {
        #[serde(flatten)]
        data: LnUrlErrorData,
    },
}

/// Different providers will demand different behaviours when the user is trying to buy bitcoin.
#[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "buy_bitcoin_provider")]
pub enum BuyBitcoinProvider {
    Moonpay,
}

impl FromStr for BuyBitcoinProvider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "moonpay" => Ok(BuyBitcoinProvider::Moonpay),
            _ => Err(anyhow!("unknown buy bitcoin provider")),
        }
    }
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
