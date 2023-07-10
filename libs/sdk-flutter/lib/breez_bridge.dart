import 'dart:async';
import 'dart:typed_data';

import 'package:breez_sdk/bridge_generated.dart';
import 'package:breez_sdk/native_toolkit.dart';
import 'package:fimber/fimber.dart';
import 'package:rxdart/rxdart.dart';

class BreezBridge {
  final _lnToolkit = getNativeToolkit();
  final _log = FimberLog("BreezBridge");

  BreezBridge();

  /* Streams */
  /// Listen to node state
  final StreamController<NodeState?> nodeStateController =
      BehaviorSubject<NodeState?>();

  Stream<NodeState?> get nodeStateStream => nodeStateController.stream;

  /// Listen to payment list
  final StreamController<List<Payment>> paymentsController =
      BehaviorSubject<List<Payment>>();

  Stream<List<Payment>> get paymentsStream => paymentsController.stream;

  /// Listen to paid Invoice events
  final StreamController<InvoicePaidDetails> _invoicePaidStream =
      BehaviorSubject<InvoicePaidDetails>();

  Stream<InvoicePaidDetails> get invoicePaidStream => _invoicePaidStream.stream;

  /// Listen to payment results
  final StreamController<Payment> _paymentResultStream =
      BehaviorSubject<Payment>();

  Stream<Payment> get paymentResultStream => _paymentResultStream.stream;

  // Listen to backup results
  final StreamController<BreezEvent?> _backupStreamController =
      BehaviorSubject<BreezEvent?>();

  Stream<BreezEvent?> get backupStream => _backupStreamController.stream;

  void initialize() {
    /// Listen to BreezEvent's(new block, invoice paid, synced)
    _lnToolkit.breezEventsStream().listen((event) async {
      _log.v("Received breez event: $event");
      if (event is BreezEvent_InvoicePaid) {
        _invoicePaidStream.add(event.details);
        await fetchNodeData();
      }
      if (event is BreezEvent_Synced) {
        await fetchNodeData();
      }
      if (event is BreezEvent_PaymentSucceed) {
        _paymentResultStream.add(event.details);
      }
      if (event is BreezEvent_PaymentFailed) {
        _paymentResultStream.addError(Exception(event.details.error));
      }
      if (event is BreezEvent_BackupSucceeded) {
        _backupStreamController.add(event);
      }
      if (event is BreezEvent_BackupStarted) {
        _backupStreamController.add(event);
      }
      if (event is BreezEvent_BackupFailed) {
        _backupStreamController.addError(Exception(event.details.error));
      }
    });

    /// Listen to SDK logs and log them accordingly to their severity
    _lnToolkit.breezLogStream().listen(_registerToolkitLog);
  }

  /// Check whether node service is initialized or not
  Future<bool> isInitialized() async => await _lnToolkit.initialized();

  /// connect initializes the global NodeService, schedule the node to run in the cloud and
  /// run the signer. This must be called in order to start communicate with the node
  ///
  /// # Arguments
  ///
  /// * `config` - The sdk configuration
  /// * `seed` - The node private key
  Future connect({
    required Config config,
    required Uint8List seed,
  }) async {
    await _lnToolkit.connect(
      config: config,
      seed: seed,
    );
    await fetchNodeData();
  }

  /// Cleanup node resources and stop the signer.
  Future<void> stopNode() async => await _lnToolkit.stopNode();

  /// This method sync the local state with the remote node state.
  /// The synced items are as follows:
  /// * node state - General information about the node and its liquidity status
  /// * channels - The list of channels and their status
  /// * payments - The incoming/outgoing payments
  Future<void> syncNode() async => await _lnToolkit.syncNode();

  /// pay a bolt11 invoice
  ///
  /// # Arguments
  ///
  /// * `bolt11` - The bolt11 invoice
  /// * `amountSats` - The amount to pay in satoshis
  Future<Payment> sendPayment({
    required String bolt11,
    int? amountSats,
  }) async {
    return await _lnToolkit.sendPayment(
      bolt11: bolt11,
      amountSats: amountSats,
    );
  }

  /// pay directly to a node id using keysend
  ///
  /// # Arguments
  ///
  /// * `nodeId` - The destination nodeId
  /// * `amountSats` - The amount to pay in satoshis
  Future<Payment> sendSpontaneousPayment({
    required String nodeId,
    required int amountSats,
  }) async {
    return await _lnToolkit.sendSpontaneousPayment(
      nodeId: nodeId,
      amountSats: amountSats,
    );
  }

  /// Creates an bolt11 payment request.
  /// This also works when the node doesn't have any channels and need inbound liquidity.
  /// In such case when the invoice is paid a new zero-conf channel will be open by the LSP,
  /// providing inbound liquidity and the payment will be routed via this new channel.
  ///
  /// # Arguments
  ///
  /// * `amountSats` - The amount to receive in satoshis
  /// * `description` - The bolt11 payment request description
  Future<LNInvoice> receivePayment({
    required int amountSats,
    required String description,
  }) async =>
      await _lnToolkit.receivePayment(
        amountSats: amountSats,
        description: description,
      );

  /// get the node state from the persistent storage
  Future<NodeState?> getNodeState() async {
    final nodeState = await _lnToolkit.nodeInfo();
    nodeStateController.add(nodeState);
    return nodeState;
  }

  /// Get the full default config for a specific environment type
  Future<Config> defaultConfig({
    required EnvironmentType envType,
    required String apiKey,
    required NodeConfig nodeConfig,
  }) {
    return _lnToolkit.defaultConfig(
      envType: envType,
      apiKey: apiKey,
      nodeConfig: nodeConfig,
    );
  }

  /// list payments (incoming/outgoing payments) from the persistent storage
  Future<List<Payment>> listPayments({
    PaymentTypeFilter filter = PaymentTypeFilter.All,
    int? fromTimestamp,
    int? toTimestamp,
  }) async {
    var paymentsList = await _lnToolkit.listPayments(
      filter: filter,
      fromTimestamp: fromTimestamp,
      toTimestamp: toTimestamp,
    );
    paymentsController.add(paymentsList);
    return paymentsList;
  }

  /// Fetch a specific payment by its hash.
  Future<Payment?> paymentByHash({
    required String hash,
  }) async =>
      await _lnToolkit.paymentByHash(
        hash: hash,
      );

  /// List available lsps that can be selected by the user
  Future<List<LspInformation>> listLsps() async => await _lnToolkit.listLsps();

  /// Select the lsp to be used and provide inbound liquidity
  Future connectLSP(String lspId) async {
    await _lnToolkit.connectLsp(lspId: lspId);
  }

  /// Convenience method to look up LSP info
  Future<LspInformation?> fetchLspInfo(String lspId) async =>
      await _lnToolkit.fetchLspInfo(id: lspId);

  /// Get the current LSP's ID
  Future<String?> getLspId() async => await _lnToolkit.lspId();

  /// Fetch live rates of fiat currencies
  Future<Map<String, Rate>> fetchFiatRates() async {
    final List<Rate> rates = await _lnToolkit.fetchFiatRates();
    return rates.fold<Map<String, Rate>>({}, (map, rate) {
      map[rate.coin] = rate;
      return map;
    });
  }

  /// List all available fiat currencies
  Future<List<FiatCurrency>> listFiatCurrencies() async =>
      await _lnToolkit.listFiatCurrencies();

  /// close all channels with the current lsp
  Future closeLspChannels() async => await _lnToolkit.closeLspChannels();

  /// Withdraw on-chain funds in the wallet to an external btc address
  Future sweep({
    required String toAddress,
    required int feeRateSatsPerVbyte,
  }) async {
    await _lnToolkit.sweep(
      toAddress: toAddress,
      feeRateSatsPerVbyte: feeRateSatsPerVbyte,
    );
    await listPayments();
  }

  /// Onchain receive swap API
  Future<SwapInfo> receiveOnchain() async => await _lnToolkit.receiveOnchain();

  /// Returns the blocking [ReverseSwapInfo]s that are in progress
  Future<SwapInfo?> inProgressSwap() async => await _lnToolkit.inProgressSwap();

  /// list non-completed expired swaps that should be refunded by calling refund()
  Future<List<SwapInfo>> listRefundables() async =>
      await _lnToolkit.listRefundables();

  /// Construct and broadcast a refund transaction for a failed/expired swap
  Future<String> refund({
    required String swapAddress,
    required String toAddress,
    required int satPerVbyte,
  }) async =>
      await _lnToolkit.refund(
        swapAddress: swapAddress,
        toAddress: toAddress,
        satPerVbyte: satPerVbyte,
      );

  /// Lookup the most recent reverse swap pair info using the Boltz API
  Future<ReverseSwapPairInfo> fetchReverseSwapFees() async =>
      _lnToolkit.fetchReverseSwapFees();

  /// Returns the blocking [ReverseSwapInfo]s that are in progress
  Future<List<ReverseSwapInfo>> inProgressReverseSwaps() async =>
      _lnToolkit.inProgressReverseSwaps();

  /// Creates a reverse swap and attempts to pay the HODL invoice
  Future<ReverseSwapInfo> sendOnchain({
    required int amountSat,
    required String onchainRecipientAddress,
    required String pairHash,
    required int satPerVbyte,
  }) async =>
      _lnToolkit.sendOnchain(
        amountSat: amountSat,
        onchainRecipientAddress: onchainRecipientAddress,
        pairHash: pairHash,
        satPerVbyte: satPerVbyte,
      );

  /// Execute a command directly on the NodeAPI interface.
  /// Mainly used to debugging.
  Future<String> executeCommand({required String command}) async =>
      _lnToolkit.executeCommand(command: command);

  /// Parse a BOLT11 payment request and return a structure contains the parsed fields.
  Future<LNInvoice> parseInvoice(String invoice) async =>
      await _lnToolkit.parseInvoice(invoice: invoice);

  /// Parses generic user input, typically pasted from clipboard or scanned from a QR.
  Future<InputType> parseInput({required String input}) async =>
      await _lnToolkit.parse(s: input);

  /// Second step of LNURL-pay. The first step is `parse()`, which also validates the LNURL destination
  /// and generates the `LnUrlPayRequestData` payload needed here.
  Future<LnUrlPayResult> lnurlPay({
    required int userAmountSat,
    String? comment,
    required LnUrlPayRequestData reqData,
  }) async {
    return _lnToolkit.lnurlPay(
      userAmountSat: userAmountSat,
      comment: comment,
      reqData: reqData,
    );
  }

  /// Second step of LNURL-withdraw. The first step is `parse()`, which also validates the LNURL destination
  /// and generates the `LnUrlWithdrawRequestData` payload needed here.
  ///
  /// This call will validate the given `amount_sats` against the parameters
  /// of the LNURL endpoint (`req_data`). If they match the endpoint requirements, the LNURL withdraw
  /// request is made. A successful result here means the endpoint started the payment.
  Future<LnUrlCallbackStatus> lnurlWithdraw({
    required int amountSats,
    String? description,
    required LnUrlWithdrawRequestData reqData,
  }) async {
    return _lnToolkit.lnurlWithdraw(
      amountSats: amountSats,
      reqData: reqData,
      description: description,
    );
  }

  /// Third and last step of LNURL-auth. The first step is `parse()`, which also validates the LNURL destination
  /// and generates the `LnUrlAuthRequestData` payload needed here. The second step is user approval of auth action.
  ///
  /// This call will sign `k1` of the LNURL endpoint (`req_data`) on `secp256k1` using `linkingPrivKey` and DER-encodes the signature.
  /// If they match the endpoint requirements, the LNURL auth request is made. A successful result here means the client signature is verified.
  Future<LnUrlCallbackStatus> lnurlAuth({
    required LnUrlAuthRequestData reqData,
  }) async {
    return _lnToolkit.lnurlAuth(
      reqData: reqData,
    );
  }

  /// Attempts to convert the phrase to a mnemonic, then to a seed.
  ///
  /// If the phrase is not a valid mnemonic, an error is returned.
  Future<Uint8List> mnemonicToSeed(String phrase) async =>
      await _lnToolkit.mnemonicToSeed(phrase: phrase);

  /// Fetches the current recommended fees
  Future<RecommendedFees> recommendedFees() => _lnToolkit.recommendedFees();

  /// Start the backup process
  Future<void> backup() => _lnToolkit.backup();

  /// Returns the state of the backup process
  Future<BackupStatus> backupStatus() => _lnToolkit.backupStatus();

  /* Helper Methods */
  /// Validate if given address is a valid BTC address
  Future<bool> isValidBitcoinAddress(
    String address,
  ) async {
    try {
      final inputType = await _lnToolkit.parse(s: address);
      return inputType is InputType_BitcoinAddress;
    } catch (e) {
      return false;
    }
  }

  /// Log entries according to their severity
  void _registerToolkitLog(LogEntry log) {
    switch (log.level) {
      case "ERROR":
        _log.e(log.line);
        break;
      case "WARN":
        _log.w(log.line);
        break;
      case "INFO":
        _log.i(log.line);
        break;
      case "DEBUG":
        _log.d(log.line);
        break;
      default:
        _log.v(log.line);
        break;
    }
  }

  /// Fetch node state & payment list
  Future fetchNodeData() async {
    await getNodeState();
    await listPayments();
  }

  /// Generates an url that can be used by a third part provider to buy Bitcoin with fiat currency
  Future<String> buyBitcoin(BuyBitcoinProvider provider) =>
      _lnToolkit.buyBitcoin(provider: provider);
}

extension SDKConfig on Config {
  Config copyWith({
    String? breezserver,
    String? mempoolspaceUrl,
    String? workingDir,
    Network? network,
    int? paymentTimeoutSec,
    String? defaultLspId,
    String? apiKey,
    double? maxfeePercent,
    NodeConfig? nodeConfig,
  }) {
    return Config(
      breezserver: breezserver ?? this.breezserver,
      mempoolspaceUrl: mempoolspaceUrl ?? this.mempoolspaceUrl,
      workingDir: workingDir ?? this.workingDir,
      network: network ?? this.network,
      paymentTimeoutSec: paymentTimeoutSec ?? this.paymentTimeoutSec,
      defaultLspId: defaultLspId ?? this.defaultLspId,
      apiKey: apiKey ?? this.apiKey,
      maxfeePercent: maxfeePercent ?? this.maxfeePercent,
      nodeConfig: nodeConfig ?? this.nodeConfig,
    );
  }
}
