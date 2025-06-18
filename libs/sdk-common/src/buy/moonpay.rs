use anyhow::Result;
use serde::Deserialize;
use url::Url;

use crate::{grpc::SignUrlRequest, prelude::BreezServer, utils::Arc};

use super::BuyBitcoinProviderApi;

#[derive(Clone)]
struct MoonPayConfig {
    // Shared configuration
    pub api_key: String,
    pub btc_currency_code: String,

    // Widget-specific configuration
    pub widget_base_url: String,
    pub lock_amount: String,
    pub color_code: String,
    pub default_redirect_url: String,
    pub enabled_payment_methods: String,

    // API-specific configuration
    pub api_base_url: String,
    pub currencies_endpoint_path: String,
    pub limits_endpoint_path: String,
    pub default_fiat_currency: String,
}

impl MoonPayConfig {
    fn new() -> Self {
        let currencies_endpoint_path = String::from("/currencies");
        let limits_endpoint_path = format!("{}/{{currency}}/limits", &currencies_endpoint_path);

        Self {
            // Shared
            api_key: String::from("pk_live_Mx5g6bpD6Etd7T0bupthv7smoTNn2Vr"),
            btc_currency_code: String::from("btc"),

            // Widget configuration
            widget_base_url: String::from("https://buy.moonpay.io"),
            lock_amount: String::from("true"),
            color_code: String::from("#055DEB"),
            default_redirect_url: String::from(
                "https://buy.moonpay.io/transaction_receipt?addFunds=true",
            ),
            enabled_payment_methods: String::from(
                "credit_debit_card,sepa_bank_transfer,gbp_bank_transfer",
            ),

            // API configuration
            api_base_url: String::from("https://api.moonpay.com/v3"),
            currencies_endpoint_path,
            limits_endpoint_path,
            default_fiat_currency: String::from("usd"),
        }
    }

    pub fn create_widget_url(
        &self,
        wallet_address: String,
        quote_currency_amount: Option<String>,
        max_quote_currency_amount: Option<String>,
        redirect_url: Option<String>,
    ) -> Result<Url> {
        let mut params = vec![
            ("apiKey", self.api_key.clone()),
            ("currencyCode", self.btc_currency_code.clone()),
            ("colorCode", self.color_code.clone()),
            (
                "redirectURL",
                redirect_url.unwrap_or_else(|| self.default_redirect_url.clone()),
            ),
            (
                "enabledPaymentMethods",
                self.enabled_payment_methods.clone(),
            ),
            ("walletAddress", wallet_address),
        ];

        if let Some(quote_currency_amount) = quote_currency_amount {
            params.extend(vec![
                ("quoteCurrencyAmount", quote_currency_amount),
                ("lockAmount", self.lock_amount.clone()),
            ]);
        }

        if let Some(max_quote_currency_amount) = max_quote_currency_amount {
            params.extend(vec![("maxQuoteCurrencyAmount", max_quote_currency_amount)]);
        }

        let url = Url::parse_with_params(&self.widget_base_url, params)?;
        Ok(url)
    }

    pub fn build_limits_url(&self, fiat_currency: &str) -> String {
        let endpoint = self
            .limits_endpoint_path
            .replace("{currency}", &self.btc_currency_code);
        format!(
            "{}{}?baseCurrencyCode={}&apiKey={}",
            self.api_base_url,
            endpoint,
            fiat_currency.to_lowercase(),
            self.api_key
        )
    }

    pub fn build_currencies_url(&self) -> String {
        format!(
            "{}{}?apiKey={}",
            self.api_base_url, self.currencies_endpoint_path, self.api_key
        )
    }

    // Utility methods
    pub fn get_fiat_currency(&self, requested: Option<String>) -> String {
        requested.unwrap_or_else(|| self.default_fiat_currency.clone())
    }

    pub fn has_valid_api_key_format(&self) -> bool {
        self.api_key.starts_with("pk_") && self.api_key.len() > 10
    }
}

#[derive(Deserialize, Debug)]
struct MoonPayLimitsResponse {
    #[serde(rename = "quoteCurrency")]
    quote_currency: QuoteCurrency,
}

#[derive(Deserialize, Debug)]
struct QuoteCurrency {
    #[allow(dead_code)]
    code: String,
    #[serde(rename = "minBuyAmount")]
    min_buy_amount: f64,
    #[serde(rename = "maxBuyAmount")]
    max_buy_amount: f64,
}

#[derive(Deserialize, Debug)]
struct CurrencyResponse {
    #[serde(rename = "type")]
    currency_type: String,
    code: String,
}

pub struct MoonpayProvider {
    breez_server: Arc<BreezServer>,
}

impl MoonpayProvider {
    pub fn new(breez_server: Arc<BreezServer>) -> Self {
        Self { breez_server }
    }

    // Helper method to convert BTC amount to satoshis
    fn btc_to_satoshis(btc_amount: f64) -> u64 {
        (btc_amount * 100_000_000.0) as u64
    }

    async fn buy_bitcoin_limits(&self, fiat_currency_code: Option<String>) -> Result<(u64, u64)> {
        let config = MoonPayConfig::new();

        if !config.has_valid_api_key_format() {
            return Err(anyhow::anyhow!("Invalid MoonPay API key format"));
        }

        let fiat_currency = config.get_fiat_currency(fiat_currency_code);

        if !self.is_currency_supported(&fiat_currency).await? {
            return Err(anyhow::anyhow!(
                "Fiat currency '{}' is not supported by MoonPay",
                fiat_currency
            ));
        }

        let api_url = config.build_limits_url(&fiat_currency);

        let client = reqwest::Client::new();
        let response = client.get(&api_url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "MoonPay API request failed with status: {} for fiat currency: {}",
                response.status(),
                fiat_currency
            ));
        }

        let moonpay_limits: MoonPayLimitsResponse = response.json().await?;

        Ok((
            MoonpayProvider::btc_to_satoshis(moonpay_limits.quote_currency.min_buy_amount),
            MoonpayProvider::btc_to_satoshis(moonpay_limits.quote_currency.max_buy_amount),
        ))
    }

    // Validate amounts against MoonPay buy limits
    async fn validate_amounts(
        &self,
        locked_amount_sat: Option<u64>,
        max_amount_sat: Option<u64>,
        fiat_currency_code: Option<String>,
    ) -> Result<()> {
        // Skip validation if no amounts are provided
        if locked_amount_sat.is_none() && max_amount_sat.is_none() {
            return Ok(());
        }

        let (min_buy_amount_sat, max_buy_amount_sat) =
            self.buy_bitcoin_limits(fiat_currency_code).await?;

        // Validate locked amount if provided
        if let Some(locked_amount) = locked_amount_sat {
            if locked_amount < min_buy_amount_sat {
                return Err(anyhow::anyhow!(
                    "Locked amount {} satoshis is below minimum buy amount {} satoshis",
                    locked_amount,
                    min_buy_amount_sat
                ));
            }

            if locked_amount > max_buy_amount_sat {
                return Err(anyhow::anyhow!(
                    "Locked amount {} satoshis exceeds maximum buy amount {} satoshis",
                    locked_amount,
                    max_buy_amount_sat
                ));
            }
        }

        // Validate max amount if provided
        if let Some(max_amount) = max_amount_sat {
            if max_amount > max_buy_amount_sat {
                return Err(anyhow::anyhow!(
                    "Maximum amount {} satoshis exceeds maximum buy amount {} satoshis",
                    max_amount,
                    max_buy_amount_sat
                ));
            }

            if max_amount < min_buy_amount_sat {
                return Err(anyhow::anyhow!(
                    "Maximum amount {} satoshis is below minimum buy amount {} satoshis",
                    max_amount,
                    min_buy_amount_sat
                ));
            }
        }

        // Cross-validation: ensure locked_amount <= max_amount
        if let (Some(locked_amount), Some(max_amount)) = (locked_amount_sat, max_amount_sat) {
            if locked_amount > max_amount {
                return Err(anyhow::anyhow!(
                    "Locked amount {} satoshis cannot exceed maximum amount {} satoshis",
                    locked_amount,
                    max_amount
                ));
            }
        }

        Ok(())
    }

    async fn is_currency_supported(&self, currency_code: &str) -> Result<bool> {
        let config = MoonPayConfig::new();
        let api_url = config.build_currencies_url();

        let client = reqwest::Client::new();
        let response = client.get(&api_url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch supported currencies"));
        }

        let currencies: Vec<CurrencyResponse> = response.json().await?;

        // Check if the fiat currency is supported
        Ok(currencies.iter().any(|currency| {
            currency.code.to_lowercase() == currency_code.to_lowercase()
                && currency.currency_type == "fiat"
        }))
    }
}

#[sdk_macros::async_trait]
impl BuyBitcoinProviderApi for MoonpayProvider {
    async fn buy_bitcoin(
        &self,
        address: String,
        locked_amount_sat: Option<u64>,
        max_amount_sat: Option<u64>,
        redirect_url: Option<String>,
        fiat_currency_code: Option<String>,
    ) -> Result<String> {
        let config = MoonPayConfig::new();

        if !config.has_valid_api_key_format() {
            return Err(anyhow::anyhow!("Invalid MoonPay API key format"));
        }

        // Validate amounts against buy limits
        self.validate_amounts(locked_amount_sat, max_amount_sat, fiat_currency_code)
            .await?;

        // Create widget URL for user to complete their Bitcoin purchase
        let url = config.create_widget_url(
            address,
            locked_amount_sat.map(|amount| format!("{:.8}", amount as f64 / 100_000_000.0)),
            max_amount_sat.map(|amount| format!("{:.8}", amount as f64 / 100_000_000.0)),
            redirect_url,
        )?;
        let mut signer = self.breez_server.get_signer_client().await;
        let signed_url = signer
            .sign_url(SignUrlRequest {
                base_url: config.widget_base_url.clone(),
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

    use crate::prelude::moonpay::{moonpay_config, MoonpayProvider};

    #[sdk_macros::async_test_all]
    async fn test_sign_moonpay_url() -> Result<(), Box<dyn std::error::Error>> {
        let wallet_address = "a wallet address".to_string();
        let quote_amount = "a quote amount".to_string();
        let config = MoonPayConfig::new();

        let url = config.create_widget_url(
            wallet_address.clone(),
            Some(quote_amount.clone()),
            None,
            None,
        )?;

        let query_pairs = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
        assert_eq!(url.host_str(), Some("buy.moonpay.io"));
        assert_eq!(url.path(), "/");
        assert_eq!(query_pairs.get("apiKey"), Some(&config.api_key));
        assert_eq!(
            query_pairs.get("currencyCode"),
            Some(&config.btc_currency_code)
        );
        assert_eq!(query_pairs.get("colorCode"), Some(&config.color_code));
        assert_eq!(
            query_pairs.get("redirectURL"),
            Some(&config.default_redirect_url)
        );
        assert_eq!(query_pairs.get("lockAmount"), Some(&config.lock_amount));
        assert_eq!(
            query_pairs.get("enabledPaymentMethods"),
            Some(&config.enabled_payment_methods),
        );
        assert_eq!(query_pairs.get("walletAddress"), Some(&wallet_address));
        assert_eq!(query_pairs.get("quoteCurrencyAmount"), Some(&quote_amount),);
        Ok(())
    }

    #[sdk_macros::async_test_all]
    async fn test_sign_moonpay_url_with_redirect() -> Result<(), Box<dyn std::error::Error>> {
        let wallet_address = "a wallet address".to_string();
        let quote_amount = "a quote amount".to_string();
        let redirect_url = "https://test.moonpay.url/receipt".to_string();
        let config = MoonPayConfig::new();

        let url = config.create_widget_url(
            wallet_address.clone(),
            Some(quote_amount.clone()),
            None,
            Some(redirect_url.clone()),
        )?;

        let query_pairs = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
        assert_eq!(url.host_str(), Some("buy.moonpay.io"));
        assert_eq!(url.path(), "/");
        assert_eq!(query_pairs.get("apiKey"), Some(&config.api_key));
        assert_eq!(
            query_pairs.get("currencyCode"),
            Some(&config.btc_currency_code)
        );
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

    #[test]
    fn test_btc_to_satoshis_conversion() {
        assert_eq!(MoonpayProvider::btc_to_satoshis(0.001), 100_000);
        assert_eq!(MoonpayProvider::btc_to_satoshis(1.0), 100_000_000);
        assert_eq!(MoonpayProvider::btc_to_satoshis(0.00000001), 1);
    }

    #[test]
    fn test_moonpay_config_structure() {
        let config = MoonPayConfig::new();

        // Test shared config
        assert!(config.has_valid_api_key_format());
        assert_eq!(config.btc_currency_code, "btc");

        // Test widget config
        assert!(config.widget_base_url.starts_with("https://buy.moonpay.io"));
        assert_eq!(config.color_code, "#055DEB");

        // Test API config
        assert!(config.api_base_url.starts_with("https://api.moonpay.com"));
        assert!(config.limits_endpoint_path.contains("{currency}"));
        assert_eq!(config.default_fiat_currency, "usd");
    }

    #[test]
    fn test_build_limits_url_uses_endpoint_path() {
        let config = MoonPayConfig::new();
        let url = config.build_limits_url("eur");

        assert!(url.starts_with(&config.api_base_url));
        assert!(url.contains("currencies/btc/limits"));
        assert!(url.contains("baseCurrencyCode=eur"));
        assert!(url.contains("apiKey="));
        assert!(url.contains(&config.api_key));
    }

    #[test]
    fn test_build_currencies_url_uses_endpoint_path() {
        let config = MoonPayConfig::new();
        let url = config.build_currencies_url();

        assert!(url.starts_with(&config.api_base_url));
        assert!(url.contains(&config.currencies_endpoint_path));
        assert!(url.contains("apiKey="));
        assert!(url.contains(&config.api_key));
    }

    #[test]
    fn test_get_fiat_currency() {
        let config = MoonPayConfig::new();

        // Test with provided currency
        assert_eq!(config.get_fiat_currency(Some("eur".to_string())), "eur");

        // Test with default fallback
        assert_eq!(config.get_fiat_currency(None), "usd");
    }

    #[test]
    fn test_api_key_validation() {
        let mut config = MoonPayConfig::new();
        assert!(config.has_valid_api_key_format());

        config.api_key = "invalid_key".to_string();
        assert!(!config.has_valid_api_key_format());

        config.api_key = "pk_".to_string();
        assert!(!config.has_valid_api_key_format());
    }
}
