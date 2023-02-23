use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Result};

use const_format::concatcp;

use crate::models::ReverseSwapInfo;

const BOLTZ_API_URL: &str = "https://boltz.exchange/api/";
const GET_PAIRS_ENDPOINT: &str = concatcp!(BOLTZ_API_URL, "getpairs");
pub(crate) const CREATE_REVERSE_SWAP_ENDPOINT: &str = concatcp!(BOLTZ_API_URL, "createswap");

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MaximalZeroConf {
    base_asset: u64,
    quote_asset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Limits {
    maximal: u64,
    minimal: u64,
    maximal_zero_conf: MaximalZeroConf,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReverseFeesAsset {
    lockup: u64,
    claim: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct FeesAsset {
    normal: u64,
    reverse: ReverseFeesAsset,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MinerFees {
    base_asset: FeesAsset,
    quote_asset: FeesAsset,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fees {
    percentage: f64,
    miner_fees: MinerFees,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pair {
    rate: f64,
    hash: String,
    limits: Limits,
    fees: Fees,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pairs {
    warnings: Vec<String>,
    info: Vec<String>,
    pairs: HashMap<String, Pair>,
}

pub async fn reverse_swap_info() -> Result<ReverseSwapInfo> {
    let pairs = reqwest::get(GET_PAIRS_ENDPOINT)
        .await?
        .json::<Pairs>()
        .await?;
    match pairs.pairs.get("BTC/BTC") {
        None => Err(anyhow!("BTC pair not found")),
        Some(btc_pair) => {
            println!("result: {}", serde_json::to_string_pretty(&btc_pair)?);
            let hash = String::from(&btc_pair.hash);
            Ok(ReverseSwapInfo {
                fees_hash: hash,
                min: btc_pair.limits.minimal,
                max: btc_pair.limits.maximal,
                fees_percentage: btc_pair.fees.percentage,
                fees_lockup: btc_pair.fees.miner_fees.base_asset.reverse.lockup,
                fees_claim: btc_pair.fees.miner_fees.base_asset.reverse.claim,
            })
        }
    }
}
