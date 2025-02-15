use std::sync::Arc;
use anyhow::Result;
use url::Url;
use crate::{grpc::breez::SignUrlRequest, prelude::BreezServer};
use super::BuyBitcoinProviderApi;

#[derive(Clone)]
struct MoonPayConfig {
    pub base_url: String,
    pub api_key: String,
    pub lock_amount: String,
    pub currency_code: String,
    pub color_code: String,
    pub redirect_url: String,
    pub enabled_payment_methods: String,
}

fn moonpay_config() -> MoonPayConfig {
    MoonPayConfig {
        base_url: String::from("https://buy.moonpay.io"),
        api_key: String::from("pk_live_Mx5g6bpD6Etd7T0bupthv7smoTNn2Vr"),
        lock_amount: String::from("true"),
        currency_code: String::from("btc"),
        color_code: String::from("#055DEB"),
        redirect_url: String::from("https://buy.moonpay.io/transaction_receipt?addFunds=true"),
        enabled_payment_methods: String::from(
            "credit_debit_card,sepa_bank_transfer,gbp_bank_transfer",
        ),
    }
}

fn create_moonpay_url(
    wallet_address: String,
    quote_currency_amount: Option<String>,
    max_quote_currency_amount: Option<String>,
    redirect_url: Option<String>,
) -> Result<Url> {
    let config = moonpay_config();
    let mut params = vec![
        ("apiKey", config.api_key),
        ("currencyCode", config.currency_code),
        ("colorCode", config.color_code),
        ("redirectURL", redirect_url.unwrap_or(config.redirect_url)),
        ("enabledPaymentMethods", config.enabled_payment_methods),
        ("walletAddress", wallet_address),
    ];

    if let Some(quote_currency_amount) = quote_currency_amount {
        params.extend(vec![
            ("quoteCurrencyAmount", quote_currency_amount),
            ("lockAmount", config.lock_amount),
        ]);
    }

    if let Some(max_quote_currency_amount) = max_quote_currency_amount {
        params.extend(vec![("maxQuoteCurrencyAmount", max_quote_currency_amount)]);
    }

    let url = Url::parse_with_params(&config.base_url, params)?;
    Ok(url)
}

pub struct MoonpayProvider {
    breez_server: Arc<BreezServer>,
}

impl MoonpayProvider {
    pub fn new(breez_server: Arc<BreezServer>) -> Self {
        Self { breez_server }
    }
}

#[tonic::async_trait]
impl BuyBitcoinProviderApi for MoonpayProvider {
    async fn buy_bitcoin(
        &self,
        address: String,
        locked_amount_sat: Option<u64>,
        max_amount_sat: Option<u64>,
        redirect_url: Option<String>,
    ) -> Result<String> {
        let config = moonpay_config();
        let url = create_moonpay_url(
            address,
            locked_amount_sat.map(|amount| format!("{:.8}", amount as f64 / 100_000_000.0)),
            max_amount_sat.map(|amount| format!("{:.8}", amount as f64 / 100_000_000.0)),
            redirect_url,
        )?;
        let mut signer = self.breez_server.get_signer_client().await;
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
    use crate::prelude::moonpay::{create_moonpay_url, moonpay_config};

    #[tokio::test]
    async fn test_sign_moonpay_url() -> Result<(), Box<dyn std::error::Error>> {
        let wallet_address = "a wallet address".to_string();
        let quote_amount = "a quote amount".to_string();
        let config = moonpay_config();

        let url = create_moonpay_url(
            wallet_address.clone(),
            Some(quote_amount.clone()),
            None,
            None,
        )?;

        let query_pairs = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
        assert_eq!(url.host_str(), Some("buy.moonpay.io"));
        assert_eq!(url.path(), "/");
        assert_eq!(query_pairs.get("apiKey"), Some(&config.api_key));
        assert_eq!(query_pairs.get("currencyCode"), Some(&config.currency_code));
        assert_eq!(query_pairs.get("colorCode"), Some(&config.color_code));
        assert_eq!(query_pairs.get("redirectURL"), Some(&config.redirect_url));
        assert_eq!(query_pairs.get("lockAmount"), Some(&config.lock_amount));
        assert_eq!(
            query_pairs.get("enabledPaymentMethods"),
            Some(&config.enabled_payment_methods),
        );
        assert_eq!(query_pairs.get("walletAddress"), Some(&wallet_address));
        assert_eq!(query_pairs.get("quoteCurrencyAmount"), Some(&quote_amount),);
        Ok(())
    }

    #[tokio::test]
    async fn test_sign_moonpay_url_with_redirect() -> Result<(), Box<dyn std::error::Error>> {
        let wallet_address = "a wallet address".to_string();
        let quote_amount = "a quote amount".to_string();
        let redirect_url = "https://test.moonpay.url/receipt".to_string();
        let config = moonpay_config();

        let url = create_moonpay_url(
            wallet_address.clone(),
            Some(quote_amount.clone()),
            None,
            Some(redirect_url.clone()),
        )?;

        let query_pairs = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
        assert_eq!(url.host_str(), Some("buy.moonpay.io"));
        assert_eq!(url.path(), "/");
        assert_eq!(query_pairs.get("apiKey"), Some(&config.api_key));
        assert_eq!(query_pairs.get("currencyCode"), Some(&config.currency_code));
        assert_eq!(query_pairs.get("colorCode"), Some(&config.color_code));
        assert_eq!(query_pairs.get("redirectURL"), Some(&redirect_url));
        assert_eq!(query_pairs.get("lockAmount"), Some(&config.lock_amount));
        assert_eq!(
            query_pairs.get("enabledPaymentMethods"),
            Some(&config.enabled_payment_methods),
        );
        assert_eq!(query_pairs.get("walletAddress"), Some(&wallet_address));
        assert_eq!(query_pairs.get("quoteCurrencyAmount"), Some(&quote_amount),);
        Ok(())
    }
}
