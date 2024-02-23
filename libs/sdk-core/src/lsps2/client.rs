use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::lsps0;

#[derive(Debug, Serialize, Deserialize)]
struct GetVersionsRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVersionsResponse {
    pub versions: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInfoRequest {
    pub version: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GetInfoResponse {
    pub opening_fee_params_menu: Vec<OpeningFeeParams>,

    #[serde_as(as = "DisplayFromStr")]
    pub min_payment_size_msat: u64,

    #[serde_as(as = "DisplayFromStr")]
    pub max_payment_size_msat: u64,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OpeningFeeParams {
    #[serde_as(as = "DisplayFromStr")]
    pub min_fee_msat: u64,
    pub proportional: u32,
    pub valid_until: String,
    pub min_lifetime: u32,
    pub max_client_to_self_delay: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub min_payment_size_msat: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub max_payment_size_msat: u64,
    pub promise: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct BuyRequest {
    pub version: i32,
    pub opening_fee_params: OpeningFeeParams,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub payment_size_msat: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BuyResponse {
    pub jit_channel_scid: String,
    pub lsp_cltv_expiry_delta: u32,

    #[serde(default)]
    pub client_trusts_lsp: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum GetInfoError {
    #[error("lsps2.get_info unsupported_version error: {0:?}")]
    UnsupportedVersion(lsps0::jsonrpc::RpcError),
    #[error("lsps2.get_info unrecognized_or_stale_token error: {0:?}")]
    UnrecognizedOrStaleToken(lsps0::jsonrpc::RpcError),
    #[error("lsps2.get_info general error: {0}")]
    Lsps0(lsps0::Error),
}

impl From<lsps0::Error> for GetInfoError {
    fn from(value: lsps0::Error) -> Self {
        match value {
            lsps0::Error::Remote(e) => match e.code {
                1 => Self::UnsupportedVersion(e),
                2 => Self::UnrecognizedOrStaleToken(e),
                _ => Self::Lsps0(lsps0::Error::Remote(e)),
            },
            _ => Self::Lsps0(value),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BuyError {
    #[error("lsps2.buy unsupported_version error: {0:?}")]
    UnsupportedVersion(lsps0::jsonrpc::RpcError),

    #[error("lsps2.buy invalid_opening_fee_params error: {0:?}")]
    InvalidOpeningFeeParams(lsps0::jsonrpc::RpcError),

    #[error("lsps2.buy payment_size_too_small error: {0:?}")]
    PaymentSizeTooSmall(lsps0::jsonrpc::RpcError),

    #[error("lsps2.buy payment_size_too_large error: {0:?}")]
    PaymentSizeTooLarge(lsps0::jsonrpc::RpcError),

    #[error("lsps2.buy general error: {0}")]
    Lsps0(lsps0::Error),
}

impl From<lsps0::Error> for BuyError {
    fn from(value: lsps0::Error) -> Self {
        match value {
            lsps0::Error::Remote(e) => match e.code {
                1 => Self::UnsupportedVersion(e),
                2 => Self::InvalidOpeningFeeParams(e),
                3 => Self::PaymentSizeTooSmall(e),
                4 => Self::PaymentSizeTooLarge(e),
                _ => Self::Lsps0(lsps0::Error::Remote(e)),
            },
            _ => Self::Lsps0(value),
        }
    }
}
pub struct Client {
    client: lsps0::Client,
}

impl Client {
    #[allow(dead_code)]
    pub fn new(client: lsps0::Client) -> Self {
        Self { client }
    }

    #[allow(dead_code)]
    pub async fn get_versions(&self) -> Result<GetVersionsResponse, lsps0::Error> {
        self.client
            .call(String::from("lsps2.get_versions"), GetVersionsRequest {})
            .await
    }

    #[allow(dead_code)]
    pub async fn get_info(&self, req: GetInfoRequest) -> Result<GetInfoResponse, GetInfoError> {
        match self.client.call(String::from("lsps2.get_info"), req).await {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    #[allow(dead_code)]
    pub async fn buy(&self, req: BuyRequest) -> Result<BuyResponse, BuyError> {
        match self.client.call(String::from("lsps2.buy"), req).await {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lsps2::client::{
        BuyRequest, BuyResponse, GetInfoRequest, GetInfoResponse, GetVersionsResponse,
        OpeningFeeParams,
    };

    #[test]
    fn test_get_versions_response_deserialize() {
        let json = r#"{"versions":[1]}"#;
        let result = serde_json::from_str::<GetVersionsResponse>(json).unwrap();
        assert_eq!(vec![1], result.versions)
    }

    #[test]
    fn test_get_info_request_serialize_with_token() {
        let req = GetInfoRequest {
            token: Some(String::from("token")),
            version: 1,
        };
        let result = serde_json::to_string(&req).unwrap();
        assert_eq!(r#"{"version":1,"token":"token"}"#, result)
    }

    #[test]
    fn test_get_info_request_serialize_without_token() {
        let req = GetInfoRequest {
            token: None,
            version: 1,
        };
        let result = serde_json::to_string(&req).unwrap();
        assert_eq!(r#"{"version":1}"#, result)
    }

    #[test]
    fn test_get_info_response_deserialize() {
        let json = r#"{
            "opening_fee_params_menu": [
                {
                    "min_fee_msat": "546000",
                    "proportional": 1200,
                    "valid_until": "2023-02-23T08:47:30.511Z",
                    "min_lifetime": 1008,
                    "max_client_to_self_delay": 2016,
                    "min_payment_size_msat": "546000",
                    "max_payment_size_msat": "4000000000",
                    "promise": "abcdefghijklmnopqrstuvwxyz"
                },
                {
                    "min_fee_msat": "1092000",
                    "proportional": 2400,
                    "valid_until": "2023-02-27T21:23:57.984Z",
                    "min_lifetime": 1008,
                    "max_client_to_self_delay": 2016,
                    "min_payment_size_msat": "1092000",
                    "max_payment_size_msat": "4000000000",
                    "promise": "abcdefghijklmnopqrstuvwxyz"
                }
            ]
        }"#;

        let result = serde_json::from_str::<GetInfoResponse>(json).unwrap();
        assert_eq!(
            result,
            GetInfoResponse {
                max_payment_size_msat: 1000000,
                min_payment_size_msat: 1000,
                opening_fee_params_menu: vec![
                    OpeningFeeParams {
                        min_fee_msat: 546000,
                        proportional: 1200,
                        valid_until: String::from("2023-02-23T08:47:30.511Z"),
                        min_lifetime: 1008,
                        max_client_to_self_delay: 2016,
                        min_payment_size_msat: 546000,
                        max_payment_size_msat: 4_000_000_000,
                        promise: String::from("abcdefghijklmnopqrstuvwxyz")
                    },
                    OpeningFeeParams {
                        min_fee_msat: 1092000,
                        proportional: 2400,
                        valid_until: String::from("2023-02-27T21:23:57.984Z"),
                        min_lifetime: 1008,
                        max_client_to_self_delay: 2016,
                        min_payment_size_msat: 1092000,
                        max_payment_size_msat: 4_000_000_000,
                        promise: String::from("abcdefghijklmnopqrstuvwxyz")
                    },
                ]
            }
        )
    }

    #[test]
    fn test_buy_request_serialize_with_payment_size() {
        let req = BuyRequest {
            version: 1,
            opening_fee_params: OpeningFeeParams {
                min_fee_msat: 546000,
                proportional: 1200,
                valid_until: String::from("2023-02-23T08:47:30.511Z"),
                min_lifetime: 1008,
                max_client_to_self_delay: 2016,
                min_payment_size_msat: 546000,
                max_payment_size_msat: 4_000_000_000,
                promise: String::from("abcdefghijklmnopqrstuvwxyz"),
            },
            payment_size_msat: Some(42000),
        };
        let result = serde_json::to_string(&req).unwrap();
        assert_eq!(
            r#"{"version":1,"opening_fee_params":{"min_fee_msat":"546000","proportional":1200,"valid_until":"2023-02-23T08:47:30.511Z","min_lifetime":1008,"max_client_to_self_delay":2016,"min_payment_size_msat":"546000","max_payment_size_msat":"4000000000","promise":"abcdefghijklmnopqrstuvwxyz"},"payment_size_msat":"42000"}"#,
            result
        )
    }

    #[test]
    fn test_buy_request_serialize_without_payment_size() {
        let req = BuyRequest {
            version: 1,
            opening_fee_params: OpeningFeeParams {
                min_fee_msat: 546000,
                proportional: 1200,
                valid_until: String::from("2023-02-23T08:47:30.511Z"),
                min_lifetime: 1008,
                max_client_to_self_delay: 2016,
                min_payment_size_msat: 546000,
                max_payment_size_msat: 4_000_000_000,
                promise: String::from("abcdefghijklmnopqrstuvwxyz"),
            },
            payment_size_msat: None,
        };
        let result = serde_json::to_string(&req).unwrap();
        assert_eq!(
            r#"{"version":1,"opening_fee_params":{"min_fee_msat":"546000","proportional":1200,"valid_until":"2023-02-23T08:47:30.511Z","min_lifetime":1008,"max_client_to_self_delay":2016,"min_payment_size_msat":"546000","max_payment_size_msat":"4000000000","promise":"abcdefghijklmnopqrstuvwxyz"}}"#,
            result
        )
    }

    #[test]
    fn test_buy_response_deserialize_with_client_trusts_lsp_false() {
        let json = r#"{
            "jit_channel_scid": "1x4815x29451",
            "lsp_cltv_expiry_delta" : 144,
            "client_trusts_lsp": false
        }"#;

        let result = serde_json::from_str::<BuyResponse>(json).unwrap();
        assert_eq!(
            result,
            BuyResponse {
                client_trusts_lsp: false,
                jit_channel_scid: String::from("1x4815x29451"),
                lsp_cltv_expiry_delta: 144
            }
        );
    }

    #[test]
    fn test_buy_response_deserialize_with_client_trusts_lsp_true() {
        let json = r#"{
            "jit_channel_scid": "1x4815x29451",
            "lsp_cltv_expiry_delta" : 144,
            "client_trusts_lsp": true
        }"#;

        let result = serde_json::from_str::<BuyResponse>(json).unwrap();
        assert_eq!(
            result,
            BuyResponse {
                client_trusts_lsp: true,
                jit_channel_scid: String::from("1x4815x29451"),
                lsp_cltv_expiry_delta: 144
            }
        );
    }

    #[test]
    fn test_buy_response_deserialize_without_client_trusts_lsp() {
        let json = r#"{
            "jit_channel_scid": "1x4815x29451",
            "lsp_cltv_expiry_delta": 144
        }"#;

        let result = serde_json::from_str::<BuyResponse>(json).unwrap();
        assert_eq!(
            result,
            BuyResponse {
                client_trusts_lsp: false,
                jit_channel_scid: String::from("1x4815x29451"),
                lsp_cltv_expiry_delta: 144
            }
        );
    }
}
