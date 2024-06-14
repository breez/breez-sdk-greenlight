use sdk_common::prelude::*;
use serde::Serialize;

use crate::Payment;

/// Wrapper around [LnUrlPayResult], which includes the [Payment] on success
#[derive(Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum WrappedLnUrlPayResult {
    EndpointSuccess { data: WrappedLnUrlPaySuccessData },
    EndpointError { data: LnUrlErrorData },
    PayError { data: LnUrlPayErrorData },
}

#[derive(Serialize)]
pub struct WrappedLnUrlPaySuccessData {
    pub payment: Payment,
    pub success_action: Option<SuccessActionProcessed>,
}
