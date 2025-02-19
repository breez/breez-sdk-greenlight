UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
	CLANG_PREFIX += AR=/usr/local/opt/llvm/bin/llvm-ar CC=/usr/local/opt/llvm/bin/clang
endif

init:
	cargo install wasm-pack

clippy: cargo-clippy wasm-clippy

test: cargo-test wasm-test

cargo-clippy:
	cargo clippy -- -D warnings -A clippy::uninlined-format-args

cargo-test:
	cargo test

wasm-clippy:
	$(CLANG_PREFIX) cargo clippy --target=wasm32-unknown-unknown -- -D warnings -A clippy::uninlined-format-args

wasm-test:
	$(CLANG_PREFIX) wasm-pack test --headless --firefox
