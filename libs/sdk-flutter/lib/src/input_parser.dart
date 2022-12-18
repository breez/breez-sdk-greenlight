import 'package:breez_sdk/bridge_generated.dart';
import 'package:dart_lnurl/dart_lnurl.dart';
import 'package:url_launcher/url_launcher_string.dart';
import 'package:validators/validators.dart';

import '../native_toolkit.dart';

const NODE_URI_SEPARATOR = "@";
const NODE_ID_LENGTH = 66;

class InputParser {
  final BreezSdkCore _lnToolkit = getNativeToolkit();
  static RegExp lnurlPrefix = RegExp(",*?((lnurl)([0-9]{1,}[a-z0-9]+){1})");
  static RegExp lnurlRfc17Prefix = RegExp("(lnurl)(c|w|p)");

  Future<ParsedInput> parse(String s) async {
    // lnurl
    try {
      LNURLParseResult parseResult = await getParams(s);
      if (parseResult.payParams != null ||
          parseResult.withdrawalParams != null) {
        return ParsedInput(InputProtocol.lnurl, parseResult);
      }
    } catch (error) {
      // do nothing
    }

    // lightning link
    String lower = s.toLowerCase();
    if (lower.startsWith('lightning:')) {
      final invoice = await _lnToolkit.parseInvoice(invoice: s.substring(10));
      return ParsedInput(InputProtocol.paymentRequest, invoice);
    }

    // bolt 11 lightning
    String? bolt11 = _extractBolt11FromBip21(s);
    if (bolt11 != null) {
      final invoice = await _lnToolkit.parseInvoice(invoice: bolt11);
      return ParsedInput(InputProtocol.paymentRequest, invoice);
    }
    try {
      final invoice = await _lnToolkit.parseInvoice(invoice: lower);
      return ParsedInput(InputProtocol.paymentRequest, invoice);
    } catch (e) {
      // do nothing
    }

    // nodeID
    String? nodeID = _parseNodeID(s);
    if (nodeID != null) {
      return ParsedInput(InputProtocol.nodeID, nodeID);
    }

    // Open on whichever app the system links to
    if (await canLaunchUrlString(s)) {
      return ParsedInput(InputProtocol.appLink, s);
    }

    // Open on browser
    bool validUrl = isURL(s, requireProtocol: true, allowUnderscore: true);
    if (validUrl) {
      return ParsedInput(InputProtocol.webView, s);
    }

    throw Exception("not implemented");
  }
}

String? _extractBolt11FromBip21(String bip21) {
  String lowerBip21 = bip21.toLowerCase();
  if (lowerBip21.startsWith("bitcoin:")) {
    try {
      Uri uri = Uri.parse(lowerBip21);
      String? bolt11 = uri.queryParameters["lightning"];
      if (bolt11 != null && bolt11.isNotEmpty) {
        return bolt11;
      }
    } on FormatException {
      // do nothing.
    }
  }
  return null;
}

String? _parseNodeID(String? nodeID) {
  if (nodeID == null) {
    return null;
  }
  if (nodeID.length == NODE_ID_LENGTH) {
    return nodeID;
  }

  if (nodeID.length > NODE_ID_LENGTH &&
      nodeID.substring(NODE_ID_LENGTH, NODE_ID_LENGTH + 1) ==
          NODE_URI_SEPARATOR) {
    return nodeID.substring(0, 66);
  }

  return null;
}

enum InputProtocol { paymentRequest, lnurl, nodeID, appLink, webView }

class ParsedInput {
  final InputProtocol protocol;
  final dynamic decoded;

  ParsedInput(this.protocol, this.decoded);
}