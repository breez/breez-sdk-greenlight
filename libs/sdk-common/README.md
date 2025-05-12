# sdk-common
This crate packages together common SDK functionality for [Breez SDK - Native](https://github.com/breez/breez-sdk-greenlight) and [Breez SDK - Nodeless](https://github.com/breez/breez-sdk-liquid).

## Prerequisites
### Building WASM
  * Install [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/): `cargo install wasm-pack`
  * (Mac Only) Install [llvm](https://llvm.org/): `brew install llvm`
### Testing WASM
  * Install the default testing browser [Firefox](https://www.mozilla.org/en-US/firefox/)
  * When testing on Safari:
    * Enable safaridriver (might need sudo): `safaridriver --enable`
  * When testing on Chrome:
    * Install chromedriver: `brew install chromedriver`
    + (Mac Only) Allow use of chromeriver if the first run fails: ` Settings > Privacy & Security > Allow use of chromedriver`

## Test
`cargo test`

When testing WASM:
* Firefox (default): `make wasm-test`
* Chrome: `make wasm-test-chrome`
* Safari: `make wasm-test-safari`
