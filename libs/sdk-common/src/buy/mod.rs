use anyhow::Result;
use async_trait::async_trait;

pub mod moonpay;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait BuyBitcoinProviderApi: Send + Sync {
    /// Configure buying Bitcoin and return a URL to continue
    async fn buy_bitcoin(
        &self,
        address: String,
        locked_amount_sat: Option<u64>,
        max_amount_sat: Option<u64>,
        redirect_url: Option<String>,
    ) -> Result<String>;
}
