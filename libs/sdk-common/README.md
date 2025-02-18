# sdk-common
This crate packages together common SDK functionality for [Breez SDK - Native](https://github.com/breez/breez-sdk-greenlight) and [Breez SDK - Nodeless](https://github.com/breez/breez-sdk-liquid).

## Prerequisites
* When building for WASM:
  * Install [just](https://github.com/casey/just?tab=readme-ov-file#installation) command runner
  * Install [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/): `cargo install wasm-pack`
  * (Mac Only) Install [llvm](https://llvm.org/): `brew install llvm`

## Test
`cargo test`

When testing WASM:
`just wasm-test`
