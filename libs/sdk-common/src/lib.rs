pub mod input_parser;
pub mod invoice;
mod lnurl;
mod model;
mod utils;

// Re-export, to make it easy for callers to refer to the specific bitcoin crate version we're using
// This is important because certain error conversions defined in this crate map to or from structs
// from this specific bitcoin crate version. If the caller would use a different version, the Into
// traits defined here would not be usable by them (e.g. impl From<bip32::Error> for LnUrlError)
pub use bitcoin;

#[rustfmt::skip]
pub mod prelude {
    pub use crate::*;
    pub use crate::input_parser::*;
    pub use crate::invoice::*;
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
