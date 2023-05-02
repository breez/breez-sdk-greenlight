## Build

### Prerequisites
* set the ANDROID_NDK_HOME env variable to your sdk home folder
```
export ANDROID_NDK_HOME=<your android ndk directory>
```

### Building the plugin
On first usage you will need to run:
```
make init
```
Then to build and copy the kotlin and ios libraries:
```
make all
```

### Generated artifacts
* Android libraries
 >* android/src/main/java/com/breezsdk/breez_sdk.kt
 >* android/src/main/jniLibs/arm64-v8a/libbreez_sdk_core.so
 >* android/src/main/jniLibs/armeabi-v7a/libbreez_core_sdk.so
 >* android/src/main/jniLibs/x86/libbreez_sdk_core.so
 >* android/src/main/jniLibs/x86_64/libbreez_sdk_core.so
* iOS library
 >* ios/include/breez_sdkFFI.h
 >* ios/breez_sdk.swift
 >* ios/libs/libbreez_sdk_core.a

### Publish
When publishing, make sure the version number in `package.json` is updated. Then login to npm:
```
npm login --@scope=@breeztech
```
Then publish:
```
npm publish --access public
```