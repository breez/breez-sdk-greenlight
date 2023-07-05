# breez-sdk-bindings

This project provides bindings for breez-sdk to various languages.
Currently supported languages are Kotlin, Swift, C# and Go for which we are using [UniFFI](https://github.com/mozilla/uniffi-rs) to generate the bindings.

## Prerequisites
* When building for Android:
  * Set the ANDROID_NDK_HOME env variable to your sdk home folder
  * Install `cargo-ndk` to provide simplified android build: ```cargo install cargo-ndk```

## Build

On first usage you will need to run:

```
make init
```

### Swift

For most users, we recommend using our official Swift package: [breez/breez-sdk-swift](https://github.com/breez/breez-sdk-swift).

If you want to compile from source or need more options, read on.

#### Swift Module

These commands will build libraries for different architectures in `../target/` and generate Swift bindings as well as Swift module artifacts in `ffi/swift-ios/` and `ffi/swift-darwin/` respectively:

```
make swift-ios
```

```
make swift-darwin
```

#### Swift Package

This command will produce a fully configured Swift Package in `bindings-swift/`.
See [Adding package dependencies to your app](https://developer.apple.com/documentation/xcode/adding-package-dependencies-to-your-app) in Apple's docs for more information on how to integrate such a package into your project.

```
make bindings-swift
```

### Kotlin

For most users, we recommend integrating the Breez SDK as Gradle dependency from [our Maven repository](https://mvn.breez.technology/releases).

To do so, add the following to your Gradle dependencies:

``` groovy
repositories {
  maven {
      url("https://mvn.breez.technology/releases")
  }
}

dependencies {
  implementation("breez_sdk:bindings-android:<version>")
}
```

You can then import and use the Breez SDK in your app:

``` kotlin
import breez_sdk.*
```

If you want to compile from source or need more options, read on.

#### Libraries and Bindings

This command will build libraries for different platforms in `../target/` and copy them to `ffi/kotlin/jniLibs`.
In addition it will generate Kotlin bindings in `ffi/kotlin/breez-sdk`.

```
make kotlin
```

#### Android Archive (AAR)

This command will build an AAR file in `ffi/android/lib-release.aar`:

```
make bindings-android
```

See [Add your AAR or JAR as a dependency](https://developer.android.com/studio/projects/android-library#psd-add-aar-jar-dependency) in Android's docs for more information on how to integrate such an archive into your project.

#### Known Issues

The Kotlin bindings for the Breez SDK rely on [JNA](https://github.com/java-native-access/jna) to call native methods. JNA 5.7 or greater is required. Depending on the JVM version you use, you might not have the JNA dependency in your classpath. The exception thrown will be something like:

```
class file for com.sun.jna.Pointer not found
```

The solution is to add JNA as a dependency:

```
dependencies {
    // ...
    implementation "net.java.dev.jna:jna:5.7.0@aar"
}
```

### C#

```
# For linux
make csharp-linux

# Alternatively, for mac:
make csharp-darwin
```

This will generate the artifacts in the `ffi/csharp` folder.

### Go

```
# For linux
make golang-linux

# Alternatively, for mac:
make golang-darwin
```

This will generate the artifacts in the `ffi/golang` folder.

### Python

```
# For linux
make python-linux

# Alternatively, for mac:
make python-darwin
```

This will generate the artifacts in the `ffi/python` folder.

## Test

In the tests directory there are some small scripts with some examples on how to use the sdk.
  * Kotlin `tests/bindings/test_breez-sdk.kts`
  * Swift `tests/bindings/test_breez-sdk.swift`  
  * C# `tests/bindings/test_breez_sdk.cs`
  * Go `tests/bindings/golang/test_breez_sdk.go`
  * Python `tests/bindings/test_breez_sdk.py`

If you want to try them out, you will need:

* The [Kotlin command-line tools](https://kotlinlang.org/docs/tutorials/command-line.html), particularly `kotlinc`.
* The [Java Native Access](https://github.com/java-native-access/jna#download) JAR downloaded and its path
  added to your `$CLASSPATH` environment variable.
* The [Swift command-line tools](https://swift.org/download/), particularly `swift`, `swiftc` and
  the `Foundation` package.

Then you can run ```cargo test```
