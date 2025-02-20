#![cfg(test)]

#[cfg_attr(
    all(target_family = "wasm", target_os = "unknown"),
    path = "mock_server_wasm.rs"
)]
pub(crate) mod mock_server;
