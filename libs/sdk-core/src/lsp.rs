use crate::breez_services::{BreezServer, OpeningFeeParamsMenu};
use crate::crypt::encrypt;
use crate::grpc::{
    self, LspListRequest, PaymentInformation, RegisterPaymentReply, RegisterPaymentRequest,
};
use crate::models::{LspAPI, OpeningFeeParams};
use anyhow::Result;
use prost::Message;
use serde::{Deserialize, Serialize};
use tonic::Request;

/// Details of supported LSP
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LspInformation {
    pub id: String,
    pub name: String,
    pub widget_url: String,
    pub pubkey: String,
    pub host: String,
    pub channel_capacity: i64,
    pub target_conf: i32,
    pub base_fee_msat: i64,
    pub fee_rate: f64,
    pub time_lock_delta: u32,
    pub min_htlc_msat: i64,
    #[deprecated]
    pub channel_fee_permyriad: i64,
    pub lsp_pubkey: Vec<u8>,
    #[deprecated]
    pub max_inactive_duration: i64,
    #[deprecated]
    pub channel_minimum_fee_msat: i64,
    pub opening_fee_params_menu: Vec<OpeningFeeParams>,
}

impl LspInformation {
    fn from(lsp_id: String, lsp_info: grpc::LspInformation) -> Self {
        LspInformation {
            id: lsp_id,
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
            channel_fee_permyriad: lsp_info.channel_fee_permyriad,
            lsp_pubkey: lsp_info.lsp_pubkey,
            max_inactive_duration: lsp_info.max_inactive_duration,
            channel_minimum_fee_msat: lsp_info.channel_minimum_fee_msat,
            opening_fee_params_menu: lsp_info
                .opening_fee_params_menu
                .into_iter()
                .map(|ofp| ofp.into())
                .collect::<Vec<OpeningFeeParams>>(),
        }
    }

    pub(crate) fn choose_channel_opening_fees(
        &self,
        maybe_user_supplied_fee_params: Option<OpeningFeeParams>,
        cheapest_or_longest: bool,
    ) -> Result<Option<OpeningFeeParams>> {
        match maybe_user_supplied_fee_params {
            // Validate given opening_fee_params and use it if possible
            Some(user_supplied_fees) => {
                user_supplied_fees.validate()?;
                Ok(Some(user_supplied_fees))
            }
            // We pick our own if None is supplied
            None => {
                let menu = OpeningFeeParamsMenu::try_from(self.opening_fee_params_menu.clone())?;
                match cheapest_or_longest {
                    true => Ok(menu.get_cheapest_opening_fee_params()),
                    false => menu.get_longest_valid_opening_fee_params(),
                }
            }
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
            lsp_list.push(LspInformation::from(lsp_id, lsp_info));
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
