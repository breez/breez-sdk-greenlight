PROJECTNAME=$(shell basename "$(PWD)")
CURRENT_DIR := $(shell pwd)
OS_NAME=$(shell uname | tr '[:upper:]' '[:lower:]')

SOURCES=$(sort $(wildcard ./src/*.rs ./src/**/*.rs))
PATH := $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin:$(PATH)

ANDROID_AARCH64_CC=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/aarch64-linux-android29-clang
ANDROID_AARCH64_CXX=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/aarch64-linux-android29-clang++
ANDROID_AARCH64_AR=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/llvm-ar

ANDROID_ARMV7_CC=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/armv7a-linux-androideabi29-clang
ANDROID_ARMV7_CXX=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/armv7a-linux-androideabi29-clang++
ANDROID_ARMV7_AR=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/llvm-ar

ANDROID_I686_CC=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/i686-linux-android29-clang
ANDROID_I686_CXX=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/i686-linux-android29-clang++
ANDROID_I686_AR=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/llvm-ar

ANDROID_X86_64_CC=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/x86_64-linux-android29-clang
ANDROID_X86_64_CXX=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/x86_64-linux-android29-clang++
ANDROID_X86_64_AR=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/llvm-ar

SHELL := /bin/bash

# ##############################################################################
# # GENERAL
# ##############################################################################

## init: Install missing dependencies.
.PHONY: init
init:
	rustup target add aarch64-apple-ios x86_64-apple-ios
	rustup target add aarch64-apple-darwin x86_64-apple-darwin
	rustup target add aarch64-apple-ios-sim
	#rustup target add armv7-apple-ios armv7s-apple-ios i386-apple-ios ## deprecated
	rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
	@if [ $$(uname) == "Darwin" ] ; then cargo install cargo-lipo ; fi
	cargo install cbindgen

## :

# ##############################################################################
# # RECIPES
# ##############################################################################

## all: Compile iOS, Android
all: ios-universal aarch64-apple-darwin android

ios-universal: $(SOURCES)	
	mkdir -p target/universal/release
	cargo build --release --target aarch64-apple-ios ;\
	cargo build --release --target x86_64-apple-ios ;\
	/usr/bin/lipo -create -output target/universal/release/libbreez_sdk_bindings.a target/aarch64-apple-ios/release/libbreez_sdk_bindings.a target/x86_64-apple-ios/release/libbreez_sdk_bindings.a	


aarch64-apple-darwin: $(SOURCES)
	@if [ $$(uname) == "Darwin" ] ; then \
		cargo lipo --release --targets aarch64-apple-darwin ; \
		else echo "Skipping macOS compilation on $$(uname)" ; \
	fi
	@echo "[DONE] $@"

swift-ios: ios-universal
	uniffi-bindgen generate src/breez_sdk.udl -l swift -o ffi/swift-ios
	cp $(CURRENT_DIR)/target/universal/release/libbreez_sdk_bindings.a ffi/swift-ios
	cd ffi/swift-ios && "swiftc" "-emit-module" "-module-name" "breez_sdk_bindings"  "-Xcc" "-fmodule-map-file=$(CURRENT_DIR)/ffi/swift-ios/breez_sdkFFI.modulemap" "-I" "."  "-L" "." "-lbreez_sdk_bindings" breez_sdk.swift

swift-darwin: aarch64-apple-darwin
	uniffi-bindgen generate src/breez_sdk.udl -l swift -o ffi/swift-darwin
	cp $(CURRENT_DIR)/target/aarch64-apple-darwin/release/libbreez_sdk_bindings.dylib ffi/swift-darwin	
	cd ffi/swift-darwin && "swiftc" "-emit-module" "-module-name" "breez_sdk_bindings"  "-Xcc" "-fmodule-map-file=$(CURRENT_DIR)/ffi/swift-ios/breez_sdkFFI.modulemap" "-I" "."  "-L" "." "-lbreez_sdk_bindings" breez_sdk.swift

kotlin: android
	uniffi-bindgen generate src/breez_sdk.udl --language kotlin -o ffi/kotlin 

android: aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

aarch64-linux-android: $(SOURCES) ndk-home
	@mkdir -p target/aarch64-linux-android/release/deps
	@echo "INPUT(-lunwind)" > target/aarch64-linux-android/release/deps/libgcc.a
	CC_aarch64_linux_android=$(ANDROID_AARCH64_CC) \
	CXX_aarch64_linux_android=$(ANDROID_AARCH64_CXX) \
	AR_aarch64_linux_android=$(ANDROID_AARCH64_AR) \
	CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$(ANDROID_AARCH64_CC) \
		cargo  build --target aarch64-linux-android --release
	mkdir -p ffi/kotlin/jniLibs/aarch64-linux-android/
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so ffi/kotlin/jniLibs/aarch64-linux-android/	
	cp target/aarch64-linux-android/release/libbreez_sdk_bindings.so ffi/kotlin/jniLibs/aarch64-linux-android/libbreez_sdk_bindings.so
	@echo "[DONE] $@"


armv7-linux-androideabi: $(SOURCES) ndk-home
	@mkdir -p target/armv7-linux-androideabi/release/deps
	@echo "INPUT(-lunwind)" > target/armv7-linux-androideabi/release/deps/libgcc.a
	CC_armv7_linux_androideabi=$(ANDROID_ARMV7_CC) \
	CXX_armv7_linux_androideabi=$(ANDROID_ARMV7_CXX) \
	AR_armv7_linux_androideabi=$(ANDROID_ARMV7_AR) \
	CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$(ANDROID_ARMV7_CC) \
		cargo build --target armv7-linux-androideabi --release
	mkdir -p ffi/kotlin/jniLibs/armeabi-v7a/
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/arm-linux-androideabi/libc++_shared.so ffi/kotlin/jniLibs/armeabi-v7a/
	cp target/armv7-linux-androideabi/release/libbreez_sdk_bindings.so ffi/kotlin/jniLibs/armeabi-v7a/libbreez_sdk_bindings.so
	@echo "[DONE] $@"

i686-linux-android: $(SOURCES) ndk-home
	@mkdir -p target/i686-linux-android/release/deps
	@echo "INPUT(-lunwind)" > target/i686-linux-android/release/deps/libgcc.a
	CC_i686_linux_android=$(ANDROID_I686_CC) \
	CXX_i686_linux_android=$(ANDROID_I686_CXX) \
	AR_i686_linux_android=$(ANDROID_I686_AR) \
	CARGO_TARGET_I686_LINUX_ANDROID_LINKER=$(ANDROID_I686_CC) \
		cargo  build --target i686-linux-android --release
	mkdir -p ffi/kotlin/jniLibs/i686-linux-android/
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/i686-linux-android/libc++_shared.so ffi/kotlin/jniLibs/i686-linux-android/
	cp target/i686-linux-android/release/libbreez_sdk_bindings.so ffi/kotlin/jniLibs/i686-linux-android/libbreez_sdk_bindings.so
	@echo "[DONE] $@"

x86_64-linux-android: $(SOURCES) ndk-home
	@mkdir -p target/x86_64-linux-android/release/deps
	@echo "INPUT(-lunwind)" > target/x86_64-linux-android/release/deps/libgcc.a
	CC_x86_64_linux_android=$(ANDROID_X86_64_CC) \
	CXX_x86_64_linux_android=$(ANDROID_X86_64_CXX) \
	AR_x86_64_linux_android=$(ANDROID_X86_64_AR) \
	CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=$(ANDROID_X86_64_CC) \
		cargo build --target x86_64-linux-android --release
	mkdir -p ffi/kotlin/jniLibs/x86_64/
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/x86_64-linux-android/libc++_shared.so ffi/kotlin/jniLibs/x86_64/
	cp target/x86_64-linux-android/release/libbreez_sdk_bindings.so ffi/kotlin/jniLibs/x86_64/libbreez_sdk_bindings.so
	@echo "[DONE] $@"
		
.PHONY: ndk-home
ndk-home:
	@if [ ! -d "${ANDROID_NDK_HOME}" ] ; then \
		echo "Error: Please, set the ANDROID_NDK_HOME env variable to point to your NDK folder" ; \
		exit 1 ; \
	fi

## clean:
.PHONY: clean
clean:
	cargo clean
	rm -rf target
	rm -rf ffi/swift-darwin
	rm -rf ffi/swift-ios

## test:
.PHONY: test
test:
	cargo test
