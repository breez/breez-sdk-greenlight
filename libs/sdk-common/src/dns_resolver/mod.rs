#[cfg_attr(
    all(target_family = "wasm", target_os = "unknown"),
    path = "resolver_wasm.rs"
)]
mod resolver;

pub(crate) use resolver::txt_lookup;
