use std::{
    fs::OpenOptions,
    io::Write,
    sync::Arc,
    time::{Duration, SystemTime},
};

use anyhow::{anyhow, Result};
use bip39::{Language, Mnemonic, Seed};
use chrono::Local;
use log::{LevelFilter, Metadata, Record};
use sdk_common::prelude::{
    BreezServer, FiatCurrency, LnUrlAuthError, LnUrlAuthRequestData, LnUrlCallbackStatus,
    LnUrlPayError, LnUrlPayRequest, LnUrlWithdrawError, LnUrlWithdrawRequest, LnUrlWithdrawResult,
    Rate, PRODUCTION_BREEZSERVER_URL,
};
use tokio::sync::Mutex;

use crate::{
    error::{
        ReceiveOnchainError, ReceiveOnchainResult, ReceivePaymentError, RedeemOnchainResult,
        SdkResult, SendOnchainError, SendPaymentError,
    },
    internal_breez_services::{
        BreezServicesResult, CheckMessageRequest, CheckMessageResponse, EventListener,
        InternalBreezServices, SignMessageRequest, SignMessageResponse,
    },
    lnurl::pay::LnUrlPayResult,
    persist::db::SqliteStorage,
    BackupStatus, BuyBitcoinRequest, BuyBitcoinResponse, Config, ConfigureNodeRequest,
    ConnectRequest, EnvironmentType, ListPaymentsRequest, LspInformation,
    MaxReverseSwapAmountResponse, NodeConfig, NodeCredentials, NodeState,
    OnchainPaymentLimitsResponse, OpenChannelFeeRequest, OpenChannelFeeResponse, PayOnchainRequest,
    PayOnchainResponse, Payment, PrepareOnchainPaymentRequest, PrepareOnchainPaymentResponse,
    PrepareRedeemOnchainFundsRequest, PrepareRedeemOnchainFundsResponse, PrepareRefundRequest,
    PrepareRefundResponse, ReceiveOnchainRequest, ReceivePaymentRequest, ReceivePaymentResponse,
    RecommendedFees, RedeemOnchainFundsRequest, RedeemOnchainFundsResponse, RefundRequest,
    RefundResponse, ReportIssueRequest, ReverseSwapFeesRequest, ReverseSwapInfo,
    ReverseSwapPairInfo, SendOnchainRequest, SendOnchainResponse, SendPaymentRequest,
    SendPaymentResponse, SendSpontaneousPaymentRequest, ServiceHealthCheckResponse,
    StaticBackupRequest, StaticBackupResponse, SupportAPI, SwapInfo,
};

const DETECT_HIBERNATE_SLEEP_DURATION: Duration = Duration::from_secs(1);
const DETECT_HIBERNATE_MAX_OFFSET: Duration = Duration::from_secs(2);
/// BreezServices is a facade and the single entry point for the SDK.
pub struct BreezServices {
    services: Mutex<Arc<InternalBreezServices>>,
    req: ConnectRequest,
    event_listener: Arc<Box<dyn EventListener>>,
}

impl BreezServices {
    /// `connect` initializes the SDK services, schedules the node to run in the cloud and
    /// runs the signer. This must be called in order to start communicating with the node.
    ///
    /// # Arguments
    ///
    /// * `req` - The connect request containing the `config` SDK configuration and `seed` node
    ///   private key, typically derived from the mnemonic. When using a new `invite_code`,
    ///   the seed should be derived from a new random mnemonic. When re-using an `invite_code`,
    ///   the same mnemonic should be used as when the `invite_code` was first used.
    /// * `event_listener` - Listener to SDK events
    ///
    pub async fn connect(
        req: ConnectRequest,
        event_listener: Box<dyn EventListener>,
    ) -> BreezServicesResult<Arc<BreezServices>> {
        let event_listener = Arc::new(event_listener);
        let services =
            InternalBreezServices::connect(req.clone(), Arc::clone(&event_listener)).await?;

        let services = Arc::new(BreezServices {
            event_listener,
            req,
            services: Mutex::new(services),
        });
        services.detect_hibernation();
        Ok(services)
    }

    /// Get the full default config for a specific environment type
    pub fn default_config(
        env_type: EnvironmentType,
        api_key: String,
        node_config: NodeConfig,
    ) -> Config {
        match env_type {
            EnvironmentType::Production => Config::production(api_key, node_config),
            EnvironmentType::Staging => Config::staging(api_key, node_config),
        }
    }

    fn detect_hibernation(self: &Arc<BreezServices>) {
        let cloned = Arc::clone(self);
        tokio::spawn(async move {
            loop {
                let now = SystemTime::now();
                tokio::time::sleep(DETECT_HIBERNATE_SLEEP_DURATION).await;
                let elapsed = match now.elapsed() {
                    Ok(elapsed) => elapsed,
                    Err(e) => {
                        error!("track_hibernation failed with: {:?}", e);
                        continue;
                    }
                };

                if elapsed
                    .saturating_sub(DETECT_HIBERNATE_SLEEP_DURATION)
                    .ge(&DETECT_HIBERNATE_MAX_OFFSET)
                {
                    debug!("Hibernation detected: time diff {}s", elapsed.as_secs_f32());
                    let mut services = cloned.services.lock().await;
                    debug!("Hibernation detected: disconnecting services");
                    let _ = services.disconnect().await;
                    debug!("Hibernation detected: services disconnected");
                    debug!("Hibernation detected: reconnecting services");
                    let new_services = match InternalBreezServices::connect(
                        cloned.req.clone(),
                        Arc::clone(&cloned.event_listener),
                    )
                    .await
                    {
                        Ok(new_services) => new_services,
                        Err(e) => {
                            // TODO: retry this reconnect later.
                            error!(
                                "Failed to reconnect breez services after hibernation: {:?}",
                                e
                            );
                            continue;
                        }
                    };

                    debug!("Hibernation detected: services reconnected");
                    *services = new_services;
                }
            }
        });
    }

    /// Configures a global SDK logger that will log to file and will forward log events to
    /// an optional application-specific logger.
    ///
    /// If called, it should be called before any SDK methods (for example, before `connect`).
    ///
    /// It must be called only once in the application lifecycle. Alternatively, If the application
    /// already uses a globally-registered logger, this method shouldn't be called at all.
    ///
    /// ### Arguments
    ///
    /// - `log_dir`: Location where the the SDK log file will be created. The directory must already exist.
    ///
    /// - `app_logger`: Optional application logger.
    ///
    /// If the application is to use it's own logger, but would also like the SDK to log SDK-specific
    /// log output to a file in the configured `log_dir`, then do not register the
    /// app-specific logger as a global logger and instead call this method with the app logger as an arg.
    ///
    /// ### Logging Configuration
    ///
    /// Setting `breez_sdk_core::input_parser=debug` will include in the logs the raw payloads received
    /// when interacting with JSON endpoints, for example those used during all LNURL workflows.
    ///
    /// ### Errors
    ///
    /// An error is thrown if the log file cannot be created in the working directory.
    ///
    /// An error is thrown if a global logger is already configured.
    pub fn init_logging(log_dir: &str, app_logger: Option<Box<dyn log::Log>>) -> Result<()> {
        let target_log_file = Box::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(format!("{log_dir}/sdk.log"))
                .map_err(|e| anyhow!("Can't create log file: {e}"))?,
        );
        let logger = env_logger::Builder::new()
            .target(env_logger::Target::Pipe(target_log_file))
            .parse_filters(
                r#"
                debug,
                breez_sdk_core::input_parser=warn,
                breez_sdk_core::backup=info,
                breez_sdk_core::persist::reverseswap=info,
                breez_sdk_core::reverseswap=info,
                gl_client=debug,
                h2=warn,
                hyper=warn,
                lightning_signer=warn,
                reqwest=warn,
                rustls=warn,
                rustyline=warn,
                vls_protocol_signer=warn
            "#,
            )
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{} {} {}:{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.level(),
                    record.module_path().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    record.args()
                )
            })
            .build();

        let global_logger = GlobalSdkLogger {
            logger,
            log_listener: app_logger,
        };

        log::set_boxed_logger(Box::new(global_logger))
            .map_err(|e| anyhow!("Failed to set global logger: {e}"))?;
        log::set_max_level(LevelFilter::Trace);

        Ok(())
    }

    /// Fetches the service health check from the support API.
    pub async fn service_health_check(api_key: String) -> SdkResult<ServiceHealthCheckResponse> {
        let support_api: Arc<dyn SupportAPI> = Arc::new(BreezServer::new(
            PRODUCTION_BREEZSERVER_URL.to_string(),
            Some(api_key),
        )?);

        support_api.service_health_check().await
    }

    /// Get the static backup data from the persistent storage.
    /// This data enables the user to recover the node in an external core ligntning node.
    /// See here for instructions on how to recover using this data: <https://docs.corelightning.org/docs/backup-and-recovery#backing-up-using-static-channel-backup>
    pub fn static_backup(req: StaticBackupRequest) -> SdkResult<StaticBackupResponse> {
        let storage = SqliteStorage::new(req.working_dir);
        Ok(StaticBackupResponse {
            backup: storage.get_static_backup()?,
        })
    }

    /// Force running backup
    pub async fn backup(&self) -> SdkResult<()> {
        self.get_services().await.backup().await
    }

    /// Retrieve the node up to date BackupStatus
    pub async fn backup_status(&self) -> SdkResult<BackupStatus> {
        self.get_services().await.backup_status()
    }

    /// Generates an url that can be used by a third part provider to buy Bitcoin with fiat currency.
    ///
    /// A user-selected [OpeningFeeParams] can be optionally set in the argument. If set, and the
    /// operation requires a new channel, the SDK will try to use the given fee params.
    pub async fn buy_bitcoin(
        &self,
        req: BuyBitcoinRequest,
    ) -> Result<BuyBitcoinResponse, ReceiveOnchainError> {
        self.get_services().await.buy_bitcoin(req).await
    }

    /// Check whether given message was signed by the private key or the given
    /// pubkey and the signature (zbase encoded) is valid.
    pub async fn check_message(&self, req: CheckMessageRequest) -> SdkResult<CheckMessageResponse> {
        self.get_services().await.check_message(req).await
    }

    /// Claims an individual reverse swap.
    ///
    /// To be used only in the context of mobile notifications, where the notification triggers
    /// an individual reverse swap to be claimed.
    ///
    /// This is taken care of automatically in the context of typical SDK usage.
    pub async fn claim_reverse_swap(&self, lockup_address: String) -> SdkResult<()> {
        self.get_services()
            .await
            .claim_reverse_swap(lockup_address)
            .await
    }

    /// Close all channels with the current LSP.
    ///
    /// Should be called  when the user wants to close all the channels.
    pub async fn close_lsp_channels(&self) -> SdkResult<Vec<String>> {
        self.get_services().await.close_lsp_channels().await
    }

    /// Configure the node
    ///
    /// This calls [NodeAPI::configure_node] to make changes to the active node's configuration.
    /// Configuring the [ConfigureNodeRequest::close_to_address] only needs to be done one time
    /// when registering the node or when the close to address need to be changed. Otherwise it is
    /// stored by the node and used when neccessary.
    pub async fn configure_node(&self, req: ConfigureNodeRequest) -> SdkResult<()> {
        self.get_services().await.configure_node(req).await
    }

    /// Select the LSP to be used and provide inbound liquidity
    pub async fn connect_lsp(&self, lsp_id: String) -> SdkResult<()> {
        self.get_services().await.connect_lsp(lsp_id).await
    }

    /// Trigger the stopping of BreezServices background threads for this instance.
    pub async fn disconnect(&self) -> SdkResult<()> {
        self.get_services().await.disconnect().await
    }

    /// Execute a command directly on the NodeAPI interface.
    /// Mainly used to debugging.
    pub async fn execute_dev_command(&self, command: String) -> SdkResult<String> {
        self.get_services().await.execute_dev_command(command).await
    }

    /// Fetch live rates of fiat currencies, sorted by name
    pub async fn fetch_fiat_rates(&self) -> SdkResult<Vec<Rate>> {
        self.get_services().await.fetch_fiat_rates().await
    }

    /// Convenience method to look up [LspInformation] for a given LSP ID
    pub async fn fetch_lsp_info(&self, id: String) -> SdkResult<Option<LspInformation>> {
        self.get_services().await.fetch_lsp_info(id).await
    }

    /// Lookup the reverse swap fees (see [ReverseSwapServiceAPI::fetch_reverse_swap_fees]).
    ///
    /// If the request has the `send_amount_sat` set, the returned [ReverseSwapPairInfo] will have
    /// the total estimated fees for the reverse swap in its `total_estimated_fees`.
    ///
    /// If, in addition to that, the request has the `claim_tx_feerate` set as well, then
    /// - `fees_claim` will have the actual claim transaction fees, instead of an estimate, and
    /// - `total_estimated_fees` will have the actual total fees for the given parameters
    ///
    /// ### Errors
    ///
    /// If a `send_amount_sat` is specified in the `req`, but is outside the `min` and `max`,
    /// this will result in an error. If you are not sure what are the `min` and `max`, please call
    /// this with `send_amount_sat` as `None` first, then repeat the call with the desired amount.
    pub async fn fetch_reverse_swap_fees(
        &self,
        req: ReverseSwapFeesRequest,
    ) -> SdkResult<ReverseSwapPairInfo> {
        self.get_services().await.fetch_reverse_swap_fees(req).await
    }

    // Collects various user data from the node and the sdk storage.
    // This is used for debugging and support purposes only.
    pub async fn generate_diagnostic_data(&self) -> SdkResult<String> {
        self.get_services().await.generate_diagnostic_data().await
    }

    async fn get_services(&self) -> Arc<InternalBreezServices> {
        Arc::clone(&*self.services.lock().await)
    }

    /// Returns the blocking [ReverseSwapInfo]s that are in progress.
    ///
    /// Supersedes [BreezServices::in_progress_reverse_swaps]
    pub async fn in_progress_onchain_payments(&self) -> SdkResult<Vec<ReverseSwapInfo>> {
        self.get_services()
            .await
            .in_progress_onchain_payments()
            .await
    }

    /// Returns the blocking [ReverseSwapInfo]s that are in progress
    #[deprecated(note = "use in_progress_onchain_payments instead")]
    pub async fn in_progress_reverse_swaps(&self) -> SdkResult<Vec<ReverseSwapInfo>> {
        #[allow(deprecated)]
        self.get_services().await.in_progress_reverse_swaps().await
    }

    /// Returns an optional in-progress [SwapInfo].
    /// A [SwapInfo] is in-progress if it is waiting for confirmation to be redeemed and complete the swap.
    pub async fn in_progress_swap(&self) -> SdkResult<Option<SwapInfo>> {
        self.get_services().await.in_progress_swap().await
    }

    /// List all supported fiat currencies for which there is a known exchange rate.
    /// List is sorted by the canonical name of the currency
    pub async fn list_fiat_currencies(&self) -> SdkResult<Vec<FiatCurrency>> {
        self.get_services().await.list_fiat_currencies().await
    }

    /// List available LSPs that can be selected by the user
    pub async fn list_lsps(&self) -> SdkResult<Vec<LspInformation>> {
        self.get_services().await.list_lsps().await
    }

    /// List payments matching the given filters, as retrieved from persistent storage
    pub async fn list_payments(&self, req: ListPaymentsRequest) -> SdkResult<Vec<Payment>> {
        self.get_services().await.list_payments(req).await
    }

    /// list non-completed expired swaps that should be refunded by calling [BreezServices::refund]
    pub async fn list_refundables(&self) -> SdkResult<Vec<SwapInfo>> {
        self.get_services().await.list_refundables().await
    }

    /// Third and last step of LNURL-auth. The first step is `parse()`, which also validates the LNURL destination
    /// and generates the `LnUrlAuthRequestData` payload needed here. The second step is user approval of auth action.
    ///
    /// This call will sign `k1` of the LNURL endpoint (`req_data`) on `secp256k1` using `linkingPrivKey` and DER-encodes the signature.
    /// If they match the endpoint requirements, the LNURL auth request is made. A successful result here means the client signature is verified.
    pub async fn lnurl_auth(
        &self,
        req_data: LnUrlAuthRequestData,
    ) -> Result<LnUrlCallbackStatus, LnUrlAuthError> {
        self.get_services().await.lnurl_auth(req_data).await
    }

    /// Second step of LNURL-pay. The first step is `parse()`, which also validates the LNURL destination
    /// and generates the `LnUrlPayRequest` payload needed here.
    ///
    /// This call will validate the `amount_msat` and `comment` parameters of `req` against the parameters
    /// of the LNURL endpoint (`req_data`). If they match the endpoint requirements, the LNURL payment
    /// is made.
    ///
    /// This method will return an [anyhow::Error] when any validation check fails.
    pub async fn lnurl_pay(&self, req: LnUrlPayRequest) -> Result<LnUrlPayResult, LnUrlPayError> {
        self.get_services().await.lnurl_pay(req).await
    }

    /// Second step of LNURL-withdraw. The first step is `parse()`, which also validates the LNURL destination
    /// and generates the `LnUrlWithdrawRequest` payload needed here.
    ///
    /// This call will validate the given `amount_msat` against the parameters
    /// of the LNURL endpoint (`data`). If they match the endpoint requirements, the LNURL withdraw
    /// request is made. A successful result here means the endpoint started the payment.
    pub async fn lnurl_withdraw(
        &self,
        req: LnUrlWithdrawRequest,
    ) -> Result<LnUrlWithdrawResult, LnUrlWithdrawError> {
        self.get_services().await.lnurl_withdraw(req).await
    }

    /// Get the current LSP's ID
    pub async fn lsp_id(&self) -> SdkResult<Option<String>> {
        self.get_services().await.lsp_id().await
    }

    /// Convenience method to look up LSP info based on current LSP ID
    pub async fn lsp_info(&self) -> SdkResult<LspInformation> {
        self.get_services().await.lsp_info().await
    }

    /// Returns the max amount that can be sent on-chain using the send_onchain method.
    /// The returned amount is the sum of the max amount that can be sent on each channel
    /// minus the expected fees.
    /// This is possible since the route to the swapper node is known in advance and is expected
    /// to consist of maximum 3 hops.
    #[deprecated(note = "use onchain_payment_limits instead")]
    pub async fn max_reverse_swap_amount(&self) -> SdkResult<MaxReverseSwapAmountResponse> {
        #[allow(deprecated)]
        self.get_services().await.max_reverse_swap_amount().await
    }

    /// Retrieve the decrypted credentials from the node.
    pub async fn node_credentials(&self) -> SdkResult<Option<NodeCredentials>> {
        self.get_services().await.node_credentials()
    }

    /// Retrieve the node state from the persistent storage.
    ///
    /// Fail if it could not be retrieved or if `None` was found.
    pub async fn node_info(&self) -> SdkResult<NodeState> {
        self.get_services().await.node_info()
    }

    pub async fn onchain_payment_limits(&self) -> SdkResult<OnchainPaymentLimitsResponse> {
        self.get_services().await.onchain_payment_limits().await
    }

    /// Gets the fees required to open a channel for a given amount.
    /// If no channel is needed, returns 0. If a channel is needed, returns the required opening fees.
    pub async fn open_channel_fee(
        &self,
        req: OpenChannelFeeRequest,
    ) -> SdkResult<OpenChannelFeeResponse> {
        self.get_services().await.open_channel_fee(req).await
    }

    /// Creates a reverse swap and attempts to pay the HODL invoice
    ///
    /// Supersedes [BreezServices::send_onchain]
    pub async fn pay_onchain(
        &self,
        req: PayOnchainRequest,
    ) -> Result<PayOnchainResponse, SendOnchainError> {
        self.get_services().await.pay_onchain(req).await
    }

    /// Fetch a specific payment by its hash.
    pub async fn payment_by_hash(&self, hash: String) -> SdkResult<Option<Payment>> {
        self.get_services().await.payment_by_hash(hash).await
    }

    /// Supersedes [BreezServices::fetch_reverse_swap_fees]
    ///
    /// ### Errors
    ///
    /// - `OutOfRange`: This indicates the send amount is outside the range of minimum and maximum
    ///   values returned by [BreezServices::onchain_payment_limits]. When you get this error, please first call
    ///   [BreezServices::onchain_payment_limits] to get the new limits, before calling this method again.
    pub async fn prepare_onchain_payment(
        &self,
        req: PrepareOnchainPaymentRequest,
    ) -> Result<PrepareOnchainPaymentResponse, SendOnchainError> {
        self.get_services().await.prepare_onchain_payment(req).await
    }

    pub async fn prepare_redeem_onchain_funds(
        &self,
        req: PrepareRedeemOnchainFundsRequest,
    ) -> RedeemOnchainResult<PrepareRedeemOnchainFundsResponse> {
        self.get_services()
            .await
            .prepare_redeem_onchain_funds(req)
            .await
    }

    /// Prepares a refund transaction for a failed/expired swap.
    ///
    /// Can optionally be used before [BreezServices::refund] to know how much fees will be paid
    /// to perform the refund.
    pub async fn prepare_refund(
        &self,
        req: PrepareRefundRequest,
    ) -> SdkResult<PrepareRefundResponse> {
        self.get_services().await.prepare_refund(req).await
    }

    /// Creates an bolt11 payment request.
    /// This also works when the node doesn't have any channels and need inbound liquidity.
    /// In such case when the invoice is paid a new zero-conf channel will be open by the LSP,
    /// providing inbound liquidity and the payment will be routed via this new channel.
    pub async fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, ReceivePaymentError> {
        self.get_services().await.receive_payment(req).await
    }

    /// Onchain receive swap API
    ///
    /// Create and start a new swap. A user-selected [OpeningFeeParams] can be optionally set in the argument.
    /// If set, and the operation requires a new channel, the SDK will try to use the given fee params.
    ///
    /// Since we only allow one in-progress swap this method will return error if there is currently
    /// a swap waiting for confirmation to be redeemed and by that complete the swap.
    /// In such case the [BreezServices::in_progress_swap] can be used to query the live swap status.
    ///
    /// The returned [SwapInfo] contains the created swap details. The channel opening fees are
    /// available at [SwapInfo::channel_opening_fees].
    pub async fn receive_onchain(
        &self,
        req: ReceiveOnchainRequest,
    ) -> ReceiveOnchainResult<SwapInfo> {
        self.get_services().await.receive_onchain(req).await
    }

    /// Get the recommended fees for onchain transactions
    pub async fn recommended_fees(&self) -> SdkResult<RecommendedFees> {
        self.get_services().await.recommended_fees().await
    }

    /// Redeem on-chain funds from closed channels to the specified on-chain address, with the given feerate
    pub async fn redeem_onchain_funds(
        &self,
        req: RedeemOnchainFundsRequest,
    ) -> RedeemOnchainResult<RedeemOnchainFundsResponse> {
        self.get_services().await.redeem_onchain_funds(req).await
    }

    /// Redeems an individual swap.
    ///
    /// To be used only in the context of mobile notifications, where the notification triggers
    /// an individual redeem.
    ///
    /// This is taken care of automatically in the context of typical SDK usage.
    pub async fn redeem_swap(&self, swap_address: String) -> SdkResult<()> {
        self.get_services().await.redeem_swap(swap_address).await
    }

    /// Construct and broadcast a refund transaction for a failed/expired swap
    ///
    /// Returns the txid of the refund transaction.
    pub async fn refund(&self, req: RefundRequest) -> SdkResult<RefundResponse> {
        self.get_services().await.refund(req).await
    }

    /// Register for webhook callbacks at the given `webhook_url`.
    ///
    /// More specifically, it registers for the following types of callbacks:
    /// - a payment is received
    /// - a swap tx is confirmed
    ///
    /// This method should be called every time the application is started and when the `webhook_url` changes.
    /// For example, if the `webhook_url` contains a push notification token and the token changes after
    /// the application was started, then this method should be called to register for callbacks at
    /// the new correct `webhook_url`. To unregister a webhook call [BreezServices::unregister_webhook].
    pub async fn register_webhook(&self, webhook_url: String) -> SdkResult<()> {
        self.get_services()
            .await
            .register_webhook(webhook_url)
            .await
    }

    /// Report an issue.
    ///
    /// Calling `report_issue` with a [ReportIssueRequest] enum param sends an issue report using the Support API.
    /// - [ReportIssueRequest::PaymentFailure] sends a payment failure report to the Support API
    ///   using the provided `payment_hash` to lookup the failed payment and the current [NodeState].
    pub async fn report_issue(&self, req: ReportIssueRequest) -> SdkResult<()> {
        self.get_services().await.report_issue(req).await
    }

    /// Iterate all historical swap addresses and fetch their current status from the blockchain.
    /// The status is then updated in the persistent storage.
    pub async fn rescan_swaps(&self) -> SdkResult<()> {
        self.get_services().await.rescan_swaps().await
    }

    /// Creates a reverse swap and attempts to pay the HODL invoice
    #[deprecated(note = "use pay_onchain instead")]
    pub async fn send_onchain(
        &self,
        req: SendOnchainRequest,
    ) -> Result<SendOnchainResponse, SendOnchainError> {
        #[allow(deprecated)]
        self.get_services().await.send_onchain(req).await
    }

    /// Pay a bolt11 invoice
    ///
    /// Calling `send_payment` ensures that the payment is not already completed, if so it will result in an error.
    /// If the invoice doesn't specify an amount, the amount is taken from the `amount_msat` arg.
    pub async fn send_payment(
        &self,
        req: SendPaymentRequest,
    ) -> Result<SendPaymentResponse, SendPaymentError> {
        self.get_services().await.send_payment(req).await
    }

    /// Pay directly to a node id using keysend
    pub async fn send_spontaneous_payment(
        &self,
        req: SendSpontaneousPaymentRequest,
    ) -> Result<SendPaymentResponse, SendPaymentError> {
        self.get_services()
            .await
            .send_spontaneous_payment(req)
            .await
    }

    /// Set the external metadata of a payment as a valid JSON string
    pub async fn set_payment_metadata(&self, hash: String, metadata: String) -> SdkResult<()> {
        self.get_services()
            .await
            .set_payment_metadata(hash, metadata)
            .await
    }

    /// Sign given message with the private key of the node id. Returns a zbase
    /// encoded signature.
    pub async fn sign_message(&self, req: SignMessageRequest) -> SdkResult<SignMessageResponse> {
        self.get_services().await.sign_message(req).await
    }

    /// This method sync the local state with the remote node state.
    /// The synced items are as follows:
    /// * node state - General information about the node and its liquidity status
    /// * channels - The list of channels and their status
    /// * payments - The incoming/outgoing payments
    pub async fn sync(&self) -> SdkResult<()> {
        self.get_services().await.sync().await
    }

    /// Unregister webhook callbacks for the given `webhook_url`.
    ///
    /// When called, it unregisters for the following types of callbacks:
    /// - a payment is received
    /// - a swap tx is confirmed
    ///
    /// This can be called when callbacks are no longer needed or the `webhook_url`
    /// has changed such that it needs unregistering. For example, the token is valid but the locale changes.
    /// To register a webhook call [BreezServices::register_webhook].
    pub async fn unregister_webhook(&self, webhook_url: String) -> SdkResult<()> {
        self.get_services()
            .await
            .unregister_webhook(webhook_url)
            .await
    }
}

struct GlobalSdkLogger {
    /// SDK internal logger, which logs to file
    logger: env_logger::Logger,
    /// Optional external log listener, that can receive a stream of log statements
    log_listener: Option<Box<dyn log::Log>>,
}
impl log::Log for GlobalSdkLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.logger.log(record);

            if let Some(s) = &self.log_listener.as_ref() {
                if s.enabled(record.metadata()) {
                    s.log(record);
                }
            }
        }
    }

    fn flush(&self) {}
}

/// Attempts to convert the phrase to a mnemonic, then to a seed.
///
/// If the phrase is not a valid mnemonic, an error is returned.
pub fn mnemonic_to_seed(phrase: String) -> anyhow::Result<Vec<u8>> {
    let mnemonic = Mnemonic::from_phrase(&phrase, Language::English)?;
    let seed = Seed::new(&mnemonic, "");
    Ok(seed.as_bytes().to_vec())
}
