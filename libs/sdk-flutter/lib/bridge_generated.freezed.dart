// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'bridge_generated.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$AesSuccessActionDataResult {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataDecrypted data) decrypted,
    required TResult Function(String reason) errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataDecrypted data)? decrypted,
    TResult? Function(String reason)? errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(AesSuccessActionDataDecrypted data)? decrypted,
    TResult Function(String reason)? errorStatus,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataResult_Decrypted value) decrypted,
    required TResult Function(AesSuccessActionDataResult_ErrorStatus value) errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataResult_Decrypted value)? decrypted,
    TResult? Function(AesSuccessActionDataResult_ErrorStatus value)? errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AesSuccessActionDataResult_Decrypted value)? decrypted,
    TResult Function(AesSuccessActionDataResult_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AesSuccessActionDataResultCopyWith<$Res> {
  factory $AesSuccessActionDataResultCopyWith(
          AesSuccessActionDataResult value, $Res Function(AesSuccessActionDataResult) then) =
      _$AesSuccessActionDataResultCopyWithImpl<$Res, AesSuccessActionDataResult>;
}

/// @nodoc
class _$AesSuccessActionDataResultCopyWithImpl<$Res, $Val extends AesSuccessActionDataResult>
    implements $AesSuccessActionDataResultCopyWith<$Res> {
  _$AesSuccessActionDataResultCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$AesSuccessActionDataResult_DecryptedImplCopyWith<$Res> {
  factory _$$AesSuccessActionDataResult_DecryptedImplCopyWith(
          _$AesSuccessActionDataResult_DecryptedImpl value,
          $Res Function(_$AesSuccessActionDataResult_DecryptedImpl) then) =
      __$$AesSuccessActionDataResult_DecryptedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({AesSuccessActionDataDecrypted data});
}

/// @nodoc
class __$$AesSuccessActionDataResult_DecryptedImplCopyWithImpl<$Res>
    extends _$AesSuccessActionDataResultCopyWithImpl<$Res, _$AesSuccessActionDataResult_DecryptedImpl>
    implements _$$AesSuccessActionDataResult_DecryptedImplCopyWith<$Res> {
  __$$AesSuccessActionDataResult_DecryptedImplCopyWithImpl(_$AesSuccessActionDataResult_DecryptedImpl _value,
      $Res Function(_$AesSuccessActionDataResult_DecryptedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$AesSuccessActionDataResult_DecryptedImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as AesSuccessActionDataDecrypted,
    ));
  }
}

/// @nodoc

class _$AesSuccessActionDataResult_DecryptedImpl implements AesSuccessActionDataResult_Decrypted {
  const _$AesSuccessActionDataResult_DecryptedImpl({required this.data});

  @override
  final AesSuccessActionDataDecrypted data;

  @override
  String toString() {
    return 'AesSuccessActionDataResult.decrypted(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AesSuccessActionDataResult_DecryptedImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AesSuccessActionDataResult_DecryptedImplCopyWith<_$AesSuccessActionDataResult_DecryptedImpl>
      get copyWith => __$$AesSuccessActionDataResult_DecryptedImplCopyWithImpl<
          _$AesSuccessActionDataResult_DecryptedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataDecrypted data) decrypted,
    required TResult Function(String reason) errorStatus,
  }) {
    return decrypted(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataDecrypted data)? decrypted,
    TResult? Function(String reason)? errorStatus,
  }) {
    return decrypted?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(AesSuccessActionDataDecrypted data)? decrypted,
    TResult Function(String reason)? errorStatus,
    required TResult orElse(),
  }) {
    if (decrypted != null) {
      return decrypted(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataResult_Decrypted value) decrypted,
    required TResult Function(AesSuccessActionDataResult_ErrorStatus value) errorStatus,
  }) {
    return decrypted(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataResult_Decrypted value)? decrypted,
    TResult? Function(AesSuccessActionDataResult_ErrorStatus value)? errorStatus,
  }) {
    return decrypted?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AesSuccessActionDataResult_Decrypted value)? decrypted,
    TResult Function(AesSuccessActionDataResult_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) {
    if (decrypted != null) {
      return decrypted(this);
    }
    return orElse();
  }
}

abstract class AesSuccessActionDataResult_Decrypted implements AesSuccessActionDataResult {
  const factory AesSuccessActionDataResult_Decrypted({required final AesSuccessActionDataDecrypted data}) =
      _$AesSuccessActionDataResult_DecryptedImpl;

  AesSuccessActionDataDecrypted get data;
  @JsonKey(ignore: true)
  _$$AesSuccessActionDataResult_DecryptedImplCopyWith<_$AesSuccessActionDataResult_DecryptedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AesSuccessActionDataResult_ErrorStatusImplCopyWith<$Res> {
  factory _$$AesSuccessActionDataResult_ErrorStatusImplCopyWith(
          _$AesSuccessActionDataResult_ErrorStatusImpl value,
          $Res Function(_$AesSuccessActionDataResult_ErrorStatusImpl) then) =
      __$$AesSuccessActionDataResult_ErrorStatusImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String reason});
}

/// @nodoc
class __$$AesSuccessActionDataResult_ErrorStatusImplCopyWithImpl<$Res>
    extends _$AesSuccessActionDataResultCopyWithImpl<$Res, _$AesSuccessActionDataResult_ErrorStatusImpl>
    implements _$$AesSuccessActionDataResult_ErrorStatusImplCopyWith<$Res> {
  __$$AesSuccessActionDataResult_ErrorStatusImplCopyWithImpl(
      _$AesSuccessActionDataResult_ErrorStatusImpl _value,
      $Res Function(_$AesSuccessActionDataResult_ErrorStatusImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? reason = null,
  }) {
    return _then(_$AesSuccessActionDataResult_ErrorStatusImpl(
      reason: null == reason
          ? _value.reason
          : reason // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$AesSuccessActionDataResult_ErrorStatusImpl implements AesSuccessActionDataResult_ErrorStatus {
  const _$AesSuccessActionDataResult_ErrorStatusImpl({required this.reason});

  @override
  final String reason;

  @override
  String toString() {
    return 'AesSuccessActionDataResult.errorStatus(reason: $reason)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AesSuccessActionDataResult_ErrorStatusImpl &&
            (identical(other.reason, reason) || other.reason == reason));
  }

  @override
  int get hashCode => Object.hash(runtimeType, reason);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AesSuccessActionDataResult_ErrorStatusImplCopyWith<_$AesSuccessActionDataResult_ErrorStatusImpl>
      get copyWith => __$$AesSuccessActionDataResult_ErrorStatusImplCopyWithImpl<
          _$AesSuccessActionDataResult_ErrorStatusImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataDecrypted data) decrypted,
    required TResult Function(String reason) errorStatus,
  }) {
    return errorStatus(reason);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataDecrypted data)? decrypted,
    TResult? Function(String reason)? errorStatus,
  }) {
    return errorStatus?.call(reason);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(AesSuccessActionDataDecrypted data)? decrypted,
    TResult Function(String reason)? errorStatus,
    required TResult orElse(),
  }) {
    if (errorStatus != null) {
      return errorStatus(reason);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataResult_Decrypted value) decrypted,
    required TResult Function(AesSuccessActionDataResult_ErrorStatus value) errorStatus,
  }) {
    return errorStatus(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataResult_Decrypted value)? decrypted,
    TResult? Function(AesSuccessActionDataResult_ErrorStatus value)? errorStatus,
  }) {
    return errorStatus?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AesSuccessActionDataResult_Decrypted value)? decrypted,
    TResult Function(AesSuccessActionDataResult_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) {
    if (errorStatus != null) {
      return errorStatus(this);
    }
    return orElse();
  }
}

abstract class AesSuccessActionDataResult_ErrorStatus implements AesSuccessActionDataResult {
  const factory AesSuccessActionDataResult_ErrorStatus({required final String reason}) =
      _$AesSuccessActionDataResult_ErrorStatusImpl;

  String get reason;
  @JsonKey(ignore: true)
  _$$AesSuccessActionDataResult_ErrorStatusImplCopyWith<_$AesSuccessActionDataResult_ErrorStatusImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$BitcoinAddressData {
  String get address => throw _privateConstructorUsedError;
  Network get network => throw _privateConstructorUsedError;
  int? get amountSat => throw _privateConstructorUsedError;
  String? get label => throw _privateConstructorUsedError;
  String? get message => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $BitcoinAddressDataCopyWith<BitcoinAddressData> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BitcoinAddressDataCopyWith<$Res> {
  factory $BitcoinAddressDataCopyWith(BitcoinAddressData value, $Res Function(BitcoinAddressData) then) =
      _$BitcoinAddressDataCopyWithImpl<$Res, BitcoinAddressData>;
  @useResult
  $Res call({String address, Network network, int? amountSat, String? label, String? message});
}

/// @nodoc
class _$BitcoinAddressDataCopyWithImpl<$Res, $Val extends BitcoinAddressData>
    implements $BitcoinAddressDataCopyWith<$Res> {
  _$BitcoinAddressDataCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
    Object? network = null,
    Object? amountSat = freezed,
    Object? label = freezed,
    Object? message = freezed,
  }) {
    return _then(_value.copyWith(
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      network: null == network
          ? _value.network
          : network // ignore: cast_nullable_to_non_nullable
              as Network,
      amountSat: freezed == amountSat
          ? _value.amountSat
          : amountSat // ignore: cast_nullable_to_non_nullable
              as int?,
      label: freezed == label
          ? _value.label
          : label // ignore: cast_nullable_to_non_nullable
              as String?,
      message: freezed == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$BitcoinAddressDataImplCopyWith<$Res> implements $BitcoinAddressDataCopyWith<$Res> {
  factory _$$BitcoinAddressDataImplCopyWith(
          _$BitcoinAddressDataImpl value, $Res Function(_$BitcoinAddressDataImpl) then) =
      __$$BitcoinAddressDataImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String address, Network network, int? amountSat, String? label, String? message});
}

/// @nodoc
class __$$BitcoinAddressDataImplCopyWithImpl<$Res>
    extends _$BitcoinAddressDataCopyWithImpl<$Res, _$BitcoinAddressDataImpl>
    implements _$$BitcoinAddressDataImplCopyWith<$Res> {
  __$$BitcoinAddressDataImplCopyWithImpl(
      _$BitcoinAddressDataImpl _value, $Res Function(_$BitcoinAddressDataImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
    Object? network = null,
    Object? amountSat = freezed,
    Object? label = freezed,
    Object? message = freezed,
  }) {
    return _then(_$BitcoinAddressDataImpl(
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      network: null == network
          ? _value.network
          : network // ignore: cast_nullable_to_non_nullable
              as Network,
      amountSat: freezed == amountSat
          ? _value.amountSat
          : amountSat // ignore: cast_nullable_to_non_nullable
              as int?,
      label: freezed == label
          ? _value.label
          : label // ignore: cast_nullable_to_non_nullable
              as String?,
      message: freezed == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class _$BitcoinAddressDataImpl implements _BitcoinAddressData {
  const _$BitcoinAddressDataImpl(
      {required this.address, required this.network, this.amountSat, this.label, this.message});

  @override
  final String address;
  @override
  final Network network;
  @override
  final int? amountSat;
  @override
  final String? label;
  @override
  final String? message;

  @override
  String toString() {
    return 'BitcoinAddressData(address: $address, network: $network, amountSat: $amountSat, label: $label, message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BitcoinAddressDataImpl &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.network, network) || other.network == network) &&
            (identical(other.amountSat, amountSat) || other.amountSat == amountSat) &&
            (identical(other.label, label) || other.label == label) &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, network, amountSat, label, message);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BitcoinAddressDataImplCopyWith<_$BitcoinAddressDataImpl> get copyWith =>
      __$$BitcoinAddressDataImplCopyWithImpl<_$BitcoinAddressDataImpl>(this, _$identity);
}

abstract class _BitcoinAddressData implements BitcoinAddressData {
  const factory _BitcoinAddressData(
      {required final String address,
      required final Network network,
      final int? amountSat,
      final String? label,
      final String? message}) = _$BitcoinAddressDataImpl;

  @override
  String get address;
  @override
  Network get network;
  @override
  int? get amountSat;
  @override
  String? get label;
  @override
  String? get message;
  @override
  @JsonKey(ignore: true)
  _$$BitcoinAddressDataImplCopyWith<_$BitcoinAddressDataImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$BreezEvent {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
    required TResult Function(Payment details) paymentSucceed,
    required TResult Function(PaymentFailedData details) paymentFailed,
    required TResult Function() backupStarted,
    required TResult Function() backupSucceeded,
    required TResult Function(BackupFailedData details) backupFailed,
    required TResult Function(SwapInfo details) swapUpdated,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
    TResult? Function(Payment details)? paymentSucceed,
    TResult? Function(PaymentFailedData details)? paymentFailed,
    TResult? Function()? backupStarted,
    TResult? Function()? backupSucceeded,
    TResult? Function(BackupFailedData details)? backupFailed,
    TResult? Function(SwapInfo details)? swapUpdated,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
    TResult Function(Payment details)? paymentSucceed,
    TResult Function(PaymentFailedData details)? paymentFailed,
    TResult Function()? backupStarted,
    TResult Function()? backupSucceeded,
    TResult Function(BackupFailedData details)? backupFailed,
    TResult Function(SwapInfo details)? swapUpdated,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BreezEvent_NewBlock value) newBlock,
    required TResult Function(BreezEvent_InvoicePaid value) invoicePaid,
    required TResult Function(BreezEvent_Synced value) synced,
    required TResult Function(BreezEvent_PaymentSucceed value) paymentSucceed,
    required TResult Function(BreezEvent_PaymentFailed value) paymentFailed,
    required TResult Function(BreezEvent_BackupStarted value) backupStarted,
    required TResult Function(BreezEvent_BackupSucceeded value) backupSucceeded,
    required TResult Function(BreezEvent_BackupFailed value) backupFailed,
    required TResult Function(BreezEvent_SwapUpdated value) swapUpdated,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
    TResult? Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult? Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult? Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult? Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult? Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult? Function(BreezEvent_SwapUpdated value)? swapUpdated,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    TResult Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult Function(BreezEvent_SwapUpdated value)? swapUpdated,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BreezEventCopyWith<$Res> {
  factory $BreezEventCopyWith(BreezEvent value, $Res Function(BreezEvent) then) =
      _$BreezEventCopyWithImpl<$Res, BreezEvent>;
}

/// @nodoc
class _$BreezEventCopyWithImpl<$Res, $Val extends BreezEvent> implements $BreezEventCopyWith<$Res> {
  _$BreezEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$BreezEvent_NewBlockImplCopyWith<$Res> {
  factory _$$BreezEvent_NewBlockImplCopyWith(
          _$BreezEvent_NewBlockImpl value, $Res Function(_$BreezEvent_NewBlockImpl) then) =
      __$$BreezEvent_NewBlockImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int block});
}

/// @nodoc
class __$$BreezEvent_NewBlockImplCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_NewBlockImpl>
    implements _$$BreezEvent_NewBlockImplCopyWith<$Res> {
  __$$BreezEvent_NewBlockImplCopyWithImpl(
      _$BreezEvent_NewBlockImpl _value, $Res Function(_$BreezEvent_NewBlockImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? block = null,
  }) {
    return _then(_$BreezEvent_NewBlockImpl(
      block: null == block
          ? _value.block
          : block // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$BreezEvent_NewBlockImpl implements BreezEvent_NewBlock {
  const _$BreezEvent_NewBlockImpl({required this.block});

  @override
  final int block;

  @override
  String toString() {
    return 'BreezEvent.newBlock(block: $block)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BreezEvent_NewBlockImpl &&
            (identical(other.block, block) || other.block == block));
  }

  @override
  int get hashCode => Object.hash(runtimeType, block);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BreezEvent_NewBlockImplCopyWith<_$BreezEvent_NewBlockImpl> get copyWith =>
      __$$BreezEvent_NewBlockImplCopyWithImpl<_$BreezEvent_NewBlockImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
    required TResult Function(Payment details) paymentSucceed,
    required TResult Function(PaymentFailedData details) paymentFailed,
    required TResult Function() backupStarted,
    required TResult Function() backupSucceeded,
    required TResult Function(BackupFailedData details) backupFailed,
    required TResult Function(SwapInfo details) swapUpdated,
  }) {
    return newBlock(block);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
    TResult? Function(Payment details)? paymentSucceed,
    TResult? Function(PaymentFailedData details)? paymentFailed,
    TResult? Function()? backupStarted,
    TResult? Function()? backupSucceeded,
    TResult? Function(BackupFailedData details)? backupFailed,
    TResult? Function(SwapInfo details)? swapUpdated,
  }) {
    return newBlock?.call(block);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
    TResult Function(Payment details)? paymentSucceed,
    TResult Function(PaymentFailedData details)? paymentFailed,
    TResult Function()? backupStarted,
    TResult Function()? backupSucceeded,
    TResult Function(BackupFailedData details)? backupFailed,
    TResult Function(SwapInfo details)? swapUpdated,
    required TResult orElse(),
  }) {
    if (newBlock != null) {
      return newBlock(block);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BreezEvent_NewBlock value) newBlock,
    required TResult Function(BreezEvent_InvoicePaid value) invoicePaid,
    required TResult Function(BreezEvent_Synced value) synced,
    required TResult Function(BreezEvent_PaymentSucceed value) paymentSucceed,
    required TResult Function(BreezEvent_PaymentFailed value) paymentFailed,
    required TResult Function(BreezEvent_BackupStarted value) backupStarted,
    required TResult Function(BreezEvent_BackupSucceeded value) backupSucceeded,
    required TResult Function(BreezEvent_BackupFailed value) backupFailed,
    required TResult Function(BreezEvent_SwapUpdated value) swapUpdated,
  }) {
    return newBlock(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
    TResult? Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult? Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult? Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult? Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult? Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult? Function(BreezEvent_SwapUpdated value)? swapUpdated,
  }) {
    return newBlock?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    TResult Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult Function(BreezEvent_SwapUpdated value)? swapUpdated,
    required TResult orElse(),
  }) {
    if (newBlock != null) {
      return newBlock(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_NewBlock implements BreezEvent {
  const factory BreezEvent_NewBlock({required final int block}) = _$BreezEvent_NewBlockImpl;

  int get block;
  @JsonKey(ignore: true)
  _$$BreezEvent_NewBlockImplCopyWith<_$BreezEvent_NewBlockImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BreezEvent_InvoicePaidImplCopyWith<$Res> {
  factory _$$BreezEvent_InvoicePaidImplCopyWith(
          _$BreezEvent_InvoicePaidImpl value, $Res Function(_$BreezEvent_InvoicePaidImpl) then) =
      __$$BreezEvent_InvoicePaidImplCopyWithImpl<$Res>;
  @useResult
  $Res call({InvoicePaidDetails details});
}

/// @nodoc
class __$$BreezEvent_InvoicePaidImplCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_InvoicePaidImpl>
    implements _$$BreezEvent_InvoicePaidImplCopyWith<$Res> {
  __$$BreezEvent_InvoicePaidImplCopyWithImpl(
      _$BreezEvent_InvoicePaidImpl _value, $Res Function(_$BreezEvent_InvoicePaidImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$BreezEvent_InvoicePaidImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as InvoicePaidDetails,
    ));
  }
}

/// @nodoc

class _$BreezEvent_InvoicePaidImpl implements BreezEvent_InvoicePaid {
  const _$BreezEvent_InvoicePaidImpl({required this.details});

  @override
  final InvoicePaidDetails details;

  @override
  String toString() {
    return 'BreezEvent.invoicePaid(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BreezEvent_InvoicePaidImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BreezEvent_InvoicePaidImplCopyWith<_$BreezEvent_InvoicePaidImpl> get copyWith =>
      __$$BreezEvent_InvoicePaidImplCopyWithImpl<_$BreezEvent_InvoicePaidImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
    required TResult Function(Payment details) paymentSucceed,
    required TResult Function(PaymentFailedData details) paymentFailed,
    required TResult Function() backupStarted,
    required TResult Function() backupSucceeded,
    required TResult Function(BackupFailedData details) backupFailed,
    required TResult Function(SwapInfo details) swapUpdated,
  }) {
    return invoicePaid(details);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
    TResult? Function(Payment details)? paymentSucceed,
    TResult? Function(PaymentFailedData details)? paymentFailed,
    TResult? Function()? backupStarted,
    TResult? Function()? backupSucceeded,
    TResult? Function(BackupFailedData details)? backupFailed,
    TResult? Function(SwapInfo details)? swapUpdated,
  }) {
    return invoicePaid?.call(details);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
    TResult Function(Payment details)? paymentSucceed,
    TResult Function(PaymentFailedData details)? paymentFailed,
    TResult Function()? backupStarted,
    TResult Function()? backupSucceeded,
    TResult Function(BackupFailedData details)? backupFailed,
    TResult Function(SwapInfo details)? swapUpdated,
    required TResult orElse(),
  }) {
    if (invoicePaid != null) {
      return invoicePaid(details);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BreezEvent_NewBlock value) newBlock,
    required TResult Function(BreezEvent_InvoicePaid value) invoicePaid,
    required TResult Function(BreezEvent_Synced value) synced,
    required TResult Function(BreezEvent_PaymentSucceed value) paymentSucceed,
    required TResult Function(BreezEvent_PaymentFailed value) paymentFailed,
    required TResult Function(BreezEvent_BackupStarted value) backupStarted,
    required TResult Function(BreezEvent_BackupSucceeded value) backupSucceeded,
    required TResult Function(BreezEvent_BackupFailed value) backupFailed,
    required TResult Function(BreezEvent_SwapUpdated value) swapUpdated,
  }) {
    return invoicePaid(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
    TResult? Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult? Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult? Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult? Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult? Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult? Function(BreezEvent_SwapUpdated value)? swapUpdated,
  }) {
    return invoicePaid?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    TResult Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult Function(BreezEvent_SwapUpdated value)? swapUpdated,
    required TResult orElse(),
  }) {
    if (invoicePaid != null) {
      return invoicePaid(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_InvoicePaid implements BreezEvent {
  const factory BreezEvent_InvoicePaid({required final InvoicePaidDetails details}) =
      _$BreezEvent_InvoicePaidImpl;

  InvoicePaidDetails get details;
  @JsonKey(ignore: true)
  _$$BreezEvent_InvoicePaidImplCopyWith<_$BreezEvent_InvoicePaidImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BreezEvent_SyncedImplCopyWith<$Res> {
  factory _$$BreezEvent_SyncedImplCopyWith(
          _$BreezEvent_SyncedImpl value, $Res Function(_$BreezEvent_SyncedImpl) then) =
      __$$BreezEvent_SyncedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$BreezEvent_SyncedImplCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_SyncedImpl>
    implements _$$BreezEvent_SyncedImplCopyWith<$Res> {
  __$$BreezEvent_SyncedImplCopyWithImpl(
      _$BreezEvent_SyncedImpl _value, $Res Function(_$BreezEvent_SyncedImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$BreezEvent_SyncedImpl implements BreezEvent_Synced {
  const _$BreezEvent_SyncedImpl();

  @override
  String toString() {
    return 'BreezEvent.synced()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$BreezEvent_SyncedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
    required TResult Function(Payment details) paymentSucceed,
    required TResult Function(PaymentFailedData details) paymentFailed,
    required TResult Function() backupStarted,
    required TResult Function() backupSucceeded,
    required TResult Function(BackupFailedData details) backupFailed,
    required TResult Function(SwapInfo details) swapUpdated,
  }) {
    return synced();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
    TResult? Function(Payment details)? paymentSucceed,
    TResult? Function(PaymentFailedData details)? paymentFailed,
    TResult? Function()? backupStarted,
    TResult? Function()? backupSucceeded,
    TResult? Function(BackupFailedData details)? backupFailed,
    TResult? Function(SwapInfo details)? swapUpdated,
  }) {
    return synced?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
    TResult Function(Payment details)? paymentSucceed,
    TResult Function(PaymentFailedData details)? paymentFailed,
    TResult Function()? backupStarted,
    TResult Function()? backupSucceeded,
    TResult Function(BackupFailedData details)? backupFailed,
    TResult Function(SwapInfo details)? swapUpdated,
    required TResult orElse(),
  }) {
    if (synced != null) {
      return synced();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BreezEvent_NewBlock value) newBlock,
    required TResult Function(BreezEvent_InvoicePaid value) invoicePaid,
    required TResult Function(BreezEvent_Synced value) synced,
    required TResult Function(BreezEvent_PaymentSucceed value) paymentSucceed,
    required TResult Function(BreezEvent_PaymentFailed value) paymentFailed,
    required TResult Function(BreezEvent_BackupStarted value) backupStarted,
    required TResult Function(BreezEvent_BackupSucceeded value) backupSucceeded,
    required TResult Function(BreezEvent_BackupFailed value) backupFailed,
    required TResult Function(BreezEvent_SwapUpdated value) swapUpdated,
  }) {
    return synced(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
    TResult? Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult? Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult? Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult? Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult? Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult? Function(BreezEvent_SwapUpdated value)? swapUpdated,
  }) {
    return synced?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    TResult Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult Function(BreezEvent_SwapUpdated value)? swapUpdated,
    required TResult orElse(),
  }) {
    if (synced != null) {
      return synced(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_Synced implements BreezEvent {
  const factory BreezEvent_Synced() = _$BreezEvent_SyncedImpl;
}

/// @nodoc
abstract class _$$BreezEvent_PaymentSucceedImplCopyWith<$Res> {
  factory _$$BreezEvent_PaymentSucceedImplCopyWith(
          _$BreezEvent_PaymentSucceedImpl value, $Res Function(_$BreezEvent_PaymentSucceedImpl) then) =
      __$$BreezEvent_PaymentSucceedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});

  $PaymentCopyWith<$Res> get details;
}

/// @nodoc
class __$$BreezEvent_PaymentSucceedImplCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_PaymentSucceedImpl>
    implements _$$BreezEvent_PaymentSucceedImplCopyWith<$Res> {
  __$$BreezEvent_PaymentSucceedImplCopyWithImpl(
      _$BreezEvent_PaymentSucceedImpl _value, $Res Function(_$BreezEvent_PaymentSucceedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$BreezEvent_PaymentSucceedImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $PaymentCopyWith<$Res> get details {
    return $PaymentCopyWith<$Res>(_value.details, (value) {
      return _then(_value.copyWith(details: value));
    });
  }
}

/// @nodoc

class _$BreezEvent_PaymentSucceedImpl implements BreezEvent_PaymentSucceed {
  const _$BreezEvent_PaymentSucceedImpl({required this.details});

  @override
  final Payment details;

  @override
  String toString() {
    return 'BreezEvent.paymentSucceed(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BreezEvent_PaymentSucceedImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BreezEvent_PaymentSucceedImplCopyWith<_$BreezEvent_PaymentSucceedImpl> get copyWith =>
      __$$BreezEvent_PaymentSucceedImplCopyWithImpl<_$BreezEvent_PaymentSucceedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
    required TResult Function(Payment details) paymentSucceed,
    required TResult Function(PaymentFailedData details) paymentFailed,
    required TResult Function() backupStarted,
    required TResult Function() backupSucceeded,
    required TResult Function(BackupFailedData details) backupFailed,
    required TResult Function(SwapInfo details) swapUpdated,
  }) {
    return paymentSucceed(details);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
    TResult? Function(Payment details)? paymentSucceed,
    TResult? Function(PaymentFailedData details)? paymentFailed,
    TResult? Function()? backupStarted,
    TResult? Function()? backupSucceeded,
    TResult? Function(BackupFailedData details)? backupFailed,
    TResult? Function(SwapInfo details)? swapUpdated,
  }) {
    return paymentSucceed?.call(details);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
    TResult Function(Payment details)? paymentSucceed,
    TResult Function(PaymentFailedData details)? paymentFailed,
    TResult Function()? backupStarted,
    TResult Function()? backupSucceeded,
    TResult Function(BackupFailedData details)? backupFailed,
    TResult Function(SwapInfo details)? swapUpdated,
    required TResult orElse(),
  }) {
    if (paymentSucceed != null) {
      return paymentSucceed(details);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BreezEvent_NewBlock value) newBlock,
    required TResult Function(BreezEvent_InvoicePaid value) invoicePaid,
    required TResult Function(BreezEvent_Synced value) synced,
    required TResult Function(BreezEvent_PaymentSucceed value) paymentSucceed,
    required TResult Function(BreezEvent_PaymentFailed value) paymentFailed,
    required TResult Function(BreezEvent_BackupStarted value) backupStarted,
    required TResult Function(BreezEvent_BackupSucceeded value) backupSucceeded,
    required TResult Function(BreezEvent_BackupFailed value) backupFailed,
    required TResult Function(BreezEvent_SwapUpdated value) swapUpdated,
  }) {
    return paymentSucceed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
    TResult? Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult? Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult? Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult? Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult? Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult? Function(BreezEvent_SwapUpdated value)? swapUpdated,
  }) {
    return paymentSucceed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    TResult Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult Function(BreezEvent_SwapUpdated value)? swapUpdated,
    required TResult orElse(),
  }) {
    if (paymentSucceed != null) {
      return paymentSucceed(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_PaymentSucceed implements BreezEvent {
  const factory BreezEvent_PaymentSucceed({required final Payment details}) = _$BreezEvent_PaymentSucceedImpl;

  Payment get details;
  @JsonKey(ignore: true)
  _$$BreezEvent_PaymentSucceedImplCopyWith<_$BreezEvent_PaymentSucceedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BreezEvent_PaymentFailedImplCopyWith<$Res> {
  factory _$$BreezEvent_PaymentFailedImplCopyWith(
          _$BreezEvent_PaymentFailedImpl value, $Res Function(_$BreezEvent_PaymentFailedImpl) then) =
      __$$BreezEvent_PaymentFailedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({PaymentFailedData details});
}

/// @nodoc
class __$$BreezEvent_PaymentFailedImplCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_PaymentFailedImpl>
    implements _$$BreezEvent_PaymentFailedImplCopyWith<$Res> {
  __$$BreezEvent_PaymentFailedImplCopyWithImpl(
      _$BreezEvent_PaymentFailedImpl _value, $Res Function(_$BreezEvent_PaymentFailedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$BreezEvent_PaymentFailedImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as PaymentFailedData,
    ));
  }
}

/// @nodoc

class _$BreezEvent_PaymentFailedImpl implements BreezEvent_PaymentFailed {
  const _$BreezEvent_PaymentFailedImpl({required this.details});

  @override
  final PaymentFailedData details;

  @override
  String toString() {
    return 'BreezEvent.paymentFailed(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BreezEvent_PaymentFailedImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BreezEvent_PaymentFailedImplCopyWith<_$BreezEvent_PaymentFailedImpl> get copyWith =>
      __$$BreezEvent_PaymentFailedImplCopyWithImpl<_$BreezEvent_PaymentFailedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
    required TResult Function(Payment details) paymentSucceed,
    required TResult Function(PaymentFailedData details) paymentFailed,
    required TResult Function() backupStarted,
    required TResult Function() backupSucceeded,
    required TResult Function(BackupFailedData details) backupFailed,
    required TResult Function(SwapInfo details) swapUpdated,
  }) {
    return paymentFailed(details);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
    TResult? Function(Payment details)? paymentSucceed,
    TResult? Function(PaymentFailedData details)? paymentFailed,
    TResult? Function()? backupStarted,
    TResult? Function()? backupSucceeded,
    TResult? Function(BackupFailedData details)? backupFailed,
    TResult? Function(SwapInfo details)? swapUpdated,
  }) {
    return paymentFailed?.call(details);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
    TResult Function(Payment details)? paymentSucceed,
    TResult Function(PaymentFailedData details)? paymentFailed,
    TResult Function()? backupStarted,
    TResult Function()? backupSucceeded,
    TResult Function(BackupFailedData details)? backupFailed,
    TResult Function(SwapInfo details)? swapUpdated,
    required TResult orElse(),
  }) {
    if (paymentFailed != null) {
      return paymentFailed(details);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BreezEvent_NewBlock value) newBlock,
    required TResult Function(BreezEvent_InvoicePaid value) invoicePaid,
    required TResult Function(BreezEvent_Synced value) synced,
    required TResult Function(BreezEvent_PaymentSucceed value) paymentSucceed,
    required TResult Function(BreezEvent_PaymentFailed value) paymentFailed,
    required TResult Function(BreezEvent_BackupStarted value) backupStarted,
    required TResult Function(BreezEvent_BackupSucceeded value) backupSucceeded,
    required TResult Function(BreezEvent_BackupFailed value) backupFailed,
    required TResult Function(BreezEvent_SwapUpdated value) swapUpdated,
  }) {
    return paymentFailed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
    TResult? Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult? Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult? Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult? Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult? Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult? Function(BreezEvent_SwapUpdated value)? swapUpdated,
  }) {
    return paymentFailed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    TResult Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult Function(BreezEvent_SwapUpdated value)? swapUpdated,
    required TResult orElse(),
  }) {
    if (paymentFailed != null) {
      return paymentFailed(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_PaymentFailed implements BreezEvent {
  const factory BreezEvent_PaymentFailed({required final PaymentFailedData details}) =
      _$BreezEvent_PaymentFailedImpl;

  PaymentFailedData get details;
  @JsonKey(ignore: true)
  _$$BreezEvent_PaymentFailedImplCopyWith<_$BreezEvent_PaymentFailedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BreezEvent_BackupStartedImplCopyWith<$Res> {
  factory _$$BreezEvent_BackupStartedImplCopyWith(
          _$BreezEvent_BackupStartedImpl value, $Res Function(_$BreezEvent_BackupStartedImpl) then) =
      __$$BreezEvent_BackupStartedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$BreezEvent_BackupStartedImplCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_BackupStartedImpl>
    implements _$$BreezEvent_BackupStartedImplCopyWith<$Res> {
  __$$BreezEvent_BackupStartedImplCopyWithImpl(
      _$BreezEvent_BackupStartedImpl _value, $Res Function(_$BreezEvent_BackupStartedImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$BreezEvent_BackupStartedImpl implements BreezEvent_BackupStarted {
  const _$BreezEvent_BackupStartedImpl();

  @override
  String toString() {
    return 'BreezEvent.backupStarted()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$BreezEvent_BackupStartedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
    required TResult Function(Payment details) paymentSucceed,
    required TResult Function(PaymentFailedData details) paymentFailed,
    required TResult Function() backupStarted,
    required TResult Function() backupSucceeded,
    required TResult Function(BackupFailedData details) backupFailed,
    required TResult Function(SwapInfo details) swapUpdated,
  }) {
    return backupStarted();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
    TResult? Function(Payment details)? paymentSucceed,
    TResult? Function(PaymentFailedData details)? paymentFailed,
    TResult? Function()? backupStarted,
    TResult? Function()? backupSucceeded,
    TResult? Function(BackupFailedData details)? backupFailed,
    TResult? Function(SwapInfo details)? swapUpdated,
  }) {
    return backupStarted?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
    TResult Function(Payment details)? paymentSucceed,
    TResult Function(PaymentFailedData details)? paymentFailed,
    TResult Function()? backupStarted,
    TResult Function()? backupSucceeded,
    TResult Function(BackupFailedData details)? backupFailed,
    TResult Function(SwapInfo details)? swapUpdated,
    required TResult orElse(),
  }) {
    if (backupStarted != null) {
      return backupStarted();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BreezEvent_NewBlock value) newBlock,
    required TResult Function(BreezEvent_InvoicePaid value) invoicePaid,
    required TResult Function(BreezEvent_Synced value) synced,
    required TResult Function(BreezEvent_PaymentSucceed value) paymentSucceed,
    required TResult Function(BreezEvent_PaymentFailed value) paymentFailed,
    required TResult Function(BreezEvent_BackupStarted value) backupStarted,
    required TResult Function(BreezEvent_BackupSucceeded value) backupSucceeded,
    required TResult Function(BreezEvent_BackupFailed value) backupFailed,
    required TResult Function(BreezEvent_SwapUpdated value) swapUpdated,
  }) {
    return backupStarted(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
    TResult? Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult? Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult? Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult? Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult? Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult? Function(BreezEvent_SwapUpdated value)? swapUpdated,
  }) {
    return backupStarted?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    TResult Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult Function(BreezEvent_SwapUpdated value)? swapUpdated,
    required TResult orElse(),
  }) {
    if (backupStarted != null) {
      return backupStarted(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_BackupStarted implements BreezEvent {
  const factory BreezEvent_BackupStarted() = _$BreezEvent_BackupStartedImpl;
}

/// @nodoc
abstract class _$$BreezEvent_BackupSucceededImplCopyWith<$Res> {
  factory _$$BreezEvent_BackupSucceededImplCopyWith(
          _$BreezEvent_BackupSucceededImpl value, $Res Function(_$BreezEvent_BackupSucceededImpl) then) =
      __$$BreezEvent_BackupSucceededImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$BreezEvent_BackupSucceededImplCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_BackupSucceededImpl>
    implements _$$BreezEvent_BackupSucceededImplCopyWith<$Res> {
  __$$BreezEvent_BackupSucceededImplCopyWithImpl(
      _$BreezEvent_BackupSucceededImpl _value, $Res Function(_$BreezEvent_BackupSucceededImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$BreezEvent_BackupSucceededImpl implements BreezEvent_BackupSucceeded {
  const _$BreezEvent_BackupSucceededImpl();

  @override
  String toString() {
    return 'BreezEvent.backupSucceeded()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$BreezEvent_BackupSucceededImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
    required TResult Function(Payment details) paymentSucceed,
    required TResult Function(PaymentFailedData details) paymentFailed,
    required TResult Function() backupStarted,
    required TResult Function() backupSucceeded,
    required TResult Function(BackupFailedData details) backupFailed,
    required TResult Function(SwapInfo details) swapUpdated,
  }) {
    return backupSucceeded();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
    TResult? Function(Payment details)? paymentSucceed,
    TResult? Function(PaymentFailedData details)? paymentFailed,
    TResult? Function()? backupStarted,
    TResult? Function()? backupSucceeded,
    TResult? Function(BackupFailedData details)? backupFailed,
    TResult? Function(SwapInfo details)? swapUpdated,
  }) {
    return backupSucceeded?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
    TResult Function(Payment details)? paymentSucceed,
    TResult Function(PaymentFailedData details)? paymentFailed,
    TResult Function()? backupStarted,
    TResult Function()? backupSucceeded,
    TResult Function(BackupFailedData details)? backupFailed,
    TResult Function(SwapInfo details)? swapUpdated,
    required TResult orElse(),
  }) {
    if (backupSucceeded != null) {
      return backupSucceeded();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BreezEvent_NewBlock value) newBlock,
    required TResult Function(BreezEvent_InvoicePaid value) invoicePaid,
    required TResult Function(BreezEvent_Synced value) synced,
    required TResult Function(BreezEvent_PaymentSucceed value) paymentSucceed,
    required TResult Function(BreezEvent_PaymentFailed value) paymentFailed,
    required TResult Function(BreezEvent_BackupStarted value) backupStarted,
    required TResult Function(BreezEvent_BackupSucceeded value) backupSucceeded,
    required TResult Function(BreezEvent_BackupFailed value) backupFailed,
    required TResult Function(BreezEvent_SwapUpdated value) swapUpdated,
  }) {
    return backupSucceeded(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
    TResult? Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult? Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult? Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult? Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult? Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult? Function(BreezEvent_SwapUpdated value)? swapUpdated,
  }) {
    return backupSucceeded?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    TResult Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult Function(BreezEvent_SwapUpdated value)? swapUpdated,
    required TResult orElse(),
  }) {
    if (backupSucceeded != null) {
      return backupSucceeded(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_BackupSucceeded implements BreezEvent {
  const factory BreezEvent_BackupSucceeded() = _$BreezEvent_BackupSucceededImpl;
}

/// @nodoc
abstract class _$$BreezEvent_BackupFailedImplCopyWith<$Res> {
  factory _$$BreezEvent_BackupFailedImplCopyWith(
          _$BreezEvent_BackupFailedImpl value, $Res Function(_$BreezEvent_BackupFailedImpl) then) =
      __$$BreezEvent_BackupFailedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BackupFailedData details});
}

/// @nodoc
class __$$BreezEvent_BackupFailedImplCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_BackupFailedImpl>
    implements _$$BreezEvent_BackupFailedImplCopyWith<$Res> {
  __$$BreezEvent_BackupFailedImplCopyWithImpl(
      _$BreezEvent_BackupFailedImpl _value, $Res Function(_$BreezEvent_BackupFailedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$BreezEvent_BackupFailedImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as BackupFailedData,
    ));
  }
}

/// @nodoc

class _$BreezEvent_BackupFailedImpl implements BreezEvent_BackupFailed {
  const _$BreezEvent_BackupFailedImpl({required this.details});

  @override
  final BackupFailedData details;

  @override
  String toString() {
    return 'BreezEvent.backupFailed(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BreezEvent_BackupFailedImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BreezEvent_BackupFailedImplCopyWith<_$BreezEvent_BackupFailedImpl> get copyWith =>
      __$$BreezEvent_BackupFailedImplCopyWithImpl<_$BreezEvent_BackupFailedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
    required TResult Function(Payment details) paymentSucceed,
    required TResult Function(PaymentFailedData details) paymentFailed,
    required TResult Function() backupStarted,
    required TResult Function() backupSucceeded,
    required TResult Function(BackupFailedData details) backupFailed,
    required TResult Function(SwapInfo details) swapUpdated,
  }) {
    return backupFailed(details);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
    TResult? Function(Payment details)? paymentSucceed,
    TResult? Function(PaymentFailedData details)? paymentFailed,
    TResult? Function()? backupStarted,
    TResult? Function()? backupSucceeded,
    TResult? Function(BackupFailedData details)? backupFailed,
    TResult? Function(SwapInfo details)? swapUpdated,
  }) {
    return backupFailed?.call(details);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
    TResult Function(Payment details)? paymentSucceed,
    TResult Function(PaymentFailedData details)? paymentFailed,
    TResult Function()? backupStarted,
    TResult Function()? backupSucceeded,
    TResult Function(BackupFailedData details)? backupFailed,
    TResult Function(SwapInfo details)? swapUpdated,
    required TResult orElse(),
  }) {
    if (backupFailed != null) {
      return backupFailed(details);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BreezEvent_NewBlock value) newBlock,
    required TResult Function(BreezEvent_InvoicePaid value) invoicePaid,
    required TResult Function(BreezEvent_Synced value) synced,
    required TResult Function(BreezEvent_PaymentSucceed value) paymentSucceed,
    required TResult Function(BreezEvent_PaymentFailed value) paymentFailed,
    required TResult Function(BreezEvent_BackupStarted value) backupStarted,
    required TResult Function(BreezEvent_BackupSucceeded value) backupSucceeded,
    required TResult Function(BreezEvent_BackupFailed value) backupFailed,
    required TResult Function(BreezEvent_SwapUpdated value) swapUpdated,
  }) {
    return backupFailed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
    TResult? Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult? Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult? Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult? Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult? Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult? Function(BreezEvent_SwapUpdated value)? swapUpdated,
  }) {
    return backupFailed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    TResult Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult Function(BreezEvent_SwapUpdated value)? swapUpdated,
    required TResult orElse(),
  }) {
    if (backupFailed != null) {
      return backupFailed(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_BackupFailed implements BreezEvent {
  const factory BreezEvent_BackupFailed({required final BackupFailedData details}) =
      _$BreezEvent_BackupFailedImpl;

  BackupFailedData get details;
  @JsonKey(ignore: true)
  _$$BreezEvent_BackupFailedImplCopyWith<_$BreezEvent_BackupFailedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BreezEvent_SwapUpdatedImplCopyWith<$Res> {
  factory _$$BreezEvent_SwapUpdatedImplCopyWith(
          _$BreezEvent_SwapUpdatedImpl value, $Res Function(_$BreezEvent_SwapUpdatedImpl) then) =
      __$$BreezEvent_SwapUpdatedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({SwapInfo details});

  $SwapInfoCopyWith<$Res> get details;
}

/// @nodoc
class __$$BreezEvent_SwapUpdatedImplCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_SwapUpdatedImpl>
    implements _$$BreezEvent_SwapUpdatedImplCopyWith<$Res> {
  __$$BreezEvent_SwapUpdatedImplCopyWithImpl(
      _$BreezEvent_SwapUpdatedImpl _value, $Res Function(_$BreezEvent_SwapUpdatedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$BreezEvent_SwapUpdatedImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as SwapInfo,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $SwapInfoCopyWith<$Res> get details {
    return $SwapInfoCopyWith<$Res>(_value.details, (value) {
      return _then(_value.copyWith(details: value));
    });
  }
}

/// @nodoc

class _$BreezEvent_SwapUpdatedImpl implements BreezEvent_SwapUpdated {
  const _$BreezEvent_SwapUpdatedImpl({required this.details});

  @override
  final SwapInfo details;

  @override
  String toString() {
    return 'BreezEvent.swapUpdated(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BreezEvent_SwapUpdatedImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BreezEvent_SwapUpdatedImplCopyWith<_$BreezEvent_SwapUpdatedImpl> get copyWith =>
      __$$BreezEvent_SwapUpdatedImplCopyWithImpl<_$BreezEvent_SwapUpdatedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
    required TResult Function(Payment details) paymentSucceed,
    required TResult Function(PaymentFailedData details) paymentFailed,
    required TResult Function() backupStarted,
    required TResult Function() backupSucceeded,
    required TResult Function(BackupFailedData details) backupFailed,
    required TResult Function(SwapInfo details) swapUpdated,
  }) {
    return swapUpdated(details);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
    TResult? Function(Payment details)? paymentSucceed,
    TResult? Function(PaymentFailedData details)? paymentFailed,
    TResult? Function()? backupStarted,
    TResult? Function()? backupSucceeded,
    TResult? Function(BackupFailedData details)? backupFailed,
    TResult? Function(SwapInfo details)? swapUpdated,
  }) {
    return swapUpdated?.call(details);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
    TResult Function(Payment details)? paymentSucceed,
    TResult Function(PaymentFailedData details)? paymentFailed,
    TResult Function()? backupStarted,
    TResult Function()? backupSucceeded,
    TResult Function(BackupFailedData details)? backupFailed,
    TResult Function(SwapInfo details)? swapUpdated,
    required TResult orElse(),
  }) {
    if (swapUpdated != null) {
      return swapUpdated(details);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BreezEvent_NewBlock value) newBlock,
    required TResult Function(BreezEvent_InvoicePaid value) invoicePaid,
    required TResult Function(BreezEvent_Synced value) synced,
    required TResult Function(BreezEvent_PaymentSucceed value) paymentSucceed,
    required TResult Function(BreezEvent_PaymentFailed value) paymentFailed,
    required TResult Function(BreezEvent_BackupStarted value) backupStarted,
    required TResult Function(BreezEvent_BackupSucceeded value) backupSucceeded,
    required TResult Function(BreezEvent_BackupFailed value) backupFailed,
    required TResult Function(BreezEvent_SwapUpdated value) swapUpdated,
  }) {
    return swapUpdated(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
    TResult? Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult? Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult? Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult? Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult? Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult? Function(BreezEvent_SwapUpdated value)? swapUpdated,
  }) {
    return swapUpdated?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    TResult Function(BreezEvent_PaymentSucceed value)? paymentSucceed,
    TResult Function(BreezEvent_PaymentFailed value)? paymentFailed,
    TResult Function(BreezEvent_BackupStarted value)? backupStarted,
    TResult Function(BreezEvent_BackupSucceeded value)? backupSucceeded,
    TResult Function(BreezEvent_BackupFailed value)? backupFailed,
    TResult Function(BreezEvent_SwapUpdated value)? swapUpdated,
    required TResult orElse(),
  }) {
    if (swapUpdated != null) {
      return swapUpdated(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_SwapUpdated implements BreezEvent {
  const factory BreezEvent_SwapUpdated({required final SwapInfo details}) = _$BreezEvent_SwapUpdatedImpl;

  SwapInfo get details;
  @JsonKey(ignore: true)
  _$$BreezEvent_SwapUpdatedImplCopyWith<_$BreezEvent_SwapUpdatedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$BuyBitcoinRequest {
  BuyBitcoinProvider get provider => throw _privateConstructorUsedError;
  OpeningFeeParams? get openingFeeParams => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $BuyBitcoinRequestCopyWith<BuyBitcoinRequest> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BuyBitcoinRequestCopyWith<$Res> {
  factory $BuyBitcoinRequestCopyWith(BuyBitcoinRequest value, $Res Function(BuyBitcoinRequest) then) =
      _$BuyBitcoinRequestCopyWithImpl<$Res, BuyBitcoinRequest>;
  @useResult
  $Res call({BuyBitcoinProvider provider, OpeningFeeParams? openingFeeParams});

  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams;
}

/// @nodoc
class _$BuyBitcoinRequestCopyWithImpl<$Res, $Val extends BuyBitcoinRequest>
    implements $BuyBitcoinRequestCopyWith<$Res> {
  _$BuyBitcoinRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? provider = null,
    Object? openingFeeParams = freezed,
  }) {
    return _then(_value.copyWith(
      provider: null == provider
          ? _value.provider
          : provider // ignore: cast_nullable_to_non_nullable
              as BuyBitcoinProvider,
      openingFeeParams: freezed == openingFeeParams
          ? _value.openingFeeParams
          : openingFeeParams // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams {
    if (_value.openingFeeParams == null) {
      return null;
    }

    return $OpeningFeeParamsCopyWith<$Res>(_value.openingFeeParams!, (value) {
      return _then(_value.copyWith(openingFeeParams: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$BuyBitcoinRequestImplCopyWith<$Res> implements $BuyBitcoinRequestCopyWith<$Res> {
  factory _$$BuyBitcoinRequestImplCopyWith(
          _$BuyBitcoinRequestImpl value, $Res Function(_$BuyBitcoinRequestImpl) then) =
      __$$BuyBitcoinRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({BuyBitcoinProvider provider, OpeningFeeParams? openingFeeParams});

  @override
  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams;
}

/// @nodoc
class __$$BuyBitcoinRequestImplCopyWithImpl<$Res>
    extends _$BuyBitcoinRequestCopyWithImpl<$Res, _$BuyBitcoinRequestImpl>
    implements _$$BuyBitcoinRequestImplCopyWith<$Res> {
  __$$BuyBitcoinRequestImplCopyWithImpl(
      _$BuyBitcoinRequestImpl _value, $Res Function(_$BuyBitcoinRequestImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? provider = null,
    Object? openingFeeParams = freezed,
  }) {
    return _then(_$BuyBitcoinRequestImpl(
      provider: null == provider
          ? _value.provider
          : provider // ignore: cast_nullable_to_non_nullable
              as BuyBitcoinProvider,
      openingFeeParams: freezed == openingFeeParams
          ? _value.openingFeeParams
          : openingFeeParams // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
    ));
  }
}

/// @nodoc

class _$BuyBitcoinRequestImpl implements _BuyBitcoinRequest {
  const _$BuyBitcoinRequestImpl({required this.provider, this.openingFeeParams});

  @override
  final BuyBitcoinProvider provider;
  @override
  final OpeningFeeParams? openingFeeParams;

  @override
  String toString() {
    return 'BuyBitcoinRequest(provider: $provider, openingFeeParams: $openingFeeParams)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BuyBitcoinRequestImpl &&
            (identical(other.provider, provider) || other.provider == provider) &&
            (identical(other.openingFeeParams, openingFeeParams) ||
                other.openingFeeParams == openingFeeParams));
  }

  @override
  int get hashCode => Object.hash(runtimeType, provider, openingFeeParams);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BuyBitcoinRequestImplCopyWith<_$BuyBitcoinRequestImpl> get copyWith =>
      __$$BuyBitcoinRequestImplCopyWithImpl<_$BuyBitcoinRequestImpl>(this, _$identity);
}

abstract class _BuyBitcoinRequest implements BuyBitcoinRequest {
  const factory _BuyBitcoinRequest(
      {required final BuyBitcoinProvider provider,
      final OpeningFeeParams? openingFeeParams}) = _$BuyBitcoinRequestImpl;

  @override
  BuyBitcoinProvider get provider;
  @override
  OpeningFeeParams? get openingFeeParams;
  @override
  @JsonKey(ignore: true)
  _$$BuyBitcoinRequestImplCopyWith<_$BuyBitcoinRequestImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$BuyBitcoinResponse {
  String get url => throw _privateConstructorUsedError;
  OpeningFeeParams? get openingFeeParams => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $BuyBitcoinResponseCopyWith<BuyBitcoinResponse> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BuyBitcoinResponseCopyWith<$Res> {
  factory $BuyBitcoinResponseCopyWith(BuyBitcoinResponse value, $Res Function(BuyBitcoinResponse) then) =
      _$BuyBitcoinResponseCopyWithImpl<$Res, BuyBitcoinResponse>;
  @useResult
  $Res call({String url, OpeningFeeParams? openingFeeParams});

  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams;
}

/// @nodoc
class _$BuyBitcoinResponseCopyWithImpl<$Res, $Val extends BuyBitcoinResponse>
    implements $BuyBitcoinResponseCopyWith<$Res> {
  _$BuyBitcoinResponseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? url = null,
    Object? openingFeeParams = freezed,
  }) {
    return _then(_value.copyWith(
      url: null == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String,
      openingFeeParams: freezed == openingFeeParams
          ? _value.openingFeeParams
          : openingFeeParams // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams {
    if (_value.openingFeeParams == null) {
      return null;
    }

    return $OpeningFeeParamsCopyWith<$Res>(_value.openingFeeParams!, (value) {
      return _then(_value.copyWith(openingFeeParams: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$BuyBitcoinResponseImplCopyWith<$Res> implements $BuyBitcoinResponseCopyWith<$Res> {
  factory _$$BuyBitcoinResponseImplCopyWith(
          _$BuyBitcoinResponseImpl value, $Res Function(_$BuyBitcoinResponseImpl) then) =
      __$$BuyBitcoinResponseImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String url, OpeningFeeParams? openingFeeParams});

  @override
  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams;
}

/// @nodoc
class __$$BuyBitcoinResponseImplCopyWithImpl<$Res>
    extends _$BuyBitcoinResponseCopyWithImpl<$Res, _$BuyBitcoinResponseImpl>
    implements _$$BuyBitcoinResponseImplCopyWith<$Res> {
  __$$BuyBitcoinResponseImplCopyWithImpl(
      _$BuyBitcoinResponseImpl _value, $Res Function(_$BuyBitcoinResponseImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? url = null,
    Object? openingFeeParams = freezed,
  }) {
    return _then(_$BuyBitcoinResponseImpl(
      url: null == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String,
      openingFeeParams: freezed == openingFeeParams
          ? _value.openingFeeParams
          : openingFeeParams // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
    ));
  }
}

/// @nodoc

class _$BuyBitcoinResponseImpl implements _BuyBitcoinResponse {
  const _$BuyBitcoinResponseImpl({required this.url, this.openingFeeParams});

  @override
  final String url;
  @override
  final OpeningFeeParams? openingFeeParams;

  @override
  String toString() {
    return 'BuyBitcoinResponse(url: $url, openingFeeParams: $openingFeeParams)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BuyBitcoinResponseImpl &&
            (identical(other.url, url) || other.url == url) &&
            (identical(other.openingFeeParams, openingFeeParams) ||
                other.openingFeeParams == openingFeeParams));
  }

  @override
  int get hashCode => Object.hash(runtimeType, url, openingFeeParams);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BuyBitcoinResponseImplCopyWith<_$BuyBitcoinResponseImpl> get copyWith =>
      __$$BuyBitcoinResponseImplCopyWithImpl<_$BuyBitcoinResponseImpl>(this, _$identity);
}

abstract class _BuyBitcoinResponse implements BuyBitcoinResponse {
  const factory _BuyBitcoinResponse({required final String url, final OpeningFeeParams? openingFeeParams}) =
      _$BuyBitcoinResponseImpl;

  @override
  String get url;
  @override
  OpeningFeeParams? get openingFeeParams;
  @override
  @JsonKey(ignore: true)
  _$$BuyBitcoinResponseImplCopyWith<_$BuyBitcoinResponseImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$ClosedChannelPaymentDetails {
  ChannelState get state => throw _privateConstructorUsedError;
  String get fundingTxid => throw _privateConstructorUsedError;
  String? get shortChannelId => throw _privateConstructorUsedError;
  String? get closingTxid => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ClosedChannelPaymentDetailsCopyWith<ClosedChannelPaymentDetails> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ClosedChannelPaymentDetailsCopyWith<$Res> {
  factory $ClosedChannelPaymentDetailsCopyWith(
          ClosedChannelPaymentDetails value, $Res Function(ClosedChannelPaymentDetails) then) =
      _$ClosedChannelPaymentDetailsCopyWithImpl<$Res, ClosedChannelPaymentDetails>;
  @useResult
  $Res call({ChannelState state, String fundingTxid, String? shortChannelId, String? closingTxid});
}

/// @nodoc
class _$ClosedChannelPaymentDetailsCopyWithImpl<$Res, $Val extends ClosedChannelPaymentDetails>
    implements $ClosedChannelPaymentDetailsCopyWith<$Res> {
  _$ClosedChannelPaymentDetailsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? state = null,
    Object? fundingTxid = null,
    Object? shortChannelId = freezed,
    Object? closingTxid = freezed,
  }) {
    return _then(_value.copyWith(
      state: null == state
          ? _value.state
          : state // ignore: cast_nullable_to_non_nullable
              as ChannelState,
      fundingTxid: null == fundingTxid
          ? _value.fundingTxid
          : fundingTxid // ignore: cast_nullable_to_non_nullable
              as String,
      shortChannelId: freezed == shortChannelId
          ? _value.shortChannelId
          : shortChannelId // ignore: cast_nullable_to_non_nullable
              as String?,
      closingTxid: freezed == closingTxid
          ? _value.closingTxid
          : closingTxid // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ClosedChannelPaymentDetailsImplCopyWith<$Res>
    implements $ClosedChannelPaymentDetailsCopyWith<$Res> {
  factory _$$ClosedChannelPaymentDetailsImplCopyWith(
          _$ClosedChannelPaymentDetailsImpl value, $Res Function(_$ClosedChannelPaymentDetailsImpl) then) =
      __$$ClosedChannelPaymentDetailsImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({ChannelState state, String fundingTxid, String? shortChannelId, String? closingTxid});
}

/// @nodoc
class __$$ClosedChannelPaymentDetailsImplCopyWithImpl<$Res>
    extends _$ClosedChannelPaymentDetailsCopyWithImpl<$Res, _$ClosedChannelPaymentDetailsImpl>
    implements _$$ClosedChannelPaymentDetailsImplCopyWith<$Res> {
  __$$ClosedChannelPaymentDetailsImplCopyWithImpl(
      _$ClosedChannelPaymentDetailsImpl _value, $Res Function(_$ClosedChannelPaymentDetailsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? state = null,
    Object? fundingTxid = null,
    Object? shortChannelId = freezed,
    Object? closingTxid = freezed,
  }) {
    return _then(_$ClosedChannelPaymentDetailsImpl(
      state: null == state
          ? _value.state
          : state // ignore: cast_nullable_to_non_nullable
              as ChannelState,
      fundingTxid: null == fundingTxid
          ? _value.fundingTxid
          : fundingTxid // ignore: cast_nullable_to_non_nullable
              as String,
      shortChannelId: freezed == shortChannelId
          ? _value.shortChannelId
          : shortChannelId // ignore: cast_nullable_to_non_nullable
              as String?,
      closingTxid: freezed == closingTxid
          ? _value.closingTxid
          : closingTxid // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class _$ClosedChannelPaymentDetailsImpl implements _ClosedChannelPaymentDetails {
  const _$ClosedChannelPaymentDetailsImpl(
      {required this.state, required this.fundingTxid, this.shortChannelId, this.closingTxid});

  @override
  final ChannelState state;
  @override
  final String fundingTxid;
  @override
  final String? shortChannelId;
  @override
  final String? closingTxid;

  @override
  String toString() {
    return 'ClosedChannelPaymentDetails(state: $state, fundingTxid: $fundingTxid, shortChannelId: $shortChannelId, closingTxid: $closingTxid)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ClosedChannelPaymentDetailsImpl &&
            (identical(other.state, state) || other.state == state) &&
            (identical(other.fundingTxid, fundingTxid) || other.fundingTxid == fundingTxid) &&
            (identical(other.shortChannelId, shortChannelId) || other.shortChannelId == shortChannelId) &&
            (identical(other.closingTxid, closingTxid) || other.closingTxid == closingTxid));
  }

  @override
  int get hashCode => Object.hash(runtimeType, state, fundingTxid, shortChannelId, closingTxid);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ClosedChannelPaymentDetailsImplCopyWith<_$ClosedChannelPaymentDetailsImpl> get copyWith =>
      __$$ClosedChannelPaymentDetailsImplCopyWithImpl<_$ClosedChannelPaymentDetailsImpl>(this, _$identity);
}

abstract class _ClosedChannelPaymentDetails implements ClosedChannelPaymentDetails {
  const factory _ClosedChannelPaymentDetails(
      {required final ChannelState state,
      required final String fundingTxid,
      final String? shortChannelId,
      final String? closingTxid}) = _$ClosedChannelPaymentDetailsImpl;

  @override
  ChannelState get state;
  @override
  String get fundingTxid;
  @override
  String? get shortChannelId;
  @override
  String? get closingTxid;
  @override
  @JsonKey(ignore: true)
  _$$ClosedChannelPaymentDetailsImplCopyWith<_$ClosedChannelPaymentDetailsImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Config {
  String get breezserver => throw _privateConstructorUsedError;
  String get chainnotifierUrl => throw _privateConstructorUsedError;
  String? get mempoolspaceUrl => throw _privateConstructorUsedError;
  String get workingDir => throw _privateConstructorUsedError;
  Network get network => throw _privateConstructorUsedError;
  int get paymentTimeoutSec => throw _privateConstructorUsedError;
  String? get defaultLspId => throw _privateConstructorUsedError;
  String? get apiKey => throw _privateConstructorUsedError;
  double get maxfeePercent => throw _privateConstructorUsedError;
  int get exemptfeeMsat => throw _privateConstructorUsedError;
  NodeConfig get nodeConfig => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ConfigCopyWith<Config> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ConfigCopyWith<$Res> {
  factory $ConfigCopyWith(Config value, $Res Function(Config) then) = _$ConfigCopyWithImpl<$Res, Config>;
  @useResult
  $Res call(
      {String breezserver,
      String chainnotifierUrl,
      String? mempoolspaceUrl,
      String workingDir,
      Network network,
      int paymentTimeoutSec,
      String? defaultLspId,
      String? apiKey,
      double maxfeePercent,
      int exemptfeeMsat,
      NodeConfig nodeConfig});

  $NodeConfigCopyWith<$Res> get nodeConfig;
}

/// @nodoc
class _$ConfigCopyWithImpl<$Res, $Val extends Config> implements $ConfigCopyWith<$Res> {
  _$ConfigCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? breezserver = null,
    Object? chainnotifierUrl = null,
    Object? mempoolspaceUrl = freezed,
    Object? workingDir = null,
    Object? network = null,
    Object? paymentTimeoutSec = null,
    Object? defaultLspId = freezed,
    Object? apiKey = freezed,
    Object? maxfeePercent = null,
    Object? exemptfeeMsat = null,
    Object? nodeConfig = null,
  }) {
    return _then(_value.copyWith(
      breezserver: null == breezserver
          ? _value.breezserver
          : breezserver // ignore: cast_nullable_to_non_nullable
              as String,
      chainnotifierUrl: null == chainnotifierUrl
          ? _value.chainnotifierUrl
          : chainnotifierUrl // ignore: cast_nullable_to_non_nullable
              as String,
      mempoolspaceUrl: freezed == mempoolspaceUrl
          ? _value.mempoolspaceUrl
          : mempoolspaceUrl // ignore: cast_nullable_to_non_nullable
              as String?,
      workingDir: null == workingDir
          ? _value.workingDir
          : workingDir // ignore: cast_nullable_to_non_nullable
              as String,
      network: null == network
          ? _value.network
          : network // ignore: cast_nullable_to_non_nullable
              as Network,
      paymentTimeoutSec: null == paymentTimeoutSec
          ? _value.paymentTimeoutSec
          : paymentTimeoutSec // ignore: cast_nullable_to_non_nullable
              as int,
      defaultLspId: freezed == defaultLspId
          ? _value.defaultLspId
          : defaultLspId // ignore: cast_nullable_to_non_nullable
              as String?,
      apiKey: freezed == apiKey
          ? _value.apiKey
          : apiKey // ignore: cast_nullable_to_non_nullable
              as String?,
      maxfeePercent: null == maxfeePercent
          ? _value.maxfeePercent
          : maxfeePercent // ignore: cast_nullable_to_non_nullable
              as double,
      exemptfeeMsat: null == exemptfeeMsat
          ? _value.exemptfeeMsat
          : exemptfeeMsat // ignore: cast_nullable_to_non_nullable
              as int,
      nodeConfig: null == nodeConfig
          ? _value.nodeConfig
          : nodeConfig // ignore: cast_nullable_to_non_nullable
              as NodeConfig,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $NodeConfigCopyWith<$Res> get nodeConfig {
    return $NodeConfigCopyWith<$Res>(_value.nodeConfig, (value) {
      return _then(_value.copyWith(nodeConfig: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$ConfigImplCopyWith<$Res> implements $ConfigCopyWith<$Res> {
  factory _$$ConfigImplCopyWith(_$ConfigImpl value, $Res Function(_$ConfigImpl) then) =
      __$$ConfigImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String breezserver,
      String chainnotifierUrl,
      String? mempoolspaceUrl,
      String workingDir,
      Network network,
      int paymentTimeoutSec,
      String? defaultLspId,
      String? apiKey,
      double maxfeePercent,
      int exemptfeeMsat,
      NodeConfig nodeConfig});

  @override
  $NodeConfigCopyWith<$Res> get nodeConfig;
}

/// @nodoc
class __$$ConfigImplCopyWithImpl<$Res> extends _$ConfigCopyWithImpl<$Res, _$ConfigImpl>
    implements _$$ConfigImplCopyWith<$Res> {
  __$$ConfigImplCopyWithImpl(_$ConfigImpl _value, $Res Function(_$ConfigImpl) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? breezserver = null,
    Object? chainnotifierUrl = null,
    Object? mempoolspaceUrl = freezed,
    Object? workingDir = null,
    Object? network = null,
    Object? paymentTimeoutSec = null,
    Object? defaultLspId = freezed,
    Object? apiKey = freezed,
    Object? maxfeePercent = null,
    Object? exemptfeeMsat = null,
    Object? nodeConfig = null,
  }) {
    return _then(_$ConfigImpl(
      breezserver: null == breezserver
          ? _value.breezserver
          : breezserver // ignore: cast_nullable_to_non_nullable
              as String,
      chainnotifierUrl: null == chainnotifierUrl
          ? _value.chainnotifierUrl
          : chainnotifierUrl // ignore: cast_nullable_to_non_nullable
              as String,
      mempoolspaceUrl: freezed == mempoolspaceUrl
          ? _value.mempoolspaceUrl
          : mempoolspaceUrl // ignore: cast_nullable_to_non_nullable
              as String?,
      workingDir: null == workingDir
          ? _value.workingDir
          : workingDir // ignore: cast_nullable_to_non_nullable
              as String,
      network: null == network
          ? _value.network
          : network // ignore: cast_nullable_to_non_nullable
              as Network,
      paymentTimeoutSec: null == paymentTimeoutSec
          ? _value.paymentTimeoutSec
          : paymentTimeoutSec // ignore: cast_nullable_to_non_nullable
              as int,
      defaultLspId: freezed == defaultLspId
          ? _value.defaultLspId
          : defaultLspId // ignore: cast_nullable_to_non_nullable
              as String?,
      apiKey: freezed == apiKey
          ? _value.apiKey
          : apiKey // ignore: cast_nullable_to_non_nullable
              as String?,
      maxfeePercent: null == maxfeePercent
          ? _value.maxfeePercent
          : maxfeePercent // ignore: cast_nullable_to_non_nullable
              as double,
      exemptfeeMsat: null == exemptfeeMsat
          ? _value.exemptfeeMsat
          : exemptfeeMsat // ignore: cast_nullable_to_non_nullable
              as int,
      nodeConfig: null == nodeConfig
          ? _value.nodeConfig
          : nodeConfig // ignore: cast_nullable_to_non_nullable
              as NodeConfig,
    ));
  }
}

/// @nodoc

class _$ConfigImpl implements _Config {
  const _$ConfigImpl(
      {required this.breezserver,
      required this.chainnotifierUrl,
      this.mempoolspaceUrl,
      required this.workingDir,
      required this.network,
      required this.paymentTimeoutSec,
      this.defaultLspId,
      this.apiKey,
      required this.maxfeePercent,
      required this.exemptfeeMsat,
      required this.nodeConfig});

  @override
  final String breezserver;
  @override
  final String chainnotifierUrl;
  @override
  final String? mempoolspaceUrl;
  @override
  final String workingDir;
  @override
  final Network network;
  @override
  final int paymentTimeoutSec;
  @override
  final String? defaultLspId;
  @override
  final String? apiKey;
  @override
  final double maxfeePercent;
  @override
  final int exemptfeeMsat;
  @override
  final NodeConfig nodeConfig;

  @override
  String toString() {
    return 'Config(breezserver: $breezserver, chainnotifierUrl: $chainnotifierUrl, mempoolspaceUrl: $mempoolspaceUrl, workingDir: $workingDir, network: $network, paymentTimeoutSec: $paymentTimeoutSec, defaultLspId: $defaultLspId, apiKey: $apiKey, maxfeePercent: $maxfeePercent, exemptfeeMsat: $exemptfeeMsat, nodeConfig: $nodeConfig)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConfigImpl &&
            (identical(other.breezserver, breezserver) || other.breezserver == breezserver) &&
            (identical(other.chainnotifierUrl, chainnotifierUrl) ||
                other.chainnotifierUrl == chainnotifierUrl) &&
            (identical(other.mempoolspaceUrl, mempoolspaceUrl) || other.mempoolspaceUrl == mempoolspaceUrl) &&
            (identical(other.workingDir, workingDir) || other.workingDir == workingDir) &&
            (identical(other.network, network) || other.network == network) &&
            (identical(other.paymentTimeoutSec, paymentTimeoutSec) ||
                other.paymentTimeoutSec == paymentTimeoutSec) &&
            (identical(other.defaultLspId, defaultLspId) || other.defaultLspId == defaultLspId) &&
            (identical(other.apiKey, apiKey) || other.apiKey == apiKey) &&
            (identical(other.maxfeePercent, maxfeePercent) || other.maxfeePercent == maxfeePercent) &&
            (identical(other.exemptfeeMsat, exemptfeeMsat) || other.exemptfeeMsat == exemptfeeMsat) &&
            (identical(other.nodeConfig, nodeConfig) || other.nodeConfig == nodeConfig));
  }

  @override
  int get hashCode => Object.hash(runtimeType, breezserver, chainnotifierUrl, mempoolspaceUrl, workingDir,
      network, paymentTimeoutSec, defaultLspId, apiKey, maxfeePercent, exemptfeeMsat, nodeConfig);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ConfigImplCopyWith<_$ConfigImpl> get copyWith =>
      __$$ConfigImplCopyWithImpl<_$ConfigImpl>(this, _$identity);
}

abstract class _Config implements Config {
  const factory _Config(
      {required final String breezserver,
      required final String chainnotifierUrl,
      final String? mempoolspaceUrl,
      required final String workingDir,
      required final Network network,
      required final int paymentTimeoutSec,
      final String? defaultLspId,
      final String? apiKey,
      required final double maxfeePercent,
      required final int exemptfeeMsat,
      required final NodeConfig nodeConfig}) = _$ConfigImpl;

  @override
  String get breezserver;
  @override
  String get chainnotifierUrl;
  @override
  String? get mempoolspaceUrl;
  @override
  String get workingDir;
  @override
  Network get network;
  @override
  int get paymentTimeoutSec;
  @override
  String? get defaultLspId;
  @override
  String? get apiKey;
  @override
  double get maxfeePercent;
  @override
  int get exemptfeeMsat;
  @override
  NodeConfig get nodeConfig;
  @override
  @JsonKey(ignore: true)
  _$$ConfigImplCopyWith<_$ConfigImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$CurrencyInfo {
  String get name => throw _privateConstructorUsedError;
  int get fractionSize => throw _privateConstructorUsedError;
  int? get spacing => throw _privateConstructorUsedError;
  Symbol? get symbol => throw _privateConstructorUsedError;
  Symbol? get uniqSymbol => throw _privateConstructorUsedError;
  List<LocalizedName>? get localizedName => throw _privateConstructorUsedError;
  List<LocaleOverrides>? get localeOverrides => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $CurrencyInfoCopyWith<CurrencyInfo> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CurrencyInfoCopyWith<$Res> {
  factory $CurrencyInfoCopyWith(CurrencyInfo value, $Res Function(CurrencyInfo) then) =
      _$CurrencyInfoCopyWithImpl<$Res, CurrencyInfo>;
  @useResult
  $Res call(
      {String name,
      int fractionSize,
      int? spacing,
      Symbol? symbol,
      Symbol? uniqSymbol,
      List<LocalizedName>? localizedName,
      List<LocaleOverrides>? localeOverrides});

  $SymbolCopyWith<$Res>? get symbol;
  $SymbolCopyWith<$Res>? get uniqSymbol;
}

/// @nodoc
class _$CurrencyInfoCopyWithImpl<$Res, $Val extends CurrencyInfo> implements $CurrencyInfoCopyWith<$Res> {
  _$CurrencyInfoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? fractionSize = null,
    Object? spacing = freezed,
    Object? symbol = freezed,
    Object? uniqSymbol = freezed,
    Object? localizedName = freezed,
    Object? localeOverrides = freezed,
  }) {
    return _then(_value.copyWith(
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      fractionSize: null == fractionSize
          ? _value.fractionSize
          : fractionSize // ignore: cast_nullable_to_non_nullable
              as int,
      spacing: freezed == spacing
          ? _value.spacing
          : spacing // ignore: cast_nullable_to_non_nullable
              as int?,
      symbol: freezed == symbol
          ? _value.symbol
          : symbol // ignore: cast_nullable_to_non_nullable
              as Symbol?,
      uniqSymbol: freezed == uniqSymbol
          ? _value.uniqSymbol
          : uniqSymbol // ignore: cast_nullable_to_non_nullable
              as Symbol?,
      localizedName: freezed == localizedName
          ? _value.localizedName
          : localizedName // ignore: cast_nullable_to_non_nullable
              as List<LocalizedName>?,
      localeOverrides: freezed == localeOverrides
          ? _value.localeOverrides
          : localeOverrides // ignore: cast_nullable_to_non_nullable
              as List<LocaleOverrides>?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $SymbolCopyWith<$Res>? get symbol {
    if (_value.symbol == null) {
      return null;
    }

    return $SymbolCopyWith<$Res>(_value.symbol!, (value) {
      return _then(_value.copyWith(symbol: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $SymbolCopyWith<$Res>? get uniqSymbol {
    if (_value.uniqSymbol == null) {
      return null;
    }

    return $SymbolCopyWith<$Res>(_value.uniqSymbol!, (value) {
      return _then(_value.copyWith(uniqSymbol: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$CurrencyInfoImplCopyWith<$Res> implements $CurrencyInfoCopyWith<$Res> {
  factory _$$CurrencyInfoImplCopyWith(_$CurrencyInfoImpl value, $Res Function(_$CurrencyInfoImpl) then) =
      __$$CurrencyInfoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String name,
      int fractionSize,
      int? spacing,
      Symbol? symbol,
      Symbol? uniqSymbol,
      List<LocalizedName>? localizedName,
      List<LocaleOverrides>? localeOverrides});

  @override
  $SymbolCopyWith<$Res>? get symbol;
  @override
  $SymbolCopyWith<$Res>? get uniqSymbol;
}

/// @nodoc
class __$$CurrencyInfoImplCopyWithImpl<$Res> extends _$CurrencyInfoCopyWithImpl<$Res, _$CurrencyInfoImpl>
    implements _$$CurrencyInfoImplCopyWith<$Res> {
  __$$CurrencyInfoImplCopyWithImpl(_$CurrencyInfoImpl _value, $Res Function(_$CurrencyInfoImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? fractionSize = null,
    Object? spacing = freezed,
    Object? symbol = freezed,
    Object? uniqSymbol = freezed,
    Object? localizedName = freezed,
    Object? localeOverrides = freezed,
  }) {
    return _then(_$CurrencyInfoImpl(
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      fractionSize: null == fractionSize
          ? _value.fractionSize
          : fractionSize // ignore: cast_nullable_to_non_nullable
              as int,
      spacing: freezed == spacing
          ? _value.spacing
          : spacing // ignore: cast_nullable_to_non_nullable
              as int?,
      symbol: freezed == symbol
          ? _value.symbol
          : symbol // ignore: cast_nullable_to_non_nullable
              as Symbol?,
      uniqSymbol: freezed == uniqSymbol
          ? _value.uniqSymbol
          : uniqSymbol // ignore: cast_nullable_to_non_nullable
              as Symbol?,
      localizedName: freezed == localizedName
          ? _value._localizedName
          : localizedName // ignore: cast_nullable_to_non_nullable
              as List<LocalizedName>?,
      localeOverrides: freezed == localeOverrides
          ? _value._localeOverrides
          : localeOverrides // ignore: cast_nullable_to_non_nullable
              as List<LocaleOverrides>?,
    ));
  }
}

/// @nodoc

class _$CurrencyInfoImpl implements _CurrencyInfo {
  const _$CurrencyInfoImpl(
      {required this.name,
      required this.fractionSize,
      this.spacing,
      this.symbol,
      this.uniqSymbol,
      final List<LocalizedName>? localizedName,
      final List<LocaleOverrides>? localeOverrides})
      : _localizedName = localizedName,
        _localeOverrides = localeOverrides;

  @override
  final String name;
  @override
  final int fractionSize;
  @override
  final int? spacing;
  @override
  final Symbol? symbol;
  @override
  final Symbol? uniqSymbol;
  final List<LocalizedName>? _localizedName;
  @override
  List<LocalizedName>? get localizedName {
    final value = _localizedName;
    if (value == null) return null;
    if (_localizedName is EqualUnmodifiableListView) return _localizedName;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  final List<LocaleOverrides>? _localeOverrides;
  @override
  List<LocaleOverrides>? get localeOverrides {
    final value = _localeOverrides;
    if (value == null) return null;
    if (_localeOverrides is EqualUnmodifiableListView) return _localeOverrides;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  @override
  String toString() {
    return 'CurrencyInfo(name: $name, fractionSize: $fractionSize, spacing: $spacing, symbol: $symbol, uniqSymbol: $uniqSymbol, localizedName: $localizedName, localeOverrides: $localeOverrides)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CurrencyInfoImpl &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.fractionSize, fractionSize) || other.fractionSize == fractionSize) &&
            (identical(other.spacing, spacing) || other.spacing == spacing) &&
            (identical(other.symbol, symbol) || other.symbol == symbol) &&
            (identical(other.uniqSymbol, uniqSymbol) || other.uniqSymbol == uniqSymbol) &&
            const DeepCollectionEquality().equals(other._localizedName, _localizedName) &&
            const DeepCollectionEquality().equals(other._localeOverrides, _localeOverrides));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      name,
      fractionSize,
      spacing,
      symbol,
      uniqSymbol,
      const DeepCollectionEquality().hash(_localizedName),
      const DeepCollectionEquality().hash(_localeOverrides));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CurrencyInfoImplCopyWith<_$CurrencyInfoImpl> get copyWith =>
      __$$CurrencyInfoImplCopyWithImpl<_$CurrencyInfoImpl>(this, _$identity);
}

abstract class _CurrencyInfo implements CurrencyInfo {
  const factory _CurrencyInfo(
      {required final String name,
      required final int fractionSize,
      final int? spacing,
      final Symbol? symbol,
      final Symbol? uniqSymbol,
      final List<LocalizedName>? localizedName,
      final List<LocaleOverrides>? localeOverrides}) = _$CurrencyInfoImpl;

  @override
  String get name;
  @override
  int get fractionSize;
  @override
  int? get spacing;
  @override
  Symbol? get symbol;
  @override
  Symbol? get uniqSymbol;
  @override
  List<LocalizedName>? get localizedName;
  @override
  List<LocaleOverrides>? get localeOverrides;
  @override
  @JsonKey(ignore: true)
  _$$CurrencyInfoImplCopyWith<_$CurrencyInfoImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$FiatCurrency {
  String get id => throw _privateConstructorUsedError;
  CurrencyInfo get info => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $FiatCurrencyCopyWith<FiatCurrency> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FiatCurrencyCopyWith<$Res> {
  factory $FiatCurrencyCopyWith(FiatCurrency value, $Res Function(FiatCurrency) then) =
      _$FiatCurrencyCopyWithImpl<$Res, FiatCurrency>;
  @useResult
  $Res call({String id, CurrencyInfo info});

  $CurrencyInfoCopyWith<$Res> get info;
}

/// @nodoc
class _$FiatCurrencyCopyWithImpl<$Res, $Val extends FiatCurrency> implements $FiatCurrencyCopyWith<$Res> {
  _$FiatCurrencyCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? info = null,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      info: null == info
          ? _value.info
          : info // ignore: cast_nullable_to_non_nullable
              as CurrencyInfo,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $CurrencyInfoCopyWith<$Res> get info {
    return $CurrencyInfoCopyWith<$Res>(_value.info, (value) {
      return _then(_value.copyWith(info: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$FiatCurrencyImplCopyWith<$Res> implements $FiatCurrencyCopyWith<$Res> {
  factory _$$FiatCurrencyImplCopyWith(_$FiatCurrencyImpl value, $Res Function(_$FiatCurrencyImpl) then) =
      __$$FiatCurrencyImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String id, CurrencyInfo info});

  @override
  $CurrencyInfoCopyWith<$Res> get info;
}

/// @nodoc
class __$$FiatCurrencyImplCopyWithImpl<$Res> extends _$FiatCurrencyCopyWithImpl<$Res, _$FiatCurrencyImpl>
    implements _$$FiatCurrencyImplCopyWith<$Res> {
  __$$FiatCurrencyImplCopyWithImpl(_$FiatCurrencyImpl _value, $Res Function(_$FiatCurrencyImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? info = null,
  }) {
    return _then(_$FiatCurrencyImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      info: null == info
          ? _value.info
          : info // ignore: cast_nullable_to_non_nullable
              as CurrencyInfo,
    ));
  }
}

/// @nodoc

class _$FiatCurrencyImpl implements _FiatCurrency {
  const _$FiatCurrencyImpl({required this.id, required this.info});

  @override
  final String id;
  @override
  final CurrencyInfo info;

  @override
  String toString() {
    return 'FiatCurrency(id: $id, info: $info)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FiatCurrencyImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.info, info) || other.info == info));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id, info);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$FiatCurrencyImplCopyWith<_$FiatCurrencyImpl> get copyWith =>
      __$$FiatCurrencyImplCopyWithImpl<_$FiatCurrencyImpl>(this, _$identity);
}

abstract class _FiatCurrency implements FiatCurrency {
  const factory _FiatCurrency({required final String id, required final CurrencyInfo info}) =
      _$FiatCurrencyImpl;

  @override
  String get id;
  @override
  CurrencyInfo get info;
  @override
  @JsonKey(ignore: true)
  _$$FiatCurrencyImplCopyWith<_$FiatCurrencyImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$GreenlightNodeConfig {
  GreenlightCredentials? get partnerCredentials => throw _privateConstructorUsedError;
  String? get inviteCode => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $GreenlightNodeConfigCopyWith<GreenlightNodeConfig> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $GreenlightNodeConfigCopyWith<$Res> {
  factory $GreenlightNodeConfigCopyWith(
          GreenlightNodeConfig value, $Res Function(GreenlightNodeConfig) then) =
      _$GreenlightNodeConfigCopyWithImpl<$Res, GreenlightNodeConfig>;
  @useResult
  $Res call({GreenlightCredentials? partnerCredentials, String? inviteCode});
}

/// @nodoc
class _$GreenlightNodeConfigCopyWithImpl<$Res, $Val extends GreenlightNodeConfig>
    implements $GreenlightNodeConfigCopyWith<$Res> {
  _$GreenlightNodeConfigCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? partnerCredentials = freezed,
    Object? inviteCode = freezed,
  }) {
    return _then(_value.copyWith(
      partnerCredentials: freezed == partnerCredentials
          ? _value.partnerCredentials
          : partnerCredentials // ignore: cast_nullable_to_non_nullable
              as GreenlightCredentials?,
      inviteCode: freezed == inviteCode
          ? _value.inviteCode
          : inviteCode // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$GreenlightNodeConfigImplCopyWith<$Res> implements $GreenlightNodeConfigCopyWith<$Res> {
  factory _$$GreenlightNodeConfigImplCopyWith(
          _$GreenlightNodeConfigImpl value, $Res Function(_$GreenlightNodeConfigImpl) then) =
      __$$GreenlightNodeConfigImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({GreenlightCredentials? partnerCredentials, String? inviteCode});
}

/// @nodoc
class __$$GreenlightNodeConfigImplCopyWithImpl<$Res>
    extends _$GreenlightNodeConfigCopyWithImpl<$Res, _$GreenlightNodeConfigImpl>
    implements _$$GreenlightNodeConfigImplCopyWith<$Res> {
  __$$GreenlightNodeConfigImplCopyWithImpl(
      _$GreenlightNodeConfigImpl _value, $Res Function(_$GreenlightNodeConfigImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? partnerCredentials = freezed,
    Object? inviteCode = freezed,
  }) {
    return _then(_$GreenlightNodeConfigImpl(
      partnerCredentials: freezed == partnerCredentials
          ? _value.partnerCredentials
          : partnerCredentials // ignore: cast_nullable_to_non_nullable
              as GreenlightCredentials?,
      inviteCode: freezed == inviteCode
          ? _value.inviteCode
          : inviteCode // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class _$GreenlightNodeConfigImpl implements _GreenlightNodeConfig {
  const _$GreenlightNodeConfigImpl({this.partnerCredentials, this.inviteCode});

  @override
  final GreenlightCredentials? partnerCredentials;
  @override
  final String? inviteCode;

  @override
  String toString() {
    return 'GreenlightNodeConfig(partnerCredentials: $partnerCredentials, inviteCode: $inviteCode)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GreenlightNodeConfigImpl &&
            (identical(other.partnerCredentials, partnerCredentials) ||
                other.partnerCredentials == partnerCredentials) &&
            (identical(other.inviteCode, inviteCode) || other.inviteCode == inviteCode));
  }

  @override
  int get hashCode => Object.hash(runtimeType, partnerCredentials, inviteCode);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$GreenlightNodeConfigImplCopyWith<_$GreenlightNodeConfigImpl> get copyWith =>
      __$$GreenlightNodeConfigImplCopyWithImpl<_$GreenlightNodeConfigImpl>(this, _$identity);
}

abstract class _GreenlightNodeConfig implements GreenlightNodeConfig {
  const factory _GreenlightNodeConfig(
      {final GreenlightCredentials? partnerCredentials,
      final String? inviteCode}) = _$GreenlightNodeConfigImpl;

  @override
  GreenlightCredentials? get partnerCredentials;
  @override
  String? get inviteCode;
  @override
  @JsonKey(ignore: true)
  _$$GreenlightNodeConfigImplCopyWith<_$GreenlightNodeConfigImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$InputType {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BitcoinAddressData address) bitcoinAddress,
    required TResult Function(LNInvoice invoice) bolt11,
    required TResult Function(String nodeId) nodeId,
    required TResult Function(String url) url,
    required TResult Function(LnUrlPayRequestData data) lnUrlPay,
    required TResult Function(LnUrlWithdrawRequestData data) lnUrlWithdraw,
    required TResult Function(LnUrlAuthRequestData data) lnUrlAuth,
    required TResult Function(LnUrlErrorData data) lnUrlError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BitcoinAddressData address)? bitcoinAddress,
    TResult? Function(LNInvoice invoice)? bolt11,
    TResult? Function(String nodeId)? nodeId,
    TResult? Function(String url)? url,
    TResult? Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult? Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult? Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult? Function(LnUrlErrorData data)? lnUrlError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BitcoinAddressData address)? bitcoinAddress,
    TResult Function(LNInvoice invoice)? bolt11,
    TResult Function(String nodeId)? nodeId,
    TResult Function(String url)? url,
    TResult Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult Function(LnUrlErrorData data)? lnUrlError,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InputType_BitcoinAddress value) bitcoinAddress,
    required TResult Function(InputType_Bolt11 value) bolt11,
    required TResult Function(InputType_NodeId value) nodeId,
    required TResult Function(InputType_Url value) url,
    required TResult Function(InputType_LnUrlPay value) lnUrlPay,
    required TResult Function(InputType_LnUrlWithdraw value) lnUrlWithdraw,
    required TResult Function(InputType_LnUrlAuth value) lnUrlAuth,
    required TResult Function(InputType_LnUrlError value) lnUrlError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult? Function(InputType_Bolt11 value)? bolt11,
    TResult? Function(InputType_NodeId value)? nodeId,
    TResult? Function(InputType_Url value)? url,
    TResult? Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult? Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult? Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult? Function(InputType_LnUrlError value)? lnUrlError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult Function(InputType_Bolt11 value)? bolt11,
    TResult Function(InputType_NodeId value)? nodeId,
    TResult Function(InputType_Url value)? url,
    TResult Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult Function(InputType_LnUrlError value)? lnUrlError,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $InputTypeCopyWith<$Res> {
  factory $InputTypeCopyWith(InputType value, $Res Function(InputType) then) =
      _$InputTypeCopyWithImpl<$Res, InputType>;
}

/// @nodoc
class _$InputTypeCopyWithImpl<$Res, $Val extends InputType> implements $InputTypeCopyWith<$Res> {
  _$InputTypeCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$InputType_BitcoinAddressImplCopyWith<$Res> {
  factory _$$InputType_BitcoinAddressImplCopyWith(
          _$InputType_BitcoinAddressImpl value, $Res Function(_$InputType_BitcoinAddressImpl) then) =
      __$$InputType_BitcoinAddressImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BitcoinAddressData address});

  $BitcoinAddressDataCopyWith<$Res> get address;
}

/// @nodoc
class __$$InputType_BitcoinAddressImplCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_BitcoinAddressImpl>
    implements _$$InputType_BitcoinAddressImplCopyWith<$Res> {
  __$$InputType_BitcoinAddressImplCopyWithImpl(
      _$InputType_BitcoinAddressImpl _value, $Res Function(_$InputType_BitcoinAddressImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
  }) {
    return _then(_$InputType_BitcoinAddressImpl(
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as BitcoinAddressData,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $BitcoinAddressDataCopyWith<$Res> get address {
    return $BitcoinAddressDataCopyWith<$Res>(_value.address, (value) {
      return _then(_value.copyWith(address: value));
    });
  }
}

/// @nodoc

class _$InputType_BitcoinAddressImpl implements InputType_BitcoinAddress {
  const _$InputType_BitcoinAddressImpl({required this.address});

  @override
  final BitcoinAddressData address;

  @override
  String toString() {
    return 'InputType.bitcoinAddress(address: $address)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_BitcoinAddressImpl &&
            (identical(other.address, address) || other.address == address));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_BitcoinAddressImplCopyWith<_$InputType_BitcoinAddressImpl> get copyWith =>
      __$$InputType_BitcoinAddressImplCopyWithImpl<_$InputType_BitcoinAddressImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BitcoinAddressData address) bitcoinAddress,
    required TResult Function(LNInvoice invoice) bolt11,
    required TResult Function(String nodeId) nodeId,
    required TResult Function(String url) url,
    required TResult Function(LnUrlPayRequestData data) lnUrlPay,
    required TResult Function(LnUrlWithdrawRequestData data) lnUrlWithdraw,
    required TResult Function(LnUrlAuthRequestData data) lnUrlAuth,
    required TResult Function(LnUrlErrorData data) lnUrlError,
  }) {
    return bitcoinAddress(address);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BitcoinAddressData address)? bitcoinAddress,
    TResult? Function(LNInvoice invoice)? bolt11,
    TResult? Function(String nodeId)? nodeId,
    TResult? Function(String url)? url,
    TResult? Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult? Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult? Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult? Function(LnUrlErrorData data)? lnUrlError,
  }) {
    return bitcoinAddress?.call(address);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BitcoinAddressData address)? bitcoinAddress,
    TResult Function(LNInvoice invoice)? bolt11,
    TResult Function(String nodeId)? nodeId,
    TResult Function(String url)? url,
    TResult Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult Function(LnUrlErrorData data)? lnUrlError,
    required TResult orElse(),
  }) {
    if (bitcoinAddress != null) {
      return bitcoinAddress(address);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InputType_BitcoinAddress value) bitcoinAddress,
    required TResult Function(InputType_Bolt11 value) bolt11,
    required TResult Function(InputType_NodeId value) nodeId,
    required TResult Function(InputType_Url value) url,
    required TResult Function(InputType_LnUrlPay value) lnUrlPay,
    required TResult Function(InputType_LnUrlWithdraw value) lnUrlWithdraw,
    required TResult Function(InputType_LnUrlAuth value) lnUrlAuth,
    required TResult Function(InputType_LnUrlError value) lnUrlError,
  }) {
    return bitcoinAddress(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult? Function(InputType_Bolt11 value)? bolt11,
    TResult? Function(InputType_NodeId value)? nodeId,
    TResult? Function(InputType_Url value)? url,
    TResult? Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult? Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult? Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult? Function(InputType_LnUrlError value)? lnUrlError,
  }) {
    return bitcoinAddress?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult Function(InputType_Bolt11 value)? bolt11,
    TResult Function(InputType_NodeId value)? nodeId,
    TResult Function(InputType_Url value)? url,
    TResult Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult Function(InputType_LnUrlError value)? lnUrlError,
    required TResult orElse(),
  }) {
    if (bitcoinAddress != null) {
      return bitcoinAddress(this);
    }
    return orElse();
  }
}

abstract class InputType_BitcoinAddress implements InputType {
  const factory InputType_BitcoinAddress({required final BitcoinAddressData address}) =
      _$InputType_BitcoinAddressImpl;

  BitcoinAddressData get address;
  @JsonKey(ignore: true)
  _$$InputType_BitcoinAddressImplCopyWith<_$InputType_BitcoinAddressImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_Bolt11ImplCopyWith<$Res> {
  factory _$$InputType_Bolt11ImplCopyWith(
          _$InputType_Bolt11Impl value, $Res Function(_$InputType_Bolt11Impl) then) =
      __$$InputType_Bolt11ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LNInvoice invoice});

  $LNInvoiceCopyWith<$Res> get invoice;
}

/// @nodoc
class __$$InputType_Bolt11ImplCopyWithImpl<$Res> extends _$InputTypeCopyWithImpl<$Res, _$InputType_Bolt11Impl>
    implements _$$InputType_Bolt11ImplCopyWith<$Res> {
  __$$InputType_Bolt11ImplCopyWithImpl(
      _$InputType_Bolt11Impl _value, $Res Function(_$InputType_Bolt11Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? invoice = null,
  }) {
    return _then(_$InputType_Bolt11Impl(
      invoice: null == invoice
          ? _value.invoice
          : invoice // ignore: cast_nullable_to_non_nullable
              as LNInvoice,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $LNInvoiceCopyWith<$Res> get invoice {
    return $LNInvoiceCopyWith<$Res>(_value.invoice, (value) {
      return _then(_value.copyWith(invoice: value));
    });
  }
}

/// @nodoc

class _$InputType_Bolt11Impl implements InputType_Bolt11 {
  const _$InputType_Bolt11Impl({required this.invoice});

  @override
  final LNInvoice invoice;

  @override
  String toString() {
    return 'InputType.bolt11(invoice: $invoice)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_Bolt11Impl &&
            (identical(other.invoice, invoice) || other.invoice == invoice));
  }

  @override
  int get hashCode => Object.hash(runtimeType, invoice);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_Bolt11ImplCopyWith<_$InputType_Bolt11Impl> get copyWith =>
      __$$InputType_Bolt11ImplCopyWithImpl<_$InputType_Bolt11Impl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BitcoinAddressData address) bitcoinAddress,
    required TResult Function(LNInvoice invoice) bolt11,
    required TResult Function(String nodeId) nodeId,
    required TResult Function(String url) url,
    required TResult Function(LnUrlPayRequestData data) lnUrlPay,
    required TResult Function(LnUrlWithdrawRequestData data) lnUrlWithdraw,
    required TResult Function(LnUrlAuthRequestData data) lnUrlAuth,
    required TResult Function(LnUrlErrorData data) lnUrlError,
  }) {
    return bolt11(invoice);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BitcoinAddressData address)? bitcoinAddress,
    TResult? Function(LNInvoice invoice)? bolt11,
    TResult? Function(String nodeId)? nodeId,
    TResult? Function(String url)? url,
    TResult? Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult? Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult? Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult? Function(LnUrlErrorData data)? lnUrlError,
  }) {
    return bolt11?.call(invoice);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BitcoinAddressData address)? bitcoinAddress,
    TResult Function(LNInvoice invoice)? bolt11,
    TResult Function(String nodeId)? nodeId,
    TResult Function(String url)? url,
    TResult Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult Function(LnUrlErrorData data)? lnUrlError,
    required TResult orElse(),
  }) {
    if (bolt11 != null) {
      return bolt11(invoice);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InputType_BitcoinAddress value) bitcoinAddress,
    required TResult Function(InputType_Bolt11 value) bolt11,
    required TResult Function(InputType_NodeId value) nodeId,
    required TResult Function(InputType_Url value) url,
    required TResult Function(InputType_LnUrlPay value) lnUrlPay,
    required TResult Function(InputType_LnUrlWithdraw value) lnUrlWithdraw,
    required TResult Function(InputType_LnUrlAuth value) lnUrlAuth,
    required TResult Function(InputType_LnUrlError value) lnUrlError,
  }) {
    return bolt11(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult? Function(InputType_Bolt11 value)? bolt11,
    TResult? Function(InputType_NodeId value)? nodeId,
    TResult? Function(InputType_Url value)? url,
    TResult? Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult? Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult? Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult? Function(InputType_LnUrlError value)? lnUrlError,
  }) {
    return bolt11?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult Function(InputType_Bolt11 value)? bolt11,
    TResult Function(InputType_NodeId value)? nodeId,
    TResult Function(InputType_Url value)? url,
    TResult Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult Function(InputType_LnUrlError value)? lnUrlError,
    required TResult orElse(),
  }) {
    if (bolt11 != null) {
      return bolt11(this);
    }
    return orElse();
  }
}

abstract class InputType_Bolt11 implements InputType {
  const factory InputType_Bolt11({required final LNInvoice invoice}) = _$InputType_Bolt11Impl;

  LNInvoice get invoice;
  @JsonKey(ignore: true)
  _$$InputType_Bolt11ImplCopyWith<_$InputType_Bolt11Impl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_NodeIdImplCopyWith<$Res> {
  factory _$$InputType_NodeIdImplCopyWith(
          _$InputType_NodeIdImpl value, $Res Function(_$InputType_NodeIdImpl) then) =
      __$$InputType_NodeIdImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String nodeId});
}

/// @nodoc
class __$$InputType_NodeIdImplCopyWithImpl<$Res> extends _$InputTypeCopyWithImpl<$Res, _$InputType_NodeIdImpl>
    implements _$$InputType_NodeIdImplCopyWith<$Res> {
  __$$InputType_NodeIdImplCopyWithImpl(
      _$InputType_NodeIdImpl _value, $Res Function(_$InputType_NodeIdImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? nodeId = null,
  }) {
    return _then(_$InputType_NodeIdImpl(
      nodeId: null == nodeId
          ? _value.nodeId
          : nodeId // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$InputType_NodeIdImpl implements InputType_NodeId {
  const _$InputType_NodeIdImpl({required this.nodeId});

  @override
  final String nodeId;

  @override
  String toString() {
    return 'InputType.nodeId(nodeId: $nodeId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_NodeIdImpl &&
            (identical(other.nodeId, nodeId) || other.nodeId == nodeId));
  }

  @override
  int get hashCode => Object.hash(runtimeType, nodeId);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_NodeIdImplCopyWith<_$InputType_NodeIdImpl> get copyWith =>
      __$$InputType_NodeIdImplCopyWithImpl<_$InputType_NodeIdImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BitcoinAddressData address) bitcoinAddress,
    required TResult Function(LNInvoice invoice) bolt11,
    required TResult Function(String nodeId) nodeId,
    required TResult Function(String url) url,
    required TResult Function(LnUrlPayRequestData data) lnUrlPay,
    required TResult Function(LnUrlWithdrawRequestData data) lnUrlWithdraw,
    required TResult Function(LnUrlAuthRequestData data) lnUrlAuth,
    required TResult Function(LnUrlErrorData data) lnUrlError,
  }) {
    return nodeId(this.nodeId);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BitcoinAddressData address)? bitcoinAddress,
    TResult? Function(LNInvoice invoice)? bolt11,
    TResult? Function(String nodeId)? nodeId,
    TResult? Function(String url)? url,
    TResult? Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult? Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult? Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult? Function(LnUrlErrorData data)? lnUrlError,
  }) {
    return nodeId?.call(this.nodeId);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BitcoinAddressData address)? bitcoinAddress,
    TResult Function(LNInvoice invoice)? bolt11,
    TResult Function(String nodeId)? nodeId,
    TResult Function(String url)? url,
    TResult Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult Function(LnUrlErrorData data)? lnUrlError,
    required TResult orElse(),
  }) {
    if (nodeId != null) {
      return nodeId(this.nodeId);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InputType_BitcoinAddress value) bitcoinAddress,
    required TResult Function(InputType_Bolt11 value) bolt11,
    required TResult Function(InputType_NodeId value) nodeId,
    required TResult Function(InputType_Url value) url,
    required TResult Function(InputType_LnUrlPay value) lnUrlPay,
    required TResult Function(InputType_LnUrlWithdraw value) lnUrlWithdraw,
    required TResult Function(InputType_LnUrlAuth value) lnUrlAuth,
    required TResult Function(InputType_LnUrlError value) lnUrlError,
  }) {
    return nodeId(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult? Function(InputType_Bolt11 value)? bolt11,
    TResult? Function(InputType_NodeId value)? nodeId,
    TResult? Function(InputType_Url value)? url,
    TResult? Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult? Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult? Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult? Function(InputType_LnUrlError value)? lnUrlError,
  }) {
    return nodeId?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult Function(InputType_Bolt11 value)? bolt11,
    TResult Function(InputType_NodeId value)? nodeId,
    TResult Function(InputType_Url value)? url,
    TResult Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult Function(InputType_LnUrlError value)? lnUrlError,
    required TResult orElse(),
  }) {
    if (nodeId != null) {
      return nodeId(this);
    }
    return orElse();
  }
}

abstract class InputType_NodeId implements InputType {
  const factory InputType_NodeId({required final String nodeId}) = _$InputType_NodeIdImpl;

  String get nodeId;
  @JsonKey(ignore: true)
  _$$InputType_NodeIdImplCopyWith<_$InputType_NodeIdImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_UrlImplCopyWith<$Res> {
  factory _$$InputType_UrlImplCopyWith(_$InputType_UrlImpl value, $Res Function(_$InputType_UrlImpl) then) =
      __$$InputType_UrlImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String url});
}

/// @nodoc
class __$$InputType_UrlImplCopyWithImpl<$Res> extends _$InputTypeCopyWithImpl<$Res, _$InputType_UrlImpl>
    implements _$$InputType_UrlImplCopyWith<$Res> {
  __$$InputType_UrlImplCopyWithImpl(_$InputType_UrlImpl _value, $Res Function(_$InputType_UrlImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? url = null,
  }) {
    return _then(_$InputType_UrlImpl(
      url: null == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$InputType_UrlImpl implements InputType_Url {
  const _$InputType_UrlImpl({required this.url});

  @override
  final String url;

  @override
  String toString() {
    return 'InputType.url(url: $url)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_UrlImpl &&
            (identical(other.url, url) || other.url == url));
  }

  @override
  int get hashCode => Object.hash(runtimeType, url);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_UrlImplCopyWith<_$InputType_UrlImpl> get copyWith =>
      __$$InputType_UrlImplCopyWithImpl<_$InputType_UrlImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BitcoinAddressData address) bitcoinAddress,
    required TResult Function(LNInvoice invoice) bolt11,
    required TResult Function(String nodeId) nodeId,
    required TResult Function(String url) url,
    required TResult Function(LnUrlPayRequestData data) lnUrlPay,
    required TResult Function(LnUrlWithdrawRequestData data) lnUrlWithdraw,
    required TResult Function(LnUrlAuthRequestData data) lnUrlAuth,
    required TResult Function(LnUrlErrorData data) lnUrlError,
  }) {
    return url(this.url);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BitcoinAddressData address)? bitcoinAddress,
    TResult? Function(LNInvoice invoice)? bolt11,
    TResult? Function(String nodeId)? nodeId,
    TResult? Function(String url)? url,
    TResult? Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult? Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult? Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult? Function(LnUrlErrorData data)? lnUrlError,
  }) {
    return url?.call(this.url);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BitcoinAddressData address)? bitcoinAddress,
    TResult Function(LNInvoice invoice)? bolt11,
    TResult Function(String nodeId)? nodeId,
    TResult Function(String url)? url,
    TResult Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult Function(LnUrlErrorData data)? lnUrlError,
    required TResult orElse(),
  }) {
    if (url != null) {
      return url(this.url);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InputType_BitcoinAddress value) bitcoinAddress,
    required TResult Function(InputType_Bolt11 value) bolt11,
    required TResult Function(InputType_NodeId value) nodeId,
    required TResult Function(InputType_Url value) url,
    required TResult Function(InputType_LnUrlPay value) lnUrlPay,
    required TResult Function(InputType_LnUrlWithdraw value) lnUrlWithdraw,
    required TResult Function(InputType_LnUrlAuth value) lnUrlAuth,
    required TResult Function(InputType_LnUrlError value) lnUrlError,
  }) {
    return url(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult? Function(InputType_Bolt11 value)? bolt11,
    TResult? Function(InputType_NodeId value)? nodeId,
    TResult? Function(InputType_Url value)? url,
    TResult? Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult? Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult? Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult? Function(InputType_LnUrlError value)? lnUrlError,
  }) {
    return url?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult Function(InputType_Bolt11 value)? bolt11,
    TResult Function(InputType_NodeId value)? nodeId,
    TResult Function(InputType_Url value)? url,
    TResult Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult Function(InputType_LnUrlError value)? lnUrlError,
    required TResult orElse(),
  }) {
    if (url != null) {
      return url(this);
    }
    return orElse();
  }
}

abstract class InputType_Url implements InputType {
  const factory InputType_Url({required final String url}) = _$InputType_UrlImpl;

  String get url;
  @JsonKey(ignore: true)
  _$$InputType_UrlImplCopyWith<_$InputType_UrlImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlPayImplCopyWith<$Res> {
  factory _$$InputType_LnUrlPayImplCopyWith(
          _$InputType_LnUrlPayImpl value, $Res Function(_$InputType_LnUrlPayImpl) then) =
      __$$InputType_LnUrlPayImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlPayRequestData data});

  $LnUrlPayRequestDataCopyWith<$Res> get data;
}

/// @nodoc
class __$$InputType_LnUrlPayImplCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlPayImpl>
    implements _$$InputType_LnUrlPayImplCopyWith<$Res> {
  __$$InputType_LnUrlPayImplCopyWithImpl(
      _$InputType_LnUrlPayImpl _value, $Res Function(_$InputType_LnUrlPayImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$InputType_LnUrlPayImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlPayRequestData,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $LnUrlPayRequestDataCopyWith<$Res> get data {
    return $LnUrlPayRequestDataCopyWith<$Res>(_value.data, (value) {
      return _then(_value.copyWith(data: value));
    });
  }
}

/// @nodoc

class _$InputType_LnUrlPayImpl implements InputType_LnUrlPay {
  const _$InputType_LnUrlPayImpl({required this.data});

  @override
  final LnUrlPayRequestData data;

  @override
  String toString() {
    return 'InputType.lnUrlPay(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_LnUrlPayImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_LnUrlPayImplCopyWith<_$InputType_LnUrlPayImpl> get copyWith =>
      __$$InputType_LnUrlPayImplCopyWithImpl<_$InputType_LnUrlPayImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BitcoinAddressData address) bitcoinAddress,
    required TResult Function(LNInvoice invoice) bolt11,
    required TResult Function(String nodeId) nodeId,
    required TResult Function(String url) url,
    required TResult Function(LnUrlPayRequestData data) lnUrlPay,
    required TResult Function(LnUrlWithdrawRequestData data) lnUrlWithdraw,
    required TResult Function(LnUrlAuthRequestData data) lnUrlAuth,
    required TResult Function(LnUrlErrorData data) lnUrlError,
  }) {
    return lnUrlPay(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BitcoinAddressData address)? bitcoinAddress,
    TResult? Function(LNInvoice invoice)? bolt11,
    TResult? Function(String nodeId)? nodeId,
    TResult? Function(String url)? url,
    TResult? Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult? Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult? Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult? Function(LnUrlErrorData data)? lnUrlError,
  }) {
    return lnUrlPay?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BitcoinAddressData address)? bitcoinAddress,
    TResult Function(LNInvoice invoice)? bolt11,
    TResult Function(String nodeId)? nodeId,
    TResult Function(String url)? url,
    TResult Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult Function(LnUrlErrorData data)? lnUrlError,
    required TResult orElse(),
  }) {
    if (lnUrlPay != null) {
      return lnUrlPay(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InputType_BitcoinAddress value) bitcoinAddress,
    required TResult Function(InputType_Bolt11 value) bolt11,
    required TResult Function(InputType_NodeId value) nodeId,
    required TResult Function(InputType_Url value) url,
    required TResult Function(InputType_LnUrlPay value) lnUrlPay,
    required TResult Function(InputType_LnUrlWithdraw value) lnUrlWithdraw,
    required TResult Function(InputType_LnUrlAuth value) lnUrlAuth,
    required TResult Function(InputType_LnUrlError value) lnUrlError,
  }) {
    return lnUrlPay(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult? Function(InputType_Bolt11 value)? bolt11,
    TResult? Function(InputType_NodeId value)? nodeId,
    TResult? Function(InputType_Url value)? url,
    TResult? Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult? Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult? Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult? Function(InputType_LnUrlError value)? lnUrlError,
  }) {
    return lnUrlPay?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult Function(InputType_Bolt11 value)? bolt11,
    TResult Function(InputType_NodeId value)? nodeId,
    TResult Function(InputType_Url value)? url,
    TResult Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult Function(InputType_LnUrlError value)? lnUrlError,
    required TResult orElse(),
  }) {
    if (lnUrlPay != null) {
      return lnUrlPay(this);
    }
    return orElse();
  }
}

abstract class InputType_LnUrlPay implements InputType {
  const factory InputType_LnUrlPay({required final LnUrlPayRequestData data}) = _$InputType_LnUrlPayImpl;

  LnUrlPayRequestData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlPayImplCopyWith<_$InputType_LnUrlPayImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlWithdrawImplCopyWith<$Res> {
  factory _$$InputType_LnUrlWithdrawImplCopyWith(
          _$InputType_LnUrlWithdrawImpl value, $Res Function(_$InputType_LnUrlWithdrawImpl) then) =
      __$$InputType_LnUrlWithdrawImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlWithdrawRequestData data});

  $LnUrlWithdrawRequestDataCopyWith<$Res> get data;
}

/// @nodoc
class __$$InputType_LnUrlWithdrawImplCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlWithdrawImpl>
    implements _$$InputType_LnUrlWithdrawImplCopyWith<$Res> {
  __$$InputType_LnUrlWithdrawImplCopyWithImpl(
      _$InputType_LnUrlWithdrawImpl _value, $Res Function(_$InputType_LnUrlWithdrawImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$InputType_LnUrlWithdrawImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlWithdrawRequestData,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $LnUrlWithdrawRequestDataCopyWith<$Res> get data {
    return $LnUrlWithdrawRequestDataCopyWith<$Res>(_value.data, (value) {
      return _then(_value.copyWith(data: value));
    });
  }
}

/// @nodoc

class _$InputType_LnUrlWithdrawImpl implements InputType_LnUrlWithdraw {
  const _$InputType_LnUrlWithdrawImpl({required this.data});

  @override
  final LnUrlWithdrawRequestData data;

  @override
  String toString() {
    return 'InputType.lnUrlWithdraw(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_LnUrlWithdrawImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_LnUrlWithdrawImplCopyWith<_$InputType_LnUrlWithdrawImpl> get copyWith =>
      __$$InputType_LnUrlWithdrawImplCopyWithImpl<_$InputType_LnUrlWithdrawImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BitcoinAddressData address) bitcoinAddress,
    required TResult Function(LNInvoice invoice) bolt11,
    required TResult Function(String nodeId) nodeId,
    required TResult Function(String url) url,
    required TResult Function(LnUrlPayRequestData data) lnUrlPay,
    required TResult Function(LnUrlWithdrawRequestData data) lnUrlWithdraw,
    required TResult Function(LnUrlAuthRequestData data) lnUrlAuth,
    required TResult Function(LnUrlErrorData data) lnUrlError,
  }) {
    return lnUrlWithdraw(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BitcoinAddressData address)? bitcoinAddress,
    TResult? Function(LNInvoice invoice)? bolt11,
    TResult? Function(String nodeId)? nodeId,
    TResult? Function(String url)? url,
    TResult? Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult? Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult? Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult? Function(LnUrlErrorData data)? lnUrlError,
  }) {
    return lnUrlWithdraw?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BitcoinAddressData address)? bitcoinAddress,
    TResult Function(LNInvoice invoice)? bolt11,
    TResult Function(String nodeId)? nodeId,
    TResult Function(String url)? url,
    TResult Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult Function(LnUrlErrorData data)? lnUrlError,
    required TResult orElse(),
  }) {
    if (lnUrlWithdraw != null) {
      return lnUrlWithdraw(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InputType_BitcoinAddress value) bitcoinAddress,
    required TResult Function(InputType_Bolt11 value) bolt11,
    required TResult Function(InputType_NodeId value) nodeId,
    required TResult Function(InputType_Url value) url,
    required TResult Function(InputType_LnUrlPay value) lnUrlPay,
    required TResult Function(InputType_LnUrlWithdraw value) lnUrlWithdraw,
    required TResult Function(InputType_LnUrlAuth value) lnUrlAuth,
    required TResult Function(InputType_LnUrlError value) lnUrlError,
  }) {
    return lnUrlWithdraw(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult? Function(InputType_Bolt11 value)? bolt11,
    TResult? Function(InputType_NodeId value)? nodeId,
    TResult? Function(InputType_Url value)? url,
    TResult? Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult? Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult? Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult? Function(InputType_LnUrlError value)? lnUrlError,
  }) {
    return lnUrlWithdraw?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult Function(InputType_Bolt11 value)? bolt11,
    TResult Function(InputType_NodeId value)? nodeId,
    TResult Function(InputType_Url value)? url,
    TResult Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult Function(InputType_LnUrlError value)? lnUrlError,
    required TResult orElse(),
  }) {
    if (lnUrlWithdraw != null) {
      return lnUrlWithdraw(this);
    }
    return orElse();
  }
}

abstract class InputType_LnUrlWithdraw implements InputType {
  const factory InputType_LnUrlWithdraw({required final LnUrlWithdrawRequestData data}) =
      _$InputType_LnUrlWithdrawImpl;

  LnUrlWithdrawRequestData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlWithdrawImplCopyWith<_$InputType_LnUrlWithdrawImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlAuthImplCopyWith<$Res> {
  factory _$$InputType_LnUrlAuthImplCopyWith(
          _$InputType_LnUrlAuthImpl value, $Res Function(_$InputType_LnUrlAuthImpl) then) =
      __$$InputType_LnUrlAuthImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlAuthRequestData data});

  $LnUrlAuthRequestDataCopyWith<$Res> get data;
}

/// @nodoc
class __$$InputType_LnUrlAuthImplCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlAuthImpl>
    implements _$$InputType_LnUrlAuthImplCopyWith<$Res> {
  __$$InputType_LnUrlAuthImplCopyWithImpl(
      _$InputType_LnUrlAuthImpl _value, $Res Function(_$InputType_LnUrlAuthImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$InputType_LnUrlAuthImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlAuthRequestData,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $LnUrlAuthRequestDataCopyWith<$Res> get data {
    return $LnUrlAuthRequestDataCopyWith<$Res>(_value.data, (value) {
      return _then(_value.copyWith(data: value));
    });
  }
}

/// @nodoc

class _$InputType_LnUrlAuthImpl implements InputType_LnUrlAuth {
  const _$InputType_LnUrlAuthImpl({required this.data});

  @override
  final LnUrlAuthRequestData data;

  @override
  String toString() {
    return 'InputType.lnUrlAuth(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_LnUrlAuthImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_LnUrlAuthImplCopyWith<_$InputType_LnUrlAuthImpl> get copyWith =>
      __$$InputType_LnUrlAuthImplCopyWithImpl<_$InputType_LnUrlAuthImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BitcoinAddressData address) bitcoinAddress,
    required TResult Function(LNInvoice invoice) bolt11,
    required TResult Function(String nodeId) nodeId,
    required TResult Function(String url) url,
    required TResult Function(LnUrlPayRequestData data) lnUrlPay,
    required TResult Function(LnUrlWithdrawRequestData data) lnUrlWithdraw,
    required TResult Function(LnUrlAuthRequestData data) lnUrlAuth,
    required TResult Function(LnUrlErrorData data) lnUrlError,
  }) {
    return lnUrlAuth(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BitcoinAddressData address)? bitcoinAddress,
    TResult? Function(LNInvoice invoice)? bolt11,
    TResult? Function(String nodeId)? nodeId,
    TResult? Function(String url)? url,
    TResult? Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult? Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult? Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult? Function(LnUrlErrorData data)? lnUrlError,
  }) {
    return lnUrlAuth?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BitcoinAddressData address)? bitcoinAddress,
    TResult Function(LNInvoice invoice)? bolt11,
    TResult Function(String nodeId)? nodeId,
    TResult Function(String url)? url,
    TResult Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult Function(LnUrlErrorData data)? lnUrlError,
    required TResult orElse(),
  }) {
    if (lnUrlAuth != null) {
      return lnUrlAuth(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InputType_BitcoinAddress value) bitcoinAddress,
    required TResult Function(InputType_Bolt11 value) bolt11,
    required TResult Function(InputType_NodeId value) nodeId,
    required TResult Function(InputType_Url value) url,
    required TResult Function(InputType_LnUrlPay value) lnUrlPay,
    required TResult Function(InputType_LnUrlWithdraw value) lnUrlWithdraw,
    required TResult Function(InputType_LnUrlAuth value) lnUrlAuth,
    required TResult Function(InputType_LnUrlError value) lnUrlError,
  }) {
    return lnUrlAuth(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult? Function(InputType_Bolt11 value)? bolt11,
    TResult? Function(InputType_NodeId value)? nodeId,
    TResult? Function(InputType_Url value)? url,
    TResult? Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult? Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult? Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult? Function(InputType_LnUrlError value)? lnUrlError,
  }) {
    return lnUrlAuth?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult Function(InputType_Bolt11 value)? bolt11,
    TResult Function(InputType_NodeId value)? nodeId,
    TResult Function(InputType_Url value)? url,
    TResult Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult Function(InputType_LnUrlError value)? lnUrlError,
    required TResult orElse(),
  }) {
    if (lnUrlAuth != null) {
      return lnUrlAuth(this);
    }
    return orElse();
  }
}

abstract class InputType_LnUrlAuth implements InputType {
  const factory InputType_LnUrlAuth({required final LnUrlAuthRequestData data}) = _$InputType_LnUrlAuthImpl;

  LnUrlAuthRequestData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlAuthImplCopyWith<_$InputType_LnUrlAuthImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlErrorImplCopyWith<$Res> {
  factory _$$InputType_LnUrlErrorImplCopyWith(
          _$InputType_LnUrlErrorImpl value, $Res Function(_$InputType_LnUrlErrorImpl) then) =
      __$$InputType_LnUrlErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});

  $LnUrlErrorDataCopyWith<$Res> get data;
}

/// @nodoc
class __$$InputType_LnUrlErrorImplCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlErrorImpl>
    implements _$$InputType_LnUrlErrorImplCopyWith<$Res> {
  __$$InputType_LnUrlErrorImplCopyWithImpl(
      _$InputType_LnUrlErrorImpl _value, $Res Function(_$InputType_LnUrlErrorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$InputType_LnUrlErrorImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $LnUrlErrorDataCopyWith<$Res> get data {
    return $LnUrlErrorDataCopyWith<$Res>(_value.data, (value) {
      return _then(_value.copyWith(data: value));
    });
  }
}

/// @nodoc

class _$InputType_LnUrlErrorImpl implements InputType_LnUrlError {
  const _$InputType_LnUrlErrorImpl({required this.data});

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'InputType.lnUrlError(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_LnUrlErrorImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_LnUrlErrorImplCopyWith<_$InputType_LnUrlErrorImpl> get copyWith =>
      __$$InputType_LnUrlErrorImplCopyWithImpl<_$InputType_LnUrlErrorImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BitcoinAddressData address) bitcoinAddress,
    required TResult Function(LNInvoice invoice) bolt11,
    required TResult Function(String nodeId) nodeId,
    required TResult Function(String url) url,
    required TResult Function(LnUrlPayRequestData data) lnUrlPay,
    required TResult Function(LnUrlWithdrawRequestData data) lnUrlWithdraw,
    required TResult Function(LnUrlAuthRequestData data) lnUrlAuth,
    required TResult Function(LnUrlErrorData data) lnUrlError,
  }) {
    return lnUrlError(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BitcoinAddressData address)? bitcoinAddress,
    TResult? Function(LNInvoice invoice)? bolt11,
    TResult? Function(String nodeId)? nodeId,
    TResult? Function(String url)? url,
    TResult? Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult? Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult? Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult? Function(LnUrlErrorData data)? lnUrlError,
  }) {
    return lnUrlError?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BitcoinAddressData address)? bitcoinAddress,
    TResult Function(LNInvoice invoice)? bolt11,
    TResult Function(String nodeId)? nodeId,
    TResult Function(String url)? url,
    TResult Function(LnUrlPayRequestData data)? lnUrlPay,
    TResult Function(LnUrlWithdrawRequestData data)? lnUrlWithdraw,
    TResult Function(LnUrlAuthRequestData data)? lnUrlAuth,
    TResult Function(LnUrlErrorData data)? lnUrlError,
    required TResult orElse(),
  }) {
    if (lnUrlError != null) {
      return lnUrlError(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InputType_BitcoinAddress value) bitcoinAddress,
    required TResult Function(InputType_Bolt11 value) bolt11,
    required TResult Function(InputType_NodeId value) nodeId,
    required TResult Function(InputType_Url value) url,
    required TResult Function(InputType_LnUrlPay value) lnUrlPay,
    required TResult Function(InputType_LnUrlWithdraw value) lnUrlWithdraw,
    required TResult Function(InputType_LnUrlAuth value) lnUrlAuth,
    required TResult Function(InputType_LnUrlError value) lnUrlError,
  }) {
    return lnUrlError(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult? Function(InputType_Bolt11 value)? bolt11,
    TResult? Function(InputType_NodeId value)? nodeId,
    TResult? Function(InputType_Url value)? url,
    TResult? Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult? Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult? Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult? Function(InputType_LnUrlError value)? lnUrlError,
  }) {
    return lnUrlError?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputType_BitcoinAddress value)? bitcoinAddress,
    TResult Function(InputType_Bolt11 value)? bolt11,
    TResult Function(InputType_NodeId value)? nodeId,
    TResult Function(InputType_Url value)? url,
    TResult Function(InputType_LnUrlPay value)? lnUrlPay,
    TResult Function(InputType_LnUrlWithdraw value)? lnUrlWithdraw,
    TResult Function(InputType_LnUrlAuth value)? lnUrlAuth,
    TResult Function(InputType_LnUrlError value)? lnUrlError,
    required TResult orElse(),
  }) {
    if (lnUrlError != null) {
      return lnUrlError(this);
    }
    return orElse();
  }
}

abstract class InputType_LnUrlError implements InputType {
  const factory InputType_LnUrlError({required final LnUrlErrorData data}) = _$InputType_LnUrlErrorImpl;

  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlErrorImplCopyWith<_$InputType_LnUrlErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$ListPaymentsRequest {
  List<PaymentTypeFilter>? get filters => throw _privateConstructorUsedError;
  List<MetadataFilter>? get metadataFilters => throw _privateConstructorUsedError;
  int? get fromTimestamp => throw _privateConstructorUsedError;
  int? get toTimestamp => throw _privateConstructorUsedError;
  bool? get includeFailures => throw _privateConstructorUsedError;
  int? get offset => throw _privateConstructorUsedError;
  int? get limit => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ListPaymentsRequestCopyWith<ListPaymentsRequest> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ListPaymentsRequestCopyWith<$Res> {
  factory $ListPaymentsRequestCopyWith(ListPaymentsRequest value, $Res Function(ListPaymentsRequest) then) =
      _$ListPaymentsRequestCopyWithImpl<$Res, ListPaymentsRequest>;
  @useResult
  $Res call(
      {List<PaymentTypeFilter>? filters,
      List<MetadataFilter>? metadataFilters,
      int? fromTimestamp,
      int? toTimestamp,
      bool? includeFailures,
      int? offset,
      int? limit});
}

/// @nodoc
class _$ListPaymentsRequestCopyWithImpl<$Res, $Val extends ListPaymentsRequest>
    implements $ListPaymentsRequestCopyWith<$Res> {
  _$ListPaymentsRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? filters = freezed,
    Object? metadataFilters = freezed,
    Object? fromTimestamp = freezed,
    Object? toTimestamp = freezed,
    Object? includeFailures = freezed,
    Object? offset = freezed,
    Object? limit = freezed,
  }) {
    return _then(_value.copyWith(
      filters: freezed == filters
          ? _value.filters
          : filters // ignore: cast_nullable_to_non_nullable
              as List<PaymentTypeFilter>?,
      metadataFilters: freezed == metadataFilters
          ? _value.metadataFilters
          : metadataFilters // ignore: cast_nullable_to_non_nullable
              as List<MetadataFilter>?,
      fromTimestamp: freezed == fromTimestamp
          ? _value.fromTimestamp
          : fromTimestamp // ignore: cast_nullable_to_non_nullable
              as int?,
      toTimestamp: freezed == toTimestamp
          ? _value.toTimestamp
          : toTimestamp // ignore: cast_nullable_to_non_nullable
              as int?,
      includeFailures: freezed == includeFailures
          ? _value.includeFailures
          : includeFailures // ignore: cast_nullable_to_non_nullable
              as bool?,
      offset: freezed == offset
          ? _value.offset
          : offset // ignore: cast_nullable_to_non_nullable
              as int?,
      limit: freezed == limit
          ? _value.limit
          : limit // ignore: cast_nullable_to_non_nullable
              as int?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ListPaymentsRequestImplCopyWith<$Res> implements $ListPaymentsRequestCopyWith<$Res> {
  factory _$$ListPaymentsRequestImplCopyWith(
          _$ListPaymentsRequestImpl value, $Res Function(_$ListPaymentsRequestImpl) then) =
      __$$ListPaymentsRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {List<PaymentTypeFilter>? filters,
      List<MetadataFilter>? metadataFilters,
      int? fromTimestamp,
      int? toTimestamp,
      bool? includeFailures,
      int? offset,
      int? limit});
}

/// @nodoc
class __$$ListPaymentsRequestImplCopyWithImpl<$Res>
    extends _$ListPaymentsRequestCopyWithImpl<$Res, _$ListPaymentsRequestImpl>
    implements _$$ListPaymentsRequestImplCopyWith<$Res> {
  __$$ListPaymentsRequestImplCopyWithImpl(
      _$ListPaymentsRequestImpl _value, $Res Function(_$ListPaymentsRequestImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? filters = freezed,
    Object? metadataFilters = freezed,
    Object? fromTimestamp = freezed,
    Object? toTimestamp = freezed,
    Object? includeFailures = freezed,
    Object? offset = freezed,
    Object? limit = freezed,
  }) {
    return _then(_$ListPaymentsRequestImpl(
      filters: freezed == filters
          ? _value._filters
          : filters // ignore: cast_nullable_to_non_nullable
              as List<PaymentTypeFilter>?,
      metadataFilters: freezed == metadataFilters
          ? _value._metadataFilters
          : metadataFilters // ignore: cast_nullable_to_non_nullable
              as List<MetadataFilter>?,
      fromTimestamp: freezed == fromTimestamp
          ? _value.fromTimestamp
          : fromTimestamp // ignore: cast_nullable_to_non_nullable
              as int?,
      toTimestamp: freezed == toTimestamp
          ? _value.toTimestamp
          : toTimestamp // ignore: cast_nullable_to_non_nullable
              as int?,
      includeFailures: freezed == includeFailures
          ? _value.includeFailures
          : includeFailures // ignore: cast_nullable_to_non_nullable
              as bool?,
      offset: freezed == offset
          ? _value.offset
          : offset // ignore: cast_nullable_to_non_nullable
              as int?,
      limit: freezed == limit
          ? _value.limit
          : limit // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$ListPaymentsRequestImpl implements _ListPaymentsRequest {
  const _$ListPaymentsRequestImpl(
      {final List<PaymentTypeFilter>? filters,
      final List<MetadataFilter>? metadataFilters,
      this.fromTimestamp,
      this.toTimestamp,
      this.includeFailures,
      this.offset,
      this.limit})
      : _filters = filters,
        _metadataFilters = metadataFilters;

  final List<PaymentTypeFilter>? _filters;
  @override
  List<PaymentTypeFilter>? get filters {
    final value = _filters;
    if (value == null) return null;
    if (_filters is EqualUnmodifiableListView) return _filters;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  final List<MetadataFilter>? _metadataFilters;
  @override
  List<MetadataFilter>? get metadataFilters {
    final value = _metadataFilters;
    if (value == null) return null;
    if (_metadataFilters is EqualUnmodifiableListView) return _metadataFilters;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  @override
  final int? fromTimestamp;
  @override
  final int? toTimestamp;
  @override
  final bool? includeFailures;
  @override
  final int? offset;
  @override
  final int? limit;

  @override
  String toString() {
    return 'ListPaymentsRequest(filters: $filters, metadataFilters: $metadataFilters, fromTimestamp: $fromTimestamp, toTimestamp: $toTimestamp, includeFailures: $includeFailures, offset: $offset, limit: $limit)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ListPaymentsRequestImpl &&
            const DeepCollectionEquality().equals(other._filters, _filters) &&
            const DeepCollectionEquality().equals(other._metadataFilters, _metadataFilters) &&
            (identical(other.fromTimestamp, fromTimestamp) || other.fromTimestamp == fromTimestamp) &&
            (identical(other.toTimestamp, toTimestamp) || other.toTimestamp == toTimestamp) &&
            (identical(other.includeFailures, includeFailures) || other.includeFailures == includeFailures) &&
            (identical(other.offset, offset) || other.offset == offset) &&
            (identical(other.limit, limit) || other.limit == limit));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(_filters),
      const DeepCollectionEquality().hash(_metadataFilters),
      fromTimestamp,
      toTimestamp,
      includeFailures,
      offset,
      limit);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ListPaymentsRequestImplCopyWith<_$ListPaymentsRequestImpl> get copyWith =>
      __$$ListPaymentsRequestImplCopyWithImpl<_$ListPaymentsRequestImpl>(this, _$identity);
}

abstract class _ListPaymentsRequest implements ListPaymentsRequest {
  const factory _ListPaymentsRequest(
      {final List<PaymentTypeFilter>? filters,
      final List<MetadataFilter>? metadataFilters,
      final int? fromTimestamp,
      final int? toTimestamp,
      final bool? includeFailures,
      final int? offset,
      final int? limit}) = _$ListPaymentsRequestImpl;

  @override
  List<PaymentTypeFilter>? get filters;
  @override
  List<MetadataFilter>? get metadataFilters;
  @override
  int? get fromTimestamp;
  @override
  int? get toTimestamp;
  @override
  bool? get includeFailures;
  @override
  int? get offset;
  @override
  int? get limit;
  @override
  @JsonKey(ignore: true)
  _$$ListPaymentsRequestImplCopyWith<_$ListPaymentsRequestImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LNInvoice {
  String get bolt11 => throw _privateConstructorUsedError;
  Network get network => throw _privateConstructorUsedError;
  String get payeePubkey => throw _privateConstructorUsedError;
  String get paymentHash => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;
  String? get descriptionHash => throw _privateConstructorUsedError;
  int? get amountMsat => throw _privateConstructorUsedError;
  int get timestamp => throw _privateConstructorUsedError;
  int get expiry => throw _privateConstructorUsedError;
  List<RouteHint> get routingHints => throw _privateConstructorUsedError;
  Uint8List get paymentSecret => throw _privateConstructorUsedError;
  int get minFinalCltvExpiryDelta => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LNInvoiceCopyWith<LNInvoice> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LNInvoiceCopyWith<$Res> {
  factory $LNInvoiceCopyWith(LNInvoice value, $Res Function(LNInvoice) then) =
      _$LNInvoiceCopyWithImpl<$Res, LNInvoice>;
  @useResult
  $Res call(
      {String bolt11,
      Network network,
      String payeePubkey,
      String paymentHash,
      String? description,
      String? descriptionHash,
      int? amountMsat,
      int timestamp,
      int expiry,
      List<RouteHint> routingHints,
      Uint8List paymentSecret,
      int minFinalCltvExpiryDelta});
}

/// @nodoc
class _$LNInvoiceCopyWithImpl<$Res, $Val extends LNInvoice> implements $LNInvoiceCopyWith<$Res> {
  _$LNInvoiceCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? bolt11 = null,
    Object? network = null,
    Object? payeePubkey = null,
    Object? paymentHash = null,
    Object? description = freezed,
    Object? descriptionHash = freezed,
    Object? amountMsat = freezed,
    Object? timestamp = null,
    Object? expiry = null,
    Object? routingHints = null,
    Object? paymentSecret = null,
    Object? minFinalCltvExpiryDelta = null,
  }) {
    return _then(_value.copyWith(
      bolt11: null == bolt11
          ? _value.bolt11
          : bolt11 // ignore: cast_nullable_to_non_nullable
              as String,
      network: null == network
          ? _value.network
          : network // ignore: cast_nullable_to_non_nullable
              as Network,
      payeePubkey: null == payeePubkey
          ? _value.payeePubkey
          : payeePubkey // ignore: cast_nullable_to_non_nullable
              as String,
      paymentHash: null == paymentHash
          ? _value.paymentHash
          : paymentHash // ignore: cast_nullable_to_non_nullable
              as String,
      description: freezed == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String?,
      descriptionHash: freezed == descriptionHash
          ? _value.descriptionHash
          : descriptionHash // ignore: cast_nullable_to_non_nullable
              as String?,
      amountMsat: freezed == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int?,
      timestamp: null == timestamp
          ? _value.timestamp
          : timestamp // ignore: cast_nullable_to_non_nullable
              as int,
      expiry: null == expiry
          ? _value.expiry
          : expiry // ignore: cast_nullable_to_non_nullable
              as int,
      routingHints: null == routingHints
          ? _value.routingHints
          : routingHints // ignore: cast_nullable_to_non_nullable
              as List<RouteHint>,
      paymentSecret: null == paymentSecret
          ? _value.paymentSecret
          : paymentSecret // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      minFinalCltvExpiryDelta: null == minFinalCltvExpiryDelta
          ? _value.minFinalCltvExpiryDelta
          : minFinalCltvExpiryDelta // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$LNInvoiceImplCopyWith<$Res> implements $LNInvoiceCopyWith<$Res> {
  factory _$$LNInvoiceImplCopyWith(_$LNInvoiceImpl value, $Res Function(_$LNInvoiceImpl) then) =
      __$$LNInvoiceImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String bolt11,
      Network network,
      String payeePubkey,
      String paymentHash,
      String? description,
      String? descriptionHash,
      int? amountMsat,
      int timestamp,
      int expiry,
      List<RouteHint> routingHints,
      Uint8List paymentSecret,
      int minFinalCltvExpiryDelta});
}

/// @nodoc
class __$$LNInvoiceImplCopyWithImpl<$Res> extends _$LNInvoiceCopyWithImpl<$Res, _$LNInvoiceImpl>
    implements _$$LNInvoiceImplCopyWith<$Res> {
  __$$LNInvoiceImplCopyWithImpl(_$LNInvoiceImpl _value, $Res Function(_$LNInvoiceImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? bolt11 = null,
    Object? network = null,
    Object? payeePubkey = null,
    Object? paymentHash = null,
    Object? description = freezed,
    Object? descriptionHash = freezed,
    Object? amountMsat = freezed,
    Object? timestamp = null,
    Object? expiry = null,
    Object? routingHints = null,
    Object? paymentSecret = null,
    Object? minFinalCltvExpiryDelta = null,
  }) {
    return _then(_$LNInvoiceImpl(
      bolt11: null == bolt11
          ? _value.bolt11
          : bolt11 // ignore: cast_nullable_to_non_nullable
              as String,
      network: null == network
          ? _value.network
          : network // ignore: cast_nullable_to_non_nullable
              as Network,
      payeePubkey: null == payeePubkey
          ? _value.payeePubkey
          : payeePubkey // ignore: cast_nullable_to_non_nullable
              as String,
      paymentHash: null == paymentHash
          ? _value.paymentHash
          : paymentHash // ignore: cast_nullable_to_non_nullable
              as String,
      description: freezed == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String?,
      descriptionHash: freezed == descriptionHash
          ? _value.descriptionHash
          : descriptionHash // ignore: cast_nullable_to_non_nullable
              as String?,
      amountMsat: freezed == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int?,
      timestamp: null == timestamp
          ? _value.timestamp
          : timestamp // ignore: cast_nullable_to_non_nullable
              as int,
      expiry: null == expiry
          ? _value.expiry
          : expiry // ignore: cast_nullable_to_non_nullable
              as int,
      routingHints: null == routingHints
          ? _value._routingHints
          : routingHints // ignore: cast_nullable_to_non_nullable
              as List<RouteHint>,
      paymentSecret: null == paymentSecret
          ? _value.paymentSecret
          : paymentSecret // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      minFinalCltvExpiryDelta: null == minFinalCltvExpiryDelta
          ? _value.minFinalCltvExpiryDelta
          : minFinalCltvExpiryDelta // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$LNInvoiceImpl implements _LNInvoice {
  const _$LNInvoiceImpl(
      {required this.bolt11,
      required this.network,
      required this.payeePubkey,
      required this.paymentHash,
      this.description,
      this.descriptionHash,
      this.amountMsat,
      required this.timestamp,
      required this.expiry,
      required final List<RouteHint> routingHints,
      required this.paymentSecret,
      required this.minFinalCltvExpiryDelta})
      : _routingHints = routingHints;

  @override
  final String bolt11;
  @override
  final Network network;
  @override
  final String payeePubkey;
  @override
  final String paymentHash;
  @override
  final String? description;
  @override
  final String? descriptionHash;
  @override
  final int? amountMsat;
  @override
  final int timestamp;
  @override
  final int expiry;
  final List<RouteHint> _routingHints;
  @override
  List<RouteHint> get routingHints {
    if (_routingHints is EqualUnmodifiableListView) return _routingHints;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_routingHints);
  }

  @override
  final Uint8List paymentSecret;
  @override
  final int minFinalCltvExpiryDelta;

  @override
  String toString() {
    return 'LNInvoice(bolt11: $bolt11, network: $network, payeePubkey: $payeePubkey, paymentHash: $paymentHash, description: $description, descriptionHash: $descriptionHash, amountMsat: $amountMsat, timestamp: $timestamp, expiry: $expiry, routingHints: $routingHints, paymentSecret: $paymentSecret, minFinalCltvExpiryDelta: $minFinalCltvExpiryDelta)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LNInvoiceImpl &&
            (identical(other.bolt11, bolt11) || other.bolt11 == bolt11) &&
            (identical(other.network, network) || other.network == network) &&
            (identical(other.payeePubkey, payeePubkey) || other.payeePubkey == payeePubkey) &&
            (identical(other.paymentHash, paymentHash) || other.paymentHash == paymentHash) &&
            (identical(other.description, description) || other.description == description) &&
            (identical(other.descriptionHash, descriptionHash) || other.descriptionHash == descriptionHash) &&
            (identical(other.amountMsat, amountMsat) || other.amountMsat == amountMsat) &&
            (identical(other.timestamp, timestamp) || other.timestamp == timestamp) &&
            (identical(other.expiry, expiry) || other.expiry == expiry) &&
            const DeepCollectionEquality().equals(other._routingHints, _routingHints) &&
            const DeepCollectionEquality().equals(other.paymentSecret, paymentSecret) &&
            (identical(other.minFinalCltvExpiryDelta, minFinalCltvExpiryDelta) ||
                other.minFinalCltvExpiryDelta == minFinalCltvExpiryDelta));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      bolt11,
      network,
      payeePubkey,
      paymentHash,
      description,
      descriptionHash,
      amountMsat,
      timestamp,
      expiry,
      const DeepCollectionEquality().hash(_routingHints),
      const DeepCollectionEquality().hash(paymentSecret),
      minFinalCltvExpiryDelta);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LNInvoiceImplCopyWith<_$LNInvoiceImpl> get copyWith =>
      __$$LNInvoiceImplCopyWithImpl<_$LNInvoiceImpl>(this, _$identity);
}

abstract class _LNInvoice implements LNInvoice {
  const factory _LNInvoice(
      {required final String bolt11,
      required final Network network,
      required final String payeePubkey,
      required final String paymentHash,
      final String? description,
      final String? descriptionHash,
      final int? amountMsat,
      required final int timestamp,
      required final int expiry,
      required final List<RouteHint> routingHints,
      required final Uint8List paymentSecret,
      required final int minFinalCltvExpiryDelta}) = _$LNInvoiceImpl;

  @override
  String get bolt11;
  @override
  Network get network;
  @override
  String get payeePubkey;
  @override
  String get paymentHash;
  @override
  String? get description;
  @override
  String? get descriptionHash;
  @override
  int? get amountMsat;
  @override
  int get timestamp;
  @override
  int get expiry;
  @override
  List<RouteHint> get routingHints;
  @override
  Uint8List get paymentSecret;
  @override
  int get minFinalCltvExpiryDelta;
  @override
  @JsonKey(ignore: true)
  _$$LNInvoiceImplCopyWith<_$LNInvoiceImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnPaymentDetails {
  String get paymentHash => throw _privateConstructorUsedError;
  String get label => throw _privateConstructorUsedError;
  String get destinationPubkey => throw _privateConstructorUsedError;
  String get paymentPreimage => throw _privateConstructorUsedError;
  bool get keysend => throw _privateConstructorUsedError;
  String get bolt11 => throw _privateConstructorUsedError;
  String? get openChannelBolt11 => throw _privateConstructorUsedError;
  SuccessActionProcessed? get lnurlSuccessAction => throw _privateConstructorUsedError;
  String? get lnurlPayDomain => throw _privateConstructorUsedError;
  String? get lnAddress => throw _privateConstructorUsedError;
  String? get lnurlMetadata => throw _privateConstructorUsedError;
  String? get lnurlWithdrawEndpoint => throw _privateConstructorUsedError;
  SwapInfo? get swapInfo => throw _privateConstructorUsedError;
  ReverseSwapInfo? get reverseSwapInfo => throw _privateConstructorUsedError;
  int? get pendingExpirationBlock => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LnPaymentDetailsCopyWith<LnPaymentDetails> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnPaymentDetailsCopyWith<$Res> {
  factory $LnPaymentDetailsCopyWith(LnPaymentDetails value, $Res Function(LnPaymentDetails) then) =
      _$LnPaymentDetailsCopyWithImpl<$Res, LnPaymentDetails>;
  @useResult
  $Res call(
      {String paymentHash,
      String label,
      String destinationPubkey,
      String paymentPreimage,
      bool keysend,
      String bolt11,
      String? openChannelBolt11,
      SuccessActionProcessed? lnurlSuccessAction,
      String? lnurlPayDomain,
      String? lnAddress,
      String? lnurlMetadata,
      String? lnurlWithdrawEndpoint,
      SwapInfo? swapInfo,
      ReverseSwapInfo? reverseSwapInfo,
      int? pendingExpirationBlock});

  $SuccessActionProcessedCopyWith<$Res>? get lnurlSuccessAction;
  $SwapInfoCopyWith<$Res>? get swapInfo;
  $ReverseSwapInfoCopyWith<$Res>? get reverseSwapInfo;
}

/// @nodoc
class _$LnPaymentDetailsCopyWithImpl<$Res, $Val extends LnPaymentDetails>
    implements $LnPaymentDetailsCopyWith<$Res> {
  _$LnPaymentDetailsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? paymentHash = null,
    Object? label = null,
    Object? destinationPubkey = null,
    Object? paymentPreimage = null,
    Object? keysend = null,
    Object? bolt11 = null,
    Object? openChannelBolt11 = freezed,
    Object? lnurlSuccessAction = freezed,
    Object? lnurlPayDomain = freezed,
    Object? lnAddress = freezed,
    Object? lnurlMetadata = freezed,
    Object? lnurlWithdrawEndpoint = freezed,
    Object? swapInfo = freezed,
    Object? reverseSwapInfo = freezed,
    Object? pendingExpirationBlock = freezed,
  }) {
    return _then(_value.copyWith(
      paymentHash: null == paymentHash
          ? _value.paymentHash
          : paymentHash // ignore: cast_nullable_to_non_nullable
              as String,
      label: null == label
          ? _value.label
          : label // ignore: cast_nullable_to_non_nullable
              as String,
      destinationPubkey: null == destinationPubkey
          ? _value.destinationPubkey
          : destinationPubkey // ignore: cast_nullable_to_non_nullable
              as String,
      paymentPreimage: null == paymentPreimage
          ? _value.paymentPreimage
          : paymentPreimage // ignore: cast_nullable_to_non_nullable
              as String,
      keysend: null == keysend
          ? _value.keysend
          : keysend // ignore: cast_nullable_to_non_nullable
              as bool,
      bolt11: null == bolt11
          ? _value.bolt11
          : bolt11 // ignore: cast_nullable_to_non_nullable
              as String,
      openChannelBolt11: freezed == openChannelBolt11
          ? _value.openChannelBolt11
          : openChannelBolt11 // ignore: cast_nullable_to_non_nullable
              as String?,
      lnurlSuccessAction: freezed == lnurlSuccessAction
          ? _value.lnurlSuccessAction
          : lnurlSuccessAction // ignore: cast_nullable_to_non_nullable
              as SuccessActionProcessed?,
      lnurlPayDomain: freezed == lnurlPayDomain
          ? _value.lnurlPayDomain
          : lnurlPayDomain // ignore: cast_nullable_to_non_nullable
              as String?,
      lnAddress: freezed == lnAddress
          ? _value.lnAddress
          : lnAddress // ignore: cast_nullable_to_non_nullable
              as String?,
      lnurlMetadata: freezed == lnurlMetadata
          ? _value.lnurlMetadata
          : lnurlMetadata // ignore: cast_nullable_to_non_nullable
              as String?,
      lnurlWithdrawEndpoint: freezed == lnurlWithdrawEndpoint
          ? _value.lnurlWithdrawEndpoint
          : lnurlWithdrawEndpoint // ignore: cast_nullable_to_non_nullable
              as String?,
      swapInfo: freezed == swapInfo
          ? _value.swapInfo
          : swapInfo // ignore: cast_nullable_to_non_nullable
              as SwapInfo?,
      reverseSwapInfo: freezed == reverseSwapInfo
          ? _value.reverseSwapInfo
          : reverseSwapInfo // ignore: cast_nullable_to_non_nullable
              as ReverseSwapInfo?,
      pendingExpirationBlock: freezed == pendingExpirationBlock
          ? _value.pendingExpirationBlock
          : pendingExpirationBlock // ignore: cast_nullable_to_non_nullable
              as int?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $SuccessActionProcessedCopyWith<$Res>? get lnurlSuccessAction {
    if (_value.lnurlSuccessAction == null) {
      return null;
    }

    return $SuccessActionProcessedCopyWith<$Res>(_value.lnurlSuccessAction!, (value) {
      return _then(_value.copyWith(lnurlSuccessAction: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $SwapInfoCopyWith<$Res>? get swapInfo {
    if (_value.swapInfo == null) {
      return null;
    }

    return $SwapInfoCopyWith<$Res>(_value.swapInfo!, (value) {
      return _then(_value.copyWith(swapInfo: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ReverseSwapInfoCopyWith<$Res>? get reverseSwapInfo {
    if (_value.reverseSwapInfo == null) {
      return null;
    }

    return $ReverseSwapInfoCopyWith<$Res>(_value.reverseSwapInfo!, (value) {
      return _then(_value.copyWith(reverseSwapInfo: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$LnPaymentDetailsImplCopyWith<$Res> implements $LnPaymentDetailsCopyWith<$Res> {
  factory _$$LnPaymentDetailsImplCopyWith(
          _$LnPaymentDetailsImpl value, $Res Function(_$LnPaymentDetailsImpl) then) =
      __$$LnPaymentDetailsImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String paymentHash,
      String label,
      String destinationPubkey,
      String paymentPreimage,
      bool keysend,
      String bolt11,
      String? openChannelBolt11,
      SuccessActionProcessed? lnurlSuccessAction,
      String? lnurlPayDomain,
      String? lnAddress,
      String? lnurlMetadata,
      String? lnurlWithdrawEndpoint,
      SwapInfo? swapInfo,
      ReverseSwapInfo? reverseSwapInfo,
      int? pendingExpirationBlock});

  @override
  $SuccessActionProcessedCopyWith<$Res>? get lnurlSuccessAction;
  @override
  $SwapInfoCopyWith<$Res>? get swapInfo;
  @override
  $ReverseSwapInfoCopyWith<$Res>? get reverseSwapInfo;
}

/// @nodoc
class __$$LnPaymentDetailsImplCopyWithImpl<$Res>
    extends _$LnPaymentDetailsCopyWithImpl<$Res, _$LnPaymentDetailsImpl>
    implements _$$LnPaymentDetailsImplCopyWith<$Res> {
  __$$LnPaymentDetailsImplCopyWithImpl(
      _$LnPaymentDetailsImpl _value, $Res Function(_$LnPaymentDetailsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? paymentHash = null,
    Object? label = null,
    Object? destinationPubkey = null,
    Object? paymentPreimage = null,
    Object? keysend = null,
    Object? bolt11 = null,
    Object? openChannelBolt11 = freezed,
    Object? lnurlSuccessAction = freezed,
    Object? lnurlPayDomain = freezed,
    Object? lnAddress = freezed,
    Object? lnurlMetadata = freezed,
    Object? lnurlWithdrawEndpoint = freezed,
    Object? swapInfo = freezed,
    Object? reverseSwapInfo = freezed,
    Object? pendingExpirationBlock = freezed,
  }) {
    return _then(_$LnPaymentDetailsImpl(
      paymentHash: null == paymentHash
          ? _value.paymentHash
          : paymentHash // ignore: cast_nullable_to_non_nullable
              as String,
      label: null == label
          ? _value.label
          : label // ignore: cast_nullable_to_non_nullable
              as String,
      destinationPubkey: null == destinationPubkey
          ? _value.destinationPubkey
          : destinationPubkey // ignore: cast_nullable_to_non_nullable
              as String,
      paymentPreimage: null == paymentPreimage
          ? _value.paymentPreimage
          : paymentPreimage // ignore: cast_nullable_to_non_nullable
              as String,
      keysend: null == keysend
          ? _value.keysend
          : keysend // ignore: cast_nullable_to_non_nullable
              as bool,
      bolt11: null == bolt11
          ? _value.bolt11
          : bolt11 // ignore: cast_nullable_to_non_nullable
              as String,
      openChannelBolt11: freezed == openChannelBolt11
          ? _value.openChannelBolt11
          : openChannelBolt11 // ignore: cast_nullable_to_non_nullable
              as String?,
      lnurlSuccessAction: freezed == lnurlSuccessAction
          ? _value.lnurlSuccessAction
          : lnurlSuccessAction // ignore: cast_nullable_to_non_nullable
              as SuccessActionProcessed?,
      lnurlPayDomain: freezed == lnurlPayDomain
          ? _value.lnurlPayDomain
          : lnurlPayDomain // ignore: cast_nullable_to_non_nullable
              as String?,
      lnAddress: freezed == lnAddress
          ? _value.lnAddress
          : lnAddress // ignore: cast_nullable_to_non_nullable
              as String?,
      lnurlMetadata: freezed == lnurlMetadata
          ? _value.lnurlMetadata
          : lnurlMetadata // ignore: cast_nullable_to_non_nullable
              as String?,
      lnurlWithdrawEndpoint: freezed == lnurlWithdrawEndpoint
          ? _value.lnurlWithdrawEndpoint
          : lnurlWithdrawEndpoint // ignore: cast_nullable_to_non_nullable
              as String?,
      swapInfo: freezed == swapInfo
          ? _value.swapInfo
          : swapInfo // ignore: cast_nullable_to_non_nullable
              as SwapInfo?,
      reverseSwapInfo: freezed == reverseSwapInfo
          ? _value.reverseSwapInfo
          : reverseSwapInfo // ignore: cast_nullable_to_non_nullable
              as ReverseSwapInfo?,
      pendingExpirationBlock: freezed == pendingExpirationBlock
          ? _value.pendingExpirationBlock
          : pendingExpirationBlock // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$LnPaymentDetailsImpl implements _LnPaymentDetails {
  const _$LnPaymentDetailsImpl(
      {required this.paymentHash,
      required this.label,
      required this.destinationPubkey,
      required this.paymentPreimage,
      required this.keysend,
      required this.bolt11,
      this.openChannelBolt11,
      this.lnurlSuccessAction,
      this.lnurlPayDomain,
      this.lnAddress,
      this.lnurlMetadata,
      this.lnurlWithdrawEndpoint,
      this.swapInfo,
      this.reverseSwapInfo,
      this.pendingExpirationBlock});

  @override
  final String paymentHash;
  @override
  final String label;
  @override
  final String destinationPubkey;
  @override
  final String paymentPreimage;
  @override
  final bool keysend;
  @override
  final String bolt11;
  @override
  final String? openChannelBolt11;
  @override
  final SuccessActionProcessed? lnurlSuccessAction;
  @override
  final String? lnurlPayDomain;
  @override
  final String? lnAddress;
  @override
  final String? lnurlMetadata;
  @override
  final String? lnurlWithdrawEndpoint;
  @override
  final SwapInfo? swapInfo;
  @override
  final ReverseSwapInfo? reverseSwapInfo;
  @override
  final int? pendingExpirationBlock;

  @override
  String toString() {
    return 'LnPaymentDetails(paymentHash: $paymentHash, label: $label, destinationPubkey: $destinationPubkey, paymentPreimage: $paymentPreimage, keysend: $keysend, bolt11: $bolt11, openChannelBolt11: $openChannelBolt11, lnurlSuccessAction: $lnurlSuccessAction, lnurlPayDomain: $lnurlPayDomain, lnAddress: $lnAddress, lnurlMetadata: $lnurlMetadata, lnurlWithdrawEndpoint: $lnurlWithdrawEndpoint, swapInfo: $swapInfo, reverseSwapInfo: $reverseSwapInfo, pendingExpirationBlock: $pendingExpirationBlock)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnPaymentDetailsImpl &&
            (identical(other.paymentHash, paymentHash) || other.paymentHash == paymentHash) &&
            (identical(other.label, label) || other.label == label) &&
            (identical(other.destinationPubkey, destinationPubkey) ||
                other.destinationPubkey == destinationPubkey) &&
            (identical(other.paymentPreimage, paymentPreimage) || other.paymentPreimage == paymentPreimage) &&
            (identical(other.keysend, keysend) || other.keysend == keysend) &&
            (identical(other.bolt11, bolt11) || other.bolt11 == bolt11) &&
            (identical(other.openChannelBolt11, openChannelBolt11) ||
                other.openChannelBolt11 == openChannelBolt11) &&
            (identical(other.lnurlSuccessAction, lnurlSuccessAction) ||
                other.lnurlSuccessAction == lnurlSuccessAction) &&
            (identical(other.lnurlPayDomain, lnurlPayDomain) || other.lnurlPayDomain == lnurlPayDomain) &&
            (identical(other.lnAddress, lnAddress) || other.lnAddress == lnAddress) &&
            (identical(other.lnurlMetadata, lnurlMetadata) || other.lnurlMetadata == lnurlMetadata) &&
            (identical(other.lnurlWithdrawEndpoint, lnurlWithdrawEndpoint) ||
                other.lnurlWithdrawEndpoint == lnurlWithdrawEndpoint) &&
            (identical(other.swapInfo, swapInfo) || other.swapInfo == swapInfo) &&
            (identical(other.reverseSwapInfo, reverseSwapInfo) || other.reverseSwapInfo == reverseSwapInfo) &&
            (identical(other.pendingExpirationBlock, pendingExpirationBlock) ||
                other.pendingExpirationBlock == pendingExpirationBlock));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      paymentHash,
      label,
      destinationPubkey,
      paymentPreimage,
      keysend,
      bolt11,
      openChannelBolt11,
      lnurlSuccessAction,
      lnurlPayDomain,
      lnAddress,
      lnurlMetadata,
      lnurlWithdrawEndpoint,
      swapInfo,
      reverseSwapInfo,
      pendingExpirationBlock);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnPaymentDetailsImplCopyWith<_$LnPaymentDetailsImpl> get copyWith =>
      __$$LnPaymentDetailsImplCopyWithImpl<_$LnPaymentDetailsImpl>(this, _$identity);
}

abstract class _LnPaymentDetails implements LnPaymentDetails {
  const factory _LnPaymentDetails(
      {required final String paymentHash,
      required final String label,
      required final String destinationPubkey,
      required final String paymentPreimage,
      required final bool keysend,
      required final String bolt11,
      final String? openChannelBolt11,
      final SuccessActionProcessed? lnurlSuccessAction,
      final String? lnurlPayDomain,
      final String? lnAddress,
      final String? lnurlMetadata,
      final String? lnurlWithdrawEndpoint,
      final SwapInfo? swapInfo,
      final ReverseSwapInfo? reverseSwapInfo,
      final int? pendingExpirationBlock}) = _$LnPaymentDetailsImpl;

  @override
  String get paymentHash;
  @override
  String get label;
  @override
  String get destinationPubkey;
  @override
  String get paymentPreimage;
  @override
  bool get keysend;
  @override
  String get bolt11;
  @override
  String? get openChannelBolt11;
  @override
  SuccessActionProcessed? get lnurlSuccessAction;
  @override
  String? get lnurlPayDomain;
  @override
  String? get lnAddress;
  @override
  String? get lnurlMetadata;
  @override
  String? get lnurlWithdrawEndpoint;
  @override
  SwapInfo? get swapInfo;
  @override
  ReverseSwapInfo? get reverseSwapInfo;
  @override
  int? get pendingExpirationBlock;
  @override
  @JsonKey(ignore: true)
  _$$LnPaymentDetailsImplCopyWith<_$LnPaymentDetailsImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlAuthRequestData {
  String get k1 => throw _privateConstructorUsedError;
  String? get action => throw _privateConstructorUsedError;
  String get domain => throw _privateConstructorUsedError;
  String get url => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LnUrlAuthRequestDataCopyWith<LnUrlAuthRequestData> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlAuthRequestDataCopyWith<$Res> {
  factory $LnUrlAuthRequestDataCopyWith(
          LnUrlAuthRequestData value, $Res Function(LnUrlAuthRequestData) then) =
      _$LnUrlAuthRequestDataCopyWithImpl<$Res, LnUrlAuthRequestData>;
  @useResult
  $Res call({String k1, String? action, String domain, String url});
}

/// @nodoc
class _$LnUrlAuthRequestDataCopyWithImpl<$Res, $Val extends LnUrlAuthRequestData>
    implements $LnUrlAuthRequestDataCopyWith<$Res> {
  _$LnUrlAuthRequestDataCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? k1 = null,
    Object? action = freezed,
    Object? domain = null,
    Object? url = null,
  }) {
    return _then(_value.copyWith(
      k1: null == k1
          ? _value.k1
          : k1 // ignore: cast_nullable_to_non_nullable
              as String,
      action: freezed == action
          ? _value.action
          : action // ignore: cast_nullable_to_non_nullable
              as String?,
      domain: null == domain
          ? _value.domain
          : domain // ignore: cast_nullable_to_non_nullable
              as String,
      url: null == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$LnUrlAuthRequestDataImplCopyWith<$Res> implements $LnUrlAuthRequestDataCopyWith<$Res> {
  factory _$$LnUrlAuthRequestDataImplCopyWith(
          _$LnUrlAuthRequestDataImpl value, $Res Function(_$LnUrlAuthRequestDataImpl) then) =
      __$$LnUrlAuthRequestDataImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String k1, String? action, String domain, String url});
}

/// @nodoc
class __$$LnUrlAuthRequestDataImplCopyWithImpl<$Res>
    extends _$LnUrlAuthRequestDataCopyWithImpl<$Res, _$LnUrlAuthRequestDataImpl>
    implements _$$LnUrlAuthRequestDataImplCopyWith<$Res> {
  __$$LnUrlAuthRequestDataImplCopyWithImpl(
      _$LnUrlAuthRequestDataImpl _value, $Res Function(_$LnUrlAuthRequestDataImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? k1 = null,
    Object? action = freezed,
    Object? domain = null,
    Object? url = null,
  }) {
    return _then(_$LnUrlAuthRequestDataImpl(
      k1: null == k1
          ? _value.k1
          : k1 // ignore: cast_nullable_to_non_nullable
              as String,
      action: freezed == action
          ? _value.action
          : action // ignore: cast_nullable_to_non_nullable
              as String?,
      domain: null == domain
          ? _value.domain
          : domain // ignore: cast_nullable_to_non_nullable
              as String,
      url: null == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlAuthRequestDataImpl implements _LnUrlAuthRequestData {
  const _$LnUrlAuthRequestDataImpl({required this.k1, this.action, required this.domain, required this.url});

  @override
  final String k1;
  @override
  final String? action;
  @override
  final String domain;
  @override
  final String url;

  @override
  String toString() {
    return 'LnUrlAuthRequestData(k1: $k1, action: $action, domain: $domain, url: $url)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlAuthRequestDataImpl &&
            (identical(other.k1, k1) || other.k1 == k1) &&
            (identical(other.action, action) || other.action == action) &&
            (identical(other.domain, domain) || other.domain == domain) &&
            (identical(other.url, url) || other.url == url));
  }

  @override
  int get hashCode => Object.hash(runtimeType, k1, action, domain, url);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlAuthRequestDataImplCopyWith<_$LnUrlAuthRequestDataImpl> get copyWith =>
      __$$LnUrlAuthRequestDataImplCopyWithImpl<_$LnUrlAuthRequestDataImpl>(this, _$identity);
}

abstract class _LnUrlAuthRequestData implements LnUrlAuthRequestData {
  const factory _LnUrlAuthRequestData(
      {required final String k1,
      final String? action,
      required final String domain,
      required final String url}) = _$LnUrlAuthRequestDataImpl;

  @override
  String get k1;
  @override
  String? get action;
  @override
  String get domain;
  @override
  String get url;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlAuthRequestDataImplCopyWith<_$LnUrlAuthRequestDataImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlCallbackStatus {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlCallbackStatus_Ok value) ok,
    required TResult Function(LnUrlCallbackStatus_ErrorStatus value) errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult? Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlCallbackStatusCopyWith<$Res> {
  factory $LnUrlCallbackStatusCopyWith(LnUrlCallbackStatus value, $Res Function(LnUrlCallbackStatus) then) =
      _$LnUrlCallbackStatusCopyWithImpl<$Res, LnUrlCallbackStatus>;
}

/// @nodoc
class _$LnUrlCallbackStatusCopyWithImpl<$Res, $Val extends LnUrlCallbackStatus>
    implements $LnUrlCallbackStatusCopyWith<$Res> {
  _$LnUrlCallbackStatusCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$LnUrlCallbackStatus_OkImplCopyWith<$Res> {
  factory _$$LnUrlCallbackStatus_OkImplCopyWith(
          _$LnUrlCallbackStatus_OkImpl value, $Res Function(_$LnUrlCallbackStatus_OkImpl) then) =
      __$$LnUrlCallbackStatus_OkImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$LnUrlCallbackStatus_OkImplCopyWithImpl<$Res>
    extends _$LnUrlCallbackStatusCopyWithImpl<$Res, _$LnUrlCallbackStatus_OkImpl>
    implements _$$LnUrlCallbackStatus_OkImplCopyWith<$Res> {
  __$$LnUrlCallbackStatus_OkImplCopyWithImpl(
      _$LnUrlCallbackStatus_OkImpl _value, $Res Function(_$LnUrlCallbackStatus_OkImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$LnUrlCallbackStatus_OkImpl implements LnUrlCallbackStatus_Ok {
  const _$LnUrlCallbackStatus_OkImpl();

  @override
  String toString() {
    return 'LnUrlCallbackStatus.ok()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$LnUrlCallbackStatus_OkImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) {
    return ok();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) {
    return ok?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) {
    if (ok != null) {
      return ok();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlCallbackStatus_Ok value) ok,
    required TResult Function(LnUrlCallbackStatus_ErrorStatus value) errorStatus,
  }) {
    return ok(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult? Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
  }) {
    return ok?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) {
    if (ok != null) {
      return ok(this);
    }
    return orElse();
  }
}

abstract class LnUrlCallbackStatus_Ok implements LnUrlCallbackStatus {
  const factory LnUrlCallbackStatus_Ok() = _$LnUrlCallbackStatus_OkImpl;
}

/// @nodoc
abstract class _$$LnUrlCallbackStatus_ErrorStatusImplCopyWith<$Res> {
  factory _$$LnUrlCallbackStatus_ErrorStatusImplCopyWith(_$LnUrlCallbackStatus_ErrorStatusImpl value,
          $Res Function(_$LnUrlCallbackStatus_ErrorStatusImpl) then) =
      __$$LnUrlCallbackStatus_ErrorStatusImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});

  $LnUrlErrorDataCopyWith<$Res> get data;
}

/// @nodoc
class __$$LnUrlCallbackStatus_ErrorStatusImplCopyWithImpl<$Res>
    extends _$LnUrlCallbackStatusCopyWithImpl<$Res, _$LnUrlCallbackStatus_ErrorStatusImpl>
    implements _$$LnUrlCallbackStatus_ErrorStatusImplCopyWith<$Res> {
  __$$LnUrlCallbackStatus_ErrorStatusImplCopyWithImpl(_$LnUrlCallbackStatus_ErrorStatusImpl _value,
      $Res Function(_$LnUrlCallbackStatus_ErrorStatusImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlCallbackStatus_ErrorStatusImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $LnUrlErrorDataCopyWith<$Res> get data {
    return $LnUrlErrorDataCopyWith<$Res>(_value.data, (value) {
      return _then(_value.copyWith(data: value));
    });
  }
}

/// @nodoc

class _$LnUrlCallbackStatus_ErrorStatusImpl implements LnUrlCallbackStatus_ErrorStatus {
  const _$LnUrlCallbackStatus_ErrorStatusImpl({required this.data});

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'LnUrlCallbackStatus.errorStatus(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlCallbackStatus_ErrorStatusImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlCallbackStatus_ErrorStatusImplCopyWith<_$LnUrlCallbackStatus_ErrorStatusImpl> get copyWith =>
      __$$LnUrlCallbackStatus_ErrorStatusImplCopyWithImpl<_$LnUrlCallbackStatus_ErrorStatusImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) {
    return errorStatus(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) {
    return errorStatus?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) {
    if (errorStatus != null) {
      return errorStatus(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlCallbackStatus_Ok value) ok,
    required TResult Function(LnUrlCallbackStatus_ErrorStatus value) errorStatus,
  }) {
    return errorStatus(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult? Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
  }) {
    return errorStatus?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlCallbackStatus_Ok value)? ok,
    TResult Function(LnUrlCallbackStatus_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) {
    if (errorStatus != null) {
      return errorStatus(this);
    }
    return orElse();
  }
}

abstract class LnUrlCallbackStatus_ErrorStatus implements LnUrlCallbackStatus {
  const factory LnUrlCallbackStatus_ErrorStatus({required final LnUrlErrorData data}) =
      _$LnUrlCallbackStatus_ErrorStatusImpl;

  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlCallbackStatus_ErrorStatusImplCopyWith<_$LnUrlCallbackStatus_ErrorStatusImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlErrorData {
  String get reason => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LnUrlErrorDataCopyWith<LnUrlErrorData> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlErrorDataCopyWith<$Res> {
  factory $LnUrlErrorDataCopyWith(LnUrlErrorData value, $Res Function(LnUrlErrorData) then) =
      _$LnUrlErrorDataCopyWithImpl<$Res, LnUrlErrorData>;
  @useResult
  $Res call({String reason});
}

/// @nodoc
class _$LnUrlErrorDataCopyWithImpl<$Res, $Val extends LnUrlErrorData>
    implements $LnUrlErrorDataCopyWith<$Res> {
  _$LnUrlErrorDataCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? reason = null,
  }) {
    return _then(_value.copyWith(
      reason: null == reason
          ? _value.reason
          : reason // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$LnUrlErrorDataImplCopyWith<$Res> implements $LnUrlErrorDataCopyWith<$Res> {
  factory _$$LnUrlErrorDataImplCopyWith(
          _$LnUrlErrorDataImpl value, $Res Function(_$LnUrlErrorDataImpl) then) =
      __$$LnUrlErrorDataImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String reason});
}

/// @nodoc
class __$$LnUrlErrorDataImplCopyWithImpl<$Res>
    extends _$LnUrlErrorDataCopyWithImpl<$Res, _$LnUrlErrorDataImpl>
    implements _$$LnUrlErrorDataImplCopyWith<$Res> {
  __$$LnUrlErrorDataImplCopyWithImpl(_$LnUrlErrorDataImpl _value, $Res Function(_$LnUrlErrorDataImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? reason = null,
  }) {
    return _then(_$LnUrlErrorDataImpl(
      reason: null == reason
          ? _value.reason
          : reason // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LnUrlErrorDataImpl implements _LnUrlErrorData {
  const _$LnUrlErrorDataImpl({required this.reason});

  @override
  final String reason;

  @override
  String toString() {
    return 'LnUrlErrorData(reason: $reason)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlErrorDataImpl &&
            (identical(other.reason, reason) || other.reason == reason));
  }

  @override
  int get hashCode => Object.hash(runtimeType, reason);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlErrorDataImplCopyWith<_$LnUrlErrorDataImpl> get copyWith =>
      __$$LnUrlErrorDataImplCopyWithImpl<_$LnUrlErrorDataImpl>(this, _$identity);
}

abstract class _LnUrlErrorData implements LnUrlErrorData {
  const factory _LnUrlErrorData({required final String reason}) = _$LnUrlErrorDataImpl;

  @override
  String get reason;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlErrorDataImplCopyWith<_$LnUrlErrorDataImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlPayRequest {
  LnUrlPayRequestData get data => throw _privateConstructorUsedError;
  int get amountMsat => throw _privateConstructorUsedError;
  String? get comment => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LnUrlPayRequestCopyWith<LnUrlPayRequest> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlPayRequestCopyWith<$Res> {
  factory $LnUrlPayRequestCopyWith(LnUrlPayRequest value, $Res Function(LnUrlPayRequest) then) =
      _$LnUrlPayRequestCopyWithImpl<$Res, LnUrlPayRequest>;
  @useResult
  $Res call({LnUrlPayRequestData data, int amountMsat, String? comment});

  $LnUrlPayRequestDataCopyWith<$Res> get data;
}

/// @nodoc
class _$LnUrlPayRequestCopyWithImpl<$Res, $Val extends LnUrlPayRequest>
    implements $LnUrlPayRequestCopyWith<$Res> {
  _$LnUrlPayRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
    Object? amountMsat = null,
    Object? comment = freezed,
  }) {
    return _then(_value.copyWith(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlPayRequestData,
      amountMsat: null == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      comment: freezed == comment
          ? _value.comment
          : comment // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $LnUrlPayRequestDataCopyWith<$Res> get data {
    return $LnUrlPayRequestDataCopyWith<$Res>(_value.data, (value) {
      return _then(_value.copyWith(data: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$LnUrlPayRequestImplCopyWith<$Res> implements $LnUrlPayRequestCopyWith<$Res> {
  factory _$$LnUrlPayRequestImplCopyWith(
          _$LnUrlPayRequestImpl value, $Res Function(_$LnUrlPayRequestImpl) then) =
      __$$LnUrlPayRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({LnUrlPayRequestData data, int amountMsat, String? comment});

  @override
  $LnUrlPayRequestDataCopyWith<$Res> get data;
}

/// @nodoc
class __$$LnUrlPayRequestImplCopyWithImpl<$Res>
    extends _$LnUrlPayRequestCopyWithImpl<$Res, _$LnUrlPayRequestImpl>
    implements _$$LnUrlPayRequestImplCopyWith<$Res> {
  __$$LnUrlPayRequestImplCopyWithImpl(
      _$LnUrlPayRequestImpl _value, $Res Function(_$LnUrlPayRequestImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
    Object? amountMsat = null,
    Object? comment = freezed,
  }) {
    return _then(_$LnUrlPayRequestImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlPayRequestData,
      amountMsat: null == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      comment: freezed == comment
          ? _value.comment
          : comment // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class _$LnUrlPayRequestImpl implements _LnUrlPayRequest {
  const _$LnUrlPayRequestImpl({required this.data, required this.amountMsat, this.comment});

  @override
  final LnUrlPayRequestData data;
  @override
  final int amountMsat;
  @override
  final String? comment;

  @override
  String toString() {
    return 'LnUrlPayRequest(data: $data, amountMsat: $amountMsat, comment: $comment)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayRequestImpl &&
            (identical(other.data, data) || other.data == data) &&
            (identical(other.amountMsat, amountMsat) || other.amountMsat == amountMsat) &&
            (identical(other.comment, comment) || other.comment == comment));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data, amountMsat, comment);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayRequestImplCopyWith<_$LnUrlPayRequestImpl> get copyWith =>
      __$$LnUrlPayRequestImplCopyWithImpl<_$LnUrlPayRequestImpl>(this, _$identity);
}

abstract class _LnUrlPayRequest implements LnUrlPayRequest {
  const factory _LnUrlPayRequest(
      {required final LnUrlPayRequestData data,
      required final int amountMsat,
      final String? comment}) = _$LnUrlPayRequestImpl;

  @override
  LnUrlPayRequestData get data;
  @override
  int get amountMsat;
  @override
  String? get comment;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlPayRequestImplCopyWith<_$LnUrlPayRequestImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlPayRequestData {
  String get callback => throw _privateConstructorUsedError;
  int get minSendable => throw _privateConstructorUsedError;
  int get maxSendable => throw _privateConstructorUsedError;
  String get metadataStr => throw _privateConstructorUsedError;
  int get commentAllowed => throw _privateConstructorUsedError;
  String get domain => throw _privateConstructorUsedError;
  String? get lnAddress => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LnUrlPayRequestDataCopyWith<LnUrlPayRequestData> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlPayRequestDataCopyWith<$Res> {
  factory $LnUrlPayRequestDataCopyWith(LnUrlPayRequestData value, $Res Function(LnUrlPayRequestData) then) =
      _$LnUrlPayRequestDataCopyWithImpl<$Res, LnUrlPayRequestData>;
  @useResult
  $Res call(
      {String callback,
      int minSendable,
      int maxSendable,
      String metadataStr,
      int commentAllowed,
      String domain,
      String? lnAddress});
}

/// @nodoc
class _$LnUrlPayRequestDataCopyWithImpl<$Res, $Val extends LnUrlPayRequestData>
    implements $LnUrlPayRequestDataCopyWith<$Res> {
  _$LnUrlPayRequestDataCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? callback = null,
    Object? minSendable = null,
    Object? maxSendable = null,
    Object? metadataStr = null,
    Object? commentAllowed = null,
    Object? domain = null,
    Object? lnAddress = freezed,
  }) {
    return _then(_value.copyWith(
      callback: null == callback
          ? _value.callback
          : callback // ignore: cast_nullable_to_non_nullable
              as String,
      minSendable: null == minSendable
          ? _value.minSendable
          : minSendable // ignore: cast_nullable_to_non_nullable
              as int,
      maxSendable: null == maxSendable
          ? _value.maxSendable
          : maxSendable // ignore: cast_nullable_to_non_nullable
              as int,
      metadataStr: null == metadataStr
          ? _value.metadataStr
          : metadataStr // ignore: cast_nullable_to_non_nullable
              as String,
      commentAllowed: null == commentAllowed
          ? _value.commentAllowed
          : commentAllowed // ignore: cast_nullable_to_non_nullable
              as int,
      domain: null == domain
          ? _value.domain
          : domain // ignore: cast_nullable_to_non_nullable
              as String,
      lnAddress: freezed == lnAddress
          ? _value.lnAddress
          : lnAddress // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$LnUrlPayRequestDataImplCopyWith<$Res> implements $LnUrlPayRequestDataCopyWith<$Res> {
  factory _$$LnUrlPayRequestDataImplCopyWith(
          _$LnUrlPayRequestDataImpl value, $Res Function(_$LnUrlPayRequestDataImpl) then) =
      __$$LnUrlPayRequestDataImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String callback,
      int minSendable,
      int maxSendable,
      String metadataStr,
      int commentAllowed,
      String domain,
      String? lnAddress});
}

/// @nodoc
class __$$LnUrlPayRequestDataImplCopyWithImpl<$Res>
    extends _$LnUrlPayRequestDataCopyWithImpl<$Res, _$LnUrlPayRequestDataImpl>
    implements _$$LnUrlPayRequestDataImplCopyWith<$Res> {
  __$$LnUrlPayRequestDataImplCopyWithImpl(
      _$LnUrlPayRequestDataImpl _value, $Res Function(_$LnUrlPayRequestDataImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? callback = null,
    Object? minSendable = null,
    Object? maxSendable = null,
    Object? metadataStr = null,
    Object? commentAllowed = null,
    Object? domain = null,
    Object? lnAddress = freezed,
  }) {
    return _then(_$LnUrlPayRequestDataImpl(
      callback: null == callback
          ? _value.callback
          : callback // ignore: cast_nullable_to_non_nullable
              as String,
      minSendable: null == minSendable
          ? _value.minSendable
          : minSendable // ignore: cast_nullable_to_non_nullable
              as int,
      maxSendable: null == maxSendable
          ? _value.maxSendable
          : maxSendable // ignore: cast_nullable_to_non_nullable
              as int,
      metadataStr: null == metadataStr
          ? _value.metadataStr
          : metadataStr // ignore: cast_nullable_to_non_nullable
              as String,
      commentAllowed: null == commentAllowed
          ? _value.commentAllowed
          : commentAllowed // ignore: cast_nullable_to_non_nullable
              as int,
      domain: null == domain
          ? _value.domain
          : domain // ignore: cast_nullable_to_non_nullable
              as String,
      lnAddress: freezed == lnAddress
          ? _value.lnAddress
          : lnAddress // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class _$LnUrlPayRequestDataImpl implements _LnUrlPayRequestData {
  const _$LnUrlPayRequestDataImpl(
      {required this.callback,
      required this.minSendable,
      required this.maxSendable,
      required this.metadataStr,
      required this.commentAllowed,
      required this.domain,
      this.lnAddress});

  @override
  final String callback;
  @override
  final int minSendable;
  @override
  final int maxSendable;
  @override
  final String metadataStr;
  @override
  final int commentAllowed;
  @override
  final String domain;
  @override
  final String? lnAddress;

  @override
  String toString() {
    return 'LnUrlPayRequestData(callback: $callback, minSendable: $minSendable, maxSendable: $maxSendable, metadataStr: $metadataStr, commentAllowed: $commentAllowed, domain: $domain, lnAddress: $lnAddress)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayRequestDataImpl &&
            (identical(other.callback, callback) || other.callback == callback) &&
            (identical(other.minSendable, minSendable) || other.minSendable == minSendable) &&
            (identical(other.maxSendable, maxSendable) || other.maxSendable == maxSendable) &&
            (identical(other.metadataStr, metadataStr) || other.metadataStr == metadataStr) &&
            (identical(other.commentAllowed, commentAllowed) || other.commentAllowed == commentAllowed) &&
            (identical(other.domain, domain) || other.domain == domain) &&
            (identical(other.lnAddress, lnAddress) || other.lnAddress == lnAddress));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, callback, minSendable, maxSendable, metadataStr, commentAllowed, domain, lnAddress);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayRequestDataImplCopyWith<_$LnUrlPayRequestDataImpl> get copyWith =>
      __$$LnUrlPayRequestDataImplCopyWithImpl<_$LnUrlPayRequestDataImpl>(this, _$identity);
}

abstract class _LnUrlPayRequestData implements LnUrlPayRequestData {
  const factory _LnUrlPayRequestData(
      {required final String callback,
      required final int minSendable,
      required final int maxSendable,
      required final String metadataStr,
      required final int commentAllowed,
      required final String domain,
      final String? lnAddress}) = _$LnUrlPayRequestDataImpl;

  @override
  String get callback;
  @override
  int get minSendable;
  @override
  int get maxSendable;
  @override
  String get metadataStr;
  @override
  int get commentAllowed;
  @override
  String get domain;
  @override
  String? get lnAddress;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlPayRequestDataImplCopyWith<_$LnUrlPayRequestDataImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlPayResult {
  Object get data => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnUrlPaySuccessData data) endpointSuccess,
    required TResult Function(LnUrlErrorData data) endpointError,
    required TResult Function(LnUrlPayErrorData data) payError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPaySuccessData data)? endpointSuccess,
    TResult? Function(LnUrlErrorData data)? endpointError,
    TResult? Function(LnUrlPayErrorData data)? payError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnUrlPaySuccessData data)? endpointSuccess,
    TResult Function(LnUrlErrorData data)? endpointError,
    TResult Function(LnUrlPayErrorData data)? payError,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayResult_EndpointSuccess value) endpointSuccess,
    required TResult Function(LnUrlPayResult_EndpointError value) endpointError,
    required TResult Function(LnUrlPayResult_PayError value) payError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult? Function(LnUrlPayResult_EndpointError value)? endpointError,
    TResult? Function(LnUrlPayResult_PayError value)? payError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult Function(LnUrlPayResult_EndpointError value)? endpointError,
    TResult Function(LnUrlPayResult_PayError value)? payError,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlPayResultCopyWith<$Res> {
  factory $LnUrlPayResultCopyWith(LnUrlPayResult value, $Res Function(LnUrlPayResult) then) =
      _$LnUrlPayResultCopyWithImpl<$Res, LnUrlPayResult>;
}

/// @nodoc
class _$LnUrlPayResultCopyWithImpl<$Res, $Val extends LnUrlPayResult>
    implements $LnUrlPayResultCopyWith<$Res> {
  _$LnUrlPayResultCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$LnUrlPayResult_EndpointSuccessImplCopyWith<$Res> {
  factory _$$LnUrlPayResult_EndpointSuccessImplCopyWith(_$LnUrlPayResult_EndpointSuccessImpl value,
          $Res Function(_$LnUrlPayResult_EndpointSuccessImpl) then) =
      __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlPaySuccessData data});
}

/// @nodoc
class __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointSuccessImpl>
    implements _$$LnUrlPayResult_EndpointSuccessImplCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl(
      _$LnUrlPayResult_EndpointSuccessImpl _value, $Res Function(_$LnUrlPayResult_EndpointSuccessImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlPayResult_EndpointSuccessImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlPaySuccessData,
    ));
  }
}

/// @nodoc

class _$LnUrlPayResult_EndpointSuccessImpl implements LnUrlPayResult_EndpointSuccess {
  const _$LnUrlPayResult_EndpointSuccessImpl({required this.data});

  @override
  final LnUrlPaySuccessData data;

  @override
  String toString() {
    return 'LnUrlPayResult.endpointSuccess(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayResult_EndpointSuccessImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayResult_EndpointSuccessImplCopyWith<_$LnUrlPayResult_EndpointSuccessImpl> get copyWith =>
      __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl<_$LnUrlPayResult_EndpointSuccessImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnUrlPaySuccessData data) endpointSuccess,
    required TResult Function(LnUrlErrorData data) endpointError,
    required TResult Function(LnUrlPayErrorData data) payError,
  }) {
    return endpointSuccess(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPaySuccessData data)? endpointSuccess,
    TResult? Function(LnUrlErrorData data)? endpointError,
    TResult? Function(LnUrlPayErrorData data)? payError,
  }) {
    return endpointSuccess?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnUrlPaySuccessData data)? endpointSuccess,
    TResult Function(LnUrlErrorData data)? endpointError,
    TResult Function(LnUrlPayErrorData data)? payError,
    required TResult orElse(),
  }) {
    if (endpointSuccess != null) {
      return endpointSuccess(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayResult_EndpointSuccess value) endpointSuccess,
    required TResult Function(LnUrlPayResult_EndpointError value) endpointError,
    required TResult Function(LnUrlPayResult_PayError value) payError,
  }) {
    return endpointSuccess(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult? Function(LnUrlPayResult_EndpointError value)? endpointError,
    TResult? Function(LnUrlPayResult_PayError value)? payError,
  }) {
    return endpointSuccess?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult Function(LnUrlPayResult_EndpointError value)? endpointError,
    TResult Function(LnUrlPayResult_PayError value)? payError,
    required TResult orElse(),
  }) {
    if (endpointSuccess != null) {
      return endpointSuccess(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayResult_EndpointSuccess implements LnUrlPayResult {
  const factory LnUrlPayResult_EndpointSuccess({required final LnUrlPaySuccessData data}) =
      _$LnUrlPayResult_EndpointSuccessImpl;

  @override
  LnUrlPaySuccessData get data;
  @JsonKey(ignore: true)
  _$$LnUrlPayResult_EndpointSuccessImplCopyWith<_$LnUrlPayResult_EndpointSuccessImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayResult_EndpointErrorImplCopyWith<$Res> {
  factory _$$LnUrlPayResult_EndpointErrorImplCopyWith(
          _$LnUrlPayResult_EndpointErrorImpl value, $Res Function(_$LnUrlPayResult_EndpointErrorImpl) then) =
      __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});

  $LnUrlErrorDataCopyWith<$Res> get data;
}

/// @nodoc
class __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointErrorImpl>
    implements _$$LnUrlPayResult_EndpointErrorImplCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl(
      _$LnUrlPayResult_EndpointErrorImpl _value, $Res Function(_$LnUrlPayResult_EndpointErrorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlPayResult_EndpointErrorImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $LnUrlErrorDataCopyWith<$Res> get data {
    return $LnUrlErrorDataCopyWith<$Res>(_value.data, (value) {
      return _then(_value.copyWith(data: value));
    });
  }
}

/// @nodoc

class _$LnUrlPayResult_EndpointErrorImpl implements LnUrlPayResult_EndpointError {
  const _$LnUrlPayResult_EndpointErrorImpl({required this.data});

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'LnUrlPayResult.endpointError(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayResult_EndpointErrorImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayResult_EndpointErrorImplCopyWith<_$LnUrlPayResult_EndpointErrorImpl> get copyWith =>
      __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl<_$LnUrlPayResult_EndpointErrorImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnUrlPaySuccessData data) endpointSuccess,
    required TResult Function(LnUrlErrorData data) endpointError,
    required TResult Function(LnUrlPayErrorData data) payError,
  }) {
    return endpointError(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPaySuccessData data)? endpointSuccess,
    TResult? Function(LnUrlErrorData data)? endpointError,
    TResult? Function(LnUrlPayErrorData data)? payError,
  }) {
    return endpointError?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnUrlPaySuccessData data)? endpointSuccess,
    TResult Function(LnUrlErrorData data)? endpointError,
    TResult Function(LnUrlPayErrorData data)? payError,
    required TResult orElse(),
  }) {
    if (endpointError != null) {
      return endpointError(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayResult_EndpointSuccess value) endpointSuccess,
    required TResult Function(LnUrlPayResult_EndpointError value) endpointError,
    required TResult Function(LnUrlPayResult_PayError value) payError,
  }) {
    return endpointError(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult? Function(LnUrlPayResult_EndpointError value)? endpointError,
    TResult? Function(LnUrlPayResult_PayError value)? payError,
  }) {
    return endpointError?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult Function(LnUrlPayResult_EndpointError value)? endpointError,
    TResult Function(LnUrlPayResult_PayError value)? payError,
    required TResult orElse(),
  }) {
    if (endpointError != null) {
      return endpointError(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayResult_EndpointError implements LnUrlPayResult {
  const factory LnUrlPayResult_EndpointError({required final LnUrlErrorData data}) =
      _$LnUrlPayResult_EndpointErrorImpl;

  @override
  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlPayResult_EndpointErrorImplCopyWith<_$LnUrlPayResult_EndpointErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayResult_PayErrorImplCopyWith<$Res> {
  factory _$$LnUrlPayResult_PayErrorImplCopyWith(
          _$LnUrlPayResult_PayErrorImpl value, $Res Function(_$LnUrlPayResult_PayErrorImpl) then) =
      __$$LnUrlPayResult_PayErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlPayErrorData data});
}

/// @nodoc
class __$$LnUrlPayResult_PayErrorImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_PayErrorImpl>
    implements _$$LnUrlPayResult_PayErrorImplCopyWith<$Res> {
  __$$LnUrlPayResult_PayErrorImplCopyWithImpl(
      _$LnUrlPayResult_PayErrorImpl _value, $Res Function(_$LnUrlPayResult_PayErrorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlPayResult_PayErrorImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlPayErrorData,
    ));
  }
}

/// @nodoc

class _$LnUrlPayResult_PayErrorImpl implements LnUrlPayResult_PayError {
  const _$LnUrlPayResult_PayErrorImpl({required this.data});

  @override
  final LnUrlPayErrorData data;

  @override
  String toString() {
    return 'LnUrlPayResult.payError(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayResult_PayErrorImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayResult_PayErrorImplCopyWith<_$LnUrlPayResult_PayErrorImpl> get copyWith =>
      __$$LnUrlPayResult_PayErrorImplCopyWithImpl<_$LnUrlPayResult_PayErrorImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnUrlPaySuccessData data) endpointSuccess,
    required TResult Function(LnUrlErrorData data) endpointError,
    required TResult Function(LnUrlPayErrorData data) payError,
  }) {
    return payError(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPaySuccessData data)? endpointSuccess,
    TResult? Function(LnUrlErrorData data)? endpointError,
    TResult? Function(LnUrlPayErrorData data)? payError,
  }) {
    return payError?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnUrlPaySuccessData data)? endpointSuccess,
    TResult Function(LnUrlErrorData data)? endpointError,
    TResult Function(LnUrlPayErrorData data)? payError,
    required TResult orElse(),
  }) {
    if (payError != null) {
      return payError(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayResult_EndpointSuccess value) endpointSuccess,
    required TResult Function(LnUrlPayResult_EndpointError value) endpointError,
    required TResult Function(LnUrlPayResult_PayError value) payError,
  }) {
    return payError(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult? Function(LnUrlPayResult_EndpointError value)? endpointError,
    TResult? Function(LnUrlPayResult_PayError value)? payError,
  }) {
    return payError?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult Function(LnUrlPayResult_EndpointError value)? endpointError,
    TResult Function(LnUrlPayResult_PayError value)? payError,
    required TResult orElse(),
  }) {
    if (payError != null) {
      return payError(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayResult_PayError implements LnUrlPayResult {
  const factory LnUrlPayResult_PayError({required final LnUrlPayErrorData data}) =
      _$LnUrlPayResult_PayErrorImpl;

  @override
  LnUrlPayErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlPayResult_PayErrorImplCopyWith<_$LnUrlPayResult_PayErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlWithdrawRequest {
  LnUrlWithdrawRequestData get data => throw _privateConstructorUsedError;
  int get amountMsat => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LnUrlWithdrawRequestCopyWith<LnUrlWithdrawRequest> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlWithdrawRequestCopyWith<$Res> {
  factory $LnUrlWithdrawRequestCopyWith(
          LnUrlWithdrawRequest value, $Res Function(LnUrlWithdrawRequest) then) =
      _$LnUrlWithdrawRequestCopyWithImpl<$Res, LnUrlWithdrawRequest>;
  @useResult
  $Res call({LnUrlWithdrawRequestData data, int amountMsat, String? description});

  $LnUrlWithdrawRequestDataCopyWith<$Res> get data;
}

/// @nodoc
class _$LnUrlWithdrawRequestCopyWithImpl<$Res, $Val extends LnUrlWithdrawRequest>
    implements $LnUrlWithdrawRequestCopyWith<$Res> {
  _$LnUrlWithdrawRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
    Object? amountMsat = null,
    Object? description = freezed,
  }) {
    return _then(_value.copyWith(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlWithdrawRequestData,
      amountMsat: null == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      description: freezed == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $LnUrlWithdrawRequestDataCopyWith<$Res> get data {
    return $LnUrlWithdrawRequestDataCopyWith<$Res>(_value.data, (value) {
      return _then(_value.copyWith(data: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$LnUrlWithdrawRequestImplCopyWith<$Res> implements $LnUrlWithdrawRequestCopyWith<$Res> {
  factory _$$LnUrlWithdrawRequestImplCopyWith(
          _$LnUrlWithdrawRequestImpl value, $Res Function(_$LnUrlWithdrawRequestImpl) then) =
      __$$LnUrlWithdrawRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({LnUrlWithdrawRequestData data, int amountMsat, String? description});

  @override
  $LnUrlWithdrawRequestDataCopyWith<$Res> get data;
}

/// @nodoc
class __$$LnUrlWithdrawRequestImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawRequestCopyWithImpl<$Res, _$LnUrlWithdrawRequestImpl>
    implements _$$LnUrlWithdrawRequestImplCopyWith<$Res> {
  __$$LnUrlWithdrawRequestImplCopyWithImpl(
      _$LnUrlWithdrawRequestImpl _value, $Res Function(_$LnUrlWithdrawRequestImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
    Object? amountMsat = null,
    Object? description = freezed,
  }) {
    return _then(_$LnUrlWithdrawRequestImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlWithdrawRequestData,
      amountMsat: null == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      description: freezed == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawRequestImpl implements _LnUrlWithdrawRequest {
  const _$LnUrlWithdrawRequestImpl({required this.data, required this.amountMsat, this.description});

  @override
  final LnUrlWithdrawRequestData data;
  @override
  final int amountMsat;
  @override
  final String? description;

  @override
  String toString() {
    return 'LnUrlWithdrawRequest(data: $data, amountMsat: $amountMsat, description: $description)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawRequestImpl &&
            (identical(other.data, data) || other.data == data) &&
            (identical(other.amountMsat, amountMsat) || other.amountMsat == amountMsat) &&
            (identical(other.description, description) || other.description == description));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data, amountMsat, description);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawRequestImplCopyWith<_$LnUrlWithdrawRequestImpl> get copyWith =>
      __$$LnUrlWithdrawRequestImplCopyWithImpl<_$LnUrlWithdrawRequestImpl>(this, _$identity);
}

abstract class _LnUrlWithdrawRequest implements LnUrlWithdrawRequest {
  const factory _LnUrlWithdrawRequest(
      {required final LnUrlWithdrawRequestData data,
      required final int amountMsat,
      final String? description}) = _$LnUrlWithdrawRequestImpl;

  @override
  LnUrlWithdrawRequestData get data;
  @override
  int get amountMsat;
  @override
  String? get description;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawRequestImplCopyWith<_$LnUrlWithdrawRequestImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlWithdrawRequestData {
  String get callback => throw _privateConstructorUsedError;
  String get k1 => throw _privateConstructorUsedError;
  String get defaultDescription => throw _privateConstructorUsedError;
  int get minWithdrawable => throw _privateConstructorUsedError;
  int get maxWithdrawable => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LnUrlWithdrawRequestDataCopyWith<LnUrlWithdrawRequestData> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlWithdrawRequestDataCopyWith<$Res> {
  factory $LnUrlWithdrawRequestDataCopyWith(
          LnUrlWithdrawRequestData value, $Res Function(LnUrlWithdrawRequestData) then) =
      _$LnUrlWithdrawRequestDataCopyWithImpl<$Res, LnUrlWithdrawRequestData>;
  @useResult
  $Res call(
      {String callback, String k1, String defaultDescription, int minWithdrawable, int maxWithdrawable});
}

/// @nodoc
class _$LnUrlWithdrawRequestDataCopyWithImpl<$Res, $Val extends LnUrlWithdrawRequestData>
    implements $LnUrlWithdrawRequestDataCopyWith<$Res> {
  _$LnUrlWithdrawRequestDataCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? callback = null,
    Object? k1 = null,
    Object? defaultDescription = null,
    Object? minWithdrawable = null,
    Object? maxWithdrawable = null,
  }) {
    return _then(_value.copyWith(
      callback: null == callback
          ? _value.callback
          : callback // ignore: cast_nullable_to_non_nullable
              as String,
      k1: null == k1
          ? _value.k1
          : k1 // ignore: cast_nullable_to_non_nullable
              as String,
      defaultDescription: null == defaultDescription
          ? _value.defaultDescription
          : defaultDescription // ignore: cast_nullable_to_non_nullable
              as String,
      minWithdrawable: null == minWithdrawable
          ? _value.minWithdrawable
          : minWithdrawable // ignore: cast_nullable_to_non_nullable
              as int,
      maxWithdrawable: null == maxWithdrawable
          ? _value.maxWithdrawable
          : maxWithdrawable // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$LnUrlWithdrawRequestDataImplCopyWith<$Res>
    implements $LnUrlWithdrawRequestDataCopyWith<$Res> {
  factory _$$LnUrlWithdrawRequestDataImplCopyWith(
          _$LnUrlWithdrawRequestDataImpl value, $Res Function(_$LnUrlWithdrawRequestDataImpl) then) =
      __$$LnUrlWithdrawRequestDataImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String callback, String k1, String defaultDescription, int minWithdrawable, int maxWithdrawable});
}

/// @nodoc
class __$$LnUrlWithdrawRequestDataImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawRequestDataCopyWithImpl<$Res, _$LnUrlWithdrawRequestDataImpl>
    implements _$$LnUrlWithdrawRequestDataImplCopyWith<$Res> {
  __$$LnUrlWithdrawRequestDataImplCopyWithImpl(
      _$LnUrlWithdrawRequestDataImpl _value, $Res Function(_$LnUrlWithdrawRequestDataImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? callback = null,
    Object? k1 = null,
    Object? defaultDescription = null,
    Object? minWithdrawable = null,
    Object? maxWithdrawable = null,
  }) {
    return _then(_$LnUrlWithdrawRequestDataImpl(
      callback: null == callback
          ? _value.callback
          : callback // ignore: cast_nullable_to_non_nullable
              as String,
      k1: null == k1
          ? _value.k1
          : k1 // ignore: cast_nullable_to_non_nullable
              as String,
      defaultDescription: null == defaultDescription
          ? _value.defaultDescription
          : defaultDescription // ignore: cast_nullable_to_non_nullable
              as String,
      minWithdrawable: null == minWithdrawable
          ? _value.minWithdrawable
          : minWithdrawable // ignore: cast_nullable_to_non_nullable
              as int,
      maxWithdrawable: null == maxWithdrawable
          ? _value.maxWithdrawable
          : maxWithdrawable // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawRequestDataImpl implements _LnUrlWithdrawRequestData {
  const _$LnUrlWithdrawRequestDataImpl(
      {required this.callback,
      required this.k1,
      required this.defaultDescription,
      required this.minWithdrawable,
      required this.maxWithdrawable});

  @override
  final String callback;
  @override
  final String k1;
  @override
  final String defaultDescription;
  @override
  final int minWithdrawable;
  @override
  final int maxWithdrawable;

  @override
  String toString() {
    return 'LnUrlWithdrawRequestData(callback: $callback, k1: $k1, defaultDescription: $defaultDescription, minWithdrawable: $minWithdrawable, maxWithdrawable: $maxWithdrawable)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawRequestDataImpl &&
            (identical(other.callback, callback) || other.callback == callback) &&
            (identical(other.k1, k1) || other.k1 == k1) &&
            (identical(other.defaultDescription, defaultDescription) ||
                other.defaultDescription == defaultDescription) &&
            (identical(other.minWithdrawable, minWithdrawable) || other.minWithdrawable == minWithdrawable) &&
            (identical(other.maxWithdrawable, maxWithdrawable) || other.maxWithdrawable == maxWithdrawable));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, callback, k1, defaultDescription, minWithdrawable, maxWithdrawable);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawRequestDataImplCopyWith<_$LnUrlWithdrawRequestDataImpl> get copyWith =>
      __$$LnUrlWithdrawRequestDataImplCopyWithImpl<_$LnUrlWithdrawRequestDataImpl>(this, _$identity);
}

abstract class _LnUrlWithdrawRequestData implements LnUrlWithdrawRequestData {
  const factory _LnUrlWithdrawRequestData(
      {required final String callback,
      required final String k1,
      required final String defaultDescription,
      required final int minWithdrawable,
      required final int maxWithdrawable}) = _$LnUrlWithdrawRequestDataImpl;

  @override
  String get callback;
  @override
  String get k1;
  @override
  String get defaultDescription;
  @override
  int get minWithdrawable;
  @override
  int get maxWithdrawable;
  @override
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawRequestDataImplCopyWith<_$LnUrlWithdrawRequestDataImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlWithdrawResult {
  Object get data => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawSuccessData data) ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawSuccessData data)? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnUrlWithdrawSuccessData data)? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawResult_Ok value) ok,
    required TResult Function(LnUrlWithdrawResult_ErrorStatus value) errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult? Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlWithdrawResultCopyWith<$Res> {
  factory $LnUrlWithdrawResultCopyWith(LnUrlWithdrawResult value, $Res Function(LnUrlWithdrawResult) then) =
      _$LnUrlWithdrawResultCopyWithImpl<$Res, LnUrlWithdrawResult>;
}

/// @nodoc
class _$LnUrlWithdrawResultCopyWithImpl<$Res, $Val extends LnUrlWithdrawResult>
    implements $LnUrlWithdrawResultCopyWith<$Res> {
  _$LnUrlWithdrawResultCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$LnUrlWithdrawResult_OkImplCopyWith<$Res> {
  factory _$$LnUrlWithdrawResult_OkImplCopyWith(
          _$LnUrlWithdrawResult_OkImpl value, $Res Function(_$LnUrlWithdrawResult_OkImpl) then) =
      __$$LnUrlWithdrawResult_OkImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlWithdrawSuccessData data});
}

/// @nodoc
class __$$LnUrlWithdrawResult_OkImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawResultCopyWithImpl<$Res, _$LnUrlWithdrawResult_OkImpl>
    implements _$$LnUrlWithdrawResult_OkImplCopyWith<$Res> {
  __$$LnUrlWithdrawResult_OkImplCopyWithImpl(
      _$LnUrlWithdrawResult_OkImpl _value, $Res Function(_$LnUrlWithdrawResult_OkImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlWithdrawResult_OkImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlWithdrawSuccessData,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawResult_OkImpl implements LnUrlWithdrawResult_Ok {
  const _$LnUrlWithdrawResult_OkImpl({required this.data});

  @override
  final LnUrlWithdrawSuccessData data;

  @override
  String toString() {
    return 'LnUrlWithdrawResult.ok(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawResult_OkImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawResult_OkImplCopyWith<_$LnUrlWithdrawResult_OkImpl> get copyWith =>
      __$$LnUrlWithdrawResult_OkImplCopyWithImpl<_$LnUrlWithdrawResult_OkImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawSuccessData data) ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) {
    return ok(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawSuccessData data)? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) {
    return ok?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnUrlWithdrawSuccessData data)? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) {
    if (ok != null) {
      return ok(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawResult_Ok value) ok,
    required TResult Function(LnUrlWithdrawResult_ErrorStatus value) errorStatus,
  }) {
    return ok(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult? Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
  }) {
    return ok?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) {
    if (ok != null) {
      return ok(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawResult_Ok implements LnUrlWithdrawResult {
  const factory LnUrlWithdrawResult_Ok({required final LnUrlWithdrawSuccessData data}) =
      _$LnUrlWithdrawResult_OkImpl;

  @override
  LnUrlWithdrawSuccessData get data;
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawResult_OkImplCopyWith<_$LnUrlWithdrawResult_OkImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlWithdrawResult_ErrorStatusImplCopyWith<$Res> {
  factory _$$LnUrlWithdrawResult_ErrorStatusImplCopyWith(_$LnUrlWithdrawResult_ErrorStatusImpl value,
          $Res Function(_$LnUrlWithdrawResult_ErrorStatusImpl) then) =
      __$$LnUrlWithdrawResult_ErrorStatusImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});

  $LnUrlErrorDataCopyWith<$Res> get data;
}

/// @nodoc
class __$$LnUrlWithdrawResult_ErrorStatusImplCopyWithImpl<$Res>
    extends _$LnUrlWithdrawResultCopyWithImpl<$Res, _$LnUrlWithdrawResult_ErrorStatusImpl>
    implements _$$LnUrlWithdrawResult_ErrorStatusImplCopyWith<$Res> {
  __$$LnUrlWithdrawResult_ErrorStatusImplCopyWithImpl(_$LnUrlWithdrawResult_ErrorStatusImpl _value,
      $Res Function(_$LnUrlWithdrawResult_ErrorStatusImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlWithdrawResult_ErrorStatusImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $LnUrlErrorDataCopyWith<$Res> get data {
    return $LnUrlErrorDataCopyWith<$Res>(_value.data, (value) {
      return _then(_value.copyWith(data: value));
    });
  }
}

/// @nodoc

class _$LnUrlWithdrawResult_ErrorStatusImpl implements LnUrlWithdrawResult_ErrorStatus {
  const _$LnUrlWithdrawResult_ErrorStatusImpl({required this.data});

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'LnUrlWithdrawResult.errorStatus(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawResult_ErrorStatusImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawResult_ErrorStatusImplCopyWith<_$LnUrlWithdrawResult_ErrorStatusImpl> get copyWith =>
      __$$LnUrlWithdrawResult_ErrorStatusImplCopyWithImpl<_$LnUrlWithdrawResult_ErrorStatusImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawSuccessData data) ok,
    required TResult Function(LnUrlErrorData data) errorStatus,
  }) {
    return errorStatus(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawSuccessData data)? ok,
    TResult? Function(LnUrlErrorData data)? errorStatus,
  }) {
    return errorStatus?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnUrlWithdrawSuccessData data)? ok,
    TResult Function(LnUrlErrorData data)? errorStatus,
    required TResult orElse(),
  }) {
    if (errorStatus != null) {
      return errorStatus(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlWithdrawResult_Ok value) ok,
    required TResult Function(LnUrlWithdrawResult_ErrorStatus value) errorStatus,
  }) {
    return errorStatus(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult? Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
  }) {
    return errorStatus?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawResult_Ok value)? ok,
    TResult Function(LnUrlWithdrawResult_ErrorStatus value)? errorStatus,
    required TResult orElse(),
  }) {
    if (errorStatus != null) {
      return errorStatus(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawResult_ErrorStatus implements LnUrlWithdrawResult {
  const factory LnUrlWithdrawResult_ErrorStatus({required final LnUrlErrorData data}) =
      _$LnUrlWithdrawResult_ErrorStatusImpl;

  @override
  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawResult_ErrorStatusImplCopyWith<_$LnUrlWithdrawResult_ErrorStatusImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LocaleOverrides {
  String get locale => throw _privateConstructorUsedError;
  int? get spacing => throw _privateConstructorUsedError;
  Symbol get symbol => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LocaleOverridesCopyWith<LocaleOverrides> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LocaleOverridesCopyWith<$Res> {
  factory $LocaleOverridesCopyWith(LocaleOverrides value, $Res Function(LocaleOverrides) then) =
      _$LocaleOverridesCopyWithImpl<$Res, LocaleOverrides>;
  @useResult
  $Res call({String locale, int? spacing, Symbol symbol});

  $SymbolCopyWith<$Res> get symbol;
}

/// @nodoc
class _$LocaleOverridesCopyWithImpl<$Res, $Val extends LocaleOverrides>
    implements $LocaleOverridesCopyWith<$Res> {
  _$LocaleOverridesCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? locale = null,
    Object? spacing = freezed,
    Object? symbol = null,
  }) {
    return _then(_value.copyWith(
      locale: null == locale
          ? _value.locale
          : locale // ignore: cast_nullable_to_non_nullable
              as String,
      spacing: freezed == spacing
          ? _value.spacing
          : spacing // ignore: cast_nullable_to_non_nullable
              as int?,
      symbol: null == symbol
          ? _value.symbol
          : symbol // ignore: cast_nullable_to_non_nullable
              as Symbol,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $SymbolCopyWith<$Res> get symbol {
    return $SymbolCopyWith<$Res>(_value.symbol, (value) {
      return _then(_value.copyWith(symbol: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$LocaleOverridesImplCopyWith<$Res> implements $LocaleOverridesCopyWith<$Res> {
  factory _$$LocaleOverridesImplCopyWith(
          _$LocaleOverridesImpl value, $Res Function(_$LocaleOverridesImpl) then) =
      __$$LocaleOverridesImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String locale, int? spacing, Symbol symbol});

  @override
  $SymbolCopyWith<$Res> get symbol;
}

/// @nodoc
class __$$LocaleOverridesImplCopyWithImpl<$Res>
    extends _$LocaleOverridesCopyWithImpl<$Res, _$LocaleOverridesImpl>
    implements _$$LocaleOverridesImplCopyWith<$Res> {
  __$$LocaleOverridesImplCopyWithImpl(
      _$LocaleOverridesImpl _value, $Res Function(_$LocaleOverridesImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? locale = null,
    Object? spacing = freezed,
    Object? symbol = null,
  }) {
    return _then(_$LocaleOverridesImpl(
      locale: null == locale
          ? _value.locale
          : locale // ignore: cast_nullable_to_non_nullable
              as String,
      spacing: freezed == spacing
          ? _value.spacing
          : spacing // ignore: cast_nullable_to_non_nullable
              as int?,
      symbol: null == symbol
          ? _value.symbol
          : symbol // ignore: cast_nullable_to_non_nullable
              as Symbol,
    ));
  }
}

/// @nodoc

class _$LocaleOverridesImpl implements _LocaleOverrides {
  const _$LocaleOverridesImpl({required this.locale, this.spacing, required this.symbol});

  @override
  final String locale;
  @override
  final int? spacing;
  @override
  final Symbol symbol;

  @override
  String toString() {
    return 'LocaleOverrides(locale: $locale, spacing: $spacing, symbol: $symbol)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LocaleOverridesImpl &&
            (identical(other.locale, locale) || other.locale == locale) &&
            (identical(other.spacing, spacing) || other.spacing == spacing) &&
            (identical(other.symbol, symbol) || other.symbol == symbol));
  }

  @override
  int get hashCode => Object.hash(runtimeType, locale, spacing, symbol);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LocaleOverridesImplCopyWith<_$LocaleOverridesImpl> get copyWith =>
      __$$LocaleOverridesImplCopyWithImpl<_$LocaleOverridesImpl>(this, _$identity);
}

abstract class _LocaleOverrides implements LocaleOverrides {
  const factory _LocaleOverrides(
      {required final String locale,
      final int? spacing,
      required final Symbol symbol}) = _$LocaleOverridesImpl;

  @override
  String get locale;
  @override
  int? get spacing;
  @override
  Symbol get symbol;
  @override
  @JsonKey(ignore: true)
  _$$LocaleOverridesImplCopyWith<_$LocaleOverridesImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LocalizedName {
  String get locale => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LocalizedNameCopyWith<LocalizedName> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LocalizedNameCopyWith<$Res> {
  factory $LocalizedNameCopyWith(LocalizedName value, $Res Function(LocalizedName) then) =
      _$LocalizedNameCopyWithImpl<$Res, LocalizedName>;
  @useResult
  $Res call({String locale, String name});
}

/// @nodoc
class _$LocalizedNameCopyWithImpl<$Res, $Val extends LocalizedName> implements $LocalizedNameCopyWith<$Res> {
  _$LocalizedNameCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? locale = null,
    Object? name = null,
  }) {
    return _then(_value.copyWith(
      locale: null == locale
          ? _value.locale
          : locale // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$LocalizedNameImplCopyWith<$Res> implements $LocalizedNameCopyWith<$Res> {
  factory _$$LocalizedNameImplCopyWith(_$LocalizedNameImpl value, $Res Function(_$LocalizedNameImpl) then) =
      __$$LocalizedNameImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String locale, String name});
}

/// @nodoc
class __$$LocalizedNameImplCopyWithImpl<$Res> extends _$LocalizedNameCopyWithImpl<$Res, _$LocalizedNameImpl>
    implements _$$LocalizedNameImplCopyWith<$Res> {
  __$$LocalizedNameImplCopyWithImpl(_$LocalizedNameImpl _value, $Res Function(_$LocalizedNameImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? locale = null,
    Object? name = null,
  }) {
    return _then(_$LocalizedNameImpl(
      locale: null == locale
          ? _value.locale
          : locale // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LocalizedNameImpl implements _LocalizedName {
  const _$LocalizedNameImpl({required this.locale, required this.name});

  @override
  final String locale;
  @override
  final String name;

  @override
  String toString() {
    return 'LocalizedName(locale: $locale, name: $name)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LocalizedNameImpl &&
            (identical(other.locale, locale) || other.locale == locale) &&
            (identical(other.name, name) || other.name == name));
  }

  @override
  int get hashCode => Object.hash(runtimeType, locale, name);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LocalizedNameImplCopyWith<_$LocalizedNameImpl> get copyWith =>
      __$$LocalizedNameImplCopyWithImpl<_$LocalizedNameImpl>(this, _$identity);
}

abstract class _LocalizedName implements LocalizedName {
  const factory _LocalizedName({required final String locale, required final String name}) =
      _$LocalizedNameImpl;

  @override
  String get locale;
  @override
  String get name;
  @override
  @JsonKey(ignore: true)
  _$$LocalizedNameImplCopyWith<_$LocalizedNameImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LspInformation {
  String get id => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  String get widgetUrl => throw _privateConstructorUsedError;
  String get pubkey => throw _privateConstructorUsedError;
  String get host => throw _privateConstructorUsedError;
  int get baseFeeMsat => throw _privateConstructorUsedError;
  double get feeRate => throw _privateConstructorUsedError;
  int get timeLockDelta => throw _privateConstructorUsedError;
  int get minHtlcMsat => throw _privateConstructorUsedError;
  Uint8List get lspPubkey => throw _privateConstructorUsedError;
  OpeningFeeParamsMenu get openingFeeParamsList => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $LspInformationCopyWith<LspInformation> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LspInformationCopyWith<$Res> {
  factory $LspInformationCopyWith(LspInformation value, $Res Function(LspInformation) then) =
      _$LspInformationCopyWithImpl<$Res, LspInformation>;
  @useResult
  $Res call(
      {String id,
      String name,
      String widgetUrl,
      String pubkey,
      String host,
      int baseFeeMsat,
      double feeRate,
      int timeLockDelta,
      int minHtlcMsat,
      Uint8List lspPubkey,
      OpeningFeeParamsMenu openingFeeParamsList});

  $OpeningFeeParamsMenuCopyWith<$Res> get openingFeeParamsList;
}

/// @nodoc
class _$LspInformationCopyWithImpl<$Res, $Val extends LspInformation>
    implements $LspInformationCopyWith<$Res> {
  _$LspInformationCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? widgetUrl = null,
    Object? pubkey = null,
    Object? host = null,
    Object? baseFeeMsat = null,
    Object? feeRate = null,
    Object? timeLockDelta = null,
    Object? minHtlcMsat = null,
    Object? lspPubkey = null,
    Object? openingFeeParamsList = null,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      widgetUrl: null == widgetUrl
          ? _value.widgetUrl
          : widgetUrl // ignore: cast_nullable_to_non_nullable
              as String,
      pubkey: null == pubkey
          ? _value.pubkey
          : pubkey // ignore: cast_nullable_to_non_nullable
              as String,
      host: null == host
          ? _value.host
          : host // ignore: cast_nullable_to_non_nullable
              as String,
      baseFeeMsat: null == baseFeeMsat
          ? _value.baseFeeMsat
          : baseFeeMsat // ignore: cast_nullable_to_non_nullable
              as int,
      feeRate: null == feeRate
          ? _value.feeRate
          : feeRate // ignore: cast_nullable_to_non_nullable
              as double,
      timeLockDelta: null == timeLockDelta
          ? _value.timeLockDelta
          : timeLockDelta // ignore: cast_nullable_to_non_nullable
              as int,
      minHtlcMsat: null == minHtlcMsat
          ? _value.minHtlcMsat
          : minHtlcMsat // ignore: cast_nullable_to_non_nullable
              as int,
      lspPubkey: null == lspPubkey
          ? _value.lspPubkey
          : lspPubkey // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      openingFeeParamsList: null == openingFeeParamsList
          ? _value.openingFeeParamsList
          : openingFeeParamsList // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParamsMenu,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $OpeningFeeParamsMenuCopyWith<$Res> get openingFeeParamsList {
    return $OpeningFeeParamsMenuCopyWith<$Res>(_value.openingFeeParamsList, (value) {
      return _then(_value.copyWith(openingFeeParamsList: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$LspInformationImplCopyWith<$Res> implements $LspInformationCopyWith<$Res> {
  factory _$$LspInformationImplCopyWith(
          _$LspInformationImpl value, $Res Function(_$LspInformationImpl) then) =
      __$$LspInformationImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String id,
      String name,
      String widgetUrl,
      String pubkey,
      String host,
      int baseFeeMsat,
      double feeRate,
      int timeLockDelta,
      int minHtlcMsat,
      Uint8List lspPubkey,
      OpeningFeeParamsMenu openingFeeParamsList});

  @override
  $OpeningFeeParamsMenuCopyWith<$Res> get openingFeeParamsList;
}

/// @nodoc
class __$$LspInformationImplCopyWithImpl<$Res>
    extends _$LspInformationCopyWithImpl<$Res, _$LspInformationImpl>
    implements _$$LspInformationImplCopyWith<$Res> {
  __$$LspInformationImplCopyWithImpl(_$LspInformationImpl _value, $Res Function(_$LspInformationImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? widgetUrl = null,
    Object? pubkey = null,
    Object? host = null,
    Object? baseFeeMsat = null,
    Object? feeRate = null,
    Object? timeLockDelta = null,
    Object? minHtlcMsat = null,
    Object? lspPubkey = null,
    Object? openingFeeParamsList = null,
  }) {
    return _then(_$LspInformationImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      widgetUrl: null == widgetUrl
          ? _value.widgetUrl
          : widgetUrl // ignore: cast_nullable_to_non_nullable
              as String,
      pubkey: null == pubkey
          ? _value.pubkey
          : pubkey // ignore: cast_nullable_to_non_nullable
              as String,
      host: null == host
          ? _value.host
          : host // ignore: cast_nullable_to_non_nullable
              as String,
      baseFeeMsat: null == baseFeeMsat
          ? _value.baseFeeMsat
          : baseFeeMsat // ignore: cast_nullable_to_non_nullable
              as int,
      feeRate: null == feeRate
          ? _value.feeRate
          : feeRate // ignore: cast_nullable_to_non_nullable
              as double,
      timeLockDelta: null == timeLockDelta
          ? _value.timeLockDelta
          : timeLockDelta // ignore: cast_nullable_to_non_nullable
              as int,
      minHtlcMsat: null == minHtlcMsat
          ? _value.minHtlcMsat
          : minHtlcMsat // ignore: cast_nullable_to_non_nullable
              as int,
      lspPubkey: null == lspPubkey
          ? _value.lspPubkey
          : lspPubkey // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      openingFeeParamsList: null == openingFeeParamsList
          ? _value.openingFeeParamsList
          : openingFeeParamsList // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParamsMenu,
    ));
  }
}

/// @nodoc

class _$LspInformationImpl implements _LspInformation {
  const _$LspInformationImpl(
      {required this.id,
      required this.name,
      required this.widgetUrl,
      required this.pubkey,
      required this.host,
      required this.baseFeeMsat,
      required this.feeRate,
      required this.timeLockDelta,
      required this.minHtlcMsat,
      required this.lspPubkey,
      required this.openingFeeParamsList});

  @override
  final String id;
  @override
  final String name;
  @override
  final String widgetUrl;
  @override
  final String pubkey;
  @override
  final String host;
  @override
  final int baseFeeMsat;
  @override
  final double feeRate;
  @override
  final int timeLockDelta;
  @override
  final int minHtlcMsat;
  @override
  final Uint8List lspPubkey;
  @override
  final OpeningFeeParamsMenu openingFeeParamsList;

  @override
  String toString() {
    return 'LspInformation(id: $id, name: $name, widgetUrl: $widgetUrl, pubkey: $pubkey, host: $host, baseFeeMsat: $baseFeeMsat, feeRate: $feeRate, timeLockDelta: $timeLockDelta, minHtlcMsat: $minHtlcMsat, lspPubkey: $lspPubkey, openingFeeParamsList: $openingFeeParamsList)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LspInformationImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.widgetUrl, widgetUrl) || other.widgetUrl == widgetUrl) &&
            (identical(other.pubkey, pubkey) || other.pubkey == pubkey) &&
            (identical(other.host, host) || other.host == host) &&
            (identical(other.baseFeeMsat, baseFeeMsat) || other.baseFeeMsat == baseFeeMsat) &&
            (identical(other.feeRate, feeRate) || other.feeRate == feeRate) &&
            (identical(other.timeLockDelta, timeLockDelta) || other.timeLockDelta == timeLockDelta) &&
            (identical(other.minHtlcMsat, minHtlcMsat) || other.minHtlcMsat == minHtlcMsat) &&
            const DeepCollectionEquality().equals(other.lspPubkey, lspPubkey) &&
            (identical(other.openingFeeParamsList, openingFeeParamsList) ||
                other.openingFeeParamsList == openingFeeParamsList));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id, name, widgetUrl, pubkey, host, baseFeeMsat, feeRate,
      timeLockDelta, minHtlcMsat, const DeepCollectionEquality().hash(lspPubkey), openingFeeParamsList);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LspInformationImplCopyWith<_$LspInformationImpl> get copyWith =>
      __$$LspInformationImplCopyWithImpl<_$LspInformationImpl>(this, _$identity);
}

abstract class _LspInformation implements LspInformation {
  const factory _LspInformation(
      {required final String id,
      required final String name,
      required final String widgetUrl,
      required final String pubkey,
      required final String host,
      required final int baseFeeMsat,
      required final double feeRate,
      required final int timeLockDelta,
      required final int minHtlcMsat,
      required final Uint8List lspPubkey,
      required final OpeningFeeParamsMenu openingFeeParamsList}) = _$LspInformationImpl;

  @override
  String get id;
  @override
  String get name;
  @override
  String get widgetUrl;
  @override
  String get pubkey;
  @override
  String get host;
  @override
  int get baseFeeMsat;
  @override
  double get feeRate;
  @override
  int get timeLockDelta;
  @override
  int get minHtlcMsat;
  @override
  Uint8List get lspPubkey;
  @override
  OpeningFeeParamsMenu get openingFeeParamsList;
  @override
  @JsonKey(ignore: true)
  _$$LspInformationImplCopyWith<_$LspInformationImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$NodeConfig {
  GreenlightNodeConfig get config => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(GreenlightNodeConfig config) greenlight,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(GreenlightNodeConfig config)? greenlight,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(GreenlightNodeConfig config)? greenlight,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(NodeConfig_Greenlight value) greenlight,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(NodeConfig_Greenlight value)? greenlight,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(NodeConfig_Greenlight value)? greenlight,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $NodeConfigCopyWith<NodeConfig> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $NodeConfigCopyWith<$Res> {
  factory $NodeConfigCopyWith(NodeConfig value, $Res Function(NodeConfig) then) =
      _$NodeConfigCopyWithImpl<$Res, NodeConfig>;
  @useResult
  $Res call({GreenlightNodeConfig config});

  $GreenlightNodeConfigCopyWith<$Res> get config;
}

/// @nodoc
class _$NodeConfigCopyWithImpl<$Res, $Val extends NodeConfig> implements $NodeConfigCopyWith<$Res> {
  _$NodeConfigCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? config = null,
  }) {
    return _then(_value.copyWith(
      config: null == config
          ? _value.config
          : config // ignore: cast_nullable_to_non_nullable
              as GreenlightNodeConfig,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $GreenlightNodeConfigCopyWith<$Res> get config {
    return $GreenlightNodeConfigCopyWith<$Res>(_value.config, (value) {
      return _then(_value.copyWith(config: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$NodeConfig_GreenlightImplCopyWith<$Res> implements $NodeConfigCopyWith<$Res> {
  factory _$$NodeConfig_GreenlightImplCopyWith(
          _$NodeConfig_GreenlightImpl value, $Res Function(_$NodeConfig_GreenlightImpl) then) =
      __$$NodeConfig_GreenlightImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({GreenlightNodeConfig config});

  @override
  $GreenlightNodeConfigCopyWith<$Res> get config;
}

/// @nodoc
class __$$NodeConfig_GreenlightImplCopyWithImpl<$Res>
    extends _$NodeConfigCopyWithImpl<$Res, _$NodeConfig_GreenlightImpl>
    implements _$$NodeConfig_GreenlightImplCopyWith<$Res> {
  __$$NodeConfig_GreenlightImplCopyWithImpl(
      _$NodeConfig_GreenlightImpl _value, $Res Function(_$NodeConfig_GreenlightImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? config = null,
  }) {
    return _then(_$NodeConfig_GreenlightImpl(
      config: null == config
          ? _value.config
          : config // ignore: cast_nullable_to_non_nullable
              as GreenlightNodeConfig,
    ));
  }
}

/// @nodoc

class _$NodeConfig_GreenlightImpl implements NodeConfig_Greenlight {
  const _$NodeConfig_GreenlightImpl({required this.config});

  @override
  final GreenlightNodeConfig config;

  @override
  String toString() {
    return 'NodeConfig.greenlight(config: $config)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$NodeConfig_GreenlightImpl &&
            (identical(other.config, config) || other.config == config));
  }

  @override
  int get hashCode => Object.hash(runtimeType, config);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$NodeConfig_GreenlightImplCopyWith<_$NodeConfig_GreenlightImpl> get copyWith =>
      __$$NodeConfig_GreenlightImplCopyWithImpl<_$NodeConfig_GreenlightImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(GreenlightNodeConfig config) greenlight,
  }) {
    return greenlight(config);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(GreenlightNodeConfig config)? greenlight,
  }) {
    return greenlight?.call(config);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(GreenlightNodeConfig config)? greenlight,
    required TResult orElse(),
  }) {
    if (greenlight != null) {
      return greenlight(config);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(NodeConfig_Greenlight value) greenlight,
  }) {
    return greenlight(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(NodeConfig_Greenlight value)? greenlight,
  }) {
    return greenlight?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(NodeConfig_Greenlight value)? greenlight,
    required TResult orElse(),
  }) {
    if (greenlight != null) {
      return greenlight(this);
    }
    return orElse();
  }
}

abstract class NodeConfig_Greenlight implements NodeConfig {
  const factory NodeConfig_Greenlight({required final GreenlightNodeConfig config}) =
      _$NodeConfig_GreenlightImpl;

  @override
  GreenlightNodeConfig get config;
  @override
  @JsonKey(ignore: true)
  _$$NodeConfig_GreenlightImplCopyWith<_$NodeConfig_GreenlightImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$NodeCredentials {
  GreenlightCredentials get credentials => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(GreenlightCredentials credentials) greenlight,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(GreenlightCredentials credentials)? greenlight,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(GreenlightCredentials credentials)? greenlight,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(NodeCredentials_Greenlight value) greenlight,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(NodeCredentials_Greenlight value)? greenlight,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(NodeCredentials_Greenlight value)? greenlight,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $NodeCredentialsCopyWith<NodeCredentials> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $NodeCredentialsCopyWith<$Res> {
  factory $NodeCredentialsCopyWith(NodeCredentials value, $Res Function(NodeCredentials) then) =
      _$NodeCredentialsCopyWithImpl<$Res, NodeCredentials>;
  @useResult
  $Res call({GreenlightCredentials credentials});
}

/// @nodoc
class _$NodeCredentialsCopyWithImpl<$Res, $Val extends NodeCredentials>
    implements $NodeCredentialsCopyWith<$Res> {
  _$NodeCredentialsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? credentials = null,
  }) {
    return _then(_value.copyWith(
      credentials: null == credentials
          ? _value.credentials
          : credentials // ignore: cast_nullable_to_non_nullable
              as GreenlightCredentials,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$NodeCredentials_GreenlightImplCopyWith<$Res> implements $NodeCredentialsCopyWith<$Res> {
  factory _$$NodeCredentials_GreenlightImplCopyWith(
          _$NodeCredentials_GreenlightImpl value, $Res Function(_$NodeCredentials_GreenlightImpl) then) =
      __$$NodeCredentials_GreenlightImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({GreenlightCredentials credentials});
}

/// @nodoc
class __$$NodeCredentials_GreenlightImplCopyWithImpl<$Res>
    extends _$NodeCredentialsCopyWithImpl<$Res, _$NodeCredentials_GreenlightImpl>
    implements _$$NodeCredentials_GreenlightImplCopyWith<$Res> {
  __$$NodeCredentials_GreenlightImplCopyWithImpl(
      _$NodeCredentials_GreenlightImpl _value, $Res Function(_$NodeCredentials_GreenlightImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? credentials = null,
  }) {
    return _then(_$NodeCredentials_GreenlightImpl(
      credentials: null == credentials
          ? _value.credentials
          : credentials // ignore: cast_nullable_to_non_nullable
              as GreenlightCredentials,
    ));
  }
}

/// @nodoc

class _$NodeCredentials_GreenlightImpl implements NodeCredentials_Greenlight {
  const _$NodeCredentials_GreenlightImpl({required this.credentials});

  @override
  final GreenlightCredentials credentials;

  @override
  String toString() {
    return 'NodeCredentials.greenlight(credentials: $credentials)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$NodeCredentials_GreenlightImpl &&
            (identical(other.credentials, credentials) || other.credentials == credentials));
  }

  @override
  int get hashCode => Object.hash(runtimeType, credentials);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$NodeCredentials_GreenlightImplCopyWith<_$NodeCredentials_GreenlightImpl> get copyWith =>
      __$$NodeCredentials_GreenlightImplCopyWithImpl<_$NodeCredentials_GreenlightImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(GreenlightCredentials credentials) greenlight,
  }) {
    return greenlight(credentials);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(GreenlightCredentials credentials)? greenlight,
  }) {
    return greenlight?.call(credentials);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(GreenlightCredentials credentials)? greenlight,
    required TResult orElse(),
  }) {
    if (greenlight != null) {
      return greenlight(credentials);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(NodeCredentials_Greenlight value) greenlight,
  }) {
    return greenlight(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(NodeCredentials_Greenlight value)? greenlight,
  }) {
    return greenlight?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(NodeCredentials_Greenlight value)? greenlight,
    required TResult orElse(),
  }) {
    if (greenlight != null) {
      return greenlight(this);
    }
    return orElse();
  }
}

abstract class NodeCredentials_Greenlight implements NodeCredentials {
  const factory NodeCredentials_Greenlight({required final GreenlightCredentials credentials}) =
      _$NodeCredentials_GreenlightImpl;

  @override
  GreenlightCredentials get credentials;
  @override
  @JsonKey(ignore: true)
  _$$NodeCredentials_GreenlightImplCopyWith<_$NodeCredentials_GreenlightImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$NodeState {
  String get id => throw _privateConstructorUsedError;
  int get blockHeight => throw _privateConstructorUsedError;
  int get channelsBalanceMsat => throw _privateConstructorUsedError;
  int get onchainBalanceMsat => throw _privateConstructorUsedError;
  int get pendingOnchainBalanceMsat => throw _privateConstructorUsedError;
  List<UnspentTransactionOutput> get utxos => throw _privateConstructorUsedError;
  int get maxPayableMsat => throw _privateConstructorUsedError;
  int get maxReceivableMsat => throw _privateConstructorUsedError;
  int get maxSinglePaymentAmountMsat => throw _privateConstructorUsedError;
  int get maxChanReserveMsats => throw _privateConstructorUsedError;
  List<String> get connectedPeers => throw _privateConstructorUsedError;
  int get inboundLiquidityMsats => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $NodeStateCopyWith<NodeState> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $NodeStateCopyWith<$Res> {
  factory $NodeStateCopyWith(NodeState value, $Res Function(NodeState) then) =
      _$NodeStateCopyWithImpl<$Res, NodeState>;
  @useResult
  $Res call(
      {String id,
      int blockHeight,
      int channelsBalanceMsat,
      int onchainBalanceMsat,
      int pendingOnchainBalanceMsat,
      List<UnspentTransactionOutput> utxos,
      int maxPayableMsat,
      int maxReceivableMsat,
      int maxSinglePaymentAmountMsat,
      int maxChanReserveMsats,
      List<String> connectedPeers,
      int inboundLiquidityMsats});
}

/// @nodoc
class _$NodeStateCopyWithImpl<$Res, $Val extends NodeState> implements $NodeStateCopyWith<$Res> {
  _$NodeStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? blockHeight = null,
    Object? channelsBalanceMsat = null,
    Object? onchainBalanceMsat = null,
    Object? pendingOnchainBalanceMsat = null,
    Object? utxos = null,
    Object? maxPayableMsat = null,
    Object? maxReceivableMsat = null,
    Object? maxSinglePaymentAmountMsat = null,
    Object? maxChanReserveMsats = null,
    Object? connectedPeers = null,
    Object? inboundLiquidityMsats = null,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      blockHeight: null == blockHeight
          ? _value.blockHeight
          : blockHeight // ignore: cast_nullable_to_non_nullable
              as int,
      channelsBalanceMsat: null == channelsBalanceMsat
          ? _value.channelsBalanceMsat
          : channelsBalanceMsat // ignore: cast_nullable_to_non_nullable
              as int,
      onchainBalanceMsat: null == onchainBalanceMsat
          ? _value.onchainBalanceMsat
          : onchainBalanceMsat // ignore: cast_nullable_to_non_nullable
              as int,
      pendingOnchainBalanceMsat: null == pendingOnchainBalanceMsat
          ? _value.pendingOnchainBalanceMsat
          : pendingOnchainBalanceMsat // ignore: cast_nullable_to_non_nullable
              as int,
      utxos: null == utxos
          ? _value.utxos
          : utxos // ignore: cast_nullable_to_non_nullable
              as List<UnspentTransactionOutput>,
      maxPayableMsat: null == maxPayableMsat
          ? _value.maxPayableMsat
          : maxPayableMsat // ignore: cast_nullable_to_non_nullable
              as int,
      maxReceivableMsat: null == maxReceivableMsat
          ? _value.maxReceivableMsat
          : maxReceivableMsat // ignore: cast_nullable_to_non_nullable
              as int,
      maxSinglePaymentAmountMsat: null == maxSinglePaymentAmountMsat
          ? _value.maxSinglePaymentAmountMsat
          : maxSinglePaymentAmountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      maxChanReserveMsats: null == maxChanReserveMsats
          ? _value.maxChanReserveMsats
          : maxChanReserveMsats // ignore: cast_nullable_to_non_nullable
              as int,
      connectedPeers: null == connectedPeers
          ? _value.connectedPeers
          : connectedPeers // ignore: cast_nullable_to_non_nullable
              as List<String>,
      inboundLiquidityMsats: null == inboundLiquidityMsats
          ? _value.inboundLiquidityMsats
          : inboundLiquidityMsats // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$NodeStateImplCopyWith<$Res> implements $NodeStateCopyWith<$Res> {
  factory _$$NodeStateImplCopyWith(_$NodeStateImpl value, $Res Function(_$NodeStateImpl) then) =
      __$$NodeStateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String id,
      int blockHeight,
      int channelsBalanceMsat,
      int onchainBalanceMsat,
      int pendingOnchainBalanceMsat,
      List<UnspentTransactionOutput> utxos,
      int maxPayableMsat,
      int maxReceivableMsat,
      int maxSinglePaymentAmountMsat,
      int maxChanReserveMsats,
      List<String> connectedPeers,
      int inboundLiquidityMsats});
}

/// @nodoc
class __$$NodeStateImplCopyWithImpl<$Res> extends _$NodeStateCopyWithImpl<$Res, _$NodeStateImpl>
    implements _$$NodeStateImplCopyWith<$Res> {
  __$$NodeStateImplCopyWithImpl(_$NodeStateImpl _value, $Res Function(_$NodeStateImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? blockHeight = null,
    Object? channelsBalanceMsat = null,
    Object? onchainBalanceMsat = null,
    Object? pendingOnchainBalanceMsat = null,
    Object? utxos = null,
    Object? maxPayableMsat = null,
    Object? maxReceivableMsat = null,
    Object? maxSinglePaymentAmountMsat = null,
    Object? maxChanReserveMsats = null,
    Object? connectedPeers = null,
    Object? inboundLiquidityMsats = null,
  }) {
    return _then(_$NodeStateImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      blockHeight: null == blockHeight
          ? _value.blockHeight
          : blockHeight // ignore: cast_nullable_to_non_nullable
              as int,
      channelsBalanceMsat: null == channelsBalanceMsat
          ? _value.channelsBalanceMsat
          : channelsBalanceMsat // ignore: cast_nullable_to_non_nullable
              as int,
      onchainBalanceMsat: null == onchainBalanceMsat
          ? _value.onchainBalanceMsat
          : onchainBalanceMsat // ignore: cast_nullable_to_non_nullable
              as int,
      pendingOnchainBalanceMsat: null == pendingOnchainBalanceMsat
          ? _value.pendingOnchainBalanceMsat
          : pendingOnchainBalanceMsat // ignore: cast_nullable_to_non_nullable
              as int,
      utxos: null == utxos
          ? _value._utxos
          : utxos // ignore: cast_nullable_to_non_nullable
              as List<UnspentTransactionOutput>,
      maxPayableMsat: null == maxPayableMsat
          ? _value.maxPayableMsat
          : maxPayableMsat // ignore: cast_nullable_to_non_nullable
              as int,
      maxReceivableMsat: null == maxReceivableMsat
          ? _value.maxReceivableMsat
          : maxReceivableMsat // ignore: cast_nullable_to_non_nullable
              as int,
      maxSinglePaymentAmountMsat: null == maxSinglePaymentAmountMsat
          ? _value.maxSinglePaymentAmountMsat
          : maxSinglePaymentAmountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      maxChanReserveMsats: null == maxChanReserveMsats
          ? _value.maxChanReserveMsats
          : maxChanReserveMsats // ignore: cast_nullable_to_non_nullable
              as int,
      connectedPeers: null == connectedPeers
          ? _value._connectedPeers
          : connectedPeers // ignore: cast_nullable_to_non_nullable
              as List<String>,
      inboundLiquidityMsats: null == inboundLiquidityMsats
          ? _value.inboundLiquidityMsats
          : inboundLiquidityMsats // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$NodeStateImpl implements _NodeState {
  const _$NodeStateImpl(
      {required this.id,
      required this.blockHeight,
      required this.channelsBalanceMsat,
      required this.onchainBalanceMsat,
      required this.pendingOnchainBalanceMsat,
      required final List<UnspentTransactionOutput> utxos,
      required this.maxPayableMsat,
      required this.maxReceivableMsat,
      required this.maxSinglePaymentAmountMsat,
      required this.maxChanReserveMsats,
      required final List<String> connectedPeers,
      required this.inboundLiquidityMsats})
      : _utxos = utxos,
        _connectedPeers = connectedPeers;

  @override
  final String id;
  @override
  final int blockHeight;
  @override
  final int channelsBalanceMsat;
  @override
  final int onchainBalanceMsat;
  @override
  final int pendingOnchainBalanceMsat;
  final List<UnspentTransactionOutput> _utxos;
  @override
  List<UnspentTransactionOutput> get utxos {
    if (_utxos is EqualUnmodifiableListView) return _utxos;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_utxos);
  }

  @override
  final int maxPayableMsat;
  @override
  final int maxReceivableMsat;
  @override
  final int maxSinglePaymentAmountMsat;
  @override
  final int maxChanReserveMsats;
  final List<String> _connectedPeers;
  @override
  List<String> get connectedPeers {
    if (_connectedPeers is EqualUnmodifiableListView) return _connectedPeers;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_connectedPeers);
  }

  @override
  final int inboundLiquidityMsats;

  @override
  String toString() {
    return 'NodeState(id: $id, blockHeight: $blockHeight, channelsBalanceMsat: $channelsBalanceMsat, onchainBalanceMsat: $onchainBalanceMsat, pendingOnchainBalanceMsat: $pendingOnchainBalanceMsat, utxos: $utxos, maxPayableMsat: $maxPayableMsat, maxReceivableMsat: $maxReceivableMsat, maxSinglePaymentAmountMsat: $maxSinglePaymentAmountMsat, maxChanReserveMsats: $maxChanReserveMsats, connectedPeers: $connectedPeers, inboundLiquidityMsats: $inboundLiquidityMsats)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$NodeStateImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.blockHeight, blockHeight) || other.blockHeight == blockHeight) &&
            (identical(other.channelsBalanceMsat, channelsBalanceMsat) ||
                other.channelsBalanceMsat == channelsBalanceMsat) &&
            (identical(other.onchainBalanceMsat, onchainBalanceMsat) ||
                other.onchainBalanceMsat == onchainBalanceMsat) &&
            (identical(other.pendingOnchainBalanceMsat, pendingOnchainBalanceMsat) ||
                other.pendingOnchainBalanceMsat == pendingOnchainBalanceMsat) &&
            const DeepCollectionEquality().equals(other._utxos, _utxos) &&
            (identical(other.maxPayableMsat, maxPayableMsat) || other.maxPayableMsat == maxPayableMsat) &&
            (identical(other.maxReceivableMsat, maxReceivableMsat) ||
                other.maxReceivableMsat == maxReceivableMsat) &&
            (identical(other.maxSinglePaymentAmountMsat, maxSinglePaymentAmountMsat) ||
                other.maxSinglePaymentAmountMsat == maxSinglePaymentAmountMsat) &&
            (identical(other.maxChanReserveMsats, maxChanReserveMsats) ||
                other.maxChanReserveMsats == maxChanReserveMsats) &&
            const DeepCollectionEquality().equals(other._connectedPeers, _connectedPeers) &&
            (identical(other.inboundLiquidityMsats, inboundLiquidityMsats) ||
                other.inboundLiquidityMsats == inboundLiquidityMsats));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      id,
      blockHeight,
      channelsBalanceMsat,
      onchainBalanceMsat,
      pendingOnchainBalanceMsat,
      const DeepCollectionEquality().hash(_utxos),
      maxPayableMsat,
      maxReceivableMsat,
      maxSinglePaymentAmountMsat,
      maxChanReserveMsats,
      const DeepCollectionEquality().hash(_connectedPeers),
      inboundLiquidityMsats);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$NodeStateImplCopyWith<_$NodeStateImpl> get copyWith =>
      __$$NodeStateImplCopyWithImpl<_$NodeStateImpl>(this, _$identity);
}

abstract class _NodeState implements NodeState {
  const factory _NodeState(
      {required final String id,
      required final int blockHeight,
      required final int channelsBalanceMsat,
      required final int onchainBalanceMsat,
      required final int pendingOnchainBalanceMsat,
      required final List<UnspentTransactionOutput> utxos,
      required final int maxPayableMsat,
      required final int maxReceivableMsat,
      required final int maxSinglePaymentAmountMsat,
      required final int maxChanReserveMsats,
      required final List<String> connectedPeers,
      required final int inboundLiquidityMsats}) = _$NodeStateImpl;

  @override
  String get id;
  @override
  int get blockHeight;
  @override
  int get channelsBalanceMsat;
  @override
  int get onchainBalanceMsat;
  @override
  int get pendingOnchainBalanceMsat;
  @override
  List<UnspentTransactionOutput> get utxos;
  @override
  int get maxPayableMsat;
  @override
  int get maxReceivableMsat;
  @override
  int get maxSinglePaymentAmountMsat;
  @override
  int get maxChanReserveMsats;
  @override
  List<String> get connectedPeers;
  @override
  int get inboundLiquidityMsats;
  @override
  @JsonKey(ignore: true)
  _$$NodeStateImplCopyWith<_$NodeStateImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$OpeningFeeParams {
  int get minMsat => throw _privateConstructorUsedError;
  int get proportional => throw _privateConstructorUsedError;
  String get validUntil => throw _privateConstructorUsedError;
  int get maxIdleTime => throw _privateConstructorUsedError;
  int get maxClientToSelfDelay => throw _privateConstructorUsedError;
  String get promise => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $OpeningFeeParamsCopyWith<OpeningFeeParams> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OpeningFeeParamsCopyWith<$Res> {
  factory $OpeningFeeParamsCopyWith(OpeningFeeParams value, $Res Function(OpeningFeeParams) then) =
      _$OpeningFeeParamsCopyWithImpl<$Res, OpeningFeeParams>;
  @useResult
  $Res call(
      {int minMsat,
      int proportional,
      String validUntil,
      int maxIdleTime,
      int maxClientToSelfDelay,
      String promise});
}

/// @nodoc
class _$OpeningFeeParamsCopyWithImpl<$Res, $Val extends OpeningFeeParams>
    implements $OpeningFeeParamsCopyWith<$Res> {
  _$OpeningFeeParamsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? minMsat = null,
    Object? proportional = null,
    Object? validUntil = null,
    Object? maxIdleTime = null,
    Object? maxClientToSelfDelay = null,
    Object? promise = null,
  }) {
    return _then(_value.copyWith(
      minMsat: null == minMsat
          ? _value.minMsat
          : minMsat // ignore: cast_nullable_to_non_nullable
              as int,
      proportional: null == proportional
          ? _value.proportional
          : proportional // ignore: cast_nullable_to_non_nullable
              as int,
      validUntil: null == validUntil
          ? _value.validUntil
          : validUntil // ignore: cast_nullable_to_non_nullable
              as String,
      maxIdleTime: null == maxIdleTime
          ? _value.maxIdleTime
          : maxIdleTime // ignore: cast_nullable_to_non_nullable
              as int,
      maxClientToSelfDelay: null == maxClientToSelfDelay
          ? _value.maxClientToSelfDelay
          : maxClientToSelfDelay // ignore: cast_nullable_to_non_nullable
              as int,
      promise: null == promise
          ? _value.promise
          : promise // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$OpeningFeeParamsImplCopyWith<$Res> implements $OpeningFeeParamsCopyWith<$Res> {
  factory _$$OpeningFeeParamsImplCopyWith(
          _$OpeningFeeParamsImpl value, $Res Function(_$OpeningFeeParamsImpl) then) =
      __$$OpeningFeeParamsImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int minMsat,
      int proportional,
      String validUntil,
      int maxIdleTime,
      int maxClientToSelfDelay,
      String promise});
}

/// @nodoc
class __$$OpeningFeeParamsImplCopyWithImpl<$Res>
    extends _$OpeningFeeParamsCopyWithImpl<$Res, _$OpeningFeeParamsImpl>
    implements _$$OpeningFeeParamsImplCopyWith<$Res> {
  __$$OpeningFeeParamsImplCopyWithImpl(
      _$OpeningFeeParamsImpl _value, $Res Function(_$OpeningFeeParamsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? minMsat = null,
    Object? proportional = null,
    Object? validUntil = null,
    Object? maxIdleTime = null,
    Object? maxClientToSelfDelay = null,
    Object? promise = null,
  }) {
    return _then(_$OpeningFeeParamsImpl(
      minMsat: null == minMsat
          ? _value.minMsat
          : minMsat // ignore: cast_nullable_to_non_nullable
              as int,
      proportional: null == proportional
          ? _value.proportional
          : proportional // ignore: cast_nullable_to_non_nullable
              as int,
      validUntil: null == validUntil
          ? _value.validUntil
          : validUntil // ignore: cast_nullable_to_non_nullable
              as String,
      maxIdleTime: null == maxIdleTime
          ? _value.maxIdleTime
          : maxIdleTime // ignore: cast_nullable_to_non_nullable
              as int,
      maxClientToSelfDelay: null == maxClientToSelfDelay
          ? _value.maxClientToSelfDelay
          : maxClientToSelfDelay // ignore: cast_nullable_to_non_nullable
              as int,
      promise: null == promise
          ? _value.promise
          : promise // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$OpeningFeeParamsImpl implements _OpeningFeeParams {
  const _$OpeningFeeParamsImpl(
      {required this.minMsat,
      required this.proportional,
      required this.validUntil,
      required this.maxIdleTime,
      required this.maxClientToSelfDelay,
      required this.promise});

  @override
  final int minMsat;
  @override
  final int proportional;
  @override
  final String validUntil;
  @override
  final int maxIdleTime;
  @override
  final int maxClientToSelfDelay;
  @override
  final String promise;

  @override
  String toString() {
    return 'OpeningFeeParams(minMsat: $minMsat, proportional: $proportional, validUntil: $validUntil, maxIdleTime: $maxIdleTime, maxClientToSelfDelay: $maxClientToSelfDelay, promise: $promise)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$OpeningFeeParamsImpl &&
            (identical(other.minMsat, minMsat) || other.minMsat == minMsat) &&
            (identical(other.proportional, proportional) || other.proportional == proportional) &&
            (identical(other.validUntil, validUntil) || other.validUntil == validUntil) &&
            (identical(other.maxIdleTime, maxIdleTime) || other.maxIdleTime == maxIdleTime) &&
            (identical(other.maxClientToSelfDelay, maxClientToSelfDelay) ||
                other.maxClientToSelfDelay == maxClientToSelfDelay) &&
            (identical(other.promise, promise) || other.promise == promise));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, minMsat, proportional, validUntil, maxIdleTime, maxClientToSelfDelay, promise);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$OpeningFeeParamsImplCopyWith<_$OpeningFeeParamsImpl> get copyWith =>
      __$$OpeningFeeParamsImplCopyWithImpl<_$OpeningFeeParamsImpl>(this, _$identity);
}

abstract class _OpeningFeeParams implements OpeningFeeParams {
  const factory _OpeningFeeParams(
      {required final int minMsat,
      required final int proportional,
      required final String validUntil,
      required final int maxIdleTime,
      required final int maxClientToSelfDelay,
      required final String promise}) = _$OpeningFeeParamsImpl;

  @override
  int get minMsat;
  @override
  int get proportional;
  @override
  String get validUntil;
  @override
  int get maxIdleTime;
  @override
  int get maxClientToSelfDelay;
  @override
  String get promise;
  @override
  @JsonKey(ignore: true)
  _$$OpeningFeeParamsImplCopyWith<_$OpeningFeeParamsImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$OpeningFeeParamsMenu {
  List<OpeningFeeParams> get values => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $OpeningFeeParamsMenuCopyWith<OpeningFeeParamsMenu> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OpeningFeeParamsMenuCopyWith<$Res> {
  factory $OpeningFeeParamsMenuCopyWith(
          OpeningFeeParamsMenu value, $Res Function(OpeningFeeParamsMenu) then) =
      _$OpeningFeeParamsMenuCopyWithImpl<$Res, OpeningFeeParamsMenu>;
  @useResult
  $Res call({List<OpeningFeeParams> values});
}

/// @nodoc
class _$OpeningFeeParamsMenuCopyWithImpl<$Res, $Val extends OpeningFeeParamsMenu>
    implements $OpeningFeeParamsMenuCopyWith<$Res> {
  _$OpeningFeeParamsMenuCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? values = null,
  }) {
    return _then(_value.copyWith(
      values: null == values
          ? _value.values
          : values // ignore: cast_nullable_to_non_nullable
              as List<OpeningFeeParams>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$OpeningFeeParamsMenuImplCopyWith<$Res> implements $OpeningFeeParamsMenuCopyWith<$Res> {
  factory _$$OpeningFeeParamsMenuImplCopyWith(
          _$OpeningFeeParamsMenuImpl value, $Res Function(_$OpeningFeeParamsMenuImpl) then) =
      __$$OpeningFeeParamsMenuImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<OpeningFeeParams> values});
}

/// @nodoc
class __$$OpeningFeeParamsMenuImplCopyWithImpl<$Res>
    extends _$OpeningFeeParamsMenuCopyWithImpl<$Res, _$OpeningFeeParamsMenuImpl>
    implements _$$OpeningFeeParamsMenuImplCopyWith<$Res> {
  __$$OpeningFeeParamsMenuImplCopyWithImpl(
      _$OpeningFeeParamsMenuImpl _value, $Res Function(_$OpeningFeeParamsMenuImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? values = null,
  }) {
    return _then(_$OpeningFeeParamsMenuImpl(
      values: null == values
          ? _value._values
          : values // ignore: cast_nullable_to_non_nullable
              as List<OpeningFeeParams>,
    ));
  }
}

/// @nodoc

class _$OpeningFeeParamsMenuImpl implements _OpeningFeeParamsMenu {
  const _$OpeningFeeParamsMenuImpl({required final List<OpeningFeeParams> values}) : _values = values;

  final List<OpeningFeeParams> _values;
  @override
  List<OpeningFeeParams> get values {
    if (_values is EqualUnmodifiableListView) return _values;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_values);
  }

  @override
  String toString() {
    return 'OpeningFeeParamsMenu(values: $values)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$OpeningFeeParamsMenuImpl &&
            const DeepCollectionEquality().equals(other._values, _values));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(_values));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$OpeningFeeParamsMenuImplCopyWith<_$OpeningFeeParamsMenuImpl> get copyWith =>
      __$$OpeningFeeParamsMenuImplCopyWithImpl<_$OpeningFeeParamsMenuImpl>(this, _$identity);
}

abstract class _OpeningFeeParamsMenu implements OpeningFeeParamsMenu {
  const factory _OpeningFeeParamsMenu({required final List<OpeningFeeParams> values}) =
      _$OpeningFeeParamsMenuImpl;

  @override
  List<OpeningFeeParams> get values;
  @override
  @JsonKey(ignore: true)
  _$$OpeningFeeParamsMenuImplCopyWith<_$OpeningFeeParamsMenuImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Payment {
  String get id => throw _privateConstructorUsedError;
  PaymentType get paymentType => throw _privateConstructorUsedError;
  int get paymentTime => throw _privateConstructorUsedError;
  int get amountMsat => throw _privateConstructorUsedError;
  int get feeMsat => throw _privateConstructorUsedError;
  PaymentStatus get status => throw _privateConstructorUsedError;
  String? get error => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;
  PaymentDetails get details => throw _privateConstructorUsedError;
  String? get metadata => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $PaymentCopyWith<Payment> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PaymentCopyWith<$Res> {
  factory $PaymentCopyWith(Payment value, $Res Function(Payment) then) = _$PaymentCopyWithImpl<$Res, Payment>;
  @useResult
  $Res call(
      {String id,
      PaymentType paymentType,
      int paymentTime,
      int amountMsat,
      int feeMsat,
      PaymentStatus status,
      String? error,
      String? description,
      PaymentDetails details,
      String? metadata});

  $PaymentDetailsCopyWith<$Res> get details;
}

/// @nodoc
class _$PaymentCopyWithImpl<$Res, $Val extends Payment> implements $PaymentCopyWith<$Res> {
  _$PaymentCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? paymentType = null,
    Object? paymentTime = null,
    Object? amountMsat = null,
    Object? feeMsat = null,
    Object? status = null,
    Object? error = freezed,
    Object? description = freezed,
    Object? details = null,
    Object? metadata = freezed,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      paymentType: null == paymentType
          ? _value.paymentType
          : paymentType // ignore: cast_nullable_to_non_nullable
              as PaymentType,
      paymentTime: null == paymentTime
          ? _value.paymentTime
          : paymentTime // ignore: cast_nullable_to_non_nullable
              as int,
      amountMsat: null == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      feeMsat: null == feeMsat
          ? _value.feeMsat
          : feeMsat // ignore: cast_nullable_to_non_nullable
              as int,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as PaymentStatus,
      error: freezed == error
          ? _value.error
          : error // ignore: cast_nullable_to_non_nullable
              as String?,
      description: freezed == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String?,
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as PaymentDetails,
      metadata: freezed == metadata
          ? _value.metadata
          : metadata // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $PaymentDetailsCopyWith<$Res> get details {
    return $PaymentDetailsCopyWith<$Res>(_value.details, (value) {
      return _then(_value.copyWith(details: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$PaymentImplCopyWith<$Res> implements $PaymentCopyWith<$Res> {
  factory _$$PaymentImplCopyWith(_$PaymentImpl value, $Res Function(_$PaymentImpl) then) =
      __$$PaymentImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String id,
      PaymentType paymentType,
      int paymentTime,
      int amountMsat,
      int feeMsat,
      PaymentStatus status,
      String? error,
      String? description,
      PaymentDetails details,
      String? metadata});

  @override
  $PaymentDetailsCopyWith<$Res> get details;
}

/// @nodoc
class __$$PaymentImplCopyWithImpl<$Res> extends _$PaymentCopyWithImpl<$Res, _$PaymentImpl>
    implements _$$PaymentImplCopyWith<$Res> {
  __$$PaymentImplCopyWithImpl(_$PaymentImpl _value, $Res Function(_$PaymentImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? paymentType = null,
    Object? paymentTime = null,
    Object? amountMsat = null,
    Object? feeMsat = null,
    Object? status = null,
    Object? error = freezed,
    Object? description = freezed,
    Object? details = null,
    Object? metadata = freezed,
  }) {
    return _then(_$PaymentImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      paymentType: null == paymentType
          ? _value.paymentType
          : paymentType // ignore: cast_nullable_to_non_nullable
              as PaymentType,
      paymentTime: null == paymentTime
          ? _value.paymentTime
          : paymentTime // ignore: cast_nullable_to_non_nullable
              as int,
      amountMsat: null == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      feeMsat: null == feeMsat
          ? _value.feeMsat
          : feeMsat // ignore: cast_nullable_to_non_nullable
              as int,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as PaymentStatus,
      error: freezed == error
          ? _value.error
          : error // ignore: cast_nullable_to_non_nullable
              as String?,
      description: freezed == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String?,
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as PaymentDetails,
      metadata: freezed == metadata
          ? _value.metadata
          : metadata // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class _$PaymentImpl implements _Payment {
  const _$PaymentImpl(
      {required this.id,
      required this.paymentType,
      required this.paymentTime,
      required this.amountMsat,
      required this.feeMsat,
      required this.status,
      this.error,
      this.description,
      required this.details,
      this.metadata});

  @override
  final String id;
  @override
  final PaymentType paymentType;
  @override
  final int paymentTime;
  @override
  final int amountMsat;
  @override
  final int feeMsat;
  @override
  final PaymentStatus status;
  @override
  final String? error;
  @override
  final String? description;
  @override
  final PaymentDetails details;
  @override
  final String? metadata;

  @override
  String toString() {
    return 'Payment(id: $id, paymentType: $paymentType, paymentTime: $paymentTime, amountMsat: $amountMsat, feeMsat: $feeMsat, status: $status, error: $error, description: $description, details: $details, metadata: $metadata)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.paymentType, paymentType) || other.paymentType == paymentType) &&
            (identical(other.paymentTime, paymentTime) || other.paymentTime == paymentTime) &&
            (identical(other.amountMsat, amountMsat) || other.amountMsat == amountMsat) &&
            (identical(other.feeMsat, feeMsat) || other.feeMsat == feeMsat) &&
            (identical(other.status, status) || other.status == status) &&
            (identical(other.error, error) || other.error == error) &&
            (identical(other.description, description) || other.description == description) &&
            (identical(other.details, details) || other.details == details) &&
            (identical(other.metadata, metadata) || other.metadata == metadata));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id, paymentType, paymentTime, amountMsat, feeMsat, status,
      error, description, details, metadata);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentImplCopyWith<_$PaymentImpl> get copyWith =>
      __$$PaymentImplCopyWithImpl<_$PaymentImpl>(this, _$identity);
}

abstract class _Payment implements Payment {
  const factory _Payment(
      {required final String id,
      required final PaymentType paymentType,
      required final int paymentTime,
      required final int amountMsat,
      required final int feeMsat,
      required final PaymentStatus status,
      final String? error,
      final String? description,
      required final PaymentDetails details,
      final String? metadata}) = _$PaymentImpl;

  @override
  String get id;
  @override
  PaymentType get paymentType;
  @override
  int get paymentTime;
  @override
  int get amountMsat;
  @override
  int get feeMsat;
  @override
  PaymentStatus get status;
  @override
  String? get error;
  @override
  String? get description;
  @override
  PaymentDetails get details;
  @override
  String? get metadata;
  @override
  @JsonKey(ignore: true)
  _$$PaymentImplCopyWith<_$PaymentImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$PaymentDetails {
  Object get data => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnPaymentDetails data) ln,
    required TResult Function(ClosedChannelPaymentDetails data) closedChannel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnPaymentDetails data)? ln,
    TResult? Function(ClosedChannelPaymentDetails data)? closedChannel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnPaymentDetails data)? ln,
    TResult Function(ClosedChannelPaymentDetails data)? closedChannel,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PaymentDetails_Ln value) ln,
    required TResult Function(PaymentDetails_ClosedChannel value) closedChannel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PaymentDetails_Ln value)? ln,
    TResult? Function(PaymentDetails_ClosedChannel value)? closedChannel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PaymentDetails_Ln value)? ln,
    TResult Function(PaymentDetails_ClosedChannel value)? closedChannel,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PaymentDetailsCopyWith<$Res> {
  factory $PaymentDetailsCopyWith(PaymentDetails value, $Res Function(PaymentDetails) then) =
      _$PaymentDetailsCopyWithImpl<$Res, PaymentDetails>;
}

/// @nodoc
class _$PaymentDetailsCopyWithImpl<$Res, $Val extends PaymentDetails>
    implements $PaymentDetailsCopyWith<$Res> {
  _$PaymentDetailsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$PaymentDetails_LnImplCopyWith<$Res> {
  factory _$$PaymentDetails_LnImplCopyWith(
          _$PaymentDetails_LnImpl value, $Res Function(_$PaymentDetails_LnImpl) then) =
      __$$PaymentDetails_LnImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnPaymentDetails data});

  $LnPaymentDetailsCopyWith<$Res> get data;
}

/// @nodoc
class __$$PaymentDetails_LnImplCopyWithImpl<$Res>
    extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_LnImpl>
    implements _$$PaymentDetails_LnImplCopyWith<$Res> {
  __$$PaymentDetails_LnImplCopyWithImpl(
      _$PaymentDetails_LnImpl _value, $Res Function(_$PaymentDetails_LnImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$PaymentDetails_LnImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnPaymentDetails,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $LnPaymentDetailsCopyWith<$Res> get data {
    return $LnPaymentDetailsCopyWith<$Res>(_value.data, (value) {
      return _then(_value.copyWith(data: value));
    });
  }
}

/// @nodoc

class _$PaymentDetails_LnImpl implements PaymentDetails_Ln {
  const _$PaymentDetails_LnImpl({required this.data});

  @override
  final LnPaymentDetails data;

  @override
  String toString() {
    return 'PaymentDetails.ln(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDetails_LnImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDetails_LnImplCopyWith<_$PaymentDetails_LnImpl> get copyWith =>
      __$$PaymentDetails_LnImplCopyWithImpl<_$PaymentDetails_LnImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnPaymentDetails data) ln,
    required TResult Function(ClosedChannelPaymentDetails data) closedChannel,
  }) {
    return ln(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnPaymentDetails data)? ln,
    TResult? Function(ClosedChannelPaymentDetails data)? closedChannel,
  }) {
    return ln?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnPaymentDetails data)? ln,
    TResult Function(ClosedChannelPaymentDetails data)? closedChannel,
    required TResult orElse(),
  }) {
    if (ln != null) {
      return ln(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PaymentDetails_Ln value) ln,
    required TResult Function(PaymentDetails_ClosedChannel value) closedChannel,
  }) {
    return ln(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PaymentDetails_Ln value)? ln,
    TResult? Function(PaymentDetails_ClosedChannel value)? closedChannel,
  }) {
    return ln?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PaymentDetails_Ln value)? ln,
    TResult Function(PaymentDetails_ClosedChannel value)? closedChannel,
    required TResult orElse(),
  }) {
    if (ln != null) {
      return ln(this);
    }
    return orElse();
  }
}

abstract class PaymentDetails_Ln implements PaymentDetails {
  const factory PaymentDetails_Ln({required final LnPaymentDetails data}) = _$PaymentDetails_LnImpl;

  @override
  LnPaymentDetails get data;
  @JsonKey(ignore: true)
  _$$PaymentDetails_LnImplCopyWith<_$PaymentDetails_LnImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PaymentDetails_ClosedChannelImplCopyWith<$Res> {
  factory _$$PaymentDetails_ClosedChannelImplCopyWith(
          _$PaymentDetails_ClosedChannelImpl value, $Res Function(_$PaymentDetails_ClosedChannelImpl) then) =
      __$$PaymentDetails_ClosedChannelImplCopyWithImpl<$Res>;
  @useResult
  $Res call({ClosedChannelPaymentDetails data});

  $ClosedChannelPaymentDetailsCopyWith<$Res> get data;
}

/// @nodoc
class __$$PaymentDetails_ClosedChannelImplCopyWithImpl<$Res>
    extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_ClosedChannelImpl>
    implements _$$PaymentDetails_ClosedChannelImplCopyWith<$Res> {
  __$$PaymentDetails_ClosedChannelImplCopyWithImpl(
      _$PaymentDetails_ClosedChannelImpl _value, $Res Function(_$PaymentDetails_ClosedChannelImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$PaymentDetails_ClosedChannelImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as ClosedChannelPaymentDetails,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ClosedChannelPaymentDetailsCopyWith<$Res> get data {
    return $ClosedChannelPaymentDetailsCopyWith<$Res>(_value.data, (value) {
      return _then(_value.copyWith(data: value));
    });
  }
}

/// @nodoc

class _$PaymentDetails_ClosedChannelImpl implements PaymentDetails_ClosedChannel {
  const _$PaymentDetails_ClosedChannelImpl({required this.data});

  @override
  final ClosedChannelPaymentDetails data;

  @override
  String toString() {
    return 'PaymentDetails.closedChannel(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDetails_ClosedChannelImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDetails_ClosedChannelImplCopyWith<_$PaymentDetails_ClosedChannelImpl> get copyWith =>
      __$$PaymentDetails_ClosedChannelImplCopyWithImpl<_$PaymentDetails_ClosedChannelImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnPaymentDetails data) ln,
    required TResult Function(ClosedChannelPaymentDetails data) closedChannel,
  }) {
    return closedChannel(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnPaymentDetails data)? ln,
    TResult? Function(ClosedChannelPaymentDetails data)? closedChannel,
  }) {
    return closedChannel?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnPaymentDetails data)? ln,
    TResult Function(ClosedChannelPaymentDetails data)? closedChannel,
    required TResult orElse(),
  }) {
    if (closedChannel != null) {
      return closedChannel(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PaymentDetails_Ln value) ln,
    required TResult Function(PaymentDetails_ClosedChannel value) closedChannel,
  }) {
    return closedChannel(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PaymentDetails_Ln value)? ln,
    TResult? Function(PaymentDetails_ClosedChannel value)? closedChannel,
  }) {
    return closedChannel?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PaymentDetails_Ln value)? ln,
    TResult Function(PaymentDetails_ClosedChannel value)? closedChannel,
    required TResult orElse(),
  }) {
    if (closedChannel != null) {
      return closedChannel(this);
    }
    return orElse();
  }
}

abstract class PaymentDetails_ClosedChannel implements PaymentDetails {
  const factory PaymentDetails_ClosedChannel({required final ClosedChannelPaymentDetails data}) =
      _$PaymentDetails_ClosedChannelImpl;

  @override
  ClosedChannelPaymentDetails get data;
  @JsonKey(ignore: true)
  _$$PaymentDetails_ClosedChannelImplCopyWith<_$PaymentDetails_ClosedChannelImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$ReceiveOnchainRequest {
  OpeningFeeParams? get openingFeeParams => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ReceiveOnchainRequestCopyWith<ReceiveOnchainRequest> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ReceiveOnchainRequestCopyWith<$Res> {
  factory $ReceiveOnchainRequestCopyWith(
          ReceiveOnchainRequest value, $Res Function(ReceiveOnchainRequest) then) =
      _$ReceiveOnchainRequestCopyWithImpl<$Res, ReceiveOnchainRequest>;
  @useResult
  $Res call({OpeningFeeParams? openingFeeParams});

  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams;
}

/// @nodoc
class _$ReceiveOnchainRequestCopyWithImpl<$Res, $Val extends ReceiveOnchainRequest>
    implements $ReceiveOnchainRequestCopyWith<$Res> {
  _$ReceiveOnchainRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? openingFeeParams = freezed,
  }) {
    return _then(_value.copyWith(
      openingFeeParams: freezed == openingFeeParams
          ? _value.openingFeeParams
          : openingFeeParams // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams {
    if (_value.openingFeeParams == null) {
      return null;
    }

    return $OpeningFeeParamsCopyWith<$Res>(_value.openingFeeParams!, (value) {
      return _then(_value.copyWith(openingFeeParams: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$ReceiveOnchainRequestImplCopyWith<$Res> implements $ReceiveOnchainRequestCopyWith<$Res> {
  factory _$$ReceiveOnchainRequestImplCopyWith(
          _$ReceiveOnchainRequestImpl value, $Res Function(_$ReceiveOnchainRequestImpl) then) =
      __$$ReceiveOnchainRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({OpeningFeeParams? openingFeeParams});

  @override
  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams;
}

/// @nodoc
class __$$ReceiveOnchainRequestImplCopyWithImpl<$Res>
    extends _$ReceiveOnchainRequestCopyWithImpl<$Res, _$ReceiveOnchainRequestImpl>
    implements _$$ReceiveOnchainRequestImplCopyWith<$Res> {
  __$$ReceiveOnchainRequestImplCopyWithImpl(
      _$ReceiveOnchainRequestImpl _value, $Res Function(_$ReceiveOnchainRequestImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? openingFeeParams = freezed,
  }) {
    return _then(_$ReceiveOnchainRequestImpl(
      openingFeeParams: freezed == openingFeeParams
          ? _value.openingFeeParams
          : openingFeeParams // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
    ));
  }
}

/// @nodoc

class _$ReceiveOnchainRequestImpl implements _ReceiveOnchainRequest {
  const _$ReceiveOnchainRequestImpl({this.openingFeeParams});

  @override
  final OpeningFeeParams? openingFeeParams;

  @override
  String toString() {
    return 'ReceiveOnchainRequest(openingFeeParams: $openingFeeParams)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ReceiveOnchainRequestImpl &&
            (identical(other.openingFeeParams, openingFeeParams) ||
                other.openingFeeParams == openingFeeParams));
  }

  @override
  int get hashCode => Object.hash(runtimeType, openingFeeParams);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ReceiveOnchainRequestImplCopyWith<_$ReceiveOnchainRequestImpl> get copyWith =>
      __$$ReceiveOnchainRequestImplCopyWithImpl<_$ReceiveOnchainRequestImpl>(this, _$identity);
}

abstract class _ReceiveOnchainRequest implements ReceiveOnchainRequest {
  const factory _ReceiveOnchainRequest({final OpeningFeeParams? openingFeeParams}) =
      _$ReceiveOnchainRequestImpl;

  @override
  OpeningFeeParams? get openingFeeParams;
  @override
  @JsonKey(ignore: true)
  _$$ReceiveOnchainRequestImplCopyWith<_$ReceiveOnchainRequestImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$ReceivePaymentRequest {
  int get amountMsat => throw _privateConstructorUsedError;
  String get description => throw _privateConstructorUsedError;
  Uint8List? get preimage => throw _privateConstructorUsedError;
  OpeningFeeParams? get openingFeeParams => throw _privateConstructorUsedError;
  bool? get useDescriptionHash => throw _privateConstructorUsedError;
  int? get expiry => throw _privateConstructorUsedError;
  int? get cltv => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ReceivePaymentRequestCopyWith<ReceivePaymentRequest> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ReceivePaymentRequestCopyWith<$Res> {
  factory $ReceivePaymentRequestCopyWith(
          ReceivePaymentRequest value, $Res Function(ReceivePaymentRequest) then) =
      _$ReceivePaymentRequestCopyWithImpl<$Res, ReceivePaymentRequest>;
  @useResult
  $Res call(
      {int amountMsat,
      String description,
      Uint8List? preimage,
      OpeningFeeParams? openingFeeParams,
      bool? useDescriptionHash,
      int? expiry,
      int? cltv});

  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams;
}

/// @nodoc
class _$ReceivePaymentRequestCopyWithImpl<$Res, $Val extends ReceivePaymentRequest>
    implements $ReceivePaymentRequestCopyWith<$Res> {
  _$ReceivePaymentRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? amountMsat = null,
    Object? description = null,
    Object? preimage = freezed,
    Object? openingFeeParams = freezed,
    Object? useDescriptionHash = freezed,
    Object? expiry = freezed,
    Object? cltv = freezed,
  }) {
    return _then(_value.copyWith(
      amountMsat: null == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
      preimage: freezed == preimage
          ? _value.preimage
          : preimage // ignore: cast_nullable_to_non_nullable
              as Uint8List?,
      openingFeeParams: freezed == openingFeeParams
          ? _value.openingFeeParams
          : openingFeeParams // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
      useDescriptionHash: freezed == useDescriptionHash
          ? _value.useDescriptionHash
          : useDescriptionHash // ignore: cast_nullable_to_non_nullable
              as bool?,
      expiry: freezed == expiry
          ? _value.expiry
          : expiry // ignore: cast_nullable_to_non_nullable
              as int?,
      cltv: freezed == cltv
          ? _value.cltv
          : cltv // ignore: cast_nullable_to_non_nullable
              as int?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams {
    if (_value.openingFeeParams == null) {
      return null;
    }

    return $OpeningFeeParamsCopyWith<$Res>(_value.openingFeeParams!, (value) {
      return _then(_value.copyWith(openingFeeParams: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$ReceivePaymentRequestImplCopyWith<$Res> implements $ReceivePaymentRequestCopyWith<$Res> {
  factory _$$ReceivePaymentRequestImplCopyWith(
          _$ReceivePaymentRequestImpl value, $Res Function(_$ReceivePaymentRequestImpl) then) =
      __$$ReceivePaymentRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int amountMsat,
      String description,
      Uint8List? preimage,
      OpeningFeeParams? openingFeeParams,
      bool? useDescriptionHash,
      int? expiry,
      int? cltv});

  @override
  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams;
}

/// @nodoc
class __$$ReceivePaymentRequestImplCopyWithImpl<$Res>
    extends _$ReceivePaymentRequestCopyWithImpl<$Res, _$ReceivePaymentRequestImpl>
    implements _$$ReceivePaymentRequestImplCopyWith<$Res> {
  __$$ReceivePaymentRequestImplCopyWithImpl(
      _$ReceivePaymentRequestImpl _value, $Res Function(_$ReceivePaymentRequestImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? amountMsat = null,
    Object? description = null,
    Object? preimage = freezed,
    Object? openingFeeParams = freezed,
    Object? useDescriptionHash = freezed,
    Object? expiry = freezed,
    Object? cltv = freezed,
  }) {
    return _then(_$ReceivePaymentRequestImpl(
      amountMsat: null == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
      preimage: freezed == preimage
          ? _value.preimage
          : preimage // ignore: cast_nullable_to_non_nullable
              as Uint8List?,
      openingFeeParams: freezed == openingFeeParams
          ? _value.openingFeeParams
          : openingFeeParams // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
      useDescriptionHash: freezed == useDescriptionHash
          ? _value.useDescriptionHash
          : useDescriptionHash // ignore: cast_nullable_to_non_nullable
              as bool?,
      expiry: freezed == expiry
          ? _value.expiry
          : expiry // ignore: cast_nullable_to_non_nullable
              as int?,
      cltv: freezed == cltv
          ? _value.cltv
          : cltv // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$ReceivePaymentRequestImpl implements _ReceivePaymentRequest {
  const _$ReceivePaymentRequestImpl(
      {required this.amountMsat,
      required this.description,
      this.preimage,
      this.openingFeeParams,
      this.useDescriptionHash,
      this.expiry,
      this.cltv});

  @override
  final int amountMsat;
  @override
  final String description;
  @override
  final Uint8List? preimage;
  @override
  final OpeningFeeParams? openingFeeParams;
  @override
  final bool? useDescriptionHash;
  @override
  final int? expiry;
  @override
  final int? cltv;

  @override
  String toString() {
    return 'ReceivePaymentRequest(amountMsat: $amountMsat, description: $description, preimage: $preimage, openingFeeParams: $openingFeeParams, useDescriptionHash: $useDescriptionHash, expiry: $expiry, cltv: $cltv)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ReceivePaymentRequestImpl &&
            (identical(other.amountMsat, amountMsat) || other.amountMsat == amountMsat) &&
            (identical(other.description, description) || other.description == description) &&
            const DeepCollectionEquality().equals(other.preimage, preimage) &&
            (identical(other.openingFeeParams, openingFeeParams) ||
                other.openingFeeParams == openingFeeParams) &&
            (identical(other.useDescriptionHash, useDescriptionHash) ||
                other.useDescriptionHash == useDescriptionHash) &&
            (identical(other.expiry, expiry) || other.expiry == expiry) &&
            (identical(other.cltv, cltv) || other.cltv == cltv));
  }

  @override
  int get hashCode => Object.hash(runtimeType, amountMsat, description,
      const DeepCollectionEquality().hash(preimage), openingFeeParams, useDescriptionHash, expiry, cltv);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ReceivePaymentRequestImplCopyWith<_$ReceivePaymentRequestImpl> get copyWith =>
      __$$ReceivePaymentRequestImplCopyWithImpl<_$ReceivePaymentRequestImpl>(this, _$identity);
}

abstract class _ReceivePaymentRequest implements ReceivePaymentRequest {
  const factory _ReceivePaymentRequest(
      {required final int amountMsat,
      required final String description,
      final Uint8List? preimage,
      final OpeningFeeParams? openingFeeParams,
      final bool? useDescriptionHash,
      final int? expiry,
      final int? cltv}) = _$ReceivePaymentRequestImpl;

  @override
  int get amountMsat;
  @override
  String get description;
  @override
  Uint8List? get preimage;
  @override
  OpeningFeeParams? get openingFeeParams;
  @override
  bool? get useDescriptionHash;
  @override
  int? get expiry;
  @override
  int? get cltv;
  @override
  @JsonKey(ignore: true)
  _$$ReceivePaymentRequestImplCopyWith<_$ReceivePaymentRequestImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$ReceivePaymentResponse {
  LNInvoice get lnInvoice => throw _privateConstructorUsedError;
  OpeningFeeParams? get openingFeeParams => throw _privateConstructorUsedError;
  int? get openingFeeMsat => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ReceivePaymentResponseCopyWith<ReceivePaymentResponse> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ReceivePaymentResponseCopyWith<$Res> {
  factory $ReceivePaymentResponseCopyWith(
          ReceivePaymentResponse value, $Res Function(ReceivePaymentResponse) then) =
      _$ReceivePaymentResponseCopyWithImpl<$Res, ReceivePaymentResponse>;
  @useResult
  $Res call({LNInvoice lnInvoice, OpeningFeeParams? openingFeeParams, int? openingFeeMsat});

  $LNInvoiceCopyWith<$Res> get lnInvoice;
  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams;
}

/// @nodoc
class _$ReceivePaymentResponseCopyWithImpl<$Res, $Val extends ReceivePaymentResponse>
    implements $ReceivePaymentResponseCopyWith<$Res> {
  _$ReceivePaymentResponseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? lnInvoice = null,
    Object? openingFeeParams = freezed,
    Object? openingFeeMsat = freezed,
  }) {
    return _then(_value.copyWith(
      lnInvoice: null == lnInvoice
          ? _value.lnInvoice
          : lnInvoice // ignore: cast_nullable_to_non_nullable
              as LNInvoice,
      openingFeeParams: freezed == openingFeeParams
          ? _value.openingFeeParams
          : openingFeeParams // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
      openingFeeMsat: freezed == openingFeeMsat
          ? _value.openingFeeMsat
          : openingFeeMsat // ignore: cast_nullable_to_non_nullable
              as int?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $LNInvoiceCopyWith<$Res> get lnInvoice {
    return $LNInvoiceCopyWith<$Res>(_value.lnInvoice, (value) {
      return _then(_value.copyWith(lnInvoice: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams {
    if (_value.openingFeeParams == null) {
      return null;
    }

    return $OpeningFeeParamsCopyWith<$Res>(_value.openingFeeParams!, (value) {
      return _then(_value.copyWith(openingFeeParams: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$ReceivePaymentResponseImplCopyWith<$Res> implements $ReceivePaymentResponseCopyWith<$Res> {
  factory _$$ReceivePaymentResponseImplCopyWith(
          _$ReceivePaymentResponseImpl value, $Res Function(_$ReceivePaymentResponseImpl) then) =
      __$$ReceivePaymentResponseImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({LNInvoice lnInvoice, OpeningFeeParams? openingFeeParams, int? openingFeeMsat});

  @override
  $LNInvoiceCopyWith<$Res> get lnInvoice;
  @override
  $OpeningFeeParamsCopyWith<$Res>? get openingFeeParams;
}

/// @nodoc
class __$$ReceivePaymentResponseImplCopyWithImpl<$Res>
    extends _$ReceivePaymentResponseCopyWithImpl<$Res, _$ReceivePaymentResponseImpl>
    implements _$$ReceivePaymentResponseImplCopyWith<$Res> {
  __$$ReceivePaymentResponseImplCopyWithImpl(
      _$ReceivePaymentResponseImpl _value, $Res Function(_$ReceivePaymentResponseImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? lnInvoice = null,
    Object? openingFeeParams = freezed,
    Object? openingFeeMsat = freezed,
  }) {
    return _then(_$ReceivePaymentResponseImpl(
      lnInvoice: null == lnInvoice
          ? _value.lnInvoice
          : lnInvoice // ignore: cast_nullable_to_non_nullable
              as LNInvoice,
      openingFeeParams: freezed == openingFeeParams
          ? _value.openingFeeParams
          : openingFeeParams // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
      openingFeeMsat: freezed == openingFeeMsat
          ? _value.openingFeeMsat
          : openingFeeMsat // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$ReceivePaymentResponseImpl implements _ReceivePaymentResponse {
  const _$ReceivePaymentResponseImpl({required this.lnInvoice, this.openingFeeParams, this.openingFeeMsat});

  @override
  final LNInvoice lnInvoice;
  @override
  final OpeningFeeParams? openingFeeParams;
  @override
  final int? openingFeeMsat;

  @override
  String toString() {
    return 'ReceivePaymentResponse(lnInvoice: $lnInvoice, openingFeeParams: $openingFeeParams, openingFeeMsat: $openingFeeMsat)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ReceivePaymentResponseImpl &&
            (identical(other.lnInvoice, lnInvoice) || other.lnInvoice == lnInvoice) &&
            (identical(other.openingFeeParams, openingFeeParams) ||
                other.openingFeeParams == openingFeeParams) &&
            (identical(other.openingFeeMsat, openingFeeMsat) || other.openingFeeMsat == openingFeeMsat));
  }

  @override
  int get hashCode => Object.hash(runtimeType, lnInvoice, openingFeeParams, openingFeeMsat);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ReceivePaymentResponseImplCopyWith<_$ReceivePaymentResponseImpl> get copyWith =>
      __$$ReceivePaymentResponseImplCopyWithImpl<_$ReceivePaymentResponseImpl>(this, _$identity);
}

abstract class _ReceivePaymentResponse implements ReceivePaymentResponse {
  const factory _ReceivePaymentResponse(
      {required final LNInvoice lnInvoice,
      final OpeningFeeParams? openingFeeParams,
      final int? openingFeeMsat}) = _$ReceivePaymentResponseImpl;

  @override
  LNInvoice get lnInvoice;
  @override
  OpeningFeeParams? get openingFeeParams;
  @override
  int? get openingFeeMsat;
  @override
  @JsonKey(ignore: true)
  _$$ReceivePaymentResponseImplCopyWith<_$ReceivePaymentResponseImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$ReportIssueRequest {
  ReportPaymentFailureDetails get data => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ReportPaymentFailureDetails data) paymentFailure,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ReportPaymentFailureDetails data)? paymentFailure,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ReportPaymentFailureDetails data)? paymentFailure,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ReportIssueRequest_PaymentFailure value) paymentFailure,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ReportIssueRequest_PaymentFailure value)? paymentFailure,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ReportIssueRequest_PaymentFailure value)? paymentFailure,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ReportIssueRequestCopyWith<ReportIssueRequest> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ReportIssueRequestCopyWith<$Res> {
  factory $ReportIssueRequestCopyWith(ReportIssueRequest value, $Res Function(ReportIssueRequest) then) =
      _$ReportIssueRequestCopyWithImpl<$Res, ReportIssueRequest>;
  @useResult
  $Res call({ReportPaymentFailureDetails data});
}

/// @nodoc
class _$ReportIssueRequestCopyWithImpl<$Res, $Val extends ReportIssueRequest>
    implements $ReportIssueRequestCopyWith<$Res> {
  _$ReportIssueRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_value.copyWith(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as ReportPaymentFailureDetails,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ReportIssueRequest_PaymentFailureImplCopyWith<$Res>
    implements $ReportIssueRequestCopyWith<$Res> {
  factory _$$ReportIssueRequest_PaymentFailureImplCopyWith(_$ReportIssueRequest_PaymentFailureImpl value,
          $Res Function(_$ReportIssueRequest_PaymentFailureImpl) then) =
      __$$ReportIssueRequest_PaymentFailureImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({ReportPaymentFailureDetails data});
}

/// @nodoc
class __$$ReportIssueRequest_PaymentFailureImplCopyWithImpl<$Res>
    extends _$ReportIssueRequestCopyWithImpl<$Res, _$ReportIssueRequest_PaymentFailureImpl>
    implements _$$ReportIssueRequest_PaymentFailureImplCopyWith<$Res> {
  __$$ReportIssueRequest_PaymentFailureImplCopyWithImpl(_$ReportIssueRequest_PaymentFailureImpl _value,
      $Res Function(_$ReportIssueRequest_PaymentFailureImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$ReportIssueRequest_PaymentFailureImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as ReportPaymentFailureDetails,
    ));
  }
}

/// @nodoc

class _$ReportIssueRequest_PaymentFailureImpl implements ReportIssueRequest_PaymentFailure {
  const _$ReportIssueRequest_PaymentFailureImpl({required this.data});

  @override
  final ReportPaymentFailureDetails data;

  @override
  String toString() {
    return 'ReportIssueRequest.paymentFailure(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ReportIssueRequest_PaymentFailureImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ReportIssueRequest_PaymentFailureImplCopyWith<_$ReportIssueRequest_PaymentFailureImpl> get copyWith =>
      __$$ReportIssueRequest_PaymentFailureImplCopyWithImpl<_$ReportIssueRequest_PaymentFailureImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ReportPaymentFailureDetails data) paymentFailure,
  }) {
    return paymentFailure(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ReportPaymentFailureDetails data)? paymentFailure,
  }) {
    return paymentFailure?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ReportPaymentFailureDetails data)? paymentFailure,
    required TResult orElse(),
  }) {
    if (paymentFailure != null) {
      return paymentFailure(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ReportIssueRequest_PaymentFailure value) paymentFailure,
  }) {
    return paymentFailure(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ReportIssueRequest_PaymentFailure value)? paymentFailure,
  }) {
    return paymentFailure?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ReportIssueRequest_PaymentFailure value)? paymentFailure,
    required TResult orElse(),
  }) {
    if (paymentFailure != null) {
      return paymentFailure(this);
    }
    return orElse();
  }
}

abstract class ReportIssueRequest_PaymentFailure implements ReportIssueRequest {
  const factory ReportIssueRequest_PaymentFailure({required final ReportPaymentFailureDetails data}) =
      _$ReportIssueRequest_PaymentFailureImpl;

  @override
  ReportPaymentFailureDetails get data;
  @override
  @JsonKey(ignore: true)
  _$$ReportIssueRequest_PaymentFailureImplCopyWith<_$ReportIssueRequest_PaymentFailureImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$ReverseSwapInfo {
  String get id => throw _privateConstructorUsedError;
  String get claimPubkey => throw _privateConstructorUsedError;
  String? get lockupTxid => throw _privateConstructorUsedError;
  String? get claimTxid => throw _privateConstructorUsedError;
  int get onchainAmountSat => throw _privateConstructorUsedError;
  ReverseSwapStatus get status => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ReverseSwapInfoCopyWith<ReverseSwapInfo> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ReverseSwapInfoCopyWith<$Res> {
  factory $ReverseSwapInfoCopyWith(ReverseSwapInfo value, $Res Function(ReverseSwapInfo) then) =
      _$ReverseSwapInfoCopyWithImpl<$Res, ReverseSwapInfo>;
  @useResult
  $Res call(
      {String id,
      String claimPubkey,
      String? lockupTxid,
      String? claimTxid,
      int onchainAmountSat,
      ReverseSwapStatus status});
}

/// @nodoc
class _$ReverseSwapInfoCopyWithImpl<$Res, $Val extends ReverseSwapInfo>
    implements $ReverseSwapInfoCopyWith<$Res> {
  _$ReverseSwapInfoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? claimPubkey = null,
    Object? lockupTxid = freezed,
    Object? claimTxid = freezed,
    Object? onchainAmountSat = null,
    Object? status = null,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      claimPubkey: null == claimPubkey
          ? _value.claimPubkey
          : claimPubkey // ignore: cast_nullable_to_non_nullable
              as String,
      lockupTxid: freezed == lockupTxid
          ? _value.lockupTxid
          : lockupTxid // ignore: cast_nullable_to_non_nullable
              as String?,
      claimTxid: freezed == claimTxid
          ? _value.claimTxid
          : claimTxid // ignore: cast_nullable_to_non_nullable
              as String?,
      onchainAmountSat: null == onchainAmountSat
          ? _value.onchainAmountSat
          : onchainAmountSat // ignore: cast_nullable_to_non_nullable
              as int,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as ReverseSwapStatus,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ReverseSwapInfoImplCopyWith<$Res> implements $ReverseSwapInfoCopyWith<$Res> {
  factory _$$ReverseSwapInfoImplCopyWith(
          _$ReverseSwapInfoImpl value, $Res Function(_$ReverseSwapInfoImpl) then) =
      __$$ReverseSwapInfoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String id,
      String claimPubkey,
      String? lockupTxid,
      String? claimTxid,
      int onchainAmountSat,
      ReverseSwapStatus status});
}

/// @nodoc
class __$$ReverseSwapInfoImplCopyWithImpl<$Res>
    extends _$ReverseSwapInfoCopyWithImpl<$Res, _$ReverseSwapInfoImpl>
    implements _$$ReverseSwapInfoImplCopyWith<$Res> {
  __$$ReverseSwapInfoImplCopyWithImpl(
      _$ReverseSwapInfoImpl _value, $Res Function(_$ReverseSwapInfoImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? claimPubkey = null,
    Object? lockupTxid = freezed,
    Object? claimTxid = freezed,
    Object? onchainAmountSat = null,
    Object? status = null,
  }) {
    return _then(_$ReverseSwapInfoImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      claimPubkey: null == claimPubkey
          ? _value.claimPubkey
          : claimPubkey // ignore: cast_nullable_to_non_nullable
              as String,
      lockupTxid: freezed == lockupTxid
          ? _value.lockupTxid
          : lockupTxid // ignore: cast_nullable_to_non_nullable
              as String?,
      claimTxid: freezed == claimTxid
          ? _value.claimTxid
          : claimTxid // ignore: cast_nullable_to_non_nullable
              as String?,
      onchainAmountSat: null == onchainAmountSat
          ? _value.onchainAmountSat
          : onchainAmountSat // ignore: cast_nullable_to_non_nullable
              as int,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as ReverseSwapStatus,
    ));
  }
}

/// @nodoc

class _$ReverseSwapInfoImpl implements _ReverseSwapInfo {
  const _$ReverseSwapInfoImpl(
      {required this.id,
      required this.claimPubkey,
      this.lockupTxid,
      this.claimTxid,
      required this.onchainAmountSat,
      required this.status});

  @override
  final String id;
  @override
  final String claimPubkey;
  @override
  final String? lockupTxid;
  @override
  final String? claimTxid;
  @override
  final int onchainAmountSat;
  @override
  final ReverseSwapStatus status;

  @override
  String toString() {
    return 'ReverseSwapInfo(id: $id, claimPubkey: $claimPubkey, lockupTxid: $lockupTxid, claimTxid: $claimTxid, onchainAmountSat: $onchainAmountSat, status: $status)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ReverseSwapInfoImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.claimPubkey, claimPubkey) || other.claimPubkey == claimPubkey) &&
            (identical(other.lockupTxid, lockupTxid) || other.lockupTxid == lockupTxid) &&
            (identical(other.claimTxid, claimTxid) || other.claimTxid == claimTxid) &&
            (identical(other.onchainAmountSat, onchainAmountSat) ||
                other.onchainAmountSat == onchainAmountSat) &&
            (identical(other.status, status) || other.status == status));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, id, claimPubkey, lockupTxid, claimTxid, onchainAmountSat, status);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ReverseSwapInfoImplCopyWith<_$ReverseSwapInfoImpl> get copyWith =>
      __$$ReverseSwapInfoImplCopyWithImpl<_$ReverseSwapInfoImpl>(this, _$identity);
}

abstract class _ReverseSwapInfo implements ReverseSwapInfo {
  const factory _ReverseSwapInfo(
      {required final String id,
      required final String claimPubkey,
      final String? lockupTxid,
      final String? claimTxid,
      required final int onchainAmountSat,
      required final ReverseSwapStatus status}) = _$ReverseSwapInfoImpl;

  @override
  String get id;
  @override
  String get claimPubkey;
  @override
  String? get lockupTxid;
  @override
  String? get claimTxid;
  @override
  int get onchainAmountSat;
  @override
  ReverseSwapStatus get status;
  @override
  @JsonKey(ignore: true)
  _$$ReverseSwapInfoImplCopyWith<_$ReverseSwapInfoImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$RouteHint {
  List<RouteHintHop> get hops => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $RouteHintCopyWith<RouteHint> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RouteHintCopyWith<$Res> {
  factory $RouteHintCopyWith(RouteHint value, $Res Function(RouteHint) then) =
      _$RouteHintCopyWithImpl<$Res, RouteHint>;
  @useResult
  $Res call({List<RouteHintHop> hops});
}

/// @nodoc
class _$RouteHintCopyWithImpl<$Res, $Val extends RouteHint> implements $RouteHintCopyWith<$Res> {
  _$RouteHintCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? hops = null,
  }) {
    return _then(_value.copyWith(
      hops: null == hops
          ? _value.hops
          : hops // ignore: cast_nullable_to_non_nullable
              as List<RouteHintHop>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$RouteHintImplCopyWith<$Res> implements $RouteHintCopyWith<$Res> {
  factory _$$RouteHintImplCopyWith(_$RouteHintImpl value, $Res Function(_$RouteHintImpl) then) =
      __$$RouteHintImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<RouteHintHop> hops});
}

/// @nodoc
class __$$RouteHintImplCopyWithImpl<$Res> extends _$RouteHintCopyWithImpl<$Res, _$RouteHintImpl>
    implements _$$RouteHintImplCopyWith<$Res> {
  __$$RouteHintImplCopyWithImpl(_$RouteHintImpl _value, $Res Function(_$RouteHintImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? hops = null,
  }) {
    return _then(_$RouteHintImpl(
      hops: null == hops
          ? _value._hops
          : hops // ignore: cast_nullable_to_non_nullable
              as List<RouteHintHop>,
    ));
  }
}

/// @nodoc

class _$RouteHintImpl implements _RouteHint {
  const _$RouteHintImpl({required final List<RouteHintHop> hops}) : _hops = hops;

  final List<RouteHintHop> _hops;
  @override
  List<RouteHintHop> get hops {
    if (_hops is EqualUnmodifiableListView) return _hops;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_hops);
  }

  @override
  String toString() {
    return 'RouteHint(hops: $hops)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RouteHintImpl &&
            const DeepCollectionEquality().equals(other._hops, _hops));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(_hops));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RouteHintImplCopyWith<_$RouteHintImpl> get copyWith =>
      __$$RouteHintImplCopyWithImpl<_$RouteHintImpl>(this, _$identity);
}

abstract class _RouteHint implements RouteHint {
  const factory _RouteHint({required final List<RouteHintHop> hops}) = _$RouteHintImpl;

  @override
  List<RouteHintHop> get hops;
  @override
  @JsonKey(ignore: true)
  _$$RouteHintImplCopyWith<_$RouteHintImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$RouteHintHop {
  String get srcNodeId => throw _privateConstructorUsedError;
  int get shortChannelId => throw _privateConstructorUsedError;
  int get feesBaseMsat => throw _privateConstructorUsedError;
  int get feesProportionalMillionths => throw _privateConstructorUsedError;
  int get cltvExpiryDelta => throw _privateConstructorUsedError;
  int? get htlcMinimumMsat => throw _privateConstructorUsedError;
  int? get htlcMaximumMsat => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $RouteHintHopCopyWith<RouteHintHop> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RouteHintHopCopyWith<$Res> {
  factory $RouteHintHopCopyWith(RouteHintHop value, $Res Function(RouteHintHop) then) =
      _$RouteHintHopCopyWithImpl<$Res, RouteHintHop>;
  @useResult
  $Res call(
      {String srcNodeId,
      int shortChannelId,
      int feesBaseMsat,
      int feesProportionalMillionths,
      int cltvExpiryDelta,
      int? htlcMinimumMsat,
      int? htlcMaximumMsat});
}

/// @nodoc
class _$RouteHintHopCopyWithImpl<$Res, $Val extends RouteHintHop> implements $RouteHintHopCopyWith<$Res> {
  _$RouteHintHopCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? srcNodeId = null,
    Object? shortChannelId = null,
    Object? feesBaseMsat = null,
    Object? feesProportionalMillionths = null,
    Object? cltvExpiryDelta = null,
    Object? htlcMinimumMsat = freezed,
    Object? htlcMaximumMsat = freezed,
  }) {
    return _then(_value.copyWith(
      srcNodeId: null == srcNodeId
          ? _value.srcNodeId
          : srcNodeId // ignore: cast_nullable_to_non_nullable
              as String,
      shortChannelId: null == shortChannelId
          ? _value.shortChannelId
          : shortChannelId // ignore: cast_nullable_to_non_nullable
              as int,
      feesBaseMsat: null == feesBaseMsat
          ? _value.feesBaseMsat
          : feesBaseMsat // ignore: cast_nullable_to_non_nullable
              as int,
      feesProportionalMillionths: null == feesProportionalMillionths
          ? _value.feesProportionalMillionths
          : feesProportionalMillionths // ignore: cast_nullable_to_non_nullable
              as int,
      cltvExpiryDelta: null == cltvExpiryDelta
          ? _value.cltvExpiryDelta
          : cltvExpiryDelta // ignore: cast_nullable_to_non_nullable
              as int,
      htlcMinimumMsat: freezed == htlcMinimumMsat
          ? _value.htlcMinimumMsat
          : htlcMinimumMsat // ignore: cast_nullable_to_non_nullable
              as int?,
      htlcMaximumMsat: freezed == htlcMaximumMsat
          ? _value.htlcMaximumMsat
          : htlcMaximumMsat // ignore: cast_nullable_to_non_nullable
              as int?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$RouteHintHopImplCopyWith<$Res> implements $RouteHintHopCopyWith<$Res> {
  factory _$$RouteHintHopImplCopyWith(_$RouteHintHopImpl value, $Res Function(_$RouteHintHopImpl) then) =
      __$$RouteHintHopImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String srcNodeId,
      int shortChannelId,
      int feesBaseMsat,
      int feesProportionalMillionths,
      int cltvExpiryDelta,
      int? htlcMinimumMsat,
      int? htlcMaximumMsat});
}

/// @nodoc
class __$$RouteHintHopImplCopyWithImpl<$Res> extends _$RouteHintHopCopyWithImpl<$Res, _$RouteHintHopImpl>
    implements _$$RouteHintHopImplCopyWith<$Res> {
  __$$RouteHintHopImplCopyWithImpl(_$RouteHintHopImpl _value, $Res Function(_$RouteHintHopImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? srcNodeId = null,
    Object? shortChannelId = null,
    Object? feesBaseMsat = null,
    Object? feesProportionalMillionths = null,
    Object? cltvExpiryDelta = null,
    Object? htlcMinimumMsat = freezed,
    Object? htlcMaximumMsat = freezed,
  }) {
    return _then(_$RouteHintHopImpl(
      srcNodeId: null == srcNodeId
          ? _value.srcNodeId
          : srcNodeId // ignore: cast_nullable_to_non_nullable
              as String,
      shortChannelId: null == shortChannelId
          ? _value.shortChannelId
          : shortChannelId // ignore: cast_nullable_to_non_nullable
              as int,
      feesBaseMsat: null == feesBaseMsat
          ? _value.feesBaseMsat
          : feesBaseMsat // ignore: cast_nullable_to_non_nullable
              as int,
      feesProportionalMillionths: null == feesProportionalMillionths
          ? _value.feesProportionalMillionths
          : feesProportionalMillionths // ignore: cast_nullable_to_non_nullable
              as int,
      cltvExpiryDelta: null == cltvExpiryDelta
          ? _value.cltvExpiryDelta
          : cltvExpiryDelta // ignore: cast_nullable_to_non_nullable
              as int,
      htlcMinimumMsat: freezed == htlcMinimumMsat
          ? _value.htlcMinimumMsat
          : htlcMinimumMsat // ignore: cast_nullable_to_non_nullable
              as int?,
      htlcMaximumMsat: freezed == htlcMaximumMsat
          ? _value.htlcMaximumMsat
          : htlcMaximumMsat // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$RouteHintHopImpl implements _RouteHintHop {
  const _$RouteHintHopImpl(
      {required this.srcNodeId,
      required this.shortChannelId,
      required this.feesBaseMsat,
      required this.feesProportionalMillionths,
      required this.cltvExpiryDelta,
      this.htlcMinimumMsat,
      this.htlcMaximumMsat});

  @override
  final String srcNodeId;
  @override
  final int shortChannelId;
  @override
  final int feesBaseMsat;
  @override
  final int feesProportionalMillionths;
  @override
  final int cltvExpiryDelta;
  @override
  final int? htlcMinimumMsat;
  @override
  final int? htlcMaximumMsat;

  @override
  String toString() {
    return 'RouteHintHop(srcNodeId: $srcNodeId, shortChannelId: $shortChannelId, feesBaseMsat: $feesBaseMsat, feesProportionalMillionths: $feesProportionalMillionths, cltvExpiryDelta: $cltvExpiryDelta, htlcMinimumMsat: $htlcMinimumMsat, htlcMaximumMsat: $htlcMaximumMsat)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RouteHintHopImpl &&
            (identical(other.srcNodeId, srcNodeId) || other.srcNodeId == srcNodeId) &&
            (identical(other.shortChannelId, shortChannelId) || other.shortChannelId == shortChannelId) &&
            (identical(other.feesBaseMsat, feesBaseMsat) || other.feesBaseMsat == feesBaseMsat) &&
            (identical(other.feesProportionalMillionths, feesProportionalMillionths) ||
                other.feesProportionalMillionths == feesProportionalMillionths) &&
            (identical(other.cltvExpiryDelta, cltvExpiryDelta) || other.cltvExpiryDelta == cltvExpiryDelta) &&
            (identical(other.htlcMinimumMsat, htlcMinimumMsat) || other.htlcMinimumMsat == htlcMinimumMsat) &&
            (identical(other.htlcMaximumMsat, htlcMaximumMsat) || other.htlcMaximumMsat == htlcMaximumMsat));
  }

  @override
  int get hashCode => Object.hash(runtimeType, srcNodeId, shortChannelId, feesBaseMsat,
      feesProportionalMillionths, cltvExpiryDelta, htlcMinimumMsat, htlcMaximumMsat);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RouteHintHopImplCopyWith<_$RouteHintHopImpl> get copyWith =>
      __$$RouteHintHopImplCopyWithImpl<_$RouteHintHopImpl>(this, _$identity);
}

abstract class _RouteHintHop implements RouteHintHop {
  const factory _RouteHintHop(
      {required final String srcNodeId,
      required final int shortChannelId,
      required final int feesBaseMsat,
      required final int feesProportionalMillionths,
      required final int cltvExpiryDelta,
      final int? htlcMinimumMsat,
      final int? htlcMaximumMsat}) = _$RouteHintHopImpl;

  @override
  String get srcNodeId;
  @override
  int get shortChannelId;
  @override
  int get feesBaseMsat;
  @override
  int get feesProportionalMillionths;
  @override
  int get cltvExpiryDelta;
  @override
  int? get htlcMinimumMsat;
  @override
  int? get htlcMaximumMsat;
  @override
  @JsonKey(ignore: true)
  _$$RouteHintHopImplCopyWith<_$RouteHintHopImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SendOnchainResponse {
  ReverseSwapInfo get reverseSwapInfo => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $SendOnchainResponseCopyWith<SendOnchainResponse> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SendOnchainResponseCopyWith<$Res> {
  factory $SendOnchainResponseCopyWith(SendOnchainResponse value, $Res Function(SendOnchainResponse) then) =
      _$SendOnchainResponseCopyWithImpl<$Res, SendOnchainResponse>;
  @useResult
  $Res call({ReverseSwapInfo reverseSwapInfo});

  $ReverseSwapInfoCopyWith<$Res> get reverseSwapInfo;
}

/// @nodoc
class _$SendOnchainResponseCopyWithImpl<$Res, $Val extends SendOnchainResponse>
    implements $SendOnchainResponseCopyWith<$Res> {
  _$SendOnchainResponseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? reverseSwapInfo = null,
  }) {
    return _then(_value.copyWith(
      reverseSwapInfo: null == reverseSwapInfo
          ? _value.reverseSwapInfo
          : reverseSwapInfo // ignore: cast_nullable_to_non_nullable
              as ReverseSwapInfo,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $ReverseSwapInfoCopyWith<$Res> get reverseSwapInfo {
    return $ReverseSwapInfoCopyWith<$Res>(_value.reverseSwapInfo, (value) {
      return _then(_value.copyWith(reverseSwapInfo: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$SendOnchainResponseImplCopyWith<$Res> implements $SendOnchainResponseCopyWith<$Res> {
  factory _$$SendOnchainResponseImplCopyWith(
          _$SendOnchainResponseImpl value, $Res Function(_$SendOnchainResponseImpl) then) =
      __$$SendOnchainResponseImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({ReverseSwapInfo reverseSwapInfo});

  @override
  $ReverseSwapInfoCopyWith<$Res> get reverseSwapInfo;
}

/// @nodoc
class __$$SendOnchainResponseImplCopyWithImpl<$Res>
    extends _$SendOnchainResponseCopyWithImpl<$Res, _$SendOnchainResponseImpl>
    implements _$$SendOnchainResponseImplCopyWith<$Res> {
  __$$SendOnchainResponseImplCopyWithImpl(
      _$SendOnchainResponseImpl _value, $Res Function(_$SendOnchainResponseImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? reverseSwapInfo = null,
  }) {
    return _then(_$SendOnchainResponseImpl(
      reverseSwapInfo: null == reverseSwapInfo
          ? _value.reverseSwapInfo
          : reverseSwapInfo // ignore: cast_nullable_to_non_nullable
              as ReverseSwapInfo,
    ));
  }
}

/// @nodoc

class _$SendOnchainResponseImpl implements _SendOnchainResponse {
  const _$SendOnchainResponseImpl({required this.reverseSwapInfo});

  @override
  final ReverseSwapInfo reverseSwapInfo;

  @override
  String toString() {
    return 'SendOnchainResponse(reverseSwapInfo: $reverseSwapInfo)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SendOnchainResponseImpl &&
            (identical(other.reverseSwapInfo, reverseSwapInfo) || other.reverseSwapInfo == reverseSwapInfo));
  }

  @override
  int get hashCode => Object.hash(runtimeType, reverseSwapInfo);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SendOnchainResponseImplCopyWith<_$SendOnchainResponseImpl> get copyWith =>
      __$$SendOnchainResponseImplCopyWithImpl<_$SendOnchainResponseImpl>(this, _$identity);
}

abstract class _SendOnchainResponse implements SendOnchainResponse {
  const factory _SendOnchainResponse({required final ReverseSwapInfo reverseSwapInfo}) =
      _$SendOnchainResponseImpl;

  @override
  ReverseSwapInfo get reverseSwapInfo;
  @override
  @JsonKey(ignore: true)
  _$$SendOnchainResponseImplCopyWith<_$SendOnchainResponseImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SendPaymentResponse {
  Payment get payment => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $SendPaymentResponseCopyWith<SendPaymentResponse> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SendPaymentResponseCopyWith<$Res> {
  factory $SendPaymentResponseCopyWith(SendPaymentResponse value, $Res Function(SendPaymentResponse) then) =
      _$SendPaymentResponseCopyWithImpl<$Res, SendPaymentResponse>;
  @useResult
  $Res call({Payment payment});

  $PaymentCopyWith<$Res> get payment;
}

/// @nodoc
class _$SendPaymentResponseCopyWithImpl<$Res, $Val extends SendPaymentResponse>
    implements $SendPaymentResponseCopyWith<$Res> {
  _$SendPaymentResponseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? payment = null,
  }) {
    return _then(_value.copyWith(
      payment: null == payment
          ? _value.payment
          : payment // ignore: cast_nullable_to_non_nullable
              as Payment,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $PaymentCopyWith<$Res> get payment {
    return $PaymentCopyWith<$Res>(_value.payment, (value) {
      return _then(_value.copyWith(payment: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$SendPaymentResponseImplCopyWith<$Res> implements $SendPaymentResponseCopyWith<$Res> {
  factory _$$SendPaymentResponseImplCopyWith(
          _$SendPaymentResponseImpl value, $Res Function(_$SendPaymentResponseImpl) then) =
      __$$SendPaymentResponseImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({Payment payment});

  @override
  $PaymentCopyWith<$Res> get payment;
}

/// @nodoc
class __$$SendPaymentResponseImplCopyWithImpl<$Res>
    extends _$SendPaymentResponseCopyWithImpl<$Res, _$SendPaymentResponseImpl>
    implements _$$SendPaymentResponseImplCopyWith<$Res> {
  __$$SendPaymentResponseImplCopyWithImpl(
      _$SendPaymentResponseImpl _value, $Res Function(_$SendPaymentResponseImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? payment = null,
  }) {
    return _then(_$SendPaymentResponseImpl(
      payment: null == payment
          ? _value.payment
          : payment // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SendPaymentResponseImpl implements _SendPaymentResponse {
  const _$SendPaymentResponseImpl({required this.payment});

  @override
  final Payment payment;

  @override
  String toString() {
    return 'SendPaymentResponse(payment: $payment)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SendPaymentResponseImpl &&
            (identical(other.payment, payment) || other.payment == payment));
  }

  @override
  int get hashCode => Object.hash(runtimeType, payment);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SendPaymentResponseImplCopyWith<_$SendPaymentResponseImpl> get copyWith =>
      __$$SendPaymentResponseImplCopyWithImpl<_$SendPaymentResponseImpl>(this, _$identity);
}

abstract class _SendPaymentResponse implements SendPaymentResponse {
  const factory _SendPaymentResponse({required final Payment payment}) = _$SendPaymentResponseImpl;

  @override
  Payment get payment;
  @override
  @JsonKey(ignore: true)
  _$$SendPaymentResponseImplCopyWith<_$SendPaymentResponseImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SendSpontaneousPaymentRequest {
  String get nodeId => throw _privateConstructorUsedError;
  int get amountMsat => throw _privateConstructorUsedError;
  List<TlvEntry>? get extraTlvs => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $SendSpontaneousPaymentRequestCopyWith<SendSpontaneousPaymentRequest> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SendSpontaneousPaymentRequestCopyWith<$Res> {
  factory $SendSpontaneousPaymentRequestCopyWith(
          SendSpontaneousPaymentRequest value, $Res Function(SendSpontaneousPaymentRequest) then) =
      _$SendSpontaneousPaymentRequestCopyWithImpl<$Res, SendSpontaneousPaymentRequest>;
  @useResult
  $Res call({String nodeId, int amountMsat, List<TlvEntry>? extraTlvs});
}

/// @nodoc
class _$SendSpontaneousPaymentRequestCopyWithImpl<$Res, $Val extends SendSpontaneousPaymentRequest>
    implements $SendSpontaneousPaymentRequestCopyWith<$Res> {
  _$SendSpontaneousPaymentRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? nodeId = null,
    Object? amountMsat = null,
    Object? extraTlvs = freezed,
  }) {
    return _then(_value.copyWith(
      nodeId: null == nodeId
          ? _value.nodeId
          : nodeId // ignore: cast_nullable_to_non_nullable
              as String,
      amountMsat: null == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      extraTlvs: freezed == extraTlvs
          ? _value.extraTlvs
          : extraTlvs // ignore: cast_nullable_to_non_nullable
              as List<TlvEntry>?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$SendSpontaneousPaymentRequestImplCopyWith<$Res>
    implements $SendSpontaneousPaymentRequestCopyWith<$Res> {
  factory _$$SendSpontaneousPaymentRequestImplCopyWith(_$SendSpontaneousPaymentRequestImpl value,
          $Res Function(_$SendSpontaneousPaymentRequestImpl) then) =
      __$$SendSpontaneousPaymentRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String nodeId, int amountMsat, List<TlvEntry>? extraTlvs});
}

/// @nodoc
class __$$SendSpontaneousPaymentRequestImplCopyWithImpl<$Res>
    extends _$SendSpontaneousPaymentRequestCopyWithImpl<$Res, _$SendSpontaneousPaymentRequestImpl>
    implements _$$SendSpontaneousPaymentRequestImplCopyWith<$Res> {
  __$$SendSpontaneousPaymentRequestImplCopyWithImpl(
      _$SendSpontaneousPaymentRequestImpl _value, $Res Function(_$SendSpontaneousPaymentRequestImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? nodeId = null,
    Object? amountMsat = null,
    Object? extraTlvs = freezed,
  }) {
    return _then(_$SendSpontaneousPaymentRequestImpl(
      nodeId: null == nodeId
          ? _value.nodeId
          : nodeId // ignore: cast_nullable_to_non_nullable
              as String,
      amountMsat: null == amountMsat
          ? _value.amountMsat
          : amountMsat // ignore: cast_nullable_to_non_nullable
              as int,
      extraTlvs: freezed == extraTlvs
          ? _value._extraTlvs
          : extraTlvs // ignore: cast_nullable_to_non_nullable
              as List<TlvEntry>?,
    ));
  }
}

/// @nodoc

class _$SendSpontaneousPaymentRequestImpl implements _SendSpontaneousPaymentRequest {
  const _$SendSpontaneousPaymentRequestImpl(
      {required this.nodeId, required this.amountMsat, final List<TlvEntry>? extraTlvs})
      : _extraTlvs = extraTlvs;

  @override
  final String nodeId;
  @override
  final int amountMsat;
  final List<TlvEntry>? _extraTlvs;
  @override
  List<TlvEntry>? get extraTlvs {
    final value = _extraTlvs;
    if (value == null) return null;
    if (_extraTlvs is EqualUnmodifiableListView) return _extraTlvs;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  @override
  String toString() {
    return 'SendSpontaneousPaymentRequest(nodeId: $nodeId, amountMsat: $amountMsat, extraTlvs: $extraTlvs)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SendSpontaneousPaymentRequestImpl &&
            (identical(other.nodeId, nodeId) || other.nodeId == nodeId) &&
            (identical(other.amountMsat, amountMsat) || other.amountMsat == amountMsat) &&
            const DeepCollectionEquality().equals(other._extraTlvs, _extraTlvs));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, nodeId, amountMsat, const DeepCollectionEquality().hash(_extraTlvs));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SendSpontaneousPaymentRequestImplCopyWith<_$SendSpontaneousPaymentRequestImpl> get copyWith =>
      __$$SendSpontaneousPaymentRequestImplCopyWithImpl<_$SendSpontaneousPaymentRequestImpl>(
          this, _$identity);
}

abstract class _SendSpontaneousPaymentRequest implements SendSpontaneousPaymentRequest {
  const factory _SendSpontaneousPaymentRequest(
      {required final String nodeId,
      required final int amountMsat,
      final List<TlvEntry>? extraTlvs}) = _$SendSpontaneousPaymentRequestImpl;

  @override
  String get nodeId;
  @override
  int get amountMsat;
  @override
  List<TlvEntry>? get extraTlvs;
  @override
  @JsonKey(ignore: true)
  _$$SendSpontaneousPaymentRequestImplCopyWith<_$SendSpontaneousPaymentRequestImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$ServiceHealthCheckResponse {
  HealthCheckStatus get status => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ServiceHealthCheckResponseCopyWith<ServiceHealthCheckResponse> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ServiceHealthCheckResponseCopyWith<$Res> {
  factory $ServiceHealthCheckResponseCopyWith(
          ServiceHealthCheckResponse value, $Res Function(ServiceHealthCheckResponse) then) =
      _$ServiceHealthCheckResponseCopyWithImpl<$Res, ServiceHealthCheckResponse>;
  @useResult
  $Res call({HealthCheckStatus status});
}

/// @nodoc
class _$ServiceHealthCheckResponseCopyWithImpl<$Res, $Val extends ServiceHealthCheckResponse>
    implements $ServiceHealthCheckResponseCopyWith<$Res> {
  _$ServiceHealthCheckResponseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? status = null,
  }) {
    return _then(_value.copyWith(
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as HealthCheckStatus,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ServiceHealthCheckResponseImplCopyWith<$Res>
    implements $ServiceHealthCheckResponseCopyWith<$Res> {
  factory _$$ServiceHealthCheckResponseImplCopyWith(
          _$ServiceHealthCheckResponseImpl value, $Res Function(_$ServiceHealthCheckResponseImpl) then) =
      __$$ServiceHealthCheckResponseImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({HealthCheckStatus status});
}

/// @nodoc
class __$$ServiceHealthCheckResponseImplCopyWithImpl<$Res>
    extends _$ServiceHealthCheckResponseCopyWithImpl<$Res, _$ServiceHealthCheckResponseImpl>
    implements _$$ServiceHealthCheckResponseImplCopyWith<$Res> {
  __$$ServiceHealthCheckResponseImplCopyWithImpl(
      _$ServiceHealthCheckResponseImpl _value, $Res Function(_$ServiceHealthCheckResponseImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? status = null,
  }) {
    return _then(_$ServiceHealthCheckResponseImpl(
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as HealthCheckStatus,
    ));
  }
}

/// @nodoc

class _$ServiceHealthCheckResponseImpl implements _ServiceHealthCheckResponse {
  const _$ServiceHealthCheckResponseImpl({required this.status});

  @override
  final HealthCheckStatus status;

  @override
  String toString() {
    return 'ServiceHealthCheckResponse(status: $status)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ServiceHealthCheckResponseImpl &&
            (identical(other.status, status) || other.status == status));
  }

  @override
  int get hashCode => Object.hash(runtimeType, status);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ServiceHealthCheckResponseImplCopyWith<_$ServiceHealthCheckResponseImpl> get copyWith =>
      __$$ServiceHealthCheckResponseImplCopyWithImpl<_$ServiceHealthCheckResponseImpl>(this, _$identity);
}

abstract class _ServiceHealthCheckResponse implements ServiceHealthCheckResponse {
  const factory _ServiceHealthCheckResponse({required final HealthCheckStatus status}) =
      _$ServiceHealthCheckResponseImpl;

  @override
  HealthCheckStatus get status;
  @override
  @JsonKey(ignore: true)
  _$$ServiceHealthCheckResponseImplCopyWith<_$ServiceHealthCheckResponseImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SuccessActionProcessed {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataResult result) aes,
    required TResult Function(MessageSuccessActionData data) message,
    required TResult Function(UrlSuccessActionData data) url,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataResult result)? aes,
    TResult? Function(MessageSuccessActionData data)? message,
    TResult? Function(UrlSuccessActionData data)? url,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(AesSuccessActionDataResult result)? aes,
    TResult Function(MessageSuccessActionData data)? message,
    TResult Function(UrlSuccessActionData data)? url,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SuccessActionProcessed_Aes value) aes,
    required TResult Function(SuccessActionProcessed_Message value) message,
    required TResult Function(SuccessActionProcessed_Url value) url,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SuccessActionProcessed_Aes value)? aes,
    TResult? Function(SuccessActionProcessed_Message value)? message,
    TResult? Function(SuccessActionProcessed_Url value)? url,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SuccessActionProcessed_Aes value)? aes,
    TResult Function(SuccessActionProcessed_Message value)? message,
    TResult Function(SuccessActionProcessed_Url value)? url,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SuccessActionProcessedCopyWith<$Res> {
  factory $SuccessActionProcessedCopyWith(
          SuccessActionProcessed value, $Res Function(SuccessActionProcessed) then) =
      _$SuccessActionProcessedCopyWithImpl<$Res, SuccessActionProcessed>;
}

/// @nodoc
class _$SuccessActionProcessedCopyWithImpl<$Res, $Val extends SuccessActionProcessed>
    implements $SuccessActionProcessedCopyWith<$Res> {
  _$SuccessActionProcessedCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$SuccessActionProcessed_AesImplCopyWith<$Res> {
  factory _$$SuccessActionProcessed_AesImplCopyWith(
          _$SuccessActionProcessed_AesImpl value, $Res Function(_$SuccessActionProcessed_AesImpl) then) =
      __$$SuccessActionProcessed_AesImplCopyWithImpl<$Res>;
  @useResult
  $Res call({AesSuccessActionDataResult result});

  $AesSuccessActionDataResultCopyWith<$Res> get result;
}

/// @nodoc
class __$$SuccessActionProcessed_AesImplCopyWithImpl<$Res>
    extends _$SuccessActionProcessedCopyWithImpl<$Res, _$SuccessActionProcessed_AesImpl>
    implements _$$SuccessActionProcessed_AesImplCopyWith<$Res> {
  __$$SuccessActionProcessed_AesImplCopyWithImpl(
      _$SuccessActionProcessed_AesImpl _value, $Res Function(_$SuccessActionProcessed_AesImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? result = null,
  }) {
    return _then(_$SuccessActionProcessed_AesImpl(
      result: null == result
          ? _value.result
          : result // ignore: cast_nullable_to_non_nullable
              as AesSuccessActionDataResult,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $AesSuccessActionDataResultCopyWith<$Res> get result {
    return $AesSuccessActionDataResultCopyWith<$Res>(_value.result, (value) {
      return _then(_value.copyWith(result: value));
    });
  }
}

/// @nodoc

class _$SuccessActionProcessed_AesImpl implements SuccessActionProcessed_Aes {
  const _$SuccessActionProcessed_AesImpl({required this.result});

  @override
  final AesSuccessActionDataResult result;

  @override
  String toString() {
    return 'SuccessActionProcessed.aes(result: $result)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SuccessActionProcessed_AesImpl &&
            (identical(other.result, result) || other.result == result));
  }

  @override
  int get hashCode => Object.hash(runtimeType, result);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SuccessActionProcessed_AesImplCopyWith<_$SuccessActionProcessed_AesImpl> get copyWith =>
      __$$SuccessActionProcessed_AesImplCopyWithImpl<_$SuccessActionProcessed_AesImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataResult result) aes,
    required TResult Function(MessageSuccessActionData data) message,
    required TResult Function(UrlSuccessActionData data) url,
  }) {
    return aes(result);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataResult result)? aes,
    TResult? Function(MessageSuccessActionData data)? message,
    TResult? Function(UrlSuccessActionData data)? url,
  }) {
    return aes?.call(result);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(AesSuccessActionDataResult result)? aes,
    TResult Function(MessageSuccessActionData data)? message,
    TResult Function(UrlSuccessActionData data)? url,
    required TResult orElse(),
  }) {
    if (aes != null) {
      return aes(result);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SuccessActionProcessed_Aes value) aes,
    required TResult Function(SuccessActionProcessed_Message value) message,
    required TResult Function(SuccessActionProcessed_Url value) url,
  }) {
    return aes(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SuccessActionProcessed_Aes value)? aes,
    TResult? Function(SuccessActionProcessed_Message value)? message,
    TResult? Function(SuccessActionProcessed_Url value)? url,
  }) {
    return aes?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SuccessActionProcessed_Aes value)? aes,
    TResult Function(SuccessActionProcessed_Message value)? message,
    TResult Function(SuccessActionProcessed_Url value)? url,
    required TResult orElse(),
  }) {
    if (aes != null) {
      return aes(this);
    }
    return orElse();
  }
}

abstract class SuccessActionProcessed_Aes implements SuccessActionProcessed {
  const factory SuccessActionProcessed_Aes({required final AesSuccessActionDataResult result}) =
      _$SuccessActionProcessed_AesImpl;

  AesSuccessActionDataResult get result;
  @JsonKey(ignore: true)
  _$$SuccessActionProcessed_AesImplCopyWith<_$SuccessActionProcessed_AesImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SuccessActionProcessed_MessageImplCopyWith<$Res> {
  factory _$$SuccessActionProcessed_MessageImplCopyWith(_$SuccessActionProcessed_MessageImpl value,
          $Res Function(_$SuccessActionProcessed_MessageImpl) then) =
      __$$SuccessActionProcessed_MessageImplCopyWithImpl<$Res>;
  @useResult
  $Res call({MessageSuccessActionData data});
}

/// @nodoc
class __$$SuccessActionProcessed_MessageImplCopyWithImpl<$Res>
    extends _$SuccessActionProcessedCopyWithImpl<$Res, _$SuccessActionProcessed_MessageImpl>
    implements _$$SuccessActionProcessed_MessageImplCopyWith<$Res> {
  __$$SuccessActionProcessed_MessageImplCopyWithImpl(
      _$SuccessActionProcessed_MessageImpl _value, $Res Function(_$SuccessActionProcessed_MessageImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$SuccessActionProcessed_MessageImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as MessageSuccessActionData,
    ));
  }
}

/// @nodoc

class _$SuccessActionProcessed_MessageImpl implements SuccessActionProcessed_Message {
  const _$SuccessActionProcessed_MessageImpl({required this.data});

  @override
  final MessageSuccessActionData data;

  @override
  String toString() {
    return 'SuccessActionProcessed.message(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SuccessActionProcessed_MessageImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SuccessActionProcessed_MessageImplCopyWith<_$SuccessActionProcessed_MessageImpl> get copyWith =>
      __$$SuccessActionProcessed_MessageImplCopyWithImpl<_$SuccessActionProcessed_MessageImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataResult result) aes,
    required TResult Function(MessageSuccessActionData data) message,
    required TResult Function(UrlSuccessActionData data) url,
  }) {
    return message(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataResult result)? aes,
    TResult? Function(MessageSuccessActionData data)? message,
    TResult? Function(UrlSuccessActionData data)? url,
  }) {
    return message?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(AesSuccessActionDataResult result)? aes,
    TResult Function(MessageSuccessActionData data)? message,
    TResult Function(UrlSuccessActionData data)? url,
    required TResult orElse(),
  }) {
    if (message != null) {
      return message(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SuccessActionProcessed_Aes value) aes,
    required TResult Function(SuccessActionProcessed_Message value) message,
    required TResult Function(SuccessActionProcessed_Url value) url,
  }) {
    return message(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SuccessActionProcessed_Aes value)? aes,
    TResult? Function(SuccessActionProcessed_Message value)? message,
    TResult? Function(SuccessActionProcessed_Url value)? url,
  }) {
    return message?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SuccessActionProcessed_Aes value)? aes,
    TResult Function(SuccessActionProcessed_Message value)? message,
    TResult Function(SuccessActionProcessed_Url value)? url,
    required TResult orElse(),
  }) {
    if (message != null) {
      return message(this);
    }
    return orElse();
  }
}

abstract class SuccessActionProcessed_Message implements SuccessActionProcessed {
  const factory SuccessActionProcessed_Message({required final MessageSuccessActionData data}) =
      _$SuccessActionProcessed_MessageImpl;

  MessageSuccessActionData get data;
  @JsonKey(ignore: true)
  _$$SuccessActionProcessed_MessageImplCopyWith<_$SuccessActionProcessed_MessageImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SuccessActionProcessed_UrlImplCopyWith<$Res> {
  factory _$$SuccessActionProcessed_UrlImplCopyWith(
          _$SuccessActionProcessed_UrlImpl value, $Res Function(_$SuccessActionProcessed_UrlImpl) then) =
      __$$SuccessActionProcessed_UrlImplCopyWithImpl<$Res>;
  @useResult
  $Res call({UrlSuccessActionData data});
}

/// @nodoc
class __$$SuccessActionProcessed_UrlImplCopyWithImpl<$Res>
    extends _$SuccessActionProcessedCopyWithImpl<$Res, _$SuccessActionProcessed_UrlImpl>
    implements _$$SuccessActionProcessed_UrlImplCopyWith<$Res> {
  __$$SuccessActionProcessed_UrlImplCopyWithImpl(
      _$SuccessActionProcessed_UrlImpl _value, $Res Function(_$SuccessActionProcessed_UrlImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$SuccessActionProcessed_UrlImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as UrlSuccessActionData,
    ));
  }
}

/// @nodoc

class _$SuccessActionProcessed_UrlImpl implements SuccessActionProcessed_Url {
  const _$SuccessActionProcessed_UrlImpl({required this.data});

  @override
  final UrlSuccessActionData data;

  @override
  String toString() {
    return 'SuccessActionProcessed.url(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SuccessActionProcessed_UrlImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SuccessActionProcessed_UrlImplCopyWith<_$SuccessActionProcessed_UrlImpl> get copyWith =>
      __$$SuccessActionProcessed_UrlImplCopyWithImpl<_$SuccessActionProcessed_UrlImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataResult result) aes,
    required TResult Function(MessageSuccessActionData data) message,
    required TResult Function(UrlSuccessActionData data) url,
  }) {
    return url(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataResult result)? aes,
    TResult? Function(MessageSuccessActionData data)? message,
    TResult? Function(UrlSuccessActionData data)? url,
  }) {
    return url?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(AesSuccessActionDataResult result)? aes,
    TResult Function(MessageSuccessActionData data)? message,
    TResult Function(UrlSuccessActionData data)? url,
    required TResult orElse(),
  }) {
    if (url != null) {
      return url(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SuccessActionProcessed_Aes value) aes,
    required TResult Function(SuccessActionProcessed_Message value) message,
    required TResult Function(SuccessActionProcessed_Url value) url,
  }) {
    return url(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SuccessActionProcessed_Aes value)? aes,
    TResult? Function(SuccessActionProcessed_Message value)? message,
    TResult? Function(SuccessActionProcessed_Url value)? url,
  }) {
    return url?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SuccessActionProcessed_Aes value)? aes,
    TResult Function(SuccessActionProcessed_Message value)? message,
    TResult Function(SuccessActionProcessed_Url value)? url,
    required TResult orElse(),
  }) {
    if (url != null) {
      return url(this);
    }
    return orElse();
  }
}

abstract class SuccessActionProcessed_Url implements SuccessActionProcessed {
  const factory SuccessActionProcessed_Url({required final UrlSuccessActionData data}) =
      _$SuccessActionProcessed_UrlImpl;

  UrlSuccessActionData get data;
  @JsonKey(ignore: true)
  _$$SuccessActionProcessed_UrlImplCopyWith<_$SuccessActionProcessed_UrlImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SwapInfo {
  String get bitcoinAddress => throw _privateConstructorUsedError;
  int get createdAt => throw _privateConstructorUsedError;
  int get lockHeight => throw _privateConstructorUsedError;
  Uint8List get paymentHash => throw _privateConstructorUsedError;
  Uint8List get preimage => throw _privateConstructorUsedError;
  Uint8List get privateKey => throw _privateConstructorUsedError;
  Uint8List get publicKey => throw _privateConstructorUsedError;
  Uint8List get swapperPublicKey => throw _privateConstructorUsedError;
  Uint8List get script => throw _privateConstructorUsedError;
  String? get bolt11 => throw _privateConstructorUsedError;
  int get paidMsat => throw _privateConstructorUsedError;
  int get totalIncomingTxs => throw _privateConstructorUsedError;
  int get confirmedSats => throw _privateConstructorUsedError;
  int get unconfirmedSats => throw _privateConstructorUsedError;
  SwapStatus get status => throw _privateConstructorUsedError;
  List<String> get refundTxIds => throw _privateConstructorUsedError;
  List<String> get unconfirmedTxIds => throw _privateConstructorUsedError;
  List<String> get confirmedTxIds => throw _privateConstructorUsedError;
  int get minAllowedDeposit => throw _privateConstructorUsedError;
  int get maxAllowedDeposit => throw _privateConstructorUsedError;
  String? get lastRedeemError => throw _privateConstructorUsedError;
  OpeningFeeParams? get channelOpeningFees => throw _privateConstructorUsedError;
  int? get confirmedAt => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $SwapInfoCopyWith<SwapInfo> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SwapInfoCopyWith<$Res> {
  factory $SwapInfoCopyWith(SwapInfo value, $Res Function(SwapInfo) then) =
      _$SwapInfoCopyWithImpl<$Res, SwapInfo>;
  @useResult
  $Res call(
      {String bitcoinAddress,
      int createdAt,
      int lockHeight,
      Uint8List paymentHash,
      Uint8List preimage,
      Uint8List privateKey,
      Uint8List publicKey,
      Uint8List swapperPublicKey,
      Uint8List script,
      String? bolt11,
      int paidMsat,
      int totalIncomingTxs,
      int confirmedSats,
      int unconfirmedSats,
      SwapStatus status,
      List<String> refundTxIds,
      List<String> unconfirmedTxIds,
      List<String> confirmedTxIds,
      int minAllowedDeposit,
      int maxAllowedDeposit,
      String? lastRedeemError,
      OpeningFeeParams? channelOpeningFees,
      int? confirmedAt});

  $OpeningFeeParamsCopyWith<$Res>? get channelOpeningFees;
}

/// @nodoc
class _$SwapInfoCopyWithImpl<$Res, $Val extends SwapInfo> implements $SwapInfoCopyWith<$Res> {
  _$SwapInfoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? bitcoinAddress = null,
    Object? createdAt = null,
    Object? lockHeight = null,
    Object? paymentHash = null,
    Object? preimage = null,
    Object? privateKey = null,
    Object? publicKey = null,
    Object? swapperPublicKey = null,
    Object? script = null,
    Object? bolt11 = freezed,
    Object? paidMsat = null,
    Object? totalIncomingTxs = null,
    Object? confirmedSats = null,
    Object? unconfirmedSats = null,
    Object? status = null,
    Object? refundTxIds = null,
    Object? unconfirmedTxIds = null,
    Object? confirmedTxIds = null,
    Object? minAllowedDeposit = null,
    Object? maxAllowedDeposit = null,
    Object? lastRedeemError = freezed,
    Object? channelOpeningFees = freezed,
    Object? confirmedAt = freezed,
  }) {
    return _then(_value.copyWith(
      bitcoinAddress: null == bitcoinAddress
          ? _value.bitcoinAddress
          : bitcoinAddress // ignore: cast_nullable_to_non_nullable
              as String,
      createdAt: null == createdAt
          ? _value.createdAt
          : createdAt // ignore: cast_nullable_to_non_nullable
              as int,
      lockHeight: null == lockHeight
          ? _value.lockHeight
          : lockHeight // ignore: cast_nullable_to_non_nullable
              as int,
      paymentHash: null == paymentHash
          ? _value.paymentHash
          : paymentHash // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      preimage: null == preimage
          ? _value.preimage
          : preimage // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      privateKey: null == privateKey
          ? _value.privateKey
          : privateKey // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      publicKey: null == publicKey
          ? _value.publicKey
          : publicKey // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      swapperPublicKey: null == swapperPublicKey
          ? _value.swapperPublicKey
          : swapperPublicKey // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      script: null == script
          ? _value.script
          : script // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      bolt11: freezed == bolt11
          ? _value.bolt11
          : bolt11 // ignore: cast_nullable_to_non_nullable
              as String?,
      paidMsat: null == paidMsat
          ? _value.paidMsat
          : paidMsat // ignore: cast_nullable_to_non_nullable
              as int,
      totalIncomingTxs: null == totalIncomingTxs
          ? _value.totalIncomingTxs
          : totalIncomingTxs // ignore: cast_nullable_to_non_nullable
              as int,
      confirmedSats: null == confirmedSats
          ? _value.confirmedSats
          : confirmedSats // ignore: cast_nullable_to_non_nullable
              as int,
      unconfirmedSats: null == unconfirmedSats
          ? _value.unconfirmedSats
          : unconfirmedSats // ignore: cast_nullable_to_non_nullable
              as int,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as SwapStatus,
      refundTxIds: null == refundTxIds
          ? _value.refundTxIds
          : refundTxIds // ignore: cast_nullable_to_non_nullable
              as List<String>,
      unconfirmedTxIds: null == unconfirmedTxIds
          ? _value.unconfirmedTxIds
          : unconfirmedTxIds // ignore: cast_nullable_to_non_nullable
              as List<String>,
      confirmedTxIds: null == confirmedTxIds
          ? _value.confirmedTxIds
          : confirmedTxIds // ignore: cast_nullable_to_non_nullable
              as List<String>,
      minAllowedDeposit: null == minAllowedDeposit
          ? _value.minAllowedDeposit
          : minAllowedDeposit // ignore: cast_nullable_to_non_nullable
              as int,
      maxAllowedDeposit: null == maxAllowedDeposit
          ? _value.maxAllowedDeposit
          : maxAllowedDeposit // ignore: cast_nullable_to_non_nullable
              as int,
      lastRedeemError: freezed == lastRedeemError
          ? _value.lastRedeemError
          : lastRedeemError // ignore: cast_nullable_to_non_nullable
              as String?,
      channelOpeningFees: freezed == channelOpeningFees
          ? _value.channelOpeningFees
          : channelOpeningFees // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
      confirmedAt: freezed == confirmedAt
          ? _value.confirmedAt
          : confirmedAt // ignore: cast_nullable_to_non_nullable
              as int?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $OpeningFeeParamsCopyWith<$Res>? get channelOpeningFees {
    if (_value.channelOpeningFees == null) {
      return null;
    }

    return $OpeningFeeParamsCopyWith<$Res>(_value.channelOpeningFees!, (value) {
      return _then(_value.copyWith(channelOpeningFees: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$SwapInfoImplCopyWith<$Res> implements $SwapInfoCopyWith<$Res> {
  factory _$$SwapInfoImplCopyWith(_$SwapInfoImpl value, $Res Function(_$SwapInfoImpl) then) =
      __$$SwapInfoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String bitcoinAddress,
      int createdAt,
      int lockHeight,
      Uint8List paymentHash,
      Uint8List preimage,
      Uint8List privateKey,
      Uint8List publicKey,
      Uint8List swapperPublicKey,
      Uint8List script,
      String? bolt11,
      int paidMsat,
      int totalIncomingTxs,
      int confirmedSats,
      int unconfirmedSats,
      SwapStatus status,
      List<String> refundTxIds,
      List<String> unconfirmedTxIds,
      List<String> confirmedTxIds,
      int minAllowedDeposit,
      int maxAllowedDeposit,
      String? lastRedeemError,
      OpeningFeeParams? channelOpeningFees,
      int? confirmedAt});

  @override
  $OpeningFeeParamsCopyWith<$Res>? get channelOpeningFees;
}

/// @nodoc
class __$$SwapInfoImplCopyWithImpl<$Res> extends _$SwapInfoCopyWithImpl<$Res, _$SwapInfoImpl>
    implements _$$SwapInfoImplCopyWith<$Res> {
  __$$SwapInfoImplCopyWithImpl(_$SwapInfoImpl _value, $Res Function(_$SwapInfoImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? bitcoinAddress = null,
    Object? createdAt = null,
    Object? lockHeight = null,
    Object? paymentHash = null,
    Object? preimage = null,
    Object? privateKey = null,
    Object? publicKey = null,
    Object? swapperPublicKey = null,
    Object? script = null,
    Object? bolt11 = freezed,
    Object? paidMsat = null,
    Object? totalIncomingTxs = null,
    Object? confirmedSats = null,
    Object? unconfirmedSats = null,
    Object? status = null,
    Object? refundTxIds = null,
    Object? unconfirmedTxIds = null,
    Object? confirmedTxIds = null,
    Object? minAllowedDeposit = null,
    Object? maxAllowedDeposit = null,
    Object? lastRedeemError = freezed,
    Object? channelOpeningFees = freezed,
    Object? confirmedAt = freezed,
  }) {
    return _then(_$SwapInfoImpl(
      bitcoinAddress: null == bitcoinAddress
          ? _value.bitcoinAddress
          : bitcoinAddress // ignore: cast_nullable_to_non_nullable
              as String,
      createdAt: null == createdAt
          ? _value.createdAt
          : createdAt // ignore: cast_nullable_to_non_nullable
              as int,
      lockHeight: null == lockHeight
          ? _value.lockHeight
          : lockHeight // ignore: cast_nullable_to_non_nullable
              as int,
      paymentHash: null == paymentHash
          ? _value.paymentHash
          : paymentHash // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      preimage: null == preimage
          ? _value.preimage
          : preimage // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      privateKey: null == privateKey
          ? _value.privateKey
          : privateKey // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      publicKey: null == publicKey
          ? _value.publicKey
          : publicKey // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      swapperPublicKey: null == swapperPublicKey
          ? _value.swapperPublicKey
          : swapperPublicKey // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      script: null == script
          ? _value.script
          : script // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      bolt11: freezed == bolt11
          ? _value.bolt11
          : bolt11 // ignore: cast_nullable_to_non_nullable
              as String?,
      paidMsat: null == paidMsat
          ? _value.paidMsat
          : paidMsat // ignore: cast_nullable_to_non_nullable
              as int,
      totalIncomingTxs: null == totalIncomingTxs
          ? _value.totalIncomingTxs
          : totalIncomingTxs // ignore: cast_nullable_to_non_nullable
              as int,
      confirmedSats: null == confirmedSats
          ? _value.confirmedSats
          : confirmedSats // ignore: cast_nullable_to_non_nullable
              as int,
      unconfirmedSats: null == unconfirmedSats
          ? _value.unconfirmedSats
          : unconfirmedSats // ignore: cast_nullable_to_non_nullable
              as int,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as SwapStatus,
      refundTxIds: null == refundTxIds
          ? _value._refundTxIds
          : refundTxIds // ignore: cast_nullable_to_non_nullable
              as List<String>,
      unconfirmedTxIds: null == unconfirmedTxIds
          ? _value._unconfirmedTxIds
          : unconfirmedTxIds // ignore: cast_nullable_to_non_nullable
              as List<String>,
      confirmedTxIds: null == confirmedTxIds
          ? _value._confirmedTxIds
          : confirmedTxIds // ignore: cast_nullable_to_non_nullable
              as List<String>,
      minAllowedDeposit: null == minAllowedDeposit
          ? _value.minAllowedDeposit
          : minAllowedDeposit // ignore: cast_nullable_to_non_nullable
              as int,
      maxAllowedDeposit: null == maxAllowedDeposit
          ? _value.maxAllowedDeposit
          : maxAllowedDeposit // ignore: cast_nullable_to_non_nullable
              as int,
      lastRedeemError: freezed == lastRedeemError
          ? _value.lastRedeemError
          : lastRedeemError // ignore: cast_nullable_to_non_nullable
              as String?,
      channelOpeningFees: freezed == channelOpeningFees
          ? _value.channelOpeningFees
          : channelOpeningFees // ignore: cast_nullable_to_non_nullable
              as OpeningFeeParams?,
      confirmedAt: freezed == confirmedAt
          ? _value.confirmedAt
          : confirmedAt // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$SwapInfoImpl implements _SwapInfo {
  const _$SwapInfoImpl(
      {required this.bitcoinAddress,
      required this.createdAt,
      required this.lockHeight,
      required this.paymentHash,
      required this.preimage,
      required this.privateKey,
      required this.publicKey,
      required this.swapperPublicKey,
      required this.script,
      this.bolt11,
      required this.paidMsat,
      required this.totalIncomingTxs,
      required this.confirmedSats,
      required this.unconfirmedSats,
      required this.status,
      required final List<String> refundTxIds,
      required final List<String> unconfirmedTxIds,
      required final List<String> confirmedTxIds,
      required this.minAllowedDeposit,
      required this.maxAllowedDeposit,
      this.lastRedeemError,
      this.channelOpeningFees,
      this.confirmedAt})
      : _refundTxIds = refundTxIds,
        _unconfirmedTxIds = unconfirmedTxIds,
        _confirmedTxIds = confirmedTxIds;

  @override
  final String bitcoinAddress;
  @override
  final int createdAt;
  @override
  final int lockHeight;
  @override
  final Uint8List paymentHash;
  @override
  final Uint8List preimage;
  @override
  final Uint8List privateKey;
  @override
  final Uint8List publicKey;
  @override
  final Uint8List swapperPublicKey;
  @override
  final Uint8List script;
  @override
  final String? bolt11;
  @override
  final int paidMsat;
  @override
  final int totalIncomingTxs;
  @override
  final int confirmedSats;
  @override
  final int unconfirmedSats;
  @override
  final SwapStatus status;
  final List<String> _refundTxIds;
  @override
  List<String> get refundTxIds {
    if (_refundTxIds is EqualUnmodifiableListView) return _refundTxIds;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_refundTxIds);
  }

  final List<String> _unconfirmedTxIds;
  @override
  List<String> get unconfirmedTxIds {
    if (_unconfirmedTxIds is EqualUnmodifiableListView) return _unconfirmedTxIds;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_unconfirmedTxIds);
  }

  final List<String> _confirmedTxIds;
  @override
  List<String> get confirmedTxIds {
    if (_confirmedTxIds is EqualUnmodifiableListView) return _confirmedTxIds;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_confirmedTxIds);
  }

  @override
  final int minAllowedDeposit;
  @override
  final int maxAllowedDeposit;
  @override
  final String? lastRedeemError;
  @override
  final OpeningFeeParams? channelOpeningFees;
  @override
  final int? confirmedAt;

  @override
  String toString() {
    return 'SwapInfo(bitcoinAddress: $bitcoinAddress, createdAt: $createdAt, lockHeight: $lockHeight, paymentHash: $paymentHash, preimage: $preimage, privateKey: $privateKey, publicKey: $publicKey, swapperPublicKey: $swapperPublicKey, script: $script, bolt11: $bolt11, paidMsat: $paidMsat, totalIncomingTxs: $totalIncomingTxs, confirmedSats: $confirmedSats, unconfirmedSats: $unconfirmedSats, status: $status, refundTxIds: $refundTxIds, unconfirmedTxIds: $unconfirmedTxIds, confirmedTxIds: $confirmedTxIds, minAllowedDeposit: $minAllowedDeposit, maxAllowedDeposit: $maxAllowedDeposit, lastRedeemError: $lastRedeemError, channelOpeningFees: $channelOpeningFees, confirmedAt: $confirmedAt)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SwapInfoImpl &&
            (identical(other.bitcoinAddress, bitcoinAddress) || other.bitcoinAddress == bitcoinAddress) &&
            (identical(other.createdAt, createdAt) || other.createdAt == createdAt) &&
            (identical(other.lockHeight, lockHeight) || other.lockHeight == lockHeight) &&
            const DeepCollectionEquality().equals(other.paymentHash, paymentHash) &&
            const DeepCollectionEquality().equals(other.preimage, preimage) &&
            const DeepCollectionEquality().equals(other.privateKey, privateKey) &&
            const DeepCollectionEquality().equals(other.publicKey, publicKey) &&
            const DeepCollectionEquality().equals(other.swapperPublicKey, swapperPublicKey) &&
            const DeepCollectionEquality().equals(other.script, script) &&
            (identical(other.bolt11, bolt11) || other.bolt11 == bolt11) &&
            (identical(other.paidMsat, paidMsat) || other.paidMsat == paidMsat) &&
            (identical(other.totalIncomingTxs, totalIncomingTxs) ||
                other.totalIncomingTxs == totalIncomingTxs) &&
            (identical(other.confirmedSats, confirmedSats) || other.confirmedSats == confirmedSats) &&
            (identical(other.unconfirmedSats, unconfirmedSats) || other.unconfirmedSats == unconfirmedSats) &&
            (identical(other.status, status) || other.status == status) &&
            const DeepCollectionEquality().equals(other._refundTxIds, _refundTxIds) &&
            const DeepCollectionEquality().equals(other._unconfirmedTxIds, _unconfirmedTxIds) &&
            const DeepCollectionEquality().equals(other._confirmedTxIds, _confirmedTxIds) &&
            (identical(other.minAllowedDeposit, minAllowedDeposit) ||
                other.minAllowedDeposit == minAllowedDeposit) &&
            (identical(other.maxAllowedDeposit, maxAllowedDeposit) ||
                other.maxAllowedDeposit == maxAllowedDeposit) &&
            (identical(other.lastRedeemError, lastRedeemError) || other.lastRedeemError == lastRedeemError) &&
            (identical(other.channelOpeningFees, channelOpeningFees) ||
                other.channelOpeningFees == channelOpeningFees) &&
            (identical(other.confirmedAt, confirmedAt) || other.confirmedAt == confirmedAt));
  }

  @override
  int get hashCode => Object.hashAll([
        runtimeType,
        bitcoinAddress,
        createdAt,
        lockHeight,
        const DeepCollectionEquality().hash(paymentHash),
        const DeepCollectionEquality().hash(preimage),
        const DeepCollectionEquality().hash(privateKey),
        const DeepCollectionEquality().hash(publicKey),
        const DeepCollectionEquality().hash(swapperPublicKey),
        const DeepCollectionEquality().hash(script),
        bolt11,
        paidMsat,
        totalIncomingTxs,
        confirmedSats,
        unconfirmedSats,
        status,
        const DeepCollectionEquality().hash(_refundTxIds),
        const DeepCollectionEquality().hash(_unconfirmedTxIds),
        const DeepCollectionEquality().hash(_confirmedTxIds),
        minAllowedDeposit,
        maxAllowedDeposit,
        lastRedeemError,
        channelOpeningFees,
        confirmedAt
      ]);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SwapInfoImplCopyWith<_$SwapInfoImpl> get copyWith =>
      __$$SwapInfoImplCopyWithImpl<_$SwapInfoImpl>(this, _$identity);
}

abstract class _SwapInfo implements SwapInfo {
  const factory _SwapInfo(
      {required final String bitcoinAddress,
      required final int createdAt,
      required final int lockHeight,
      required final Uint8List paymentHash,
      required final Uint8List preimage,
      required final Uint8List privateKey,
      required final Uint8List publicKey,
      required final Uint8List swapperPublicKey,
      required final Uint8List script,
      final String? bolt11,
      required final int paidMsat,
      required final int totalIncomingTxs,
      required final int confirmedSats,
      required final int unconfirmedSats,
      required final SwapStatus status,
      required final List<String> refundTxIds,
      required final List<String> unconfirmedTxIds,
      required final List<String> confirmedTxIds,
      required final int minAllowedDeposit,
      required final int maxAllowedDeposit,
      final String? lastRedeemError,
      final OpeningFeeParams? channelOpeningFees,
      final int? confirmedAt}) = _$SwapInfoImpl;

  @override
  String get bitcoinAddress;
  @override
  int get createdAt;
  @override
  int get lockHeight;
  @override
  Uint8List get paymentHash;
  @override
  Uint8List get preimage;
  @override
  Uint8List get privateKey;
  @override
  Uint8List get publicKey;
  @override
  Uint8List get swapperPublicKey;
  @override
  Uint8List get script;
  @override
  String? get bolt11;
  @override
  int get paidMsat;
  @override
  int get totalIncomingTxs;
  @override
  int get confirmedSats;
  @override
  int get unconfirmedSats;
  @override
  SwapStatus get status;
  @override
  List<String> get refundTxIds;
  @override
  List<String> get unconfirmedTxIds;
  @override
  List<String> get confirmedTxIds;
  @override
  int get minAllowedDeposit;
  @override
  int get maxAllowedDeposit;
  @override
  String? get lastRedeemError;
  @override
  OpeningFeeParams? get channelOpeningFees;
  @override
  int? get confirmedAt;
  @override
  @JsonKey(ignore: true)
  _$$SwapInfoImplCopyWith<_$SwapInfoImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Symbol {
  String? get grapheme => throw _privateConstructorUsedError;
  String? get template => throw _privateConstructorUsedError;
  bool? get rtl => throw _privateConstructorUsedError;
  int? get position => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $SymbolCopyWith<Symbol> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SymbolCopyWith<$Res> {
  factory $SymbolCopyWith(Symbol value, $Res Function(Symbol) then) = _$SymbolCopyWithImpl<$Res, Symbol>;
  @useResult
  $Res call({String? grapheme, String? template, bool? rtl, int? position});
}

/// @nodoc
class _$SymbolCopyWithImpl<$Res, $Val extends Symbol> implements $SymbolCopyWith<$Res> {
  _$SymbolCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? grapheme = freezed,
    Object? template = freezed,
    Object? rtl = freezed,
    Object? position = freezed,
  }) {
    return _then(_value.copyWith(
      grapheme: freezed == grapheme
          ? _value.grapheme
          : grapheme // ignore: cast_nullable_to_non_nullable
              as String?,
      template: freezed == template
          ? _value.template
          : template // ignore: cast_nullable_to_non_nullable
              as String?,
      rtl: freezed == rtl
          ? _value.rtl
          : rtl // ignore: cast_nullable_to_non_nullable
              as bool?,
      position: freezed == position
          ? _value.position
          : position // ignore: cast_nullable_to_non_nullable
              as int?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$SymbolImplCopyWith<$Res> implements $SymbolCopyWith<$Res> {
  factory _$$SymbolImplCopyWith(_$SymbolImpl value, $Res Function(_$SymbolImpl) then) =
      __$$SymbolImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String? grapheme, String? template, bool? rtl, int? position});
}

/// @nodoc
class __$$SymbolImplCopyWithImpl<$Res> extends _$SymbolCopyWithImpl<$Res, _$SymbolImpl>
    implements _$$SymbolImplCopyWith<$Res> {
  __$$SymbolImplCopyWithImpl(_$SymbolImpl _value, $Res Function(_$SymbolImpl) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? grapheme = freezed,
    Object? template = freezed,
    Object? rtl = freezed,
    Object? position = freezed,
  }) {
    return _then(_$SymbolImpl(
      grapheme: freezed == grapheme
          ? _value.grapheme
          : grapheme // ignore: cast_nullable_to_non_nullable
              as String?,
      template: freezed == template
          ? _value.template
          : template // ignore: cast_nullable_to_non_nullable
              as String?,
      rtl: freezed == rtl
          ? _value.rtl
          : rtl // ignore: cast_nullable_to_non_nullable
              as bool?,
      position: freezed == position
          ? _value.position
          : position // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$SymbolImpl implements _Symbol {
  const _$SymbolImpl({this.grapheme, this.template, this.rtl, this.position});

  @override
  final String? grapheme;
  @override
  final String? template;
  @override
  final bool? rtl;
  @override
  final int? position;

  @override
  String toString() {
    return 'Symbol(grapheme: $grapheme, template: $template, rtl: $rtl, position: $position)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SymbolImpl &&
            (identical(other.grapheme, grapheme) || other.grapheme == grapheme) &&
            (identical(other.template, template) || other.template == template) &&
            (identical(other.rtl, rtl) || other.rtl == rtl) &&
            (identical(other.position, position) || other.position == position));
  }

  @override
  int get hashCode => Object.hash(runtimeType, grapheme, template, rtl, position);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SymbolImplCopyWith<_$SymbolImpl> get copyWith =>
      __$$SymbolImplCopyWithImpl<_$SymbolImpl>(this, _$identity);
}

abstract class _Symbol implements Symbol {
  const factory _Symbol(
      {final String? grapheme, final String? template, final bool? rtl, final int? position}) = _$SymbolImpl;

  @override
  String? get grapheme;
  @override
  String? get template;
  @override
  bool? get rtl;
  @override
  int? get position;
  @override
  @JsonKey(ignore: true)
  _$$SymbolImplCopyWith<_$SymbolImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$UnspentTransactionOutput {
  Uint8List get txid => throw _privateConstructorUsedError;
  int get outnum => throw _privateConstructorUsedError;
  int get amountMillisatoshi => throw _privateConstructorUsedError;
  String get address => throw _privateConstructorUsedError;
  bool get reserved => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $UnspentTransactionOutputCopyWith<UnspentTransactionOutput> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UnspentTransactionOutputCopyWith<$Res> {
  factory $UnspentTransactionOutputCopyWith(
          UnspentTransactionOutput value, $Res Function(UnspentTransactionOutput) then) =
      _$UnspentTransactionOutputCopyWithImpl<$Res, UnspentTransactionOutput>;
  @useResult
  $Res call({Uint8List txid, int outnum, int amountMillisatoshi, String address, bool reserved});
}

/// @nodoc
class _$UnspentTransactionOutputCopyWithImpl<$Res, $Val extends UnspentTransactionOutput>
    implements $UnspentTransactionOutputCopyWith<$Res> {
  _$UnspentTransactionOutputCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? txid = null,
    Object? outnum = null,
    Object? amountMillisatoshi = null,
    Object? address = null,
    Object? reserved = null,
  }) {
    return _then(_value.copyWith(
      txid: null == txid
          ? _value.txid
          : txid // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      outnum: null == outnum
          ? _value.outnum
          : outnum // ignore: cast_nullable_to_non_nullable
              as int,
      amountMillisatoshi: null == amountMillisatoshi
          ? _value.amountMillisatoshi
          : amountMillisatoshi // ignore: cast_nullable_to_non_nullable
              as int,
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      reserved: null == reserved
          ? _value.reserved
          : reserved // ignore: cast_nullable_to_non_nullable
              as bool,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$UnspentTransactionOutputImplCopyWith<$Res>
    implements $UnspentTransactionOutputCopyWith<$Res> {
  factory _$$UnspentTransactionOutputImplCopyWith(
          _$UnspentTransactionOutputImpl value, $Res Function(_$UnspentTransactionOutputImpl) then) =
      __$$UnspentTransactionOutputImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({Uint8List txid, int outnum, int amountMillisatoshi, String address, bool reserved});
}

/// @nodoc
class __$$UnspentTransactionOutputImplCopyWithImpl<$Res>
    extends _$UnspentTransactionOutputCopyWithImpl<$Res, _$UnspentTransactionOutputImpl>
    implements _$$UnspentTransactionOutputImplCopyWith<$Res> {
  __$$UnspentTransactionOutputImplCopyWithImpl(
      _$UnspentTransactionOutputImpl _value, $Res Function(_$UnspentTransactionOutputImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? txid = null,
    Object? outnum = null,
    Object? amountMillisatoshi = null,
    Object? address = null,
    Object? reserved = null,
  }) {
    return _then(_$UnspentTransactionOutputImpl(
      txid: null == txid
          ? _value.txid
          : txid // ignore: cast_nullable_to_non_nullable
              as Uint8List,
      outnum: null == outnum
          ? _value.outnum
          : outnum // ignore: cast_nullable_to_non_nullable
              as int,
      amountMillisatoshi: null == amountMillisatoshi
          ? _value.amountMillisatoshi
          : amountMillisatoshi // ignore: cast_nullable_to_non_nullable
              as int,
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      reserved: null == reserved
          ? _value.reserved
          : reserved // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$UnspentTransactionOutputImpl implements _UnspentTransactionOutput {
  const _$UnspentTransactionOutputImpl(
      {required this.txid,
      required this.outnum,
      required this.amountMillisatoshi,
      required this.address,
      required this.reserved});

  @override
  final Uint8List txid;
  @override
  final int outnum;
  @override
  final int amountMillisatoshi;
  @override
  final String address;
  @override
  final bool reserved;

  @override
  String toString() {
    return 'UnspentTransactionOutput(txid: $txid, outnum: $outnum, amountMillisatoshi: $amountMillisatoshi, address: $address, reserved: $reserved)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UnspentTransactionOutputImpl &&
            const DeepCollectionEquality().equals(other.txid, txid) &&
            (identical(other.outnum, outnum) || other.outnum == outnum) &&
            (identical(other.amountMillisatoshi, amountMillisatoshi) ||
                other.amountMillisatoshi == amountMillisatoshi) &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.reserved, reserved) || other.reserved == reserved));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(txid), outnum, amountMillisatoshi, address, reserved);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$UnspentTransactionOutputImplCopyWith<_$UnspentTransactionOutputImpl> get copyWith =>
      __$$UnspentTransactionOutputImplCopyWithImpl<_$UnspentTransactionOutputImpl>(this, _$identity);
}

abstract class _UnspentTransactionOutput implements UnspentTransactionOutput {
  const factory _UnspentTransactionOutput(
      {required final Uint8List txid,
      required final int outnum,
      required final int amountMillisatoshi,
      required final String address,
      required final bool reserved}) = _$UnspentTransactionOutputImpl;

  @override
  Uint8List get txid;
  @override
  int get outnum;
  @override
  int get amountMillisatoshi;
  @override
  String get address;
  @override
  bool get reserved;
  @override
  @JsonKey(ignore: true)
  _$$UnspentTransactionOutputImplCopyWith<_$UnspentTransactionOutputImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
