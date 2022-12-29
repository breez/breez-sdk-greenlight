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
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BreezEvent_NewBlock value) newBlock,
    required TResult Function(BreezEvent_InvoicePaid value) invoicePaid,
    required TResult Function(BreezEvent_Synced value) synced,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BreezEventCopyWith<$Res> {
  factory $BreezEventCopyWith(
          BreezEvent value, $Res Function(BreezEvent) then) =
      _$BreezEventCopyWithImpl<$Res, BreezEvent>;
}

/// @nodoc
class _$BreezEventCopyWithImpl<$Res, $Val extends BreezEvent>
    implements $BreezEventCopyWith<$Res> {
  _$BreezEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$BreezEvent_NewBlockCopyWith<$Res> {
  factory _$$BreezEvent_NewBlockCopyWith(_$BreezEvent_NewBlock value,
          $Res Function(_$BreezEvent_NewBlock) then) =
      __$$BreezEvent_NewBlockCopyWithImpl<$Res>;
  @useResult
  $Res call({int block});
}

/// @nodoc
class __$$BreezEvent_NewBlockCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_NewBlock>
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
      __$$BreezEvent_NewBlockCopyWithImpl<_$BreezEvent_NewBlock>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
  }) {
    return newBlock(block);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
  }) {
    return newBlock?.call(block);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
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
  }) {
    return newBlock(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
  }) {
    return newBlock?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    required TResult orElse(),
  }) {
    if (newBlock != null) {
      return newBlock(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_NewBlock implements BreezEvent {
  const factory BreezEvent_NewBlock({required final int block}) =
      _$BreezEvent_NewBlock;

  int get block;
  @JsonKey(ignore: true)
  _$$BreezEvent_NewBlockCopyWith<_$BreezEvent_NewBlock> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BreezEvent_InvoicePaidCopyWith<$Res> {
  factory _$$BreezEvent_InvoicePaidCopyWith(_$BreezEvent_InvoicePaid value,
          $Res Function(_$BreezEvent_InvoicePaid) then) =
      __$$BreezEvent_InvoicePaidCopyWithImpl<$Res>;
  @useResult
  $Res call({InvoicePaidDetails details});
}

/// @nodoc
class __$$BreezEvent_InvoicePaidCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_InvoicePaid>
    implements _$$BreezEvent_InvoicePaidCopyWith<$Res> {
  __$$BreezEvent_InvoicePaidCopyWithImpl(_$BreezEvent_InvoicePaid _value,
      $Res Function(_$BreezEvent_InvoicePaid) _then)
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
      __$$BreezEvent_InvoicePaidCopyWithImpl<_$BreezEvent_InvoicePaid>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
  }) {
    return invoicePaid(details);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
  }) {
    return invoicePaid?.call(details);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
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
  }) {
    return invoicePaid(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
  }) {
    return invoicePaid?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
    required TResult orElse(),
  }) {
    if (invoicePaid != null) {
      return invoicePaid(this);
    }
    return orElse();
  }
}

abstract class BreezEvent_InvoicePaid implements BreezEvent {
  const factory BreezEvent_InvoicePaid(
      {required final InvoicePaidDetails details}) = _$BreezEvent_InvoicePaid;

  InvoicePaidDetails get details;
  @JsonKey(ignore: true)
  _$$BreezEvent_InvoicePaidCopyWith<_$BreezEvent_InvoicePaid> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BreezEvent_SyncedCopyWith<$Res> {
  factory _$$BreezEvent_SyncedCopyWith(
          _$BreezEvent_Synced value, $Res Function(_$BreezEvent_Synced) then) =
      __$$BreezEvent_SyncedCopyWithImpl<$Res>;
}

/// @nodoc
class __$$BreezEvent_SyncedCopyWithImpl<$Res>
    extends _$BreezEventCopyWithImpl<$Res, _$BreezEvent_Synced>
    implements _$$BreezEvent_SyncedCopyWith<$Res> {
  __$$BreezEvent_SyncedCopyWithImpl(
      _$BreezEvent_Synced _value, $Res Function(_$BreezEvent_Synced) _then)
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
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$BreezEvent_Synced);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int block) newBlock,
    required TResult Function(InvoicePaidDetails details) invoicePaid,
    required TResult Function() synced,
  }) {
    return synced();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int block)? newBlock,
    TResult? Function(InvoicePaidDetails details)? invoicePaid,
    TResult? Function()? synced,
  }) {
    return synced?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int block)? newBlock,
    TResult Function(InvoicePaidDetails details)? invoicePaid,
    TResult Function()? synced,
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
  }) {
    return synced(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BreezEvent_NewBlock value)? newBlock,
    TResult? Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult? Function(BreezEvent_Synced value)? synced,
  }) {
    return synced?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BreezEvent_NewBlock value)? newBlock,
    TResult Function(BreezEvent_InvoicePaid value)? invoicePaid,
    TResult Function(BreezEvent_Synced value)? synced,
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
class _$InputTypeCopyWithImpl<$Res, $Val extends InputType>
    implements $InputTypeCopyWith<$Res> {
  _$InputTypeCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$InputType_BitcoinAddressCopyWith<$Res> {
  factory _$$InputType_BitcoinAddressCopyWith(_$InputType_BitcoinAddress value,
          $Res Function(_$InputType_BitcoinAddress) then) =
      __$$InputType_BitcoinAddressCopyWithImpl<$Res>;
  @useResult
  $Res call({BitcoinAddressData address});
}

/// @nodoc
class __$$InputType_BitcoinAddressCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_BitcoinAddress>
    implements _$$InputType_BitcoinAddressCopyWith<$Res> {
  __$$InputType_BitcoinAddressCopyWithImpl(_$InputType_BitcoinAddress _value,
      $Res Function(_$InputType_BitcoinAddress) _then)
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
  _$$InputType_BitcoinAddressCopyWith<_$InputType_BitcoinAddress>
      get copyWith =>
          __$$InputType_BitcoinAddressCopyWithImpl<_$InputType_BitcoinAddress>(
              this, _$identity);

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
  const factory InputType_BitcoinAddress(
      {required final BitcoinAddressData address}) = _$InputType_BitcoinAddress;

  BitcoinAddressData get address;
  @JsonKey(ignore: true)
  _$$InputType_BitcoinAddressCopyWith<_$InputType_BitcoinAddress>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_Bolt11CopyWith<$Res> {
  factory _$$InputType_Bolt11CopyWith(
          _$InputType_Bolt11 value, $Res Function(_$InputType_Bolt11) then) =
      __$$InputType_Bolt11CopyWithImpl<$Res>;
  @useResult
  $Res call({LNInvoice invoice});
}

/// @nodoc
class __$$InputType_Bolt11CopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_Bolt11>
    implements _$$InputType_Bolt11CopyWith<$Res> {
  __$$InputType_Bolt11CopyWithImpl(
      _$InputType_Bolt11 _value, $Res Function(_$InputType_Bolt11) _then)
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
  const factory InputType_Bolt11({required final LNInvoice invoice}) =
      _$InputType_Bolt11;

  LNInvoice get invoice;
  @JsonKey(ignore: true)
  _$$InputType_Bolt11CopyWith<_$InputType_Bolt11> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_NodeIdCopyWith<$Res> {
  factory _$$InputType_NodeIdCopyWith(
          _$InputType_NodeId value, $Res Function(_$InputType_NodeId) then) =
      __$$InputType_NodeIdCopyWithImpl<$Res>;
  @useResult
  $Res call({String nodeId});
}

/// @nodoc
class __$$InputType_NodeIdCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_NodeId>
    implements _$$InputType_NodeIdCopyWith<$Res> {
  __$$InputType_NodeIdCopyWithImpl(
      _$InputType_NodeId _value, $Res Function(_$InputType_NodeId) _then)
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
  const factory InputType_NodeId({required final String nodeId}) =
      _$InputType_NodeId;

  String get nodeId;
  @JsonKey(ignore: true)
  _$$InputType_NodeIdCopyWith<_$InputType_NodeId> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_UrlCopyWith<$Res> {
  factory _$$InputType_UrlCopyWith(
          _$InputType_Url value, $Res Function(_$InputType_Url) then) =
      __$$InputType_UrlCopyWithImpl<$Res>;
  @useResult
  $Res call({String url});
}

/// @nodoc
class __$$InputType_UrlCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_Url>
    implements _$$InputType_UrlCopyWith<$Res> {
  __$$InputType_UrlCopyWithImpl(
      _$InputType_Url _value, $Res Function(_$InputType_Url) _then)
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
  _$$InputType_UrlCopyWith<_$InputType_Url> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlPayCopyWith<$Res> {
  factory _$$InputType_LnUrlPayCopyWith(_$InputType_LnUrlPay value,
          $Res Function(_$InputType_LnUrlPay) then) =
      __$$InputType_LnUrlPayCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlPayRequestData data});
}

/// @nodoc
class __$$InputType_LnUrlPayCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlPay>
    implements _$$InputType_LnUrlPayCopyWith<$Res> {
  __$$InputType_LnUrlPayCopyWithImpl(
      _$InputType_LnUrlPay _value, $Res Function(_$InputType_LnUrlPay) _then)
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
      __$$InputType_LnUrlPayCopyWithImpl<_$InputType_LnUrlPay>(
          this, _$identity);

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
  const factory InputType_LnUrlPay({required final LnUrlPayRequestData data}) =
      _$InputType_LnUrlPay;

  LnUrlPayRequestData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlPayCopyWith<_$InputType_LnUrlPay> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlWithdrawCopyWith<$Res> {
  factory _$$InputType_LnUrlWithdrawCopyWith(_$InputType_LnUrlWithdraw value,
          $Res Function(_$InputType_LnUrlWithdraw) then) =
      __$$InputType_LnUrlWithdrawCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlWithdrawRequestData data});
}

/// @nodoc
class __$$InputType_LnUrlWithdrawCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlWithdraw>
    implements _$$InputType_LnUrlWithdrawCopyWith<$Res> {
  __$$InputType_LnUrlWithdrawCopyWithImpl(_$InputType_LnUrlWithdraw _value,
      $Res Function(_$InputType_LnUrlWithdraw) _then)
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
      __$$InputType_LnUrlWithdrawCopyWithImpl<_$InputType_LnUrlWithdraw>(
          this, _$identity);

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
  const factory InputType_LnUrlWithdraw(
          {required final LnUrlWithdrawRequestData data}) =
      _$InputType_LnUrlWithdraw;

  LnUrlWithdrawRequestData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlWithdrawCopyWith<_$InputType_LnUrlWithdraw> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlAuthCopyWith<$Res> {
  factory _$$InputType_LnUrlAuthCopyWith(_$InputType_LnUrlAuth value,
          $Res Function(_$InputType_LnUrlAuth) then) =
      __$$InputType_LnUrlAuthCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlAuthRequestData data});
}

/// @nodoc
class __$$InputType_LnUrlAuthCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlAuth>
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
      __$$InputType_LnUrlAuthCopyWithImpl<_$InputType_LnUrlAuth>(
          this, _$identity);

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
  const factory InputType_LnUrlAuth(
      {required final LnUrlAuthRequestData data}) = _$InputType_LnUrlAuth;

  LnUrlAuthRequestData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlAuthCopyWith<_$InputType_LnUrlAuth> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputType_LnUrlErrorCopyWith<$Res> {
  factory _$$InputType_LnUrlErrorCopyWith(_$InputType_LnUrlError value,
          $Res Function(_$InputType_LnUrlError) then) =
      __$$InputType_LnUrlErrorCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});
}

/// @nodoc
class __$$InputType_LnUrlErrorCopyWithImpl<$Res>
    extends _$InputTypeCopyWithImpl<$Res, _$InputType_LnUrlError>
    implements _$$InputType_LnUrlErrorCopyWith<$Res> {
  __$$InputType_LnUrlErrorCopyWithImpl(_$InputType_LnUrlError _value,
      $Res Function(_$InputType_LnUrlError) _then)
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
      __$$InputType_LnUrlErrorCopyWithImpl<_$InputType_LnUrlError>(
          this, _$identity);

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
  const factory InputType_LnUrlError({required final LnUrlErrorData data}) =
      _$InputType_LnUrlError;

  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$InputType_LnUrlErrorCopyWith<_$InputType_LnUrlError> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlPayResult {
  Object? get data => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SuccessAction? data) endpointSuccess,
    required TResult Function(LnUrlErrorData data) endpointError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SuccessAction? data)? endpointSuccess,
    TResult? Function(LnUrlErrorData data)? endpointError,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SuccessAction? data)? endpointSuccess,
    TResult Function(LnUrlErrorData data)? endpointError,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LnUrlPayResult_EndpointSuccess value)
        endpointSuccess,
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
  factory $LnUrlPayResultCopyWith(
          LnUrlPayResult value, $Res Function(LnUrlPayResult) then) =
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
          _$LnUrlPayResult_EndpointSuccess value,
          $Res Function(_$LnUrlPayResult_EndpointSuccess) then) =
      __$$LnUrlPayResult_EndpointSuccessCopyWithImpl<$Res>;
  @useResult
  $Res call({SuccessAction? data});

  $SuccessActionCopyWith<$Res>? get data;
}

/// @nodoc
class __$$LnUrlPayResult_EndpointSuccessCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointSuccess>
    implements _$$LnUrlPayResult_EndpointSuccessCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointSuccessCopyWithImpl(
      _$LnUrlPayResult_EndpointSuccess _value,
      $Res Function(_$LnUrlPayResult_EndpointSuccess) _then)
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
              as SuccessAction?,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $SuccessActionCopyWith<$Res>? get data {
    if (_value.data == null) {
      return null;
    }

    return $SuccessActionCopyWith<$Res>(_value.data!, (value) {
      return _then(_value.copyWith(data: value));
    });
  }
}

/// @nodoc

class _$LnUrlPayResult_EndpointSuccess
    implements LnUrlPayResult_EndpointSuccess {
  const _$LnUrlPayResult_EndpointSuccess({this.data});

  @override
  final SuccessAction? data;

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
  _$$LnUrlPayResult_EndpointSuccessCopyWith<_$LnUrlPayResult_EndpointSuccess>
      get copyWith => __$$LnUrlPayResult_EndpointSuccessCopyWithImpl<
          _$LnUrlPayResult_EndpointSuccess>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SuccessAction? data) endpointSuccess,
    required TResult Function(LnUrlErrorData data) endpointError,
  }) {
    return endpointSuccess(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SuccessAction? data)? endpointSuccess,
    TResult? Function(LnUrlErrorData data)? endpointError,
  }) {
    return endpointSuccess?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SuccessAction? data)? endpointSuccess,
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
    required TResult Function(LnUrlPayResult_EndpointSuccess value)
        endpointSuccess,
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
  const factory LnUrlPayResult_EndpointSuccess({final SuccessAction? data}) =
      _$LnUrlPayResult_EndpointSuccess;

  @override
  SuccessAction? get data;
  @JsonKey(ignore: true)
  _$$LnUrlPayResult_EndpointSuccessCopyWith<_$LnUrlPayResult_EndpointSuccess>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayResult_EndpointErrorCopyWith<$Res> {
  factory _$$LnUrlPayResult_EndpointErrorCopyWith(
          _$LnUrlPayResult_EndpointError value,
          $Res Function(_$LnUrlPayResult_EndpointError) then) =
      __$$LnUrlPayResult_EndpointErrorCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});
}

/// @nodoc
class __$$LnUrlPayResult_EndpointErrorCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointError>
    implements _$$LnUrlPayResult_EndpointErrorCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointErrorCopyWithImpl(
      _$LnUrlPayResult_EndpointError _value,
      $Res Function(_$LnUrlPayResult_EndpointError) _then)
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
  _$$LnUrlPayResult_EndpointErrorCopyWith<_$LnUrlPayResult_EndpointError>
      get copyWith => __$$LnUrlPayResult_EndpointErrorCopyWithImpl<
          _$LnUrlPayResult_EndpointError>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SuccessAction? data) endpointSuccess,
    required TResult Function(LnUrlErrorData data) endpointError,
  }) {
    return endpointError(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SuccessAction? data)? endpointSuccess,
    TResult? Function(LnUrlErrorData data)? endpointError,
  }) {
    return endpointError?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SuccessAction? data)? endpointSuccess,
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
    required TResult Function(LnUrlPayResult_EndpointSuccess value)
        endpointSuccess,
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
  const factory LnUrlPayResult_EndpointError(
      {required final LnUrlErrorData data}) = _$LnUrlPayResult_EndpointError;

  @override
  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlPayResult_EndpointErrorCopyWith<_$LnUrlPayResult_EndpointError>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LnUrlWithdrawCallbackStatus {
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
    required TResult Function(LnUrlWithdrawCallbackStatus_Ok value) ok,
    required TResult Function(LnUrlWithdrawCallbackStatus_ErrorStatus value)
        errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawCallbackStatus_Ok value)? ok,
    TResult? Function(LnUrlWithdrawCallbackStatus_ErrorStatus value)?
        errorStatus,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawCallbackStatus_Ok value)? ok,
    TResult Function(LnUrlWithdrawCallbackStatus_ErrorStatus value)?
        errorStatus,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlWithdrawCallbackStatusCopyWith<$Res> {
  factory $LnUrlWithdrawCallbackStatusCopyWith(
          LnUrlWithdrawCallbackStatus value,
          $Res Function(LnUrlWithdrawCallbackStatus) then) =
      _$LnUrlWithdrawCallbackStatusCopyWithImpl<$Res,
          LnUrlWithdrawCallbackStatus>;
}

/// @nodoc
class _$LnUrlWithdrawCallbackStatusCopyWithImpl<$Res,
        $Val extends LnUrlWithdrawCallbackStatus>
    implements $LnUrlWithdrawCallbackStatusCopyWith<$Res> {
  _$LnUrlWithdrawCallbackStatusCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$LnUrlWithdrawCallbackStatus_OkCopyWith<$Res> {
  factory _$$LnUrlWithdrawCallbackStatus_OkCopyWith(
          _$LnUrlWithdrawCallbackStatus_Ok value,
          $Res Function(_$LnUrlWithdrawCallbackStatus_Ok) then) =
      __$$LnUrlWithdrawCallbackStatus_OkCopyWithImpl<$Res>;
}

/// @nodoc
class __$$LnUrlWithdrawCallbackStatus_OkCopyWithImpl<$Res>
    extends _$LnUrlWithdrawCallbackStatusCopyWithImpl<$Res,
        _$LnUrlWithdrawCallbackStatus_Ok>
    implements _$$LnUrlWithdrawCallbackStatus_OkCopyWith<$Res> {
  __$$LnUrlWithdrawCallbackStatus_OkCopyWithImpl(
      _$LnUrlWithdrawCallbackStatus_Ok _value,
      $Res Function(_$LnUrlWithdrawCallbackStatus_Ok) _then)
      : super(_value, _then);
}

/// @nodoc

class _$LnUrlWithdrawCallbackStatus_Ok
    implements LnUrlWithdrawCallbackStatus_Ok {
  const _$LnUrlWithdrawCallbackStatus_Ok();

  @override
  String toString() {
    return 'LnUrlWithdrawCallbackStatus.ok()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawCallbackStatus_Ok);
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
    required TResult Function(LnUrlWithdrawCallbackStatus_Ok value) ok,
    required TResult Function(LnUrlWithdrawCallbackStatus_ErrorStatus value)
        errorStatus,
  }) {
    return ok(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawCallbackStatus_Ok value)? ok,
    TResult? Function(LnUrlWithdrawCallbackStatus_ErrorStatus value)?
        errorStatus,
  }) {
    return ok?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawCallbackStatus_Ok value)? ok,
    TResult Function(LnUrlWithdrawCallbackStatus_ErrorStatus value)?
        errorStatus,
    required TResult orElse(),
  }) {
    if (ok != null) {
      return ok(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawCallbackStatus_Ok
    implements LnUrlWithdrawCallbackStatus {
  const factory LnUrlWithdrawCallbackStatus_Ok() =
      _$LnUrlWithdrawCallbackStatus_Ok;
}

/// @nodoc
abstract class _$$LnUrlWithdrawCallbackStatus_ErrorStatusCopyWith<$Res> {
  factory _$$LnUrlWithdrawCallbackStatus_ErrorStatusCopyWith(
          _$LnUrlWithdrawCallbackStatus_ErrorStatus value,
          $Res Function(_$LnUrlWithdrawCallbackStatus_ErrorStatus) then) =
      __$$LnUrlWithdrawCallbackStatus_ErrorStatusCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});
}

/// @nodoc
class __$$LnUrlWithdrawCallbackStatus_ErrorStatusCopyWithImpl<$Res>
    extends _$LnUrlWithdrawCallbackStatusCopyWithImpl<$Res,
        _$LnUrlWithdrawCallbackStatus_ErrorStatus>
    implements _$$LnUrlWithdrawCallbackStatus_ErrorStatusCopyWith<$Res> {
  __$$LnUrlWithdrawCallbackStatus_ErrorStatusCopyWithImpl(
      _$LnUrlWithdrawCallbackStatus_ErrorStatus _value,
      $Res Function(_$LnUrlWithdrawCallbackStatus_ErrorStatus) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlWithdrawCallbackStatus_ErrorStatus(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }
}

/// @nodoc

class _$LnUrlWithdrawCallbackStatus_ErrorStatus
    implements LnUrlWithdrawCallbackStatus_ErrorStatus {
  const _$LnUrlWithdrawCallbackStatus_ErrorStatus({required this.data});

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'LnUrlWithdrawCallbackStatus.errorStatus(data: $data)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlWithdrawCallbackStatus_ErrorStatus &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlWithdrawCallbackStatus_ErrorStatusCopyWith<
          _$LnUrlWithdrawCallbackStatus_ErrorStatus>
      get copyWith => __$$LnUrlWithdrawCallbackStatus_ErrorStatusCopyWithImpl<
          _$LnUrlWithdrawCallbackStatus_ErrorStatus>(this, _$identity);

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
    required TResult Function(LnUrlWithdrawCallbackStatus_Ok value) ok,
    required TResult Function(LnUrlWithdrawCallbackStatus_ErrorStatus value)
        errorStatus,
  }) {
    return errorStatus(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LnUrlWithdrawCallbackStatus_Ok value)? ok,
    TResult? Function(LnUrlWithdrawCallbackStatus_ErrorStatus value)?
        errorStatus,
  }) {
    return errorStatus?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LnUrlWithdrawCallbackStatus_Ok value)? ok,
    TResult Function(LnUrlWithdrawCallbackStatus_ErrorStatus value)?
        errorStatus,
    required TResult orElse(),
  }) {
    if (errorStatus != null) {
      return errorStatus(this);
    }
    return orElse();
  }
}

abstract class LnUrlWithdrawCallbackStatus_ErrorStatus
    implements LnUrlWithdrawCallbackStatus {
  const factory LnUrlWithdrawCallbackStatus_ErrorStatus(
          {required final LnUrlErrorData data}) =
      _$LnUrlWithdrawCallbackStatus_ErrorStatus;

  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlWithdrawCallbackStatus_ErrorStatusCopyWith<
          _$LnUrlWithdrawCallbackStatus_ErrorStatus>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$PaymentDetails {
  Object get data => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnPaymentDetails data) ln,
    required TResult Function(ClosesChannelPaymentDetails data) closedChannel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnPaymentDetails data)? ln,
    TResult? Function(ClosesChannelPaymentDetails data)? closedChannel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnPaymentDetails data)? ln,
    TResult Function(ClosesChannelPaymentDetails data)? closedChannel,
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
  factory $PaymentDetailsCopyWith(
          PaymentDetails value, $Res Function(PaymentDetails) then) =
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
  factory _$$PaymentDetails_LnCopyWith(
          _$PaymentDetails_Ln value, $Res Function(_$PaymentDetails_Ln) then) =
      __$$PaymentDetails_LnCopyWithImpl<$Res>;
  @useResult
  $Res call({LnPaymentDetails data});
}

/// @nodoc
class __$$PaymentDetails_LnCopyWithImpl<$Res>
    extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_Ln>
    implements _$$PaymentDetails_LnCopyWith<$Res> {
  __$$PaymentDetails_LnCopyWithImpl(
      _$PaymentDetails_Ln _value, $Res Function(_$PaymentDetails_Ln) _then)
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
    required TResult Function(ClosesChannelPaymentDetails data) closedChannel,
  }) {
    return ln(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnPaymentDetails data)? ln,
    TResult? Function(ClosesChannelPaymentDetails data)? closedChannel,
  }) {
    return ln?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnPaymentDetails data)? ln,
    TResult Function(ClosesChannelPaymentDetails data)? closedChannel,
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
  const factory PaymentDetails_Ln({required final LnPaymentDetails data}) =
      _$PaymentDetails_Ln;

  @override
  LnPaymentDetails get data;
  @JsonKey(ignore: true)
  _$$PaymentDetails_LnCopyWith<_$PaymentDetails_Ln> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PaymentDetails_ClosedChannelCopyWith<$Res> {
  factory _$$PaymentDetails_ClosedChannelCopyWith(
          _$PaymentDetails_ClosedChannel value,
          $Res Function(_$PaymentDetails_ClosedChannel) then) =
      __$$PaymentDetails_ClosedChannelCopyWithImpl<$Res>;
  @useResult
  $Res call({ClosesChannelPaymentDetails data});
}

/// @nodoc
class __$$PaymentDetails_ClosedChannelCopyWithImpl<$Res>
    extends _$PaymentDetailsCopyWithImpl<$Res, _$PaymentDetails_ClosedChannel>
    implements _$$PaymentDetails_ClosedChannelCopyWith<$Res> {
  __$$PaymentDetails_ClosedChannelCopyWithImpl(
      _$PaymentDetails_ClosedChannel _value,
      $Res Function(_$PaymentDetails_ClosedChannel) _then)
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
              as ClosesChannelPaymentDetails,
    ));
  }
}

/// @nodoc

class _$PaymentDetails_ClosedChannel implements PaymentDetails_ClosedChannel {
  const _$PaymentDetails_ClosedChannel({required this.data});

  @override
  final ClosesChannelPaymentDetails data;

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
  _$$PaymentDetails_ClosedChannelCopyWith<_$PaymentDetails_ClosedChannel>
      get copyWith => __$$PaymentDetails_ClosedChannelCopyWithImpl<
          _$PaymentDetails_ClosedChannel>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(LnPaymentDetails data) ln,
    required TResult Function(ClosesChannelPaymentDetails data) closedChannel,
  }) {
    return closedChannel(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(LnPaymentDetails data)? ln,
    TResult? Function(ClosesChannelPaymentDetails data)? closedChannel,
  }) {
    return closedChannel?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(LnPaymentDetails data)? ln,
    TResult Function(ClosesChannelPaymentDetails data)? closedChannel,
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
  const factory PaymentDetails_ClosedChannel(
          {required final ClosesChannelPaymentDetails data}) =
      _$PaymentDetails_ClosedChannel;

  @override
  ClosesChannelPaymentDetails get data;
  @JsonKey(ignore: true)
  _$$PaymentDetails_ClosedChannelCopyWith<_$PaymentDetails_ClosedChannel>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SuccessAction {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MessageSuccessActionData field0) message,
    required TResult Function(UrlSuccessActionData field0) url,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(MessageSuccessActionData field0)? message,
    TResult? Function(UrlSuccessActionData field0)? url,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MessageSuccessActionData field0)? message,
    TResult Function(UrlSuccessActionData field0)? url,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SuccessAction_Message value) message,
    required TResult Function(SuccessAction_Url value) url,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SuccessAction_Message value)? message,
    TResult? Function(SuccessAction_Url value)? url,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SuccessAction_Message value)? message,
    TResult Function(SuccessAction_Url value)? url,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SuccessActionCopyWith<$Res> {
  factory $SuccessActionCopyWith(
          SuccessAction value, $Res Function(SuccessAction) then) =
      _$SuccessActionCopyWithImpl<$Res, SuccessAction>;
}

/// @nodoc
class _$SuccessActionCopyWithImpl<$Res, $Val extends SuccessAction>
    implements $SuccessActionCopyWith<$Res> {
  _$SuccessActionCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$SuccessAction_MessageCopyWith<$Res> {
  factory _$$SuccessAction_MessageCopyWith(_$SuccessAction_Message value,
          $Res Function(_$SuccessAction_Message) then) =
      __$$SuccessAction_MessageCopyWithImpl<$Res>;
  @useResult
  $Res call({MessageSuccessActionData field0});
}

/// @nodoc
class __$$SuccessAction_MessageCopyWithImpl<$Res>
    extends _$SuccessActionCopyWithImpl<$Res, _$SuccessAction_Message>
    implements _$$SuccessAction_MessageCopyWith<$Res> {
  __$$SuccessAction_MessageCopyWithImpl(_$SuccessAction_Message _value,
      $Res Function(_$SuccessAction_Message) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$SuccessAction_Message(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MessageSuccessActionData,
    ));
  }
}

/// @nodoc

class _$SuccessAction_Message implements SuccessAction_Message {
  const _$SuccessAction_Message(this.field0);

  @override
  final MessageSuccessActionData field0;

  @override
  String toString() {
    return 'SuccessAction.message(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SuccessAction_Message &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SuccessAction_MessageCopyWith<_$SuccessAction_Message> get copyWith =>
      __$$SuccessAction_MessageCopyWithImpl<_$SuccessAction_Message>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MessageSuccessActionData field0) message,
    required TResult Function(UrlSuccessActionData field0) url,
  }) {
    return message(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(MessageSuccessActionData field0)? message,
    TResult? Function(UrlSuccessActionData field0)? url,
  }) {
    return message?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MessageSuccessActionData field0)? message,
    TResult Function(UrlSuccessActionData field0)? url,
    required TResult orElse(),
  }) {
    if (message != null) {
      return message(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SuccessAction_Message value) message,
    required TResult Function(SuccessAction_Url value) url,
  }) {
    return message(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SuccessAction_Message value)? message,
    TResult? Function(SuccessAction_Url value)? url,
  }) {
    return message?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SuccessAction_Message value)? message,
    TResult Function(SuccessAction_Url value)? url,
    required TResult orElse(),
  }) {
    if (message != null) {
      return message(this);
    }
    return orElse();
  }
}

abstract class SuccessAction_Message implements SuccessAction {
  const factory SuccessAction_Message(final MessageSuccessActionData field0) =
      _$SuccessAction_Message;

  @override
  MessageSuccessActionData get field0;
  @JsonKey(ignore: true)
  _$$SuccessAction_MessageCopyWith<_$SuccessAction_Message> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SuccessAction_UrlCopyWith<$Res> {
  factory _$$SuccessAction_UrlCopyWith(
          _$SuccessAction_Url value, $Res Function(_$SuccessAction_Url) then) =
      __$$SuccessAction_UrlCopyWithImpl<$Res>;
  @useResult
  $Res call({UrlSuccessActionData field0});
}

/// @nodoc
class __$$SuccessAction_UrlCopyWithImpl<$Res>
    extends _$SuccessActionCopyWithImpl<$Res, _$SuccessAction_Url>
    implements _$$SuccessAction_UrlCopyWith<$Res> {
  __$$SuccessAction_UrlCopyWithImpl(
      _$SuccessAction_Url _value, $Res Function(_$SuccessAction_Url) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$SuccessAction_Url(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as UrlSuccessActionData,
    ));
  }
}

/// @nodoc

class _$SuccessAction_Url implements SuccessAction_Url {
  const _$SuccessAction_Url(this.field0);

  @override
  final UrlSuccessActionData field0;

  @override
  String toString() {
    return 'SuccessAction.url(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SuccessAction_Url &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SuccessAction_UrlCopyWith<_$SuccessAction_Url> get copyWith =>
      __$$SuccessAction_UrlCopyWithImpl<_$SuccessAction_Url>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MessageSuccessActionData field0) message,
    required TResult Function(UrlSuccessActionData field0) url,
  }) {
    return url(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(MessageSuccessActionData field0)? message,
    TResult? Function(UrlSuccessActionData field0)? url,
  }) {
    return url?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MessageSuccessActionData field0)? message,
    TResult Function(UrlSuccessActionData field0)? url,
    required TResult orElse(),
  }) {
    if (url != null) {
      return url(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SuccessAction_Message value) message,
    required TResult Function(SuccessAction_Url value) url,
  }) {
    return url(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SuccessAction_Message value)? message,
    TResult? Function(SuccessAction_Url value)? url,
  }) {
    return url?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SuccessAction_Message value)? message,
    TResult Function(SuccessAction_Url value)? url,
    required TResult orElse(),
  }) {
    if (url != null) {
      return url(this);
    }
    return orElse();
  }
}

abstract class SuccessAction_Url implements SuccessAction {
  const factory SuccessAction_Url(final UrlSuccessActionData field0) =
      _$SuccessAction_Url;

  @override
  UrlSuccessActionData get field0;
  @JsonKey(ignore: true)
  _$$SuccessAction_UrlCopyWith<_$SuccessAction_Url> get copyWith =>
      throw _privateConstructorUsedError;
}
