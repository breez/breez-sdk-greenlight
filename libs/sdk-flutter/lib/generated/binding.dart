// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.6.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'breez_services.dart';
import 'chain.dart';
import 'frb_generated.dart';
import 'lnurl/pay.dart';
import 'lsp.dart';
import 'models.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'binding.freezed.dart';

// These functions are ignored because they are not marked as `pub`: `block_on`, `get_breez_services`, `init`, `new`, `rt`
// These types are ignored because they are not used by any `pub` functions: `BindingLogger`, `LnUrlPayError`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `enabled`, `flush`, `log`

/// Wrapper around [BreezServices::connect] which also initializes SDK logging
Future<void> connect({required ConnectRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingConnect(req: req);

/// Check whether node service is initialized or not
Future<bool> isInitialized() => BreezSdkBindings.instance.api.crateBindingIsInitialized();

/// See [BreezServices::sync]
Future<void> sync() => BreezSdkBindings.instance.api.crateBindingSync();

/// See [BreezServices::node_credentials]
Future<NodeCredentials?> nodeCredentials() => BreezSdkBindings.instance.api.crateBindingNodeCredentials();

/// See [BreezServices::node_info]
Future<NodeState> nodeInfo() => BreezSdkBindings.instance.api.crateBindingNodeInfo();

/// See [BreezServices::configure_node]
Future<void> configureNode({required ConfigureNodeRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingConfigureNode(req: req);

/// Cleanup node resources and stop the signer.
Future<void> disconnect() => BreezSdkBindings.instance.api.crateBindingDisconnect();

/// See [BreezServices::sign_message]
Future<SignMessageResponse> signMessage({required SignMessageRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingSignMessage(req: req);

/// See [BreezServices::check_message]
Future<CheckMessageResponse> checkMessage({required CheckMessageRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingCheckMessage(req: req);

/// See [breez_services::mnemonic_to_seed]
Future<Uint8List> mnemonicToSeed({required String phrase}) =>
    BreezSdkBindings.instance.api.crateBindingMnemonicToSeed(phrase: phrase);

/// See [BreezServices::default_config]
Future<Config> defaultConfig(
        {required EnvironmentType envType, required String apiKey, required NodeConfig nodeConfig}) =>
    BreezSdkBindings.instance.api
        .crateBindingDefaultConfig(envType: envType, apiKey: apiKey, nodeConfig: nodeConfig);

/// See [BreezServices::static_backup]
Future<StaticBackupResponse> staticBackup({required StaticBackupRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingStaticBackup(req: req);

/// See [BreezServices::service_health_check]
Future<ServiceHealthCheckResponse> serviceHealthCheck({required String apiKey}) =>
    BreezSdkBindings.instance.api.crateBindingServiceHealthCheck(apiKey: apiKey);

/// If used, this must be called before `connect`. It can only be called once.
Stream<BreezEvent> breezEventsStream() => BreezSdkBindings.instance.api.crateBindingBreezEventsStream();

/// If used, this must be called before `connect`. It can only be called once.
Stream<LogEntry> breezLogStream() => BreezSdkBindings.instance.api.crateBindingBreezLogStream();

/// See [BreezServices::list_lsps]
Future<List<LspInformation>> listLsps() => BreezSdkBindings.instance.api.crateBindingListLsps();

/// See [BreezServices::connect_lsp]
Future<void> connectLsp({required String lspId}) =>
    BreezSdkBindings.instance.api.crateBindingConnectLsp(lspId: lspId);

/// See [BreezServices::lsp_id]
Future<String?> lspId() => BreezSdkBindings.instance.api.crateBindingLspId();

/// See [BreezServices::fetch_lsp_info]
Future<LspInformation?> fetchLspInfo({required String id}) =>
    BreezSdkBindings.instance.api.crateBindingFetchLspInfo(id: id);

/// See [BreezServices::lsp_info]
Future<LspInformation> lspInfo() => BreezSdkBindings.instance.api.crateBindingLspInfo();

/// See [BreezServices::close_lsp_channels]
Future<void> closeLspChannels() => BreezSdkBindings.instance.api.crateBindingCloseLspChannels();

Future<void> registerWebhook({required String webhookUrl}) =>
    BreezSdkBindings.instance.api.crateBindingRegisterWebhook(webhookUrl: webhookUrl);

Future<void> unregisterWebhook({required String webhookUrl}) =>
    BreezSdkBindings.instance.api.crateBindingUnregisterWebhook(webhookUrl: webhookUrl);

/// See [BreezServices::backup]
Future<void> backup() => BreezSdkBindings.instance.api.crateBindingBackup();

/// See [BreezServices::backup_status]
Future<BackupStatus> backupStatus() => BreezSdkBindings.instance.api.crateBindingBackupStatus();

Future<LNInvoice> parseInvoice({required String invoice}) =>
    BreezSdkBindings.instance.api.crateBindingParseInvoice(invoice: invoice);

Future<InputType> parseInput({required String input}) =>
    BreezSdkBindings.instance.api.crateBindingParseInput(input: input);

/// See [BreezServices::list_payments]
Future<List<Payment>> listPayments({required ListPaymentsRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingListPayments(req: req);

/// See [BreezServices::list_payments]
Future<Payment?> paymentByHash({required String hash}) =>
    BreezSdkBindings.instance.api.crateBindingPaymentByHash(hash: hash);

/// See [BreezServices::set_payment_metadata]
Future<void> setPaymentMetadata({required String hash, required String metadata}) =>
    BreezSdkBindings.instance.api.crateBindingSetPaymentMetadata(hash: hash, metadata: metadata);

/// See [BreezServices::send_payment]
Future<SendPaymentResponse> sendPayment({required SendPaymentRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingSendPayment(req: req);

/// See [BreezServices::send_spontaneous_payment]
Future<SendPaymentResponse> sendSpontaneousPayment({required SendSpontaneousPaymentRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingSendSpontaneousPayment(req: req);

/// See [BreezServices::receive_payment]
Future<ReceivePaymentResponse> receivePayment({required ReceivePaymentRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingReceivePayment(req: req);

/// See [BreezServices::lnurl_pay]
Future<LnUrlPayResult> lnurlPay({required LnUrlPayRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingLnurlPay(req: req);

/// See [BreezServices::lnurl_withdraw]
Future<LnUrlWithdrawResult> lnurlWithdraw({required LnUrlWithdrawRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingLnurlWithdraw(req: req);

/// See [BreezServices::lnurl_auth]
Future<LnUrlCallbackStatus> lnurlAuth({required LnUrlAuthRequestData reqData}) =>
    BreezSdkBindings.instance.api.crateBindingLnurlAuth(reqData: reqData);

/// See [BreezServices::report_issue]
Future<void> reportIssue({required ReportIssueRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingReportIssue(req: req);

/// See [BreezServices::fetch_fiat_rates]
Future<List<Rate>> fetchFiatRates() => BreezSdkBindings.instance.api.crateBindingFetchFiatRates();

/// See [BreezServices::list_fiat_currencies]
Future<List<FiatCurrency>> listFiatCurrencies() =>
    BreezSdkBindings.instance.api.crateBindingListFiatCurrencies();

/// See [BreezServices::max_reverse_swap_amount]
Future<MaxReverseSwapAmountResponse> maxReverseSwapAmount() =>
    BreezSdkBindings.instance.api.crateBindingMaxReverseSwapAmount();

/// See [BreezServices::send_onchain]
Future<SendOnchainResponse> sendOnchain({required SendOnchainRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingSendOnchain(req: req);

/// See [BreezServices::pay_onchain]
Future<PayOnchainResponse> payOnchain({required PayOnchainRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingPayOnchain(req: req);

/// See [BreezServices::receive_onchain]
Future<SwapInfo> receiveOnchain({required ReceiveOnchainRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingReceiveOnchain(req: req);

/// See [BreezServices::buy_bitcoin]
Future<BuyBitcoinResponse> buyBitcoin({required BuyBitcoinRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingBuyBitcoin(req: req);

/// See [BreezServices::redeem_onchain_funds]
Future<RedeemOnchainFundsResponse> redeemOnchainFunds({required RedeemOnchainFundsRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingRedeemOnchainFunds(req: req);

/// See [BreezServices::prepare_redeem_onchain_funds]
Future<PrepareRedeemOnchainFundsResponse> prepareRedeemOnchainFunds(
        {required PrepareRedeemOnchainFundsRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingPrepareRedeemOnchainFunds(req: req);

/// See [BreezServices::list_refundables]
Future<List<SwapInfo>> listRefundables() => BreezSdkBindings.instance.api.crateBindingListRefundables();

/// See [BreezServices::prepare_refund]
Future<PrepareRefundResponse> prepareRefund({required PrepareRefundRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingPrepareRefund(req: req);

/// See [BreezServices::refund]
Future<RefundResponse> refund({required RefundRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingRefund(req: req);

/// See [BreezServices::rescan_swaps]
Future<void> rescanSwaps() => BreezSdkBindings.instance.api.crateBindingRescanSwaps();

/// See [BreezServices::redeem_swap]
Future<void> redeemSwap({required String swapAddress}) =>
    BreezSdkBindings.instance.api.crateBindingRedeemSwap(swapAddress: swapAddress);

/// See [BreezServices::in_progress_swap]
Future<SwapInfo?> inProgressSwap() => BreezSdkBindings.instance.api.crateBindingInProgressSwap();

/// See [BreezServices::in_progress_reverse_swaps]
Future<List<ReverseSwapInfo>> inProgressReverseSwaps() =>
    BreezSdkBindings.instance.api.crateBindingInProgressReverseSwaps();

/// See [BreezServices::claim_reverse_swap]
Future<void> claimReverseSwap({required String lockupAddress}) =>
    BreezSdkBindings.instance.api.crateBindingClaimReverseSwap(lockupAddress: lockupAddress);

/// See [BreezServices::open_channel_fee]
Future<OpenChannelFeeResponse> openChannelFee({required OpenChannelFeeRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingOpenChannelFee(req: req);

/// See [BreezServices::fetch_reverse_swap_fees]
Future<ReverseSwapPairInfo> fetchReverseSwapFees({required ReverseSwapFeesRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingFetchReverseSwapFees(req: req);

/// See [BreezServices::onchain_payment_limits]
Future<OnchainPaymentLimitsResponse> onchainPaymentLimits() =>
    BreezSdkBindings.instance.api.crateBindingOnchainPaymentLimits();

/// See [BreezServices::prepare_onchain_payment]
Future<PrepareOnchainPaymentResponse> prepareOnchainPayment({required PrepareOnchainPaymentRequest req}) =>
    BreezSdkBindings.instance.api.crateBindingPrepareOnchainPayment(req: req);

/// See [BreezServices::in_progress_onchain_payments]
Future<List<ReverseSwapInfo>> inProgressOnchainPayments() =>
    BreezSdkBindings.instance.api.crateBindingInProgressOnchainPayments();

/// See [BreezServices::recommended_fees]
Future<RecommendedFees> recommendedFees() => BreezSdkBindings.instance.api.crateBindingRecommendedFees();

/// See [BreezServices::execute_dev_command]
Future<String> executeCommand({required String command}) =>
    BreezSdkBindings.instance.api.crateBindingExecuteCommand(command: command);

/// See [BreezServices::generate_diagnostic_data]
Future<String> generateDiagnosticData() => BreezSdkBindings.instance.api.crateBindingGenerateDiagnosticData();

class AesSuccessActionDataDecrypted {
  final String description;
  final String plaintext;

  const AesSuccessActionDataDecrypted({
    required this.description,
    required this.plaintext,
  });

  @override
  int get hashCode => description.hashCode ^ plaintext.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AesSuccessActionDataDecrypted &&
          runtimeType == other.runtimeType &&
          description == other.description &&
          plaintext == other.plaintext;
}

@freezed
sealed class AesSuccessActionDataResult with _$AesSuccessActionDataResult {
  const AesSuccessActionDataResult._();

  const factory AesSuccessActionDataResult.decrypted({
    required AesSuccessActionDataDecrypted data,
  }) = AesSuccessActionDataResult_Decrypted;
  const factory AesSuccessActionDataResult.errorStatus({
    required String reason,
  }) = AesSuccessActionDataResult_ErrorStatus;
}

class BindingEventListener {
  const BindingEventListener();

  Future<void> onEvent({required BreezEvent e}) =>
      BreezSdkBindings.instance.api.crateBindingBindingEventListenerOnEvent(that: this, e: e);

  @override
  int get hashCode => 0;

  @override
  bool operator ==(Object other) =>
      identical(this, other) || other is BindingEventListener && runtimeType == other.runtimeType;
}

class BitcoinAddressData {
  final String address;
  final Network network;
  final BigInt? amountSat;
  final String? label;
  final String? message;

  const BitcoinAddressData({
    required this.address,
    required this.network,
    this.amountSat,
    this.label,
    this.message,
  });

  @override
  int get hashCode =>
      address.hashCode ^ network.hashCode ^ amountSat.hashCode ^ label.hashCode ^ message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BitcoinAddressData &&
          runtimeType == other.runtimeType &&
          address == other.address &&
          network == other.network &&
          amountSat == other.amountSat &&
          label == other.label &&
          message == other.message;
}

class CurrencyInfo {
  final String name;
  final int fractionSize;
  final int? spacing;
  final Symbol? symbol;
  final Symbol? uniqSymbol;
  final List<LocalizedName> localizedName;
  final List<LocaleOverrides> localeOverrides;

  const CurrencyInfo({
    required this.name,
    required this.fractionSize,
    this.spacing,
    this.symbol,
    this.uniqSymbol,
    required this.localizedName,
    required this.localeOverrides,
  });

  @override
  int get hashCode =>
      name.hashCode ^
      fractionSize.hashCode ^
      spacing.hashCode ^
      symbol.hashCode ^
      uniqSymbol.hashCode ^
      localizedName.hashCode ^
      localeOverrides.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CurrencyInfo &&
          runtimeType == other.runtimeType &&
          name == other.name &&
          fractionSize == other.fractionSize &&
          spacing == other.spacing &&
          symbol == other.symbol &&
          uniqSymbol == other.uniqSymbol &&
          localizedName == other.localizedName &&
          localeOverrides == other.localeOverrides;
}

class FiatCurrency {
  final String id;
  final CurrencyInfo info;

  const FiatCurrency({
    required this.id,
    required this.info,
  });

  @override
  int get hashCode => id.hashCode ^ info.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FiatCurrency && runtimeType == other.runtimeType && id == other.id && info == other.info;
}

@freezed
sealed class InputType with _$InputType {
  const InputType._();

  const factory InputType.bitcoinAddress({
    required BitcoinAddressData address,
  }) = InputType_BitcoinAddress;
  const factory InputType.bolt11({
    required LNInvoice invoice,
  }) = InputType_Bolt11;
  const factory InputType.nodeId({
    required String nodeId,
  }) = InputType_NodeId;
  const factory InputType.url({
    required String url,
  }) = InputType_Url;
  const factory InputType.lnUrlPay({
    required LnUrlPayRequestData data,
  }) = InputType_LnUrlPay;
  const factory InputType.lnUrlWithdraw({
    required LnUrlWithdrawRequestData data,
  }) = InputType_LnUrlWithdraw;
  const factory InputType.lnUrlAuth({
    required LnUrlAuthRequestData data,
  }) = InputType_LnUrlAuth;
  const factory InputType.lnUrlError({
    required LnUrlErrorData data,
  }) = InputType_LnUrlError;
}

class LNInvoice {
  final String bolt11;
  final Network network;
  final String payeePubkey;
  final String paymentHash;
  final String? description;
  final String? descriptionHash;
  final BigInt? amountMsat;
  final BigInt timestamp;
  final BigInt expiry;
  final List<RouteHint> routingHints;
  final Uint8List paymentSecret;
  final BigInt minFinalCltvExpiryDelta;

  const LNInvoice({
    required this.bolt11,
    required this.network,
    required this.payeePubkey,
    required this.paymentHash,
    this.description,
    this.descriptionHash,
    this.amountMsat,
    required this.timestamp,
    required this.expiry,
    required this.routingHints,
    required this.paymentSecret,
    required this.minFinalCltvExpiryDelta,
  });

  @override
  int get hashCode =>
      bolt11.hashCode ^
      network.hashCode ^
      payeePubkey.hashCode ^
      paymentHash.hashCode ^
      description.hashCode ^
      descriptionHash.hashCode ^
      amountMsat.hashCode ^
      timestamp.hashCode ^
      expiry.hashCode ^
      routingHints.hashCode ^
      paymentSecret.hashCode ^
      minFinalCltvExpiryDelta.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LNInvoice &&
          runtimeType == other.runtimeType &&
          bolt11 == other.bolt11 &&
          network == other.network &&
          payeePubkey == other.payeePubkey &&
          paymentHash == other.paymentHash &&
          description == other.description &&
          descriptionHash == other.descriptionHash &&
          amountMsat == other.amountMsat &&
          timestamp == other.timestamp &&
          expiry == other.expiry &&
          routingHints == other.routingHints &&
          paymentSecret == other.paymentSecret &&
          minFinalCltvExpiryDelta == other.minFinalCltvExpiryDelta;
}

class LnUrlAuthRequestData {
  final String k1;
  final String? action;
  final String domain;
  final String url;

  const LnUrlAuthRequestData({
    required this.k1,
    this.action,
    required this.domain,
    required this.url,
  });

  @override
  int get hashCode => k1.hashCode ^ action.hashCode ^ domain.hashCode ^ url.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlAuthRequestData &&
          runtimeType == other.runtimeType &&
          k1 == other.k1 &&
          action == other.action &&
          domain == other.domain &&
          url == other.url;
}

@freezed
sealed class LnUrlCallbackStatus with _$LnUrlCallbackStatus {
  const LnUrlCallbackStatus._();

  const factory LnUrlCallbackStatus.ok() = LnUrlCallbackStatus_Ok;
  const factory LnUrlCallbackStatus.errorStatus({
    required LnUrlErrorData data,
  }) = LnUrlCallbackStatus_ErrorStatus;
}

class LnUrlErrorData {
  final String reason;

  const LnUrlErrorData({
    required this.reason,
  });

  @override
  int get hashCode => reason.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlErrorData && runtimeType == other.runtimeType && reason == other.reason;
}

class LnUrlPayErrorData {
  final String paymentHash;
  final String reason;

  const LnUrlPayErrorData({
    required this.paymentHash,
    required this.reason,
  });

  @override
  int get hashCode => paymentHash.hashCode ^ reason.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlPayErrorData &&
          runtimeType == other.runtimeType &&
          paymentHash == other.paymentHash &&
          reason == other.reason;
}

class LnUrlPayRequest {
  final LnUrlPayRequestData data;
  final BigInt amountMsat;
  final bool useTrampoline;
  final String? comment;
  final String? paymentLabel;
  final bool? validateSuccessActionUrl;

  const LnUrlPayRequest({
    required this.data,
    required this.amountMsat,
    required this.useTrampoline,
    this.comment,
    this.paymentLabel,
    this.validateSuccessActionUrl,
  });

  @override
  int get hashCode =>
      data.hashCode ^
      amountMsat.hashCode ^
      useTrampoline.hashCode ^
      comment.hashCode ^
      paymentLabel.hashCode ^
      validateSuccessActionUrl.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlPayRequest &&
          runtimeType == other.runtimeType &&
          data == other.data &&
          amountMsat == other.amountMsat &&
          useTrampoline == other.useTrampoline &&
          comment == other.comment &&
          paymentLabel == other.paymentLabel &&
          validateSuccessActionUrl == other.validateSuccessActionUrl;
}

class LnUrlPayRequestData {
  final String callback;
  final BigInt minSendable;
  final BigInt maxSendable;
  final String metadataStr;
  final int commentAllowed;
  final String domain;
  final bool allowsNostr;
  final String? nostrPubkey;
  final String? lnAddress;

  const LnUrlPayRequestData({
    required this.callback,
    required this.minSendable,
    required this.maxSendable,
    required this.metadataStr,
    required this.commentAllowed,
    required this.domain,
    required this.allowsNostr,
    this.nostrPubkey,
    this.lnAddress,
  });

  @override
  int get hashCode =>
      callback.hashCode ^
      minSendable.hashCode ^
      maxSendable.hashCode ^
      metadataStr.hashCode ^
      commentAllowed.hashCode ^
      domain.hashCode ^
      allowsNostr.hashCode ^
      nostrPubkey.hashCode ^
      lnAddress.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlPayRequestData &&
          runtimeType == other.runtimeType &&
          callback == other.callback &&
          minSendable == other.minSendable &&
          maxSendable == other.maxSendable &&
          metadataStr == other.metadataStr &&
          commentAllowed == other.commentAllowed &&
          domain == other.domain &&
          allowsNostr == other.allowsNostr &&
          nostrPubkey == other.nostrPubkey &&
          lnAddress == other.lnAddress;
}

class LnUrlWithdrawRequest {
  final LnUrlWithdrawRequestData data;
  final BigInt amountMsat;
  final String? description;

  const LnUrlWithdrawRequest({
    required this.data,
    required this.amountMsat,
    this.description,
  });

  @override
  int get hashCode => data.hashCode ^ amountMsat.hashCode ^ description.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlWithdrawRequest &&
          runtimeType == other.runtimeType &&
          data == other.data &&
          amountMsat == other.amountMsat &&
          description == other.description;
}

class LnUrlWithdrawRequestData {
  final String callback;
  final String k1;
  final String defaultDescription;
  final BigInt minWithdrawable;
  final BigInt maxWithdrawable;

  const LnUrlWithdrawRequestData({
    required this.callback,
    required this.k1,
    required this.defaultDescription,
    required this.minWithdrawable,
    required this.maxWithdrawable,
  });

  @override
  int get hashCode =>
      callback.hashCode ^
      k1.hashCode ^
      defaultDescription.hashCode ^
      minWithdrawable.hashCode ^
      maxWithdrawable.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlWithdrawRequestData &&
          runtimeType == other.runtimeType &&
          callback == other.callback &&
          k1 == other.k1 &&
          defaultDescription == other.defaultDescription &&
          minWithdrawable == other.minWithdrawable &&
          maxWithdrawable == other.maxWithdrawable;
}

@freezed
sealed class LnUrlWithdrawResult with _$LnUrlWithdrawResult {
  const LnUrlWithdrawResult._();

  const factory LnUrlWithdrawResult.ok({
    required LnUrlWithdrawSuccessData data,
  }) = LnUrlWithdrawResult_Ok;
  const factory LnUrlWithdrawResult.timeout({
    required LnUrlWithdrawSuccessData data,
  }) = LnUrlWithdrawResult_Timeout;
  const factory LnUrlWithdrawResult.errorStatus({
    required LnUrlErrorData data,
  }) = LnUrlWithdrawResult_ErrorStatus;
}

class LnUrlWithdrawSuccessData {
  final LNInvoice invoice;

  const LnUrlWithdrawSuccessData({
    required this.invoice,
  });

  @override
  int get hashCode => invoice.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlWithdrawSuccessData && runtimeType == other.runtimeType && invoice == other.invoice;
}

class LocaleOverrides {
  final String locale;
  final int? spacing;
  final Symbol symbol;

  const LocaleOverrides({
    required this.locale,
    this.spacing,
    required this.symbol,
  });

  @override
  int get hashCode => locale.hashCode ^ spacing.hashCode ^ symbol.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LocaleOverrides &&
          runtimeType == other.runtimeType &&
          locale == other.locale &&
          spacing == other.spacing &&
          symbol == other.symbol;
}

class LocalizedName {
  final String locale;
  final String name;

  const LocalizedName({
    required this.locale,
    required this.name,
  });

  @override
  int get hashCode => locale.hashCode ^ name.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LocalizedName &&
          runtimeType == other.runtimeType &&
          locale == other.locale &&
          name == other.name;
}

class MessageSuccessActionData {
  final String message;

  const MessageSuccessActionData({
    required this.message,
  });

  @override
  int get hashCode => message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MessageSuccessActionData && runtimeType == other.runtimeType && message == other.message;
}

enum Network {
  Bitcoin,
  Testnet,
  Signet,
  Regtest,
  ;
}

class Rate {
  final String coin;
  final double value;

  const Rate({
    required this.coin,
    required this.value,
  });

  @override
  int get hashCode => coin.hashCode ^ value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Rate && runtimeType == other.runtimeType && coin == other.coin && value == other.value;
}

class RouteHint {
  final List<RouteHintHop> hops;

  const RouteHint({
    required this.hops,
  });

  @override
  int get hashCode => hops.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) || other is RouteHint && runtimeType == other.runtimeType && hops == other.hops;
}

class RouteHintHop {
  final String srcNodeId;
  final String shortChannelId;
  final int feesBaseMsat;
  final int feesProportionalMillionths;
  final BigInt cltvExpiryDelta;
  final BigInt? htlcMinimumMsat;
  final BigInt? htlcMaximumMsat;

  const RouteHintHop({
    required this.srcNodeId,
    required this.shortChannelId,
    required this.feesBaseMsat,
    required this.feesProportionalMillionths,
    required this.cltvExpiryDelta,
    this.htlcMinimumMsat,
    this.htlcMaximumMsat,
  });

  @override
  int get hashCode =>
      srcNodeId.hashCode ^
      shortChannelId.hashCode ^
      feesBaseMsat.hashCode ^
      feesProportionalMillionths.hashCode ^
      cltvExpiryDelta.hashCode ^
      htlcMinimumMsat.hashCode ^
      htlcMaximumMsat.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RouteHintHop &&
          runtimeType == other.runtimeType &&
          srcNodeId == other.srcNodeId &&
          shortChannelId == other.shortChannelId &&
          feesBaseMsat == other.feesBaseMsat &&
          feesProportionalMillionths == other.feesProportionalMillionths &&
          cltvExpiryDelta == other.cltvExpiryDelta &&
          htlcMinimumMsat == other.htlcMinimumMsat &&
          htlcMaximumMsat == other.htlcMaximumMsat;
}

@freezed
sealed class SuccessActionProcessed with _$SuccessActionProcessed {
  const SuccessActionProcessed._();

  const factory SuccessActionProcessed.aes({
    required AesSuccessActionDataResult result,
  }) = SuccessActionProcessed_Aes;
  const factory SuccessActionProcessed.message({
    required MessageSuccessActionData data,
  }) = SuccessActionProcessed_Message;
  const factory SuccessActionProcessed.url({
    required UrlSuccessActionData data,
  }) = SuccessActionProcessed_Url;
}

class Symbol {
  final String? grapheme;
  final String? template;
  final bool? rtl;
  final int? position;

  const Symbol({
    this.grapheme,
    this.template,
    this.rtl,
    this.position,
  });

  @override
  int get hashCode => grapheme.hashCode ^ template.hashCode ^ rtl.hashCode ^ position.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Symbol &&
          runtimeType == other.runtimeType &&
          grapheme == other.grapheme &&
          template == other.template &&
          rtl == other.rtl &&
          position == other.position;
}

class UrlSuccessActionData {
  final String description;
  final String url;
  final bool matchesCallbackDomain;

  const UrlSuccessActionData({
    required this.description,
    required this.url,
    required this.matchesCallbackDomain,
  });

  @override
  int get hashCode => description.hashCode ^ url.hashCode ^ matchesCallbackDomain.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UrlSuccessActionData &&
          runtimeType == other.runtimeType &&
          description == other.description &&
          url == other.url &&
          matchesCallbackDomain == other.matchesCallbackDomain;
}
