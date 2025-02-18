use serde::{Deserialize, Serialize};

/// Contains the result of the entire LNURL interaction, as reported by the LNURL endpoint.
///
/// * `Ok` indicates the interaction with the endpoint was valid, and the endpoint
///  - started to pay the invoice asynchronously in the case of LNURL-withdraw,
///  - verified the client signature in the case of LNURL-auth
/// * `Error` indicates a generic issue the LNURL endpoint encountered, including a freetext
///    description of the reason.
///
/// Both cases are described in LUD-03 <https://github.com/lnurl/luds/blob/luds/03.md> & LUD-04: <https://github.com/lnurl/luds/blob/luds/04.md>
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi)
)]
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(tag = "status")]
pub enum LnUrlCallbackStatus {
    /// On-wire format is: `{"status": "OK"}`
    Ok,
    /// On-wire format is: `{"status": "ERROR", "reason": "error details..."}`
    #[serde(rename = "ERROR")]
    ErrorStatus {
        #[serde(flatten)]
        data: LnUrlErrorData,
    },
}

/// Wrapped in a [LnUrlError], this represents a LNURL-endpoint error.
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct LnUrlErrorData {
    pub reason: String,
}
