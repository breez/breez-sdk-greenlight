// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'breez_services.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$BreezEvent {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BreezEvent);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'BreezEvent()';
}


}

/// @nodoc
class $BreezEventCopyWith<$Res>  {
$BreezEventCopyWith(BreezEvent _, $Res Function(BreezEvent) __);
}


/// @nodoc


class BreezEvent_NewBlock extends BreezEvent {
  const BreezEvent_NewBlock({required this.block}): super._();
  

 final  int block;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BreezEvent_NewBlockCopyWith<BreezEvent_NewBlock> get copyWith => _$BreezEvent_NewBlockCopyWithImpl<BreezEvent_NewBlock>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BreezEvent_NewBlock&&(identical(other.block, block) || other.block == block));
}


@override
int get hashCode => Object.hash(runtimeType,block);

@override
String toString() {
  return 'BreezEvent.newBlock(block: $block)';
}


}

/// @nodoc
abstract mixin class $BreezEvent_NewBlockCopyWith<$Res> implements $BreezEventCopyWith<$Res> {
  factory $BreezEvent_NewBlockCopyWith(BreezEvent_NewBlock value, $Res Function(BreezEvent_NewBlock) _then) = _$BreezEvent_NewBlockCopyWithImpl;
@useResult
$Res call({
 int block
});




}
/// @nodoc
class _$BreezEvent_NewBlockCopyWithImpl<$Res>
    implements $BreezEvent_NewBlockCopyWith<$Res> {
  _$BreezEvent_NewBlockCopyWithImpl(this._self, this._then);

  final BreezEvent_NewBlock _self;
  final $Res Function(BreezEvent_NewBlock) _then;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? block = null,}) {
  return _then(BreezEvent_NewBlock(
block: null == block ? _self.block : block // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

/// @nodoc


class BreezEvent_InvoicePaid extends BreezEvent {
  const BreezEvent_InvoicePaid({required this.details}): super._();
  

 final  InvoicePaidDetails details;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BreezEvent_InvoicePaidCopyWith<BreezEvent_InvoicePaid> get copyWith => _$BreezEvent_InvoicePaidCopyWithImpl<BreezEvent_InvoicePaid>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BreezEvent_InvoicePaid&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'BreezEvent.invoicePaid(details: $details)';
}


}

/// @nodoc
abstract mixin class $BreezEvent_InvoicePaidCopyWith<$Res> implements $BreezEventCopyWith<$Res> {
  factory $BreezEvent_InvoicePaidCopyWith(BreezEvent_InvoicePaid value, $Res Function(BreezEvent_InvoicePaid) _then) = _$BreezEvent_InvoicePaidCopyWithImpl;
@useResult
$Res call({
 InvoicePaidDetails details
});




}
/// @nodoc
class _$BreezEvent_InvoicePaidCopyWithImpl<$Res>
    implements $BreezEvent_InvoicePaidCopyWith<$Res> {
  _$BreezEvent_InvoicePaidCopyWithImpl(this._self, this._then);

  final BreezEvent_InvoicePaid _self;
  final $Res Function(BreezEvent_InvoicePaid) _then;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(BreezEvent_InvoicePaid(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as InvoicePaidDetails,
  ));
}


}

/// @nodoc


class BreezEvent_Synced extends BreezEvent {
  const BreezEvent_Synced(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BreezEvent_Synced);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'BreezEvent.synced()';
}


}




/// @nodoc


class BreezEvent_PaymentSucceed extends BreezEvent {
  const BreezEvent_PaymentSucceed({required this.details}): super._();
  

 final  Payment details;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BreezEvent_PaymentSucceedCopyWith<BreezEvent_PaymentSucceed> get copyWith => _$BreezEvent_PaymentSucceedCopyWithImpl<BreezEvent_PaymentSucceed>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BreezEvent_PaymentSucceed&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'BreezEvent.paymentSucceed(details: $details)';
}


}

/// @nodoc
abstract mixin class $BreezEvent_PaymentSucceedCopyWith<$Res> implements $BreezEventCopyWith<$Res> {
  factory $BreezEvent_PaymentSucceedCopyWith(BreezEvent_PaymentSucceed value, $Res Function(BreezEvent_PaymentSucceed) _then) = _$BreezEvent_PaymentSucceedCopyWithImpl;
@useResult
$Res call({
 Payment details
});




}
/// @nodoc
class _$BreezEvent_PaymentSucceedCopyWithImpl<$Res>
    implements $BreezEvent_PaymentSucceedCopyWith<$Res> {
  _$BreezEvent_PaymentSucceedCopyWithImpl(this._self, this._then);

  final BreezEvent_PaymentSucceed _self;
  final $Res Function(BreezEvent_PaymentSucceed) _then;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(BreezEvent_PaymentSucceed(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as Payment,
  ));
}


}

/// @nodoc


class BreezEvent_PaymentFailed extends BreezEvent {
  const BreezEvent_PaymentFailed({required this.details}): super._();
  

 final  PaymentFailedData details;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BreezEvent_PaymentFailedCopyWith<BreezEvent_PaymentFailed> get copyWith => _$BreezEvent_PaymentFailedCopyWithImpl<BreezEvent_PaymentFailed>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BreezEvent_PaymentFailed&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'BreezEvent.paymentFailed(details: $details)';
}


}

/// @nodoc
abstract mixin class $BreezEvent_PaymentFailedCopyWith<$Res> implements $BreezEventCopyWith<$Res> {
  factory $BreezEvent_PaymentFailedCopyWith(BreezEvent_PaymentFailed value, $Res Function(BreezEvent_PaymentFailed) _then) = _$BreezEvent_PaymentFailedCopyWithImpl;
@useResult
$Res call({
 PaymentFailedData details
});




}
/// @nodoc
class _$BreezEvent_PaymentFailedCopyWithImpl<$Res>
    implements $BreezEvent_PaymentFailedCopyWith<$Res> {
  _$BreezEvent_PaymentFailedCopyWithImpl(this._self, this._then);

  final BreezEvent_PaymentFailed _self;
  final $Res Function(BreezEvent_PaymentFailed) _then;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(BreezEvent_PaymentFailed(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as PaymentFailedData,
  ));
}


}

/// @nodoc


class BreezEvent_BackupStarted extends BreezEvent {
  const BreezEvent_BackupStarted(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BreezEvent_BackupStarted);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'BreezEvent.backupStarted()';
}


}




/// @nodoc


class BreezEvent_BackupSucceeded extends BreezEvent {
  const BreezEvent_BackupSucceeded(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BreezEvent_BackupSucceeded);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'BreezEvent.backupSucceeded()';
}


}




/// @nodoc


class BreezEvent_BackupFailed extends BreezEvent {
  const BreezEvent_BackupFailed({required this.details}): super._();
  

 final  BackupFailedData details;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BreezEvent_BackupFailedCopyWith<BreezEvent_BackupFailed> get copyWith => _$BreezEvent_BackupFailedCopyWithImpl<BreezEvent_BackupFailed>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BreezEvent_BackupFailed&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'BreezEvent.backupFailed(details: $details)';
}


}

/// @nodoc
abstract mixin class $BreezEvent_BackupFailedCopyWith<$Res> implements $BreezEventCopyWith<$Res> {
  factory $BreezEvent_BackupFailedCopyWith(BreezEvent_BackupFailed value, $Res Function(BreezEvent_BackupFailed) _then) = _$BreezEvent_BackupFailedCopyWithImpl;
@useResult
$Res call({
 BackupFailedData details
});




}
/// @nodoc
class _$BreezEvent_BackupFailedCopyWithImpl<$Res>
    implements $BreezEvent_BackupFailedCopyWith<$Res> {
  _$BreezEvent_BackupFailedCopyWithImpl(this._self, this._then);

  final BreezEvent_BackupFailed _self;
  final $Res Function(BreezEvent_BackupFailed) _then;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(BreezEvent_BackupFailed(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as BackupFailedData,
  ));
}


}

/// @nodoc


class BreezEvent_ReverseSwapUpdated extends BreezEvent {
  const BreezEvent_ReverseSwapUpdated({required this.details}): super._();
  

 final  ReverseSwapInfo details;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BreezEvent_ReverseSwapUpdatedCopyWith<BreezEvent_ReverseSwapUpdated> get copyWith => _$BreezEvent_ReverseSwapUpdatedCopyWithImpl<BreezEvent_ReverseSwapUpdated>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BreezEvent_ReverseSwapUpdated&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'BreezEvent.reverseSwapUpdated(details: $details)';
}


}

/// @nodoc
abstract mixin class $BreezEvent_ReverseSwapUpdatedCopyWith<$Res> implements $BreezEventCopyWith<$Res> {
  factory $BreezEvent_ReverseSwapUpdatedCopyWith(BreezEvent_ReverseSwapUpdated value, $Res Function(BreezEvent_ReverseSwapUpdated) _then) = _$BreezEvent_ReverseSwapUpdatedCopyWithImpl;
@useResult
$Res call({
 ReverseSwapInfo details
});




}
/// @nodoc
class _$BreezEvent_ReverseSwapUpdatedCopyWithImpl<$Res>
    implements $BreezEvent_ReverseSwapUpdatedCopyWith<$Res> {
  _$BreezEvent_ReverseSwapUpdatedCopyWithImpl(this._self, this._then);

  final BreezEvent_ReverseSwapUpdated _self;
  final $Res Function(BreezEvent_ReverseSwapUpdated) _then;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(BreezEvent_ReverseSwapUpdated(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as ReverseSwapInfo,
  ));
}


}

/// @nodoc


class BreezEvent_SwapUpdated extends BreezEvent {
  const BreezEvent_SwapUpdated({required this.details}): super._();
  

 final  SwapInfo details;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BreezEvent_SwapUpdatedCopyWith<BreezEvent_SwapUpdated> get copyWith => _$BreezEvent_SwapUpdatedCopyWithImpl<BreezEvent_SwapUpdated>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BreezEvent_SwapUpdated&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,details);

@override
String toString() {
  return 'BreezEvent.swapUpdated(details: $details)';
}


}

/// @nodoc
abstract mixin class $BreezEvent_SwapUpdatedCopyWith<$Res> implements $BreezEventCopyWith<$Res> {
  factory $BreezEvent_SwapUpdatedCopyWith(BreezEvent_SwapUpdated value, $Res Function(BreezEvent_SwapUpdated) _then) = _$BreezEvent_SwapUpdatedCopyWithImpl;
@useResult
$Res call({
 SwapInfo details
});




}
/// @nodoc
class _$BreezEvent_SwapUpdatedCopyWithImpl<$Res>
    implements $BreezEvent_SwapUpdatedCopyWith<$Res> {
  _$BreezEvent_SwapUpdatedCopyWithImpl(this._self, this._then);

  final BreezEvent_SwapUpdated _self;
  final $Res Function(BreezEvent_SwapUpdated) _then;

/// Create a copy of BreezEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? details = null,}) {
  return _then(BreezEvent_SwapUpdated(
details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as SwapInfo,
  ));
}


}

// dart format on
