use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

use anyhow::{anyhow, Result};
use bitcoin::Txid;
use serde_json::json;

use const_format::concatcp;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Body, Client};

use crate::input_parser::get_parse_and_log_response;
use crate::models::ReverseSwapPairInfo;
use crate::reverseswap::CreateReverseSwapResponse;
use crate::{log_debug, log_trace, Logger, ReverseSwapServiceAPI};

const BOLTZ_API_URL: &str = "https://api.boltz.exchange/";
const GET_PAIRS_ENDPOINT: &str = concatcp!(BOLTZ_API_URL, "getpairs");
const GET_SWAP_STATUS_ENDPOINT: &str = concatcp!(BOLTZ_API_URL, "swapstatus");
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

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum BoltzApiCreateReverseSwapResponse {
    /// Success response by the Boltz API, indicating reverse swap was created successfully
    BoltzApiSuccess(CreateReverseSwapResponse),

    /// Error response by the Boltz API, indicating there was an issue with creating the reverse swap
    BoltzApiError { error: String },
}

/// Details of the lock tx, as reported by the Boltz endpoint
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LockTxData {
    id: Txid,
    hex: String,
    eta: Option<u32>,
}

/// Possible states of a Reverse Swap, as reported by the Boltz endpoint.
///
/// Note that Some Boltz statuses are not reflected here, for any of the following reasons:
/// - we're not using that version of the reverse swap protocol (like `channel.created`,
/// `transaction.zeroconf.rejected` for zero-conf, or `invoice.pending` and `minerfee.paid` for
/// Reverse Swap with prepay miner fee where)
/// - the statuses refer to normal swaps, not reverse swaps (like `invoice.set`, `invoice.paid`,
/// `invoice.failedToPay`, `transaction.claimed`)
/// - the statuses affect only non-BTC pairs (like `transaction.lockupFailed`)
///
/// https://docs.boltz.exchange/en/latest/lifecycle/#reverse-submarine-swaps
///
/// https://docs.boltz.exchange/en/latest/api/#getting-status-of-a-swap
///
/// https://github.com/BoltzExchange/boltz-backend/blob/78ad326db142a6180c0153a43056efd4ea6ced97/lib/consts/Enums.ts#L25-L52
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum BoltzApiReverseSwapStatus {
    /// Initial status of a reverse swap. Reverse swap was created on Boltz, but the Breez SDK has
    /// not (yet) locked the funds by paying the HODL invoice.
    #[serde(rename = "swap.created")]
    SwapCreated,

    /// The timelock expires before the HODL invoice is paid
    #[serde(rename = "swap.expired")]
    SwapExpired,

    /// The HODL invoice has been paid (pending settlement), lockup tx is in the mempool
    #[serde(rename = "transaction.mempool")]
    LockTxMempool { transaction: LockTxData },

    /// The lockup tx has at least one confirmation
    #[serde(rename = "transaction.confirmed")]
    LockTxConfirmed { transaction: LockTxData },

    /// If Boltz is unable to send the agreed amount of onchain coins after the invoice is paid, the
    /// status will become `transaction.failed` and the pending lightning HTLC will be cancelled.
    #[serde(rename = "transaction.failed")]
    LockTxFailed,

    /// The HODL invoice was paid, but the timelock expired. In this case, the invoice expires
    /// and the funds are returned to the sender.
    #[serde(rename = "transaction.refunded")]
    #[serde(rename_all = "camelCase")]
    LockTxRefunded { failure_reason: String },

    /// Claim tx was seen in the mempool, HODL invoice was settled
    #[serde(rename = "invoice.settled")]
    InvoiceSettled,

    #[serde(rename = "invoice.expired")]
    InvoiceExpired,
}

pub struct BoltzApi {
    pub logger: Arc<Box<dyn Logger>>,
}

#[tonic::async_trait]
impl ReverseSwapServiceAPI for BoltzApi {
    async fn fetch_reverse_swap_fees(&self) -> Result<ReverseSwapPairInfo> {
        reverse_swap_pair_info(self.logger.clone()).await
    }

    /// Call Boltz API and parse response as per https://docs.boltz.exchange/en/latest/api/#creating-reverse-swaps
    ///
    /// #### Errors
    ///
    /// This method returns an error for  HTTP or connection errors (404 not found, 400 bad request,
    /// 502 server error, etc).
    ///
    /// Boltz API errors (e.g. if the reverse swap could not be created, for example if the amount is too low)
    /// are returned as a successful response of type [BoltzApiCreateReverseSwapResponse::BoltzApiError]
    async fn create_reverse_swap_on_remote(
        &self,
        amount_sat: u64,
        preimage_hash_hex: String,
        claim_pubkey: String,
        pair_hash: String,
        routing_node: String,
    ) -> Result<BoltzApiCreateReverseSwapResponse> {
        Client::new()
            .post(CREATE_REVERSE_SWAP_ENDPOINT)
            .header(CONTENT_TYPE, "application/json")
            .body(build_boltz_reverse_swap_args(
                amount_sat,
                preimage_hash_hex,
                pair_hash.clone(),
                claim_pubkey.clone(),
                routing_node,
            ))
            .send()
            .await?
            .text()
            .await
            .map_err(|e| anyhow!("Failed to request creation of reverse swap: {e}"))
            .and_then(|res| {
                log_trace!(
                    self.logger,
                    "Boltz API create raw response {}",
                    to_string_pretty(&res)?
                );
                serde_json::from_str::<BoltzApiCreateReverseSwapResponse>(&res)
                    .map_err(|e| anyhow!("Failed to parse crate swap response: {e}"))
            })
    }

    /// Call and parse response as per https://docs.boltz.exchange/en/latest/api/#getting-status-of-a-swap
    ///
    /// #### Errors
    ///
    /// This method returns an error for  HTTP or connection errors (404 not found, 400 bad request,
    /// 502 server error, etc).
    ///
    /// Boltz API errors (e.g. providing an invalid ID arg) are returned as a successful response of
    /// type [BoltzApiCreateReverseSwapResponse::BoltzApiError]
    async fn get_boltz_status(&self, id: String) -> Result<BoltzApiReverseSwapStatus> {
        Client::new()
            .post(GET_SWAP_STATUS_ENDPOINT)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(json!({ "id": id }).to_string()))
            .send()
            .await?
            .text()
            .await
            .map_err(|e| anyhow!("Failed to request swap status: {e}"))
            .and_then(|res| {
                log_trace!(
                    self.logger,
                    "Boltz API status raw response {}",
                    to_string_pretty(&res)?
                );
                serde_json::from_str::<BoltzApiReverseSwapStatus>(&res)
                    .map_err(|e| anyhow!("Failed to parse get status response: {e}"))
            })
    }
}

pub async fn reverse_swap_pair_info(logger: Arc<Box<dyn Logger>>) -> Result<ReverseSwapPairInfo> {
    let pairs: Pairs = get_parse_and_log_response(GET_PAIRS_ENDPOINT, Some(logger.clone())).await?;
    match pairs.pairs.get("BTC/BTC") {
        None => Err(anyhow!("BTC pair not found")),
        Some(btc_pair) => {
            log_debug!(
                logger,
                "Boltz API pair: {}",
                serde_json::to_string_pretty(&btc_pair)?
            );
            let hash = String::from(&btc_pair.hash);
            Ok(ReverseSwapPairInfo {
                fees_hash: hash,
                min: btc_pair.limits.minimal,
                max: btc_pair.limits.maximal,
                fees_percentage: btc_pair.fees.percentage,
                fees_lockup: btc_pair.fees.miner_fees.base_asset.reverse.lockup,
                fees_claim: btc_pair.fees.miner_fees.base_asset.reverse.claim,
                total_estimated_fees: None,
            })
        }
    }
}

fn build_boltz_reverse_swap_args(
    amount_sat: u64,
    preimage_hash_hex: String,
    pair_hash: String,
    claim_pubkey: String,
    routing_node: String,
) -> String {
    json!({
        "type": "reversesubmarine",
        "pairId": "BTC/BTC",
        "orderSide": "buy",
        "invoiceAmount": amount_sat,
        "preimageHash": preimage_hash_hex,
        "pairHash": pair_hash,
        "claimPublicKey": claim_pubkey,
        "routingNode": routing_node
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use crate::boltzswap::*;
    use bitcoin::Txid;
    use std::str::FromStr;

    #[test]
    fn test_boltz_status_deserialize() {
        assert!(matches!(
            serde_json::from_str(
                r#"
                {
                    "status": "swap.created"
                }"#
            ),
            Ok(BoltzApiReverseSwapStatus::SwapCreated)
        ));

        let id = Txid::from_str("71aa5902960e453491c4531f26d3602ae31af220dbb1d86d0ec4fa6056ab77b7")
            .unwrap();
        let hex: String = "0100000000010177c9bf7b1a206d1e4ceb48d1d9efd8de4d66e1e4bf1b3db85cb73f6c6782e0c30000000000ffffffff02cfae000000000000220020befd7d08cf438d51f20879d1d9ef50e53abcd769ccb11a61adcf4207224c19926c8f2c010000000022512053f1fd711325372f39603d6f2be048a39333c9bddd57de3c03a30687d759694801405c5ab7ddbbffaffc255477bedacbad2db2061efa7fea7659430e35107bb8e8fad535b1dfd8816d52a3a336e277e137f328d23383bdb275839af5fe554ea3247b00000000".into();
        assert!(matches!(
            serde_json::from_str(
                r#"
                {
                    "status":"transaction.mempool",
                    "transaction":
                    {
                        "id":"71aa5902960e453491c4531f26d3602ae31af220dbb1d86d0ec4fa6056ab77b7",
                        "hex":"0100000000010177c9bf7b1a206d1e4ceb48d1d9efd8de4d66e1e4bf1b3db85cb73f6c6782e0c30000000000ffffffff02cfae000000000000220020befd7d08cf438d51f20879d1d9ef50e53abcd769ccb11a61adcf4207224c19926c8f2c010000000022512053f1fd711325372f39603d6f2be048a39333c9bddd57de3c03a30687d759694801405c5ab7ddbbffaffc255477bedacbad2db2061efa7fea7659430e35107bb8e8fad535b1dfd8816d52a3a336e277e137f328d23383bdb275839af5fe554ea3247b00000000",
                        "eta":2
                    }
                }"#
            ),
            Ok(BoltzApiReverseSwapStatus::LockTxMempool {
                transaction: LockTxData {
                    id: id_temp,
                    hex: hex_temp,
                    eta: Some(2)
                }
            })
            if id_temp == id && hex_temp == hex
        ));

        assert!(matches!(
            serde_json::from_str(
                r#"
                {
                    "status":"transaction.confirmed",
                    "transaction":
                    {
                        "id":"71aa5902960e453491c4531f26d3602ae31af220dbb1d86d0ec4fa6056ab77b7",
                        "hex":"0100000000010177c9bf7b1a206d1e4ceb48d1d9efd8de4d66e1e4bf1b3db85cb73f6c6782e0c30000000000ffffffff02cfae000000000000220020befd7d08cf438d51f20879d1d9ef50e53abcd769ccb11a61adcf4207224c19926c8f2c010000000022512053f1fd711325372f39603d6f2be048a39333c9bddd57de3c03a30687d759694801405c5ab7ddbbffaffc255477bedacbad2db2061efa7fea7659430e35107bb8e8fad535b1dfd8816d52a3a336e277e137f328d23383bdb275839af5fe554ea3247b00000000"
                    }
                }"#
            ),
            Ok(BoltzApiReverseSwapStatus::LockTxConfirmed {
                transaction: LockTxData { id: id_temp, hex: hex_temp, eta: None }
            })
            if id_temp == id && hex_temp == hex
        ));

        let failure_reason : String = "refunded onchain coins: 71aa5902960e453491c4531f26d3602ae31af220dbb1d86d0ec4fa6056ab77b7".into();
        assert!(matches!(
            serde_json::from_str(
                r#"
                {
                    "status":"transaction.refunded",
                    "failureReason":"refunded onchain coins: 71aa5902960e453491c4531f26d3602ae31af220dbb1d86d0ec4fa6056ab77b7"
                }"#
            ),
            Ok(BoltzApiReverseSwapStatus::LockTxRefunded { failure_reason: fr }) if fr == failure_reason
        ));
    }
}
