SOURCES=$(sort $(wildcard ../sdk-core/src/*.rs ../sdk-core/src/**/*.rs))
SHELL := /bin/bash
OS_NAME=$(shell uname | tr '[:upper:]' '[:lower:]')

## all: Compile iOS, Android
all: ios-universal android

ios-universal: $(SOURCES)
	cd ../sdk-core && make ios	
	cp ../target/universal/release/libbreez_sdk_core.a ./ios/libbreez_sdk_core.a

## android: Compile the android targets (arm64, armv7 and i686)
android: $(SOURCES)
	cd ../sdk-core && make android	
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so ./android/src/main/jniLibs/arm64-v8a/	
	cp ../target/aarch64-linux-android/release/libbreez_sdk_core.so ./android/src/main/jniLibs/arm64-v8a/libbreez_sdk_core.so
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/arm-linux-androideabi/libc++_shared.so android/src/main/jniLibs/armeabi-v7a/
	cp ../target/armv7-linux-androideabi/release/libbreez_sdk_core.so android/src/main/jniLibs/armeabi-v7a/libbreez_sdk_core.so
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/i686-linux-android/libc++_shared.so android/src/main/jniLibs/x86/
	cp ../target/i686-linux-android/release/libbreez_sdk_core.so android/src/main/jniLibs/x86/libbreez_sdk_core.so
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/x86_64-linux-android/libc++_shared.so android/src/main/jniLibs/x86_64/
	cp ../target/x86_64-linux-android/release/libbreez_sdk_core.so android/src/main/jniLibs/x86_64/libbreez_sdk_core.so

## :

# ##############################################################################
# # OTHER
# ##############################################################################

## clean:
.PHONY: clean
clean:
	cargo clean
	rm -f target/bindings.h target/bindings.src.h