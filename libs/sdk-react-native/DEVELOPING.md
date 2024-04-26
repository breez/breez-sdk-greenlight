# Setting up a development environment

The Breez SDK React Native plugin consumes the underlying Breez SDK from the following sources:

-   For iOS: The Breez SDK Swift bindings are integrated via CocoaPods.
-   For Android: The Breez SDK Android bindings are integrated via Jitpack.

When developing, it can be useful to work with a locally built version of the Breez SDK instead of relying on what is published already on CocoaPods / Jitpack.
To do this, you first need to build the Breez SDK bindings locally and then point the plugin to make use of the locally built Breez SDK bindings.

All the following commands can be run in the `libs/sdk-react-native` directory.

## Prerequisites

Set the ANDROID_NDK_HOME env variable to your SDK home directory:
```
export ANDROID_NDK_HOME=<your android ndk directory>
```

To lint the result of the code generation ktlint, swiftformat and tslint need to be installed:
```bash
brew install kotlin ktlint swiftformat
yarn global add tslint typescript
```

On first usage you will need to run:
```bash
make init
```

## Building the bindings

Then to build and copy the Kotlin, Swift and React Native bindings into the React Native package run:
```bash
make all
```

This will generate the following artifacts:

- iOS
	- `ios/BreezSDKMapper.swift`
	- `ios/RNBreezSDK.m`
	- `ios/RNBreezSDK.swift`
	- `ios/bindings-swift/breez_sdkFFI.xcframework`
	- `ios/bindings-swift/Sources/BreezSDK/BreezSDK.swift`
- Android
	- `android/src/main/java/com/breezsdk/breez_sdk.kt`
	- `android/src/main/java/com/breezsdk/BreezSDKMapper.kt`
	- `android/src/main/java/com/breezsdk/BreezSDKModule.kt`
	- `android/src/main/jniLibs/arm64-v8a/libbreez_sdk_bindings.so`
	- `android/src/main/jniLibs/armeabi-v7a/libbreez_sdk_bindings.so`
	- `android/src/main/jniLibs/x86/libbreez_sdk_bindings.so`
	- `android/src/main/jniLibs/x86_64/libbreez_sdk_bindings.so`
- Typescript
	- `src/index.ts`

### Rebuilding for one platform only

You can also build for Android or iOS only by building the platform and React Native bindings, in that case run:
```bash
make android react-native
```
or
```bash
make ios react-native
```

### Rebuilding the React Native bindings

When there are changes to the UDL file in `libs/sdk-binding/src', you can rebuild the React Native bindings by running:
```bash
make react-native
```

## Using the locally built bindings

To use the locally built bindings instead of integrating them remotely, make the following changes:

- For iOS:
	- Rename the podspec files in `libs/sdk-react-native/`:
		- Rename `breez_sdk.podspec` to `breez_sdk.podspec.prod`
		- Rename `BreezSDK.podspec.dev` to `BreezSDK.podspec`
- For Android:
	- Comment out the following line from the dependencies section in `libs/sdk-react-native/android/build.gradle`:
		- `implementation("com.github.breez:breez-sdk:${getVersionFromNpmPackage()}") { exclude group:"net.java.dev.jna" }`

Reinstall the dependencies in the example project and run it.
It will now use the locally built bindings.

## Testing with the example app

To test locally built bindings in the example app, the npm dependencies need to be updated to use the local package.
In `libs/sdk-react-native/example/package.json` replace the current version with `file:../`:
```json
    "@breeztech/react-native-breez-sdk": "file:../",
```

Run the npm/yarn install to download dependences for both the react-native-breez-sdk package and the example app:
```bash
yarn bootstrap
```

Finally in the `libs/sdk-react-native/example/` directory start either the iOS or Android app:
```bash
yarn android
```
or for iOS:
```bash
yarn ios
```

## Troubleshooting

In case you get an error like: 
> java.lang.RuntimeException: Unable to load script. Make sure you're either running Metro (run 'npx react-native start') or that your bundle 'index.android.bundle' is packaged correctly for release. 

Then manually run `npx react-native start` in the example directory and reload the app.
