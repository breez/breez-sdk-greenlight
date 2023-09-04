//! Bindings for the Dart integration
//!
//! ### Error handling
//!
//! Since the integration requires the methods to return `anyhow::Result`, but the SDK service methods
//! are being converted to return `SdkResult`, we have two ways to handle errors:
//! - by using `Into::into`, which converts the `SdkError` enum to a generic `anyhow::Error`
//! - by wrapping the `SdkError` in an `anyhow::Error`
//!
//! The first option loses the `SdkError` type. The second option keeps the type, which we can retrieve
//! with `anyhow::Error::downcast_ref` (or equivalent Dart method). We therefore use the second approach.

use std::future::Future;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use flutter_rust_bridge::StreamSink;
use once_cell::sync::{Lazy, OnceCell};
use tokio::sync::mpsc;

use crate::breez_services::{self, BreezEvent, BreezServices, EventListener};
use crate::chain::RecommendedFees;
use crate::error::SdkError;
use crate::fiat::{FiatCurrency, Rate};
use crate::input_parser::{
    self, InputType, LnUrlAuthRequestData, LnUrlPayRequestData, LnUrlWithdrawRequestData,
};
use crate::invoice::{self, LNInvoice};
use crate::lnurl::pay::model::LnUrlPayResult;
use crate::lsp::LspInformation;
use crate::models::{Config, LogEntry, NodeState, Payment, PaymentTypeFilter, SwapInfo};
use crate::{
    BackupStatus, BuyBitcoinRequest, BuyBitcoinResponse, CheckMessageRequest, CheckMessageResponse,
    EnvironmentType, LnUrlCallbackStatus, LogStream, NodeConfig, ReceiveOnchainRequest,
    ReceivePaymentRequest, ReceivePaymentResponse, ReverseSwapFeesRequest, ReverseSwapInfo,
    ReverseSwapPairInfo, SignMessageRequest, SignMessageResponse,
};

static BREEZ_SERVICES_INSTANCE: OnceCell<Arc<BreezServices>> = OnceCell::new();
static BREEZ_SERVICES_SHUTDOWN: OnceCell<mpsc::Sender<()>> = OnceCell::new();
static NOTIFICATION_STREAM: OnceCell<StreamSink<BreezEvent>> = OnceCell::new();
static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

/*  Breez Services API's */

/// Wrapper around [BreezServices::connect] which also initializes SDK logging
pub fn connect(config: Config, seed: Vec<u8>) -> Result<()> {
    block_on(async move {
        let breez_services =
            BreezServices::connect(config, seed, Box::new(BindingEventListener {})).await?;

        BREEZ_SERVICES_INSTANCE
            .set(breez_services)
            .map_err(|_| SdkError::InitFailed {
                err: "static node services already set".into(),
            })?;

        Ok(())
    })
    .map_err(anyhow::Error::new::<SdkError>)
}

/// Check whether node service is initialized or not
pub fn is_initialized() -> bool {
    block_on(async { get_breez_services().is_ok() })
}

/// See [BreezServices::sync]
pub fn sync() -> Result<()> {
    block_on(async { get_breez_services()?.sync().await })
}

/// See [BreezServices::node_info]
pub fn node_info() -> Result<NodeState> {
    block_on(async {
        get_breez_services()?
            .node_info()
            .map_err(anyhow::Error::new)
    })
}

/// Cleanup node resources and stop the signer.
pub fn disconnect() -> Result<()> {
    block_on(async {
        let shutdown_handler = BREEZ_SERVICES_SHUTDOWN.get();
        match shutdown_handler {
            None => Err(anyhow!("Background processing is not running")),
            Some(s) => s.send(()).await.map_err(anyhow::Error::msg),
        }
    })
}

/// See [BreezServices::sign_message]
pub fn sign_message(request: SignMessageRequest) -> Result<SignMessageResponse> {
    block_on(async { get_breez_services()?.sign_message(request).await })
}

/// See [BreezServices::check_message]
pub fn check_message(request: CheckMessageRequest) -> Result<CheckMessageResponse> {
    block_on(async { get_breez_services()?.check_message(request).await })
}

/*  Breez Services Helper API's */

/// See [breez_services::mnemonic_to_seed]
pub fn mnemonic_to_seed(phrase: String) -> Result<Vec<u8>> {
    breez_services::mnemonic_to_seed(phrase)
}

/// See [BreezServices::default_config]
pub fn default_config(
    env_type: EnvironmentType,
    api_key: String,
    node_config: NodeConfig,
) -> Config {
    BreezServices::default_config(env_type, api_key, node_config)
}

/*  Stream API's */

/// If used, this must be called before `connect`. It can only be called once.
pub fn breez_events_stream(s: StreamSink<BreezEvent>) -> Result<()> {
    NOTIFICATION_STREAM
        .set(s)
        .map_err(|_| anyhow!("events stream already created"))?;
    Ok(())
}

pub fn set_log_directory(log_dir: String) -> Result<()> {
    BreezServices::set_log_directory(log_dir).map_err(anyhow::Error::new)
}

/// If used, this must be called before `connect`. It can only be called once.
pub fn breez_log_stream(sink: StreamSink<LogEntry>) -> Result<()> {
    BreezServices::set_log_listener(Box::new(StreamSinkWrapper { sink }))?;
    Ok(())
}

/*  LSP API's */

/// See [BreezServices::list_lsps]
pub fn list_lsps() -> Result<Vec<LspInformation>> {
    block_on(async { get_breez_services()?.list_lsps().await }).map_err(anyhow::Error::new)
}

/// See [BreezServices::connect_lsp]
pub fn connect_lsp(lsp_id: String) -> Result<()> {
    block_on(async { get_breez_services()?.connect_lsp(lsp_id).await })
        .map_err(anyhow::Error::new::<SdkError>)
}

/// See [BreezServices::lsp_id]
pub fn lsp_id() -> Result<Option<String>> {
    block_on(async { get_breez_services()?.lsp_id().await }).map_err(anyhow::Error::new)
}

/// See [BreezServices::fetch_lsp_info]
pub fn fetch_lsp_info(id: String) -> Result<Option<LspInformation>> {
    block_on(async { get_breez_services()?.fetch_lsp_info(id).await })
}

/// See [BreezServices::lsp_info]
pub fn lsp_info() -> Result<LspInformation> {
    block_on(async { get_breez_services()?.lsp_info().await })
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
pub fn backup_status() -> Result<BackupStatus> {
    get_breez_services()?.backup_status()
}

/*  Parse API's */

pub fn parse_invoice(invoice: String) -> Result<LNInvoice> {
    invoice::parse_invoice(&invoice)
}

pub fn parse_input(input: String) -> Result<InputType> {
    block_on(async { input_parser::parse(&input).await })
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
    .map_err(anyhow::Error::new)
}

/// See [BreezServices::list_payments]
pub fn payment_by_hash(hash: String) -> Result<Option<Payment>> {
    block_on(async { get_breez_services()?.payment_by_hash(hash).await })
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
pub fn receive_payment(req_data: ReceivePaymentRequest) -> Result<ReceivePaymentResponse> {
    block_on(async { get_breez_services()?.receive_payment(req_data).await })
        .map_err(anyhow::Error::new)
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
pub fn receive_onchain(req_data: ReceiveOnchainRequest) -> Result<SwapInfo> {
    block_on(async { get_breez_services()?.receive_onchain(req_data).await })
}

/// See [BreezServices::buy_bitcoin]
pub fn buy_bitcoin(req_data: BuyBitcoinRequest) -> Result<BuyBitcoinResponse> {
    block_on(async { get_breez_services()?.buy_bitcoin(req_data).await })
        .map_err(anyhow::Error::new)
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
pub fn list_refundables() -> Result<Vec<SwapInfo>> {
    block_on(async { get_breez_services()?.list_refundables().await })
}

/// See [BreezServices::refund]
pub fn refund(swap_address: String, to_address: String, sat_per_vbyte: u32) -> Result<String> {
    block_on(async {
        get_breez_services()?
            .refund(swap_address, to_address, sat_per_vbyte)
            .await
    })
}

/*  In Progress Swap API's */

/// See [BreezServices::in_progress_swap]
pub fn in_progress_swap() -> Result<Option<SwapInfo>> {
    block_on(async { get_breez_services()?.in_progress_swap().await })
}

/// See [BreezServices::in_progress_reverse_swaps]
pub fn in_progress_reverse_swaps() -> Result<Vec<ReverseSwapInfo>> {
    block_on(async { get_breez_services()?.in_progress_reverse_swaps().await })
}

/*  Swap Fee API's */

/// See [BreezServices::fetch_reverse_swap_fees]
pub fn fetch_reverse_swap_fees(req: ReverseSwapFeesRequest) -> Result<ReverseSwapPairInfo> {
    block_on(async { get_breez_services()?.fetch_reverse_swap_fees(req).await })
}

/// See [BreezServices::recommended_fees]
pub fn recommended_fees() -> Result<RecommendedFees> {
    block_on(async { get_breez_services()?.recommended_fees().await })
}

/*  CLI API's */

/// See [BreezServices::execute_dev_command]
pub fn execute_command(command: String) -> Result<String> {
    block_on(async { get_breez_services()?.execute_dev_command(command).await })
}

/*  Binding Related Logic */

struct BindingEventListener;

impl EventListener for BindingEventListener {
    fn on_event(&self, e: BreezEvent) {
        if let Some(stream) = NOTIFICATION_STREAM.get() {
            stream.add(e);
        }
    }
}

struct StreamSinkWrapper {
    sink: StreamSink<LogEntry>,
}

impl LogStream for StreamSinkWrapper {
    fn log(&self, record: LogEntry) {
        self.sink.add(LogEntry {
            line: record.line,
            level: record.level,
        });
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
