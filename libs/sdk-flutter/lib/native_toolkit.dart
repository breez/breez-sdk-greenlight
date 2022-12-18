import 'dart:ffi';
import 'dart:io';
import 'bridge_generated.dart';

BreezSdkCore? _breezSDK;

BreezSdkCore getNativeToolkit() {
  if (_breezSDK == null) {
    final DynamicLibrary lib = Platform.isAndroid
      ? DynamicLibrary.open("libbreez_sdk_core.so")   // Load the dynamic library on Android
      : DynamicLibrary.process();
    _breezSDK = BreezSdkCoreImpl(lib);
  }
  return _breezSDK!;
}