use std::collections::HashMap;

use crate::error::SdkResult;
use crate::grpc::RatesRequest;
use crate::models::FiatAPI;
use crate::{breez_services::BreezServer, error::SdkError};
use serde::{Deserialize, Serialize};
use tonic::Request;

/// Settings for the symbol representation of a currency
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Symbol {
    pub grapheme: Option<String>,
    pub template: Option<String>,
    pub rtl: Option<bool>,
    pub position: Option<u32>,
}

/// Locale-specific settings for the representation of a currency
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LocaleOverrides {
    pub locale: String,
    pub spacing: Option<u32>,
    pub symbol: Symbol,
}

/// Localized name of a currency
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LocalizedName {
    pub locale: String,
    pub name: String,
}

/// Details about a supported currency in the fiat rate feed
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyInfo {
    pub name: String,
    pub fraction_size: u32,
    pub spacing: Option<u32>,
    pub symbol: Option<Symbol>,
    pub uniq_symbol: Option<Symbol>,
    pub localized_name: Option<Vec<LocalizedName>>,
    pub locale_overrides: Option<Vec<LocaleOverrides>>,
}

/// Wrapper around the [CurrencyInfo] of a fiat currency
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FiatCurrency {
    pub id: String,
    pub info: CurrencyInfo,
}

/// Denominator in an exchange rate
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rate {
    pub coin: String,
    pub value: f64,
}

fn convert_to_fiat_currency_with_id(id: String, info: CurrencyInfo) -> FiatCurrency {
    FiatCurrency { id, info }
}

#[tonic::async_trait]
impl FiatAPI for BreezServer {
    async fn list_fiat_currencies(&self) -> SdkResult<Vec<FiatCurrency>> {
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
        Ok(fiat_currency_list)
    }

    async fn fetch_fiat_rates(&self) -> SdkResult<Vec<Rate>> {
        let mut client = self.get_information_client().await?;

        let request = Request::new(RatesRequest {});
        let response = client
            .rates(request)
            .await
            .map_err(|e| SdkError::ServiceConnectivity {
                err: format!("Fetch rates request failed: {e}"),
            })?;
        Ok(response
            .into_inner()
            .rates
            .into_iter()
            .map(|r| Rate {
                coin: r.coin,
                value: r.value,
            })
            .collect())
    }
}
