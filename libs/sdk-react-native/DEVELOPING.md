# Setting up a development environment

The Breez SDK React Native plugin consumes the underlying Breez SDK from the following sources:

-   For iOS: The Breez SDK Swift bindings are integrated via CocoaPods.
-   For Android: The Breez SDK Android bindings are integrated via Jitpack.

When developing, it can be useful to work with a locally built version of the Breez SDK instead of relying on what is published already on CocoaPods / Jitpack.
To do this, you first need to build the Breez SDK bindings locally and then point the plugin to make use of the locally built Breez SDK bindings.

## Prerequisites

Set the ANDROID_NDK_HOME env variable to your SDK home folder:
```
export ANDROID_NDK_HOME=<your android ndk directory>
```

To lint the result of the code generation ktlint, swiftformat and tslint need to be installed:
```bash
brew install kotlin ktlint@1.1.1 swiftformat
yarn global add tslint typescript
```

On first usage you will need to run:
```bash
make init
```

## Generating the bridging code

When there are changes to the UDL file in `libs/sdk-binding/src` the React Native bridging code needs to be regenerated:
```bash
make react-native-codegen
```

## Building the bindings

Then to build and copy the Kotlin and Swift bindings into the React Native plugin:
```bash
make all
```

This will generate the following artifacts:

- iOS
	- `ios/bindings-swift/breez_sdkFFI.xcframework`
	- `ios/bindings-swift/Sources/BreezSDK/BreezSDK.swift`

- Android
	- `android/src/main/java/com/breezsdk/breez_sdk.kt`
	- `android/src/main/jniLibs/arm64-v8a/libbreez_sdk_core.so`
	- `android/src/main/jniLibs/armeabi-v7a/libbreez_core_sdk.so`
	- `android/src/main/jniLibs/x86/libbreez_sdk_core.so`
	- `android/src/main/jniLibs/x86_64/libbreez_sdk_core.so`

## Using the locally built bindings

To use the locally built bindings instead of integrating them remotely:

- For iOS:
	- Rename `breez_sdk.podspec` to `breez_sdk.podspec.prod`
	- Rename `BreezSDK.podspec.dev` to `BreezSDK.podspec`
- For Android:
	- Remove the following line from the dependencies section in `android/build.gradle`:
		- `implementation("com.github.breez:breez-sdk:${getVersionFromNpmPackage()}") { exclude group:"net.java.dev.jna" }`

Reinstall the dependencies in the example project and run it.
It will now use the locally built bindings.

## Testing with the example app

To test locally built bindings in the example app, the npm dependencies need to be updated to use the local package.
In `example/package.json` replace the current version with `file:../`:
```json
    "@breeztech/react-native-breez-sdk": "file:../",
```

Run the npm/yarn install to download dependences for both the react-native-breez-sdk package and the example app:
```bash
yarn bootstrap
```

Finally in `example/` start either the iOS or Android app:
```bash
yarn android
```
or for iOS:
```bash
yarn ios
```
