// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.33.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'frb_generated.dart';
import 'invoice.dart';
import 'models.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'input_parser.freezed.dart';

/// Wrapped in a [BitcoinAddress], this is the result of [parse] when given a plain or BIP-21 BTC address.
class BitcoinAddressData {
  final String address;
  final Network network;
  final int? amountSat;
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

@freezed
sealed class InputType with _$InputType {
  const InputType._();

  /// # Supported standards
  ///
  /// - plain on-chain BTC address
  /// - BIP21
  const factory InputType.bitcoinAddress({
    required BitcoinAddressData address,
  }) = InputType_BitcoinAddress;

  /// Also covers URIs like `bitcoin:...&lightning=bolt11`. In this case, it returns the BOLT11
  /// and discards all other data.
  const factory InputType.bolt11({
    required LNInvoice invoice,
  }) = InputType_Bolt11;
  const factory InputType.nodeId({
    required String nodeId,
  }) = InputType_NodeId;
  const factory InputType.url({
    required String url,
  }) = InputType_Url;

  /// # Supported standards
  ///
  /// - LUD-01 LNURL bech32 encoding
  /// - LUD-06 `payRequest` spec
  /// - LUD-16 LN Address
  /// - LUD-17 Support for lnurlp prefix with non-bech32-encoded LNURL URLs
  const factory InputType.lnUrlPay({
    required LnUrlPayRequestData data,
  }) = InputType_LnUrlPay;

  /// # Supported standards
  ///
  /// - LUD-01 LNURL bech32 encoding
  /// - LUD-03 `withdrawRequest` spec
  /// - LUD-17 Support for lnurlw prefix with non-bech32-encoded LNURL URLs
  ///
  /// # Not supported (yet)
  ///
  /// - LUD-14 `balanceCheck`: reusable `withdrawRequest`s
  /// - LUD-19 Pay link discoverable from withdraw link
  const factory InputType.lnUrlWithdraw({
    required LnUrlWithdrawRequestData data,
  }) = InputType_LnUrlWithdraw;

  /// # Supported standards
  ///
  /// - LUD-01 LNURL bech32 encoding
  /// - LUD-04 `auth` base spec
  /// - LUD-17 Support for keyauth prefix with non-bech32-encoded LNURL URLs
  const factory InputType.lnUrlAuth({
    required LnUrlAuthRequestData data,
  }) = InputType_LnUrlAuth;
  const factory InputType.lnUrlError({
    required LnUrlErrorData data,
  }) = InputType_LnUrlError;
}

/// Wrapped in a [LnUrlAuth], this is the result of [parse] when given a LNURL-auth endpoint.
///
/// It represents the endpoint's parameters for the LNURL workflow.
///
/// See <https://github.com/lnurl/luds/blob/luds/04.md>
class LnUrlAuthRequestData {
  /// Hex encoded 32 bytes of challenge
  final String k1;

  /// When available, one of: register, login, link, auth
  final String? action;

  /// Indicates the domain of the LNURL-auth service, to be shown to the user when asking for
  /// auth confirmation, as per LUD-04 spec.
  final String domain;

  /// Indicates the URL of the LNURL-auth service, including the query arguments. This will be
  /// extended with the signed challenge and the linking key, then called in the second step of the workflow.
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

/// Wrapped in a [LnUrlError], this represents a LNURL-endpoint error.
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

/// Wrapped in a [LnUrlPay], this is the result of [parse] when given a LNURL-pay endpoint.
///
/// It represents the endpoint's parameters for the LNURL workflow.
///
/// See <https://github.com/lnurl/luds/blob/luds/06.md>
class LnUrlPayRequestData {
  final String callback;

  /// The minimum amount, in millisats, that this LNURL-pay endpoint accepts
  final int minSendable;

  /// The maximum amount, in millisats, that this LNURL-pay endpoint accepts
  final int maxSendable;

  /// As per LUD-06, `metadata` is a raw string (e.g. a json representation of the inner map).
  /// Use `metadata_vec()` to get the parsed items.
  final String metadataStr;

  /// The comment length accepted by this endpoint
  ///
  /// See <https://github.com/lnurl/luds/blob/luds/12.md>
  final int commentAllowed;

  /// Indicates the domain of the LNURL-pay service, to be shown to the user when asking for
  /// payment input, as per LUD-06 spec.
  ///
  /// Note: this is not the domain of the callback, but the domain of the LNURL-pay endpoint.
  final String domain;

  /// Value indicating whether the recipient supports Nostr Zaps through NIP-57.
  ///
  /// See <https://github.com/nostr-protocol/nips/blob/master/57.md>
  final bool allowsNostr;

  /// Optional recipient's lnurl provider's Nostr pubkey for NIP-57. If it exists it should be a
  /// valid BIP 340 public key in hex.
  ///
  /// See <https://github.com/nostr-protocol/nips/blob/master/57.md>
  /// See <https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki>
  final String? nostrPubkey;

  /// If sending to a LN Address, this will be filled.
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

/// Wrapped in a [LnUrlWithdraw], this is the result of [parse] when given a LNURL-withdraw endpoint.
///
/// It represents the endpoint's parameters for the LNURL workflow.
///
/// See <https://github.com/lnurl/luds/blob/luds/03.md>
class LnUrlWithdrawRequestData {
  final String callback;
  final String k1;
  final String defaultDescription;

  /// The minimum amount, in millisats, that this LNURL-withdraw endpoint accepts
  final int minWithdrawable;

  /// The maximum amount, in millisats, that this LNURL-withdraw endpoint accepts
  final int maxWithdrawable;

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
