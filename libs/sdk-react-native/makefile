.PHONY: init
init: ndk-home
	make -C ../sdk-bindings init
	
## clean:
.PHONY: clean
clean:
	make -C ../sdk-bindings clean

.PHONY: ndk-home
ndk-home:
	@if [ ! -d "${ANDROID_NDK_HOME}" ] ; then \
		echo "Error: Please, set the ANDROID_NDK_HOME env variable to point to your NDK folder" ; \
		exit 1 ; \
	fi

all: android ios

android-make:
	make -C ../sdk-bindings kotlin

android-copy:
	mkdir -p android/src/main/jniLibs/x86
	mkdir -p android/src/main/jniLibs/x86_64
	mkdir -p android/src/main/jniLibs/arm64-v8a
	mkdir -p android/src/main/jniLibs/armeabi-v7a
	cp ../sdk-bindings/ffi/kotlin/breez_sdk/breez_sdk.kt  android/src/main/java/com/breezsdk/breez_sdk.kt
	cp ../sdk-bindings/ffi/kotlin/jniLibs/x86/libbreez_sdk_bindings.so android/src/main/jniLibs/x86/libbreez_sdk_bindings.so
	cp ../sdk-bindings/ffi/kotlin/jniLibs/x86_64/libbreez_sdk_bindings.so android/src/main/jniLibs/x86_64/libbreez_sdk_bindings.so
	cp ../sdk-bindings/ffi/kotlin/jniLibs/arm64-v8a/libbreez_sdk_bindings.so android/src/main/jniLibs/arm64-v8a/libbreez_sdk_bindings.so
	cp ../sdk-bindings/ffi/kotlin/jniLibs/armeabi-v7a/libbreez_sdk_bindings.so android/src/main/jniLibs/armeabi-v7a/libbreez_sdk_bindings.so

android: android-make android-copy

ios-make:
	make -C ../sdk-bindings bindings-swift

ios-copy:
	rm -rf ios/bindings-swift
	cp -r ../sdk-bindings/bindings-swift ios/bindings-swift
	rm -rf ios/bindings-swift/Tests
	rm -f ios/bindings-swift/Package.swift

ios: ios-make ios-copy
