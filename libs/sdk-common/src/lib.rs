mod breez_server;
mod buy;
mod error;
mod fiat;
pub mod grpc;
pub mod input_parser;
pub mod invoice;
#[cfg(feature = "liquid")]
pub mod liquid;
mod lnurl;
mod model;
pub mod tonic_wrap;
mod utils;

// Re-export commonly used crates, to make it easy for callers to use the specific versions we're using.
// For example, for the bitcoin crate, this is important because certain error conversions defined in
// sdk-common map to or from structs from this specific bitcoin crate version. If the caller would
// use a different version, the Into traits defined here would not be usable by them
// (e.g. impl From<bip32::Error> for LnUrlError)
pub use bitcoin;
pub use lightning;
#[cfg(feature = "liquid")]
pub use lightning_125;
pub use lightning_invoice;

// We don't include grpc::* in the prelude exports, to force callers to use the grpc path prefix.
#[rustfmt::skip]
pub mod prelude {
    pub use crate::*;
    pub use crate::breez_server::*;
    pub use crate::error::*;
    pub use crate::fiat::*;
    pub use crate::buy::*;
    pub use crate::buy::moonpay::*;
    pub use crate::input_parser::*;
    pub use crate::invoice::*;
    #[cfg(feature = "liquid")]
    pub use crate::liquid::*;
    pub use crate::lnurl::error::*;
    pub use crate::lnurl::model::*;
    pub use crate::lnurl::specs::auth::model::*;
    pub use crate::lnurl::specs::auth::*;
    pub use crate::lnurl::specs::pay::model::*;
    pub use crate::lnurl::specs::pay::*;
    pub use crate::lnurl::specs::withdraw::model::*;
    pub use crate::lnurl::specs::withdraw::*;
    pub use crate::lnurl::*;
    pub use crate::model::*;
    pub use crate::utils::rest_client::*;
}
