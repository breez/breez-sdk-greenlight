//! # Breez SDK
//!
//! The Breez SDK makes it easy to build non-custodial Bitcoin and Lightning applications.
//!
//! The Greenlight integration allows it to separate the running of a LN node from the signing of LN
//! transactions. The node runs in the cloud and is ready to receive payments non-custodially,
//! while the caller of the SDK has full custodial access to these funds and can sign spending transactions
//! locally.
//!
//! On initializing the SDK, the caller gets its [GreenlightCredentials]. These are used to interact
//! with the Greenlight LN node running in the cloud. Together with the BIP39 mnemonic, these can be used to
//! restore access to the same cloud node, either in the same app (backup / restore) or in another app
//! using the SDK.
//!
//! In addition, the SDK includes utilities for LSP management, on-chain payments and fiat on- and
//! off-ramps.
//!
//! ## Getting Started
//!
//! The first step is to initialize the SDK and start the node:
//!
//! ```ignore
//! let mnemonic = Mnemonic::new(Words12, English);
//! let seed = Seed::new(&mnemonic, "");
//!
//! let creds = BreezServices::register_node(Network::Bitcoin, seed.as_bytes().to_vec()).await?;
//! let sdk = BreezServices::init_services(
//!         BreezServices::default_config(EnvironmentType::Production),
//!         seed.to_vec(),
//!         creds.clone(),
//!         Box::new(AppEventListener {}),
//!     )
//!     .await?;
//!
//! BreezServices::start(rt(), &sdk).await?;
//! ```
//! We can now receive payments
//!
//! ```ignore
//! let invoice = sdk.receive_payment(3000, "Invoice for 3000 sats".into()).await?;
//! ```
//!
//! or make payments
//! ```ignore
//! let bolt11 = "..."; // LN invoice
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
//! ### A. Initialize the SDK
//!
//! There are two steps necessary to initialize the SDK:
//!
//! 1. [BreezServices::init_services] to setup the Breez SDK services
//! 2. [BreezServices::start] to start the Greenlight node and all needed Breez SDK services
//!
//! The first step takes the [GreenlightCredentials] as an argument. There are three ways to get them:
//!
//! * by loading the credentials from local storage, or with
//! * [BreezServices::register_node] to register a new Greenlight node, or with
//! * [BreezServices::recover_node] to recover an existing Greenlight node
//!
//! The first step also takes an implementation of [EventListener] as an argument, which is used to
//! notify the caller of SDK events.
//!
//! After initializing the Breez SDK services and starting them, the SDK is ready to be used.
//!
//! ### B. Sending and receiving Lightning Payments
//!
//! Supported BOLT11 operations are
//!
//! * [BreezServices::receive_payment] to create an invoice
//! * [BreezServices::send_payment] to pay an invoice
//! * [BreezServices::send_spontaneous_payment] for keysend payments
//!
//! ### C. On-chain Receive (swap-in)
//!
//! * [BreezServices::receive_onchain]
//! * [BreezServices::list_refundables] to get a list of swaps
//! * [BreezServices::refund] to broadcast a transaction for failed or expired swaps
//!
//! ### D. Using LNURL
//!
//! 1. [parse] the LNURL endpoint URL to get the workflow parameters.
//! 2. After getting the user input or confirmation, complete the workflow with [BreezServices::lnurl_pay] or
//! [BreezServices::lnurl_withdraw].
//!
//! ### E. Utilities
//!
//! Use [parse] to parse generic input. The input can come from the user, from a clicked link or from a QR code.
//! The resulting [InputType] will tell you what the input is and how to treat it, as well as present relevant payload data
//! in a structured form.
//!
//! ### F. Supporting fiat currencies
//!
//! * [BreezServices::list_fiat_currencies] to get the supported fiat currencies
//! * [BreezServices::fetch_fiat_rates] to get the current exchange rates
//! * [BreezServices::recommended_fees] for the recommended mempool fees
//!
//! ### G. LSP Management
//!
//! * [BreezServices::list_lsps] to get a list of available LSPs
//! * [BreezServices::connect_lsp] to connect to a chosen LSP
//! * [BreezServices::lsp_info] to get [LspInformation] on the currently selected LSP
//!
//! ### H. Stop and Cleanup
//!
//! On app shutdown, the Breez SDK can be closed with
//! ```ignore
//! sdk.stop().await?;
//! ```
//!
//! ## Bindings and Supported Platforms
//!
//! The library can be built both for Android and iOS.

mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
#[macro_use]
extern crate log;

pub mod binding;
mod breez_services;
mod chain;
mod crypt;
mod fiat;
mod greenlight;
mod grpc;
mod input_parser;
mod invoice;
mod lnurl;
mod lsp;
mod models;
mod persist;
mod swap;
#[cfg(test)]
mod test_utils;

pub use breez_services::{
    mnemonic_to_seed, BreezEvent, BreezServices, EventListener, InvoicePaidDetails,
};
pub use chain::RecommendedFees;
pub use fiat::{CurrencyInfo, FiatCurrency, LocaleOverrides, LocalizedName, Rate, Symbol};
pub use input_parser::{
    parse, BitcoinAddressData, InputType, LnUrlAuthRequestData, LnUrlErrorData,
    LnUrlPayRequestData, LnUrlRequestData, LnUrlWithdrawRequestData, MetadataItem,
};
pub use invoice::{parse_invoice, LNInvoice, RouteHint, RouteHintHop};

pub use lnurl::pay::model::LnUrlPayResult;
pub use lnurl::withdraw::model::LnUrlWithdrawCallbackStatus;
pub use lsp::LspInformation;
pub use models::*;
