// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.33.

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
    BreezSdkCore.instance.api.connect(req: req, hint: hint);

/// Check whether node service is initialized or not
Future<bool> isInitialized({dynamic hint}) => BreezSdkCore.instance.api.isInitialized(hint: hint);

/// See [BreezServices::sync]
Future<void> sync({dynamic hint}) => BreezSdkCore.instance.api.sync(hint: hint);

/// See [BreezServices::node_credentials]
Future<NodeCredentials?> nodeCredentials({dynamic hint}) =>
    BreezSdkCore.instance.api.nodeCredentials(hint: hint);

/// See [BreezServices::node_info]
Future<NodeState> nodeInfo({dynamic hint}) => BreezSdkCore.instance.api.nodeInfo(hint: hint);

/// See [BreezServices::configure_node]
Future<void> configureNode({required ConfigureNodeRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.configureNode(req: req, hint: hint);

/// Cleanup node resources and stop the signer.
Future<void> disconnect({dynamic hint}) => BreezSdkCore.instance.api.disconnect(hint: hint);

/// See [BreezServices::sign_message]
Future<SignMessageResponse> signMessage({required SignMessageRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.signMessage(req: req, hint: hint);

/// See [BreezServices::check_message]
Future<CheckMessageResponse> checkMessage({required CheckMessageRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.checkMessage(req: req, hint: hint);

/// See [breez_services::mnemonic_to_seed]
Future<Uint8List> mnemonicToSeed({required String phrase, dynamic hint}) =>
    BreezSdkCore.instance.api.mnemonicToSeed(phrase: phrase, hint: hint);

/// See [BreezServices::default_config]
Future<Config> defaultConfig(
        {required EnvironmentType envType,
        required String apiKey,
        required NodeConfig nodeConfig,
        dynamic hint}) =>
    BreezSdkCore.instance.api
        .defaultConfig(envType: envType, apiKey: apiKey, nodeConfig: nodeConfig, hint: hint);

/// See [BreezServices::static_backup]
Future<StaticBackupResponse> staticBackup({required StaticBackupRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.staticBackup(req: req, hint: hint);

/// See [BreezServices::service_health_check]
Future<ServiceHealthCheckResponse> serviceHealthCheck({required String apiKey, dynamic hint}) =>
    BreezSdkCore.instance.api.serviceHealthCheck(apiKey: apiKey, hint: hint);

/// If used, this must be called before `connect`. It can only be called once.
Stream<BreezEvent> breezEventsStream({dynamic hint}) =>
    BreezSdkCore.instance.api.breezEventsStream(hint: hint);

/// If used, this must be called before `connect`. It can only be called once.
Stream<LogEntry> breezLogStream({dynamic hint}) => BreezSdkCore.instance.api.breezLogStream(hint: hint);

/// See [BreezServices::list_lsps]
Future<List<LspInformation>> listLsps({dynamic hint}) => BreezSdkCore.instance.api.listLsps(hint: hint);

/// See [BreezServices::connect_lsp]
Future<void> connectLsp({required String lspId, dynamic hint}) =>
    BreezSdkCore.instance.api.connectLsp(lspId: lspId, hint: hint);

/// See [BreezServices::lsp_id]
Future<String?> lspId({dynamic hint}) => BreezSdkCore.instance.api.lspId(hint: hint);

/// See [BreezServices::fetch_lsp_info]
Future<LspInformation?> fetchLspInfo({required String id, dynamic hint}) =>
    BreezSdkCore.instance.api.fetchLspInfo(id: id, hint: hint);

/// See [BreezServices::lsp_info]
Future<LspInformation> lspInfo({dynamic hint}) => BreezSdkCore.instance.api.lspInfo(hint: hint);

/// See [BreezServices::close_lsp_channels]
Future<void> closeLspChannels({dynamic hint}) => BreezSdkCore.instance.api.closeLspChannels(hint: hint);

Future<void> registerWebhook({required String webhookUrl, dynamic hint}) =>
    BreezSdkCore.instance.api.registerWebhook(webhookUrl: webhookUrl, hint: hint);

/// See [BreezServices::backup]
Future<void> backup({dynamic hint}) => BreezSdkCore.instance.api.backup(hint: hint);

/// See [BreezServices::backup_status]
Future<BackupStatus> backupStatus({dynamic hint}) => BreezSdkCore.instance.api.backupStatus(hint: hint);

Future<LNInvoice> parseInvoice({required String invoice, dynamic hint}) =>
    BreezSdkCore.instance.api.parseInvoice(invoice: invoice, hint: hint);

Future<InputType> parseInput({required String input, dynamic hint}) =>
    BreezSdkCore.instance.api.parseInput(input: input, hint: hint);

/// See [BreezServices::list_payments]
Future<List<Payment>> listPayments({required ListPaymentsRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.listPayments(req: req, hint: hint);

/// See [BreezServices::list_payments]
Future<Payment?> paymentByHash({required String hash, dynamic hint}) =>
    BreezSdkCore.instance.api.paymentByHash(hash: hash, hint: hint);

/// See [BreezServices::set_payment_metadata]
Future<void> setPaymentMetadata({required String hash, required String metadata, dynamic hint}) =>
    BreezSdkCore.instance.api.setPaymentMetadata(hash: hash, metadata: metadata, hint: hint);

/// See [BreezServices::send_payment]
Future<SendPaymentResponse> sendPayment({required SendPaymentRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.sendPayment(req: req, hint: hint);

/// See [BreezServices::send_spontaneous_payment]
Future<SendPaymentResponse> sendSpontaneousPayment(
        {required SendSpontaneousPaymentRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.sendSpontaneousPayment(req: req, hint: hint);

/// See [BreezServices::receive_payment]
Future<ReceivePaymentResponse> receivePayment({required ReceivePaymentRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.receivePayment(req: req, hint: hint);

/// See [BreezServices::lnurl_pay]
Future<LnUrlPayResult> lnurlPay({required LnUrlPayRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.lnurlPay(req: req, hint: hint);

/// See [BreezServices::lnurl_withdraw]
Future<LnUrlWithdrawResult> lnurlWithdraw({required LnUrlWithdrawRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.lnurlWithdraw(req: req, hint: hint);

/// See [BreezServices::lnurl_auth]
Future<LnUrlCallbackStatus> lnurlAuth({required LnUrlAuthRequestData reqData, dynamic hint}) =>
    BreezSdkCore.instance.api.lnurlAuth(reqData: reqData, hint: hint);

/// See [BreezServices::report_issue]
Future<void> reportIssue({required ReportIssueRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.reportIssue(req: req, hint: hint);

/// See [BreezServices::fetch_fiat_rates]
Future<List<Rate>> fetchFiatRates({dynamic hint}) => BreezSdkCore.instance.api.fetchFiatRates(hint: hint);

/// See [BreezServices::list_fiat_currencies]
Future<List<FiatCurrency>> listFiatCurrencies({dynamic hint}) =>
    BreezSdkCore.instance.api.listFiatCurrencies(hint: hint);

/// See [BreezServices::max_reverse_swap_amount]
Future<MaxReverseSwapAmountResponse> maxReverseSwapAmount({dynamic hint}) =>
    BreezSdkCore.instance.api.maxReverseSwapAmount(hint: hint);

/// See [BreezServices::send_onchain]
Future<SendOnchainResponse> sendOnchain({required SendOnchainRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.sendOnchain(req: req, hint: hint);

/// See [BreezServices::pay_onchain]
Future<PayOnchainResponse> payOnchain({required PayOnchainRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.payOnchain(req: req, hint: hint);

/// See [BreezServices::receive_onchain]
Future<SwapInfo> receiveOnchain({required ReceiveOnchainRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.receiveOnchain(req: req, hint: hint);

/// See [BreezServices::buy_bitcoin]
Future<BuyBitcoinResponse> buyBitcoin({required BuyBitcoinRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.buyBitcoin(req: req, hint: hint);

/// See [BreezServices::redeem_onchain_funds]
Future<RedeemOnchainFundsResponse> redeemOnchainFunds(
        {required RedeemOnchainFundsRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.redeemOnchainFunds(req: req, hint: hint);

/// See [BreezServices::prepare_redeem_onchain_funds]
Future<PrepareRedeemOnchainFundsResponse> prepareRedeemOnchainFunds(
        {required PrepareRedeemOnchainFundsRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.prepareRedeemOnchainFunds(req: req, hint: hint);

/// See [BreezServices::list_refundables]
Future<List<SwapInfo>> listRefundables({dynamic hint}) =>
    BreezSdkCore.instance.api.listRefundables(hint: hint);

/// See [BreezServices::prepare_refund]
Future<PrepareRefundResponse> prepareRefund({required PrepareRefundRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.prepareRefund(req: req, hint: hint);

/// See [BreezServices::refund]
Future<RefundResponse> refund({required RefundRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.refund(req: req, hint: hint);

/// See [BreezServices::rescan_swaps]
Future<void> rescanSwaps({dynamic hint}) => BreezSdkCore.instance.api.rescanSwaps(hint: hint);

/// See [BreezServices::redeem_swap]
Future<void> redeemSwap({required String swapAddress, dynamic hint}) =>
    BreezSdkCore.instance.api.redeemSwap(swapAddress: swapAddress, hint: hint);

/// See [BreezServices::in_progress_swap]
Future<SwapInfo?> inProgressSwap({dynamic hint}) => BreezSdkCore.instance.api.inProgressSwap(hint: hint);

/// See [BreezServices::in_progress_reverse_swaps]
Future<List<ReverseSwapInfo>> inProgressReverseSwaps({dynamic hint}) =>
    BreezSdkCore.instance.api.inProgressReverseSwaps(hint: hint);

/// See [BreezServices::open_channel_fee]
Future<OpenChannelFeeResponse> openChannelFee({required OpenChannelFeeRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.openChannelFee(req: req, hint: hint);

/// See [BreezServices::fetch_reverse_swap_fees]
Future<ReverseSwapPairInfo> fetchReverseSwapFees({required ReverseSwapFeesRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.fetchReverseSwapFees(req: req, hint: hint);

/// See [BreezServices::onchain_payment_limits]
Future<OnchainPaymentLimitsResponse> onchainPaymentLimits({dynamic hint}) =>
    BreezSdkCore.instance.api.onchainPaymentLimits(hint: hint);

/// See [BreezServices::prepare_onchain_payment]
Future<PrepareOnchainPaymentResponse> prepareOnchainPayment(
        {required PrepareOnchainPaymentRequest req, dynamic hint}) =>
    BreezSdkCore.instance.api.prepareOnchainPayment(req: req, hint: hint);

/// See [BreezServices::in_progress_onchain_payments]
Future<List<ReverseSwapInfo>> inProgressOnchainPayments({dynamic hint}) =>
    BreezSdkCore.instance.api.inProgressOnchainPayments(hint: hint);

/// See [BreezServices::recommended_fees]
Future<RecommendedFees> recommendedFees({dynamic hint}) =>
    BreezSdkCore.instance.api.recommendedFees(hint: hint);

/// See [BreezServices::execute_dev_command]
Future<String> executeCommand({required String command, dynamic hint}) =>
    BreezSdkCore.instance.api.executeCommand(command: command, hint: hint);

/// See [BreezServices::generate_diagnostic_data]
Future<String> generateDiagnosticData({dynamic hint}) =>
    BreezSdkCore.instance.api.generateDiagnosticData(hint: hint);
