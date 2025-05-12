pub(crate) mod rest_client;

#[macro_export]
macro_rules! ensure_sdk {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

pub(crate) fn default_true() -> bool {
    true
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub use std::rc::Rc as Arc;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub use std::sync::Arc;
