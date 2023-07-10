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
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

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
abstract class _$$BreezEvent_NewBlockCopyWith<$Res> {
  factory _$$BreezEvent_NewBlockCopyWith(
          _$BreezEvent_NewBlock value, $Res Function(_$BreezEvent_NewBlock) then) =
      __$$BreezEvent_NewBlockCopyWithImpl<$Res>;
  @useResult
  $Res call({int block});
}

/// @nodoc
class __$$BreezEvent_NewBlockCopyWithImpl<$Res> extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_NewBlock>
    implements _$$BreezEvent_NewBlockCopyWith<$Res> {
  __$$BreezEvent_NewBlockCopyWithImpl(
      _$BreezEvent_NewBlock _value, $Res Function(_$BreezEvent_NewBlock) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? block = null,
  }) {
    return _then(_$BreezEvent_NewBlock(
      block: null == block
          ? _value.block
          : block // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$BreezEvent_NewBlock implements BreezEvent_NewBlock {
  const _$BreezEvent_NewBlock({required this.block});

  @override
  final int block;

  @override
  String toString() {
    return 'BreezEvent.newBlock(block: $block)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BreezEvent_NewBlock &&
            (identical(other.block, block) || other.block == block));
  }

  @override
  int get hashCode => Object.hash(runtimeType, block);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BreezEvent_NewBlockCopyWith<_$BreezEvent_NewBlock> get copyWith =>
      __$$BreezEvent_NewBlockCopyWithImpl<_$BreezEvent_NewBlock>(this, _$identity);

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
    required TResult orElse(),
  }) {
    if (newBlock != null) {
      return newBlock(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_NewBlock implements BreezEvent {
  const factory BreezEvent_NewBlock({required final int block}) = _$BreezEvent_NewBlock;

  int get block;
  @JsonKey(ignore: true)
  _$$BreezEvent_NewBlockCopyWith<_$BreezEvent_NewBlock> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BreezEvent_InvoicePaidCopyWith<$Res> {
  factory _$$BreezEvent_InvoicePaidCopyWith(
          _$BreezEvent_InvoicePaid value, $Res Function(_$BreezEvent_InvoicePaid) then) =
      __$$BreezEvent_InvoicePaidCopyWithImpl<$Res>;
  @useResult
  $Res call({InvoicePaidDetails details});
}

/// @nodoc
class __$$BreezEvent_InvoicePaidCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_InvoicePaid>
    implements _$$BreezEvent_InvoicePaidCopyWith<$Res> {
  __$$BreezEvent_InvoicePaidCopyWithImpl(
      _$BreezEvent_InvoicePaid _value, $Res Function(_$BreezEvent_InvoicePaid) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$BreezEvent_InvoicePaid(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as InvoicePaidDetails,
    ));
  }
}

/// @nodoc

class _$BreezEvent_InvoicePaid implements BreezEvent_InvoicePaid {
  const _$BreezEvent_InvoicePaid({required this.details});

  @override
  final InvoicePaidDetails details;

  @override
  String toString() {
    return 'BreezEvent.invoicePaid(details: $details)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BreezEvent_InvoicePaid &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BreezEvent_InvoicePaidCopyWith<_$BreezEvent_InvoicePaid> get copyWith =>
      __$$BreezEvent_InvoicePaidCopyWithImpl<_$BreezEvent_InvoicePaid>(this, _$identity);

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
      _$BreezEvent_InvoicePaid;

  InvoicePaidDetails get details;
  @JsonKey(ignore: true)
  _$$BreezEvent_InvoicePaidCopyWith<_$BreezEvent_InvoicePaid> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BreezEvent_SyncedCopyWith<$Res> {
  factory _$$BreezEvent_SyncedCopyWith(_$BreezEvent_Synced value, $Res Function(_$BreezEvent_Synced) then) =
      __$$BreezEvent_SyncedCopyWithImpl<$Res>;
}

/// @nodoc
class __$$BreezEvent_SyncedCopyWithImpl<$Res> extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_Synced>
    implements _$$BreezEvent_SyncedCopyWith<$Res> {
  __$$BreezEvent_SyncedCopyWithImpl(_$BreezEvent_Synced _value, $Res Function(_$BreezEvent_Synced) _then)
      : super(_value, _then);
}

/// @nodoc

class _$BreezEvent_Synced implements BreezEvent_Synced {
  const _$BreezEvent_Synced();

  @override
  String toString() {
    return 'BreezEvent.synced()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$BreezEvent_Synced);
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
    required TResult orElse(),
  }) {
    if (synced != null) {
      return synced(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_Synced implements BreezEvent {
  const factory BreezEvent_Synced() = _$BreezEvent_Synced;
}

/// @nodoc
abstract class _$$BreezEvent_PaymentSucceedCopyWith<$Res> {
  factory _$$BreezEvent_PaymentSucceedCopyWith(
          _$BreezEvent_PaymentSucceed value, $Res Function(_$BreezEvent_PaymentSucceed) then) =
      __$$BreezEvent_PaymentSucceedCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$BreezEvent_PaymentSucceedCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_PaymentSucceed>
    implements _$$BreezEvent_PaymentSucceedCopyWith<$Res> {
  __$$BreezEvent_PaymentSucceedCopyWithImpl(
      _$BreezEvent_PaymentSucceed _value, $Res Function(_$BreezEvent_PaymentSucceed) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$BreezEvent_PaymentSucceed(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$BreezEvent_PaymentSucceed implements BreezEvent_PaymentSucceed {
  const _$BreezEvent_PaymentSucceed({required this.details});

  @override
  final Payment details;

  @override
  String toString() {
    return 'BreezEvent.paymentSucceed(details: $details)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BreezEvent_PaymentSucceed &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BreezEvent_PaymentSucceedCopyWith<_$BreezEvent_PaymentSucceed> get copyWith =>
      __$$BreezEvent_PaymentSucceedCopyWithImpl<_$BreezEvent_PaymentSucceed>(this, _$identity);

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
    required TResult orElse(),
  }) {
    if (paymentSucceed != null) {
      return paymentSucceed(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_PaymentSucceed implements BreezEvent {
  const factory BreezEvent_PaymentSucceed({required final Payment details}) = _$BreezEvent_PaymentSucceed;

  Payment get details;
  @JsonKey(ignore: true)
  _$$BreezEvent_PaymentSucceedCopyWith<_$BreezEvent_PaymentSucceed> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BreezEvent_PaymentFailedCopyWith<$Res> {
  factory _$$BreezEvent_PaymentFailedCopyWith(
          _$BreezEvent_PaymentFailed value, $Res Function(_$BreezEvent_PaymentFailed) then) =
      __$$BreezEvent_PaymentFailedCopyWithImpl<$Res>;
  @useResult
  $Res call({PaymentFailedData details});
}

/// @nodoc
class __$$BreezEvent_PaymentFailedCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_PaymentFailed>
    implements _$$BreezEvent_PaymentFailedCopyWith<$Res> {
  __$$BreezEvent_PaymentFailedCopyWithImpl(
      _$BreezEvent_PaymentFailed _value, $Res Function(_$BreezEvent_PaymentFailed) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$BreezEvent_PaymentFailed(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as PaymentFailedData,
    ));
  }
}

/// @nodoc

class _$BreezEvent_PaymentFailed implements BreezEvent_PaymentFailed {
  const _$BreezEvent_PaymentFailed({required this.details});

  @override
  final PaymentFailedData details;

  @override
  String toString() {
    return 'BreezEvent.paymentFailed(details: $details)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BreezEvent_PaymentFailed &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BreezEvent_PaymentFailedCopyWith<_$BreezEvent_PaymentFailed> get copyWith =>
      __$$BreezEvent_PaymentFailedCopyWithImpl<_$BreezEvent_PaymentFailed>(this, _$identity);

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
      _$BreezEvent_PaymentFailed;

  PaymentFailedData get details;
  @JsonKey(ignore: true)
  _$$BreezEvent_PaymentFailedCopyWith<_$BreezEvent_PaymentFailed> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BreezEvent_BackupStartedCopyWith<$Res> {
  factory _$$BreezEvent_BackupStartedCopyWith(
          _$BreezEvent_BackupStarted value, $Res Function(_$BreezEvent_BackupStarted) then) =
      __$$BreezEvent_BackupStartedCopyWithImpl<$Res>;
}

/// @nodoc
class __$$BreezEvent_BackupStartedCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_BackupStarted>
    implements _$$BreezEvent_BackupStartedCopyWith<$Res> {
  __$$BreezEvent_BackupStartedCopyWithImpl(
      _$BreezEvent_BackupStarted _value, $Res Function(_$BreezEvent_BackupStarted) _then)
      : super(_value, _then);
}

/// @nodoc

class _$BreezEvent_BackupStarted implements BreezEvent_BackupStarted {
  const _$BreezEvent_BackupStarted();

  @override
  String toString() {
    return 'BreezEvent.backupStarted()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$BreezEvent_BackupStarted);
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
    required TResult orElse(),
  }) {
    if (backupStarted != null) {
      return backupStarted(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_BackupStarted implements BreezEvent {
  const factory BreezEvent_BackupStarted() = _$BreezEvent_BackupStarted;
}

/// @nodoc
abstract class _$$BreezEvent_BackupSucceededCopyWith<$Res> {
  factory _$$BreezEvent_BackupSucceededCopyWith(
          _$BreezEvent_BackupSucceeded value, $Res Function(_$BreezEvent_BackupSucceeded) then) =
      __$$BreezEvent_BackupSucceededCopyWithImpl<$Res>;
}

/// @nodoc
class __$$BreezEvent_BackupSucceededCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_BackupSucceeded>
    implements _$$BreezEvent_BackupSucceededCopyWith<$Res> {
  __$$BreezEvent_BackupSucceededCopyWithImpl(
      _$BreezEvent_BackupSucceeded _value, $Res Function(_$BreezEvent_BackupSucceeded) _then)
      : super(_value, _then);
}

/// @nodoc

class _$BreezEvent_BackupSucceeded implements BreezEvent_BackupSucceeded {
  const _$BreezEvent_BackupSucceeded();

  @override
  String toString() {
    return 'BreezEvent.backupSucceeded()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$BreezEvent_BackupSucceeded);
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
    required TResult orElse(),
  }) {
    if (backupSucceeded != null) {
      return backupSucceeded(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_BackupSucceeded implements BreezEvent {
  const factory BreezEvent_BackupSucceeded() = _$BreezEvent_BackupSucceeded;
}

/// @nodoc
abstract class _$$BreezEvent_BackupFailedCopyWith<$Res> {
  factory _$$BreezEvent_BackupFailedCopyWith(
          _$BreezEvent_BackupFailed value, $Res Function(_$BreezEvent_BackupFailed) then) =
      __$$BreezEvent_BackupFailedCopyWithImpl<$Res>;
  @useResult
  $Res call({BackupFailedData details});
}

/// @nodoc
class __$$BreezEvent_BackupFailedCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_BackupFailed>
    implements _$$BreezEvent_BackupFailedCopyWith<$Res> {
  __$$BreezEvent_BackupFailedCopyWithImpl(
      _$BreezEvent_BackupFailed _value, $Res Function(_$BreezEvent_BackupFailed) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$BreezEvent_BackupFailed(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as BackupFailedData,
    ));
  }
}

/// @nodoc

class _$BreezEvent_BackupFailed implements BreezEvent_BackupFailed {
  const _$BreezEvent_BackupFailed({required this.details});

  @override
  final BackupFailedData details;

  @override
  String toString() {
    return 'BreezEvent.backupFailed(details: $details)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BreezEvent_BackupFailed &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BreezEvent_BackupFailedCopyWith<_$BreezEvent_BackupFailed> get copyWith =>
      __$$BreezEvent_BackupFailedCopyWithImpl<_$BreezEvent_BackupFailed>(this, _$identity);

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
      _$BreezEvent_BackupFailed;

  BackupFailedData get details;
  @JsonKey(ignore: true)
  _$$BreezEvent_BackupFailedCopyWith<_$BreezEvent_BackupFailed> get copyWith =>
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
abstract class _$$InputType_BitcoinAddressCopyWith<$Res> {
  factory _$$InputType_BitcoinAddressCopyWith(
          _$InputType_BitcoinAddress value, $Res Function(_$InputType_BitcoinAddress) then) =
      __$$InputType_BitcoinAddressCopyWithImpl<$Res>;
  @useResult
  $Res call({BitcoinAddressData address});
}

/// @nodoc
class __$$InputType_BitcoinAddressCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_BitcoinAddress>
    implements _$$InputType_BitcoinAddressCopyWith<$Res> {
  __$$InputType_BitcoinAddressCopyWithImpl(
      _$InputType_BitcoinAddress _value, $Res Function(_$InputType_BitcoinAddress) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
  }) {
    return _then(_$InputType_BitcoinAddress(
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as BitcoinAddressData,
    ));
  }
}

/// @nodoc

class _$InputType_BitcoinAddress implements InputType_BitcoinAddress {
  const _$InputType_BitcoinAddress({required this.address});

  @override
  final BitcoinAddressData address;

  @override
  String toString() {
    return 'InputType.bitcoinAddress(address: $address)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_BitcoinAddress &&
            (identical(other.address, address) || other.address == address));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_BitcoinAddressCopyWith<_$InputType_BitcoinAddress> get copyWith =>
      __$$InputType_BitcoinAddressCopyWithImpl<_$InputType_BitcoinAddress>(this, _$identity);

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
      _$InputType_BitcoinAddress;

  BitcoinAddressData get address;
  @JsonKey(ignore: true)
  _$$InputType_BitcoinAddressCopyWith<_$InputType_BitcoinAddress> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_Bolt11CopyWith<$Res> {
  factory _$$InputType_Bolt11CopyWith(_$InputType_Bolt11 value, $Res Function(_$InputType_Bolt11) then) =
      __$$InputType_Bolt11CopyWithImpl<$Res>;
  @useResult
  $Res call({LNInvoice invoice});
}

/// @nodoc
class __$$InputType_Bolt11CopyWithImpl<$Res> extends _$InputTypeCopyWithImpl<$Res, _$InputType_Bolt11>
    implements _$$InputType_Bolt11CopyWith<$Res> {
  __$$InputType_Bolt11CopyWithImpl(_$InputType_Bolt11 _value, $Res Function(_$InputType_Bolt11) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? invoice = null,
  }) {
    return _then(_$InputType_Bolt11(
      invoice: null == invoice
          ? _value.invoice
          : invoice // ignore: cast_nullable_to_non_nullable
              as LNInvoice,
    ));
  }
}

/// @nodoc

class _$InputType_Bolt11 implements InputType_Bolt11 {
  const _$InputType_Bolt11({required this.invoice});

  @override
  final LNInvoice invoice;

  @override
  String toString() {
    return 'InputType.bolt11(invoice: $invoice)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_Bolt11 &&
            (identical(other.invoice, invoice) || other.invoice == invoice));
  }

  @override
  int get hashCode => Object.hash(runtimeType, invoice);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_Bolt11CopyWith<_$InputType_Bolt11> get copyWith =>
      __$$InputType_Bolt11CopyWithImpl<_$InputType_Bolt11>(this, _$identity);

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
  const factory InputType_Bolt11({required final LNInvoice invoice}) = _$InputType_Bolt11;

  LNInvoice get invoice;
  @JsonKey(ignore: true)
  _$$InputType_Bolt11CopyWith<_$InputType_Bolt11> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_NodeIdCopyWith<$Res> {
  factory _$$InputType_NodeIdCopyWith(_$InputType_NodeId value, $Res Function(_$InputType_NodeId) then) =
      __$$InputType_NodeIdCopyWithImpl<$Res>;
  @useResult
  $Res call({String nodeId});
}

/// @nodoc
class __$$InputType_NodeIdCopyWithImpl<$Res> extends _$InputTypeCopyWithImpl<$Res, _$InputType_NodeId>
    implements _$$InputType_NodeIdCopyWith<$Res> {
  __$$InputType_NodeIdCopyWithImpl(_$InputType_NodeId _value, $Res Function(_$InputType_NodeId) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? nodeId = null,
  }) {
    return _then(_$InputType_NodeId(
      nodeId: null == nodeId
          ? _value.nodeId
          : nodeId // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$InputType_NodeId implements InputType_NodeId {
  const _$InputType_NodeId({required this.nodeId});

  @override
  final String nodeId;

  @override
  String toString() {
    return 'InputType.nodeId(nodeId: $nodeId)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_NodeId &&
            (identical(other.nodeId, nodeId) || other.nodeId == nodeId));
  }

  @override
  int get hashCode => Object.hash(runtimeType, nodeId);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_NodeIdCopyWith<_$InputType_NodeId> get copyWith =>
      __$$InputType_NodeIdCopyWithImpl<_$InputType_NodeId>(this, _$identity);

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
  const factory InputType_NodeId({required final String nodeId}) = _$InputType_NodeId;

  String get nodeId;
  @JsonKey(ignore: true)
  _$$InputType_NodeIdCopyWith<_$InputType_NodeId> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_UrlCopyWith<$Res> {
  factory _$$InputType_UrlCopyWith(_$InputType_Url value, $Res Function(_$InputType_Url) then) =
      __$$InputType_UrlCopyWithImpl<$Res>;
  @useResult
  $Res call({String url});
}

/// @nodoc
class __$$InputType_UrlCopyWithImpl<$Res> extends _$InputTypeCopyWithImpl<$Res, _$InputType_Url>
    implements _$$InputType_UrlCopyWith<$Res> {
  __$$InputType_UrlCopyWithImpl(_$InputType_Url _value, $Res Function(_$InputType_Url) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? url = null,
  }) {
    return _then(_$InputType_Url(
      url: null == url
          ? _value.url
          : url // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$InputType_Url implements InputType_Url {
  const _$InputType_Url({required this.url});

  @override
  final String url;

  @override
  String toString() {
    return 'InputType.url(url: $url)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_Url &&
            (identical(other.url, url) || other.url == url));
  }

  @override
  int get hashCode => Object.hash(runtimeType, url);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_UrlCopyWith<_$InputType_Url> get copyWith =>
      __$$InputType_UrlCopyWithImpl<_$InputType_Url>(this, _$identity);

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
  const factory InputType_Url({required final String url}) = _$InputType_Url;

  String get url;
  @JsonKey(ignore: true)
  _$$InputType_UrlCopyWith<_$InputType_Url> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlPayCopyWith<$Res> {
  factory _$$InputType_LnUrlPayCopyWith(
          _$InputType_LnUrlPay value, $Res Function(_$InputType_LnUrlPay) then) =
      __$$InputType_LnUrlPayCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlPayRequestData data});
}

/// @nodoc
class __$$InputType_LnUrlPayCopyWithImpl<$Res> extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlPay>
    implements _$$InputType_LnUrlPayCopyWith<$Res> {
  __$$InputType_LnUrlPayCopyWithImpl(_$InputType_LnUrlPay _value, $Res Function(_$InputType_LnUrlPay) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$InputType_LnUrlPay(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlPayRequestData,
    ));
  }
}

/// @nodoc

class _$InputType_LnUrlPay implements InputType_LnUrlPay {
  const _$InputType_LnUrlPay({required this.data});

  @override
  final LnUrlPayRequestData data;

  @override
  String toString() {
    return 'InputType.lnUrlPay(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_LnUrlPay &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_LnUrlPayCopyWith<_$InputType_LnUrlPay> get copyWith =>
      __$$InputType_LnUrlPayCopyWithImpl<_$InputType_LnUrlPay>(this, _$identity);

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
  const factory InputType_LnUrlPay({required final LnUrlPayRequestData data}) = _$InputType_LnUrlPay;

  LnUrlPayRequestData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlPayCopyWith<_$InputType_LnUrlPay> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlWithdrawCopyWith<$Res> {
  factory _$$InputType_LnUrlWithdrawCopyWith(
          _$InputType_LnUrlWithdraw value, $Res Function(_$InputType_LnUrlWithdraw) then) =
      __$$InputType_LnUrlWithdrawCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlWithdrawRequestData data});
}

/// @nodoc
class __$$InputType_LnUrlWithdrawCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlWithdraw>
    implements _$$InputType_LnUrlWithdrawCopyWith<$Res> {
  __$$InputType_LnUrlWithdrawCopyWithImpl(
      _$InputType_LnUrlWithdraw _value, $Res Function(_$InputType_LnUrlWithdraw) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$InputType_LnUrlWithdraw(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlWithdrawRequestData,
    ));
  }
}

/// @nodoc

class _$InputType_LnUrlWithdraw implements InputType_LnUrlWithdraw {
  const _$InputType_LnUrlWithdraw({required this.data});

  @override
  final LnUrlWithdrawRequestData data;

  @override
  String toString() {
    return 'InputType.lnUrlWithdraw(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_LnUrlWithdraw &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_LnUrlWithdrawCopyWith<_$InputType_LnUrlWithdraw> get copyWith =>
      __$$InputType_LnUrlWithdrawCopyWithImpl<_$InputType_LnUrlWithdraw>(this, _$identity);

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
      _$InputType_LnUrlWithdraw;

  LnUrlWithdrawRequestData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlWithdrawCopyWith<_$InputType_LnUrlWithdraw> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlAuthCopyWith<$Res> {
  factory _$$InputType_LnUrlAuthCopyWith(
          _$InputType_LnUrlAuth value, $Res Function(_$InputType_LnUrlAuth) then) =
      __$$InputType_LnUrlAuthCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlAuthRequestData data});
}

/// @nodoc
class __$$InputType_LnUrlAuthCopyWithImpl<$Res> extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlAuth>
    implements _$$InputType_LnUrlAuthCopyWith<$Res> {
  __$$InputType_LnUrlAuthCopyWithImpl(
      _$InputType_LnUrlAuth _value, $Res Function(_$InputType_LnUrlAuth) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$InputType_LnUrlAuth(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlAuthRequestData,
    ));
  }
}

/// @nodoc

class _$InputType_LnUrlAuth implements InputType_LnUrlAuth {
  const _$InputType_LnUrlAuth({required this.data});

  @override
  final LnUrlAuthRequestData data;

  @override
  String toString() {
    return 'InputType.lnUrlAuth(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_LnUrlAuth &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_LnUrlAuthCopyWith<_$InputType_LnUrlAuth> get copyWith =>
      __$$InputType_LnUrlAuthCopyWithImpl<_$InputType_LnUrlAuth>(this, _$identity);

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
  const factory InputType_LnUrlAuth({required final LnUrlAuthRequestData data}) = _$InputType_LnUrlAuth;

  LnUrlAuthRequestData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlAuthCopyWith<_$InputType_LnUrlAuth> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlErrorCopyWith<$Res> {
  factory _$$InputType_LnUrlErrorCopyWith(
          _$InputType_LnUrlError value, $Res Function(_$InputType_LnUrlError) then) =
      __$$InputType_LnUrlErrorCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});
}

/// @nodoc
class __$$InputType_LnUrlErrorCopyWithImpl<$Res> extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlError>
    implements _$$InputType_LnUrlErrorCopyWith<$Res> {
  __$$InputType_LnUrlErrorCopyWithImpl(
      _$InputType_LnUrlError _value, $Res Function(_$InputType_LnUrlError) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$InputType_LnUrlError(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }
}

/// @nodoc

class _$InputType_LnUrlError implements InputType_LnUrlError {
  const _$InputType_LnUrlError({required this.data});

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'InputType.lnUrlError(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InputType_LnUrlError &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$InputType_LnUrlErrorCopyWith<_$InputType_LnUrlError> get copyWith =>
      __$$InputType_LnUrlErrorCopyWithImpl<_$InputType_LnUrlError>(this, _$identity);

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
  const factory InputType_LnUrlError({required final LnUrlErrorData data}) = _$InputType_LnUrlError;

  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlErrorCopyWith<_$InputType_LnUrlError> get copyWith => throw _privateConstructorUsedError;
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
abstract class _$$LnUrlCallbackStatus_OkCopyWith<$Res> {
  factory _$$LnUrlCallbackStatus_OkCopyWith(
          _$LnUrlCallbackStatus_Ok value, $Res Function(_$LnUrlCallbackStatus_Ok) then) =
      __$$LnUrlCallbackStatus_OkCopyWithImpl<$Res>;
}

/// @nodoc
class __$$LnUrlCallbackStatus_OkCopyWithImpl<$Res>
    extends _$LnUrlCallbackStatusCopyWithImpl<$Res, _$LnUrlCallbackStatus_Ok>
    implements _$$LnUrlCallbackStatus_OkCopyWith<$Res> {
  __$$LnUrlCallbackStatus_OkCopyWithImpl(
      _$LnUrlCallbackStatus_Ok _value, $Res Function(_$LnUrlCallbackStatus_Ok) _then)
      : super(_value, _then);
}

/// @nodoc

class _$LnUrlCallbackStatus_Ok implements LnUrlCallbackStatus_Ok {
  const _$LnUrlCallbackStatus_Ok();

  @override
  String toString() {
    return 'LnUrlCallbackStatus.ok()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$LnUrlCallbackStatus_Ok);
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
  const factory LnUrlCallbackStatus_Ok() = _$LnUrlCallbackStatus_Ok;
}

/// @nodoc
abstract class _$$LnUrlCallbackStatus_ErrorStatusCopyWith<$Res> {
  factory _$$LnUrlCallbackStatus_ErrorStatusCopyWith(
          _$LnUrlCallbackStatus_ErrorStatus value, $Res Function(_$LnUrlCallbackStatus_ErrorStatus) then) =
      __$$LnUrlCallbackStatus_ErrorStatusCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});
}

/// @nodoc
class __$$LnUrlCallbackStatus_ErrorStatusCopyWithImpl<$Res>
    extends _$LnUrlCallbackStatusCopyWithImpl<$Res, _$LnUrlCallbackStatus_ErrorStatus>
    implements _$$LnUrlCallbackStatus_ErrorStatusCopyWith<$Res> {
  __$$LnUrlCallbackStatus_ErrorStatusCopyWithImpl(
      _$LnUrlCallbackStatus_ErrorStatus _value, $Res Function(_$LnUrlCallbackStatus_ErrorStatus) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlCallbackStatus_ErrorStatus(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }
}

/// @nodoc

class _$LnUrlCallbackStatus_ErrorStatus implements LnUrlCallbackStatus_ErrorStatus {
  const _$LnUrlCallbackStatus_ErrorStatus({required this.data});

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'LnUrlCallbackStatus.errorStatus(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlCallbackStatus_ErrorStatus &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlCallbackStatus_ErrorStatusCopyWith<_$LnUrlCallbackStatus_ErrorStatus> get copyWith =>
      __$$LnUrlCallbackStatus_ErrorStatusCopyWithImpl<_$LnUrlCallbackStatus_ErrorStatus>(this, _$identity);

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
      _$LnUrlCallbackStatus_ErrorStatus;

  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlCallbackStatus_ErrorStatusCopyWith<_$LnUrlCallbackStatus_ErrorStatus> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlPayResult {
  Object? get data => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SuccessActionProcessed? data) endpointSuccess,
    required TResult Function(LnUrlErrorData data) endpointError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SuccessActionProcessed? data)? endpointSuccess,
    TResult? Function(LnUrlErrorData data)? endpointError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SuccessActionProcessed? data)? endpointSuccess,
    TResult Function(LnUrlErrorData data)? endpointError,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayResult_EndpointSuccess value) endpointSuccess,
    required TResult Function(LnUrlPayResult_EndpointError value) endpointError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult? Function(LnUrlPayResult_EndpointError value)? endpointError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult Function(LnUrlPayResult_EndpointError value)? endpointError,
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
abstract class _$$LnUrlPayResult_EndpointSuccessCopyWith<$Res> {
  factory _$$LnUrlPayResult_EndpointSuccessCopyWith(
          _$LnUrlPayResult_EndpointSuccess value, $Res Function(_$LnUrlPayResult_EndpointSuccess) then) =
      __$$LnUrlPayResult_EndpointSuccessCopyWithImpl<$Res>;
  @useResult
  $Res call({SuccessActionProcessed? data});

  $SuccessActionProcessedCopyWith<$Res>? get data;
}

/// @nodoc
class __$$LnUrlPayResult_EndpointSuccessCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointSuccess>
    implements _$$LnUrlPayResult_EndpointSuccessCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointSuccessCopyWithImpl(
      _$LnUrlPayResult_EndpointSuccess _value, $Res Function(_$LnUrlPayResult_EndpointSuccess) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = freezed,
  }) {
    return _then(_$LnUrlPayResult_EndpointSuccess(
      data: freezed == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as SuccessActionProcessed?,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $SuccessActionProcessedCopyWith<$Res>? get data {
    if (_value.data == null) {
      return null;
    }

    return $SuccessActionProcessedCopyWith<$Res>(_value.data!, (value) {
      return _then(_value.copyWith(data: value));
    });
  }
}

/// @nodoc

class _$LnUrlPayResult_EndpointSuccess implements LnUrlPayResult_EndpointSuccess {
  const _$LnUrlPayResult_EndpointSuccess({this.data});

  @override
  final SuccessActionProcessed? data;

  @override
  String toString() {
    return 'LnUrlPayResult.endpointSuccess(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayResult_EndpointSuccess &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayResult_EndpointSuccessCopyWith<_$LnUrlPayResult_EndpointSuccess> get copyWith =>
      __$$LnUrlPayResult_EndpointSuccessCopyWithImpl<_$LnUrlPayResult_EndpointSuccess>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SuccessActionProcessed? data) endpointSuccess,
    required TResult Function(LnUrlErrorData data) endpointError,
  }) {
    return endpointSuccess(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SuccessActionProcessed? data)? endpointSuccess,
    TResult? Function(LnUrlErrorData data)? endpointError,
  }) {
    return endpointSuccess?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SuccessActionProcessed? data)? endpointSuccess,
    TResult Function(LnUrlErrorData data)? endpointError,
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
  }) {
    return endpointSuccess(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult? Function(LnUrlPayResult_EndpointError value)? endpointError,
  }) {
    return endpointSuccess?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult Function(LnUrlPayResult_EndpointError value)? endpointError,
    required TResult orElse(),
  }) {
    if (endpointSuccess != null) {
      return endpointSuccess(this);
    }
    return orElse();
  }
}

abstract class LnUrlPayResult_EndpointSuccess implements LnUrlPayResult {
  const factory LnUrlPayResult_EndpointSuccess({final SuccessActionProcessed? data}) =
      _$LnUrlPayResult_EndpointSuccess;

  @override
  SuccessActionProcessed? get data;
  @JsonKey(ignore: true)
  _$$LnUrlPayResult_EndpointSuccessCopyWith<_$LnUrlPayResult_EndpointSuccess> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayResult_EndpointErrorCopyWith<$Res> {
  factory _$$LnUrlPayResult_EndpointErrorCopyWith(
          _$LnUrlPayResult_EndpointError value, $Res Function(_$LnUrlPayResult_EndpointError) then) =
      __$$LnUrlPayResult_EndpointErrorCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});
}

/// @nodoc
class __$$LnUrlPayResult_EndpointErrorCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointError>
    implements _$$LnUrlPayResult_EndpointErrorCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointErrorCopyWithImpl(
      _$LnUrlPayResult_EndpointError _value, $Res Function(_$LnUrlPayResult_EndpointError) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlPayResult_EndpointError(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }
}

/// @nodoc

class _$LnUrlPayResult_EndpointError implements LnUrlPayResult_EndpointError {
  const _$LnUrlPayResult_EndpointError({required this.data});

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'LnUrlPayResult.endpointError(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayResult_EndpointError &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayResult_EndpointErrorCopyWith<_$LnUrlPayResult_EndpointError> get copyWith =>
      __$$LnUrlPayResult_EndpointErrorCopyWithImpl<_$LnUrlPayResult_EndpointError>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SuccessActionProcessed? data) endpointSuccess,
    required TResult Function(LnUrlErrorData data) endpointError,
  }) {
    return endpointError(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SuccessActionProcessed? data)? endpointSuccess,
    TResult? Function(LnUrlErrorData data)? endpointError,
  }) {
    return endpointError?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SuccessActionProcessed? data)? endpointSuccess,
    TResult Function(LnUrlErrorData data)? endpointError,
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
  }) {
    return endpointError(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult? Function(LnUrlPayResult_EndpointError value)? endpointError,
  }) {
    return endpointError?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlPayResult_EndpointSuccess value)? endpointSuccess,
    TResult Function(LnUrlPayResult_EndpointError value)? endpointError,
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
      _$LnUrlPayResult_EndpointError;

  @override
  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlPayResult_EndpointErrorCopyWith<_$LnUrlPayResult_EndpointError> get copyWith =>
      throw _privateConstructorUsedError;
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
}

/// @nodoc
abstract class _$$NodeConfig_GreenlightCopyWith<$Res> implements $NodeConfigCopyWith<$Res> {
  factory _$$NodeConfig_GreenlightCopyWith(
          _$NodeConfig_Greenlight value, $Res Function(_$NodeConfig_Greenlight) then) =
      __$$NodeConfig_GreenlightCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({GreenlightNodeConfig config});
}

/// @nodoc
class __$$NodeConfig_GreenlightCopyWithImpl<$Res>
    extends _$NodeConfigCopyWithImpl<$Res, _$NodeConfig_Greenlight>
    implements _$$NodeConfig_GreenlightCopyWith<$Res> {
  __$$NodeConfig_GreenlightCopyWithImpl(
      _$NodeConfig_Greenlight _value, $Res Function(_$NodeConfig_Greenlight) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? config = null,
  }) {
    return _then(_$NodeConfig_Greenlight(
      config: null == config
          ? _value.config
          : config // ignore: cast_nullable_to_non_nullable
              as GreenlightNodeConfig,
    ));
  }
}

/// @nodoc

class _$NodeConfig_Greenlight implements NodeConfig_Greenlight {
  const _$NodeConfig_Greenlight({required this.config});

  @override
  final GreenlightNodeConfig config;

  @override
  String toString() {
    return 'NodeConfig.greenlight(config: $config)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$NodeConfig_Greenlight &&
            (identical(other.config, config) || other.config == config));
  }

  @override
  int get hashCode => Object.hash(runtimeType, config);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$NodeConfig_GreenlightCopyWith<_$NodeConfig_Greenlight> get copyWith =>
      __$$NodeConfig_GreenlightCopyWithImpl<_$NodeConfig_Greenlight>(this, _$identity);

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
  const factory NodeConfig_Greenlight({required final GreenlightNodeConfig config}) = _$NodeConfig_Greenlight;

  @override
  GreenlightNodeConfig get config;
  @override
  @JsonKey(ignore: true)
  _$$NodeConfig_GreenlightCopyWith<_$NodeConfig_Greenlight> get copyWith =>
      throw _privateConstructorUsedError;
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
abstract class _$$PaymentDetails_LnCopyWith<$Res> {
  factory _$$PaymentDetails_LnCopyWith(_$PaymentDetails_Ln value, $Res Function(_$PaymentDetails_Ln) then) =
      __$$PaymentDetails_LnCopyWithImpl<$Res>;
  @useResult
  $Res call({LnPaymentDetails data});
}

/// @nodoc
class __$$PaymentDetails_LnCopyWithImpl<$Res> extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_Ln>
    implements _$$PaymentDetails_LnCopyWith<$Res> {
  __$$PaymentDetails_LnCopyWithImpl(_$PaymentDetails_Ln _value, $Res Function(_$PaymentDetails_Ln) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$PaymentDetails_Ln(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnPaymentDetails,
    ));
  }
}

/// @nodoc

class _$PaymentDetails_Ln implements PaymentDetails_Ln {
  const _$PaymentDetails_Ln({required this.data});

  @override
  final LnPaymentDetails data;

  @override
  String toString() {
    return 'PaymentDetails.ln(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDetails_Ln &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDetails_LnCopyWith<_$PaymentDetails_Ln> get copyWith =>
      __$$PaymentDetails_LnCopyWithImpl<_$PaymentDetails_Ln>(this, _$identity);

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
  const factory PaymentDetails_Ln({required final LnPaymentDetails data}) = _$PaymentDetails_Ln;

  @override
  LnPaymentDetails get data;
  @JsonKey(ignore: true)
  _$$PaymentDetails_LnCopyWith<_$PaymentDetails_Ln> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PaymentDetails_ClosedChannelCopyWith<$Res> {
  factory _$$PaymentDetails_ClosedChannelCopyWith(
          _$PaymentDetails_ClosedChannel value, $Res Function(_$PaymentDetails_ClosedChannel) then) =
      __$$PaymentDetails_ClosedChannelCopyWithImpl<$Res>;
  @useResult
  $Res call({ClosedChannelPaymentDetails data});
}

/// @nodoc
class __$$PaymentDetails_ClosedChannelCopyWithImpl<$Res>
    extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_ClosedChannel>
    implements _$$PaymentDetails_ClosedChannelCopyWith<$Res> {
  __$$PaymentDetails_ClosedChannelCopyWithImpl(
      _$PaymentDetails_ClosedChannel _value, $Res Function(_$PaymentDetails_ClosedChannel) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$PaymentDetails_ClosedChannel(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as ClosedChannelPaymentDetails,
    ));
  }
}

/// @nodoc

class _$PaymentDetails_ClosedChannel implements PaymentDetails_ClosedChannel {
  const _$PaymentDetails_ClosedChannel({required this.data});

  @override
  final ClosedChannelPaymentDetails data;

  @override
  String toString() {
    return 'PaymentDetails.closedChannel(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PaymentDetails_ClosedChannel &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PaymentDetails_ClosedChannelCopyWith<_$PaymentDetails_ClosedChannel> get copyWith =>
      __$$PaymentDetails_ClosedChannelCopyWithImpl<_$PaymentDetails_ClosedChannel>(this, _$identity);

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
      _$PaymentDetails_ClosedChannel;

  @override
  ClosedChannelPaymentDetails get data;
  @JsonKey(ignore: true)
  _$$PaymentDetails_ClosedChannelCopyWith<_$PaymentDetails_ClosedChannel> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SuccessActionProcessed {
  Object get data => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataDecrypted data) aes,
    required TResult Function(MessageSuccessActionData data) message,
    required TResult Function(UrlSuccessActionData data) url,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataDecrypted data)? aes,
    TResult? Function(MessageSuccessActionData data)? message,
    TResult? Function(UrlSuccessActionData data)? url,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(AesSuccessActionDataDecrypted data)? aes,
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
abstract class _$$SuccessActionProcessed_AesCopyWith<$Res> {
  factory _$$SuccessActionProcessed_AesCopyWith(
          _$SuccessActionProcessed_Aes value, $Res Function(_$SuccessActionProcessed_Aes) then) =
      __$$SuccessActionProcessed_AesCopyWithImpl<$Res>;
  @useResult
  $Res call({AesSuccessActionDataDecrypted data});
}

/// @nodoc
class __$$SuccessActionProcessed_AesCopyWithImpl<$Res>
    extends _$SuccessActionProcessedCopyWithImpl<$Res, _$SuccessActionProcessed_Aes>
    implements _$$SuccessActionProcessed_AesCopyWith<$Res> {
  __$$SuccessActionProcessed_AesCopyWithImpl(
      _$SuccessActionProcessed_Aes _value, $Res Function(_$SuccessActionProcessed_Aes) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$SuccessActionProcessed_Aes(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as AesSuccessActionDataDecrypted,
    ));
  }
}

/// @nodoc

class _$SuccessActionProcessed_Aes implements SuccessActionProcessed_Aes {
  const _$SuccessActionProcessed_Aes({required this.data});

  @override
  final AesSuccessActionDataDecrypted data;

  @override
  String toString() {
    return 'SuccessActionProcessed.aes(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SuccessActionProcessed_Aes &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SuccessActionProcessed_AesCopyWith<_$SuccessActionProcessed_Aes> get copyWith =>
      __$$SuccessActionProcessed_AesCopyWithImpl<_$SuccessActionProcessed_Aes>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataDecrypted data) aes,
    required TResult Function(MessageSuccessActionData data) message,
    required TResult Function(UrlSuccessActionData data) url,
  }) {
    return aes(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataDecrypted data)? aes,
    TResult? Function(MessageSuccessActionData data)? message,
    TResult? Function(UrlSuccessActionData data)? url,
  }) {
    return aes?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(AesSuccessActionDataDecrypted data)? aes,
    TResult Function(MessageSuccessActionData data)? message,
    TResult Function(UrlSuccessActionData data)? url,
    required TResult orElse(),
  }) {
    if (aes != null) {
      return aes(data);
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
  const factory SuccessActionProcessed_Aes({required final AesSuccessActionDataDecrypted data}) =
      _$SuccessActionProcessed_Aes;

  @override
  AesSuccessActionDataDecrypted get data;
  @JsonKey(ignore: true)
  _$$SuccessActionProcessed_AesCopyWith<_$SuccessActionProcessed_Aes> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SuccessActionProcessed_MessageCopyWith<$Res> {
  factory _$$SuccessActionProcessed_MessageCopyWith(
          _$SuccessActionProcessed_Message value, $Res Function(_$SuccessActionProcessed_Message) then) =
      __$$SuccessActionProcessed_MessageCopyWithImpl<$Res>;
  @useResult
  $Res call({MessageSuccessActionData data});
}

/// @nodoc
class __$$SuccessActionProcessed_MessageCopyWithImpl<$Res>
    extends _$SuccessActionProcessedCopyWithImpl<$Res, _$SuccessActionProcessed_Message>
    implements _$$SuccessActionProcessed_MessageCopyWith<$Res> {
  __$$SuccessActionProcessed_MessageCopyWithImpl(
      _$SuccessActionProcessed_Message _value, $Res Function(_$SuccessActionProcessed_Message) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$SuccessActionProcessed_Message(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as MessageSuccessActionData,
    ));
  }
}

/// @nodoc

class _$SuccessActionProcessed_Message implements SuccessActionProcessed_Message {
  const _$SuccessActionProcessed_Message({required this.data});

  @override
  final MessageSuccessActionData data;

  @override
  String toString() {
    return 'SuccessActionProcessed.message(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SuccessActionProcessed_Message &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SuccessActionProcessed_MessageCopyWith<_$SuccessActionProcessed_Message> get copyWith =>
      __$$SuccessActionProcessed_MessageCopyWithImpl<_$SuccessActionProcessed_Message>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataDecrypted data) aes,
    required TResult Function(MessageSuccessActionData data) message,
    required TResult Function(UrlSuccessActionData data) url,
  }) {
    return message(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataDecrypted data)? aes,
    TResult? Function(MessageSuccessActionData data)? message,
    TResult? Function(UrlSuccessActionData data)? url,
  }) {
    return message?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(AesSuccessActionDataDecrypted data)? aes,
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
      _$SuccessActionProcessed_Message;

  @override
  MessageSuccessActionData get data;
  @JsonKey(ignore: true)
  _$$SuccessActionProcessed_MessageCopyWith<_$SuccessActionProcessed_Message> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SuccessActionProcessed_UrlCopyWith<$Res> {
  factory _$$SuccessActionProcessed_UrlCopyWith(
          _$SuccessActionProcessed_Url value, $Res Function(_$SuccessActionProcessed_Url) then) =
      __$$SuccessActionProcessed_UrlCopyWithImpl<$Res>;
  @useResult
  $Res call({UrlSuccessActionData data});
}

/// @nodoc
class __$$SuccessActionProcessed_UrlCopyWithImpl<$Res>
    extends _$SuccessActionProcessedCopyWithImpl<$Res, _$SuccessActionProcessed_Url>
    implements _$$SuccessActionProcessed_UrlCopyWith<$Res> {
  __$$SuccessActionProcessed_UrlCopyWithImpl(
      _$SuccessActionProcessed_Url _value, $Res Function(_$SuccessActionProcessed_Url) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$SuccessActionProcessed_Url(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as UrlSuccessActionData,
    ));
  }
}

/// @nodoc

class _$SuccessActionProcessed_Url implements SuccessActionProcessed_Url {
  const _$SuccessActionProcessed_Url({required this.data});

  @override
  final UrlSuccessActionData data;

  @override
  String toString() {
    return 'SuccessActionProcessed.url(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SuccessActionProcessed_Url &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SuccessActionProcessed_UrlCopyWith<_$SuccessActionProcessed_Url> get copyWith =>
      __$$SuccessActionProcessed_UrlCopyWithImpl<_$SuccessActionProcessed_Url>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(AesSuccessActionDataDecrypted data) aes,
    required TResult Function(MessageSuccessActionData data) message,
    required TResult Function(UrlSuccessActionData data) url,
  }) {
    return url(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(AesSuccessActionDataDecrypted data)? aes,
    TResult? Function(MessageSuccessActionData data)? message,
    TResult? Function(UrlSuccessActionData data)? url,
  }) {
    return url?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(AesSuccessActionDataDecrypted data)? aes,
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
      _$SuccessActionProcessed_Url;

  @override
  UrlSuccessActionData get data;
  @JsonKey(ignore: true)
  _$$SuccessActionProcessed_UrlCopyWith<_$SuccessActionProcessed_Url> get copyWith =>
      throw _privateConstructorUsedError;
}
