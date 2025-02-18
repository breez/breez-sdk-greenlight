use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::grpc::RatesRequest;
use crate::prelude::BreezServer;
use crate::with_connection_retry;

/// Trait covering fiat-related functionality
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait FiatAPI: Send + Sync {
    /// List all supported fiat currencies for which there is a known exchange rate.
    async fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>>;

    /// Get the live rates from the server.
    async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>>;
}

/// Settings for the symbol representation of a currency
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi)
)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Symbol {
    pub grapheme: Option<String>,
    pub template: Option<String>,
    pub rtl: Option<bool>,
    pub position: Option<u32>,
}

/// Locale-specific settings for the representation of a currency
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi)
)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LocaleOverrides {
    pub locale: String,
    pub spacing: Option<u32>,
    pub symbol: Symbol,
}

/// Localized name of a currency
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi)
)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LocalizedName {
    pub locale: String,
    pub name: String,
}

/// Details about a supported currency in the fiat rate feed
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi)
)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyInfo {
    pub name: String,
    pub fraction_size: u32,
    pub spacing: Option<u32>,
    pub symbol: Option<Symbol>,
    pub uniq_symbol: Option<Symbol>,
    #[serde(default)]
    pub localized_name: Vec<LocalizedName>,
    #[serde(default)]
    pub locale_overrides: Vec<LocaleOverrides>,
}

/// Wrapper around the [CurrencyInfo] of a fiat currency
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi)
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FiatCurrency {
    pub id: String,
    pub info: CurrencyInfo,
}

/// Denominator in an exchange rate
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rate {
    pub coin: String,
    pub value: f64,
}

fn convert_to_fiat_currency_with_id(id: String, info: CurrencyInfo) -> FiatCurrency {
    FiatCurrency { id, info }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl FiatAPI for BreezServer {
    async fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>> {
        let known_rates = self.fetch_fiat_rates().await?;
        let known_rates_currencies = known_rates
            .iter()
            .map(|r| r.coin.clone())
            .collect::<Vec<String>>();

        let data = include_str!("../assets/json/currencies.json");
        let fiat_currency_map: HashMap<String, CurrencyInfo> = serde_json::from_str(data)?;
        let mut fiat_currency_list: Vec<FiatCurrency> = Vec::new();
        for (key, value) in fiat_currency_map {
            if known_rates_currencies.contains(&key) {
                fiat_currency_list.push(convert_to_fiat_currency_with_id(key, value));
            }
        }
        fiat_currency_list.sort_by(|a, b| a.info.name.cmp(&b.info.name));
        Ok(fiat_currency_list)
    }

    async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>> {
        let mut client = self.get_information_client().await;

        let request = RatesRequest {};
        let response = with_connection_retry!(client.rates(request.clone()))
            .await
            .map_err(|e| anyhow!("Fetch rates request failed: {e}"))?;

        let mut rates = response.into_inner().rates;
        rates.sort_by(|a, b| a.coin.cmp(&b.coin));
        Ok(rates
            .into_iter()
            .map(|r| Rate {
                coin: r.coin,
                value: r.value,
            })
            .collect())
    }
}
