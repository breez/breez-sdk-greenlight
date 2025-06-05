// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'binding.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$AesSuccessActionDataResult {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AesSuccessActionDataResult);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AesSuccessActionDataResult()';
}


}

/// @nodoc
class $AesSuccessActionDataResultCopyWith<$Res>  {
$AesSuccessActionDataResultCopyWith(AesSuccessActionDataResult _, $Res Function(AesSuccessActionDataResult) __);
}


/// @nodoc


class AesSuccessActionDataResult_Decrypted extends AesSuccessActionDataResult {
  const AesSuccessActionDataResult_Decrypted({required this.data}): super._();
  

 final  AesSuccessActionDataDecrypted data;

/// Create a copy of AesSuccessActionDataResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AesSuccessActionDataResult_DecryptedCopyWith<AesSuccessActionDataResult_Decrypted> get copyWith => _$AesSuccessActionDataResult_DecryptedCopyWithImpl<AesSuccessActionDataResult_Decrypted>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AesSuccessActionDataResult_Decrypted&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'AesSuccessActionDataResult.decrypted(data: $data)';
}


}

/// @nodoc
abstract mixin class $AesSuccessActionDataResult_DecryptedCopyWith<$Res> implements $AesSuccessActionDataResultCopyWith<$Res> {
  factory $AesSuccessActionDataResult_DecryptedCopyWith(AesSuccessActionDataResult_Decrypted value, $Res Function(AesSuccessActionDataResult_Decrypted) _then) = _$AesSuccessActionDataResult_DecryptedCopyWithImpl;
@useResult
$Res call({
 AesSuccessActionDataDecrypted data
});




}
/// @nodoc
class _$AesSuccessActionDataResult_DecryptedCopyWithImpl<$Res>
    implements $AesSuccessActionDataResult_DecryptedCopyWith<$Res> {
  _$AesSuccessActionDataResult_DecryptedCopyWithImpl(this._self, this._then);

  final AesSuccessActionDataResult_Decrypted _self;
  final $Res Function(AesSuccessActionDataResult_Decrypted) _then;

/// Create a copy of AesSuccessActionDataResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(AesSuccessActionDataResult_Decrypted(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as AesSuccessActionDataDecrypted,
  ));
}


}

/// @nodoc


class AesSuccessActionDataResult_ErrorStatus extends AesSuccessActionDataResult {
  const AesSuccessActionDataResult_ErrorStatus({required this.reason}): super._();
  

 final  String reason;

/// Create a copy of AesSuccessActionDataResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AesSuccessActionDataResult_ErrorStatusCopyWith<AesSuccessActionDataResult_ErrorStatus> get copyWith => _$AesSuccessActionDataResult_ErrorStatusCopyWithImpl<AesSuccessActionDataResult_ErrorStatus>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AesSuccessActionDataResult_ErrorStatus&&(identical(other.reason, reason) || other.reason == reason));
}


@override
int get hashCode => Object.hash(runtimeType,reason);

@override
String toString() {
  return 'AesSuccessActionDataResult.errorStatus(reason: $reason)';
}


}

/// @nodoc
abstract mixin class $AesSuccessActionDataResult_ErrorStatusCopyWith<$Res> implements $AesSuccessActionDataResultCopyWith<$Res> {
  factory $AesSuccessActionDataResult_ErrorStatusCopyWith(AesSuccessActionDataResult_ErrorStatus value, $Res Function(AesSuccessActionDataResult_ErrorStatus) _then) = _$AesSuccessActionDataResult_ErrorStatusCopyWithImpl;
@useResult
$Res call({
 String reason
});




}
/// @nodoc
class _$AesSuccessActionDataResult_ErrorStatusCopyWithImpl<$Res>
    implements $AesSuccessActionDataResult_ErrorStatusCopyWith<$Res> {
  _$AesSuccessActionDataResult_ErrorStatusCopyWithImpl(this._self, this._then);

  final AesSuccessActionDataResult_ErrorStatus _self;
  final $Res Function(AesSuccessActionDataResult_ErrorStatus) _then;

/// Create a copy of AesSuccessActionDataResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? reason = null,}) {
  return _then(AesSuccessActionDataResult_ErrorStatus(
reason: null == reason ? _self.reason : reason // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$InputType {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is InputType);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'InputType()';
}


}

/// @nodoc
class $InputTypeCopyWith<$Res>  {
$InputTypeCopyWith(InputType _, $Res Function(InputType) __);
}


/// @nodoc


class InputType_BitcoinAddress extends InputType {
  const InputType_BitcoinAddress({required this.address}): super._();
  

 final  BitcoinAddressData address;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$InputType_BitcoinAddressCopyWith<InputType_BitcoinAddress> get copyWith => _$InputType_BitcoinAddressCopyWithImpl<InputType_BitcoinAddress>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is InputType_BitcoinAddress&&(identical(other.address, address) || other.address == address));
}


@override
int get hashCode => Object.hash(runtimeType,address);

@override
String toString() {
  return 'InputType.bitcoinAddress(address: $address)';
}


}

/// @nodoc
abstract mixin class $InputType_BitcoinAddressCopyWith<$Res> implements $InputTypeCopyWith<$Res> {
  factory $InputType_BitcoinAddressCopyWith(InputType_BitcoinAddress value, $Res Function(InputType_BitcoinAddress) _then) = _$InputType_BitcoinAddressCopyWithImpl;
@useResult
$Res call({
 BitcoinAddressData address
});




}
/// @nodoc
class _$InputType_BitcoinAddressCopyWithImpl<$Res>
    implements $InputType_BitcoinAddressCopyWith<$Res> {
  _$InputType_BitcoinAddressCopyWithImpl(this._self, this._then);

  final InputType_BitcoinAddress _self;
  final $Res Function(InputType_BitcoinAddress) _then;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? address = null,}) {
  return _then(InputType_BitcoinAddress(
address: null == address ? _self.address : address // ignore: cast_nullable_to_non_nullable
as BitcoinAddressData,
  ));
}


}

/// @nodoc


class InputType_Bolt11 extends InputType {
  const InputType_Bolt11({required this.invoice}): super._();
  

 final  LNInvoice invoice;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$InputType_Bolt11CopyWith<InputType_Bolt11> get copyWith => _$InputType_Bolt11CopyWithImpl<InputType_Bolt11>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is InputType_Bolt11&&(identical(other.invoice, invoice) || other.invoice == invoice));
}


@override
int get hashCode => Object.hash(runtimeType,invoice);

@override
String toString() {
  return 'InputType.bolt11(invoice: $invoice)';
}


}

/// @nodoc
abstract mixin class $InputType_Bolt11CopyWith<$Res> implements $InputTypeCopyWith<$Res> {
  factory $InputType_Bolt11CopyWith(InputType_Bolt11 value, $Res Function(InputType_Bolt11) _then) = _$InputType_Bolt11CopyWithImpl;
@useResult
$Res call({
 LNInvoice invoice
});




}
/// @nodoc
class _$InputType_Bolt11CopyWithImpl<$Res>
    implements $InputType_Bolt11CopyWith<$Res> {
  _$InputType_Bolt11CopyWithImpl(this._self, this._then);

  final InputType_Bolt11 _self;
  final $Res Function(InputType_Bolt11) _then;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? invoice = null,}) {
  return _then(InputType_Bolt11(
invoice: null == invoice ? _self.invoice : invoice // ignore: cast_nullable_to_non_nullable
as LNInvoice,
  ));
}


}

/// @nodoc


class InputType_NodeId extends InputType {
  const InputType_NodeId({required this.nodeId}): super._();
  

 final  String nodeId;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$InputType_NodeIdCopyWith<InputType_NodeId> get copyWith => _$InputType_NodeIdCopyWithImpl<InputType_NodeId>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is InputType_NodeId&&(identical(other.nodeId, nodeId) || other.nodeId == nodeId));
}


@override
int get hashCode => Object.hash(runtimeType,nodeId);

@override
String toString() {
  return 'InputType.nodeId(nodeId: $nodeId)';
}


}

/// @nodoc
abstract mixin class $InputType_NodeIdCopyWith<$Res> implements $InputTypeCopyWith<$Res> {
  factory $InputType_NodeIdCopyWith(InputType_NodeId value, $Res Function(InputType_NodeId) _then) = _$InputType_NodeIdCopyWithImpl;
@useResult
$Res call({
 String nodeId
});




}
/// @nodoc
class _$InputType_NodeIdCopyWithImpl<$Res>
    implements $InputType_NodeIdCopyWith<$Res> {
  _$InputType_NodeIdCopyWithImpl(this._self, this._then);

  final InputType_NodeId _self;
  final $Res Function(InputType_NodeId) _then;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? nodeId = null,}) {
  return _then(InputType_NodeId(
nodeId: null == nodeId ? _self.nodeId : nodeId // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class InputType_Url extends InputType {
  const InputType_Url({required this.url}): super._();
  

 final  String url;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$InputType_UrlCopyWith<InputType_Url> get copyWith => _$InputType_UrlCopyWithImpl<InputType_Url>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is InputType_Url&&(identical(other.url, url) || other.url == url));
}


@override
int get hashCode => Object.hash(runtimeType,url);

@override
String toString() {
  return 'InputType.url(url: $url)';
}


}

/// @nodoc
abstract mixin class $InputType_UrlCopyWith<$Res> implements $InputTypeCopyWith<$Res> {
  factory $InputType_UrlCopyWith(InputType_Url value, $Res Function(InputType_Url) _then) = _$InputType_UrlCopyWithImpl;
@useResult
$Res call({
 String url
});




}
/// @nodoc
class _$InputType_UrlCopyWithImpl<$Res>
    implements $InputType_UrlCopyWith<$Res> {
  _$InputType_UrlCopyWithImpl(this._self, this._then);

  final InputType_Url _self;
  final $Res Function(InputType_Url) _then;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? url = null,}) {
  return _then(InputType_Url(
url: null == url ? _self.url : url // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class InputType_LnUrlPay extends InputType {
  const InputType_LnUrlPay({required this.data, this.bip353Address}): super._();
  

 final  LnUrlPayRequestData data;
 final  String? bip353Address;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$InputType_LnUrlPayCopyWith<InputType_LnUrlPay> get copyWith => _$InputType_LnUrlPayCopyWithImpl<InputType_LnUrlPay>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is InputType_LnUrlPay&&(identical(other.data, data) || other.data == data)&&(identical(other.bip353Address, bip353Address) || other.bip353Address == bip353Address));
}


@override
int get hashCode => Object.hash(runtimeType,data,bip353Address);

@override
String toString() {
  return 'InputType.lnUrlPay(data: $data, bip353Address: $bip353Address)';
}


}

/// @nodoc
abstract mixin class $InputType_LnUrlPayCopyWith<$Res> implements $InputTypeCopyWith<$Res> {
  factory $InputType_LnUrlPayCopyWith(InputType_LnUrlPay value, $Res Function(InputType_LnUrlPay) _then) = _$InputType_LnUrlPayCopyWithImpl;
@useResult
$Res call({
 LnUrlPayRequestData data, String? bip353Address
});




}
/// @nodoc
class _$InputType_LnUrlPayCopyWithImpl<$Res>
    implements $InputType_LnUrlPayCopyWith<$Res> {
  _$InputType_LnUrlPayCopyWithImpl(this._self, this._then);

  final InputType_LnUrlPay _self;
  final $Res Function(InputType_LnUrlPay) _then;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,Object? bip353Address = freezed,}) {
  return _then(InputType_LnUrlPay(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlPayRequestData,bip353Address: freezed == bip353Address ? _self.bip353Address : bip353Address // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

/// @nodoc


class InputType_LnUrlWithdraw extends InputType {
  const InputType_LnUrlWithdraw({required this.data}): super._();
  

 final  LnUrlWithdrawRequestData data;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$InputType_LnUrlWithdrawCopyWith<InputType_LnUrlWithdraw> get copyWith => _$InputType_LnUrlWithdrawCopyWithImpl<InputType_LnUrlWithdraw>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is InputType_LnUrlWithdraw&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'InputType.lnUrlWithdraw(data: $data)';
}


}

/// @nodoc
abstract mixin class $InputType_LnUrlWithdrawCopyWith<$Res> implements $InputTypeCopyWith<$Res> {
  factory $InputType_LnUrlWithdrawCopyWith(InputType_LnUrlWithdraw value, $Res Function(InputType_LnUrlWithdraw) _then) = _$InputType_LnUrlWithdrawCopyWithImpl;
@useResult
$Res call({
 LnUrlWithdrawRequestData data
});




}
/// @nodoc
class _$InputType_LnUrlWithdrawCopyWithImpl<$Res>
    implements $InputType_LnUrlWithdrawCopyWith<$Res> {
  _$InputType_LnUrlWithdrawCopyWithImpl(this._self, this._then);

  final InputType_LnUrlWithdraw _self;
  final $Res Function(InputType_LnUrlWithdraw) _then;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(InputType_LnUrlWithdraw(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlWithdrawRequestData,
  ));
}


}

/// @nodoc


class InputType_LnUrlAuth extends InputType {
  const InputType_LnUrlAuth({required this.data}): super._();
  

 final  LnUrlAuthRequestData data;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$InputType_LnUrlAuthCopyWith<InputType_LnUrlAuth> get copyWith => _$InputType_LnUrlAuthCopyWithImpl<InputType_LnUrlAuth>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is InputType_LnUrlAuth&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'InputType.lnUrlAuth(data: $data)';
}


}

/// @nodoc
abstract mixin class $InputType_LnUrlAuthCopyWith<$Res> implements $InputTypeCopyWith<$Res> {
  factory $InputType_LnUrlAuthCopyWith(InputType_LnUrlAuth value, $Res Function(InputType_LnUrlAuth) _then) = _$InputType_LnUrlAuthCopyWithImpl;
@useResult
$Res call({
 LnUrlAuthRequestData data
});




}
/// @nodoc
class _$InputType_LnUrlAuthCopyWithImpl<$Res>
    implements $InputType_LnUrlAuthCopyWith<$Res> {
  _$InputType_LnUrlAuthCopyWithImpl(this._self, this._then);

  final InputType_LnUrlAuth _self;
  final $Res Function(InputType_LnUrlAuth) _then;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(InputType_LnUrlAuth(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlAuthRequestData,
  ));
}


}

/// @nodoc


class InputType_LnUrlError extends InputType {
  const InputType_LnUrlError({required this.data}): super._();
  

 final  LnUrlErrorData data;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$InputType_LnUrlErrorCopyWith<InputType_LnUrlError> get copyWith => _$InputType_LnUrlErrorCopyWithImpl<InputType_LnUrlError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is InputType_LnUrlError&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'InputType.lnUrlError(data: $data)';
}


}

/// @nodoc
abstract mixin class $InputType_LnUrlErrorCopyWith<$Res> implements $InputTypeCopyWith<$Res> {
  factory $InputType_LnUrlErrorCopyWith(InputType_LnUrlError value, $Res Function(InputType_LnUrlError) _then) = _$InputType_LnUrlErrorCopyWithImpl;
@useResult
$Res call({
 LnUrlErrorData data
});




}
/// @nodoc
class _$InputType_LnUrlErrorCopyWithImpl<$Res>
    implements $InputType_LnUrlErrorCopyWith<$Res> {
  _$InputType_LnUrlErrorCopyWithImpl(this._self, this._then);

  final InputType_LnUrlError _self;
  final $Res Function(InputType_LnUrlError) _then;

/// Create a copy of InputType
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(InputType_LnUrlError(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlErrorData,
  ));
}


}

/// @nodoc
mixin _$LnUrlCallbackStatus {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlCallbackStatus);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LnUrlCallbackStatus()';
}


}

/// @nodoc
class $LnUrlCallbackStatusCopyWith<$Res>  {
$LnUrlCallbackStatusCopyWith(LnUrlCallbackStatus _, $Res Function(LnUrlCallbackStatus) __);
}


/// @nodoc


class LnUrlCallbackStatus_Ok extends LnUrlCallbackStatus {
  const LnUrlCallbackStatus_Ok(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlCallbackStatus_Ok);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LnUrlCallbackStatus.ok()';
}


}




/// @nodoc


class LnUrlCallbackStatus_ErrorStatus extends LnUrlCallbackStatus {
  const LnUrlCallbackStatus_ErrorStatus({required this.data}): super._();
  

 final  LnUrlErrorData data;

/// Create a copy of LnUrlCallbackStatus
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LnUrlCallbackStatus_ErrorStatusCopyWith<LnUrlCallbackStatus_ErrorStatus> get copyWith => _$LnUrlCallbackStatus_ErrorStatusCopyWithImpl<LnUrlCallbackStatus_ErrorStatus>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlCallbackStatus_ErrorStatus&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'LnUrlCallbackStatus.errorStatus(data: $data)';
}


}

/// @nodoc
abstract mixin class $LnUrlCallbackStatus_ErrorStatusCopyWith<$Res> implements $LnUrlCallbackStatusCopyWith<$Res> {
  factory $LnUrlCallbackStatus_ErrorStatusCopyWith(LnUrlCallbackStatus_ErrorStatus value, $Res Function(LnUrlCallbackStatus_ErrorStatus) _then) = _$LnUrlCallbackStatus_ErrorStatusCopyWithImpl;
@useResult
$Res call({
 LnUrlErrorData data
});




}
/// @nodoc
class _$LnUrlCallbackStatus_ErrorStatusCopyWithImpl<$Res>
    implements $LnUrlCallbackStatus_ErrorStatusCopyWith<$Res> {
  _$LnUrlCallbackStatus_ErrorStatusCopyWithImpl(this._self, this._then);

  final LnUrlCallbackStatus_ErrorStatus _self;
  final $Res Function(LnUrlCallbackStatus_ErrorStatus) _then;

/// Create a copy of LnUrlCallbackStatus
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(LnUrlCallbackStatus_ErrorStatus(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlErrorData,
  ));
}


}

/// @nodoc
mixin _$LnUrlWithdrawResult {

 Object get data;



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlWithdrawResult&&const DeepCollectionEquality().equals(other.data, data));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(data));

@override
String toString() {
  return 'LnUrlWithdrawResult(data: $data)';
}


}

/// @nodoc
class $LnUrlWithdrawResultCopyWith<$Res>  {
$LnUrlWithdrawResultCopyWith(LnUrlWithdrawResult _, $Res Function(LnUrlWithdrawResult) __);
}


/// @nodoc


class LnUrlWithdrawResult_Ok extends LnUrlWithdrawResult {
  const LnUrlWithdrawResult_Ok({required this.data}): super._();
  

@override final  LnUrlWithdrawSuccessData data;

/// Create a copy of LnUrlWithdrawResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LnUrlWithdrawResult_OkCopyWith<LnUrlWithdrawResult_Ok> get copyWith => _$LnUrlWithdrawResult_OkCopyWithImpl<LnUrlWithdrawResult_Ok>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlWithdrawResult_Ok&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'LnUrlWithdrawResult.ok(data: $data)';
}


}

/// @nodoc
abstract mixin class $LnUrlWithdrawResult_OkCopyWith<$Res> implements $LnUrlWithdrawResultCopyWith<$Res> {
  factory $LnUrlWithdrawResult_OkCopyWith(LnUrlWithdrawResult_Ok value, $Res Function(LnUrlWithdrawResult_Ok) _then) = _$LnUrlWithdrawResult_OkCopyWithImpl;
@useResult
$Res call({
 LnUrlWithdrawSuccessData data
});




}
/// @nodoc
class _$LnUrlWithdrawResult_OkCopyWithImpl<$Res>
    implements $LnUrlWithdrawResult_OkCopyWith<$Res> {
  _$LnUrlWithdrawResult_OkCopyWithImpl(this._self, this._then);

  final LnUrlWithdrawResult_Ok _self;
  final $Res Function(LnUrlWithdrawResult_Ok) _then;

/// Create a copy of LnUrlWithdrawResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(LnUrlWithdrawResult_Ok(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlWithdrawSuccessData,
  ));
}


}

/// @nodoc


class LnUrlWithdrawResult_Timeout extends LnUrlWithdrawResult {
  const LnUrlWithdrawResult_Timeout({required this.data}): super._();
  

@override final  LnUrlWithdrawSuccessData data;

/// Create a copy of LnUrlWithdrawResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LnUrlWithdrawResult_TimeoutCopyWith<LnUrlWithdrawResult_Timeout> get copyWith => _$LnUrlWithdrawResult_TimeoutCopyWithImpl<LnUrlWithdrawResult_Timeout>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlWithdrawResult_Timeout&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'LnUrlWithdrawResult.timeout(data: $data)';
}


}

/// @nodoc
abstract mixin class $LnUrlWithdrawResult_TimeoutCopyWith<$Res> implements $LnUrlWithdrawResultCopyWith<$Res> {
  factory $LnUrlWithdrawResult_TimeoutCopyWith(LnUrlWithdrawResult_Timeout value, $Res Function(LnUrlWithdrawResult_Timeout) _then) = _$LnUrlWithdrawResult_TimeoutCopyWithImpl;
@useResult
$Res call({
 LnUrlWithdrawSuccessData data
});




}
/// @nodoc
class _$LnUrlWithdrawResult_TimeoutCopyWithImpl<$Res>
    implements $LnUrlWithdrawResult_TimeoutCopyWith<$Res> {
  _$LnUrlWithdrawResult_TimeoutCopyWithImpl(this._self, this._then);

  final LnUrlWithdrawResult_Timeout _self;
  final $Res Function(LnUrlWithdrawResult_Timeout) _then;

/// Create a copy of LnUrlWithdrawResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(LnUrlWithdrawResult_Timeout(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlWithdrawSuccessData,
  ));
}


}

/// @nodoc


class LnUrlWithdrawResult_ErrorStatus extends LnUrlWithdrawResult {
  const LnUrlWithdrawResult_ErrorStatus({required this.data}): super._();
  

@override final  LnUrlErrorData data;

/// Create a copy of LnUrlWithdrawResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LnUrlWithdrawResult_ErrorStatusCopyWith<LnUrlWithdrawResult_ErrorStatus> get copyWith => _$LnUrlWithdrawResult_ErrorStatusCopyWithImpl<LnUrlWithdrawResult_ErrorStatus>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LnUrlWithdrawResult_ErrorStatus&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'LnUrlWithdrawResult.errorStatus(data: $data)';
}


}

/// @nodoc
abstract mixin class $LnUrlWithdrawResult_ErrorStatusCopyWith<$Res> implements $LnUrlWithdrawResultCopyWith<$Res> {
  factory $LnUrlWithdrawResult_ErrorStatusCopyWith(LnUrlWithdrawResult_ErrorStatus value, $Res Function(LnUrlWithdrawResult_ErrorStatus) _then) = _$LnUrlWithdrawResult_ErrorStatusCopyWithImpl;
@useResult
$Res call({
 LnUrlErrorData data
});




}
/// @nodoc
class _$LnUrlWithdrawResult_ErrorStatusCopyWithImpl<$Res>
    implements $LnUrlWithdrawResult_ErrorStatusCopyWith<$Res> {
  _$LnUrlWithdrawResult_ErrorStatusCopyWithImpl(this._self, this._then);

  final LnUrlWithdrawResult_ErrorStatus _self;
  final $Res Function(LnUrlWithdrawResult_ErrorStatus) _then;

/// Create a copy of LnUrlWithdrawResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(LnUrlWithdrawResult_ErrorStatus(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as LnUrlErrorData,
  ));
}


}

/// @nodoc
mixin _$SuccessActionProcessed {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SuccessActionProcessed);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SuccessActionProcessed()';
}


}

/// @nodoc
class $SuccessActionProcessedCopyWith<$Res>  {
$SuccessActionProcessedCopyWith(SuccessActionProcessed _, $Res Function(SuccessActionProcessed) __);
}


/// @nodoc


class SuccessActionProcessed_Aes extends SuccessActionProcessed {
  const SuccessActionProcessed_Aes({required this.result}): super._();
  

 final  AesSuccessActionDataResult result;

/// Create a copy of SuccessActionProcessed
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SuccessActionProcessed_AesCopyWith<SuccessActionProcessed_Aes> get copyWith => _$SuccessActionProcessed_AesCopyWithImpl<SuccessActionProcessed_Aes>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SuccessActionProcessed_Aes&&(identical(other.result, result) || other.result == result));
}


@override
int get hashCode => Object.hash(runtimeType,result);

@override
String toString() {
  return 'SuccessActionProcessed.aes(result: $result)';
}


}

/// @nodoc
abstract mixin class $SuccessActionProcessed_AesCopyWith<$Res> implements $SuccessActionProcessedCopyWith<$Res> {
  factory $SuccessActionProcessed_AesCopyWith(SuccessActionProcessed_Aes value, $Res Function(SuccessActionProcessed_Aes) _then) = _$SuccessActionProcessed_AesCopyWithImpl;
@useResult
$Res call({
 AesSuccessActionDataResult result
});


$AesSuccessActionDataResultCopyWith<$Res> get result;

}
/// @nodoc
class _$SuccessActionProcessed_AesCopyWithImpl<$Res>
    implements $SuccessActionProcessed_AesCopyWith<$Res> {
  _$SuccessActionProcessed_AesCopyWithImpl(this._self, this._then);

  final SuccessActionProcessed_Aes _self;
  final $Res Function(SuccessActionProcessed_Aes) _then;

/// Create a copy of SuccessActionProcessed
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? result = null,}) {
  return _then(SuccessActionProcessed_Aes(
result: null == result ? _self.result : result // ignore: cast_nullable_to_non_nullable
as AesSuccessActionDataResult,
  ));
}

/// Create a copy of SuccessActionProcessed
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$AesSuccessActionDataResultCopyWith<$Res> get result {
  
  return $AesSuccessActionDataResultCopyWith<$Res>(_self.result, (value) {
    return _then(_self.copyWith(result: value));
  });
}
}

/// @nodoc


class SuccessActionProcessed_Message extends SuccessActionProcessed {
  const SuccessActionProcessed_Message({required this.data}): super._();
  

 final  MessageSuccessActionData data;

/// Create a copy of SuccessActionProcessed
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SuccessActionProcessed_MessageCopyWith<SuccessActionProcessed_Message> get copyWith => _$SuccessActionProcessed_MessageCopyWithImpl<SuccessActionProcessed_Message>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SuccessActionProcessed_Message&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'SuccessActionProcessed.message(data: $data)';
}


}

/// @nodoc
abstract mixin class $SuccessActionProcessed_MessageCopyWith<$Res> implements $SuccessActionProcessedCopyWith<$Res> {
  factory $SuccessActionProcessed_MessageCopyWith(SuccessActionProcessed_Message value, $Res Function(SuccessActionProcessed_Message) _then) = _$SuccessActionProcessed_MessageCopyWithImpl;
@useResult
$Res call({
 MessageSuccessActionData data
});




}
/// @nodoc
class _$SuccessActionProcessed_MessageCopyWithImpl<$Res>
    implements $SuccessActionProcessed_MessageCopyWith<$Res> {
  _$SuccessActionProcessed_MessageCopyWithImpl(this._self, this._then);

  final SuccessActionProcessed_Message _self;
  final $Res Function(SuccessActionProcessed_Message) _then;

/// Create a copy of SuccessActionProcessed
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(SuccessActionProcessed_Message(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as MessageSuccessActionData,
  ));
}


}

/// @nodoc


class SuccessActionProcessed_Url extends SuccessActionProcessed {
  const SuccessActionProcessed_Url({required this.data}): super._();
  

 final  UrlSuccessActionData data;

/// Create a copy of SuccessActionProcessed
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SuccessActionProcessed_UrlCopyWith<SuccessActionProcessed_Url> get copyWith => _$SuccessActionProcessed_UrlCopyWithImpl<SuccessActionProcessed_Url>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SuccessActionProcessed_Url&&(identical(other.data, data) || other.data == data));
}


@override
int get hashCode => Object.hash(runtimeType,data);

@override
String toString() {
  return 'SuccessActionProcessed.url(data: $data)';
}


}

/// @nodoc
abstract mixin class $SuccessActionProcessed_UrlCopyWith<$Res> implements $SuccessActionProcessedCopyWith<$Res> {
  factory $SuccessActionProcessed_UrlCopyWith(SuccessActionProcessed_Url value, $Res Function(SuccessActionProcessed_Url) _then) = _$SuccessActionProcessed_UrlCopyWithImpl;
@useResult
$Res call({
 UrlSuccessActionData data
});




}
/// @nodoc
class _$SuccessActionProcessed_UrlCopyWithImpl<$Res>
    implements $SuccessActionProcessed_UrlCopyWith<$Res> {
  _$SuccessActionProcessed_UrlCopyWithImpl(this._self, this._then);

  final SuccessActionProcessed_Url _self;
  final $Res Function(SuccessActionProcessed_Url) _then;

/// Create a copy of SuccessActionProcessed
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? data = null,}) {
  return _then(SuccessActionProcessed_Url(
data: null == data ? _self.data : data // ignore: cast_nullable_to_non_nullable
as UrlSuccessActionData,
  ));
}


}

// dart format on
