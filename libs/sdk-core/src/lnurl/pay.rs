use sdk_common::prelude::*;
use serde::Serialize;

use crate::Payment;

/// Contains the result of the entire LNURL-pay interaction, as reported by the LNURL endpoint.
///
/// * `EndpointSuccess` indicates the payment is complete. The endpoint may return a `SuccessActionProcessed`,
///   in which case, the wallet has to present it to the user as described in
///   <https://github.com/lnurl/luds/blob/luds/09.md>
///
/// * `EndpointError` indicates a generic issue the LNURL endpoint encountered, including a freetext
///   field with the reason.
///
/// * `PayError` indicates that an error occurred while trying to pay the invoice from the LNURL endpoint.
///   This includes the payment hash of the failed invoice and the failure reason.
#[derive(Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum LnUrlPayResult {
    EndpointSuccess { data: LnUrlPaySuccessData },
    EndpointError { data: LnUrlErrorData },
    PayError { data: LnUrlPayErrorData },
}

#[derive(Serialize)]
pub struct LnUrlPaySuccessData {
    pub payment: Payment,
    pub success_action: Option<SuccessActionProcessed>,
}

#[cfg(test)]
pub(crate) mod tests {
    use std::sync::Arc;

    use anyhow::{anyhow, Result};
    use gl_client::bitcoin::hashes::hex::ToHex;
    use gl_client::pb::cln::pay_response::PayStatus;
    use rand::random;
    use serde_json::json;

    use crate::bitcoin::hashes::{sha256, Hash};
    use crate::breez_services::tests::{breez_services_with, get_dummy_node_state};
    use crate::lnurl::pay::*;
    use crate::{test_utils::*, LnUrlPayRequest};

    struct LnurlPayCallbackParams {
        error: Option<String>,
        pr: Option<String>,
    }

    struct AesPayCallbackParams {
        error: Option<String>,
        pr: Option<String>,
        sa_data: AesSuccessActionDataDecrypted,
        iv_bytes: [u8; 16],
        key_bytes: [u8; 32],
    }

    /// Mock an LNURL-pay endpoint that responds with no Success Action
    fn mock_lnurl_pay_callback_endpoint_no_success_action(
        mock_rest_client: &MockRestClient,
        callback_params: LnurlPayCallbackParams,
    ) {
        let LnurlPayCallbackParams { error, pr } = callback_params;

        let response_body = match error {
            None => json!({
                "pr": pr.unwrap_or_else(|| "token-invoice".to_string()),
                "routes": []
            })
            .to_string(),
            Some(err_reason) => json!({
                "status": "ERROR",
                "reason": err_reason
            })
            .to_string(),
        };

        mock_rest_client.add_response(MockResponse::new(200, response_body));
    }

    /// Mock an LNURL-pay endpoint that responds with an unsupported Success Action
    fn mock_lnurl_pay_callback_endpoint_unsupported_success_action(
        mock_rest_client: &MockRestClient,
        callback_params: LnurlPayCallbackParams,
    ) {
        let LnurlPayCallbackParams { error, pr } = callback_params;

        let response_body = match error {
            None => json!({
                "pr": pr.unwrap_or_else(|| "token-invoice".to_string()),
                "routes": [],
                "successAction": {
                    "tag": "random-type-that-is-not-supported",
                    "message": "test msg"
                }
            })
            .to_string(),
            Some(err_reason) => json!({
                "status": "ERROR",
                "reason": err_reason
            })
            .to_string(),
        };

        mock_rest_client.add_response(MockResponse::new(200, response_body));
    }

    /// Mock an LNURL-pay endpoint that responds with a Success Action of type message
    fn mock_lnurl_pay_callback_endpoint_msg_success_action(
        mock_rest_client: &MockRestClient,
        callback_params: LnurlPayCallbackParams,
    ) {
        let LnurlPayCallbackParams { error, pr } = callback_params;

        let response_body = match error {
            None => json!({
                "pr": pr.unwrap_or_else(|| "token-invoice".to_string()),
                "routes":[],
                "successAction": {
                    "tag": "message",
                    "message": "test msg"
                }
            })
            .to_string(),
            Some(err_reason) => json!({
                "status": "ERROR",
                "reason": err_reason
            })
            .to_string(),
        };

        mock_rest_client.add_response(MockResponse::new(200, response_body));
    }

    /// Mock an LNURL-pay endpoint that responds with a Success Action of type URL
    fn mock_lnurl_pay_callback_endpoint_url_success_action(
        mock_rest_client: &MockRestClient,
        callback_params: LnurlPayCallbackParams,
        success_action_url: Option<&str>,
    ) {
        let LnurlPayCallbackParams { error, pr } = callback_params;

        let response_body = match error {
            None => json!({
                "pr": pr.unwrap_or_else(|| "token-invoice".to_string()),
                "routes":[],
                "successAction": {
                    "tag": "url",
                    "description": "test description",
                    "url": success_action_url.unwrap_or("http://localhost:8080/test-url"),
                }
            })
            .to_string(),
            Some(err_reason) => json!({
                "status": "ERROR",
                "reason": err_reason
            })
            .to_string(),
        };

        mock_rest_client.add_response(MockResponse::new(200, response_body));
    }

    /// Mock an LNURL-pay endpoint that responds with a Success Action of type AES
    fn mock_lnurl_pay_callback_endpoint_aes_success_action(
        mock_rest_client: &MockRestClient,
        aes_callback_params: AesPayCallbackParams,
    ) {
        let AesPayCallbackParams {
            error,
            pr,
            sa_data,
            iv_bytes,
            key_bytes,
        } = aes_callback_params;

        let iv_base64 = base64::encode(iv_bytes);
        let cipertext =
            AesSuccessActionData::encrypt(&key_bytes, &iv_bytes, sa_data.plaintext).unwrap();

        let response_body = match error {
            None => json!({
                "pr": pr.unwrap_or_else(|| "token-invoice".to_string()),
                "routes": [],
                "successAction": {
                    "tag": "aes",
                    "description": sa_data.description,
                    "iv": iv_base64,
                    "ciphertext": cipertext
                }
            })
            .to_string(),
            Some(err_reason) => json!({
                "status": "ERROR",
                "reason": err_reason
            })
            .to_string(),
        };

        mock_rest_client.add_response(MockResponse::new(200, response_body));
    }

    fn get_test_pay_req_data(
        min_sendable: u64,
        max_sendable: u64,
        comment_len: u16,
    ) -> LnUrlPayRequestData {
        LnUrlPayRequestData {
            min_sendable,
            max_sendable,
            comment_allowed: comment_len,
            metadata_str: "".into(),
            callback: "http://localhost:8080/callback".into(),
            domain: "localhost".into(),
            allows_nostr: false,
            nostr_pubkey: None,
            ln_address: None,
        }
    }

    #[test]
    fn test_lnurl_pay_validate_invoice() -> Result<()> {
        let req = get_test_pay_req_data(0, 100_000, 0);
        let temp_desc = req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc.clone())?;
        let payreq: String = rand_invoice_with_description_hash(temp_desc)?.to_string();

        assert!(validate_invoice(
            inv.amount_milli_satoshis().unwrap(),
            &payreq,
            Network::Bitcoin
        )
        .is_ok());
        assert!(validate_invoice(
            inv.amount_milli_satoshis().unwrap() + 1000,
            &payreq,
            Network::Bitcoin,
        )
        .is_err());

        Ok(())
    }

    #[test]
    fn test_lnurl_pay_validate_invoice_network() -> Result<()> {
        let req = get_test_pay_req_data(0, 50_000, 0);
        let temp_desc = req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc.clone())?;
        let payreq: String = rand_invoice_with_description_hash(temp_desc)?.to_string();

        assert!(validate_invoice(
            inv.amount_milli_satoshis().unwrap(),
            &payreq,
            Network::Bitcoin,
        )
        .is_ok());
        assert!(validate_invoice(
            inv.amount_milli_satoshis().unwrap() + 1000,
            &payreq,
            Network::Bitcoin,
        )
        .is_err());

        Ok(())
    }

    #[test]
    fn test_lnurl_pay_validate_invoice_wrong_network() -> Result<()> {
        let req = get_test_pay_req_data(0, 25_000, 0);
        let temp_desc = req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc.clone())?;
        let payreq: String = rand_invoice_with_description_hash(temp_desc)?.to_string();

        assert!(validate_invoice(
            inv.amount_milli_satoshis().unwrap(),
            &payreq,
            Network::Testnet,
        )
        .is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_no_success_action() -> Result<()> {
        let mock_rest_client = MockRestClient::new();
        let comment = rand_string(COMMENT_LENGTH as usize);
        let pay_req = get_test_pay_req_data(0, 100_000, COMMENT_LENGTH);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_msat = inv.amount_milli_satoshis().unwrap();

        mock_lnurl_pay_callback_endpoint_no_success_action(
            &mock_rest_client,
            LnurlPayCallbackParams {
                error: None,
                pr: Some(inv.to_string()),
            },
        );

        let rest_client: Arc<dyn RestClient> = Arc::new(mock_rest_client);
        let mock_breez_services = breez_services_with(None, Some(rest_client), vec![]).await?;
        match mock_breez_services
            .lnurl_pay(LnUrlPayRequest {
                data: pay_req,
                amount_msat: user_amount_msat,
                use_trampoline: false,
                comment: Some(comment),
                payment_label: None,
                validate_success_action_url: None,
            })
            .await?
        {
            LnUrlPayResult::EndpointSuccess {
                data:
                    LnUrlPaySuccessData {
                        success_action: None,
                        ..
                    },
            } => Ok(()),
            LnUrlPayResult::EndpointSuccess {
                data:
                    LnUrlPaySuccessData {
                        success_action: Some(_),
                        ..
                    },
            } => Err(anyhow!("Unexpected success action")),
            _ => Err(anyhow!("Unexpected success action type")),
        }
    }

    static COMMENT_LENGTH: u16 = 10;

    #[tokio::test]
    async fn test_lnurl_pay_unsupported_success_action() -> Result<()> {
        let mock_rest_client = MockRestClient::new();
        let user_amount_msat = 11000;
        let comment = rand_string(COMMENT_LENGTH as usize);
        let pay_req = get_test_pay_req_data(0, 100_000, COMMENT_LENGTH);

        mock_lnurl_pay_callback_endpoint_unsupported_success_action(&mock_rest_client, LnurlPayCallbackParams {
                error: None,
                pr: Some("lnbc110n1p38q3gtpp5ypz09jrd8p993snjwnm68cph4ftwp22le34xd4r8ftspwshxhmnsdqqxqyjw5qcqpxsp5htlg8ydpywvsa7h3u4hdn77ehs4z4e844em0apjyvmqfkzqhhd2q9qgsqqqyssqszpxzxt9uuqzymr7zxcdccj5g69s8q7zzjs7sgxn9ejhnvdh6gqjcy22mss2yexunagm5r2gqczh8k24cwrqml3njskm548aruhpwssq9nvrvz".to_string()),
            });

        let rest_client: Arc<dyn RestClient> = Arc::new(mock_rest_client);
        let mock_breez_services = breez_services_with(None, Some(rest_client), vec![]).await?;
        let r = mock_breez_services
            .lnurl_pay(LnUrlPayRequest {
                data: pay_req,
                amount_msat: user_amount_msat,
                use_trampoline: false,
                comment: Some(comment),
                payment_label: None,
                validate_success_action_url: None,
            })
            .await;
        // An unsupported Success Action results in an error
        assert!(r.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_success_payment_hash() -> Result<()> {
        let mock_rest_client = MockRestClient::new();
        let comment = rand_string(COMMENT_LENGTH as usize);
        let pay_req = get_test_pay_req_data(0, 100_000, COMMENT_LENGTH);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_msat = inv.amount_milli_satoshis().unwrap();

        mock_lnurl_pay_callback_endpoint_msg_success_action(
            &mock_rest_client,
            LnurlPayCallbackParams {
                error: None,
                pr: Some(inv.to_string()),
            },
        );

        let rest_client: Arc<dyn RestClient> = Arc::new(mock_rest_client);
        let mock_breez_services = breez_services_with(None, Some(rest_client), vec![]).await?;
        match mock_breez_services
            .lnurl_pay(LnUrlPayRequest {
                data: pay_req,
                amount_msat: user_amount_msat,
                use_trampoline: false,
                comment: Some(comment),
                payment_label: None,
                validate_success_action_url: None,
            })
            .await?
        {
            LnUrlPayResult::EndpointSuccess { data } => match data.payment.id {
                s if s == inv.payment_hash().to_hex() => Ok(()),
                _ => Err(anyhow!("Unexpected payment hash")),
            },
            _ => Err(anyhow!("Unexpected result")),
        }
    }

    #[tokio::test]
    async fn test_lnurl_pay_msg_success_action() -> Result<()> {
        let mock_rest_client = MockRestClient::new();
        let comment = rand_string(COMMENT_LENGTH as usize);
        let pay_req = get_test_pay_req_data(0, 100_000, COMMENT_LENGTH);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_msat = inv.amount_milli_satoshis().unwrap();

        mock_lnurl_pay_callback_endpoint_msg_success_action(
            &mock_rest_client,
            LnurlPayCallbackParams {
                error: None,
                pr: Some(inv.to_string()),
            },
        );

        let rest_client: Arc<dyn RestClient> = Arc::new(mock_rest_client);
        let mock_breez_services = breez_services_with(None, Some(rest_client), vec![]).await?;
        match mock_breez_services
            .lnurl_pay(LnUrlPayRequest {
                data: pay_req,
                amount_msat: user_amount_msat,
                use_trampoline: false,
                comment: Some(comment),
                payment_label: None,
                validate_success_action_url: None,
            })
            .await?
        {
            LnUrlPayResult::EndpointSuccess {
                data:
                    LnUrlPaySuccessData {
                        success_action: None,
                        ..
                    },
            } => Err(anyhow!(
                "Expected success action in callback, but none provided"
            )),
            LnUrlPayResult::EndpointSuccess {
                data:
                    LnUrlPaySuccessData {
                        success_action: Some(SuccessActionProcessed::Message { data: msg }),
                        ..
                    },
            } => match msg.message {
                s if s == "test msg" => Ok(()),
                _ => Err(anyhow!("Unexpected success action message content")),
            },
            _ => Err(anyhow!("Unexpected success action type")),
        }
    }

    #[tokio::test]
    async fn test_lnurl_pay_msg_success_action_incorrect_amount() -> Result<()> {
        let mock_rest_client = MockRestClient::new();
        let comment = rand_string(COMMENT_LENGTH as usize);
        let pay_req = get_test_pay_req_data(0, 100_000, COMMENT_LENGTH);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_msat = inv.amount_milli_satoshis().unwrap() + 1000;

        mock_lnurl_pay_callback_endpoint_msg_success_action(
            &mock_rest_client,
            LnurlPayCallbackParams {
                error: None,
                pr: Some(inv.to_string()),
            },
        );

        let rest_client: Arc<dyn RestClient> = Arc::new(mock_rest_client);
        let mock_breez_services = breez_services_with(None, Some(rest_client), vec![]).await?;
        assert!(mock_breez_services
            .lnurl_pay(LnUrlPayRequest {
                data: pay_req,
                amount_msat: user_amount_msat,
                use_trampoline: false,
                comment: Some(comment),
                payment_label: None,
                validate_success_action_url: None,
            })
            .await
            .is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_msg_success_action_error_from_endpoint() -> Result<()> {
        let mock_rest_client = MockRestClient::new();
        let comment = rand_string(COMMENT_LENGTH as usize);
        let pay_req = get_test_pay_req_data(0, 100_000, COMMENT_LENGTH);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_msat = inv.amount_milli_satoshis().unwrap();
        let expected_error_msg = "Error message from LNURL endpoint";

        mock_lnurl_pay_callback_endpoint_msg_success_action(
            &mock_rest_client,
            LnurlPayCallbackParams {
                error: Some(expected_error_msg.to_string()),
                pr: Some(inv.to_string()),
            },
        );

        let rest_client: Arc<dyn RestClient> = Arc::new(mock_rest_client);
        let mock_breez_services = breez_services_with(None, Some(rest_client), vec![]).await?;
        let res = mock_breez_services
            .lnurl_pay(LnUrlPayRequest {
                data: pay_req,
                amount_msat: user_amount_msat,
                use_trampoline: false,
                comment: Some(comment),
                payment_label: None,
                validate_success_action_url: None,
            })
            .await;
        assert!(matches!(res, Ok(LnUrlPayResult::EndpointError { data: _ })));

        if let Ok(LnUrlPayResult::EndpointError { data: err_msg }) = res {
            assert_eq!(expected_error_msg, err_msg.reason);
        } else {
            return Err(anyhow!(
                "Expected error type but received another Success Action type"
            ));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_url_success_action() -> Result<()> {
        let mock_rest_client = MockRestClient::new();
        let comment = rand_string(COMMENT_LENGTH as usize);
        let pay_req = get_test_pay_req_data(0, 100_000, COMMENT_LENGTH);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_msat = inv.amount_milli_satoshis().unwrap();

        mock_lnurl_pay_callback_endpoint_url_success_action(
            &mock_rest_client,
            LnurlPayCallbackParams {
                error: None,
                pr: Some(inv.to_string()),
            },
            None,
        );

        let rest_client: Arc<dyn RestClient> = Arc::new(mock_rest_client);
        let mock_breez_services = breez_services_with(None, Some(rest_client), vec![]).await?;
        match mock_breez_services
            .lnurl_pay(LnUrlPayRequest {
                data: pay_req,
                amount_msat: user_amount_msat,
                use_trampoline: false,
                comment: Some(comment),
                payment_label: None,
                validate_success_action_url: None,
            })
            .await?
        {
            LnUrlPayResult::EndpointSuccess {
                data:
                    LnUrlPaySuccessData {
                        success_action: Some(SuccessActionProcessed::Url { data: url }),
                        ..
                    },
            } => {
                if url.url == "http://localhost:8080/test-url"
                    && url.description == "test description"
                {
                    Ok(())
                } else {
                    Err(anyhow!("Unexpected success action content"))
                }
            }
            LnUrlPayResult::EndpointSuccess {
                data:
                    LnUrlPaySuccessData {
                        success_action: None,
                        ..
                    },
            } => Err(anyhow!(
                "Expected success action in callback, but none provided"
            )),
            _ => Err(anyhow!("Unexpected success action type")),
        }
    }

    #[tokio::test]
    async fn test_lnurl_pay_url_success_action_validate_url_invalid() -> Result<()> {
        let mock_rest_client = MockRestClient::new();
        let comment = rand_string(COMMENT_LENGTH as usize);
        let pay_req = get_test_pay_req_data(0, 100_000, COMMENT_LENGTH);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_msat = inv.amount_milli_satoshis().unwrap();

        mock_lnurl_pay_callback_endpoint_url_success_action(
            &mock_rest_client,
            LnurlPayCallbackParams {
                error: None,
                pr: Some(inv.to_string()),
            },
            Some("http://different.localhost:8080/test-url"),
        );

        let rest_client: Arc<dyn RestClient> = Arc::new(mock_rest_client);
        let mock_breez_services = breez_services_with(None, Some(rest_client), vec![]).await?;
        let r = mock_breez_services
            .lnurl_pay(LnUrlPayRequest {
                data: pay_req,
                amount_msat: user_amount_msat,
                comment: Some(comment),
                payment_label: None,
                validate_success_action_url: Some(true),
                use_trampoline: false,
            })
            .await;
        // An invalid Success Action URL results in an error
        assert!(r.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_url_success_action_validate_url_valid() -> Result<()> {
        let mock_rest_client = MockRestClient::new();
        let comment = rand_string(COMMENT_LENGTH as usize);
        let pay_req = get_test_pay_req_data(0, 100_000, COMMENT_LENGTH);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_msat = inv.amount_milli_satoshis().unwrap();

        mock_lnurl_pay_callback_endpoint_url_success_action(
            &mock_rest_client,
            LnurlPayCallbackParams {
                error: None,
                pr: Some(inv.to_string()),
            },
            Some("http://different.localhost:8080/test-url"),
        );

        let rest_client: Arc<dyn RestClient> = Arc::new(mock_rest_client);
        let mock_breez_services = breez_services_with(None, Some(rest_client), vec![]).await?;
        match mock_breez_services
            .lnurl_pay(LnUrlPayRequest {
                data: pay_req,
                amount_msat: user_amount_msat,
                comment: Some(comment),
                payment_label: None,
                validate_success_action_url: Some(false),
                use_trampoline: false,
            })
            .await?
        {
            LnUrlPayResult::EndpointSuccess {
                data:
                    LnUrlPaySuccessData {
                        success_action: Some(SuccessActionProcessed::Url { data: url }),
                        ..
                    },
            } => {
                if url.url == "http://different.localhost:8080/test-url"
                    && url.description == "test description"
                {
                    Ok(())
                } else {
                    Err(anyhow!("Unexpected success action content"))
                }
            }
            LnUrlPayResult::EndpointSuccess {
                data:
                    LnUrlPaySuccessData {
                        success_action: None,
                        ..
                    },
            } => Err(anyhow!(
                "Expected success action in callback, but none provided"
            )),
            _ => Err(anyhow!("Unexpected success action type")),
        }
    }

    #[tokio::test]
    async fn test_lnurl_pay_aes_success_action() -> Result<()> {
        let mock_rest_client = MockRestClient::new();
        // Expected fields in the AES payload
        let description = "test description in AES payload".to_string();
        let plaintext = "Hello, test plaintext".to_string();
        let sa_data = AesSuccessActionDataDecrypted {
            description: description.clone(),
            plaintext: plaintext.clone(),
        };
        let sa = SuccessActionProcessed::Aes {
            result: AesSuccessActionDataResult::Decrypted {
                data: sa_data.clone(),
            },
        };

        // Generate preimage
        let preimage = sha256::Hash::hash(&rand_vec_u8(10));

        let comment = rand_string(COMMENT_LENGTH as usize);
        let pay_req = get_test_pay_req_data(0, 100_000, COMMENT_LENGTH);
        let temp_desc = pay_req.metadata_str.clone();

        // The invoice (served by LNURL-pay endpoint, matching preimage and description hash)
        let inv = rand_invoice_with_description_hash_and_preimage(temp_desc, preimage)?;

        let user_amount_msat = inv.amount_milli_satoshis().unwrap();
        let bolt11 = inv.to_string();

        mock_lnurl_pay_callback_endpoint_aes_success_action(
            &mock_rest_client,
            AesPayCallbackParams {
                error: None,
                pr: Some(bolt11.clone()),
                sa_data: sa_data.clone(),
                iv_bytes: random::<[u8; 16]>(),
                key_bytes: preimage.into_inner(),
            },
        );

        let mock_node_api = MockNodeAPI::new(get_dummy_node_state());
        let model_payment = mock_node_api
            .add_dummy_payment_for(bolt11, Some(preimage), Some(PayStatus::Pending))
            .await?;

        let known_payments: Vec<crate::models::Payment> = vec![model_payment];
        let rest_client: Arc<dyn RestClient> = Arc::new(mock_rest_client);
        let mock_breez_services = breez_services_with(
            Some(Arc::new(mock_node_api)),
            Some(rest_client),
            known_payments,
        )
        .await?;
        match mock_breez_services
            .lnurl_pay(LnUrlPayRequest {
                data: pay_req,
                amount_msat: user_amount_msat,
                use_trampoline: false,
                comment: Some(comment),
                payment_label: None,
                validate_success_action_url: None,
            })
            .await?
        {
            LnUrlPayResult::EndpointSuccess {
                data:
                    LnUrlPaySuccessData {
                        success_action: Some(received_sa),
                        ..
                    },
            } => match received_sa == sa {
                true => Ok(()),
                false => Err(anyhow!(
                    "Decrypted payload and description doesn't match expected success action"
                )),
            },
            LnUrlPayResult::EndpointSuccess {
                data:
                    LnUrlPaySuccessData {
                        success_action: None,
                        ..
                    },
            } => Err(anyhow!(
                "Expected success action in callback, but none provided"
            )),
            _ => Err(anyhow!("Unexpected success action type")),
        }
    }

    #[tokio::test]
    async fn test_lnurl_pay_aes_success_action_fail_to_decrypt() -> Result<()> {
        let mock_rest_client = MockRestClient::new();
        // Expected error in the AES payload
        let sa = SuccessActionProcessed::Aes {
            result: AesSuccessActionDataResult::ErrorStatus {
                reason: "Unpad Error".into(),
            },
        };

        // Generate preimage
        let preimage = sha256::Hash::hash(&rand_vec_u8(10));

        let comment = rand_string(COMMENT_LENGTH as usize);
        let pay_req = get_test_pay_req_data(0, 100_000, COMMENT_LENGTH);
        let temp_desc = pay_req.metadata_str.clone();

        // The invoice (served by LNURL-pay endpoint, matching preimage and description hash)
        let inv = rand_invoice_with_description_hash_and_preimage(temp_desc, preimage)?;

        let user_amount_msat = inv.amount_milli_satoshis().unwrap();
        let bolt11 = inv.to_string();
        let description = "test description in AES payload".to_string();
        let plaintext = "Hello, test plaintext".to_string();
        let sa_data = AesSuccessActionDataDecrypted {
            description,
            plaintext,
        };
        let wrong_key = vec![0u8; 32];

        mock_lnurl_pay_callback_endpoint_aes_success_action(
            &mock_rest_client,
            AesPayCallbackParams {
                error: None,
                pr: Some(bolt11.clone()),
                sa_data: sa_data.clone(),
                iv_bytes: random::<[u8; 16]>(),
                key_bytes: wrong_key.try_into().unwrap(),
            },
        );

        let mock_node_api = MockNodeAPI::new(get_dummy_node_state());
        let model_payment = mock_node_api
            .add_dummy_payment_for(bolt11, Some(preimage), Some(PayStatus::Pending))
            .await?;

        let known_payments: Vec<crate::models::Payment> = vec![model_payment];
        let rest_client: Arc<dyn RestClient> = Arc::new(mock_rest_client);
        let mock_breez_services = breez_services_with(
            Some(Arc::new(mock_node_api)),
            Some(rest_client),
            known_payments,
        )
        .await?;
        match mock_breez_services
            .lnurl_pay(LnUrlPayRequest {
                data: pay_req,
                amount_msat: user_amount_msat,
                use_trampoline: false,
                comment: Some(comment),
                payment_label: None,
                validate_success_action_url: None,
            })
            .await?
        {
            LnUrlPayResult::EndpointSuccess {
                data:
                LnUrlPaySuccessData {
                    success_action: Some(received_sa),
                    ..
                },
            } => match received_sa == sa {
                true => Ok(()),
                false => Err(anyhow!(
                    "Decrypted payload and description doesn't match expected success action: {received_sa:?}"
                )),
            },
            LnUrlPayResult::EndpointSuccess {
                data:
                LnUrlPaySuccessData {
                    success_action: None,
                    ..
                },
            } => Err(anyhow!(
                "Expected success action in callback, but none provided"
            )),
            _ => Err(anyhow!("Unexpected success action type")),
        }
    }

    #[test]
    fn test_lnurl_pay_build_pay_callback_url() -> Result<()> {
        let pay_req = get_test_pay_req_data(0, 100_000, 0);
        let user_amount_msat = 50_000;

        let amount_arg = format!("amount={user_amount_msat}");
        let user_comment = "test comment".to_string();
        let comment_arg = format!("comment={user_comment}");

        let url_amount_no_comment = build_pay_callback_url(user_amount_msat, &None, &pay_req)?;
        assert!(url_amount_no_comment.contains(&amount_arg));
        assert!(!url_amount_no_comment.contains(&comment_arg));

        let url_amount_with_comment =
            build_pay_callback_url(user_amount_msat, &Some(user_comment), &pay_req)?;
        assert!(url_amount_with_comment.contains(&amount_arg));
        assert!(url_amount_with_comment.contains("comment=test+comment"));

        Ok(())
    }
}
