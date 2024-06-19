use anyhow::Result;
use reqwest::Url;
use sdk_common::grpc::SignUrlRequest;
use sdk_common::prelude::BreezServer;

use crate::SwapInfo;

#[derive(Clone)]
pub(crate) struct MoonPayConfig {
    pub base_url: String,
    pub api_key: String,
    pub currency_code: String,
    pub color_code: String,
    pub redirect_url: String,
    pub enabled_payment_methods: String,
}

pub(crate) fn moonpay_config() -> MoonPayConfig {
    MoonPayConfig {
        base_url: String::from("https://buy.moonpay.io"),
        api_key: String::from("pk_live_Mx5g6bpD6Etd7T0bupthv7smoTNn2Vr"),
        currency_code: String::from("btc"),
        color_code: String::from("#055DEB"),
        redirect_url: String::from("https://buy.moonpay.io/transaction_receipt?addFunds=true"),
        enabled_payment_methods: String::from(
            "credit_debit_card,sepa_bank_transfer,gbp_bank_transfer",
        ),
    }
}

async fn create_moonpay_url(wallet_address: &str, max_amount: &str) -> Result<Url> {
    let config = moonpay_config();
    let url = Url::parse_with_params(
        &config.base_url,
        &[
            ("apiKey", &config.api_key),
            ("currencyCode", &config.currency_code),
            ("colorCode", &config.color_code),
            ("redirectURL", &config.redirect_url),
            ("enabledPaymentMethods", &config.enabled_payment_methods),
            ("walletAddress", &wallet_address.to_string()),
            ("maxQuoteCurrencyAmount", &max_amount.to_string()),
        ],
    )?;
    Ok(url)
}

#[tonic::async_trait]
pub(crate) trait MoonPayApi: Send + Sync {
    async fn buy_bitcoin_url(&self, swap_info: &SwapInfo) -> Result<String>;
}

#[tonic::async_trait]
impl MoonPayApi for BreezServer {
    async fn buy_bitcoin_url(&self, swap_info: &SwapInfo) -> Result<String> {
        let config = moonpay_config();
        let url = create_moonpay_url(
            swap_info.bitcoin_address.as_str(),
            format!("{:.8}", swap_info.max_allowed_deposit as f64 / 100000000.0).as_str(),
        )
        .await?;
        let mut signer = self.get_signer_client().await?.clone();
        let signed_url = signer
            .sign_url(SignUrlRequest {
                base_url: config.base_url.clone(),
                query_string: format!("?{}", url.query().unwrap()),
            })
            .await?
            .into_inner()
            .full_url;
        Ok(signed_url)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::HashMap;

    use crate::moonpay::{create_moonpay_url, moonpay_config};

    #[tokio::test]
    async fn test_sign_moonpay_url() -> Result<(), Box<dyn std::error::Error>> {
        let wallet_address = "a wallet address";
        let max_amount = "a max amount";
        let config = moonpay_config();

        let url = create_moonpay_url(wallet_address, max_amount).await?;

        let query_pairs = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
        assert_eq!(url.host_str(), Some("buy.moonpay.io"));
        assert_eq!(url.path(), "/");
        assert_eq!(query_pairs.get("apiKey"), Some(&config.api_key));
        assert_eq!(query_pairs.get("currencyCode"), Some(&config.currency_code));
        assert_eq!(query_pairs.get("colorCode"), Some(&config.color_code));
        assert_eq!(query_pairs.get("redirectURL"), Some(&config.redirect_url));
        assert_eq!(
            query_pairs.get("enabledPaymentMethods"),
            Some(&config.enabled_payment_methods),
        );
        assert_eq!(
            query_pairs.get("walletAddress"),
            Some(&String::from(wallet_address))
        );
        assert_eq!(
            query_pairs.get("maxQuoteCurrencyAmount"),
            Some(&String::from(max_amount)),
        );
        Ok(())
    }
}
