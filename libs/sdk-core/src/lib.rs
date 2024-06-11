//! # Breez SDK
//!
//! The Breez SDK enables mobile developers to integrate Lightning and bitcoin payments into their
//! apps with a very shallow learning curve. The use cases are endless – from social apps that want
//! to integrate tipping between users to content-creation apps interested in adding bitcoin monetization.
//! Crucially, this SDK is an end-to-end, non-custodial, drop-in solution powered by Greenlight,
//! a built-in LSP, on-chain interoperability, third-party fiat on-ramps, and other services users
//! and operators need.
//!
//! The Breez SDK provides the following services:
//! * Sending payments (via various protocols such as: bolt11, keysend, lnurl-pay, lightning address, etc.)
//! * Receiving payments (via various protocols such as: bolt11, lnurl-withdraw, etc.)
//! * Fetching node status (e.g. balance, max allow to pay, max allow to receive, on-chain balance, etc.)
//! * Connecting to a new or existing node.
//!
//! ## Getting Started
//!
//! First, make sure you have your API Key and Invite Code ready (see [API Key and Invite Code](#api-key-and-invite-code) section below).
//!
//! The following code initialize the SDK and make it ready to be used:
//!
//! ```ignore
//! let mnemonic = Mnemonic::generate_in(Language::English, 12)?;
//! let seed = mnemonic.to_seed("");
//! let invite_code = Some("...".into());
//!
//! let mut config = BreezServices::default_config(
//!     EnvironmentType::Production,
//!     "your API key".into(),
//!     breez_sdk_core::NodeConfig::Greenlight {
//!         config: GreenlightNodeConfig { partner_credentials: None, invite_code },
//!     },
//! );
//!
//! // Customize the config object according to your needs
//! config.working_dir = "path to an existing directory".into();
//!
//! // Connect to the Breez SDK make it ready for use
//! let sdk = BreezServices::connect(
//!    config,
//!    seed.to_vec(),        
//!    Box::new(AppEventListener {}),
//! )
//! .await?;
//!
//! ```
//!
//! We can now receive payments
//!
//! ```ignore
//! let invoice = sdk.receive_payment(3000, "Invoice for 3000 sats".into()).await?;
//! ```
//!
//! or make payments
//! ```ignore
//! let bolt11 = "...";
//! sdk.send_payment(bolt11.into(), Some(3000)).await?;
//! ```
//!
//! At any point we can fetch our balance from the Greenlight node
//! ```ignore
//! if let Some(node_state) = sdk.node_info()? {
//!     let balance_ln = node_state.channels_balance_msat;
//!     let balance_onchain = node_state.onchain_balance_msat;
//! }
//! ```
//!
//! or fetch other useful infos, like the current mempool [RecommendedFees]
//! ```ignore
//! let fees: RecommendedFees = sdk.recommended_fees().await?;
//! ```
//!
//! These different types of operations are described below in more detail.
//!
//! ### A. Initializing the SDK
//!
//! There are two simple steps necessary to initialize the SDK:
//!
//! 1. [BreezServices::default_config] to construct the sdk configuration
//! 2. [BreezServices::connect] to connect to your node and start all required Breez SDK services
//!
//! The first step takes the [EnvironmentType] and [NodeConfig] as arguments. Although you can create
//! your own config from scratch it is recommended to use the [BreezServices::default_config] method and
//! customize it according to your needs.
//! Once the [NodeConfig] is created it is passed to the [BreezServices::connect] method along with the seed and and implementation of [EventListener] which is used to
//! notify the caller of SDK events.
//!
//! Now your SDK is ready to be used.
//!
//! ### B. Sending and receiving Lightning payments
//!
//! Supported BOLT11 operations are
//!
//! * [BreezServices::receive_payment] to create an invoice
//! * [BreezServices::send_payment] to pay an invoice
//! * [BreezServices::send_spontaneous_payment] for keysend payments
//!
//! ### C. Receiving an on-chain transaction (swap-in)
//!
//! * [BreezServices::receive_onchain] accepting an optional user-selected [OpeningFeeParams] for
//! the case when the operation requires a new channel with the LSP
//! * [BreezServices::in_progress_swap]
//! * [BreezServices::list_refundables] to get a list of swaps
//! * [BreezServices::refund] to broadcast a transaction for failed or expired swaps
//!
//! ### D. Sending to an on-chain address (swap-out)
//!
//! * [BreezServices::fetch_reverse_swap_fees] to get the current swap-out fees
//! * [BreezServices::send_onchain] to start the swap-out
//! * [BreezServices::in_progress_reverse_swaps] to see any in-progress swaps
//!
//! ### E. Using LNURL
//!
//! 1. [parse] the LNURL endpoint URL to get the workflow parameters.
//! 2. After getting the user input or confirmation, complete the workflow with [BreezServices::lnurl_pay] or
//! [BreezServices::lnurl_withdraw].
//!
//! ### F. Supporting fiat currencies
//!
//! * [BreezServices::list_fiat_currencies] to get the supported fiat currencies
//! * [BreezServices::fetch_fiat_rates] to get the current exchange rates
//! * [BreezServices::recommended_fees] for the recommended mempool fees
//!
//! ### G. Connecting to an LSP
//!
//! * [BreezServices::list_lsps] to get a list of available LSPs
//! * [BreezServices::connect_lsp] to connect to a chosen LSP
//! * [BreezServices::lsp_info] to get [LspInformation] on the currently selected LSP
//!
//! ### H. Utilities
//!
//! Use [parse] to parse generic input. The input can come from the user, from a clicked link or from a QR code.
//! The resulting [InputType] will tell you what the input is and how to treat it, as well as present relevant payload data
//! in a structured form.
//!
//!
//! ## Bindings
//!
//! * C#
//! * Dart
//! * Go
//! * Kotlin
//! * Python
//! * React-Native
//! * Swift
//!
//!
//! ## API Key and Invite Code
//!
//! You will need an API Key to use the SDK, as well as an Invite Code when you create a new node.
//!
//! To get both of them, please contact Breez via email at <contact@breez.technology> or at <https://breez.technology/#contact-us-form>
//!
//! ## Support
//!
//! Join this [telegram group](https://t.me/breezsdk).

mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
#[macro_use]
extern crate log;

// TODO If enabled, breaks frb. If disabled, breaks tests.
// #[rustfmt::skip]
// #[cfg(test)]
// mod test_utils; // flutter_rust_bridge_codegen: has to be defined before breez_services

mod backup;
pub mod binding;
mod breez_services;
mod chain;
mod crypt;
pub mod error;
#[rustfmt::skip]
mod node_api; // flutter_rust_bridge_codegen: has to be defined before greenlight; greenlight::node_api
mod greenlight;
// GRPC structs are documented as follows:
// - if they are mirrored in Rust model structs, documented in the model structs
// - if there is no corresponding model struct, documented in breez.proto
mod grpc;
#[rustfmt::skip]
mod fiat; // flutter_rust_bridge_codegen: has to be defined after grpc; grpc::Rate
mod lsp;
mod lsps0;
mod lsps2;
mod models;
mod moonpay;
mod persist;
mod support;
mod swap_in;
mod swap_out;
mod tonic_wrap;

// Re-use crates from gl_client for consistency
use gl_client::bitcoin;
use gl_client::lightning;
use gl_client::lightning_invoice;

pub use breez_services::{
    mnemonic_to_seed, BackupFailedData, BreezEvent, BreezServices, CheckMessageRequest,
    CheckMessageResponse, EventListener, InvoicePaidDetails, LogStream, PaymentFailedData,
    SignMessageRequest, SignMessageResponse,
};
pub use chain::RecommendedFees;
pub use fiat::{CurrencyInfo, FiatCurrency, LocaleOverrides, LocalizedName, Rate, Symbol};
pub use lsp::LspInformation;
pub use models::*;
pub use sdk_common::prelude::*;
pub use swap_out::reverseswap::{ESTIMATED_CLAIM_TX_VSIZE, ESTIMATED_LOCKUP_TX_VSIZE};

// === FRB mirroring
//
// This section contains frb "mirroring" structs and enums.
// These are needed by the flutter bridge in order to use structs defined in an external crate.
// See <https://cjycode.com/flutter_rust_bridge/v1/feature/lang_external.html#types-in-other-crates>
// Note: in addition to the docs above, the mirrored structs must derive the Clone trait

use flutter_rust_bridge::frb;

#[frb(mirror(LnUrlAuthRequestData))]
pub struct _LnUrlAuthRequestData {
    pub k1: String,
    pub action: Option<String>,
    pub domain: String,
    pub url: String,
}

#[frb(mirror(LnUrlErrorData))]
pub struct _LnUrlErrorData {
    pub reason: String,
}

#[frb(mirror(LnUrlCallbackStatus))]
pub enum _LnUrlCallbackStatus {
    Ok,
    ErrorStatus { data: LnUrlErrorData },
}

#[frb(mirror(Network))]
pub enum _Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

#[frb(mirror(LNInvoice))]
pub struct _LNInvoice {
    pub bolt11: String,
    pub network: Network,
    pub payee_pubkey: String,
    pub payment_hash: String,
    pub description: Option<String>,
    pub description_hash: Option<String>,
    pub amount_msat: Option<u64>,
    pub timestamp: u64,
    pub expiry: u64,
    pub routing_hints: Vec<RouteHint>,
    pub payment_secret: Vec<u8>,
    pub min_final_cltv_expiry_delta: u64,
}

#[frb(mirror(RouteHint))]
pub struct _RouteHint {
    pub hops: Vec<RouteHintHop>,
}

#[frb(mirror(RouteHintHop))]
pub struct _RouteHintHop {
    pub src_node_id: String,
    pub short_channel_id: u64,
    pub fees_base_msat: u32,
    pub fees_proportional_millionths: u32,
    pub cltv_expiry_delta: u64,
    pub htlc_minimum_msat: Option<u64>,
    pub htlc_maximum_msat: Option<u64>,
}

#[frb(mirror(LnUrlPayRequestData))]
pub struct _LnUrlPayRequestData {
    pub callback: String,
    pub min_sendable: u64,
    pub max_sendable: u64,
    pub metadata_str: String,
    pub comment_allowed: u16,
    pub domain: String,
    pub allows_nostr: bool,
    pub nostr_pubkey: Option<String>,
    pub ln_address: Option<String>,
}

#[frb(mirror(LnUrlWithdrawRequestData))]
pub struct _LnUrlWithdrawRequestData {
    pub callback: String,
    pub k1: String,
    pub default_description: String,
    pub min_withdrawable: u64,
    pub max_withdrawable: u64,
}

#[frb(mirror(InputType))]
pub enum _InputType {
    BitcoinAddress { address: BitcoinAddressData },
    Bolt11 { invoice: LNInvoice },
    NodeId { node_id: String },
    Url { url: String },
    LnUrlPay { data: LnUrlPayRequestData },
    LnUrlWithdraw { data: LnUrlWithdrawRequestData },
    LnUrlAuth { data: LnUrlAuthRequestData },
    LnUrlEndpointError { data: LnUrlErrorData },
}

#[frb(mirror(BitcoinAddressData))]
pub struct _BitcoinAddressData {
    pub address: String,
    pub network: crate::prelude::Network,
    pub amount_sat: Option<u64>,
    pub label: Option<String>,
    pub message: Option<String>,
}

#[frb(mirror(SuccessActionProcessed))]
pub enum _SuccessActionProcessed {
    Aes { result: AesSuccessActionDataResult },
    Message { data: MessageSuccessActionData },
    Url { data: UrlSuccessActionData },
}

#[frb(mirror(AesSuccessActionDataResult))]
pub enum _AesSuccessActionDataResult {
    Decrypted { data: AesSuccessActionDataDecrypted },
    ErrorStatus { reason: String },
}

#[frb(mirror(AesSuccessActionDataDecrypted))]
pub struct _AesSuccessActionDataDecrypted {
    pub description: String,
    pub plaintext: String,
}

#[frb(mirror(MessageSuccessActionData))]
pub struct _MessageSuccessActionData {
    pub message: String,
}

#[frb(mirror(UrlSuccessActionData))]
pub struct _UrlSuccessActionData {
    pub description: String,
    pub url: String,
}

#[frb(mirror(LnUrlPayResult))]
pub enum _LnUrlPayResult {
    EndpointSuccess { data: LnUrlPaySuccessData },
    EndpointError { data: LnUrlErrorData },
    PayError { data: LnUrlPayErrorData },
}

#[frb(mirror(LnUrlPaySuccessData))]
pub struct _LnUrlPaySuccessData {
    // pub payment: Payment, // TODO How to handle Payment?
    pub success_action: Option<SuccessActionProcessed>,
}

#[frb(mirror(LnUrlPayErrorData))]
pub struct _LnUrlPayErrorData {
    pub payment_hash: String,
    pub reason: String,
}

#[frb(mirror(LnUrlWithdrawResult))]
pub enum _LnUrlWithdrawResult {
    Ok { data: LnUrlWithdrawSuccessData },
    ErrorStatus { data: LnUrlErrorData },
}

#[frb(mirror(LnUrlWithdrawSuccessData))]
pub struct _LnUrlWithdrawSuccessData {
    pub invoice: LNInvoice,
}
