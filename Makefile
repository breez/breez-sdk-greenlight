all: fmt clippy

fmt:
	cd libs && cargo fmt -- --check
	cd tools/sdk-cli && cargo fmt -- --check

clippy:
	# Explicitly allow clippy::uninlined-format-args lint because it's present in the generated breez_sdk.uniffi.rs
	cd libs && cargo clippy -- -D warnings -A clippy::uninlined-format-args
	cd libs && cargo clippy --tests -- -D warnings -A clippy::uninlined-format-args
	cd tools/sdk-cli && cargo clippy -- -D warnings
