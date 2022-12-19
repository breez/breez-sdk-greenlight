# breez-sdk-bindings

This project provides bindings for breez-sdk to various languages.
Currently supported languges are kotlin & swift.
For kotlin & swift, we are using [uniffi](https://github.com/mozilla/uniffi-rs).

## prerequisite
* Install the uniffi-bindgen binary version 0.22.0 on your system using: ```cargo install --version 0.22.0 uniffi_bindgen```
* set the ANDROID_NDK_HOME env variable to your sdk home folder
* install cargo-ndk to provide simplified android build: ```cargo install cargo-ndk``` 

## Build

On first usage you will need to run:

```
make init
```

### Swift

```
make swift-ios
```

This will generate all the artifacts needed to for an iOS app to start writing code that uses breez sdk in swift.
All files are generated in the bindings/swift-ios folder.
We also provides the same binding for mac os by running the following command:

```
make swift-darwing
```
The above will generate the artifacts in the ffi/swift-darwin folder.

### Kotlin
```
make kotlin
```

This will build the android libraries for different platforms copy them to the ffi/kotlin/jniLibs folder.
In addition the kotlin binding code is generated and copied to the ffi/kotlin/breez-sdk folder.

## Test

In the tests directory there are some small scripts with some examples on how to use the sdk.
  * Kotlin `tests/bindings/test_breez-sdk.kts`
  * Swift `tests/bindings/test_breez-sdk.swift`  

If you want to try them out, you will need:

* The [Kotlin command-line tools](https://kotlinlang.org/docs/tutorials/command-line.html), particularly `kotlinc`.
* The [Java Native Access](https://github.com/java-native-access/jna#download) JAR downloaded and its path
  added to your `$CLASSPATH` environment variable.
* The [Swift command-line tools](https://swift.org/download/), particularly `swift`, `swiftc` and
  the `Foundation` package.

Then you can run ```cargo test```
