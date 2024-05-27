// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.36.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'breez_services.dart';
import 'chain.dart';
import 'fiat.dart';
import 'frb_generated.dart';
import 'input_parser.dart';
import 'invoice.dart';
import 'lnurl/pay/model.dart';
import 'lsp.dart';
import 'models.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// The type `BindingEventListener` is not used by any `pub` functions, thus it is ignored.
// The type `BindingLogger` is not used by any `pub` functions, thus it is ignored.

/// Wrapper around [BreezServices::connect] which also initializes SDK logging
Future<void> connect({required ConnectRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingConnect(req: req, hint: hint);

/// Check whether node service is initialized or not
Future<bool> isInitialized({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingIsInitialized(hint: hint);

/// See [BreezServices::sync]
Future<void> sync({dynamic hint}) => BreezSdkBindings.instance.api.crateBindingSync(hint: hint);

/// See [BreezServices::node_credentials]
Future<NodeCredentials?> nodeCredentials({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingNodeCredentials(hint: hint);

/// See [BreezServices::node_info]
Future<NodeState> nodeInfo({dynamic hint}) => BreezSdkBindings.instance.api.crateBindingNodeInfo(hint: hint);

/// See [BreezServices::configure_node]
Future<void> configureNode({required ConfigureNodeRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingConfigureNode(req: req, hint: hint);

/// Cleanup node resources and stop the signer.
Future<void> disconnect({dynamic hint}) => BreezSdkBindings.instance.api.crateBindingDisconnect(hint: hint);

/// See [BreezServices::sign_message]
Future<SignMessageResponse> signMessage({required SignMessageRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingSignMessage(req: req, hint: hint);

/// See [BreezServices::check_message]
Future<CheckMessageResponse> checkMessage({required CheckMessageRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingCheckMessage(req: req, hint: hint);

/// See [breez_services::mnemonic_to_seed]
Future<Uint8List> mnemonicToSeed({required String phrase, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingMnemonicToSeed(phrase: phrase, hint: hint);

/// See [BreezServices::default_config]
Future<Config> defaultConfig(
        {required EnvironmentType envType,
        required String apiKey,
        required NodeConfig nodeConfig,
        dynamic hint}) =>
    BreezSdkBindings.instance.api
        .crateBindingDefaultConfig(envType: envType, apiKey: apiKey, nodeConfig: nodeConfig, hint: hint);

/// See [BreezServices::static_backup]
Future<StaticBackupResponse> staticBackup({required StaticBackupRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingStaticBackup(req: req, hint: hint);

/// See [BreezServices::service_health_check]
Future<ServiceHealthCheckResponse> serviceHealthCheck({required String apiKey, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingServiceHealthCheck(apiKey: apiKey, hint: hint);

/// If used, this must be called before `connect`. It can only be called once.
Stream<BreezEvent> breezEventsStream({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingBreezEventsStream(hint: hint);

/// If used, this must be called before `connect`. It can only be called once.
Stream<LogEntry> breezLogStream({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingBreezLogStream(hint: hint);

/// See [BreezServices::list_lsps]
Future<List<LspInformation>> listLsps({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingListLsps(hint: hint);

/// See [BreezServices::connect_lsp]
Future<void> connectLsp({required String lspId, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingConnectLsp(lspId: lspId, hint: hint);

/// See [BreezServices::lsp_id]
Future<String?> lspId({dynamic hint}) => BreezSdkBindings.instance.api.crateBindingLspId(hint: hint);

/// See [BreezServices::fetch_lsp_info]
Future<LspInformation?> fetchLspInfo({required String id, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingFetchLspInfo(id: id, hint: hint);

/// See [BreezServices::lsp_info]
Future<LspInformation> lspInfo({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingLspInfo(hint: hint);

/// See [BreezServices::close_lsp_channels]
Future<void> closeLspChannels({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingCloseLspChannels(hint: hint);

Future<void> registerWebhook({required String webhookUrl, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingRegisterWebhook(webhookUrl: webhookUrl, hint: hint);

Future<void> unregisterWebhook({required String webhookUrl, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingUnregisterWebhook(webhookUrl: webhookUrl, hint: hint);

/// See [BreezServices::backup]
Future<void> backup({dynamic hint}) => BreezSdkBindings.instance.api.crateBindingBackup(hint: hint);

/// See [BreezServices::backup_status]
Future<BackupStatus> backupStatus({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingBackupStatus(hint: hint);

Future<LNInvoice> parseInvoice({required String invoice, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingParseInvoice(invoice: invoice, hint: hint);

Future<InputType> parseInput({required String input, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingParseInput(input: input, hint: hint);

/// See [BreezServices::list_payments]
Future<List<Payment>> listPayments({required ListPaymentsRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingListPayments(req: req, hint: hint);

/// See [BreezServices::list_payments]
Future<Payment?> paymentByHash({required String hash, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingPaymentByHash(hash: hash, hint: hint);

/// See [BreezServices::set_payment_metadata]
Future<void> setPaymentMetadata({required String hash, required String metadata, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingSetPaymentMetadata(hash: hash, metadata: metadata, hint: hint);

/// See [BreezServices::send_payment]
Future<SendPaymentResponse> sendPayment({required SendPaymentRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingSendPayment(req: req, hint: hint);

/// See [BreezServices::send_spontaneous_payment]
Future<SendPaymentResponse> sendSpontaneousPayment(
        {required SendSpontaneousPaymentRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingSendSpontaneousPayment(req: req, hint: hint);

/// See [BreezServices::receive_payment]
Future<ReceivePaymentResponse> receivePayment({required ReceivePaymentRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingReceivePayment(req: req, hint: hint);

/// See [BreezServices::lnurl_pay]
Future<LnUrlPayResult> lnurlPay({required LnUrlPayRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingLnurlPay(req: req, hint: hint);

/// See [BreezServices::lnurl_withdraw]
Future<LnUrlWithdrawResult> lnurlWithdraw({required LnUrlWithdrawRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingLnurlWithdraw(req: req, hint: hint);

/// See [BreezServices::lnurl_auth]
Future<LnUrlCallbackStatus> lnurlAuth({required LnUrlAuthRequestData reqData, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingLnurlAuth(reqData: reqData, hint: hint);

/// See [BreezServices::report_issue]
Future<void> reportIssue({required ReportIssueRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingReportIssue(req: req, hint: hint);

/// See [BreezServices::fetch_fiat_rates]
Future<List<Rate>> fetchFiatRates({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingFetchFiatRates(hint: hint);

/// See [BreezServices::list_fiat_currencies]
Future<List<FiatCurrency>> listFiatCurrencies({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingListFiatCurrencies(hint: hint);

/// See [BreezServices::max_reverse_swap_amount]
Future<MaxReverseSwapAmountResponse> maxReverseSwapAmount({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingMaxReverseSwapAmount(hint: hint);

/// See [BreezServices::send_onchain]
Future<SendOnchainResponse> sendOnchain({required SendOnchainRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingSendOnchain(req: req, hint: hint);

/// See [BreezServices::pay_onchain]
Future<PayOnchainResponse> payOnchain({required PayOnchainRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingPayOnchain(req: req, hint: hint);

/// See [BreezServices::receive_onchain]
Future<SwapInfo> receiveOnchain({required ReceiveOnchainRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingReceiveOnchain(req: req, hint: hint);

/// See [BreezServices::buy_bitcoin]
Future<BuyBitcoinResponse> buyBitcoin({required BuyBitcoinRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingBuyBitcoin(req: req, hint: hint);

/// See [BreezServices::redeem_onchain_funds]
Future<RedeemOnchainFundsResponse> redeemOnchainFunds(
        {required RedeemOnchainFundsRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingRedeemOnchainFunds(req: req, hint: hint);

/// See [BreezServices::prepare_redeem_onchain_funds]
Future<PrepareRedeemOnchainFundsResponse> prepareRedeemOnchainFunds(
        {required PrepareRedeemOnchainFundsRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingPrepareRedeemOnchainFunds(req: req, hint: hint);

/// See [BreezServices::list_refundables]
Future<List<SwapInfo>> listRefundables({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingListRefundables(hint: hint);

/// See [BreezServices::prepare_refund]
Future<PrepareRefundResponse> prepareRefund({required PrepareRefundRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingPrepareRefund(req: req, hint: hint);

/// See [BreezServices::refund]
Future<RefundResponse> refund({required RefundRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingRefund(req: req, hint: hint);

/// See [BreezServices::rescan_swaps]
Future<void> rescanSwaps({dynamic hint}) => BreezSdkBindings.instance.api.crateBindingRescanSwaps(hint: hint);

/// See [BreezServices::redeem_swap]
Future<void> redeemSwap({required String swapAddress, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingRedeemSwap(swapAddress: swapAddress, hint: hint);

/// See [BreezServices::in_progress_swap]
Future<SwapInfo?> inProgressSwap({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingInProgressSwap(hint: hint);

/// See [BreezServices::in_progress_reverse_swaps]
Future<List<ReverseSwapInfo>> inProgressReverseSwaps({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingInProgressReverseSwaps(hint: hint);

/// See [BreezServices::open_channel_fee]
Future<OpenChannelFeeResponse> openChannelFee({required OpenChannelFeeRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingOpenChannelFee(req: req, hint: hint);

/// See [BreezServices::fetch_reverse_swap_fees]
Future<ReverseSwapPairInfo> fetchReverseSwapFees({required ReverseSwapFeesRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingFetchReverseSwapFees(req: req, hint: hint);

/// See [BreezServices::onchain_payment_limits]
Future<OnchainPaymentLimitsResponse> onchainPaymentLimits({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingOnchainPaymentLimits(hint: hint);

/// See [BreezServices::prepare_onchain_payment]
Future<PrepareOnchainPaymentResponse> prepareOnchainPayment(
        {required PrepareOnchainPaymentRequest req, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingPrepareOnchainPayment(req: req, hint: hint);

/// See [BreezServices::in_progress_onchain_payments]
Future<List<ReverseSwapInfo>> inProgressOnchainPayments({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingInProgressOnchainPayments(hint: hint);

/// See [BreezServices::recommended_fees]
Future<RecommendedFees> recommendedFees({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingRecommendedFees(hint: hint);

/// See [BreezServices::execute_dev_command]
Future<String> executeCommand({required String command, dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingExecuteCommand(command: command, hint: hint);

/// See [BreezServices::generate_diagnostic_data]
Future<String> generateDiagnosticData({dynamic hint}) =>
    BreezSdkBindings.instance.api.crateBindingGenerateDiagnosticData(hint: hint);
