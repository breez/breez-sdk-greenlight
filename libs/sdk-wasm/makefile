init:
	rustup target add wasm32-wasi

clear:
	clear
	cargo clean
	rm Cargo.lock

build:
	cargo update
	cargo wasix build --release --verbose

wasix: clear build
