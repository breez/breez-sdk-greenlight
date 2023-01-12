use crate::input_parser::*;
use crate::invoice::parse_invoice;
use crate::lnurl::maybe_replace_host_with_mockito_test_host;
use crate::lnurl::pay::model::{CallbackResponse, ValidatedCallbackResponse};
use crate::LnUrlErrorData;
use anyhow::{anyhow, Result};
use bitcoin_hashes::{sha256, Hash};
use std::str::FromStr;

/// Validates invoice and performs the second and last step of LNURL-pay, as per
/// <https://github.com/lnurl/luds/blob/luds/06.md>
///
/// See the [parse] docs for more detail on the full workflow.
pub(crate) async fn validate_lnurl_pay(
    user_amount_sat: u64,
    comment: Option<String>,
    req_data: LnUrlPayRequestData,
) -> Result<ValidatedCallbackResponse> {
    validate_user_input(
        user_amount_sat * 1000,
        &comment,
        req_data.min_sendable,
        req_data.max_sendable,
        req_data.comment_allowed,
    )?;

    let callback_url = build_pay_callback_url(user_amount_sat, &comment, &req_data)?;
    let callback_resp_text = reqwest::get(&callback_url).await?.text().await?;

    if let Ok(err) = serde_json::from_str::<LnUrlErrorData>(&callback_resp_text) {
        Ok(ValidatedCallbackResponse::EndpointError { data: err })
    } else {
        let callback_resp: CallbackResponse = reqwest::get(&callback_url).await?.json().await?;
        if let Some(ref sa) = callback_resp.success_action {
            sa.validate(&req_data)?;
        }

        validate_invoice(user_amount_sat, &callback_resp.pr, &req_data)?;
        Ok(ValidatedCallbackResponse::EndpointSuccess {
            data: callback_resp,
        })
    }
}

fn build_pay_callback_url(
    user_amount_sat: u64,
    user_comment: &Option<String>,
    req_data: &LnUrlPayRequestData,
) -> Result<String> {
    let amount_msat = (user_amount_sat * 1000).to_string();
    let mut url = reqwest::Url::from_str(&req_data.callback)?;

    url.query_pairs_mut().append_pair("amount", &amount_msat);
    if let Some(comment) = user_comment {
        url.query_pairs_mut().append_pair("comment", comment);
    }

    let mut callback_url = url.to_string();
    callback_url = maybe_replace_host_with_mockito_test_host(callback_url)?;
    Ok(callback_url)
}

fn validate_user_input(
    user_amount_msat: u64,
    comment: &Option<String>,
    condition_min_amount_msat: u64,
    condition_max_amount_msat: u64,
    condition_max_comment_len: u16,
) -> Result<()> {
    if user_amount_msat < condition_min_amount_msat {
        return Err(anyhow!("Amount is smaller than the minimum allowed"));
    }

    if user_amount_msat > condition_max_amount_msat {
        return Err(anyhow!("Amount is bigger than the maximum allowed"));
    }

    match comment {
        None => Ok(()),
        Some(msg) => match msg.len() <= condition_max_comment_len as usize {
            true => Ok(()),
            false => Err(anyhow!(
                "Comment is longer than the maximum allowed comment length"
            )),
        },
    }
}

fn validate_invoice(
    user_amount_sat: u64,
    bolt11: &str,
    req_data: &LnUrlPayRequestData,
) -> Result<()> {
    let invoice = parse_invoice(bolt11)?;

    match invoice.description_hash {
        None => return Err(anyhow!("Invoice is missing description hash")),
        Some(received_hash) => {
            // The hash is calculated from the exact metadata string, as received from the LNURL endpoint
            let calculated_hash = sha256::Hash::hash(req_data.metadata_str.as_bytes());
            if received_hash != calculated_hash.to_string() {
                return Err(anyhow!("Invoice has an invalid description hash"));
            }
        }
    }

    match invoice.amount_msat {
        None => Err(anyhow!("Amount is bigger than the maximum allowed")),
        Some(invoice_amount_msat) => match invoice_amount_msat == (user_amount_sat * 1000) {
            true => Ok(()),
            false => Err(anyhow!(
                "Invoice amount is different than the user chosen amount"
            )),
        },
    }
}

pub(crate) mod model {
    use crate::input_parser::*;

    use anyhow::{anyhow, Result};
    use serde::Deserialize;

    pub(crate) enum ValidatedCallbackResponse {
        EndpointSuccess { data: CallbackResponse },
        EndpointError { data: LnUrlErrorData },
    }

    /// Contains the result of the entire LNURL-pay interaction, as reported by the LNURL endpoint.
    ///
    /// * `EndpointSuccess` indicates the payment is complete. The endpoint may return a `SuccessAction`,
    /// in which case, the wallet has to present it to the user as described in
    /// <https://github.com/lnurl/luds/blob/luds/09.md>
    ///
    /// * `EndpointError` indicates a generic issue the LNURL endpoint encountered, including a freetext
    /// field with the reason.
    #[derive(Debug)]
    pub enum LnUrlPayResult {
        EndpointSuccess { data: Option<SuccessAction> },
        EndpointError { data: LnUrlErrorData },
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CallbackResponse {
        pub pr: String,
        pub success_action: Option<SuccessAction>,
    }

    #[derive(Deserialize, Debug)]
    pub struct MessageSuccessActionData {
        pub message: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct UrlSuccessActionData {
        pub description: String,
        pub url: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "tag")]
    pub enum SuccessAction {
        // Any other successAction type is considered not supported, so the parsing would fail
        // and abort payment, as per LUD-09
        Message(MessageSuccessActionData),
        Url(UrlSuccessActionData),
    }

    impl SuccessAction {
        pub fn validate(&self, req_data: &LnUrlPayRequestData) -> Result<()> {
            match self {
                SuccessAction::Message(msg_action_data) => {
                    match msg_action_data.message.len() <= 144 {
                        true => Ok(()),
                        false => Err(anyhow!(
                            "Success action message is longer than the maximum allowed length"
                        )),
                    }
                }

                SuccessAction::Url(url_action_data) => {
                    match url_action_data.description.len() <= 144 {
                        true => Ok(()),
                        false => Err(anyhow!(
                            "Success action description is longer than the maximum allowed length"
                        )),
                    }
                    .and_then(|_| {
                        let req_url = reqwest::Url::parse(&req_data.callback)?;
                        let req_domain = req_url
                            .domain()
                            .ok_or_else(|| anyhow!("Could not determine callback domain"))?;

                        let action_res_url = reqwest::Url::parse(&url_action_data.url)?;
                        let action_res_domain = action_res_url.domain().ok_or_else(|| {
                            anyhow!("Could not determine Success Action URL domain")
                        })?;

                        match req_domain == action_res_domain {
                            true => Ok(()),
                            false => Err(anyhow!(
                                "Success Action URL has different domain than the callback domain"
                            )),
                        }
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lnurl::pay::model::{
        LnUrlPayResult, MessageSuccessActionData, SuccessAction, UrlSuccessActionData,
    };
    use crate::lnurl::pay::*;
    use anyhow::{anyhow, Result};
    use mockito::Mock;

    use crate::test_utils::{rand_invoice_with_description_hash, rand_string};

    /// Mock an LNURL-pay endpoint that responds with no Success Action
    fn mock_lnurl_pay_callback_endpoint_no_success_action(
        pay_req: &LnUrlPayRequestData,
        user_amount_sat: u64,
        error: Option<String>,
        pr: String,
    ) -> Result<Mock> {
        let callback_url = build_pay_callback_url(user_amount_sat, &None, pay_req)?;
        let url = reqwest::Url::parse(&callback_url)?;
        let mockito_path: &str = &format!("{}?{}", url.path(), url.query().unwrap());

        let expected_payload = r#"
{
    "pr":"token-invoice",
    "routes":[]
}
        "#
        .replace('\n', "")
        .replace("token-invoice", &pr);

        let response_body = match error {
            None => expected_payload,
            Some(err_reason) => {
                ["{\"status\": \"ERROR\", \"reason\": \"", &err_reason, "\"}"].join("")
            }
        };
        Ok(mockito::mock("GET", mockito_path)
            .with_body(response_body)
            .create())
    }

    /// Mock an LNURL-pay endpoint that responds with an unsupported Success Action
    fn mock_lnurl_pay_callback_endpoint_unsupported_success_action(
        pay_req: &LnUrlPayRequestData,
        user_amount_sat: u64,
        error: Option<String>,
    ) -> Result<Mock> {
        let callback_url = build_pay_callback_url(user_amount_sat, &None, pay_req)?;
        let url = reqwest::Url::parse(&callback_url)?;
        let mockito_path: &str = &format!("{}?{}", url.path(), url.query().unwrap());

        let expected_payload = r#"
{
    "pr":"lnbc110n1p38q3gtpp5ypz09jrd8p993snjwnm68cph4ftwp22le34xd4r8ftspwshxhmnsdqqxqyjw5qcqpxsp5htlg8ydpywvsa7h3u4hdn77ehs4z4e844em0apjyvmqfkzqhhd2q9qgsqqqyssqszpxzxt9uuqzymr7zxcdccj5g69s8q7zzjs7sgxn9ejhnvdh6gqjcy22mss2yexunagm5r2gqczh8k24cwrqml3njskm548aruhpwssq9nvrvz",
    "routes":[],
    "successAction": {
        "tag":"random-type-that-is-not-supported",
        "message":"test msg"
    }
}
        "#.replace('\n', "");

        let response_body = match error {
            None => expected_payload,
            Some(err_reason) => {
                ["{\"status\": \"ERROR\", \"reason\": \"", &err_reason, "\"}"].join("")
            }
        };
        Ok(mockito::mock("GET", mockito_path)
            .with_body(response_body)
            .create())
    }

    /// Mock an LNURL-pay endpoint that responds with a Success Action of type message
    fn mock_lnurl_pay_callback_endpoint_msg_success_action(
        pay_req: &LnUrlPayRequestData,
        user_amount_sat: u64,
        error: Option<String>,
        pr: String,
    ) -> Result<Mock> {
        let callback_url = build_pay_callback_url(user_amount_sat, &None, pay_req)?;
        let url = reqwest::Url::parse(&callback_url)?;
        let mockito_path: &str = &format!("{}?{}", url.path(), url.query().unwrap());

        let expected_payload = r#"
{
    "pr":"token-invoice",
    "routes":[],
    "successAction": {
        "tag":"message",
        "message":"test msg"
    }
}
        "#
        .replace('\n', "")
        .replace("token-invoice", &pr);

        let response_body = match error {
            None => expected_payload,
            Some(err_reason) => {
                ["{\"status\": \"ERROR\", \"reason\": \"", &err_reason, "\"}"].join("")
            }
        };
        Ok(mockito::mock("GET", mockito_path)
            .with_body(response_body)
            .create())
    }

    /// Mock an LNURL-pay endpoint that responds with a Success Action of type URL
    fn mock_lnurl_pay_callback_endpoint_url_success_action(
        pay_req: &LnUrlPayRequestData,
        user_amount_sat: u64,
        error: Option<String>,
        pr: String,
    ) -> Result<Mock> {
        let callback_url = build_pay_callback_url(user_amount_sat, &None, pay_req)?;
        let url = reqwest::Url::parse(&callback_url)?;
        let mockito_path: &str = &format!("{}?{}", url.path(), url.query().unwrap());

        let expected_payload = r#"
{
    "pr":"token-invoice",
    "routes":[],
    "successAction": {
        "tag":"url",
        "description":"test description",
        "url":"https://localhost/test-url"
    }
}
        "#
        .replace('\n', "")
        .replace("token-invoice", &pr);

        let response_body = match error {
            None => expected_payload,
            Some(err_reason) => {
                ["{\"status\": \"ERROR\", \"reason\": \"", &err_reason, "\"}"].join("")
            }
        };
        Ok(mockito::mock("GET", mockito_path)
            .with_body(response_body)
            .create())
    }

    fn get_test_pay_req_data(min_sat: u64, max_sat: u64, comment_len: u16) -> LnUrlPayRequestData {
        LnUrlPayRequestData {
            min_sendable: min_sat * 1000,
            max_sendable: max_sat * 1000,
            comment_allowed: comment_len,
            metadata_str: "".into(),
            callback: "https://localhost/callback".into(),
        }
    }

    #[test]
    fn test_lnurl_pay_validate_input() -> Result<()> {
        assert!(validate_user_input(100, &None, 0, 100, 0).is_ok());
        assert!(validate_user_input(100, &Some("test".into()), 0, 100, 5).is_ok());

        assert!(validate_user_input(5, &None, 10, 100, 5).is_err());
        assert!(validate_user_input(200, &None, 10, 100, 5).is_err());
        assert!(validate_user_input(100, &Some("test".into()), 10, 100, 0).is_err());

        Ok(())
    }

    #[test]
    fn test_lnurl_pay_validate_invoice() -> Result<()> {
        let req = get_test_pay_req_data(0, 100, 0);
        let temp_desc = req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc.clone());
        let payreq: String = rand_invoice_with_description_hash(temp_desc).to_string();

        assert!(
            validate_invoice(inv.amount_milli_satoshis().unwrap() / 1000, &payreq, &req).is_ok()
        );
        assert!(validate_invoice(
            (inv.amount_milli_satoshis().unwrap() / 1000) + 1,
            &payreq,
            &req
        )
        .is_err());

        Ok(())
    }

    #[test]
    fn test_lnurl_pay_validate_success_action_msg() -> Result<()> {
        let pay_req_data = get_test_pay_req_data(0, 100, 100);

        assert!(SuccessAction::Message(MessageSuccessActionData {
            message: "short msg".into()
        })
        .validate(&pay_req_data)
        .is_ok());

        // Too long message
        assert!(SuccessAction::Message(MessageSuccessActionData {
            message: rand_string(150)
        })
        .validate(&pay_req_data)
        .is_err());

        Ok(())
    }

    #[test]
    fn test_lnurl_pay_validate_success_url() -> Result<()> {
        let pay_req_data = get_test_pay_req_data(0, 100, 100);

        assert!(SuccessAction::Url(UrlSuccessActionData {
            description: "short msg".into(),
            url: pay_req_data.callback.clone()
        })
        .validate(&pay_req_data)
        .is_ok());

        // Too long description
        assert!(SuccessAction::Url(UrlSuccessActionData {
            description: rand_string(150),
            url: pay_req_data.callback.clone()
        })
        .validate(&pay_req_data)
        .is_err());

        // Different Success Action domain than in the callback URL
        assert!(SuccessAction::Url(UrlSuccessActionData {
            description: "short msg".into(),
            url: "https://new-domain.com/test-url".into()
        })
        .validate(&pay_req_data)
        .is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_no_success_action() -> Result<()> {
        let pay_req = get_test_pay_req_data(0, 100, 0);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc);
        let user_amount_sat = inv.amount_milli_satoshis().unwrap() / 1000;
        let _m = mock_lnurl_pay_callback_endpoint_no_success_action(
            &pay_req,
            user_amount_sat,
            None,
            inv.to_string(),
        )?;

        let mock_breez_services = crate::breez_services::tests::breez_services().await;
        match mock_breez_services
            .pay_lnurl(user_amount_sat, None, pay_req)
            .await?
        {
            LnUrlPayResult::EndpointSuccess { data: None } => Ok(()),
            LnUrlPayResult::EndpointSuccess { data: Some(_) } => {
                Err(anyhow!("Unexpected success action"))
            }
            _ => Err(anyhow!("Unexpected success action type")),
        }
    }

    #[tokio::test]
    async fn test_lnurl_pay_unsupported_success_action() -> Result<()> {
        let user_amount_sat = 11;
        let pay_req = get_test_pay_req_data(0, 100, 0);
        let _m = mock_lnurl_pay_callback_endpoint_unsupported_success_action(
            &pay_req,
            user_amount_sat,
            None,
        )?;

        let mock_breez_services = crate::breez_services::tests::breez_services().await;
        let r = mock_breez_services
            .pay_lnurl(user_amount_sat, None, pay_req)
            .await;
        // An unsupported Success Action results in an error
        assert!(r.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_msg_success_action() -> Result<()> {
        let pay_req = get_test_pay_req_data(0, 100, 0);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc);
        let user_amount_sat = inv.amount_milli_satoshis().unwrap() / 1000;
        let _m = mock_lnurl_pay_callback_endpoint_msg_success_action(
            &pay_req,
            user_amount_sat,
            None,
            inv.to_string(),
        )?;

        let mock_breez_services = crate::breez_services::tests::breez_services().await;
        match mock_breez_services
            .pay_lnurl(user_amount_sat, None, pay_req)
            .await?
        {
            LnUrlPayResult::EndpointSuccess { data: None } => Err(anyhow!(
                "Expected success action in callback, but none provided"
            )),
            LnUrlPayResult::EndpointSuccess {
                data: Some(SuccessAction::Message(msg)),
            } => match msg.message {
                s if s == "test msg" => Ok(()),
                _ => Err(anyhow!("Unexpected success action message content")),
            },
            _ => Err(anyhow!("Unexpected success action type")),
        }
    }

    #[tokio::test]
    async fn test_lnurl_pay_msg_success_action_incorrect_amount() -> Result<()> {
        let pay_req = get_test_pay_req_data(0, 100, 0);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc);
        let user_amount_sat = (inv.amount_milli_satoshis().unwrap() / 1000) + 1;
        let _m = mock_lnurl_pay_callback_endpoint_msg_success_action(
            &pay_req,
            user_amount_sat,
            None,
            inv.to_string(),
        )?;

        let mock_breez_services = crate::breez_services::tests::breez_services().await;
        assert!(mock_breez_services
            .pay_lnurl(user_amount_sat, None, pay_req)
            .await
            .is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_msg_success_action_error_from_endpoint() -> Result<()> {
        let pay_req = get_test_pay_req_data(0, 100, 0);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc);
        let user_amount_sat = inv.amount_milli_satoshis().unwrap() / 1000;
        let expected_error_msg = "Error message from LNURL endpoint";
        let _m = mock_lnurl_pay_callback_endpoint_msg_success_action(
            &pay_req,
            user_amount_sat,
            Some(expected_error_msg.to_string()),
            inv.to_string(),
        )?;

        let mock_breez_services = crate::breez_services::tests::breez_services().await;
        let res = mock_breez_services
            .pay_lnurl(user_amount_sat, None, pay_req)
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
        let pay_req = get_test_pay_req_data(0, 100, 0);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc);
        let user_amount_sat = inv.amount_milli_satoshis().unwrap() / 1000;
        let _m = mock_lnurl_pay_callback_endpoint_url_success_action(
            &pay_req,
            user_amount_sat,
            None,
            inv.to_string(),
        )?;

        let mock_breez_services = crate::breez_services::tests::breez_services().await;
        match mock_breez_services
            .pay_lnurl(user_amount_sat, None, pay_req)
            .await?
        {
            LnUrlPayResult::EndpointSuccess {
                data: Some(SuccessAction::Url(url)),
            } => {
                if url.url == "https://localhost/test-url" && url.description == "test description"
                {
                    Ok(())
                } else {
                    Err(anyhow!("Unexpected success action content"))
                }
            }
            LnUrlPayResult::EndpointSuccess { data: None } => Err(anyhow!(
                "Expected success action in callback, but none provided"
            )),
            _ => Err(anyhow!("Unexpected success action type")),
        }
    }

    #[test]
    fn test_lnurl_pay_build_pay_callback_url() -> Result<()> {
        let pay_req = get_test_pay_req_data(0, 100, 0);
        let user_amount_sat = 50;

        let amount_arg = format!("amount={}", user_amount_sat * 1000);
        let user_comment = "test comment".to_string();
        let comment_arg = format!("comment={}", user_comment);

        let url_amount_no_comment = build_pay_callback_url(user_amount_sat, &None, &pay_req)?;
        assert!(url_amount_no_comment.contains(&amount_arg));
        assert!(!url_amount_no_comment.contains(&comment_arg));

        let url_amount_with_comment =
            build_pay_callback_url(user_amount_sat, &Some(user_comment), &pay_req)?;
        assert!(url_amount_with_comment.contains(&amount_arg));
        assert!(url_amount_with_comment.contains("comment=test+comment"));

        Ok(())
    }
}
