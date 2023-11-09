.PHONY: init
init:
	make -C ../ init

darwin:
	make -C ../ python-darwin
	cp ../ffi/python/breez_sdk.py src/breez_sdk
	cp ../ffi/python/libbreez_sdk_bindings.dylib src/breez_sdk

linux:
	make -C ../ python-linux
	cp ../ffi/python/breez_sdk.py src/breez_sdk
	cp ../ffi/python/libbreez_sdk_bindings.dylib src/breez_sdk
