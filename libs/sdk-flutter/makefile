SOURCES=$(sort $(wildcard ../sdk-core/src/*.rs ../sdk-core/src/**/*.rs))
SHELL := /bin/bash
OS_NAME=$(shell uname | tr '[:upper:]' '[:lower:]')

.PHONY: init
init:
	cargo install cargo-ndk
	cargo install flutter_rust_bridge_codegen --version 1.75.2
	flutter pub get

## all: Compile iOS, Android
all: ios android

## dev: Compile iOS, Android for Development environment
dev: ios-dev android

flutter_rust_bridge:
	flutter_rust_bridge_codegen --dart-format-line-length 110 -r ../sdk-core/src/binding.rs -d lib/bridge_generated.dart -c ios/Classes/bridge_generated.h

ios: $(SOURCES) flutter_rust_bridge
	make -C ../sdk-bindings bindings-swift

ios-dev: ios
	rm -rf ios/bindings-swift
	cp -r ../sdk-bindings/bindings-swift ios/bindings-swift
	rm -rf ios/bindings-swift/Tests
	rm -f ios/bindings-swift/Package.swift

## android: Compile the android targets (arm64, armv7 and i686)
.PHONY: android
android: $(SOURCES) flutter_rust_bridge
	cd ../sdk-core && make android
	mkdir -p ./android/src/main/jniLibs/arm64-v8a
	mkdir -p ./android/src/main/jniLibs/armeabi-v7a
	mkdir -p ./android/src/main/jniLibs/x86
	mkdir -p ./android/src/main/jniLibs/x86_64

	cp ../target/aarch64-linux-android/release/libbreez_sdk_core.so ./android/src/main/jniLibs/arm64-v8a/libbreez_sdk_core.so
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so ./android/src/main/jniLibs/arm64-v8a/	

	cp ../target/armv7-linux-androideabi/release/libbreez_sdk_core.so android/src/main/jniLibs/armeabi-v7a/libbreez_sdk_core.so
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/arm-linux-androideabi/libc++_shared.so android/src/main/jniLibs/armeabi-v7a/

	cp ../target/i686-linux-android/release/libbreez_sdk_core.so android/src/main/jniLibs/x86/libbreez_sdk_core.so
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/i686-linux-android/libc++_shared.so android/src/main/jniLibs/x86/

	cp ../target/x86_64-linux-android/release/libbreez_sdk_core.so android/src/main/jniLibs/x86_64/libbreez_sdk_core.so
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/x86_64-linux-android/libc++_shared.so android/src/main/jniLibs/x86_64/

## desktop: compiles for x86_64-unknown-linux-gnu by default, other targets can be specified
## with the TARGET variable eg. make desktop TARGET=aarch64-unknown-linux-gnu
TARGET ?= x86_64-unknown-linux-gnu
.PHONY: desktop
desktop: $(SOURCES) flutter_rust_bridge
	cd ../sdk-core && make $(TARGET)
	mkdir -p ./$(TARGET)
	cp ../target/$(TARGET)/release/libbreez_sdk_core.so ./$(TARGET)/libbreez_sdk_core.so

## clean:
.PHONY: clean
clean:
	cargo clean
	rm -rf ./android/src/main/jniLibs
	rm -rf ./ios/libbreez_sdk_core.a