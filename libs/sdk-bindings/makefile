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
all: swift-ios swift-darwin bindings-swift kotlin bindings-android

ios-universal: $(SOURCES)		
	mkdir -p ../target/ios-universal/release
	cargo build --release --target aarch64-apple-ios ;\
	cargo build --release --target x86_64-apple-ios ;\
	cargo build --release --target aarch64-apple-ios-sim ;\
	lipo -create -output ../target/ios-universal/release/libbreez_sdk_bindings.a ../target/aarch64-apple-ios-sim/release/libbreez_sdk_bindings.a ../target/x86_64-apple-ios/release/libbreez_sdk_bindings.a

darwin-universal: $(SOURCES)
	mkdir -p ../target/darwin-universal/release
	cargo lipo --release --targets aarch64-apple-darwin
	cargo lipo --release --targets x86_64-apple-darwin
	lipo -create -output ../target/darwin-universal/release/libbreez_sdk_bindings.dylib ../target/aarch64-apple-darwin/release/libbreez_sdk_bindings.dylib ../target/x86_64-apple-darwin/release/libbreez_sdk_bindings.dylib
	lipo -create -output ../target/darwin-universal/release/libbreez_sdk_bindings.a ../target/aarch64-apple-darwin/release/libbreez_sdk_bindings.a ../target/x86_64-apple-darwin/release/libbreez_sdk_bindings.a

swift-ios: ios-universal
	uniffi-bindgen generate src/breez_sdk.udl -l swift -o ffi/swift-ios
	cp ../target/ios-universal/release/libbreez_sdk_bindings.a ffi/swift-ios
	cd ffi/swift-ios && "swiftc" "-emit-module" "-module-name" "breez_sdk_bindings"  "-Xcc" "-fmodule-map-file=$(CURRENT_DIR)/ffi/swift-ios/breez_sdkFFI.modulemap" "-I" "."  "-L" "." "-lbreez_sdk_bindings" breez_sdk.swift

swift-darwin: darwin-universal
	uniffi-bindgen generate src/breez_sdk.udl -l swift -o ffi/swift-darwin
	cp ../target/darwin-universal/release/libbreez_sdk_bindings.dylib ffi/swift-darwin
	cd ffi/swift-darwin && "swiftc" "-emit-module" "-module-name" "breez_sdk_bindings"  "-Xcc" "-fmodule-map-file=$(CURRENT_DIR)/ffi/swift-darwin/breez_sdkFFI.modulemap" "-I" "."  "-L" "." "-lbreez_sdk_bindings" breez_sdk.swift

bindings-swift: ios-universal darwin-universal
	mkdir -p bindings-swift/Sources/BreezSDK
	uniffi-bindgen generate src/breez_sdk.udl --no-format --language swift --out-dir bindings-swift/Sources/BreezSDK
	mv bindings-swift/Sources/BreezSDK/breez_sdk.swift bindings-swift/Sources/BreezSDK/BreezSDK.swift
	cp bindings-swift/Sources/BreezSDK/breez_sdkFFI.h bindings-swift/breez_sdkFFI.xcframework/ios-arm64/breez_sdkFFI.framework/Headers
	cp bindings-swift/Sources/BreezSDK/breez_sdkFFI.h bindings-swift/breez_sdkFFI.xcframework/ios-arm64_x86_64-simulator/breez_sdkFFI.framework/Headers
	cp bindings-swift/Sources/BreezSDK/breez_sdkFFI.h bindings-swift/breez_sdkFFI.xcframework/macos-arm64_x86_64/breez_sdkFFI.framework/Headers
	cp ../target/aarch64-apple-ios/release/libbreez_sdk_bindings.a bindings-swift/breez_sdkFFI.xcframework/ios-arm64/breez_sdkFFI.framework/breez_sdkFFI
	cp ../target/ios-universal/release/libbreez_sdk_bindings.a bindings-swift/breez_sdkFFI.xcframework/ios-arm64_x86_64-simulator/breez_sdkFFI.framework/breez_sdkFFI
	cp ../target/darwin-universal/release/libbreez_sdk_bindings.a bindings-swift/breez_sdkFFI.xcframework/macos-arm64_x86_64/breez_sdkFFI.framework/breez_sdkFFI
	rm bindings-swift/Sources/BreezSDK/breez_sdkFFI.h
	rm bindings-swift/Sources/BreezSDK/breez_sdkFFI.modulemap

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

bindings-android: android
	cp -r ffi/kotlin/jniLibs bindings-android/lib/src/main
	sed 's/package_name = "breez_sdk"/package_name = "technology.breez"/' uniffi.toml > uniffi.android.toml
	uniffi-bindgen generate src/breez_sdk.udl --language kotlin --config uniffi.android.toml --out-dir bindings-android/lib/src/main/kotlin/
	rm uniffi.android.toml
	cd bindings-android && ./gradlew assembleRelease
		
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
