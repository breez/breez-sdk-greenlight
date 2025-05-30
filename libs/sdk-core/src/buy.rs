use std::sync::Arc;

use anyhow::Result;
use sdk_common::prelude::{BreezServer, BuyBitcoinProviderApi, MoonpayProvider};

use crate::{BuyBitcoinProvider, SwapInfo};

#[tonic::async_trait]
pub(crate) trait BuyBitcoinApi: Send + Sync {
    /// Initiate buying Bitcoin and return a URL to the selected third party provider
    async fn buy_bitcoin(
        &self,
        provider: BuyBitcoinProvider,
        swap_info: &SwapInfo,
        redirect_url: Option<String>,
    ) -> Result<String>;

    /// Fetches the minimum and maximum buy limits
    async fn buy_bitcoin_limits(
        &self,
        provider: BuyBitcoinProvider,
        fiat_currency_code: Option<String>,
    ) -> Result<(u64, u64)>;
}

pub(crate) struct BuyBitcoinService {
    moonpay_provider: Arc<dyn BuyBitcoinProviderApi>,
}

impl BuyBitcoinService {
    pub fn new(breez_server: Arc<BreezServer>) -> Self {
        let moonpay_provider = Arc::new(MoonpayProvider::new(breez_server));
        Self { moonpay_provider }
    }
}

#[tonic::async_trait]
impl BuyBitcoinApi for BuyBitcoinService {
    async fn buy_bitcoin(
        &self,
        provider: BuyBitcoinProvider,
        swap_info: &SwapInfo,
        redirect_url: Option<String>,
    ) -> Result<String> {
        match provider {
            BuyBitcoinProvider::Moonpay => {
                self.moonpay_provider
                    .buy_bitcoin(
                        swap_info.bitcoin_address.clone(),
                        None,
                        Some(swap_info.max_allowed_deposit as u64),
                        redirect_url,
                    )
                    .await
            }
        }
    }

    async fn buy_bitcoin_limits(
        &self,
        provider: BuyBitcoinProvider,
        fiat_currency_code: Option<String>,
    ) -> Result<(u64, u64)> {
        match provider {
            BuyBitcoinProvider::Moonpay => {
                self.moonpay_provider
                    .buy_bitcoin_limits(fiat_currency_code)
                    .await
            }
        }
    }
}
