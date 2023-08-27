use crate::breez_services::BreezServer;
use crate::crypt::encrypt;
use crate::error::SdkResult;
use crate::grpc::{
    self, LspListRequest, PaymentInformation, RegisterPaymentReply, RegisterPaymentRequest,
};
use crate::models::{LspAPI, OpeningFeeParams, OpeningFeeParamsMenu};
use crate::DynamicFeeType;
use anyhow::Result;
use prost::Message;
use serde::{Deserialize, Serialize};
use tonic::Request;

/// Details of supported LSP
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LspInformation {
    pub id: String,

    /// The name of of LSP
    pub name: String,

    /// The URL of the LSP
    pub widget_url: String,

    /// The identity pubkey of the Lightning node
    pub pubkey: String,

    /// The network location of the lightning node, e.g. `12.34.56.78:9012` or `localhost:10011`
    pub host: String,

    /// The channel capacity in satoshis
    pub channel_capacity: i64,

    /// The target number of blocks that the funding transaction should be confirmed by
    pub target_conf: i32,

    /// The base fee charged regardless of the number of milli-satoshis sent
    pub base_fee_msat: i64,

    /// The effective fee rate in milli-satoshis. The precision of this value goes up to 6 decimal places, so 1e-6.
    pub fee_rate: f64,

    /// The required timelock delta for HTLCs forwarded over the channel
    pub time_lock_delta: u32,

    /// The minimum value in millisatoshi we will require for incoming HTLCs on the channel
    pub min_htlc_msat: i64,
    pub lsp_pubkey: Vec<u8>,
    pub opening_fee_params_list: OpeningFeeParamsMenu,
}

impl LspInformation {
    /// Validation may fail if [LspInformation.opening_fee_params_list] has invalid entries
    fn try_from(lsp_id: &str, lsp_info: grpc::LspInformation) -> Result<Self> {
        let info = LspInformation {
            id: lsp_id.to_string(),
            name: lsp_info.name,
            widget_url: lsp_info.widget_url,
            pubkey: lsp_info.pubkey,
            host: lsp_info.host,
            channel_capacity: lsp_info.channel_capacity,
            target_conf: lsp_info.target_conf,
            base_fee_msat: lsp_info.base_fee_msat,
            fee_rate: lsp_info.fee_rate,
            time_lock_delta: lsp_info.time_lock_delta,
            min_htlc_msat: lsp_info.min_htlc_msat,
            lsp_pubkey: lsp_info.lsp_pubkey,
            opening_fee_params_list: OpeningFeeParamsMenu::try_from(
                lsp_info.opening_fee_params_list,
            )?,
        };

        Ok(info)
    }

    /// Returns the channel opening fees, either the ones provided by the user (if any), or the ones from LSP.
    ///
    /// If the LSP fees are needed, the LSP is expected to have at least one dynamic fee entry in its menu,
    /// otherwise this will result in an error.
    pub(crate) fn choose_channel_opening_fees(
        &self,
        maybe_user_supplied_fee_params: Option<OpeningFeeParams>,
        fee_type: DynamicFeeType,
    ) -> SdkResult<OpeningFeeParams> {
        match maybe_user_supplied_fee_params {
            // Validate given opening_fee_params and use it if possible
            Some(user_supplied_fees) => {
                user_supplied_fees.validate()?;
                Ok(user_supplied_fees)
            }
            // We pick our own if None is supplied
            None => match fee_type {
                DynamicFeeType::Cheapest => self
                    .opening_fee_params_list
                    .get_cheapest_opening_fee_params(),
                DynamicFeeType::Longest => {
                    self.opening_fee_params_list.get_48h_opening_fee_params()
                }
            },
        }
    }
}

#[tonic::async_trait]
impl LspAPI for BreezServer {
    async fn list_lsps(&self, pubkey: String) -> Result<Vec<LspInformation>> {
        let mut client = self.get_channel_opener_client().await?;

        let request = Request::new(LspListRequest { pubkey });
        let response = client.lsp_list(request).await?;
        let mut lsp_list: Vec<LspInformation> = Vec::new();
        for (lsp_id, lsp_info) in response.into_inner().lsps.into_iter() {
            match LspInformation::try_from(&lsp_id, lsp_info) {
                Ok(lsp) => lsp_list.push(lsp),
                Err(e) => error!("LSP Information validation failed for LSP {lsp_id}: {e}"),
            }
        }
        lsp_list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Ok(lsp_list)
    }

    async fn register_payment(
        &self,
        lsp_id: String,
        lsp_pubkey: Vec<u8>,
        payment_info: PaymentInformation,
    ) -> Result<RegisterPaymentReply> {
        let mut client = self.get_channel_opener_client().await?;

        let mut buf = Vec::new();
        buf.reserve(payment_info.encoded_len());
        payment_info.encode(&mut buf)?;

        let request = Request::new(RegisterPaymentRequest {
            lsp_id,
            blob: encrypt(lsp_pubkey, buf)?,
        });
        let response = client.register_payment(request).await?;

        Ok(response.into_inner())
    }
}
