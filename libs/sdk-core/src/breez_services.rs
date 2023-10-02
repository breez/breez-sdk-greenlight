use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, ensure, Result};
use bip39::*;
use bitcoin::hashes::hex::ToHex;
use bitcoin::hashes::{sha256, Hash};
use bitcoin::util::bip32::ChildNumber;
use chrono::Local;
use log::{LevelFilter, Metadata, Record};
use serde_json::json;
use tokio::sync::{mpsc, watch, Mutex};
use tokio::time::sleep;
use tonic::codegen::InterceptedService;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
use tonic::transport::{Channel, Uri};
use tonic::{Request, Status};

use crate::backup::{BackupRequest, BackupTransport, BackupWatcher};
use crate::boltzswap::BoltzApi;
use crate::chain::{ChainService, MempoolSpace, RecommendedFees};
use crate::error::{SdkError, SdkResult};
use crate::fiat::{FiatCurrency, Rate};
use crate::greenlight::{GLBackupTransport, Greenlight};
use crate::grpc::channel_opener_client::ChannelOpenerClient;
use crate::grpc::fund_manager_client::FundManagerClient;
use crate::grpc::information_client::InformationClient;
use crate::grpc::signer_client::SignerClient;
use crate::grpc::swapper_client::SwapperClient;
use crate::grpc::PaymentInformation;
use crate::input_parser::LnUrlPayRequestData;
use crate::invoice::{add_lsp_routing_hints, parse_invoice, LNInvoice, RouteHint, RouteHintHop};
use crate::lnurl::auth::perform_lnurl_auth;
use crate::lnurl::pay::model::SuccessAction::Aes;
use crate::lnurl::pay::model::{
    LnUrlPayResult, SuccessAction, SuccessActionProcessed, ValidatedCallbackResponse,
};
use crate::lnurl::pay::validate_lnurl_pay;
use crate::lnurl::withdraw::validate_lnurl_withdraw;
use crate::lsp::LspInformation;
use crate::models::{
    parse_short_channel_id, ChannelState, ClosedChannelPaymentDetails, Config, EnvironmentType,
    FiatAPI, LnUrlCallbackStatus, LspAPI, NodeAPI, NodeState, Payment, PaymentDetails, PaymentType,
    ReverseSwapPairInfo, ReverseSwapServiceAPI, SwapInfo, SwapperAPI,
    INVOICE_PAYMENT_FEE_EXPIRY_SECONDS,
};
use crate::moonpay::MoonPayApi;
use crate::persist::db::SqliteStorage;
use crate::reverseswap::BTCSendSwap;
use crate::swap::BTCReceiveSwap;
use crate::BuyBitcoinProvider::Moonpay;
use crate::*;

/// Trait that can be used to react to various [BreezEvent]s emitted by the SDK.
pub trait EventListener: Send + Sync {
    fn on_event(&self, e: BreezEvent);
}

/// Event emitted by the SDK. To listen for and react to these events, use an [EventListener] when
/// initializing the [BreezServices].
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum BreezEvent {
    /// Indicates that a new block has just been found
    NewBlock { block: u32 },
    /// Indicates that a new invoice has just been paid
    InvoicePaid { details: InvoicePaidDetails },
    /// Indicates that the local SDK state has just been sync-ed with the remote components
    Synced,
    /// Indicates that an outgoing payment has been completed successfully
    PaymentSucceed { details: Payment },
    /// Indicates that an outgoing payment has been failed to complete
    PaymentFailed { details: PaymentFailedData },
    /// Indicates that the backup process has just started
    BackupStarted,
    /// Indicates that the backup process has just finished successfully
    BackupSucceeded,
    /// Indicates that the backup process has just failed
    BackupFailed { details: BackupFailedData },
}

#[derive(Clone, Debug, PartialEq)]
pub struct BackupFailedData {
    pub error: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentFailedData {
    pub error: String,
    pub node_id: String,
    pub invoice: Option<LNInvoice>,
}

/// Details of an invoice that has been paid, included as payload in an emitted [BreezEvent]
#[derive(Clone, Debug, PartialEq)]
pub struct InvoicePaidDetails {
    pub payment_hash: String,
    pub bolt11: String,
}

pub trait LogStream: Send + Sync {
    fn log(&self, l: LogEntry);
}

/// Request to sign a message with the node's private key.
#[derive(Clone, Debug, PartialEq)]
pub struct SignMessageRequest {
    /// The message to be signed by the node's private key.
    pub message: String,
}

/// Response to a [SignMessageRequest].
#[derive(Clone, Debug, PartialEq)]
pub struct SignMessageResponse {
    /// The signature that covers the message of SignMessageRequest. Zbase
    /// encoded.
    pub signature: String,
}

/// Request to check a message was signed by a specific node id.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckMessageRequest {
    /// The message that was signed.
    pub message: String,
    /// The public key of the node that signed the message.
    pub pubkey: String,
    /// The zbase encoded signature to verify.
    pub signature: String,
}

/// Response to a [CheckMessageRequest]
#[derive(Clone, Debug, PartialEq)]
pub struct CheckMessageResponse {
    /// Boolean value indicating whether the signature covers the message and
    /// was signed by the given pubkey.
    pub is_valid: bool,
}

/// BreezServices is a facade and the single entry point for the SDK.
pub struct BreezServices {
    started: Mutex<bool>,
    node_api: Arc<dyn NodeAPI>,
    lsp_api: Arc<dyn LspAPI>,
    fiat_api: Arc<dyn FiatAPI>,
    moonpay_api: Arc<dyn MoonPayApi>,
    chain_service: Arc<dyn ChainService>,
    persister: Arc<SqliteStorage>,
    payment_receiver: Arc<PaymentReceiver>,
    btc_receive_swapper: Arc<BTCReceiveSwap>,
    btc_send_swapper: Arc<BTCSendSwap>,
    event_listener: Option<Box<dyn EventListener>>,
    backup_watcher: Arc<BackupWatcher>,
    shutdown_sender: watch::Sender<()>,
    shutdown_receiver: watch::Receiver<()>,
}

impl BreezServices {
    /// `connect` initializes the SDK services, schedules the node to run in the cloud and
    /// runs the signer. This must be called in order to start communicating with the node.
    ///
    /// # Arguments
    ///
    /// * `config` - The sdk configuration
    /// * `seed` - The node private key, typically derived from the mnemonic.
    /// When using a new `invite_code`, the seed should be derived from a new random mnemonic.
    /// When re-using an `invite_code`, the same mnemonic should be used as when the `invite_code` was first used.
    /// * `event_listener` - Listener to SDK events
    ///
    pub async fn connect(
        config: Config,
        seed: Vec<u8>,
        event_listener: Box<dyn EventListener>,
    ) -> SdkResult<Arc<BreezServices>> {
        let start = Instant::now();
        let services = BreezServicesBuilder::new(config)
            .seed(seed)
            .build(Some(event_listener))
            .await?;
        services.start().await?;
        let connect_duration = start.elapsed();
        info!("SDK connected in: {:?}", connect_duration);
        Ok(services)
    }

    /// Internal utility method that starts the BreezServices background tasks for this instance.
    ///
    /// It should be called once right after creating [BreezServices], since it is essential for the
    /// communicating with the node.
    ///
    /// It should be called only once when the app is started, regardless whether the app is sent to
    /// background and back.
    async fn start(self: &Arc<BreezServices>) -> SdkResult<()> {
        let mut started = self.started.lock().await;
        if *started {
            return Err(SdkError::InitFailed {
                err: "BreezServices already started".into(),
            });
        }
        let start = Instant::now();
        self.start_background_tasks().await?;
        let start_duration = start.elapsed();
        info!("SDK initialized in: {:?}", start_duration);
        *started = true;
        Ok(())
    }

    /// Trigger the stopping of BreezServices background threads for this instance.
    pub async fn disconnect(&self) -> Result<()> {
        let mut started = self.started.lock().await;
        if !*started {
            return Err(anyhow::Error::msg("BreezServices is not running"));
        }
        self.shutdown_sender.send(()).map_err(anyhow::Error::msg)?;
        *started = false;
        Ok(())
    }

    /// Pay a bolt11 invoice
    ///
    /// Calling `send_payment` ensures that the payment is not already completed, if so it will result in an error.
    /// If the invoice doesn't specify an amount, the amount is taken from the `amount_sats` arg.
    ///
    /// # Arguments
    ///
    /// * `bolt11` - The bolt11 invoice
    /// * `amount_sats` - The amount to pay in satoshis
    pub async fn send_payment(
        &self,
        bolt11: String,
        amount_sats: Option<u64>,
    ) -> SdkResult<Payment> {
        self.start_node().await?;
        let parsed_invoice = parse_invoice(bolt11.as_str())?;
        match self
            .persister
            .get_completed_payment_by_hash(&parsed_invoice.payment_hash)?
        {
            Some(_) => Err(SdkError::SendPaymentFailed {
                err: "Invoice already paid".into(),
            }),
            None => {
                let payment_res = self
                    .node_api
                    .send_payment(bolt11.clone(), amount_sats)
                    .await;
                self.on_payment_completed(
                    parsed_invoice.payee_pubkey.clone(),
                    Some(parsed_invoice),
                    payment_res,
                )
                .await
            }
        }
    }

    /// Pay directly to a node id using keysend
    ///
    /// # Arguments
    ///
    /// * `node_id` - The destination node_id
    /// * `amount_sats` - The amount to pay in satoshis
    pub async fn send_spontaneous_payment(
        &self,
        node_id: String,
        amount_sats: u64,
    ) -> SdkResult<Payment> {
        self.start_node().await?;
        let payment_res = self
            .node_api
            .send_spontaneous_payment(node_id.clone(), amount_sats)
            .await;
        self.on_payment_completed(node_id, None, payment_res).await
    }

    /// Second step of LNURL-pay. The first step is `parse()`, which also validates the LNURL destination
    /// and generates the `LnUrlPayRequestData` payload needed here.
    ///
    /// This call will validate the given `user_amount_sat` and `comment` against the parameters
    /// of the LNURL endpoint (`req_data`). If they match the endpoint requirements, the LNURL payment
    /// is made.
    ///
    /// This method will return an [anyhow::Error] when any validation check fails.
    pub async fn lnurl_pay(
        &self,
        user_amount_sat: u64,
        comment: Option<String>,
        req_data: LnUrlPayRequestData,
    ) -> Result<LnUrlPayResult> {
        match validate_lnurl_pay(user_amount_sat, comment, req_data.clone()).await? {
            ValidatedCallbackResponse::EndpointError { data: e } => {
                Ok(LnUrlPayResult::EndpointError { data: e })
            }
            ValidatedCallbackResponse::EndpointSuccess { data: cb } => {
                let payment = self.send_payment(cb.pr, None).await?;
                let details = match &payment.details {
                    PaymentDetails::ClosedChannel { .. } => {
                        return Err(anyhow!("Payment lookup found unexpected payment type"));
                    }
                    PaymentDetails::Ln { data } => data,
                };

                let maybe_sa_processed: Option<SuccessActionProcessed> = match cb.success_action {
                    Some(sa) => {
                        let processed_sa = match sa {
                            // For AES, we decrypt the contents on the fly
                            Aes(data) => {
                                let preimage = sha256::Hash::from_str(&details.payment_preimage)?;
                                let preimage_arr: [u8; 32] = preimage.into_inner();

                                let decrypted = (data, &preimage_arr).try_into()?;
                                SuccessActionProcessed::Aes { data: decrypted }
                            }
                            SuccessAction::Message(data) => {
                                SuccessActionProcessed::Message { data }
                            }
                            SuccessAction::Url(data) => SuccessActionProcessed::Url { data },
                        };
                        Some(processed_sa)
                    }
                    None => None,
                };

                // Store SA (if available) + LN Address in separate table, associated to payment_hash
                self.persister.insert_lnurl_payment_external_info(
                    &details.payment_hash,
                    maybe_sa_processed.as_ref(),
                    Some(req_data.metadata_str),
                    req_data.ln_address,
                    None,
                )?;

                Ok(LnUrlPayResult::EndpointSuccess {
                    data: maybe_sa_processed,
                })
            }
        }
    }

    /// Second step of LNURL-withdraw. The first step is `parse()`, which also validates the LNURL destination
    /// and generates the `LnUrlWithdrawRequestData` payload needed here.
    ///
    /// This call will validate the given `amount_sats` against the parameters
    /// of the LNURL endpoint (`req_data`). If they match the endpoint requirements, the LNURL withdraw
    /// request is made. A successful result here means the endpoint started the payment.
    pub async fn lnurl_withdraw(
        &self,
        req_data: LnUrlWithdrawRequestData,
        amount_sats: u64,
        description: Option<String>,
    ) -> Result<LnUrlWithdrawResult> {
        let invoice = self
            .receive_payment(ReceivePaymentRequest {
                amount_sats,
                description: description.unwrap_or_default(),
                preimage: None,
                opening_fee_params: None,
                use_description_hash: Some(false),
                expiry: None,
                cltv: None,
            })
            .await
            .map_err(|_| anyhow!("Failed to receive payment"))?
            .ln_invoice;

        let lnurl_w_endpoint = req_data.callback.clone();
        let res = validate_lnurl_withdraw(req_data, invoice).await?;

        if let LnUrlWithdrawResult::Ok { ref data } = res {
            // If endpoint was successfully called, store the LNURL-withdraw endpoint URL as metadata linked to the invoice
            self.persister.insert_lnurl_payment_external_info(
                &data.invoice.payment_hash,
                None,
                None,
                None,
                Some(lnurl_w_endpoint),
            )?;
        }

        Ok(res)
    }

    /// Third and last step of LNURL-auth. The first step is `parse()`, which also validates the LNURL destination
    /// and generates the `LnUrlAuthRequestData` payload needed here. The second step is user approval of auth action.
    ///
    /// This call will sign `k1` of the LNURL endpoint (`req_data`) on `secp256k1` using `linkingPrivKey` and DER-encodes the signature.
    /// If they match the endpoint requirements, the LNURL auth request is made. A successful result here means the client signature is verified.
    pub async fn lnurl_auth(&self, req_data: LnUrlAuthRequestData) -> Result<LnUrlCallbackStatus> {
        perform_lnurl_auth(self.node_api.clone(), req_data).await
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
    pub async fn receive_payment(
        &self,
        req_data: ReceivePaymentRequest,
    ) -> SdkResult<ReceivePaymentResponse> {
        self.payment_receiver.receive_payment(req_data).await
    }

    /// Retrieve the node state from the persistent storage.
    ///
    /// Fail if it could not be retrieved or if `None` was found.
    pub fn node_info(&self) -> SdkResult<NodeState> {
        self.persister
            .get_node_state()?
            .ok_or(SdkError::PersistenceFailure {
                err: "No node info found".into(),
            })
    }

    /// Sign given message with the private key of the node id. Returns a zbase
    /// encoded signature.
    pub async fn sign_message(&self, request: SignMessageRequest) -> Result<SignMessageResponse> {
        let signature = self.node_api.sign_message(&request.message).await?;
        Ok(SignMessageResponse { signature })
    }

    /// Check whether given message was signed by the private key or the given
    /// pubkey and the signature (zbase encoded) is valid.
    pub async fn check_message(
        &self,
        request: CheckMessageRequest,
    ) -> Result<CheckMessageResponse> {
        let is_valid = self
            .node_api
            .check_message(&request.message, &request.pubkey, &request.signature)
            .await?;
        Ok(CheckMessageResponse { is_valid })
    }

    /// Retrieve the node up to date BackupStatus
    pub fn backup_status(&self) -> Result<BackupStatus> {
        let backup_time = self.persister.get_last_backup_time()?;
        let sync_request = self.persister.get_last_sync_request()?;
        Ok(BackupStatus {
            last_backup_time: backup_time,
            backed_up: sync_request.is_none(),
        })
    }

    /// Force running backup
    pub async fn backup(&self) -> Result<()> {
        let (on_complete, mut on_complete_receiver) = mpsc::channel::<Result<()>>(1);
        let request = BackupRequest::with(on_complete, true);
        self.backup_watcher.request_backup(request).await?;

        match on_complete_receiver.recv().await {
            Some(res) => res,
            None => Err(anyhow!("backup process failed to complete")),
        }
    }

    /// List payments matching the given filters, as retrieved from persistent storage
    pub async fn list_payments(&self, request: ListPaymentsRequest) -> SdkResult<Vec<Payment>> {
        self.persister.list_payments(
            request.filter,
            request.from_timestamp,
            request.to_timestamp,
            request.include_failures,
        )
    }

    /// Fetch a specific payment by its hash.
    pub async fn payment_by_hash(&self, hash: String) -> Result<Option<Payment>> {
        self.persister
            .get_payment_by_hash(&hash)
            .map_err(|err| anyhow!(err))
    }

    /// Sweep on-chain funds to the specified on-chain address, with the given feerate
    pub async fn sweep(&self, request: SweepRequest) -> Result<SweepResponse> {
        self.start_node().await?;
        let txid = self
            .node_api
            .sweep(request.to_address, request.fee_rate_sats_per_vbyte)
            .await?;
        self.sync().await?;
        Ok(SweepResponse { txid })
    }

    /// Fetch live rates of fiat currencies
    pub async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>> {
        self.fiat_api.fetch_fiat_rates().await
    }

    /// List all supported fiat currencies for which there is a known exchange rate.
    pub async fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>> {
        self.fiat_api.list_fiat_currencies().await
    }

    /// List available LSPs that can be selected by the user
    pub async fn list_lsps(&self) -> SdkResult<Vec<LspInformation>> {
        self.lsp_api
            .list_lsps(self.node_info()?.id)
            .await
            .map_err(|e| SdkError::LspConnectFailed { err: e.to_string() })
    }

    /// Select the LSP to be used and provide inbound liquidity
    pub async fn connect_lsp(&self, lsp_id: String) -> SdkResult<()> {
        match self.list_lsps().await?.iter().any(|lsp| lsp.id == lsp_id) {
            true => {
                self.persister.set_lsp_id(lsp_id)?;
                self.sync().await?;
                Ok(())
            }
            false => Err(SdkError::LspConnectFailed {
                err: format!("Unknown LSP: {lsp_id}"),
            }),
        }
    }

    /// Get the current LSP's ID
    pub async fn lsp_id(&self) -> SdkResult<Option<String>> {
        self.persister.get_lsp_id()
    }

    /// Convenience method to look up [LspInformation] for a given LSP ID
    pub async fn fetch_lsp_info(&self, id: String) -> Result<Option<LspInformation>> {
        get_lsp_by_id(self.persister.clone(), self.lsp_api.clone(), id.as_str()).await
    }

    /// Gets the fees required to open a channel for a given amount.
    /// If there is no channel needed, returns 0.
    /// If there is a channel needed, returns the required open channel fees, with a fee params object
    /// to pass to methods that require a channel open, like receive_payment, or receive_onchain.
    pub async fn open_channel_fee(
        &self,
        req: OpenChannelFeeRequest,
    ) -> SdkResult<OpenChannelFeeResponse> {
        // get the node state to fetch the current inbound liquidity.
        let node_state = self.persister.get_node_state()?.ok_or(SdkError::NotReady {
            err: "Failed to read node state".to_string(),
        })?;

        // In case we have enough inbound liquidity we return zero fee.
        if node_state.inbound_liquidity_msats >= req.amount_msat {
            return Ok(OpenChannelFeeResponse {
                fee_msat: 0,
                used_fee_params: None,
            });
        }

        // Otherwise we need to calculate the fee for opening a new channel.
        let lsp_info = self.lsp_info().await?;
        let used_fee_params = lsp_info
            .cheapest_open_channel_fee(req.expiry.unwrap_or(INVOICE_PAYMENT_FEE_EXPIRY_SECONDS))?;
        let fee_msat = used_fee_params.get_channel_fees_msat_for(req.amount_msat);

        Ok(OpenChannelFeeResponse {
            fee_msat,
            used_fee_params: Some(used_fee_params.clone()),
        })
    }

    /// Close all channels with the current LSP.
    ///
    /// Should be called  when the user wants to close all the channels.
    pub async fn close_lsp_channels(&self) -> Result<Vec<String>> {
        self.start_node().await?;
        let lsp = self.lsp_info().await?;
        let tx_ids = self.node_api.close_peer_channels(lsp.pubkey).await?;
        self.sync().await?;
        Ok(tx_ids)
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
    pub async fn receive_onchain(&self, req: ReceiveOnchainRequest) -> Result<SwapInfo> {
        if let Some(in_progress) = self.in_progress_swap().await? {
            return Err(anyhow!(format!(
                  "Swap in progress was detected for address {}.Use in_progress_swap method to get the current swap state",
                  in_progress.bitcoin_address
              )));
        }
        let channel_opening_fees = match req.opening_fee_params {
            Some(fee_params) => fee_params,
            None => self
                .lsp_info()
                .await?
                .cheapest_open_channel_fee(SWAP_PAYMENT_FEE_EXPIRY_SECONDS)?
                .clone(),
        };

        let swap_info = self
            .btc_receive_swapper
            .create_swap_address(channel_opening_fees)
            .await?;
        Ok(swap_info)
    }

    /// Returns an optional in-progress [SwapInfo].
    /// A [SwapInfo] is in-progress if it is waiting for confirmation to be redeemed and complete the swap.
    pub async fn in_progress_swap(&self) -> Result<Option<SwapInfo>> {
        let tip = self.chain_service.current_tip().await?;
        self.btc_receive_swapper.execute_pending_swaps(tip).await?;
        let in_progress = self.btc_receive_swapper.list_in_progress().await?;
        if !in_progress.is_empty() {
            return Ok(Some(in_progress[0].clone()));
        }
        Ok(None)
    }

    /// Lookup the reverse swap fees (see [ReverseSwapServiceAPI::fetch_reverse_swap_fees]).
    ///
    /// To get the total estimated fees for a specific amount, specify the amount to be sent in
    /// `send_amount_sat`. The result will then contain the total estimated fees in
    /// [`ReverseSwapPairInfo::total_estimated_fees`].
    ///
    /// ### Errors
    ///
    /// If a `send_amount_sat` is specified in the request, but is outside the `min` and `max`,
    /// this will result in an error. If you are not sure what are the `min` and `max`, please call
    /// this with `send_amount_sat` as `None` first, then repeat the call with the desired amount.
    pub async fn fetch_reverse_swap_fees(
        &self,
        req: ReverseSwapFeesRequest,
    ) -> Result<ReverseSwapPairInfo> {
        let mut res = self.btc_send_swapper.fetch_reverse_swap_fees().await?;

        if let Some(send_amount_sat) = req.send_amount_sat {
            ensure!(send_amount_sat <= res.max, "Send amount is too high");
            ensure!(send_amount_sat >= res.min, "Send amount is too low");

            let service_fee_sat = ((send_amount_sat as f64) * res.fees_percentage / 100.0) as u64;
            res.total_estimated_fees = Some(service_fee_sat + res.fees_lockup + res.fees_claim);
        }

        Ok(res)
    }

    /// Creates a reverse swap and attempts to pay the HODL invoice
    pub async fn send_onchain(
        &self,
        amount_sat: u64,
        onchain_recipient_address: String,
        pair_hash: String,
        sat_per_vbyte: u64,
    ) -> Result<ReverseSwapInfo> {
        match self.in_progress_reverse_swaps().await?.is_empty() {
            true => self.btc_send_swapper
                .create_reverse_swap(
                    amount_sat,
                    onchain_recipient_address,
                    pair_hash,
                    sat_per_vbyte,
                )
                .await
                .map(Into::into),
            false => Err(anyhow!(
                "There already is at least one Reverse Swap in progress. You can only start a new one after after the ongoing ones finish. \
                Use the in_progress_reverse_swaps method to get an overview of currently ongoing reverse swaps."
            ))
        }
    }

    /// Returns the blocking [ReverseSwapInfo]s that are in progress
    pub async fn in_progress_reverse_swaps(&self) -> Result<Vec<ReverseSwapInfo>> {
        self.btc_send_swapper
            .list_blocking()
            .await
            .map(|x| x.into_iter().map(Into::into).collect())
    }

    /// list non-completed expired swaps that should be refunded by calling [BreezServices::refund]
    pub async fn list_refundables(&self) -> Result<Vec<SwapInfo>> {
        self.btc_receive_swapper.list_refundables()
    }

    /// Construct and broadcast a refund transaction for a failed/expired swap
    pub async fn refund(
        &self,
        swap_address: String,
        to_address: String,
        sat_per_vbyte: u32,
    ) -> Result<String> {
        self.btc_receive_swapper
            .refund_swap(swap_address, to_address, sat_per_vbyte)
            .await
    }

    /// Execute a command directly on the NodeAPI interface.
    /// Mainly used to debugging.
    pub async fn execute_dev_command(&self, command: String) -> Result<String> {
        self.node_api.execute_command(command).await
    }

    /// This method sync the local state with the remote node state.
    /// The synced items are as follows:
    /// * node state - General information about the node and its liquidity status
    /// * channels - The list of channels and their status
    /// * payments - The incoming/outgoing payments
    pub async fn sync(&self) -> Result<()> {
        self.do_sync(false).await
    }

    async fn do_sync(&self, balance_changed: bool) -> Result<()> {
        let start = Instant::now();
        self.start_node().await?;
        self.connect_lsp_peer().await?;

        // First query the changes since last sync time.
        let since_timestamp = self.persister.last_payment_timestamp().unwrap_or(0);
        let new_data = &self
            .node_api
            .pull_changed(since_timestamp, balance_changed)
            .await?;

        debug!(
            "pull changed time={:?} {:?}",
            since_timestamp, new_data.payments
        );

        // update node state and channels state
        self.persister.set_node_state(&new_data.node_state)?;

        let channels_before_update = self.persister.list_channels()?;
        self.persister.update_channels(&new_data.channels)?;
        let channels_after_update = self.persister.list_channels()?;

        // Fetch the static backup if needed and persist it
        if channels_before_update.len() != channels_after_update.len() {
            info!("fetching static backup file from node");
            let backup = self.node_api.static_backup().await?;
            self.persister.set_static_backup(backup)?;
        }

        //fetch closed_channel and convert them to Payment items.
        let mut closed_channel_payments: Vec<Payment> = vec![];
        for closed_channel in
            self.persister.list_channels()?.into_iter().filter(|c| {
                c.state == ChannelState::Closed || c.state == ChannelState::PendingClose
            })
        {
            let closed_channel_tx = self.closed_channel_to_transaction(closed_channel).await?;
            closed_channel_payments.push(closed_channel_tx);
        }

        // update both closed channels and lightning transaction payments
        let mut payments = closed_channel_payments;
        payments.extend(new_data.payments.clone());
        self.persister.insert_or_update_payments(&payments)?;

        let duration = start.elapsed();
        info!("Sync duration: {:?}", duration);

        self.notify_event_listeners(BreezEvent::Synced).await?;
        Ok(())
    }

    /// Connects to the selected LSP, if any
    async fn connect_lsp_peer(&self) -> Result<()> {
        let lsp = self.lsp_info().await.ok();
        if lsp.is_some() {
            let lsp_info = lsp.unwrap().clone();
            let node_id = lsp_info.pubkey;
            let address = lsp_info.host;
            debug!("connecting to lsp {}@{}", node_id.clone(), address.clone());
            self.node_api
                .connect_peer(node_id.clone(), address.clone())
                .await
                .map_err(anyhow::Error::msg)?;
            debug!("connected to lsp {}@{}", node_id.clone(), address.clone());
        }
        Ok(())
    }

    async fn on_payment_completed(
        &self,
        node_id: String,
        invoice: Option<LNInvoice>,
        payment_res: Result<PaymentResponse>,
    ) -> SdkResult<Payment> {
        self.do_sync(payment_res.is_ok()).await?;

        match payment_res {
            Ok(payment) => match self.persister.get_payment_by_hash(&payment.payment_hash)? {
                Some(p) => {
                    self.notify_event_listeners(BreezEvent::PaymentSucceed { details: p.clone() })
                        .await?;
                    Ok(p)
                }
                None => Err(SdkError::SendPaymentFailed {
                    err: "Payment not found".into(),
                }),
            },
            Err(e) => {
                self.notify_event_listeners(BreezEvent::PaymentFailed {
                    details: PaymentFailedData {
                        error: e.to_string(),
                        node_id,
                        invoice,
                    },
                })
                .await?;
                Err(SdkError::SendPaymentFailed { err: e.to_string() })
            }
        }
    }

    async fn on_event(&self, e: BreezEvent) -> Result<()> {
        debug!("breez services got event {:?}", e);
        self.notify_event_listeners(e.clone()).await
    }

    async fn notify_event_listeners(&self, e: BreezEvent) -> Result<()> {
        if let Err(err) = self.btc_receive_swapper.on_event(e.clone()).await {
            debug!(
                "btc_receive_swapper failed to process event {:?}: {:?}",
                e, err
            )
        };
        if let Err(err) = self.btc_send_swapper.on_event(e.clone()).await {
            debug!(
                "btc_send_swapper failed to process event {:?}: {:?}",
                e, err
            )
        };

        if self.event_listener.is_some() {
            self.event_listener.as_ref().unwrap().on_event(e.clone())
        }
        Ok(())
    }

    /// Convenience method to look up LSP info based on current LSP ID
    pub async fn lsp_info(&self) -> Result<LspInformation> {
        get_lsp(self.persister.clone(), self.lsp_api.clone()).await
    }

    pub(crate) async fn start_node(&self) -> Result<()> {
        self.node_api.start().await
    }

    /// Get the recommended fees for onchain transactions
    pub async fn recommended_fees(&self) -> Result<RecommendedFees> {
        self.chain_service.recommended_fees().await
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

    /// Get the static backup data from the peristent storage.
    /// This data enables the user to recover the node in an external core ligntning node.
    /// See here for instructions on how to recover using this data: https://docs.corelightning.org/docs/backup-and-recovery#backing-up-using-static-channel-backup
    pub fn static_backup(request: StaticBackupRequest) -> SdkResult<StaticBackupResponse> {
        let storage = SqliteStorage::new(request.working_dir);
        Ok(StaticBackupResponse {
            backup: storage.get_static_backup()?,
        })
    }

    /// Generates an url that can be used by a third part provider to buy Bitcoin with fiat currency.
    ///
    /// A user-selected [OpeningFeeParams] can be optionally set in the argument. If set, and the
    /// operation requires a new channel, the SDK will try to use the given fee params.
    pub async fn buy_bitcoin(&self, req: BuyBitcoinRequest) -> SdkResult<BuyBitcoinResponse> {
        let swap_info = self
            .receive_onchain(ReceiveOnchainRequest {
                opening_fee_params: req.opening_fee_params,
            })
            .await?;
        let url = match req.provider {
            Moonpay => self.moonpay_api.buy_bitcoin_url(&swap_info).await?,
        };

        Ok(BuyBitcoinResponse {
            url,
            opening_fee_params: swap_info.channel_opening_fees,
        })
    }

    /// Starts the BreezServices background threads.
    ///
    /// Internal method. Should only be used as part of [BreezServices::start]
    async fn start_background_tasks(self: &Arc<BreezServices>) -> Result<()> {
        // start the signer
        let (shutdown_signer_sender, signer_signer_receiver) = mpsc::channel(1);
        self.start_signer(signer_signer_receiver).await;

        // Sync node state
        let sync_breez_services = self.clone();
        match sync_breez_services.persister.get_node_state()? {
            Some(node) => {
                info!("Starting existing node {}", node.id)
            }
            None => {
                // In case it is a first run we sync in foreground to get the node state.
                info!("First run, syncing in foreground");
                sync_breez_services.sync().await?;
                info!("First run, finished running syncing in foreground");
            }
        }

        // start backup watcher
        self.start_backup_watcher().await?;

        //track backup events
        self.track_backup_events().await;

        // track paid invoices
        self.track_invoices().await;

        // track new blocks
        self.track_new_blocks().await;

        // track logs
        self.track_logs().await;

        // Stop signer on shutdown
        let mut shutdown_receiver = self.shutdown_receiver.clone();
        tokio::spawn(async move {
            // start the backup watcher
            _ = shutdown_receiver.changed().await;
            _ = shutdown_signer_sender.send(()).await;
            debug!("Received the signal to exit event polling loop");
        });

        Ok(())
    }

    async fn start_signer(self: &Arc<BreezServices>, shutdown_receiver: mpsc::Receiver<()>) {
        let signer_api = self.clone();
        tokio::spawn(async move {
            signer_api.node_api.start_signer(shutdown_receiver).await;
        });
    }

    async fn start_backup_watcher(self: &Arc<BreezServices>) -> SdkResult<()> {
        self.backup_watcher
            .start(self.shutdown_receiver.clone())
            .await
            .map_err(|e| SdkError::InitFailed {
                err: format!("Failed to start backup watcher: {e}"),
            })?;

        // Restore backup state and request backup on start if needed
        let force_backup = self
            .persister
            .get_last_sync_version()
            .map_err(|e| SdkError::InitFailed {
                err: format!("Failed to read last sync version: {e}"),
            })?
            .is_none();
        self.backup_watcher
            .request_backup(BackupRequest::new(force_backup))
            .await
            .map_err(|e| SdkError::InitFailed {
                err: format!("Failed to request backup: {e}"),
            })
    }

    async fn track_backup_events(self: &Arc<BreezServices>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut events_stream = cloned.backup_watcher.subscribe_events();
            let mut shutdown_receiver = cloned.shutdown_receiver.clone();
            loop {
                tokio::select! {
                  backup_event = events_stream.recv() => {
                   if let Ok(e) = backup_event {
                    if let Err(err) = cloned.notify_event_listeners(e).await {
                        error!("error handling backup event: {:?}", err);
                    }
                   }
                   let backup_status = cloned.backup_status();
                   info!("backup status: {:?}", backup_status);
                  },
                  _ = shutdown_receiver.changed() => {
                   debug!("Backup watcher task completed");
                   break;
                 }
                }
            }
        });
    }

    async fn track_invoices(self: &Arc<BreezServices>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut shutdown_receiver = cloned.shutdown_receiver.clone();
            loop {
                if shutdown_receiver.has_changed().map_or(true, |c| c) {
                    return;
                }
                let invoice_stream_res = cloned.node_api.stream_incoming_payments().await;
                if let Ok(mut invoice_stream) = invoice_stream_res {
                    loop {
                        tokio::select! {
                                paid_invoice_res = invoice_stream.message() => {
                                      match paid_invoice_res {
                                          Ok(Some(i)) => {
                                              debug!("invoice stream got new invoice");
                                              if let Some(gl_client::pb::incoming_payment::Details::Offchain(p)) = i.details {
                                                  let payment: Option<crate::models::Payment> = p.clone().try_into().ok();
                                                  if payment.is_some() {
                                                      let res = cloned
                                                          .persister
                                                          .insert_or_update_payments(&vec![payment.unwrap()]);
                                                      debug!("paid invoice was added to payments list {:?}", res);
                                                  }
                                                  if let Err(e) = cloned.do_sync(true).await {
                                                        error!("failed to sync after paid invoice: {:?}", e);
                                                  }
                                                  _ = cloned.on_event(BreezEvent::InvoicePaid {
                                                      details: InvoicePaidDetails {
                                                          payment_hash: hex::encode(p.payment_hash),
                                                          bolt11: p.bolt11,
                                                      },
                                                  }).await;
                                              }
                                          }
                                          Ok(None) => {
                                              debug!("invoice stream got None");
                                              break;
                                          }
                                          Err(err) => {
                                              debug!("invoice stream got error: {:?}", err);
                                              break;
                                          }
                                      }
                             }

                             _ = shutdown_receiver.changed() => {
                              debug!("Invoice tracking task has completed");
                              return;
                             }
                        }
                    }
                }
                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    async fn track_logs(self: &Arc<BreezServices>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut shutdown_receiver = cloned.shutdown_receiver.clone();
            loop {
                if shutdown_receiver.has_changed().map_or(true, |c| c) {
                    return;
                }
                let log_stream_res = cloned.node_api.stream_log_messages().await;
                if let Ok(mut log_stream) = log_stream_res {
                    loop {
                        tokio::select! {
                         log_message_res = log_stream.message() => {
                          match log_message_res {
                           Ok(Some(l)) => {
                            debug!("node-logs: {}", l.line);
                           },
                           // stream is closed, renew it
                           Ok(None) => {
                            break;
                           }
                           Err(err) => {
                            debug!("failed to process log entry {:?}", err);
                            break;
                           }
                          };
                         }

                         _ = shutdown_receiver.changed() => {
                          debug!("Track logs task has completed");
                          return;
                         }
                        }
                    }
                }
                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    async fn track_new_blocks(self: &Arc<BreezServices>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut current_block: u32 = 0;
            let mut shutdown_receiver = cloned.shutdown_receiver.clone();
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                tokio::select! {
                 _ = interval.tick() => {
                  let tip_res = cloned.chain_service.current_tip().await;
                  match tip_res {
                   Ok(next_block) => {
                    debug!("got tip {:?}", next_block);
                    if next_block > current_block {
                     _ = cloned.sync().await;
                     _  = cloned.on_event(BreezEvent::NewBlock{block: next_block}).await;
                    }
                    current_block = next_block
                   },
                   Err(e) => {
                    error!("failed to fetch next block {}", e)
                   }
                  };
                 }

                 _ = shutdown_receiver.changed() => {
                  debug!("New blocks task has completed");
                  return;
                 }
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
    pub fn init_logging(log_dir: &str, app_logger: Option<Box<dyn log::Log>>) -> SdkResult<()> {
        let target_log_file = Box::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(format!("{log_dir}/sdk.log"))
                .map_err(|_| SdkError::InitFailed {
                    err: "Can't create log file".into(),
                })?,
        );
        let logger = env_logger::Builder::new()
            .target(env_logger::Target::Pipe(target_log_file))
            .parse_filters(
                r#"
                info,
                breez_sdk_core::input_parser=warn,
                breez_sdk_core::backup=info,
                breez_sdk_core::persist::reverseswap=info,
                breez_sdk_core::reverseswap=info,
                gl_client=warn,
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

        log::set_boxed_logger(Box::new(global_logger)).map_err(|e| SdkError::InitFailed {
            err: format!("Failed to set global logger: {e}"),
        })?;
        log::set_max_level(LevelFilter::Trace);

        Ok(())
    }

    async fn closed_channel_to_transaction(
        &self,
        channel: crate::models::Channel,
    ) -> Result<Payment> {
        let now_epoch_sec = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        // If we don't have it, we look it up from the channel closing tx
        let channel_closed_at = channel.closed_at.unwrap_or(
            match channel.funding_outnum {
                None => {
                    warn!("No founding_outnum found for the closing tx, defaulting closed_at to epoch time");
                    now_epoch_sec
                }
                Some(outnum) => {
                    // Find the output tx that was used to fund the channel
                    let outspends = self
                        .chain_service
                        .transaction_outspends(channel.funding_txid.clone())
                        .await?;
                    let maybe_block_time = outspends.get(outnum as usize)
                        .and_then(|outspend| outspend.status.as_ref())
                        .and_then(|status| status.block_time);

                    match maybe_block_time {
                        None => {
                            warn!("Blocktime could not be determined for funding_outnum {outnum}, defaulting closed_at to epoch time");
                            now_epoch_sec
                        }
                        Some(block_time) => block_time
                    }
                }
            }
        ) as i64;

        Ok(Payment {
            id: channel.funding_txid.clone(),
            payment_type: PaymentType::ClosedChannel,
            payment_time: channel_closed_at,
            amount_msat: channel.spendable_msat,
            fee_msat: 0,
            status: match channel.state {
                ChannelState::PendingClose => PaymentStatus::Pending,
                _ => PaymentStatus::Complete,
            },
            description: Some("Closed Channel".to_string()),
            details: PaymentDetails::ClosedChannel {
                data: ClosedChannelPaymentDetails {
                    short_channel_id: channel.short_channel_id,
                    state: channel.state,
                    funding_txid: channel.funding_txid,
                },
            },
        })
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

/// A helper struct to configure and build BreezServices
struct BreezServicesBuilder {
    config: Config,
    node_api: Option<Arc<dyn NodeAPI>>,
    backup_transport: Option<Arc<dyn BackupTransport>>,
    seed: Option<Vec<u8>>,
    lsp_api: Option<Arc<dyn LspAPI>>,
    fiat_api: Option<Arc<dyn FiatAPI>>,
    persister: Option<Arc<SqliteStorage>>,
    swapper_api: Option<Arc<dyn SwapperAPI>>,
    /// Reverse swap functionality on the Breez Server
    reverse_swapper_api: Option<Arc<dyn ReverseSwapperRoutingAPI>>,
    /// Reverse swap functionality on the 3rd party reverse swap service
    reverse_swap_service_api: Option<Arc<dyn ReverseSwapServiceAPI>>,
    moonpay_api: Option<Arc<dyn MoonPayApi>>,
}

#[allow(dead_code)]
impl BreezServicesBuilder {
    pub fn new(config: Config) -> BreezServicesBuilder {
        BreezServicesBuilder {
            config,
            node_api: None,
            seed: None,
            lsp_api: None,
            fiat_api: None,
            persister: None,
            swapper_api: None,
            reverse_swapper_api: None,
            reverse_swap_service_api: None,
            moonpay_api: None,
            backup_transport: None,
        }
    }

    pub fn node_api(&mut self, node_api: Arc<dyn NodeAPI>) -> &mut Self {
        self.node_api = Some(node_api);
        self
    }

    pub fn lsp_api(&mut self, lsp_api: Arc<dyn LspAPI>) -> &mut Self {
        self.lsp_api = Some(lsp_api.clone());
        self
    }

    pub fn fiat_api(&mut self, fiat_api: Arc<dyn FiatAPI>) -> &mut Self {
        self.fiat_api = Some(fiat_api.clone());
        self
    }

    pub fn moonpay_api(&mut self, moonpay_api: Arc<dyn MoonPayApi>) -> &mut Self {
        self.moonpay_api = Some(moonpay_api.clone());
        self
    }

    pub fn persister(&mut self, persister: Arc<SqliteStorage>) -> &mut Self {
        self.persister = Some(persister);
        self
    }

    pub fn swapper_api(&mut self, swapper_api: Arc<dyn SwapperAPI>) -> &mut Self {
        self.swapper_api = Some(swapper_api.clone());
        self
    }

    pub fn reverse_swapper_api(
        &mut self,
        reverse_swapper_api: Arc<dyn ReverseSwapperRoutingAPI>,
    ) -> &mut Self {
        self.reverse_swapper_api = Some(reverse_swapper_api.clone());
        self
    }

    pub fn reverse_swap_service_api(
        &mut self,
        reverse_swap_service_api: Arc<dyn ReverseSwapServiceAPI>,
    ) -> &mut Self {
        self.reverse_swap_service_api = Some(reverse_swap_service_api.clone());
        self
    }

    pub fn backup_transport(&mut self, backup_transport: Arc<dyn BackupTransport>) -> &mut Self {
        self.backup_transport = Some(backup_transport.clone());
        self
    }

    pub fn seed(&mut self, seed: Vec<u8>) -> &mut Self {
        self.seed = Some(seed);
        self
    }

    pub async fn build(
        &self,
        event_listener: Option<Box<dyn EventListener>>,
    ) -> SdkResult<Arc<BreezServices>> {
        if self.node_api.is_none() && self.seed.is_none() {
            return Err(SdkError::InitFailed {
                err: "Either node_api or both credentials and seed should be provided".into(),
            });
        }

        // The storage is implemented via sqlite.
        let persister = self
            .persister
            .clone()
            .unwrap_or_else(|| Arc::new(SqliteStorage::new(self.config.working_dir.clone())));
        persister.init()?;

        let mut node_api = self.node_api.clone();
        let mut backup_transport = self.backup_transport.clone();
        if node_api.is_none() {
            let greenlight = Greenlight::connect(
                self.config.clone(),
                self.seed.clone().unwrap(),
                persister.clone(),
            )
            .await
            .map_err(|e| SdkError::InitFailed {
                err: format!("Failed to connect to Greenlight: {e}"),
            })?;
            let gl_arc = Arc::new(greenlight);
            node_api = Some(gl_arc.clone());
            if backup_transport.is_none() {
                backup_transport = Some(Arc::new(GLBackupTransport { inner: gl_arc }));
            }
        }

        if backup_transport.is_none() {
            return Err(SdkError::InitFailed {
                err: "State synchronizer should be provided".into(),
            });
        }

        let unwrapped_node_api = node_api.unwrap();
        let unwrapped_backup_transport = backup_transport.unwrap();

        // create the backup encryption key and then the backup watcher
        let backup_encryption_key = unwrapped_node_api
            .derive_bip32_key(vec![
                ChildNumber::from_hardened_idx(139).map_err(|e| SdkError::InitFailed {
                    err: format!(
                        "Failed to get necessary child number to derive backup encryption key: {e}"
                    ),
                })?,
                ChildNumber::from(0),
            ])
            .map_err(|e| SdkError::InitFailed {
                err: format!("Failed to derive backup encryption key: {e}"),
            })?;

        // We calculate the legacy key as a fallback for the case where the backup is still
        // encrypted with the old key.
        let legacy_backup_encryption_key = unwrapped_node_api
            .legacy_derive_bip32_key(vec![
                ChildNumber::from_hardened_idx(139).map_err(|e| SdkError::InitFailed {
                    err: format!(
                        "Failed to get necessary child number to derive backup encryption key: {e}"
                    ),
                })?,
                ChildNumber::from(0),
            ])
            .map_err(|e| SdkError::InitFailed {
                err: format!("Failed to derive backup encryption key: {e}"),
            })?;
        let backup_watcher = BackupWatcher::new(
            self.config.clone(),
            unwrapped_backup_transport.clone(),
            persister.clone(),
            backup_encryption_key.to_priv().to_bytes(),
            legacy_backup_encryption_key.to_priv().to_bytes(),
        );

        // breez_server provides both FiatAPI & LspAPI implementations
        let breez_server = Arc::new(BreezServer::new(
            self.config.breezserver.clone(),
            self.config.api_key.clone(),
        ));

        // mempool space is used to monitor the chain
        let chain_service = Arc::new(MempoolSpace::from_base_url(
            self.config.mempoolspace_url.clone(),
        ));

        let current_lsp_id = persister.get_lsp_id()?;
        if current_lsp_id.is_none() && self.config.default_lsp_id.is_some() {
            persister.set_lsp_id(self.config.default_lsp_id.clone().unwrap())?;
        }

        let payment_receiver = Arc::new(PaymentReceiver {
            config: self.config.clone(),
            node_api: unwrapped_node_api.clone(),
            lsp: breez_server.clone(),
            persister: persister.clone(),
        });

        let btc_receive_swapper = Arc::new(BTCReceiveSwap::new(
            self.config.network.into(),
            self.swapper_api
                .clone()
                .unwrap_or_else(|| breez_server.clone()),
            persister.clone(),
            chain_service.clone(),
            payment_receiver.clone(),
        ));

        let btc_send_swapper = Arc::new(BTCSendSwap::new(
            self.config.clone(),
            self.reverse_swapper_api
                .clone()
                .unwrap_or_else(|| breez_server.clone()),
            self.reverse_swap_service_api
                .clone()
                .unwrap_or_else(|| Arc::new(BoltzApi {})),
            persister.clone(),
            chain_service.clone(),
            unwrapped_node_api.clone(),
        ));

        // create a shutdown channel (sender and receiver)
        let (shutdown_sender, shutdown_receiver) = watch::channel::<()>(());

        // Create the node services and it them statically
        let breez_services = Arc::new(BreezServices {
            started: Mutex::new(false),
            node_api: unwrapped_node_api.clone(),
            lsp_api: self.lsp_api.clone().unwrap_or_else(|| breez_server.clone()),
            fiat_api: self
                .fiat_api
                .clone()
                .unwrap_or_else(|| breez_server.clone()),
            moonpay_api: self
                .moonpay_api
                .clone()
                .unwrap_or_else(|| breez_server.clone()),
            chain_service,
            persister: persister.clone(),
            btc_receive_swapper,
            btc_send_swapper,
            payment_receiver,
            event_listener,
            backup_watcher: Arc::new(backup_watcher),
            shutdown_sender,
            shutdown_receiver,
        });

        Ok(breez_services)
    }
}

#[derive(Clone)]
pub struct BreezServer {
    server_url: String,
    api_key: Option<String>,
}

impl BreezServer {
    pub fn new(server_url: String, api_key: Option<String>) -> Self {
        Self {
            server_url,
            api_key,
        }
    }

    pub(crate) async fn get_channel_opener_client(
        &self,
    ) -> Result<ChannelOpenerClient<InterceptedService<Channel, ApiKeyInterceptor>>> {
        let s = self.server_url.clone();
        let channel = Channel::from_shared(s)?.connect().await?;

        let api_key_metadata: Option<MetadataValue<Ascii>> = match &self.api_key {
            Some(key) => Some(format!("Bearer {key}").parse()?),
            _ => None,
        };
        let client =
            ChannelOpenerClient::with_interceptor(channel, ApiKeyInterceptor { api_key_metadata });
        Ok(client)
    }

    pub(crate) async fn get_information_client(&self) -> Result<InformationClient<Channel>> {
        InformationClient::connect(Uri::from_str(&self.server_url)?)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub(crate) async fn get_fund_manager_client(&self) -> Result<FundManagerClient<Channel>> {
        FundManagerClient::connect(Uri::from_str(&self.server_url)?)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub(crate) async fn get_signer_client(&self) -> Result<SignerClient<Channel>> {
        Ok(SignerClient::new(
            tonic::transport::Endpoint::new(Uri::from_str(&self.server_url)?)?
                .connect()
                .await?,
        ))
    }

    pub(crate) async fn get_swapper_client(&self) -> Result<SwapperClient<Channel>> {
        Ok(SwapperClient::new(
            tonic::transport::Endpoint::new(Uri::from_str(&self.server_url)?)?
                .connect()
                .await?,
        ))
    }
}

pub(crate) struct ApiKeyInterceptor {
    api_key_metadata: Option<MetadataValue<Ascii>>,
}

impl Interceptor for ApiKeyInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        if self.api_key_metadata.clone().is_some() {
            req.metadata_mut()
                .insert("authorization", self.api_key_metadata.clone().unwrap());
        }
        Ok(req)
    }
}

/// Attempts to convert the phrase to a mnemonic, then to a seed.
///
/// If the phrase is not a valid mnemonic, an error is returned.
pub fn mnemonic_to_seed(phrase: String) -> Result<Vec<u8>> {
    let mnemonic = Mnemonic::from_phrase(&phrase, Language::English)?;
    let seed = Seed::new(&mnemonic, "");
    Ok(seed.as_bytes().to_vec())
}

#[tonic::async_trait]
pub trait Receiver: Send + Sync {
    async fn receive_payment(
        &self,
        req_data: ReceivePaymentRequest,
    ) -> SdkResult<ReceivePaymentResponse>;
}

pub(crate) struct PaymentReceiver {
    config: Config,
    node_api: Arc<dyn NodeAPI>,
    lsp: Arc<dyn LspAPI>,
    persister: Arc<SqliteStorage>,
}

#[tonic::async_trait]
impl Receiver for PaymentReceiver {
    async fn receive_payment(
        &self,
        req_data: ReceivePaymentRequest,
    ) -> SdkResult<ReceivePaymentResponse> {
        self.node_api.start().await?;
        let lsp_info = get_lsp(self.persister.clone(), self.lsp.clone()).await?;
        let node_state = self
            .persister
            .get_node_state()?
            .ok_or("Failed to retrieve node state")
            .map_err(|err| anyhow!(err))?;
        let expiry = req_data
            .expiry
            .unwrap_or(INVOICE_PAYMENT_FEE_EXPIRY_SECONDS);

        let amount_sats = req_data.amount_sats;
        let amount_msats = amount_sats * 1000;

        let mut short_channel_id = parse_short_channel_id("1x0x0")?;
        let mut destination_invoice_amount_sats = amount_sats;

        let mut channel_opening_fee_params = None;
        let mut channel_fees_msat = None;

        // check if we need to open channel
        let open_channel_needed = node_state.inbound_liquidity_msats < amount_msats;
        if open_channel_needed {
            info!("We need to open a channel");

            // we need to open channel so we are calculating the fees for the LSP (coming either from the user, or from the LSP)
            let ofp = match req_data.opening_fee_params {
                Some(fee_params) => fee_params,
                None => lsp_info.cheapest_open_channel_fee(expiry)?.clone(),
            };

            channel_opening_fee_params = Some(ofp.clone());
            channel_fees_msat = Some(ofp.get_channel_fees_msat_for(amount_msats));
            if let Some(channel_fees_msat) = channel_fees_msat {
                info!("zero-conf fee calculation option: lsp fee rate (proportional): {}:  (minimum {}), total fees for channel: {}",
                    ofp.proportional, ofp.min_msat, channel_fees_msat);

                if amount_msats < channel_fees_msat + 1000 {
                    return Err(SdkError::ReceivePaymentFailed {
                        err: format!(
                            "requestPayment: Amount should be more than the minimum fees {channel_fees_msat} msat, but is {amount_msats} msat"
                        ),
                    });
                }
                // remove the fees from the amount to get the small amount on the current node invoice.
                destination_invoice_amount_sats = (amount_msats - channel_fees_msat) / 1000;
            }
        } else {
            // not opening a channel so we need to get the real channel id into the routing hints
            info!("Finding channel ID for routing hint");
            for peer in self.node_api.list_peers().await? {
                if hex::encode(peer.id) == lsp_info.pubkey && !peer.channels.is_empty() {
                    let active_channel = peer
                        .channels
                        .iter()
                        .find(|&c| c.state == ChannelState::Opened)
                        .ok_or_else(|| SdkError::ReceivePaymentFailed {
                            err: "No open channel found".into(),
                        })?;
                    let hint = active_channel
                        .clone()
                        .alias_remote
                        .unwrap_or(active_channel.clone().short_channel_id);

                    short_channel_id = parse_short_channel_id(&hint)?;
                    info!("Found channel ID: {short_channel_id} {active_channel:?}");
                    break;
                }
            }
        }

        info!("Creating invoice on NodeAPI");
        let invoice = &self
            .node_api
            .create_invoice(
                destination_invoice_amount_sats,
                req_data.description,
                req_data.preimage,
                req_data.use_description_hash,
                Some(expiry),
                req_data.cltv,
            )
            .await?;
        info!("Invoice created {}", invoice);

        let mut parsed_invoice = parse_invoice(invoice)?;

        // check if the lsp hint already exists
        info!("Existing routing hints {:?}", parsed_invoice.routing_hints);
        info!("lsp info pubkey = {:?}", lsp_info.pubkey.clone());
        let has_lsp_hint = parsed_invoice.routing_hints.iter().any(|h| {
            h.hops
                .iter()
                .any(|h| h.src_node_id == lsp_info.pubkey.clone())
        });

        // We only add routing hint if we need to open a channel
        // or if the invoice doesn't have any routing hints that points to the lsp
        let mut lsp_hint: Option<RouteHint> = None;
        if !has_lsp_hint || open_channel_needed {
            let lsp_hop = RouteHintHop {
                src_node_id: lsp_info.pubkey,
                short_channel_id,
                fees_base_msat: lsp_info.base_fee_msat as u32,
                fees_proportional_millionths: (lsp_info.fee_rate * 1000000.0) as u32,
                cltv_expiry_delta: lsp_info.time_lock_delta as u64,
                htlc_minimum_msat: Some(lsp_info.min_htlc_msat as u64),
                htlc_maximum_msat: None,
            };

            info!("Adding LSP hop as routing hint: {:?}", lsp_hop);
            lsp_hint = Some(RouteHint {
                hops: vec![lsp_hop],
            });
        }

        // We only create a new invoice if we need to add the lsp hint or change the amount
        if lsp_hint.is_some() || amount_sats != destination_invoice_amount_sats {
            // create the large amount invoice
            let raw_invoice_with_hint =
                add_lsp_routing_hints(invoice.clone(), lsp_hint, amount_sats * 1000)?;

            info!("Routing hint added");
            let signed_invoice_with_hint = self.node_api.sign_invoice(raw_invoice_with_hint)?;
            info!("Signed invoice with hint = {}", signed_invoice_with_hint);

            parsed_invoice = parse_invoice(&signed_invoice_with_hint)?;
        }

        // register the payment at the lsp if needed
        if open_channel_needed {
            info!("Registering payment with LSP");

            if channel_opening_fee_params.is_none() {
                return Err(SdkError::ReceivePaymentFailed {
                    err: "We need to open a channel, but no channel opening fee params found"
                        .into(),
                });
            }

            let api_key = self.config.api_key.clone().unwrap_or_default();
            let api_key_hash = sha256::Hash::hash(api_key.as_bytes()).to_hex();

            self.lsp
                .register_payment(
                    lsp_info.id.clone(),
                    lsp_info.lsp_pubkey.clone(),
                    PaymentInformation {
                        payment_hash: hex::decode(parsed_invoice.payment_hash.clone()).map_err(
                            |e| SdkError::ReceivePaymentFailed {
                                err: format!("Failed to decode hex payment hash: {e}"),
                            },
                        )?,
                        payment_secret: parsed_invoice.payment_secret.clone(),
                        destination: hex::decode(parsed_invoice.payee_pubkey.clone()).map_err(
                            |e| SdkError::ReceivePaymentFailed {
                                err: format!("Failed to decode hex payee pubkey: {e}"),
                            },
                        )?,
                        incoming_amount_msat: amount_msats as i64,
                        outgoing_amount_msat: (destination_invoice_amount_sats * 1000) as i64,
                        tag: json!({ "apiKeyHash": api_key_hash }).to_string(),
                        opening_fee_params: channel_opening_fee_params.clone().map(Into::into),
                    },
                )
                .await?;
            info!("Payment registered");
        }

        // Make sure we save the large amount so we can deduce the fees later.
        self.persister
            .insert_open_channel_payment_info(&parsed_invoice.payment_hash, amount_sats * 1000)?;
        // return the signed, converted invoice with hints
        Ok(ReceivePaymentResponse {
            ln_invoice: parsed_invoice,
            opening_fee_params: channel_opening_fee_params,
            opening_fee_msat: channel_fees_msat,
        })
    }
}

/// Convenience method to look up LSP info based on current LSP ID
async fn get_lsp(persister: Arc<SqliteStorage>, lsp: Arc<dyn LspAPI>) -> Result<LspInformation> {
    let lsp_id = persister
        .get_lsp_id()?
        .ok_or("No LSP ID found")
        .map_err(|err| anyhow!(err))?;

    get_lsp_by_id(persister, lsp, lsp_id.as_str())
        .await?
        .ok_or_else(|| anyhow!("No LSP found for id {}", lsp_id))
}

async fn get_lsp_by_id(
    persister: Arc<SqliteStorage>,
    lsp: Arc<dyn LspAPI>,
    lsp_id: &str,
) -> Result<Option<LspInformation>> {
    let node_pubkey = persister
        .get_node_state()?
        .ok_or("No NodeState found")
        .map_err(|err| anyhow!(err))?
        .id;

    Ok(lsp
        .list_lsps(node_pubkey)
        .await?
        .iter()
        .find(|&lsp| lsp.id.as_str() == lsp_id)
        .cloned())
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use anyhow::Result;
    use regex::Regex;
    use reqwest::Url;

    use crate::breez_services::{BreezServices, BreezServicesBuilder};
    use crate::error::{SdkError, SdkResult};
    use crate::fiat::Rate;
    use crate::lnurl::pay::model::MessageSuccessActionData;
    use crate::lnurl::pay::model::SuccessActionProcessed;
    use crate::models::{LnPaymentDetails, NodeState, Payment, PaymentDetails, PaymentTypeFilter};
    use crate::{
        input_parser, parse_short_channel_id, test_utils::*, BuyBitcoinProvider, BuyBitcoinRequest,
        InputType, ListPaymentsRequest, PaymentStatus, ReceivePaymentRequest,
    };
    use crate::{NodeAPI, PaymentType};

    use super::{PaymentReceiver, Receiver};

    #[tokio::test]
    async fn test_node_state() -> SdkResult<()> {
        // let storage_path = format!("{}/storage.sql", get_test_working_dir());
        // std::fs::remove_file(storage_path).ok();

        let dummy_node_state = get_dummy_node_state();

        let lnurl_metadata = "{'key': 'sample-metadata-val'}";
        let test_ln_address = "test@ln-address.com";
        let test_lnurl_withdraw_endpoint = "https://test.endpoint.lnurl-w";
        let sa = SuccessActionProcessed::Message {
            data: MessageSuccessActionData {
                message: "test message".into(),
            },
        };

        let payment_hash_lnurl_withdraw = "2222";
        let payment_hash_with_lnurl_success_action = "3333";
        let dummy_transactions = vec![
            Payment {
                id: "1111".to_string(),
                payment_type: PaymentType::Received,
                payment_time: 100000,
                amount_msat: 10,
                fee_msat: 0,
                status: PaymentStatus::Complete,
                description: Some("test receive".to_string()),
                details: PaymentDetails::Ln {
                    data: LnPaymentDetails {
                        payment_hash: "1111".to_string(),
                        label: "".to_string(),
                        destination_pubkey: "1111".to_string(),
                        payment_preimage: "2222".to_string(),
                        keysend: false,
                        bolt11: "1111".to_string(),
                        lnurl_success_action: None,
                        lnurl_metadata: None,
                        ln_address: None,
                        lnurl_withdraw_endpoint: None,
                    },
                },
            },
            Payment {
                id: payment_hash_lnurl_withdraw.to_string(),
                payment_type: PaymentType::Received,
                payment_time: 150000,
                amount_msat: 10,
                fee_msat: 0,
                status: PaymentStatus::Complete,
                description: Some("test lnurl-withdraw receive".to_string()),
                details: PaymentDetails::Ln {
                    data: LnPaymentDetails {
                        payment_hash: payment_hash_lnurl_withdraw.to_string(),
                        label: "".to_string(),
                        destination_pubkey: "1111".to_string(),
                        payment_preimage: "2222".to_string(),
                        keysend: false,
                        bolt11: "1111".to_string(),
                        lnurl_success_action: None,
                        lnurl_metadata: None,
                        ln_address: None,
                        lnurl_withdraw_endpoint: Some(test_lnurl_withdraw_endpoint.to_string()),
                    },
                },
            },
            Payment {
                id: payment_hash_with_lnurl_success_action.to_string(),
                payment_type: PaymentType::Sent,
                payment_time: 200000,
                amount_msat: 8,
                fee_msat: 2,
                status: PaymentStatus::Complete,
                description: Some("test payment".to_string()),
                details: PaymentDetails::Ln {
                    data: LnPaymentDetails {
                        payment_hash: payment_hash_with_lnurl_success_action.to_string(),
                        label: "".to_string(),
                        destination_pubkey: "123".to_string(),
                        payment_preimage: "4444".to_string(),
                        keysend: false,
                        bolt11: "123".to_string(),
                        lnurl_success_action: Some(sa.clone()),
                        lnurl_metadata: Some(lnurl_metadata.to_string()),
                        ln_address: Some(test_ln_address.to_string()),
                        lnurl_withdraw_endpoint: None,
                    },
                },
            },
        ];
        let node_api = Arc::new(MockNodeAPI::new(dummy_node_state.clone()));

        let test_config = create_test_config();
        let persister = Arc::new(create_test_persister(test_config.clone()));
        persister.init()?;
        persister.insert_or_update_payments(&dummy_transactions)?;
        persister.insert_lnurl_payment_external_info(
            payment_hash_with_lnurl_success_action,
            Some(&sa),
            Some(lnurl_metadata.to_string()),
            Some(test_ln_address.to_string()),
            None,
        )?;
        persister.insert_lnurl_payment_external_info(
            payment_hash_lnurl_withdraw,
            None,
            None,
            None,
            Some(test_lnurl_withdraw_endpoint.to_string()),
        )?;

        let mut builder = BreezServicesBuilder::new(test_config.clone());
        let breez_services = builder
            .lsp_api(Arc::new(MockBreezServer {}))
            .fiat_api(Arc::new(MockBreezServer {}))
            .node_api(node_api)
            .persister(persister)
            .backup_transport(Arc::new(MockBackupTransport::new()))
            .build(None)
            .await?;

        breez_services.sync().await?;
        let fetched_state = breez_services.node_info()?;
        assert_eq!(fetched_state, dummy_node_state);

        let all = breez_services
            .list_payments(ListPaymentsRequest {
                filter: PaymentTypeFilter::All,
                from_timestamp: None,
                to_timestamp: None,
                include_failures: None,
            })
            .await?;
        let mut cloned = all.clone();

        // test the right order
        cloned.reverse();
        assert_eq!(dummy_transactions, cloned);

        let received = breez_services
            .list_payments(ListPaymentsRequest {
                filter: PaymentTypeFilter::Received,
                from_timestamp: None,
                to_timestamp: None,
                include_failures: None,
            })
            .await?;
        assert_eq!(received, vec![cloned[1].clone(), cloned[0].clone()]);

        let sent = breez_services
            .list_payments(ListPaymentsRequest {
                filter: PaymentTypeFilter::Sent,
                from_timestamp: None,
                to_timestamp: None,
                include_failures: None,
            })
            .await?;
        assert_eq!(sent, vec![cloned[2].clone()]);
        assert!(matches!(
                &sent[0].details,
                PaymentDetails::Ln {data: LnPaymentDetails {lnurl_success_action, ..}}
                if lnurl_success_action == &Some(sa)));
        assert!(matches!(
                &sent[0].details,
                PaymentDetails::Ln {data: LnPaymentDetails {ln_address, ..}}
                if ln_address == &Some(test_ln_address.to_string())));
        assert!(matches!(
                &received[0].details,
                PaymentDetails::Ln {data: LnPaymentDetails {lnurl_withdraw_endpoint, ..}}
                if lnurl_withdraw_endpoint == &Some(test_lnurl_withdraw_endpoint.to_string())));

        Ok(())
    }

    #[tokio::test]
    async fn test_receive_with_open_channel() -> SdkResult<()> {
        let config = create_test_config();
        let persister = Arc::new(create_test_persister(config.clone()));
        persister.init().unwrap();

        let dummy_node_state = get_dummy_node_state();

        let node_api = Arc::new(MockNodeAPI::new(dummy_node_state.clone()));

        let breez_server = Arc::new(MockBreezServer {});
        persister.set_lsp_id(breez_server.lsp_id()).unwrap();
        persister.set_node_state(&dummy_node_state).unwrap();

        let receiver: Arc<dyn Receiver> = Arc::new(PaymentReceiver {
            config,
            node_api,
            persister,
            lsp: breez_server.clone(),
        });
        let ln_invoice = receiver
            .receive_payment(ReceivePaymentRequest {
                amount_sats: 3000,
                description: "should populate lsp hints".to_string(),
                preimage: None,
                opening_fee_params: None,
                use_description_hash: Some(false),
                expiry: None,
                cltv: None,
            })
            .await?
            .ln_invoice;
        assert_eq!(ln_invoice.routing_hints[0].hops.len(), 1);
        let lsp_hop = &ln_invoice.routing_hints[0].hops[0];
        assert_eq!(lsp_hop.src_node_id, breez_server.clone().lsp_pub_key());
        assert_eq!(
            lsp_hop.short_channel_id,
            parse_short_channel_id("1x0x0").unwrap()
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_list_lsps() -> SdkResult<()> {
        let storage_path = format!("{}/storage.sql", get_test_working_dir());
        std::fs::remove_file(storage_path).ok();

        let breez_services = breez_services().await.map_err(|e| SdkError::InitFailed {
            err: format!("Failed to get the BreezServices: {e}"),
        })?;
        breez_services.sync().await?;

        let node_pubkey = breez_services.node_info()?.id;
        let lsps = breez_services.lsp_api.list_lsps(node_pubkey).await?;
        assert_eq!(lsps.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_rates() -> Result<(), Box<dyn std::error::Error>> {
        let breez_services = breez_services().await?;
        breez_services.sync().await?;

        let rates = breez_services.fiat_api.fetch_fiat_rates().await?;
        assert_eq!(rates.len(), 1);
        assert_eq!(
            rates[0],
            Rate {
                coin: "USD".to_string(),
                value: 20_000.00,
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_buy_bitcoin_with_moonpay() -> Result<(), Box<dyn std::error::Error>> {
        let breez_services = breez_services().await?;
        breez_services.sync().await?;
        let moonpay_url = breez_services
            .buy_bitcoin(BuyBitcoinRequest {
                provider: BuyBitcoinProvider::Moonpay,
                opening_fee_params: None,
            })
            .await?
            .url;
        let parsed = Url::parse(&moonpay_url)?;
        let query_pairs = parsed.query_pairs().into_owned().collect::<HashMap<_, _>>();

        assert_eq!(parsed.host_str(), Some("mock.moonpay"));
        assert_eq!(parsed.path(), "/");

        let wallet_address = input_parser::parse(query_pairs.get("wa").unwrap()).await?;
        assert!(matches!(wallet_address, InputType::BitcoinAddress { .. }));

        let max_amount = query_pairs.get("ma").unwrap();
        assert!(Regex::new(r"^\d+\.\d{8}$").unwrap().is_match(max_amount));

        Ok(())
    }

    /// Build node service for tests
    pub(crate) async fn breez_services() -> Result<Arc<BreezServices>> {
        breez_services_with(None, vec![]).await
    }

    /// Build node service for tests with a list of known payments
    pub(crate) async fn breez_services_with(
        node_api: Option<Arc<dyn NodeAPI>>,
        known_payments: Vec<Payment>,
    ) -> Result<Arc<BreezServices>> {
        let node_api =
            node_api.unwrap_or_else(|| Arc::new(MockNodeAPI::new(get_dummy_node_state())));

        let test_config = create_test_config();
        let persister = Arc::new(create_test_persister(test_config.clone()));
        persister.init()?;
        persister.insert_or_update_payments(&known_payments)?;
        persister.set_lsp_id(MockBreezServer {}.lsp_id())?;

        let mut builder = BreezServicesBuilder::new(test_config.clone());
        let breez_services = builder
            .lsp_api(Arc::new(MockBreezServer {}))
            .fiat_api(Arc::new(MockBreezServer {}))
            .moonpay_api(Arc::new(MockBreezServer {}))
            .persister(persister)
            .node_api(node_api)
            .backup_transport(Arc::new(MockBackupTransport::new()))
            .build(None)
            .await?;

        Ok(breez_services)
    }

    /// Build dummy NodeState for tests
    pub(crate) fn get_dummy_node_state() -> NodeState {
        NodeState {
            id: "tx1".to_string(),
            block_height: 1,
            channels_balance_msat: 100,
            onchain_balance_msat: 1000,
            utxos: vec![],
            max_payable_msat: 95,
            max_receivable_msat: 1000,
            max_single_payment_amount_msat: 1000,
            max_chan_reserve_msats: 0,
            connected_peers: vec!["1111".to_string()],
            inbound_liquidity_msats: 2000,
        }
    }
}
