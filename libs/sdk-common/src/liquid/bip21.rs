use bitcoin::util::amount::ParseAmountError;
use bitcoin::Denomination;
use elements::{
    address::{Address, AddressError, AddressParams},
    hashes::hex::HexToArrayError,
    issuance::AssetId,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{str::FromStr, string::FromUtf8Error};
use urlencoding::decode;

use crate::prelude::{Network, URISerializationError};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiquidAddressData {
    pub address: String,
    pub network: Network,
    pub asset_id: Option<String>,
    pub amount: Option<f64>,
    pub amount_sat: Option<u64>,
    pub label: Option<String>,
    pub message: Option<String>,
}

impl LiquidAddressData {
    /// Sets the precision for calculating the [LiquidAddressData::amount_sat] from the BIP21 URI amount.
    /// By default the precision 8 for Bitcoin-like assets.
    pub fn set_amount_precision(&mut self, precision: u32) {
        if let Some(amount) = self.amount {
            self.amount_sat = Some((amount * (10_u64.pow(precision) as f64)) as u64);
        }
    }

    /// Converts the structure to a BIP21 URI while also
    /// ensuring that all the fields are valid
    pub fn to_uri(&self) -> Result<String, URISerializationError> {
        self.address
            .parse::<Address>()
            .map_err(|_| URISerializationError::InvalidAddress)?;

        let mut optional_keys = HashMap::new();

        // Ensure that assetid is always set when an amount is set
        if let Some(asset_id) = self.asset_id.clone() {
            optional_keys.insert("assetid", asset_id);
        } else if self.amount.is_some() || self.amount_sat.is_some() {
            return Err(URISerializationError::AssetIdMissing);
        }

        // Take amount over amount_sat if both are set
        if let Some(amount) = self.amount {
            optional_keys.insert("amount", format!("{amount:.8}"));
        } else if let Some(amount_sat) = self.amount_sat {
            let amount_btc = amount_sat as f64 / 100_000_000.0;
            optional_keys.insert("amount", format!("{amount_btc:.8}"));
        }

        if let Some(message) = &self.message {
            optional_keys.insert("message", urlencoding::encode(message).to_string());
        }

        if let Some(label) = &self.label {
            optional_keys.insert("label", urlencoding::encode(label).to_string());
        }

        match optional_keys.is_empty() {
            true => Ok(self.address.clone()),
            false => {
                let scheme = match self.network {
                    Network::Bitcoin => "liquidnetwork",
                    Network::Testnet => "liquidtestnet",
                    Network::Regtest => "liquidregtest",
                    _ => {
                        return Err(URISerializationError::UnsupportedNetwork);
                    }
                };

                let suffix_str = optional_keys
                    .iter()
                    .map(|(key, value)| format!("{key}={value}"))
                    .collect::<Vec<String>>()
                    .join("&");

                Ok(format!("{scheme}:{}?{suffix_str}", self.address))
            }
        }
    }
}

#[derive(Debug)]
pub enum DeserializeError {
    InvalidScheme,
    MissingEquals,
    UnknownParameter,
    AssetNotProvided,
    InvalidString(FromUtf8Error),
    InvalidAmount(ParseAmountError),
    InvalidAsset(HexToArrayError),
    InvalidAddress(AddressError),
}

impl LiquidAddressData {
    fn deserialize_raw(string: &str) -> Result<Self, DeserializeError> {
        let (network, address_params) = string
            .split_once(':')
            .ok_or(DeserializeError::InvalidScheme)?;

        let network = match network {
            "liquidnetwork" => Network::Bitcoin,
            "liquidtestnet" => Network::Testnet,
            "liquidregtest" => Network::Regtest,
            _ => return Err(DeserializeError::InvalidScheme),
        };

        let mut address_params = address_params.split('?');

        let address = address_params
            .next()
            .ok_or_else(|| {
                DeserializeError::InvalidAddress(AddressError::InvalidAddress(
                    "No address provided".to_string(),
                ))
            })?
            .parse::<Address>()
            .map_err(DeserializeError::InvalidAddress)?
            .to_string();

        let mut amount = None;
        let mut amount_sat = None;
        let mut asset_id = None;
        let mut label = None;
        let mut message = None;
        if let Some(params) = address_params.next() {
            for pair in params.split('&') {
                if let Some((key, val)) = pair.split_once('=') {
                    match key {
                        "amount" => {
                            amount = bitcoin::Amount::from_str_in(val, Denomination::Bitcoin)
                                .map(|amt| Some(amt.to_float_in(Denomination::Bitcoin)))
                                .map_err(DeserializeError::InvalidAmount)?;
                            amount_sat = bitcoin::Amount::from_str_in(val, Denomination::Bitcoin)
                                .map(|amt| Some(amt.to_sat()))
                                .map_err(DeserializeError::InvalidAmount)?;
                        }
                        "assetid" => {
                            val.parse::<AssetId>()
                                .map_err(DeserializeError::InvalidAsset)?;
                            asset_id = Some(val.to_string());
                        }
                        "label" => {
                            let decoded = decode(val)
                                .map_err(DeserializeError::InvalidString)?
                                .into_owned();
                            label = Some(decoded)
                        }
                        "message" => {
                            let decoded = decode(val)
                                .map_err(DeserializeError::InvalidString)?
                                .into_owned();
                            message = Some(decoded)
                        }
                        _ => {}
                    }
                } else {
                    return Err(DeserializeError::MissingEquals);
                }
            }
        }

        // "assetid" MUST be provided if "amount" is present
        // See https://github.com/ElementsProject/elements/issues/805#issuecomment-576743532
        if (amount.is_some() || amount_sat.is_some()) && asset_id.is_none() {
            return Err(DeserializeError::AssetNotProvided);
        }

        Ok(Self {
            address,
            network,
            asset_id,
            amount,
            amount_sat,
            label,
            message,
        })
    }

    pub fn from_addr(address: &str) -> Result<Self, DeserializeError> {
        let elements_address = address
            .parse::<Address>()
            .map_err(DeserializeError::InvalidAddress)?;

        let network = if elements_address.params.eq(&AddressParams::LIQUID) {
            Network::Bitcoin
        } else if elements_address.params.eq(&AddressParams::LIQUID_TESTNET) {
            Network::Testnet
        } else if elements_address.params.eq(&AddressParams::ELEMENTS) {
            Network::Regtest
        } else {
            return Err(DeserializeError::InvalidAddress(
                AddressError::InvalidAddress("The specified asset is not supported".to_string()),
            ));
        };

        Ok(Self {
            address: address.to_string(),
            network,
            asset_id: None,
            amount: None,
            amount_sat: None,
            label: None,
            message: None,
        })
    }
}

impl FromStr for LiquidAddressData {
    type Err = DeserializeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize_raw(s)
    }
}

impl TryFrom<&str> for LiquidAddressData {
    type Error = DeserializeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::deserialize_raw(s)
    }
}

impl TryFrom<String> for LiquidAddressData {
    type Error = DeserializeError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::deserialize_raw(&s)
    }
}
