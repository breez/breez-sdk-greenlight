CURRENT_DIR := $(shell pwd)
SOURCES=$(sort $(wildcard ./src/*.rs ./src/**/*.rs))
SHELL := /bin/bash

.PHONY: init
init:
	rustup target add aarch64-apple-ios x86_64-apple-ios
	rustup target add aarch64-apple-darwin x86_64-apple-darwin
	rustup target add aarch64-apple-ios-sim
	#rustup target add armv7-apple-ios armv7s-apple-ios i386-apple-ios ## deprecated
	rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
	@if [ $$(uname) == "Darwin" ] ; then cargo install cargo-lipo ; fi
	cargo install cbindgen
	cargo install cargo-ndk

## all: Compile iOS, Android
all: swift-ios swift-darwin kotlin

ios-universal: $(SOURCES)		
	mkdir -p ../target/ios-simulator/release
	cargo build --release --target aarch64-apple-ios ;\
	cargo build --release --target x86_64-apple-ios ;\
	cargo build --release --target aarch64-apple-ios-sim ;\
	lipo -create -output ../target/ios-simulator/release/libbreez_sdk_bindings.a ../target/aarch64-apple-ios-sim/release/libbreez_sdk_bindings.a ../target/x86_64-apple-ios/release/libbreez_sdk_bindings.a

darwin-universal: $(SOURCES)
	mkdir -p ../target/darwin-universal/release
	cargo lipo --release --targets aarch64-apple-darwin		
	cargo lipo --release --targets x86_64-apple-darwin		
	lipo -create -output ../target/darwin-universal/release/libbreez_sdk_bindings.dylib ../target/aarch64-apple-darwin/release/libbreez_sdk_bindings.dylib ../target/x86_64-apple-darwin/release/libbreez_sdk_bindings.dylib

swift-ios: ios-universal
	uniffi-bindgen generate src/breez_sdk.udl -l swift -o ffi/swift-ios
	cp ../target/ios-simulator/release/libbreez_sdk_bindings.a ffi/swift-ios
	cd ffi/swift-ios && "swiftc" "-emit-module" "-module-name" "breez_sdk_bindings"  "-Xcc" "-fmodule-map-file=$(CURRENT_DIR)/ffi/swift-ios/breez_sdkFFI.modulemap" "-I" "."  "-L" "." "-lbreez_sdk_bindings" breez_sdk.swift

xcframework: swift-ios
	mkdir -p ffi/xcframework
	mkdir -p ffi/xcframework/build/include ffi/xcframework/build/lib/ios-simulator/release ffi/xcframework/build/lib/aarch64-apple-ios/release
	cp -r ffi/swift-ios/breez_sdkFFI.h ffi/xcframework/build/include
	cp ffi/swift-ios/breez_sdkFFI.modulemap ffi/xcframework/build/include/module.modulemap
	cp ../target/ios-simulator/release/libbreez_sdk_bindings.a ffi/xcframework/build/lib/ios-simulator/release
	cp ../target/aarch64-apple-ios/release/libbreez_sdk_bindings.a ffi/xcframework/build/lib/aarch64-apple-ios/release
	xcodebuild -create-xcframework -library ffi/xcframework/build/lib/ios-simulator/release/libbreez_sdk_bindings.a -headers ffi/xcframework/build/include -library ffi/xcframework/build/lib/aarch64-apple-ios/release/libbreez_sdk_bindings.a -headers ffi/xcframework/build/include -output ffi/xcframework/breez_sdkFFI.xcframework

swift-darwin: darwin-universal
	uniffi-bindgen generate src/breez_sdk.udl -l swift -o ffi/swift-darwin
	cp ../target/darwin-universal/release/libbreez_sdk_bindings.dylib ffi/swift-darwin
	cd ffi/swift-darwin && "swiftc" "-emit-module" "-module-name" "breez_sdk_bindings"  "-Xcc" "-fmodule-map-file=$(CURRENT_DIR)/ffi/swift-darwin/breez_sdkFFI.modulemap" "-I" "."  "-L" "." "-lbreez_sdk_bindings" breez_sdk.swift

kotlin: android
	uniffi-bindgen generate src/breez_sdk.udl --language kotlin -o ffi/kotlin 

android: aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

aarch64-linux-android: $(SOURCES) ndk-home
	cargo ndk -t aarch64-linux-android -o ffi/kotlin/jniLibs build --release	


armv7-linux-androideabi: $(SOURCES) ndk-home
	cargo ndk -t armv7-linux-androideabi -o ffi/kotlin/jniLibs build --release	

i686-linux-android: $(SOURCES) ndk-home
	cargo ndk -t i686-linux-android -o ffi/kotlin/jniLibs build --release	

x86_64-linux-android: $(SOURCES) ndk-home
	cargo ndk -t x86_64-linux-android -o ffi/kotlin/jniLibs build --release	
		
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
	rm -rf ffi
## test:
.PHONY: test
test:
	cargo test
