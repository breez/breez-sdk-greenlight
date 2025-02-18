#![cfg(test)]

#[cfg_attr(target_arch = "wasm32", path = "mock_server_wasm.rs")]
pub(crate) mod mock_server;
