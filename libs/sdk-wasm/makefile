WASI_SDK := /opt/wasi-sdk

init:
	rustup target add wasm32-wasi

clear:
	clear
	cargo clean
	rm Cargo.lock

build:
	cargo update
	WASI_SDK=$(WASI_SDK) cargo wasix build --release --verbose

wasix: clear build
