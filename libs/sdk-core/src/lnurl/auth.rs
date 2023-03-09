use crate::{mnemonic_to_seed, LnUrlAuthRequestData, LnUrlWithdrawCallbackStatus};
use anyhow::{anyhow, Result};
use bitcoin::secp256k1::{Message, Secp256k1};
use bitcoin::util::bip32::{ChildNumber, DerivationPath, ExtendedPrivKey};
use bitcoin::{KeyPair, Network};
use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::{sha256, Hash, HashEngine, Hmac, HmacEngine};
use reqwest::Url;
use std::str::FromStr;

use LnUrlWithdrawCallbackStatus as LnUrlAuthCallbackStatus;

/// Performs the second and last step of LNURL-auth, as per
/// <https://github.com/lnurl/luds/blob/luds/04.md>
///
/// See the [parse] docs for more detail on the full workflow.
pub(crate) async fn perform_lnurl_auth(
    network: Network,
    req_data: LnUrlAuthRequestData,
) -> Result<LnUrlAuthCallbackStatus> {
    // TODO Clarify how we get the seed. As arg?
    let master_prv_key = mnemonic_to_seed("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string())?;
    let linking_keys = derive_linking_keys(network, master_prv_key, Url::from_str(&req_data.url)?)?;
    let k1_to_sign = Message::from_slice(&hex::decode(req_data.k1)?)?;
    let sig = Secp256k1::new().sign_ecdsa(&k1_to_sign, &linking_keys.secret_key());

    // <LNURL_hostname_and_path>?<LNURL_existing_query_parameters>&sig=<hex(sign(utf8ToBytes(k1), linkingPrivKey))>&key=<hex(linkingKey)>
    let mut callback_url = Url::from_str(&req_data.url)?;
    callback_url
        .query_pairs_mut()
        .append_pair("sig", &sig.serialize_der().to_hex());
    callback_url
        .query_pairs_mut()
        .append_pair("key", &linking_keys.public_key().to_hex());
    debug!("Trying to call {}", callback_url.to_string());

    let callback_resp_text = reqwest::get(callback_url).await?.text().await?;
    serde_json::from_str::<LnUrlAuthCallbackStatus>(&callback_resp_text).map_err(|e| anyhow!(e))
}

pub(crate) fn validate_request(
    domain: String,
    lnurl_endpoint: String,
) -> Result<LnUrlAuthRequestData> {
    let query = Url::from_str(&lnurl_endpoint)?;
    let query_pairs = query.query_pairs();

    let k1 = query_pairs
        .into_iter()
        .find(|(key, _)| key == "k1")
        .map(|(_, v)| v.to_string())
        .ok_or(anyhow!("LNURL-auth k1 arg not found"))?;

    let maybe_action = query_pairs
        .into_iter()
        .find(|(key, _)| key == "action")
        .map(|(_, v)| v.to_string());

    let k1_bytes = hex::decode(&k1)?;
    if k1_bytes.len() != 32 {
        return Err(anyhow!("LNURL-auth k1 is of unexpected length"));
    }

    if let Some(action) = &maybe_action {
        if !["register", "login", "link", "auth"].contains(&action.as_str()) {
            return Err(anyhow!("LNURL-auth action is of unexpected type"));
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

/// Linking key is derived as per LUD-05
///
/// https://github.com/lnurl/luds/blob/luds/05.md
fn derive_linking_keys(network: Network, seed: Vec<u8>, url: Url) -> Result<KeyPair> {
    let domain = url.domain().ok_or(anyhow!("Could not determine domain"))?;
    let ctx = Secp256k1::new();

    let root_key = ExtendedPrivKey::new_master(network, &seed)?;
    let hashing_key = root_key.derive_priv(&ctx, &"m/138'/0".parse::<DerivationPath>()?)?;

    let hmac = hmac_sha256(&hashing_key.to_priv().to_bytes(), domain.as_bytes());
    let hmac_bytes = hmac.as_inner();

    // m/138'/<long1>/<long2>/<long3>/<long4>
    let linking_key = root_key.derive_priv(
        &ctx,
        &vec![
            ChildNumber::from_hardened_idx(138)?,
            ChildNumber::from(build_path_element_u32(hmac_bytes[0..4].try_into()?)),
            ChildNumber::from(build_path_element_u32(hmac_bytes[4..8].try_into()?)),
            ChildNumber::from(build_path_element_u32(hmac_bytes[8..12].try_into()?)),
            ChildNumber::from(build_path_element_u32(hmac_bytes[12..16].try_into()?)),
        ],
    )?;

    Ok(linking_key.to_keypair(&ctx))
}

fn build_path_element_u32(hmac_bytes: [u8; 4]) -> u32 {
    let mut buf = [0u8; 4];
    buf[..4].copy_from_slice(&hmac_bytes);
    u32::from_be_bytes(buf)
}
