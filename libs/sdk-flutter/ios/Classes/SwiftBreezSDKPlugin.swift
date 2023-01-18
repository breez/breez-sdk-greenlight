
import Flutter
import UIKit

public class SwiftBreezSDKPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "breez_sdk", binaryMessenger: registrar.messenger())
    let instance = SwiftBreezSDKPlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    result("iOS " + UIDevice.current.systemVersion)
  }

  public func dummyMethodToEnforceBundling() {
    let dummy = dummy_method_to_enforce_bundling();
    print(dummy)
    // ...
    // This code will force the bundler to use these functions, but will never be called
  }  
}
