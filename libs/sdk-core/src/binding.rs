//! Bindings for the Dart integration

use std::future::Future;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use flutter_rust_bridge::StreamSink;
use log::{Level, LevelFilter, Metadata, Record};
use once_cell::sync::{Lazy, OnceCell};
use tokio::sync::mpsc;

use crate::breez_services::{self, BreezEvent, BreezServices, EventListener};
use crate::chain::RecommendedFees;
use crate::fiat::{FiatCurrency, Rate};
use crate::input_parser::{
    self, InputType, LnUrlAuthRequestData, LnUrlPayRequestData, LnUrlWithdrawRequestData,
};
use crate::invoice::{self, LNInvoice};
use crate::lnurl::pay::model::LnUrlPayResult;
use crate::lsp::LspInformation;
use crate::models::{Config, LogEntry, NodeState, Payment, PaymentTypeFilter, SwapInfo};
use crate::{
    BackupStatus, BuyBitcoinProvider, EnvironmentType, LnUrlCallbackStatus, NodeConfig,
    ReverseSwapInfo, ReverseSwapPairInfo,
};

static BREEZ_SERVICES_INSTANCE: OnceCell<Arc<BreezServices>> = OnceCell::new();
static BREEZ_SERVICES_SHUTDOWN: OnceCell<mpsc::Sender<()>> = OnceCell::new();
static NOTIFICATION_STREAM: OnceCell<StreamSink<BreezEvent>> = OnceCell::new();
static LOG_STREAM: OnceCell<StreamSink<LogEntry>> = OnceCell::new();
static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

/*  Breez Services API's */

/// See [BreezServices::connect]
pub fn connect(config: Config, seed: Vec<u8>) -> Result<()> {
    block_on(async move {
        let breez_services =
            BreezServices::connect(config, seed, Box::new(BindingEventListener {})).await?;
        BREEZ_SERVICES_INSTANCE
            .set(breez_services)
            .map_err(|_| anyhow!("static node services already set"))?;

        Ok(())
    })
}

/// Check whether node service is initialized or not
pub fn check_initialized() -> bool {
    block_on(async { get_breez_services().is_ok() })
}

/// See [BreezServices::sync]
pub fn sync_node() -> Result<()> {
    block_on(async { get_breez_services()?.sync().await })
}

/// See [BreezServices::node_info]
pub fn get_node_state() -> Result<Option<NodeState>> {
    block_on(async { get_breez_services()?.node_info() })
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

/*  Breez Services Helper API's */

/// See [breez_services::mnemonic_to_seed]
pub fn convert_mnemonic_to_seed(phrase: String) -> Result<Vec<u8>> {
    breez_services::mnemonic_to_seed(phrase)
}

/// See [BreezServices::default_config]
pub fn get_default_config(
    env_type: EnvironmentType,
    api_key: String,
    node_config: NodeConfig,
) -> Config {
    BreezServices::default_config(env_type, api_key, node_config)
}

/*  Stream API's */

pub fn breez_events_stream(s: StreamSink<BreezEvent>) -> Result<()> {
    NOTIFICATION_STREAM
        .set(s)
        .map_err(|_| anyhow!("events stream already created"))?;
    Ok(())
}

pub fn breez_log_stream(s: StreamSink<LogEntry>) -> Result<()> {
    LOG_STREAM
        .set(s)
        .map_err(|_| anyhow!("log stream already created"))?;
    BindingLogger::init();
    Ok(())
}

/*  LSP API's */

/// See [BreezServices::list_lsps]
pub fn list_lsps() -> Result<Vec<LspInformation>> {
    block_on(async { get_breez_services()?.list_lsps().await })
}

/// See [BreezServices::connect_lsp]
pub fn connect_lsp(lsp_id: String) -> Result<()> {
    block_on(async { get_breez_services()?.connect_lsp(lsp_id).await })
}

/// See [BreezServices::get_lsp_id]
pub fn get_lsp_id() -> Result<Option<String>> {
    block_on(async { get_breez_services()?.get_lsp_id().await })
}

/// See [BreezServices::fetch_lsp_info]
pub fn fetch_lsp_info(id: String) -> Result<Option<LspInformation>> {
    block_on(async { get_breez_services()?.fetch_lsp_info(id).await })
}

/// See [BreezServices::close_lsp_channels]
pub fn close_lsp_channels() -> Result<()> {
    block_on(async {
        _ = get_breez_services()?.close_lsp_channels().await;
        Ok(())
    })
}

/*  Backup API's */

/// See [BreezServices::backup]
pub fn backup() -> Result<()> {
    block_on(async { get_breez_services()?.backup().await })
}

/// See [BreezServices::backup_status]
pub fn get_backup_status() -> Result<BackupStatus> {
    get_breez_services()?.backup_status()
}

/*  Parse API's */

/// See [invoice::parse_invoice]
pub fn parse_invoice(invoice: String) -> Result<LNInvoice> {
    invoice::parse_invoice(&invoice)
}

/// See [input_parser::parse]
pub fn parse_input(s: String) -> Result<InputType> {
    block_on(async { input_parser::parse(&s).await })
}

/*  Payment API's */

/// See [BreezServices::list_payments]
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

/// See [BreezServices::get_payment_by_hash]
pub fn get_payment_by_hash(hash: String) -> Result<Option<Payment>> {
    block_on(async { get_breez_services()?.get_payment_by_hash(hash).await })
}

/*  Lightning Payment API's */

/// See [BreezServices::send_payment]
pub fn send_payment(bolt11: String, amount_sats: Option<u64>) -> Result<Payment> {
    block_on(async {
        get_breez_services()?
            .send_payment(bolt11, amount_sats)
            .await
    })
}

/// See [BreezServices::send_spontaneous_payment]
pub fn send_spontaneous_payment(node_id: String, amount_sats: u64) -> Result<Payment> {
    block_on(async {
        get_breez_services()?
            .send_spontaneous_payment(node_id, amount_sats)
            .await
    })
}

/// See [BreezServices::receive_payment]
pub fn receive_payment(amount_sats: u64, description: String) -> Result<LNInvoice> {
    block_on(async {
        get_breez_services()?
            .receive_payment(amount_sats, description.to_string())
            .await
    })
}

/*  LNURL API's */

/// See [BreezServices::lnurl_pay]
pub fn lnurl_pay(
    user_amount_sat: u64,
    comment: Option<String>,
    req_data: LnUrlPayRequestData,
) -> Result<LnUrlPayResult> {
    block_on(async {
        get_breez_services()?
            .lnurl_pay(user_amount_sat, comment, req_data)
            .await
    })
}

/// See [BreezServices::lnurl_withdraw]
pub fn lnurl_withdraw(
    req_data: LnUrlWithdrawRequestData,
    amount_sats: u64,
    description: Option<String>,
) -> Result<LnUrlCallbackStatus> {
    block_on(async {
        get_breez_services()?
            .lnurl_withdraw(req_data, amount_sats, description)
            .await
    })
}

/// See [BreezServices::lnurl_auth]
pub fn lnurl_auth(req_data: LnUrlAuthRequestData) -> Result<LnUrlCallbackStatus> {
    block_on(async { get_breez_services()?.lnurl_auth(req_data).await })
}

/*  Fiat Currency API's */

/// See [BreezServices::fetch_fiat_rates]
pub fn fetch_fiat_rates() -> Result<Vec<Rate>> {
    block_on(async { get_breez_services()?.fetch_fiat_rates().await })
}

/// See [BreezServices::list_fiat_currencies]
pub fn list_fiat_currencies() -> Result<Vec<FiatCurrency>> {
    block_on(async { get_breez_services()?.list_fiat_currencies().await })
}

/*  On-Chain Swap API's */

/// See [BreezServices::send_onchain]
pub fn send_onchain(
    amount_sat: u64,
    onchain_recipient_address: String,
    pair_hash: String,
    sat_per_vbyte: u64,
) -> Result<ReverseSwapInfo> {
    block_on(async {
        get_breez_services()?
            .send_onchain(
                amount_sat,
                onchain_recipient_address,
                pair_hash,
                sat_per_vbyte,
            )
            .await
    })
}

/// See [BreezServices::receive_onchain]
pub fn receive_onchain() -> Result<SwapInfo> {
    block_on(async { get_breez_services()?.receive_onchain().await })
}

/// See [BreezServices::buy_bitcoin]
pub fn buy_bitcoin(provider: BuyBitcoinProvider) -> Result<String> {
    block_on(async { get_breez_services()?.buy_bitcoin(provider).await })
}

/// See [BreezServices::sweep]
pub fn sweep(to_address: String, fee_rate_sats_per_vbyte: u64) -> Result<()> {
    block_on(async {
        get_breez_services()?
            .sweep(to_address, fee_rate_sats_per_vbyte)
            .await
    })
}

/*  Refundables API's */

/// See [BreezServices::list_refundables]
pub fn list_refundable_swaps() -> Result<Vec<SwapInfo>> {
    block_on(async { get_breez_services()?.list_refundables().await })
}

/// See [BreezServices::refund]
pub fn refund_swap(swap_address: String, to_address: String, sat_per_vbyte: u32) -> Result<String> {
    block_on(async {
        get_breez_services()?
            .refund(swap_address, to_address, sat_per_vbyte)
            .await
    })
}

/*  In Progress Swap API's */

/// See [BreezServices::in_progress_swap]
pub fn get_in_progress_swap() -> Result<Option<SwapInfo>> {
    block_on(async { get_breez_services()?.in_progress_swap().await })
}

/// See [BreezServices::in_progress_reverse_swaps]
pub fn list_in_progress_reverse_swaps() -> Result<Vec<ReverseSwapInfo>> {
    block_on(async { get_breez_services()?.in_progress_reverse_swaps().await })
}

/*  Swap Fee API's */

/// See [BreezServices::fetch_reverse_swap_fees]
pub fn fetch_reverse_swap_fees() -> Result<ReverseSwapPairInfo> {
    block_on(async { get_breez_services()?.fetch_reverse_swap_fees().await })
}

/// See [BreezServices::recommended_fees]
pub fn fetch_recommended_fees() -> Result<RecommendedFees> {
    block_on(async { get_breez_services()?.recommended_fees().await })
}

/*  CLI API's */

/// See [BreezServices::execute_dev_command]
pub fn execute_command(command: String) -> Result<String> {
    block_on(async { get_breez_services()?.execute_dev_command(command).await })
}

/*  Binding Related Logic */

struct BindingLogger;

impl BindingLogger {
    fn init() {
        log::set_boxed_logger(Box::new(BindingLogger {})).unwrap();
        log::set_max_level(LevelFilter::Trace)
    }
}

impl log::Log for BindingLogger {
    fn enabled(&self, m: &Metadata) -> bool {
        m.level() <= Level::Debug
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
        if let Some(stream) = NOTIFICATION_STREAM.get() {
            stream.add(e);
        }
    }
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
