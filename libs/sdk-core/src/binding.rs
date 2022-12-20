use crate::breez_services::{self, BreezEvent, BreezServicesBuilder, EventListener};
use crate::fiat::{FiatCurrency, Rate};
use crate::input_parser::LnUrlPayRequestData;
use crate::lsp::LspInformation;
use crate::models::LogEntry;
use anyhow::{anyhow, Result};
use flutter_rust_bridge::StreamSink;
use log::{Metadata, Record};
use once_cell::sync::{Lazy, OnceCell};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::breez_services::BreezServices;
use crate::invoice::LNInvoice;
use crate::models::{
    Config, FeeratePreset, GreenlightCredentials, Network, NodeState, Payment, PaymentTypeFilter,
    SwapInfo,
};

use crate::input_parser::InputType;
use crate::invoice::{self};
use crate::lnurl::pay::model::LnUrlPayResult;
use bip39::{Language, Mnemonic, Seed};

static BREEZ_SERVICES_INSTANCE: OnceCell<Arc<BreezServices>> = OnceCell::new();
static BREEZ_SERVICES_SHUTDOWN: OnceCell<mpsc::Sender<()>> = OnceCell::new();
static NOTIFICATION_STREAM: OnceCell<StreamSink<BreezEvent>> = OnceCell::new();
static LOG_STREAM: OnceCell<StreamSink<LogEntry>> = OnceCell::new();
static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

struct BindingLogger;

impl BindingLogger {
    fn init() {
        log::set_logger(&BindingLogger {}).unwrap();
    }
}

impl log::Log for BindingLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(s) = LOG_STREAM.get() {
                s.add(LogEntry {
                    line: record.args().to_string(),
                    level: record.level().as_str().to_string(),
                });
            }
        };
    }
    fn flush(&self) {}
}

struct BindingEventListener;

impl EventListener for BindingEventListener {
    fn on_event(&self, e: BreezEvent) {
        let s = NOTIFICATION_STREAM.get();
        if s.is_some() {
            s.unwrap().add(e);
        }
    }
}

/// Register a new node in the cloud and return credentials to interact with it
///
/// # Arguments
///
/// * `network` - The network type which is one of (Bitcoin, Testnet, Signet, Regtest)
/// * `seed` - The node private key
/// * `config` - The sdk configuration
pub fn register_node(
    network: Network,
    seed: Vec<u8>,
    config: Option<Config>,
) -> Result<GreenlightCredentials> {
    let creds = block_on(BreezServices::register_node(network, seed.clone()))?;
    init_node(config, seed, creds.clone())?;
    Ok(creds)
}

/// Recover an existing node from the cloud and return credentials to interact with it
///
/// # Arguments
///
/// * `network` - The network type which is one of (Bitcoin, Testnet, Signet, Regtest)
/// * `seed` - The node private key
/// * `config` - The sdk configuration
pub fn recover_node(
    network: Network,
    seed: Vec<u8>,
    config: Option<Config>,
) -> Result<GreenlightCredentials> {
    let creds = block_on(BreezServices::recover_node(network, seed.clone()))?;
    init_node(config, seed, creds.clone())?;

    Ok(creds)
}

/// init_node initialized the global NodeService, schedule the node to run in the cloud and
/// run the signer. This must be called in order to start comunicate with the node
///
/// # Arguments
///
/// * `config` - The sdk configuration
/// * `seed` - The node private key
/// * `creds` - The greenlight credentials
///
pub fn init_node(
    config: Option<Config>,
    seed: Vec<u8>,
    creds: GreenlightCredentials,
) -> Result<()> {
    block_on(async move {
        let breez_services =
            BreezServices::start(rt(), config, seed, creds, Box::new(BindingEventListener {}))
                .await?;
        BREEZ_SERVICES_INSTANCE
            .set(breez_services.clone())
            .map_err(|_| anyhow!("static node services already set"))?;

        Ok(())
    })
}

pub fn breez_events_stream(s: StreamSink<BreezEvent>) -> Result<()> {
    NOTIFICATION_STREAM
        .set(s)
        .map_err(|_| anyhow!("events stream already created"))?;
    Ok(())
}

pub fn breez_log_stream(s: StreamSink<LogEntry>) -> Result<()> {
    BindingLogger::init();
    LOG_STREAM
        .set(s)
        .map_err(|_| anyhow!("log stream already created"))?;
    Ok(())
}

/// Cleanup node resources and stop the signer.
pub fn stop_node() -> Result<()> {
    block_on(async {
        let shutdown_handler = BREEZ_SERVICES_SHUTDOWN.get();
        match shutdown_handler {
            None => Err(anyhow!("Background processing is not running")),
            Some(s) => s.send(()).await.map_err(anyhow::Error::msg),
        }
    })
}

/// pay a bolt11 invoice
///
/// # Arguments
///
/// * `bolt11` - The bolt11 invoice
pub fn send_payment(bolt11: String) -> Result<()> {
    block_on(async { get_breez_services()?.send_payment(bolt11).await })
}

/// pay directly to a node id using keysend
///
/// # Arguments
///
/// * `node_id` - The destination node_id
/// * `amount_sats` - The amount to pay in satoshis
pub fn send_spontaneous_payment(node_id: String, amount_sats: u64) -> Result<()> {
    block_on(async {
        get_breez_services()?
            .send_spontaneous_payment(node_id, amount_sats)
            .await
    })
}

/// Creates an bolt11 payment request.
/// This also works when the node doesn't have any channels and need inbound liquidity.
/// In such case when the invoice is paid a new zero-conf channel will be open by the LSP,
/// providing inbound liquidity and the payment will be routed via this new channel.
///
/// # Arguments
///
/// * `description` - The bolt11 payment request description
/// * `amount_sats` - The amount to receive in satoshis
pub fn receive_payment(amount_sats: u64, description: String) -> Result<LNInvoice> {
    block_on(async {
        get_breez_services()?
            .receive_payment(amount_sats, description.to_string())
            .await
    })
}

/// get the node state from the persistent storage
pub fn node_info() -> Result<Option<NodeState>> {
    block_on(async { get_breez_services()?.node_info() })
}

/// list transactions (incoming/outgoing payments) from the persistent storage
pub fn list_payments(
    filter: PaymentTypeFilter,
    from_timestamp: Option<i64>,
    to_timestamp: Option<i64>,
) -> Result<Vec<Payment>> {
    block_on(async {
        get_breez_services()?
            .list_payments(filter, from_timestamp, to_timestamp)
            .await
    })
}

/// List available lsps that can be selected by the user
pub fn list_lsps() -> Result<Vec<LspInformation>> {
    block_on(async { get_breez_services()?.list_lsps().await })
}

/// Select the lsp to be used and provide inbound liquidity
pub fn connect_lsp(lsp_id: String) -> Result<()> {
    block_on(async { get_breez_services()?.connect_lsp(lsp_id).await })
}

/// Convenience method to look up LSP info based on current LSP ID
pub fn lsp_info() -> Result<LspInformation> {
    block_on(async { get_breez_services()?.lsp_info().await })
}

/// Fetch live rates of fiat currencies
pub fn fetch_fiat_rates() -> Result<Vec<Rate>> {
    block_on(async { get_breez_services()?.fetch_fiat_rates().await })
}

/// List all available fiat currencies
pub fn list_fiat_currencies() -> Result<Vec<FiatCurrency>> {
    block_on(async { get_breez_services()?.list_fiat_currencies() })
}

/// close all channels with the current lsp
pub fn close_lsp_channels() -> Result<()> {
    block_on(async { get_breez_services()?.close_lsp_channels().await })
}

/// Withdraw on-chain funds in the wallet to an external btc address
pub fn sweep(to_address: String, feerate_preset: FeeratePreset) -> Result<()> {
    block_on(async {
        get_breez_services()?
            .sweep(to_address, feerate_preset)
            .await
    })
}

/// swaps

/// Onchain receive swap API
pub fn receive_onchain() -> Result<SwapInfo> {
    block_on(async { get_breez_services()?.receive_onchain().await })
}

// list swaps history (all of them: expired, refunded and active)
pub fn list_refundables() -> Result<Vec<SwapInfo>> {
    block_on(async { get_breez_services()?.list_refundables().await })
}

// construct and broadcast a refund transaction for a faile/expired swap
pub fn refund(swap_address: String, to_address: String, sat_per_vbyte: u32) -> Result<String> {
    block_on(async {
        get_breez_services()?
            .refund(swap_address, to_address, sat_per_vbyte)
            .await
    })
}

fn get_breez_services() -> Result<&'static BreezServices> {
    let n = BREEZ_SERVICES_INSTANCE.get();
    match n {
        Some(a) => Ok(a),
        None => Err(anyhow!("Node service was not initialized")),
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    rt().block_on(future)
}

pub(crate) fn rt() -> &'static tokio::runtime::Runtime {
    &RT
}

// These functions are exposed temporarily for integration purposes

pub fn parse_invoice(invoice: String) -> Result<LNInvoice> {
    invoice::parse_invoice(&invoice)
}

pub fn parse(s: String) -> Result<InputType> {
    block_on(async { crate::input_parser::parse(&s).await })
}

/// Second step of LNURL-pay. The first step is `parse()`, which also validates the LNURL destination
/// and generates the `LnUrlPayRequestData` payload needed here.
pub fn pay_lnurl(
    user_amount_sat: u64,
    comment: Option<String>,
    req_data: LnUrlPayRequestData,
) -> Result<LnUrlPayResult> {
    block_on(async {
        get_breez_services()?
            .pay_lnurl(user_amount_sat, comment, req_data)
            .await
    })
}

/// Attempts to convert the phrase to a mnemonic, then to a seed.
///
/// If the phrase is not a valid mnemonic, an error is returned.
pub fn mnemonic_to_seed(phrase: String) -> Result<Vec<u8>> {
    breez_services::mnemonic_to_seed(phrase)
}
