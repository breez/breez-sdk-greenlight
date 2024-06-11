mod lnurl;

pub mod prelude {
    pub use crate::lnurl::error::*;
    pub use crate::lnurl::model::*;
    pub use crate::lnurl::specs::auth::*;
    pub use crate::lnurl::*;
}
