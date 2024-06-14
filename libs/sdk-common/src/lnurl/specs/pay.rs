use std::str::FromStr;

use crate::prelude::*;

pub type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
pub type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

/// Validates invoice and performs the second and last step of LNURL-pay, as per
/// <https://github.com/lnurl/luds/blob/luds/06.md>
///
/// See the [parse] docs for more detail on the full workflow.
pub async fn validate_lnurl_pay(
    user_amount_msat: u64,
    comment: &Option<String>,
    req_data: &LnUrlPayRequestData,
    network: Network,
) -> LnUrlResult<ValidatedCallbackResponse> {
    validate_user_input(
        user_amount_msat,
        comment,
        req_data.min_sendable,
        req_data.max_sendable,
        req_data.comment_allowed,
    )?;

    let callback_url = build_pay_callback_url(user_amount_msat, comment, req_data)?;
    let (callback_resp_text, _) = get_and_log_response(&callback_url)
        .await
        .map_err(|e| LnUrlError::ServiceConnectivity(e.to_string()))?;

    if let Ok(err) = serde_json::from_str::<LnUrlErrorData>(&callback_resp_text) {
        Ok(ValidatedCallbackResponse::EndpointError { data: err })
    } else {
        let callback_resp: CallbackResponse = serde_json::from_str(&callback_resp_text)?;
        if let Some(ref sa) = callback_resp.success_action {
            match sa {
                SuccessAction::Aes(data) => data.validate()?,
                SuccessAction::Message(data) => data.validate()?,
                SuccessAction::Url(data) => data.validate(req_data)?,
            }
        }

        validate_invoice(user_amount_msat, &callback_resp.pr, network)?;
        Ok(ValidatedCallbackResponse::EndpointSuccess {
            data: callback_resp,
        })
    }
}

pub fn build_pay_callback_url(
    user_amount_msat: u64,
    user_comment: &Option<String>,
    data: &LnUrlPayRequestData,
) -> LnUrlResult<String> {
    let amount_msat = user_amount_msat.to_string();
    let mut url = reqwest::Url::from_str(&data.callback)
        .map_err(|e| LnUrlError::InvalidUri(e.to_string()))?;

    url.query_pairs_mut().append_pair("amount", &amount_msat);
    if let Some(comment) = user_comment {
        url.query_pairs_mut().append_pair("comment", comment);
    }

    Ok(url.to_string())
}

pub fn validate_user_input(
    user_amount_msat: u64,
    comment: &Option<String>,
    condition_min_amount_msat: u64,
    condition_max_amount_msat: u64,
    condition_max_comment_len: u16,
) -> LnUrlResult<()> {
    ensure_sdk!(
        user_amount_msat >= condition_min_amount_msat,
        LnUrlError::generic("Amount is smaller than the minimum allowed")
    );

    ensure_sdk!(
        user_amount_msat <= condition_max_amount_msat,
        LnUrlError::generic("Amount is bigger than the maximum allowed")
    );

    match comment {
        None => Ok(()),
        Some(msg) => match msg.len() <= condition_max_comment_len as usize {
            true => Ok(()),
            false => Err(LnUrlError::generic(
                "Comment is longer than the maximum allowed comment length",
            )),
        },
    }
}

pub fn validate_invoice(user_amount_msat: u64, bolt11: &str, network: Network) -> LnUrlResult<()> {
    let invoice = parse_invoice(bolt11)?;
    // Valid the invoice network against the config network
    validate_network(invoice.clone(), network)?;

    match invoice.amount_msat {
        None => Err(LnUrlError::generic(
            "Amount is bigger than the maximum allowed",
        )),
        Some(invoice_amount_msat) => match invoice_amount_msat == user_amount_msat {
            true => Ok(()),
            false => Err(LnUrlError::generic(
                "Invoice amount is different than the user chosen amount",
            )),
        },
    }
}

pub mod model {
    use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
    use anyhow::Result;
    use rusqlite::types::{FromSql, FromSqlError, ToSqlOutput};
    use rusqlite::ToSql;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    use crate::prelude::specs::pay::{Aes256CbcDec, Aes256CbcEnc};
    use crate::prelude::*;
    // use crate::Payment;

    /// Represents a LNURL-pay request.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct LnUrlPayRequest {
        /// The [LnUrlPayRequestData] returned by [crate::input_parser::parse]
        pub data: LnUrlPayRequestData,
        /// The amount in millisatoshis for this payment
        pub amount_msat: u64,
        /// An optional comment for this payment
        pub comment: Option<String>,
        /// The external label or identifier of the [Payment]
        pub payment_label: Option<String>,
    }

    pub enum ValidatedCallbackResponse {
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
    ///
    /// * `PayError` indicates that an error occurred while trying to pay the invoice from the LNURL endpoint.
    /// This includes the payment hash of the failed invoice and the failure reason.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[allow(clippy::large_enum_variant)]
    pub enum LnUrlPayResult {
        EndpointSuccess { data: LnUrlPaySuccessData },
        EndpointError { data: LnUrlErrorData },
        PayError { data: LnUrlPayErrorData },
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct LnUrlPayErrorData {
        pub payment_hash: String,
        pub reason: String,
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct LnUrlPaySuccessData {
        pub success_action: Option<SuccessActionProcessed>,
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

    /// Result of decryption of [AesSuccessActionData] payload
    #[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
    pub enum AesSuccessActionDataResult {
        Decrypted { data: AesSuccessActionDataDecrypted },
        ErrorStatus { reason: String },
    }

    /// Wrapper for the decrypted [AesSuccessActionData] payload
    #[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
    pub struct AesSuccessActionDataDecrypted {
        /// Contents description, up to 144 characters
        pub description: String,

        /// Decrypted content
        pub plaintext: String,
    }

    #[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
    pub struct MessageSuccessActionData {
        pub message: String,
    }

    #[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
    pub struct UrlSuccessActionData {
        pub description: String,
        pub url: String,
    }

    /// [SuccessAction] where contents are ready to be consumed by the caller
    ///
    /// Contents are identical to [SuccessAction], except for AES where the ciphertext is decrypted.
    #[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
    pub enum SuccessActionProcessed {
        /// See [SuccessAction::Aes] for received payload
        ///
        /// See [AesSuccessActionDataDecrypted] for decrypted payload
        Aes { result: AesSuccessActionDataResult },

        /// See [SuccessAction::Message]
        Message { data: MessageSuccessActionData },

        /// See [SuccessAction::Url]
        Url { data: UrlSuccessActionData },
    }

    impl FromSql for SuccessActionProcessed {
        fn column_result(
            value: rusqlite::types::ValueRef<'_>,
        ) -> rusqlite::types::FromSqlResult<Self> {
            serde_json::from_str(value.as_str()?).map_err(|_| FromSqlError::InvalidType)
        }
    }

    impl ToSql for SuccessActionProcessed {
        fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
            Ok(ToSqlOutput::from(
                serde_json::to_string(&self).map_err(|_| FromSqlError::InvalidType)?,
            ))
        }
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
        pub fn validate(&self) -> LnUrlResult<()> {
            ensure_sdk!(
                self.description.len() <= 144,
                LnUrlError::generic(
                    "AES action description length is larger than the maximum allowed"
                )
            );

            ensure_sdk!(
                self.ciphertext.len() <= 4096,
                LnUrlError::generic(
                    "AES action ciphertext length is larger than the maximum allowed"
                )
            );

            base64::decode(&self.ciphertext)?;

            ensure_sdk!(
                self.iv.len() == 24,
                LnUrlError::generic("AES action iv has unexpected length")
            );

            base64::decode(&self.iv)?;

            Ok(())
        }

        /// Decrypts the ciphertext as a UTF-8 string, given the key (invoice preimage) parameter.
        pub fn decrypt(&self, key: &[u8; 32]) -> Result<String> {
            let plaintext_bytes =
                Aes256CbcDec::new_from_slices(key, &base64::decode(&self.iv)?)?
                    .decrypt_padded_vec_mut::<Pkcs7>(&base64::decode(&self.ciphertext)?)?;

            Ok(String::from_utf8(plaintext_bytes)?)
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
        pub fn validate(&self) -> LnUrlResult<()> {
            match self.message.len() <= 144 {
                true => Ok(()),
                false => Err(LnUrlError::generic(
                    "Success action message is longer than the maximum allowed length",
                )),
            }
        }
    }

    impl UrlSuccessActionData {
        pub fn validate(&self, data: &LnUrlPayRequestData) -> LnUrlResult<()> {
            match self.description.len() <= 144 {
                true => Ok(()),
                false => Err(LnUrlError::generic(
                    "Success action description is longer than the maximum allowed length",
                )),
            }
            .and_then(|_| {
                let req_url = reqwest::Url::parse(&data.callback)
                    .map_err(|e| LnUrlError::InvalidUri(e.to_string()))?;
                let req_domain = req_url.domain().ok_or_else(|| {
                    LnUrlError::invalid_uri("Could not determine callback domain")
                })?;

                let action_res_url = reqwest::Url::parse(&self.url)
                    .map_err(|e| LnUrlError::InvalidUri(e.to_string()))?;
                let action_res_domain = action_res_url.domain().ok_or_else(|| {
                    LnUrlError::invalid_uri("Could not determine Success Action URL domain")
                })?;

                match req_domain == action_res_domain {
                    true => Ok(()),
                    false => Err(LnUrlError::generic(
                        "Success Action URL has different domain than the callback domain",
                    )),
                }
            })
        }
    }

    /// Error returned by [crate::breez_services::BreezServices::lnurl_pay]
    #[derive(Clone, Debug, Error)]
    pub enum LnUrlPayError {
        /// This error is raised when attempting to pay an invoice that has already being paid.
        #[error("Invoice already paid")]
        AlreadyPaid,

        /// This error is raised when a general error occurs not specific to other error variants
        /// in this enum.
        #[error("Generic: {err}")]
        Generic { err: String },

        /// This error is raised when the amount from the parsed invoice is not set.
        #[error("Invalid amount: {err}")]
        InvalidAmount { err: String },

        /// This error is raised when the lightning invoice cannot be parsed.
        #[error("Invalid invoice: {err}")]
        InvalidInvoice { err: String },

        /// This error is raised when the lightning invoice is for a different Bitcoin network.
        #[error("Invalid network: {err}")]
        InvalidNetwork { err: String },

        /// This error is raised when the decoded LNURL URI is not compliant to the specification.
        #[error("Invalid uri: {err}")]
        InvalidUri { err: String },

        /// This error is raised when the lightning invoice has passed it's expiry time.
        #[error("Invoice expired: {err}")]
        InvoiceExpired { err: String },

        /// This error is raised when attempting to make a payment by the node fails.
        #[error("Payment failed: {err}")]
        PaymentFailed { err: String },

        /// This error is raised when attempting to make a payment takes too long.
        #[error("Payment timeout: {err}")]
        PaymentTimeout { err: String },

        /// This error is raised when no route can be found when attempting to make a
        /// payment by the node.
        #[error("Route not found: {err}")]
        RouteNotFound { err: String },

        /// This error is raised when the route is considered too expensive when
        /// attempting to make a payment by the node.
        #[error("Route too expensive: {err}")]
        RouteTooExpensive { err: String },

        /// This error is raised when a connection to an external service fails.
        #[error("Service connectivity: {err}")]
        ServiceConnectivity { err: String },
    }

    impl From<anyhow::Error> for LnUrlPayError {
        fn from(err: anyhow::Error) -> Self {
            Self::Generic {
                err: err.to_string(),
            }
        }
    }

    impl From<bitcoin::hashes::hex::Error> for LnUrlPayError {
        fn from(err: bitcoin::hashes::hex::Error) -> Self {
            Self::Generic {
                err: err.to_string(),
            }
        }
    }

    impl From<InvoiceError> for LnUrlPayError {
        fn from(value: InvoiceError) -> Self {
            match value {
                InvoiceError::InvalidNetwork(err) => Self::InvalidNetwork { err },
                InvoiceError::Validation(err) => Self::InvalidInvoice { err },
                InvoiceError::Generic(err) => Self::Generic { err },
            }
        }
    }

    impl From<LnUrlError> for LnUrlPayError {
        fn from(value: LnUrlError) -> Self {
            match value {
                LnUrlError::InvalidUri(err) => Self::InvalidUri { err },
                LnUrlError::InvalidInvoice(err) => Self::InvalidInvoice { err },
                LnUrlError::ServiceConnectivity(err) => Self::ServiceConnectivity { err },
                _ => Self::Generic {
                    err: value.to_string(),
                },
            }
        }
    }
}
