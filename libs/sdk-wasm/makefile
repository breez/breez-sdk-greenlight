WASI_SDK := "/opt/wasi-sdk"
CLANG_LIB_WASI := "$(WASI_SDK)/lib/clang/16/lib/wasi"
WASI_SYSROOT := "$(WASI_SDK)/share/wasi-sysroot"
CC := "$(WASI_SDK)/bin/clang --sysroot=$(WASI_SYSROOT)"
AR := "$(WASI_SDK)/bin/llvm-ar"
CC_WASM32_WASI := "$(WASI_SDK)/bin/clang"
LIBSQLITE3_FLAGS := "-DSQLITE_THREADSAFE=0 -DSQLITE_OMIT_SHARED_CACHE -D_LARGEFILE64_SOURCE"
RUSTFLAGS := "-Clink-arg=-L$(CLANG_LIB_WASI) -Clink-arg=-lclang_rt.builtins-wasm32 -Clink-arg=--export-table -Clink-arg=--export=malloc -Clink-arg=--export=free"
OPENSSL_DIR := "/opt/openssl-wasm32-wasi/precompiled"

init:
	rustup target add wasm32-wasi

clear:
	clear
	cargo clean
	rm Cargo.lock

build:
	cargo update
	OPENSSL_NO_VENDOR=1 OPENSSL_STATIC=1 OPENSSL_DIR=$(OPENSSL_DIR) CC=$(CC) AR=$(AR) CC_WASM32_WASI=$(CC_WASM32_WASI) LIBSQLITE3_FLAGS=$(LIBSQLITE3_FLAGS) RUSTFLAGS=$(RUSTFLAGS) cargo wasix build

wasix: clear build
