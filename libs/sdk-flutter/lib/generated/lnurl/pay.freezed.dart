// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'pay.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$LnUrlPayResult {

 Object get data;



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlPayResult&&const DeepCollectionEquality().equals(other.data, data));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(data));

@override
String toString() {
  return 'LnUrlPayResult(data: $data)';
}


}

/// @nodoc
class $LnUrlPayResultCopyWith<$Res>  {
$LnUrlPayResultCopyWith(LnUrlPayResult _, $Res Function(LnUrlPayResult) __);
}


/// @nodoc


class LnUrlPayResult_EndpointSuccess extends LnUrlPayResult {
  const LnUrlPayResult_EndpointSuccess({required this.data}): super._();
  

@override final  LnUrlPaySuccessData data;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LnUrlPayResult_EndpointSuccessCopyWith<LnUrlPayResult_EndpointSuccess> get copyWith => _$LnUrlPayResult_EndpointSuccessCopyWithImpl<LnUrlPayResult_EndpointSuccess>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlPayResult_EndpointSuccess&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'LnUrlPayResult.endpointSuccess(data: $data)';
}


}

/// @nodoc
abstract mixin class $LnUrlPayResult_EndpointSuccessCopyWith<$Res> implements $LnUrlPayResultCopyWith<$Res> {
  factory $LnUrlPayResult_EndpointSuccessCopyWith(LnUrlPayResult_EndpointSuccess value, $Res Function(LnUrlPayResult_EndpointSuccess) _then) = _$LnUrlPayResult_EndpointSuccessCopyWithImpl;
@useResult
$Res call({
 LnUrlPaySuccessData data
});




}
/// @nodoc
class _$LnUrlPayResult_EndpointSuccessCopyWithImpl<$Res>
    implements $LnUrlPayResult_EndpointSuccessCopyWith<$Res> {
  _$LnUrlPayResult_EndpointSuccessCopyWithImpl(this._self, this._then);

  final LnUrlPayResult_EndpointSuccess _self;
  final $Res Function(LnUrlPayResult_EndpointSuccess) _then;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(LnUrlPayResult_EndpointSuccess(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlPaySuccessData,
  ));
}


}

/// @nodoc


class LnUrlPayResult_EndpointError extends LnUrlPayResult {
  const LnUrlPayResult_EndpointError({required this.data}): super._();
  

@override final  LnUrlErrorData data;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LnUrlPayResult_EndpointErrorCopyWith<LnUrlPayResult_EndpointError> get copyWith => _$LnUrlPayResult_EndpointErrorCopyWithImpl<LnUrlPayResult_EndpointError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlPayResult_EndpointError&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'LnUrlPayResult.endpointError(data: $data)';
}


}

/// @nodoc
abstract mixin class $LnUrlPayResult_EndpointErrorCopyWith<$Res> implements $LnUrlPayResultCopyWith<$Res> {
  factory $LnUrlPayResult_EndpointErrorCopyWith(LnUrlPayResult_EndpointError value, $Res Function(LnUrlPayResult_EndpointError) _then) = _$LnUrlPayResult_EndpointErrorCopyWithImpl;
@useResult
$Res call({
 LnUrlErrorData data
});




}
/// @nodoc
class _$LnUrlPayResult_EndpointErrorCopyWithImpl<$Res>
    implements $LnUrlPayResult_EndpointErrorCopyWith<$Res> {
  _$LnUrlPayResult_EndpointErrorCopyWithImpl(this._self, this._then);

  final LnUrlPayResult_EndpointError _self;
  final $Res Function(LnUrlPayResult_EndpointError) _then;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(LnUrlPayResult_EndpointError(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlErrorData,
  ));
}


}

/// @nodoc


class LnUrlPayResult_PayError extends LnUrlPayResult {
  const LnUrlPayResult_PayError({required this.data}): super._();
  

@override final  LnUrlPayErrorData data;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LnUrlPayResult_PayErrorCopyWith<LnUrlPayResult_PayError> get copyWith => _$LnUrlPayResult_PayErrorCopyWithImpl<LnUrlPayResult_PayError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlPayResult_PayError&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'LnUrlPayResult.payError(data: $data)';
}


}

/// @nodoc
abstract mixin class $LnUrlPayResult_PayErrorCopyWith<$Res> implements $LnUrlPayResultCopyWith<$Res> {
  factory $LnUrlPayResult_PayErrorCopyWith(LnUrlPayResult_PayError value, $Res Function(LnUrlPayResult_PayError) _then) = _$LnUrlPayResult_PayErrorCopyWithImpl;
@useResult
$Res call({
 LnUrlPayErrorData data
});




}
/// @nodoc
class _$LnUrlPayResult_PayErrorCopyWithImpl<$Res>
    implements $LnUrlPayResult_PayErrorCopyWith<$Res> {
  _$LnUrlPayResult_PayErrorCopyWithImpl(this._self, this._then);

  final LnUrlPayResult_PayError _self;
  final $Res Function(LnUrlPayResult_PayError) _then;

/// Create a copy of LnUrlPayResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(LnUrlPayResult_PayError(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlPayErrorData,
  ));
}


}

// dart format on
