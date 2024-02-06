import 'dart:async';
import 'dart:typed_data';

import 'package:breez_sdk/bridge_generated.dart';
import 'package:breez_sdk/exceptions.dart';
import 'package:breez_sdk/native_toolkit.dart';
import 'package:rxdart/rxdart.dart';

class BreezSDK {
  final _lnToolkit = getNativeToolkit();

  BreezSDK();

  /* Streams */
  /// Listen to paid Invoice events
  final StreamController<InvoicePaidDetails> _invoicePaidStream = StreamController.broadcast();

  Stream<InvoicePaidDetails> get invoicePaidStream => _invoicePaidStream.stream;

  /// Listen to payment results
  final StreamController<Payment> _paymentResultStream = StreamController.broadcast();

  Stream<Payment> get paymentResultStream => _paymentResultStream.stream;

  void initialize() {
    /// Listen to BreezEvent's(new block, invoice paid, synced)
    _lnToolkit.breezEventsStream().listen((event) async {
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
        _paymentResultStream.addError(PaymentException(event.details));
      }
      if (event is BreezEvent_BackupSucceeded) {
        _backupStreamController.add(event);
      }
      if (event is BreezEvent_BackupStarted) {
        _backupStreamController.add(event);
      }
      if (event is BreezEvent_BackupFailed) {
        _backupStreamController.addError(BackupException(event.details));
      }
    });
    _lnToolkit.breezLogStream().listen((logEntry) {
      _logStreamController.add(logEntry);
    }, onError: (e) {
      _logStreamController.addError(e);
    });
  }

  final _logStreamController = StreamController<LogEntry>.broadcast();

  /// Listen to log events
  Stream<LogEntry> get logStream => _logStreamController.stream;

  /* Breez Services API's & Streams*/

  /// Listen to node state
  final StreamController<NodeState?> nodeStateController = BehaviorSubject<NodeState?>();

  Stream<NodeState?> get nodeStateStream => nodeStateController.stream;

  /// Register for webhook callbacks at the given `webhook_url` whenever a new payment is received.
  ///
  /// More webhook types may be supported in the future.
  Future<void> registerWebhook({required String webhookUrl}) async {
    return _lnToolkit.registerWebhook(webhookUrl: webhookUrl);
  }

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

  /// Check whether node service is initialized or not
  Future<bool> isInitialized() async => await _lnToolkit.isInitialized();

  /// This method sync the local state with the remote node state.
  /// The synced items are as follows:
  /// * node state - General information about the node and its liquidity status
  /// * channels - The list of channels and their status
  /// * payments - The incoming/outgoing payments
  Future<void> sync() async => await _lnToolkit.sync();

  /// get the node state from the persistent storage
  Future<NodeState?> nodeInfo() async {
    final nodeState = await _lnToolkit.nodeInfo();
    nodeStateController.add(nodeState);
    return nodeState;
  }

  /// Cleanup node resources and stop the signer.
  Future<void> disconnect() async => await _lnToolkit.disconnect();

  /* Breez Services Helper API's */

  /// Attempts to convert the phrase to a mnemonic, then to a seed.
  ///
  /// If the phrase is not a valid mnemonic, an error is returned.
  Future<Uint8List> mnemonicToSeed(String phrase) async => await _lnToolkit.mnemonicToSeed(phrase: phrase);

  /// Get the full default config for a specific environment type
  Future<Config> defaultConfig({
    required EnvironmentType envType,
    required String apiKey,
    required NodeConfig nodeConfig,
  }) async {
    return await _lnToolkit.defaultConfig(
      envType: envType,
      apiKey: apiKey,
      nodeConfig: nodeConfig,
    );
  }

  /// Sign given message with the private key of the node id. Returns a zbase
  /// encoded signature.
  Future<SignMessageResponse> signMessage({
    required SignMessageRequest req,
  }) async {
    return await _lnToolkit.signMessage(req: req);
  }

  /// Check whether given message was signed by the private key or the given
  /// pubkey and the signature (zbase encoded) is valid.
  Future<CheckMessageResponse> checkMessage({
    required CheckMessageRequest req,
  }) async {
    return await _lnToolkit.checkMessage(req: req);
  }

  /* LSP API's */

  /// List available lsps that can be selected by the user
  Future<List<LspInformation>> listLsps() async => await _lnToolkit.listLsps();

  /// Select the lsp to be used and provide inbound liquidity
  Future<void> connectLSP(String lspId) async {
    return await _lnToolkit.connectLsp(lspId: lspId);
  }

  /// Get the current LSP's ID
  Future<String?> lspId() async => await _lnToolkit.lspId();

  /// Convenience method to look up LSP info based on current LSP ID
  Future<LspInformation?> lspInfo() async => await _lnToolkit.lspInfo();

  /// Convenience method to look up [LspInformation] for a given LSP ID
  Future<LspInformation?> fetchLspInfo(String lspId) async => await _lnToolkit.fetchLspInfo(id: lspId);

  /// close all channels with the current lsp
  Future closeLspChannels() async => await _lnToolkit.closeLspChannels();

  /* Backup API's & Streams*/

  // Listen to backup results
  final StreamController<BreezEvent?> _backupStreamController = BehaviorSubject<BreezEvent?>();

  Stream<BreezEvent?> get backupStream => _backupStreamController.stream;

  /// Start the backup process
  Future<void> backup() async => await _lnToolkit.backup();

  /// Returns the state of the backup process
  Future<BackupStatus> backupStatus() async => await _lnToolkit.backupStatus();

  /* Parse API's */

  /// Parse a BOLT11 payment request and return a structure contains the parsed fields.
  Future<LNInvoice> parseInvoice(String invoice) async => await _lnToolkit.parseInvoice(invoice: invoice);

  /// Parses generic user input, typically pasted from clipboard or scanned from a QR.
  Future<InputType> parseInput({required String input}) async => await _lnToolkit.parseInput(input: input);

  /// Get the static backup data.
  Future<StaticBackupResponse> staticBackup({
    required StaticBackupRequest req,
  }) async {
    return await _lnToolkit.staticBackup(req: req);
  }

  /* Payment API's & Streams*/

  /// Listen to payment list
  final StreamController<List<Payment>> paymentsController = BehaviorSubject<List<Payment>>();

  Stream<List<Payment>> get paymentsStream => paymentsController.stream;

  /// list payments (incoming/outgoing payments) from the persistent storage
  Future<List<Payment>> listPayments({
    required ListPaymentsRequest req,
  }) async {
    final paymentsList = await _lnToolkit.listPayments(req: req);
    paymentsController.add(paymentsList);
    return paymentsList;
  }

  /// Fetch a specific payment by its hash.
  Future<Payment?> paymentByHash({
    required String hash,
  }) async {
    return await _lnToolkit.paymentByHash(hash: hash);
  }

  /* Lightning Payment API's */

  /// pay a bolt11 invoice
  Future<SendPaymentResponse> sendPayment({
    required SendPaymentRequest req,
  }) async {
    return await _lnToolkit.sendPayment(req: req);
  }

  /// pay directly to a node id using keysend
  Future<SendPaymentResponse> sendSpontaneousPayment({
    required SendSpontaneousPaymentRequest req,
  }) async {
    return await _lnToolkit.sendSpontaneousPayment(req: req);
  }

  /// Creates an bolt11 payment request.
  /// This also works when the node doesn't have any channels and need inbound liquidity.
  /// In such case when the invoice is paid a new zero-conf channel will be open by the LSP,
  /// providing inbound liquidity and the payment will be routed via this new channel.
  Future<ReceivePaymentResponse> receivePayment({
    required ReceivePaymentRequest req,
  }) async {
    return await _lnToolkit.receivePayment(req: req);
  }

  /* LNURL API's */

  /// Second step of LNURL-pay. The first step is `parse()`, which also validates the LNURL destination
  /// and generates the `LnUrlPayRequestData` payload needed here.
  Future<LnUrlPayResult> lnurlPay({
    required LnUrlPayRequest req,
  }) async {
    return await _lnToolkit.lnurlPay(req: req);
  }

  /// Second step of LNURL-withdraw. The first step is `parse()`, which also validates the LNURL destination
  /// and generates the `LnUrlWithdrawRequestData` payload needed here.
  ///
  /// This call will validate the given amount in the `request` against the parameters
  /// of the LNURL endpoint data in the `request`. If they match the endpoint requirements, the LNURL withdraw
  /// request is made. A successful result here means the endpoint started the payment.
  Future<LnUrlWithdrawResult> lnurlWithdraw({required LnUrlWithdrawRequest req}) async {
    return await _lnToolkit.lnurlWithdraw(req: req);
  }

  /// Third and last step of LNURL-auth. The first step is `parse()`, which also validates the LNURL destination
  /// and generates the `LnUrlAuthRequestData` payload needed here. The second step is user approval of auth action.
  ///
  /// This call will sign `k1` of the LNURL endpoint (`req_data`) on `secp256k1` using `linkingPrivKey` and DER-encodes the signature.
  /// If they match the endpoint requirements, the LNURL auth request is made. A successful result here means the client signature is verified.
  Future<LnUrlCallbackStatus> lnurlAuth({
    required LnUrlAuthRequestData reqData,
  }) async {
    return await _lnToolkit.lnurlAuth(reqData: reqData);
  }

  /* Fiat Currency API's */

  /// Fetch live rates of fiat currencies
  Future<Map<String, Rate>> fetchFiatRates() async {
    final List<Rate> rates = await _lnToolkit.fetchFiatRates();
    return rates.fold<Map<String, Rate>>({}, (map, rate) {
      map[rate.coin] = rate;
      return map;
    });
  }

  /// List all available fiat currencies
  Future<List<FiatCurrency>> listFiatCurrencies() async => await _lnToolkit.listFiatCurrencies();

  /* On-Chain Swap API's */

  /// Creates a reverse swap and attempts to pay the HODL invoice
  Future<SendOnchainResponse> sendOnchain({
    required SendOnchainRequest req,
  }) async {
    return await _lnToolkit.sendOnchain(req: req);
  }

  /// Onchain receive swap API
  Future<SwapInfo> receiveOnchain({
    required ReceiveOnchainRequest req,
  }) async {
    return await _lnToolkit.receiveOnchain(req: req);
  }

  /// Generates an url that can be used by a third part provider to buy Bitcoin with fiat currency
  Future<BuyBitcoinResponse> buyBitcoin({
    required BuyBitcoinRequest req,
  }) async {
    return await _lnToolkit.buyBitcoin(req: req);
  }

  /// Withdraw on-chain funds in the wallet to an external btc address
  Future<RedeemOnchainFundsResponse> redeemOnchainFunds({
    required RedeemOnchainFundsRequest req,
  }) async {
    final redeemOnchainFundsResponse = await _lnToolkit.redeemOnchainFunds(req: req);
    await listPayments(req: const ListPaymentsRequest());
    return redeemOnchainFundsResponse;
  }

  /// Returns the max amount that can be sent on-chain using the send_onchain method.
  /// The returned amount is the sum of the max amount that can be sent on each channel
  /// minus the expected fees.
  /// This is possible since the route to the swapper node is known in advance and is expected
  /// to consist of maximum 3 hops.
  Future<MaxReverseSwapAmountResponse> maxReverseSwapAmount() async {
    return await _lnToolkit.maxReverseSwapAmount();
  }

  /* Refundables API's */

  /// list non-completed expired swaps that should be refunded by calling refund()
  Future<List<SwapInfo>> listRefundables() async => await _lnToolkit.listRefundables();

  /// Construct and broadcast a refund transaction for a failed/expired swap
  Future<RefundResponse> refund({
    required RefundRequest req,
  }) async {
    return await _lnToolkit.refund(req: req);
  }

  /// Prepares a refund transaction for a failed/expired swap.
  ///
  /// Can optionally be used before refund to know how much fees will be paid
  /// to perform the refund.
  Future<PrepareRefundResponse> prepareRefund({
    required PrepareRefundRequest req,
  }) async {
    return await _lnToolkit.prepareRefund(req: req);
  }

  /* In Progress Swap API's */

  /// Returns an optional in-progress [SwapInfo].
  /// A [SwapInfo] is in-progress if it is waiting for confirmation to be redeemed and complete the swap.
  Future<SwapInfo?> inProgressSwap() async => await _lnToolkit.inProgressSwap();

  /// Returns the blocking [ReverseSwapInfo]s that are in progress
  Future<List<ReverseSwapInfo>> inProgressReverseSwaps() async => _lnToolkit.inProgressReverseSwaps();

  /* Swap Fee API's */

  /// Gets the fees required to open a channel for a given amount.
  Future<OpenChannelFeeResponse> openChannelFee({
    required OpenChannelFeeRequest req,
  }) async {
    return await _lnToolkit.openChannelFee(req: req);
  }

  /// Lookup the most recent reverse swap pair info using the Boltz API
  Future<ReverseSwapPairInfo> fetchReverseSwapFees({
    required ReverseSwapFeesRequest req,
  }) async {
    return await _lnToolkit.fetchReverseSwapFees(req: req);
  }

  /// Fetches the current recommended fees
  Future<RecommendedFees> recommendedFees() async => await _lnToolkit.recommendedFees();

  Future<PrepareRedeemOnchainFundsResponse> prepareRedeemOnchainFunds({
    required PrepareRedeemOnchainFundsRequest req,
  }) async =>
      _lnToolkit.prepareRedeemOnchainFunds(req: req);

  /* Support API's */

  /// Send an issue report using the Support API.
  /// - `ReportIssueRequest.paymentFailure` sends a payment failure report to the Support API
  ///   using the provided `paymentHash` to lookup the failed `Payment` and the current `NodeState`.
  Future<void> reportIssue({
    required ReportIssueRequest req,
  }) async {
    return await _lnToolkit.reportIssue(req: req);
  }

  /// Fetches the service health check from the support API.
  Future<ServiceHealthCheckResponse> serviceHealthCheck() async => await _lnToolkit.serviceHealthCheck();

  /* CLI API's */

  /// Execute a command directly on the NodeAPI interface.
  /// Mainly used to debugging.
  Future<String> executeCommand({required String command}) async {
    return await _lnToolkit.executeCommand(command: command);
  }

  /* Helper Methods */

  /// Validate if given address is a valid BTC address
  Future<bool> isValidBitcoinAddress(
    String address,
  ) async {
    try {
      final inputType = await _lnToolkit.parseInput(input: address);
      return inputType is InputType_BitcoinAddress;
    } catch (e) {
      return false;
    }
  }

  /// Fetch node state & payment list
  Future fetchNodeData() async {
    await nodeInfo();
    await listPayments(req: const ListPaymentsRequest());
  }
}

extension SDKConfig on Config {
  Config copyWith({
    String? breezserver,
    String? chainnotifierUrl,
    String? mempoolspaceUrl,
    String? workingDir,
    Network? network,
    int? paymentTimeoutSec,
    String? defaultLspId,
    String? apiKey,
    double? maxfeePercent,
    int? exemptfeeMsat,
    NodeConfig? nodeConfig,
  }) {
    return Config(
      breezserver: breezserver ?? this.breezserver,
      chainnotifierUrl: chainnotifierUrl ?? this.chainnotifierUrl,
      mempoolspaceUrl: mempoolspaceUrl ?? this.mempoolspaceUrl,
      workingDir: workingDir ?? this.workingDir,
      network: network ?? this.network,
      paymentTimeoutSec: paymentTimeoutSec ?? this.paymentTimeoutSec,
      defaultLspId: defaultLspId ?? this.defaultLspId,
      apiKey: apiKey ?? this.apiKey,
      maxfeePercent: maxfeePercent ?? this.maxfeePercent,
      exemptfeeMsat: exemptfeeMsat ?? this.exemptfeeMsat,
      nodeConfig: nodeConfig ?? this.nodeConfig,
    );
  }
}
