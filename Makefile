all: fmt codegen clippy

fmt:
	cd libs && cargo fmt
	cd tools/sdk-cli && cargo fmt

clippy: cargo-clippy wasm-clippy

test: cargo-test wasm-test

cargo-clippy:
	# Explicitly allow clippy::uninlined-format-args lint because it's present in the generated breez_sdk.uniffi.rs
	cd libs && cargo clippy -- -D warnings -A clippy::uninlined-format-args
	cd libs && cargo clippy --tests -- -D warnings -A clippy::uninlined-format-args
	cd tools/sdk-cli && cargo clippy -- -D warnings

cargo-test:
	cd libs && cargo test

wasm-clippy:
	make -C ./libs/sdk-common wasm-clippy

wasm-test:
	make -C ./libs/sdk-common wasm-test

codegen: flutter-codegen react-native-codegen

flutter-codegen:
	make -C ./libs/sdk-flutter flutter_rust_bridge

react-native-codegen:
	make -C ./libs/sdk-react-native react-native
