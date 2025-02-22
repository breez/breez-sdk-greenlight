use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// The different supported bitcoin networks
#[sdk_macros::tsify_wasm]
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq, Serialize, Deserialize)]
pub enum Network {
    /// Mainnet
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

impl From<bitcoin::network::constants::Network> for Network {
    fn from(network: bitcoin::network::constants::Network) -> Self {
        match network {
            bitcoin::network::constants::Network::Bitcoin => Network::Bitcoin,
            bitcoin::network::constants::Network::Testnet => Network::Testnet,
            bitcoin::network::constants::Network::Signet => Network::Signet,
            bitcoin::network::constants::Network::Regtest => Network::Regtest,
        }
    }
}

impl From<Network> for bitcoin::network::constants::Network {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => bitcoin::network::constants::Network::Bitcoin,
            Network::Testnet => bitcoin::network::constants::Network::Testnet,
            Network::Signet => bitcoin::network::constants::Network::Signet,
            Network::Regtest => bitcoin::network::constants::Network::Regtest,
        }
    }
}
