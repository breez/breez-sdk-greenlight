#[cfg_attr(target_arch = "wasm32", path = "tonic_wrap_wasm.rs")]
mod tonic_wrap;

pub use tonic_wrap::*;
