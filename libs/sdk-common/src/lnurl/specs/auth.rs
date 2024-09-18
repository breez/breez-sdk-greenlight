use std::str::FromStr;

use bitcoin::hashes::{hex::ToHex, sha256, Hash, HashEngine, Hmac, HmacEngine};
use bitcoin::secp256k1::{Message, Secp256k1};
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey};
use bitcoin::KeyPair;
use reqwest::Url;

use crate::prelude::*;

/// Performs the third and last step of LNURL-auth, as per
/// <https://github.com/lnurl/luds/blob/luds/04.md>
///
/// Linking key is derived as per LUD-05
/// https://github.com/lnurl/luds/blob/luds/05.md
///
/// See the [parse] docs for more detail on the full workflow.
pub async fn perform_lnurl_auth(
    linking_keys: KeyPair,
    req_data: LnUrlAuthRequestData,
) -> LnUrlResult<LnUrlCallbackStatus> {
    let k1_to_sign = Message::from_slice(
        &hex::decode(req_data.k1)
            .map_err(|e| LnUrlError::Generic(format!("Error decoding k1: {e}")))?,
    )?;
    let sig = Secp256k1::new().sign_ecdsa(&k1_to_sign, &linking_keys.secret_key());

    println!("xpub: {:?}", &linking_keys.public_key().to_hex());
    // <LNURL_hostname_and_path>?<LNURL_existing_query_parameters>&sig=<hex(sign(utf8ToBytes(k1), linkingPrivKey))>&key=<hex(linkingKey)>
    let mut callback_url =
        Url::from_str(&req_data.url).map_err(|e| LnUrlError::InvalidUri(e.to_string()))?;
    callback_url
        .query_pairs_mut()
        .append_pair("sig", &sig.serialize_der().to_hex());
    callback_url
        .query_pairs_mut()
        .append_pair("key", &linking_keys.public_key().to_hex());

    println!("callback_url: {:?}", callback_url.as_str());
    get_parse_and_log_response(callback_url.as_ref(), false)
        .await
        .map_err(|e| LnUrlError::ServiceConnectivity(e.to_string()))
}

pub fn validate_request(
    domain: String,
    lnurl_endpoint: String,
) -> LnUrlResult<LnUrlAuthRequestData> {
    let query =
        Url::from_str(&lnurl_endpoint).map_err(|e| LnUrlError::InvalidUri(e.to_string()))?;
    let query_pairs = query.query_pairs();

    let k1 = query_pairs
        .into_iter()
        .find(|(key, _)| key == "k1")
        .map(|(_, v)| v.to_string())
        .ok_or(LnUrlError::generic("LNURL-auth k1 arg not found"))?;

    let maybe_action = query_pairs
        .into_iter()
        .find(|(key, _)| key == "action")
        .map(|(_, v)| v.to_string());

    let k1_bytes =
        hex::decode(&k1).map_err(|e| LnUrlError::Generic(format!("Error decoding k1: {e}")))?;
    if k1_bytes.len() != 32 {
        return Err(LnUrlError::generic("LNURL-auth k1 is of unexpected length"));
    }

    if let Some(action) = &maybe_action {
        if !["register", "login", "link", "auth"].contains(&action.as_str()) {
            return Err(LnUrlError::generic(
                "LNURL-auth action is of unexpected type",
            ));
        }
    }

    Ok(LnUrlAuthRequestData {
        k1,
        action: maybe_action,
        domain,
        url: lnurl_endpoint,
    })
}

fn hmac_sha256(key: &[u8], input: &[u8]) -> Hmac<sha256::Hash> {
    let mut engine = HmacEngine::<sha256::Hash>::new(key);
    engine.input(input);
    Hmac::<sha256::Hash>::from_engine(engine)
}

pub fn get_derivation_path(
    hashing_key: ExtendedPrivKey,
    url: Url,
) -> LnUrlResult<Vec<ChildNumber>> {
    let domain = url
        .domain()
        .ok_or(LnUrlError::invalid_uri("Could not determine domain"))?;

    let hmac = hmac_sha256(&hashing_key.to_priv().to_bytes(), domain.as_bytes());
    println!("hmac: {:?}", hmac.to_hex());
    // m/138'/<long1>/<long2>/<long3>/<long4>
    Ok(vec![
        ChildNumber::from_hardened_idx(138)?,
        ChildNumber::from(build_path_element_u32(hmac[0..4].try_into()?)),
        ChildNumber::from(build_path_element_u32(hmac[4..8].try_into()?)),
        ChildNumber::from(build_path_element_u32(hmac[8..12].try_into()?)),
        ChildNumber::from(build_path_element_u32(hmac[12..16].try_into()?)),
    ])
}

fn build_path_element_u32(hmac_bytes: [u8; 4]) -> u32 {
    let mut buf = [0u8; 4];
    buf[..4].copy_from_slice(&hmac_bytes);
    u32::from_be_bytes(buf)
}

pub mod model {
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    use crate::prelude::LnUrlError;

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

    /// Error returned by [crate::breez_services::BreezServices::lnurl_auth]
    #[derive(Debug, Error)]
    pub enum LnUrlAuthError {
        /// This error is raised when a general error occurs not specific to other error variants
        /// in this enum.
        #[error("Generic: {err}")]
        Generic { err: String },

        /// This error is raised when the decoded LNURL URI is not compliant to the specification.
        #[error("Invalid uri: {err}")]
        InvalidUri { err: String },

        /// This error is raised when a connection to an external service fails.
        #[error("Service connectivity: {err}")]
        ServiceConnectivity { err: String },
    }

    impl From<LnUrlError> for LnUrlAuthError {
        fn from(value: LnUrlError) -> Self {
            match value {
                LnUrlError::InvalidUri(err) => Self::InvalidUri { err },
                LnUrlError::ServiceConnectivity(err) => Self::ServiceConnectivity { err },
                _ => Self::Generic {
                    err: value.to_string(),
                },
            }
        }
    }
}
