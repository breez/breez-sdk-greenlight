use std::sync::Arc;

use anyhow::Result;

use breez_sdk_core::{
    mnemonic_to_seed as sdk_mnemonic_to_seed, parse as sdk_parse_input,
    parse_invoice as sdk_parse_invoice, AesSuccessActionDataDecrypted, BackupFailedData,
    BackupStatus, BitcoinAddressData, BreezEvent, BreezServices, BuyBitcoinProvider, ChannelState,
    ClosedChannelPaymentDetails, Config, CurrencyInfo, EnvironmentType, EventListener,
    FeeratePreset, FiatCurrency, GreenlightCredentials, InputType, InvoicePaidDetails, LNInvoice,
    LnPaymentDetails, LnUrlAuthRequestData, LnUrlCallbackStatus, LnUrlErrorData,
    LnUrlPayRequestData, LnUrlPayResult, LnUrlWithdrawRequestData, LocaleOverrides, LocalizedName,
    LogEntry, LspInformation, MessageSuccessActionData, MetadataItem, Network, NodeState, Payment,
    PaymentDetails, PaymentFailedData, PaymentType, PaymentTypeFilter, Rate, RecommendedFees,
    ReverseSwapInfo, ReverseSwapPairInfo, ReverseSwapStatus, RouteHint, RouteHintHop,
    SuccessActionProcessed, SwapInfo, SwapStatus, Symbol, UnspentTransactionOutput,
    UrlSuccessActionData,
};
use log::LevelFilter;
use log::Metadata;
use log::Record;
use once_cell::sync::Lazy;

static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

pub trait LogStream: Send + Sync {
    fn log(&self, l: LogEntry);
}

struct BindingLogger {
    log_stream: Box<dyn LogStream>,
}

impl BindingLogger {
    fn init(log_stream: Box<dyn LogStream>) {
        let binding_logger = BindingLogger { log_stream };
        log::set_boxed_logger(Box::new(binding_logger)).unwrap();
        log::set_max_level(LevelFilter::Trace);
    }
}

impl log::Log for BindingLogger {
    fn enabled(&self, m: &Metadata) -> bool {
        // ignroe the internal uniffi log to prevent infinite loop.
        return *m.target() != *"breez_sdk_bindings::uniffi_binding";
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.log_stream.log(LogEntry {
                line: record.args().to_string(),
                level: record.level().as_str().to_string(),
            });
        }
    }
    fn flush(&self) {}
}

#[derive(Debug, thiserror::Error)]
pub enum SDKError {
    #[error("Breez SDK error: {err}")]
    Error { err: String },
}

impl From<anyhow::Error> for SDKError {
    fn from(err: anyhow::Error) -> Self {
        SDKError::Error {
            err: err.to_string(),
        }
    }
}

/// Register a new node in the cloud and return credentials to interact with it
///
/// # Arguments
///
/// * `network` - The network type which is one of (Bitcoin, Testnet, Signet, Regtest)
/// * `seed` - The node private key
pub fn register_node(
    network: Network,
    seed: Vec<u8>,
    register_credentials: Option<GreenlightCredentials>,
    invite_code: Option<String>,
) -> Result<GreenlightCredentials> {
    let creds = rt().block_on(BreezServices::register_node(
        network,
        seed,
        register_credentials,
        invite_code,
    ))?;
    Ok(creds)
}

/// Recover an existing node from the cloud and return credentials to interact with it
///
/// # Arguments
///
/// * `network` - The network type which is one of (Bitcoin, Testnet, Signet, Regtest)
/// * `seed` - The node private key
pub fn recover_node(network: Network, seed: Vec<u8>) -> Result<GreenlightCredentials> {
    let creds = rt().block_on(BreezServices::recover_node(network, seed))?;
    Ok(creds)
}

pub fn default_config(env_type: EnvironmentType) -> Config {
    BreezServices::default_config(env_type)
}

/// init_services initialized the global NodeService, schedule the node to run in the cloud and
/// run the signer. This must be called in order to start comunicate with the node
///
/// # Arguments
///
/// * `config` - The sdk configuration
/// * `seed` - The node private key
/// * `creds` - The greenlight credentials
///
pub fn init_services(
    config: Config,
    seed: Vec<u8>,
    creds: GreenlightCredentials,
    listener: Box<dyn EventListener>,
) -> Result<Arc<BlockingBreezServices>> {
    rt().block_on(async move {
        let breez_services = BreezServices::init_services(config, seed, creds, listener).await?;
        Ok(Arc::new(BlockingBreezServices { breez_services }))
    })
}

pub fn set_log_stream(log_stream: Box<dyn LogStream>) -> Result<()> {
    BindingLogger::init(log_stream);
    Ok(())
}

pub struct BlockingBreezServices {
    breez_services: Arc<BreezServices>,
}

impl BlockingBreezServices {
    pub fn start(&self) -> Result<()> {
        rt().block_on(async move { self.breez_services.start().await })
    }

    pub fn stop(&self) -> Result<()> {
        rt().block_on(self.breez_services.stop())
    }

    pub fn send_payment(
        &self,
        bolt11: String,
        amount_sats: Option<u64>,
    ) -> Result<Payment, SDKError> {
        rt().block_on(self.breez_services.send_payment(bolt11, amount_sats))
            .map_err(|e| e.into())
    }

    pub fn send_spontaneous_payment(
        &self,
        node_id: String,
        amount_sats: u64,
    ) -> Result<Payment, SDKError> {
        rt().block_on(
            self.breez_services
                .send_spontaneous_payment(node_id, amount_sats),
        )
        .map_err(|e| e.into())
    }

    pub fn receive_payment(
        &self,
        amount_sats: u64,
        description: String,
    ) -> Result<LNInvoice, SDKError> {
        rt().block_on(
            self.breez_services
                .receive_payment(amount_sats, description),
        )
        .map_err(|e| e.into())
    }

    pub fn node_info(&self) -> Result<Option<NodeState>, SDKError> {
        self.breez_services.node_info().map_err(|e| e.into())
    }

    pub fn backup_status(&self) -> Result<BackupStatus, SDKError> {
        self.breez_services.backup_status().map_err(|e| e.into())
    }

    pub fn backup(&self) -> Result<(), SDKError> {
        rt().block_on(self.breez_services.backup())
            .map_err(|e| e.into())
    }

    pub fn list_payments(
        &self,
        filter: PaymentTypeFilter,
        from_timestamp: Option<i64>,
        to_timestamp: Option<i64>,
    ) -> Result<Vec<Payment>, SDKError> {
        rt().block_on(
            self.breez_services
                .list_payments(filter, from_timestamp, to_timestamp),
        )
        .map_err(|e| e.into())
    }

    pub fn payment_by_hash(&self, hash: String) -> Result<Option<Payment>, SDKError> {
        rt().block_on(self.breez_services.payment_by_hash(hash))
            .map_err(|e| e.into())
    }

    pub fn pay_lnurl(
        &self,
        req_data: LnUrlPayRequestData,
        amount_sats: u64,
        comment: Option<String>,
    ) -> Result<LnUrlPayResult, SDKError> {
        rt().block_on(
            self.breez_services
                .lnurl_pay(amount_sats, comment, req_data),
        )
        .map_err(|e| e.into())
    }

    pub fn withdraw_lnurl(
        &self,
        req_data: LnUrlWithdrawRequestData,
        amount_sats: u64,
        description: Option<String>,
    ) -> Result<LnUrlCallbackStatus, SDKError> {
        rt().block_on(
            self.breez_services
                .lnurl_withdraw(req_data, amount_sats, description),
        )
        .map_err(|e| e.into())
    }

    pub fn lnurl_auth(
        &self,
        req_data: LnUrlAuthRequestData,
    ) -> Result<LnUrlCallbackStatus, SDKError> {
        rt().block_on(self.breez_services.lnurl_auth(req_data))
            .map_err(|e| e.into())
    }

    pub fn sweep(&self, to_address: String, fee_rate_sats_per_vbyte: u64) -> Result<(), SDKError> {
        rt().block_on(
            self.breez_services
                .sweep(to_address, fee_rate_sats_per_vbyte),
        )
        .map_err(|e| e.into())
    }

    pub fn fetch_fiat_rates(&self) -> Result<Vec<Rate>, SDKError> {
        rt().block_on(self.breez_services.fetch_fiat_rates())
            .map_err(|e| e.into())
    }

    pub fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>, SDKError> {
        rt().block_on(self.breez_services.list_fiat_currencies())
            .map_err(|e| e.into())
    }

    pub fn list_lsps(&self) -> Result<Vec<LspInformation>, SDKError> {
        rt().block_on(self.breez_services.list_lsps())
            .map_err(|e| e.into())
    }

    pub fn connect_lsp(&self, lsp_id: String) -> Result<(), SDKError> {
        rt().block_on(self.breez_services.connect_lsp(lsp_id))
            .map_err(|e| e.into())
    }

    /// Convenience method to look up LSP info based on current LSP ID
    pub fn fetch_lsp_info(&self, lsp_id: String) -> Result<Option<LspInformation>, SDKError> {
        rt().block_on(self.breez_services.fetch_lsp_info(lsp_id))
            .map_err(|e| e.into())
    }

    pub fn lsp_id(&self) -> Result<Option<String>, SDKError> {
        rt().block_on(self.breez_services.lsp_id())
            .map_err(|e| e.into())
    }

    pub fn close_lsp_channels(&self) -> Result<(), SDKError> {
        rt().block_on(async {
            _ = self.breez_services.close_lsp_channels().await?;
            Ok(())
        })
        .map_err(|e: anyhow::Error| e.into())
    }

    /// Onchain receive swap API
    pub fn receive_onchain(&self) -> Result<SwapInfo, SDKError> {
        rt().block_on(self.breez_services.receive_onchain())
            .map_err(|e| e.into())
    }

    /// Onchain receive swap API
    pub fn in_progress_swap(&self) -> Result<Option<SwapInfo>, SDKError> {
        rt().block_on(self.breez_services.in_progress_swap())
            .map_err(|e| e.into())
    }

    /// list non-completed expired swaps that should be refunded bu calling [BreezServices::refund]
    pub fn list_refundables(&self) -> Result<Vec<SwapInfo>, SDKError> {
        rt().block_on(self.breez_services.list_refundables())
            .map_err(|e| e.into())
    }

    // construct and broadcast a refund transaction for a faile/expired swap
    pub fn refund(
        &self,
        swap_address: String,
        to_address: String,
        sat_per_vbyte: u32,
    ) -> Result<String, SDKError> {
        rt().block_on(
            self.breez_services
                .refund(swap_address, to_address, sat_per_vbyte),
        )
        .map_err(|e| e.into())
    }

    pub fn fetch_reverse_swap_fees(&self) -> Result<ReverseSwapPairInfo, SDKError> {
        rt().block_on(self.breez_services.fetch_reverse_swap_fees())
            .map_err(|e| e.into())
    }

    pub fn in_progress_reverse_swaps(&self) -> Result<Vec<ReverseSwapInfo>, SDKError> {
        rt().block_on(self.breez_services.in_progress_reverse_swaps())
            .map_err(|e| e.into())
    }

    pub fn send_onchain(
        &self,
        amount_sat: u64,
        onchain_recipient_address: String,
        pair_hash: String,
        sat_per_vbyte: u64,
    ) -> Result<ReverseSwapInfo, SDKError> {
        rt().block_on(self.breez_services.send_onchain(
            amount_sat,
            onchain_recipient_address,
            pair_hash,
            sat_per_vbyte,
        ))
        .map_err(|e| e.into())
    }

    pub fn execute_dev_command(&self, command: String) -> Result<String> {
        rt().block_on(self.breez_services.execute_dev_command(command))
    }

    pub fn sync(&self) -> Result<(), SDKError> {
        rt().block_on(self.breez_services.sync())
            .map_err(|e| e.into())
    }

    pub fn recommended_fees(&self) -> Result<RecommendedFees, SDKError> {
        rt().block_on(self.breez_services.recommended_fees())
            .map_err(|e| e.into())
    }

    pub fn buy_bitcoin(&self, provider: BuyBitcoinProvider) -> Result<String, SDKError> {
        rt().block_on(self.breez_services.buy_bitcoin(provider))
            .map_err(|e| e.into())
    }
}

pub fn parse_invoice(invoice: String) -> Result<LNInvoice, SDKError> {
    sdk_parse_invoice(&invoice).map_err(|e| e.into())
}

pub fn parse_input(s: String) -> Result<InputType, SDKError> {
    rt().block_on(sdk_parse_input(&s)).map_err(|e| e.into())
}

pub fn mnemonic_to_seed(phrase: String) -> Result<Vec<u8>, SDKError> {
    sdk_mnemonic_to_seed(phrase).map_err(|e| e.into())
}

fn rt() -> &'static tokio::runtime::Runtime {
    &RT
}

uniffi_macros::include_scaffolding!("breez_sdk");
