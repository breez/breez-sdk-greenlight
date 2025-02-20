all: fmt codegen clippy

fmt:
	cd libs && cargo fmt -- --check
	cd tools/sdk-cli && cargo fmt -- --check

clippy:
	cd libs && cargo clippy -- -D warnings
	cd libs && cargo clippy --tests -- -D warnings
	cd tools/sdk-cli && cargo clippy -- -D warnings

codegen: flutter-codegen react-native-codegen

flutter-codegen:
	make -C ./libs/sdk-flutter flutter_rust_bridge

react-native-codegen:
	make -C ./libs/sdk-react-native react-native
