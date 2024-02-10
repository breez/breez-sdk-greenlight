WASI_SDK := /opt/wasi-sdk
AR:= /opt/wasi-sdk/bin/llvm-ar
NM:= /opt/wasi-sdk/bin/llvm-nm
CC:= "/opt/wasi-sdk/bin/clang"
CXX:= /opt/wasi-sdk/bin/clang++

init:
	rustup target add wasm32-wasi

clear:
	clear
	cargo clean
	rm Cargo.lock

build:
	cargo update
	WASI_SDK=$(WASI_SDK) AR=$(AR) NM=$(NM) CC=$(CC) CXX=$(CXX) cargo wasix build --release --verbose

wasix: clear build
