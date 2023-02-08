use crate::input_parser::*;
use crate::invoice::parse_invoice;
use crate::lnurl::maybe_replace_host_with_mockito_test_host;
use crate::lnurl::pay::model::{CallbackResponse, SuccessAction, ValidatedCallbackResponse};
use crate::LnUrlErrorData;
use anyhow::{anyhow, Result};
use bitcoin_hashes::{sha256, Hash};
use std::str::FromStr;

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

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
            match sa {
                SuccessAction::Aes(data) => data.validate()?,
                SuccessAction::Message(data) => data.validate()?,
                SuccessAction::Url(data) => data.validate(&req_data)?,
            }
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
    use crate::lnurl::pay::{Aes256CbcDec, Aes256CbcEnc};

    use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
    use anyhow::{anyhow, Result};
    use serde::Deserialize;

    pub(crate) enum ValidatedCallbackResponse {
        EndpointSuccess { data: CallbackResponse },
        EndpointError { data: LnUrlErrorData },
    }

    /// Contains the result of the entire LNURL-pay interaction, as reported by the LNURL endpoint.
    ///
    /// * `EndpointSuccess` indicates the payment is complete. The endpoint may return a `SuccessActionProcessed`,
    /// in which case, the wallet has to present it to the user as described in
    /// <https://github.com/lnurl/luds/blob/luds/09.md>
    ///
    /// * `EndpointError` indicates a generic issue the LNURL endpoint encountered, including a freetext
    /// field with the reason.
    #[derive(Debug)]
    pub enum LnUrlPayResult {
        EndpointSuccess {
            data: Option<SuccessActionProcessed>,
        },
        EndpointError {
            data: LnUrlErrorData,
        },
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CallbackResponse {
        pub pr: String,
        pub success_action: Option<SuccessAction>,
    }

    /// Payload of the AES success action, as received from the LNURL endpoint
    ///
    /// See [AesSuccessActionDataDecrypted] for a similar wrapper containing the decrypted payload
    #[derive(Deserialize, Debug)]
    pub struct AesSuccessActionData {
        /// Contents description, up to 144 characters
        pub description: String,

        /// Base64, AES-encrypted data where encryption key is payment preimage, up to 4kb of characters
        pub ciphertext: String,

        /// Base64, initialization vector, exactly 24 characters
        pub iv: String,
    }

    /// Wrapper for the decrypted [AesSuccessActionData] payload
    #[derive(Deserialize, Debug)]
    pub struct AesSuccessActionDataDecrypted {
        /// Contents description, up to 144 characters
        pub description: String,

        /// Decrypted content
        pub plaintext: String,
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

    /// [SuccessAction] where contents are ready to be consumed by the caller
    ///
    /// Contents are identical to [SuccessAction], except for AES where the ciphertext is decrypted.
    #[derive(Deserialize, Debug)]
    pub enum SuccessActionProcessed {
        /// See [SuccessAction::Aes] for received payload
        ///
        /// See [AesSuccessActionDataDecrypted] for decrypted payload
        Aes(AesSuccessActionDataDecrypted),

        /// See [SuccessAction::Message]
        Message(MessageSuccessActionData),

        /// See [SuccessAction::Url]
        Url(UrlSuccessActionData),
    }

    /// Supported success action types
    ///
    /// Receiving any other (unsupported) success action type will result in a failed parsing,
    /// which will abort the LNURL-pay workflow, as per LUD-09.
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "tag")]
    pub enum SuccessAction {
        /// AES type, described in LUD-10
        Aes(AesSuccessActionData),

        /// Message type, described in LUD-09
        Message(MessageSuccessActionData),

        /// URL type, described in LUD-09
        Url(UrlSuccessActionData),
    }

    impl AesSuccessActionData {
        /// Validates the fields, but does not decrypt and validate the ciphertext.
        pub fn validate(&self) -> Result<()> {
            if self.description.len() > 144 {
                return Err(anyhow!(
                    "AES action description length is larger than the maximum allowed"
                ));
            }

            if self.ciphertext.len() > 4096 {
                return Err(anyhow!(
                    "AES action ciphertext length is larger than the maximum allowed"
                ));
            }
            base64::decode(&self.ciphertext)?;

            if self.iv.len() != 24 {
                return Err(anyhow!("AES action iv has unexpected length"));
            }
            base64::decode(&self.iv)?;

            Ok(())
        }

        /// Decrypts the ciphertext as a UTF-8 string, given the key (invoice preimage) parameter.
        pub fn decrypt(&self, key: &[u8; 32]) -> Result<String> {
            let plaintext_bytes =
                Aes256CbcDec::new_from_slices(key, &base64::decode(&self.iv)?)?
                    .decrypt_padded_vec_mut::<Pkcs7>(&base64::decode(&self.ciphertext)?)?;

            String::from_utf8(plaintext_bytes).map_err(|e| e.into())
        }

        /// Helper method that encrypts a given plaintext, with a given key and IV.
        pub fn encrypt(key: &[u8; 32], iv: &[u8; 16], plaintext: String) -> Result<String> {
            let ciphertext_bytes = Aes256CbcEnc::new_from_slices(key, iv)?
                .encrypt_padded_vec_mut::<Pkcs7>(plaintext.as_bytes());

            Ok(base64::encode(ciphertext_bytes))
        }
    }

    impl TryFrom<(AesSuccessActionData, &[u8; 32])> for AesSuccessActionDataDecrypted {
        type Error = anyhow::Error;

        fn try_from(
            value: (AesSuccessActionData, &[u8; 32]),
        ) -> std::result::Result<Self, Self::Error> {
            let data = value.0;
            let key = value.1;

            Ok(AesSuccessActionDataDecrypted {
                description: data.description.clone(),
                plaintext: data.decrypt(key)?,
            })
        }
    }

    impl MessageSuccessActionData {
        pub fn validate(&self) -> Result<()> {
            match self.message.len() <= 144 {
                true => Ok(()),
                false => Err(anyhow!(
                    "Success action message is longer than the maximum allowed length"
                )),
            }
        }
    }

    impl UrlSuccessActionData {
        pub fn validate(&self, req_data: &LnUrlPayRequestData) -> Result<()> {
            match self.description.len() <= 144 {
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

                let action_res_url = reqwest::Url::parse(&self.url)?;
                let action_res_domain = action_res_url
                    .domain()
                    .ok_or_else(|| anyhow!("Could not determine Success Action URL domain"))?;

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

#[cfg(test)]
mod tests {
    use crate::lnurl::pay::model::*;
    use crate::lnurl::pay::*;

    use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
    use anyhow::{anyhow, Result};
    use mockito::Mock;
    use rand::Rng;

    use crate::test_utils::*;

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

    /// Mock an LNURL-pay endpoint that responds with a Success Action of type AES
    fn mock_lnurl_pay_callback_endpoint_aes_success_action(
        pay_req: &LnUrlPayRequestData,
        user_amount_sat: u64,
        error: Option<String>,
        pr: String,
        plaintext: String,
        iv_bytes: [u8; 16],
        key_bytes: [u8; 32],
    ) -> Result<Mock> {
        let callback_url = build_pay_callback_url(user_amount_sat, &None, pay_req)?;
        let url = reqwest::Url::parse(&callback_url)?;
        let mockito_path: &str = &format!("{}?{}", url.path(), url.query().unwrap());

        let iv_base64 = base64::encode(iv_bytes);
        let cipertext = AesSuccessActionData::encrypt(&key_bytes, &iv_bytes, plaintext)?;

        let expected_payload = r#"
{
    "pr":"token-invoice",
    "routes":[],
    "successAction": {
        "tag":"aes",
        "description":"test description",
        "iv":"token-iv",
        "ciphertext":"token-ciphertext"
    }
}
        "#
        .replace('\n', "")
        .replace("token-iv", &iv_base64)
        .replace("token-ciphertext", &cipertext)
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
        let inv = rand_invoice_with_description_hash(temp_desc.clone())?;
        let payreq: String = rand_invoice_with_description_hash(temp_desc)?.to_string();

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
    fn test_lnurl_pay_validate_success_action_encrypt_decrypt() -> Result<()> {
        // Simulate a preimage, which will be the AES key
        let key = sha256::Hash::hash(&[0x42; 16]);
        let key_bytes = key.as_inner();

        let iv_bytes = [0x24; 16]; // 16 bytes = 24 chars
        let iv_base64 = base64::encode(iv_bytes); // JCQkJCQkJCQkJCQkJCQkJA==

        let plaintext = "hello world! this is my plaintext.";
        let plaintext_bytes = plaintext.as_bytes();

        // hex = 91239ab5d94369a18474ee58372c7d0fcee5e227903f671bfe19ef32f1cada804d10f0f006265289d936317343dbc0ca
        // base64 = kSOatdlDaaGEdO5YNyx9D87l4ieQP2cb/hnvMvHK2oBNEPDwBiZSidk2MXND28DK
        let ciphertext_bytes =
            &hex::decode("91239ab5d94369a18474ee58372c7d0fcee5e227903f671bfe19ef32f1cada804d10f0f006265289d936317343dbc0ca")?;
        let ciphertext_base64 = base64::encode(ciphertext_bytes);

        // Encrypt raw (which returns raw bytes)
        let res = Aes256CbcEnc::new_from_slices(key_bytes, &iv_bytes)?
            .encrypt_padded_vec_mut::<Pkcs7>(plaintext_bytes);
        assert_eq!(res[..], ciphertext_bytes[..]);

        // Decrypt raw (which returns raw bytes)
        let res = Aes256CbcDec::new_from_slices(key_bytes, &iv_bytes)?
            .decrypt_padded_vec_mut::<Pkcs7>(&res)?;
        assert_eq!(res[..], plaintext_bytes[..]);

        // Encrypt via AesSuccessActionData helper method (which returns a base64 representation of the bytes)
        let res = AesSuccessActionData::encrypt(key_bytes, &iv_bytes, plaintext.into())?;
        assert_eq!(res, base64::encode(ciphertext_bytes));

        // Decrypt via AesSuccessActionData instance method (which returns an UTF-8 string of the plaintext bytes)
        let res = AesSuccessActionData {
            description: "Test AES successData description".into(),
            ciphertext: ciphertext_base64,
            iv: iv_base64,
        }
        .decrypt(key_bytes)?;
        assert_eq!(res.as_bytes(), plaintext_bytes);

        Ok(())
    }

    #[test]
    fn test_lnurl_pay_validate_success_action_aes() -> Result<()> {
        assert!(AesSuccessActionData {
            description: "Test AES successData description".into(),
            ciphertext: "kSOatdlDaaGEdO5YNyx9D87l4ieQP2cb/hnvMvHK2oBNEPDwBiZSidk2MXND28DK".into(),
            iv: base64::encode([0xa; 16])
        }
        .validate()
        .is_ok());

        // Description longer than 144 chars
        assert!(AesSuccessActionData {
            description: rand_string(150),
            ciphertext: "kSOatdlDaaGEdO5YNyx9D87l4ieQP2cb/hnvMvHK2oBNEPDwBiZSidk2MXND28DK".into(),
            iv: base64::encode([0xa; 16])
        }
        .validate()
        .is_err());

        // IV size below 16 bytes (24 chars)
        assert!(AesSuccessActionData {
            description: "Test AES successData description".into(),
            ciphertext: "kSOatdlDaaGEdO5YNyx9D87l4ieQP2cb/hnvMvHK2oBNEPDwBiZSidk2MXND28DK".into(),
            iv: base64::encode([0xa; 10])
        }
        .validate()
        .is_err());

        // IV size above 16 bytes (24 chars)
        assert!(AesSuccessActionData {
            description: "Test AES successData description".into(),
            ciphertext: "kSOatdlDaaGEdO5YNyx9D87l4ieQP2cb/hnvMvHK2oBNEPDwBiZSidk2MXND28DK".into(),
            iv: base64::encode([0xa; 20])
        }
        .validate()
        .is_err());

        // IV is not base64 encoded (but fits length of 24 chars)
        assert!(AesSuccessActionData {
            description: "Test AES successData description".into(),
            ciphertext: "kSOatdlDaaGEdO5YNyx9D87l4ieQP2cb/hnvMvHK2oBNEPDwBiZSidk2MXND28DK".into(),
            iv: ",".repeat(24)
        }
        .validate()
        .is_err());

        // Ciphertext is not base64 encoded
        assert!(AesSuccessActionData {
            description: "Test AES successData description".into(),
            ciphertext: ",".repeat(96),
            iv: base64::encode([0xa; 16])
        }
        .validate()
        .is_err());

        // Ciphertext longer than 4KB
        assert!(AesSuccessActionData {
            description: "Test AES successData description".into(),
            ciphertext: base64::encode(rand_string(5000)),
            iv: base64::encode([0xa; 16])
        }
        .validate()
        .is_err());

        Ok(())
    }

    #[test]
    fn test_lnurl_pay_validate_success_action_msg() -> Result<()> {
        assert!(MessageSuccessActionData {
            message: "short msg".into()
        }
        .validate()
        .is_ok());

        // Too long message
        assert!(MessageSuccessActionData {
            message: rand_string(150)
        }
        .validate()
        .is_err());

        Ok(())
    }

    #[test]
    fn test_lnurl_pay_validate_success_url() -> Result<()> {
        let pay_req_data = get_test_pay_req_data(0, 100, 100);

        assert!(UrlSuccessActionData {
            description: "short msg".into(),
            url: pay_req_data.callback.clone()
        }
        .validate(&pay_req_data)
        .is_ok());

        // Too long description
        assert!(UrlSuccessActionData {
            description: rand_string(150),
            url: pay_req_data.callback.clone()
        }
        .validate(&pay_req_data)
        .is_err());

        // Different Success Action domain than in the callback URL
        assert!(UrlSuccessActionData {
            description: "short msg".into(),
            url: "https://new-domain.com/test-url".into()
        }
        .validate(&pay_req_data)
        .is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_no_success_action() -> Result<()> {
        let pay_req = get_test_pay_req_data(0, 100, 0);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_sat = inv.amount_milli_satoshis().unwrap() / 1000;
        let _m = mock_lnurl_pay_callback_endpoint_no_success_action(
            &pay_req,
            user_amount_sat,
            None,
            inv.to_string(),
        )?;

        let mock_breez_services = crate::breez_services::tests::breez_services().await?;
        match mock_breez_services
            .lnurl_pay(user_amount_sat, None, pay_req)
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

        let mock_breez_services = crate::breez_services::tests::breez_services().await?;
        let r = mock_breez_services
            .lnurl_pay(user_amount_sat, None, pay_req)
            .await;
        // An unsupported Success Action results in an error
        assert!(r.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_msg_success_action() -> Result<()> {
        let pay_req = get_test_pay_req_data(0, 100, 0);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_sat = inv.amount_milli_satoshis().unwrap() / 1000;
        let _m = mock_lnurl_pay_callback_endpoint_msg_success_action(
            &pay_req,
            user_amount_sat,
            None,
            inv.to_string(),
        )?;

        let mock_breez_services = crate::breez_services::tests::breez_services().await?;
        match mock_breez_services
            .lnurl_pay(user_amount_sat, None, pay_req)
            .await?
        {
            LnUrlPayResult::EndpointSuccess { data: None } => Err(anyhow!(
                "Expected success action in callback, but none provided"
            )),
            LnUrlPayResult::EndpointSuccess {
                data: Some(SuccessActionProcessed::Message(msg)),
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
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_sat = (inv.amount_milli_satoshis().unwrap() / 1000) + 1;
        let _m = mock_lnurl_pay_callback_endpoint_msg_success_action(
            &pay_req,
            user_amount_sat,
            None,
            inv.to_string(),
        )?;

        let mock_breez_services = crate::breez_services::tests::breez_services().await?;
        assert!(mock_breez_services
            .lnurl_pay(user_amount_sat, None, pay_req)
            .await
            .is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_msg_success_action_error_from_endpoint() -> Result<()> {
        let pay_req = get_test_pay_req_data(0, 100, 0);
        let temp_desc = pay_req.metadata_str.clone();
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_sat = inv.amount_milli_satoshis().unwrap() / 1000;
        let expected_error_msg = "Error message from LNURL endpoint";
        let _m = mock_lnurl_pay_callback_endpoint_msg_success_action(
            &pay_req,
            user_amount_sat,
            Some(expected_error_msg.to_string()),
            inv.to_string(),
        )?;

        let mock_breez_services = crate::breez_services::tests::breez_services().await?;
        let res = mock_breez_services
            .lnurl_pay(user_amount_sat, None, pay_req)
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
        let inv = rand_invoice_with_description_hash(temp_desc)?;
        let user_amount_sat = inv.amount_milli_satoshis().unwrap() / 1000;
        let _m = mock_lnurl_pay_callback_endpoint_url_success_action(
            &pay_req,
            user_amount_sat,
            None,
            inv.to_string(),
        )?;

        let mock_breez_services = crate::breez_services::tests::breez_services().await?;
        match mock_breez_services
            .lnurl_pay(user_amount_sat, None, pay_req)
            .await?
        {
            LnUrlPayResult::EndpointSuccess {
                data: Some(SuccessActionProcessed::Url(url)),
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

    #[tokio::test]
    async fn test_lnurl_pay_aes_success_action() -> Result<()> {
        let plaintext = "Hello, test plaintext";

        // Generate preimage
        let preimage = sha256::Hash::hash(&rand_vec_u8(10));

        let pay_req = get_test_pay_req_data(0, 100, 0);
        let temp_desc = pay_req.metadata_str.clone();

        // The invoice (served by LNURL-pay endpoint, matching preimage and description hash)
        let inv = rand_invoice_with_description_hash_and_preimage(temp_desc, preimage)?;

        let user_amount_sat = inv.amount_milli_satoshis().unwrap() / 1000;
        let bolt11 = inv.to_string();
        let _m = mock_lnurl_pay_callback_endpoint_aes_success_action(
            &pay_req,
            user_amount_sat,
            None,
            bolt11.clone(),
            plaintext.into(),
            rand::thread_rng().gen::<[u8; 16]>(),
            preimage.into_inner(),
        )?;

        let model_payment = MockNodeAPI::add_dummy_payment_for(bolt11, Some(preimage)).await?;

        let known_payments: Vec<crate::models::Payment> = vec![model_payment];
        let mock_breez_services =
            crate::breez_services::tests::breez_services_with(known_payments).await?;
        match mock_breez_services
            .lnurl_pay(user_amount_sat, None, pay_req)
            .await?
        {
            LnUrlPayResult::EndpointSuccess {
                data: Some(SuccessActionProcessed::Aes(aes_data)),
            } => match aes_data.plaintext == plaintext {
                true => Ok(()),
                false => Err(anyhow!(
                    "Decrypted payload doesn't match original plaintext"
                )),
            },
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
