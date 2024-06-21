// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'frb_generated.dart';
import 'models.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

/// Details of supported LSP
class LspInformation {
  final String id;

  /// The name of of LSP
  final String name;

  /// The URL of the LSP
  final String widgetUrl;

  /// The identity pubkey of the Lightning node
  final String pubkey;

  /// The network location of the lightning node, e.g. `12.34.56.78:9012` or `localhost:10011`
  final String host;

  /// The base fee charged regardless of the number of milli-satoshis sent
  final PlatformInt64 baseFeeMsat;

  /// The effective fee rate in milli-satoshis. The precision of this value goes up to 6 decimal places, so 1e-6.
  final double feeRate;

  /// The required timelock delta for HTLCs forwarded over the channel
  final int timeLockDelta;

  /// The minimum value in millisatoshi we will require for incoming HTLCs on the channel
  final PlatformInt64 minHtlcMsat;
  final Uint8List lspPubkey;
  final OpeningFeeParamsMenu openingFeeParamsList;

  const LspInformation({
    required this.id,
    required this.name,
    required this.widgetUrl,
    required this.pubkey,
    required this.host,
    required this.baseFeeMsat,
    required this.feeRate,
    required this.timeLockDelta,
    required this.minHtlcMsat,
    required this.lspPubkey,
    required this.openingFeeParamsList,
  });

  @override
  int get hashCode =>
      id.hashCode ^
      name.hashCode ^
      widgetUrl.hashCode ^
      pubkey.hashCode ^
      host.hashCode ^
      baseFeeMsat.hashCode ^
      feeRate.hashCode ^
      timeLockDelta.hashCode ^
      minHtlcMsat.hashCode ^
      lspPubkey.hashCode ^
      openingFeeParamsList.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LspInformation &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          name == other.name &&
          widgetUrl == other.widgetUrl &&
          pubkey == other.pubkey &&
          host == other.host &&
          baseFeeMsat == other.baseFeeMsat &&
          feeRate == other.feeRate &&
          timeLockDelta == other.timeLockDelta &&
          minHtlcMsat == other.minHtlcMsat &&
          lspPubkey == other.lspPubkey &&
          openingFeeParamsList == other.openingFeeParamsList;
}
