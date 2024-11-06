// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.6.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../binding.dart';
import '../frb_generated.dart';
import '../models.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'pay.freezed.dart';

@freezed
sealed class LnUrlPayResult with _$LnUrlPayResult {
  const LnUrlPayResult._();

  const factory LnUrlPayResult.endpointSuccess({
    required LnUrlPaySuccessData data,
  }) = LnUrlPayResult_EndpointSuccess;
  const factory LnUrlPayResult.endpointError({
    required LnUrlErrorData data,
  }) = LnUrlPayResult_EndpointError;
  const factory LnUrlPayResult.payError({
    required LnUrlPayErrorData data,
  }) = LnUrlPayResult_PayError;
}

class LnUrlPaySuccessData {
  final Payment payment;
  final SuccessActionProcessed? successAction;

  const LnUrlPaySuccessData({
    required this.payment,
    this.successAction,
  });

  @override
  int get hashCode => payment.hashCode ^ successAction.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LnUrlPaySuccessData &&
          runtimeType == other.runtimeType &&
          payment == other.payment &&
          successAction == other.successAction;
}
