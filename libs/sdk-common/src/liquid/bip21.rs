use elements::{
    address::{Address, AddressError, AddressParams},
    hashes::hex::HexToArrayError,
    issuance::AssetId,
};
use serde::Serialize;
use std::{num::ParseFloatError, str::FromStr, string::FromUtf8Error};
use urlencoding::decode;

use crate::{
    invoice::{parse_invoice, InvoiceError},
    prelude::Network,
};

#[derive(Debug, Clone, Serialize)]
pub struct LiquidAddressData {
    pub address: String,
    pub network: Network,
    pub asset_id: Option<String>,
    pub amount_sat: Option<u64>,
    pub label: Option<String>,
    pub message: Option<String>,
    pub invoice: Option<String>,
}

#[derive(Debug)]
pub enum DeserializeError {
    InvalidScheme,
    MissingEquals,
    UnknownParameter,
    AssetNotProvided,
    InvalidString(FromUtf8Error),
    InvalidAmount(ParseFloatError),
    InvalidAsset(HexToArrayError),
    InvalidAddress(AddressError),
    InvalidInvoice(InvoiceError),
}

impl LiquidAddressData {
    fn deserialize_raw(string: &str) -> Result<Self, DeserializeError> {
        let (network, address_params) = string
            .split_once(':')
            .ok_or(DeserializeError::InvalidScheme)?;

        let network = match network {
            "liquidnetwork" => Network::Bitcoin,
            "liquidtestnet" => Network::Testnet,
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

        let mut amount_sat = None;
        let mut asset_id = None;
        let mut label = None;
        let mut message = None;
        let mut invoice = None;
        if let Some(params) = address_params.next() {
            for pair in params.split('&') {
                if let Some((key, val)) = pair.split_once('=') {
                    match key {
                        "amount" => {
                            let parsed_amount = val
                                .parse::<f64>()
                                .map_err(DeserializeError::InvalidAmount)?;
                            amount_sat = Some((parsed_amount * 100_000_000.0) as u64);
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
                        "lightning" => {
                            parse_invoice(val).map_err(DeserializeError::InvalidInvoice)?;
                            invoice = Some(val.to_string());
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
        if amount_sat.is_some() && asset_id.is_none() {
            return Err(DeserializeError::AssetNotProvided);
        }

        Ok(Self {
            address,
            network,
            asset_id,
            amount_sat,
            label,
            message,
            invoice,
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
        } else {
            return Err(DeserializeError::InvalidAddress(
                AddressError::InvalidAddress("The specified asset is not supported".to_string()),
            ));
        };

        Ok(Self {
            address: address.to_string(),
            network,
            asset_id: None,
            amount_sat: None,
            label: None,
            message: None,
            invoice: None,
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
