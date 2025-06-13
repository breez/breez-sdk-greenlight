use anyhow::Result;
use maybe_sync::{MaybeSend, MaybeSync};

pub mod moonpay;

#[sdk_macros::async_trait]
pub trait BuyBitcoinProviderApi: MaybeSend + MaybeSync {
    /// Configure buying Bitcoin and return a URL to continue
    async fn buy_bitcoin(
        &self,
        address: String,
        locked_amount_sat: Option<u64>,
        max_amount_sat: Option<u64>,
        redirect_url: Option<String>,
        fiat_currency_code: Option<String>,
    ) -> Result<String>;
}
