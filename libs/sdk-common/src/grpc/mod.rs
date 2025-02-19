tonic::include_proto!("breez");

#[cfg_attr(target_arch = "wasm32", path = "transport_wasm.rs")]
pub(crate) mod transport;
