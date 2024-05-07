SOURCES=$(sort $(wildcard ../sdk-core/src/*.rs ../sdk-core/src/**/*.rs))
SHELL := /bin/bash
OS_NAME=$(shell uname | tr '[:upper:]' '[:lower:]')

.PHONY: init
init:
	cargo install cargo-ndk
	cargo install cargo-expand
	flutter pub get

## all: Compile iOS, Android
all: ios android

flutter_rust_bridge:
	cargo install flutter_rust_bridge_codegen --version 2.0.0-dev.33
	mkdir -p ./lib/generated
	flutter_rust_bridge_codegen generate

ios: $(SOURCES) flutter_rust_bridge
	make -C ../sdk-bindings bindings-swift
	rm -rf ios/bindings-swift
	cp -r ../sdk-bindings/bindings-swift ios/bindings-swift
	rm -rf ios/bindings-swift/Tests
	rm -f ios/bindings-swift/Package.swift

## android: Compile the android targets (arm64, armv7 and i686)
.PHONY: android
android: $(SOURCES) flutter_rust_bridge
	make -C ../sdk-bindings kotlin
	mkdir -p ./android/src/main/jniLibs
	cp -r ../sdk-bindings/ffi/kotlin/jniLibs/* ./android/src/main/jniLibs
	mkdir -p ./android/src/main/kotlin/breez_sdk/
	cp -r ../sdk-bindings/ffi/kotlin/breez_sdk/* ./android/src/main/kotlin/breez_sdk/
	mkdir -p ./android/src/main/kotlin/breez_sdk_notification/
	cp -r ../sdk-bindings/bindings-android/lib/src/main/kotlin/breez_sdk_notification/* ./android/src/main/kotlin/breez_sdk_notification/

## desktop: compiles for x86_64-unknown-linux-gnu by default, other targets can be specified
## with the TARGET variable eg. make desktop TARGET=aarch64-unknown-linux-gnu
TARGET ?= x86_64-unknown-linux-gnu
.PHONY: desktop
desktop: $(SOURCES) flutter_rust_bridge
	make -C ../sdk-bindings $(TARGET)
	cd ../sdk-bindings && make $(TARGET)
	mkdir -p ./$(TARGET)
	cp ../target/$(TARGET)/release/libbreez_sdk_bindings.so ./$(TARGET)/libbreez_sdk_bindings.so

## clean:
.PHONY: clean
clean:
	cargo clean
	rm -rf ./lib/generated/
	rm -rf ./android/src/main/jniLibs
	rm -rf ./android/src/main/kotlin/breez_sdk/
	rm -rf ./android/src/main/kotlin/breez_sdk_notification/
	rm -rf ./ios/libbreez_sdk_bindings.a
	rm -rf ./ios/bindings-swift
	rm -rf ./$(TARGET)/libbreez_sdk_bindings.so

## bootstrap:
.PHONY: bootstrap
bootstrap:
	flutter clean
	cargo clean --package breez-sdk-core --manifest-path ../Cargo.toml
	cargo install cargo-ndk
	cargo install cargo-expand
	cargo install flutter_rust_bridge_codegen --version 2.0.0-dev.33
	dart pub global activate ffigen
	dart pub global activate ffi
	cargo install cargo-xcode
	cargo check --package breez-sdk-core --manifest-path ../Cargo.toml
	flutter pub get
	mkdir -p ./lib/generated
	flutter_rust_bridge_codegen generate
	flutter pub run ffigen --config ffigen.yaml
	cargo build --package breez-sdk-core --manifest-path ../Cargo.toml
	rustup target add aarch64-apple-ios
	rustup target add x86_64-apple-ios
	rustup target add aarch64-apple-ios-sim
	rustup target add x86_64-apple-darwin
	rustup target add aarch64-apple-darwin
	mkdir -p platform-build
	cd platform-build
	cargo build --package breez-sdk-core  --manifest-path ../../Cargo.toml --target=aarch64-apple-ios
	cargo build --package breez-sdk-core  --manifest-path ../../Cargo.toml --target=x86_64-apple-ios
	cargo build --package breez-sdk-core  --manifest-path ../../Cargo.toml --target=aarch64-apple-ios-sim
	cargo build --package breez-sdk-core  --manifest-path ../../Cargo.toml --target=x86_64-apple-darwin
	cargo build --package breez-sdk-core  --manifest-path ../../Cargo.toml --target=aarch64-apple-darwin
	mkdir -p ios-sim-lipo
	lipo -create -output ios-sim-lipo/libbreez_sdk_core.a ../../target/aarch64-apple-ios/debug/libbreez_sdk_core.a
	lipo -create -output ios-sim-lipo/libbreez_sdk_core.a ../../target/x86_64-apple-ios/debug/libbreez_sdk_core.a
	lipo -create -output ios-sim-lipo/libbreez_sdk_core.a ../../target/aarch64-apple-ios-sim/debug/libbreez_sdk_core.a
	lipo -create -output ios-sim-lipo/libbreez_sdk_core.a ../../target/x86_64-apple-darwin/debug/libbreez_sdk_core.a
	lipo -create -output ios-sim-lipo/libbreez_sdk_core.a ../../target/aarch64-apple-darwin/debug/libbreez_sdk_core.a
	xcodebuild -create-xcframework -library ios-sim-lipo/libbreez_sdk_core.a -headers ios/Classes/breez_sdk_core.h -output breez_sdk_core.xcframework
	zip -ry  breez_sdk_core.xcframework.zip breez_sdk_core.xcframework
	rm -rf ios-sim-lipo
	rm -rf breez_sdk_core.xcframework
	rustup target add aarch64-linux-android
	rustup target add armv7-linux-androideabi
	rustup target add x86_64-linux-android
	rustup target add i686-linux-android
	mkdir -p jniLibs
	cargo ndk -o jniLibs \
        --manifest-path ../../sdk-core/Cargo.toml \
        -t armeabi-v7a \
        -t arm64-v8a \
        -t x86 \
        -t x86_64 \
        build "$@"
	cd jniLibs
	tar -czvf ../android.tar.gz *
	cd ..
	rm -rf jniLibs