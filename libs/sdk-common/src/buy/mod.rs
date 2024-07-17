use anyhow::Result;

pub mod moonpay;

#[tonic::async_trait]
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
