use anyhow::anyhow;
use lightning::bitcoin::{self, address::NetworkUnchecked};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// The different supported bitcoin networks
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq, Serialize, Deserialize)]
pub enum Network {
    /// Mainnet
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

impl TryFrom<&bitcoin::Address<NetworkUnchecked>> for Network {
    type Error = anyhow::Error;

    fn try_from(address: &bitcoin::Address<NetworkUnchecked>) -> Result<Self, Self::Error> {
        let networks = vec![
            Network::Bitcoin,
            Network::Testnet,
            Network::Signet,
            Network::Regtest,
        ];
        for network in networks {
            if address.is_valid_for_network(network.into()) {
                return Ok(network);
            }
        }
        Err(anyhow!("Unknown network"))
    }
}

impl TryFrom<bitcoin::Network> for Network {
    type Error = anyhow::Error;

    fn try_from(network: bitcoin::Network) -> Result<Self, Self::Error> {
        match network {
            bitcoin::Network::Bitcoin => Ok(Network::Bitcoin),
            bitcoin::Network::Testnet => Ok(Network::Testnet),
            bitcoin::Network::Signet => Ok(Network::Signet),
            bitcoin::Network::Regtest => Ok(Network::Regtest),
            _ => Err(anyhow!("Unknown network")),
        }
    }
}

impl From<Network> for bitcoin::Network {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => bitcoin::Network::Bitcoin,
            Network::Testnet => bitcoin::Network::Testnet,
            Network::Signet => bitcoin::Network::Signet,
            Network::Regtest => bitcoin::Network::Regtest,
        }
    }
}
