import 'dart:async';
import 'dart:typed_data';

import 'package:breez_sdk/bridge_generated.dart';
import 'package:breez_sdk/native_toolkit.dart';
import 'package:fimber/fimber.dart';
import 'package:rxdart/rxdart.dart';

class BreezBridge {
  final _lnToolkit = getNativeToolkit();
  final _log = FimberLog("BreezBridge");

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

  BreezBridge() {
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
    });

    /// Listen to SDK logs and log them accordingly to their severity
    _lnToolkit.breezLogStream().listen(_registerToolkitLog);
  }

  /// Register a new node in the cloud and return credentials to interact with it
  ///
  /// # Arguments
  ///
  /// * `network` - The network type which is one of (Bitcoin, Testnet, Signet, Regtest)
  /// * `seed` - The node private key
  /// * `config` - The sdk configuration
  Future<GreenlightCredentials> registerNode({
    required Config config,
    required Network network,
    required Uint8List seed,
  }) async {
    var creds = await _lnToolkit.registerNode(
      config: config,
      network: network,
      seed: seed,
    );
    await fetchNodeData();
    await startNode();
    return creds;
  }

  /// Recover an existing node from the cloud and return credentials to interact with it
  ///
  /// # Arguments
  ///
  /// * `network` - The network type which is one of (Bitcoin, Testnet, Signet, Regtest)
  /// * `seed` - The node private key
  /// * `config` - The sdk configuration
  Future<GreenlightCredentials> recoverNode({
    required Config config,
    required Network network,
    required Uint8List seed,
  }) async {
    var creds = await _lnToolkit.recoverNode(
      config: config,
      network: network,
      seed: seed,
    );
    await fetchNodeData();
    await startNode();
    return creds;
  }

  /// init_services initialized the global NodeService, schedule the node to run in the cloud and
  /// run the signer. This must be called in order to start comunicate with the node
  ///
  /// # Arguments
  ///
  /// * `config` - The sdk configuration
  /// * `seed` - The node private key
  /// * `creds` - The greenlight credentials
  Future initServices({
    required Config config,
    required Uint8List seed,
    required GreenlightCredentials creds,
  }) async {
    await _lnToolkit.initServices(
      config: config,
      seed: seed,
      creds: creds,
    );
    await fetchNodeData();
    await startNode();
  }

  Future<void> startNode() async => await _lnToolkit.startNode();

  /// Cleanup node resources and stop the signer.
  Future<void> stopNode() async => await _lnToolkit.stopNode();

  /// pay a bolt11 invoice
  ///
  /// # Arguments
  ///
  /// * `bolt11` - The bolt11 invoice
  /// * `amountSats` - The amount to pay in satoshis
  Future sendPayment({required String bolt11, int? amountSats}) async {
    await _lnToolkit.sendPayment(bolt11: bolt11, amountSats: amountSats);
  }

  /// pay directly to a node id using keysend
  ///
  /// # Arguments
  ///
  /// * `nodeId` - The destination nodeId
  /// * `amountSats` - The amount to pay in satoshis
  Future sendSpontaneousPayment(
      {required String nodeId, required int amountSats}) async {
    await _lnToolkit.sendSpontaneousPayment(
        nodeId: nodeId, amountSats: amountSats);
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
  Future<LNInvoice> receivePayment(
          {required int amountSats, required String description}) async =>
      await _lnToolkit.receivePayment(
          amountSats: amountSats, description: description);

  /// get the node state from the persistent storage
  Future<NodeState?> getNodeState() async {
    final nodeState = await _lnToolkit.nodeInfo();
    nodeStateController.add(nodeState);
    return nodeState;
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

  /// List available lsps that can be selected by the user
  Future<List<LspInformation>> listLsps() async => await _lnToolkit.listLsps();

  /// Select the lsp to be used and provide inbound liquidity
  Future connectLSP(String lspId) async {
    await _lnToolkit.connectLsp(lspId: lspId);
  }

  /// Convenience method to look up LSP info
  Future<LspInformation?> fetchLspInfo(String lspId) async => await _lnToolkit.fetchLspInfo(id: lspId);

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
    required int feeRateSatsPerByte,
  }) async {
    await _lnToolkit.sweep(
      toAddress: toAddress,
      feeRateSatsPerByte: feeRateSatsPerByte,
    );
    await listPayments();
  }

  /// Onchain receive swap API
  Future<SwapInfo> receiveOnchain() async => await _lnToolkit.receiveOnchain();

  Future<List<SwapInfo>> listRefundables() async =>
      await _lnToolkit.listRefundables();

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

  Future<String> executeCommand({required String command}) =>
      _lnToolkit.executeCommand(command: command);

  Future<LNInvoice> parseInvoice(String invoice) async =>
      await _lnToolkit.parseInvoice(invoice: invoice);

  Future<InputType> parseInput({required String input}) async =>
      await _lnToolkit.parse(s: input);

  /// Second step of LNURL-pay. The first step is `parse()`, which also validates the LNURL destination
  /// and generates the `LnUrlPayRequestData` payload needed here.
  Future<LnUrlPayResult> payLnUrl({
    required int userAmountSat,
    String? comment,
    required LnUrlPayRequestData reqData,
  }) async {
    return _lnToolkit.payLnurl(
      userAmountSat: userAmountSat,
      comment: comment,
      reqData: reqData,
    );
  }

  /// Second step of LNURL-withdraw. The first step is `parse()`, which also validates the LNURL destination
  /// and generates the `LnUrlW` payload needed here.
  Future<LnUrlWithdrawCallbackStatus> withdrawLnurl({
    required int amountSats,
    String? description,
    required LnUrlWithdrawRequestData reqData,
  }) async {
    return _lnToolkit.withdrawLnurl(
        amountSats: amountSats, reqData: reqData, description: description);
  }

  /// Attempts to convert the phrase to a mnemonic, then to a seed.
  ///
  /// If the phrase is not a valid mnemonic, an error is returned.
  Future<Uint8List> mnemonicToSeed(String phrase) async =>
      await _lnToolkit.mnemonicToSeed(phrase: phrase);

  /// Fetches the current recommended fees
  Future<RecommendedFees> recommendedFees() => _lnToolkit.recommendedFees();

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
}
