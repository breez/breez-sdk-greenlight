all: fmt clippy codegen

fmt:
	cd libs && cargo fmt -- --check
	cd tools/sdk-cli && cargo fmt -- --check

clippy:
	# Explicitly allow clippy::uninlined-format-args lint because it's present in the generated breez_sdk.uniffi.rs
	cd libs && cargo clippy -- -D warnings -A clippy::uninlined-format-args
	cd libs && cargo clippy --tests -- -D warnings -A clippy::uninlined-format-args
	cd tools/sdk-cli && cargo clippy -- -D warnings

codegen: flutter-codegen react-native-codegen

flutter-codegen:
	make -C ./libs/sdk-flutter flutter_rust_bridge

react-native-codegen:
	make -C ./libs/sdk-react-native react-native-codegen
