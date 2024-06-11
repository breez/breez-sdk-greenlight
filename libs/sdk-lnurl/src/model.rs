use serde::{Deserialize, Serialize};

/// Contains the result of the entire LNURL interaction, as reported by the LNURL endpoint.
///
/// * `Ok` indicates the interaction with the endpoint was valid, and the endpoint
///  - started to pay the invoice asynchronously in the case of LNURL-withdraw,
///  - verified the client signature in the case of LNURL-auth,////// * `Error` indicates a generic issue the LNURL endpoint encountered, including a freetext
/// description of the reason.
///
/// Both cases are described in LUD-03 <https://github.com/lnurl/luds/blob/luds/03.md> & LUD-04: <https://github.com/lnurl/luds/blob/luds/04.md>
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
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct LnUrlErrorData {
    pub reason: String,
}

/// Wrapped in a [LnUrlAuth], this is the result of [parse] when given a LNURL-auth endpoint.
///
/// It represents the endpoint's parameters for the LNURL workflow.
///
/// See <https://github.com/lnurl/luds/blob/luds/04.md>
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct LnUrlAuthRequestData {
    /// Hex encoded 32 bytes of challenge
    pub k1: String,

    /// When available, one of: register, login, link, auth
    pub action: Option<String>,

    /// Indicates the domain of the LNURL-auth service, to be shown to the user when asking for
    /// auth confirmation, as per LUD-04 spec.
    #[serde(skip_serializing, skip_deserializing)]
    pub domain: String,

    /// Indicates the URL of the LNURL-auth service, including the query arguments. This will be
    /// extended with the signed challenge and the linking key, then called in the second step of the workflow.
    #[serde(skip_serializing, skip_deserializing)]
    pub url: String,
}