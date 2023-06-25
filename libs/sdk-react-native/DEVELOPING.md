# Setting up a development environment

The Breez SDK React Native plugin consumes the underlying Breez SDK from the following sources:

-   For iOS: The Breez SDK CocoaPod is used.
-   For Android: A loccaly-built version of the Breez SDK is used directly. This will soon be changed to consume the SDK fro our Maven repository.

When developing, it can be useful to work with a locally built  version of the Breez SDK instead of relying on what is published already on CocoaPods / Maven.
To do this, you first need to build the Breez SDK bindings locally and then point the plugin to make use of the locally built Breez SDK bindings.

### Prerequisites

Set the ANDROID_NDK_HOME env variable to your SDK home folder:

```
export ANDROID_NDK_HOME=<your android ndk directory>
```

### Building the bindings

On first usage you will need to run:

```
make init
```

Then to build and copy the Kotlin and Swift bindings:

```
make all
```

This will generate the following artifacts:

- iOS
	- `ios/bindings-swift/breez_sdkFFI.xcframework`
	- `ios/bindings-swift/Sources/BreezSDK/BreezSDK.swift`
	- `ios/libs/libbreez_sdk_core.a`

- Android
	- `android/src/main/java/com/breezsdk/breez_sdk.kt`
	- `android/src/main/jniLibs/arm64-v8a/libbreez_sdk_core.so`
	- `android/src/main/jniLibs/armeabi-v7a/libbreez_core_sdk.so`
	- `android/src/main/jniLibs/x86/libbreez_sdk_core.so`
	- `android/src/main/jniLibs/x86_64/libbreez_sdk_core.so`

### Using the locally built bindings

- For iOS:
	- Rename `BreezSDK.podspec.dev` to `BreezSDK.podspec`
	- Rename `breez_sdk.podspec` to `breez_sdk.podspec.prod`
