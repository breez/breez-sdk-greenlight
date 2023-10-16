import 'dart:ffi';
import 'dart:io';
import 'bridge_generated.dart';

BreezSdkCore? _breezSDK;

const _libName = "libbreez_sdk_bindings.so";

class UnsupportedPlatform implements Exception {
  UnsupportedPlatform(String s);
}

BreezSdkCore getNativeToolkit() {
  if (_breezSDK == null) {
    if (Platform.isAndroid || Platform.isLinux) {
      // On Linux the lib needs to be in LD_LIBRARY_PATH or working directory
      _breezSDK = BreezSdkCoreImpl(DynamicLibrary.open(_libName));
    } else if (Platform.isIOS || Platform.isMacOS) {
      // iOS and macOS are statically linked
      _breezSDK = BreezSdkCoreImpl(DynamicLibrary.process());
    } else {
      throw UnsupportedPlatform('${Platform.operatingSystem} is not yet supported!');
    }
  }
  return _breezSDK!;
}
