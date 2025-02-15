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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)] // <-- Added PartialEq here
pub enum PaymentStatus {
    Pending,
    Processing,
    Complete,
    Failed,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub payment_hash: String,
    pub id: String,
    pub amount_msat: u64,
    pub status: PaymentStatus,
    // ... other fields
}

// Tests for the Payment model
#[cfg(test)]
mod tests {
    use super::*; // Import Payment from parent module

    #[test]
    fn payment_hash_verification() {
        let payment = Payment {
            payment_hash: "test_hash".to_string(),
            id: "test_id".to_string(),
            amount_msat: 1000,
            status: PaymentStatus::Complete,
            // ... initialize other fields
        };

        assert!(!payment.payment_hash.is_empty());
    }

    #[test]
    fn payment_status_serialization() {
       let payment = Payment {
            payment_hash: "test_hash".to_string(),
            id: "test_id".to_string(),
            amount_msat: 1000,
            status: PaymentStatus::Complete,
        };

        let json = serde_json::to_string(&payment).unwrap();
        assert!(json.contains("\"status\":\"Complete\""));

        let payment_back: Payment = serde_json::from_str(&json).unwrap();
        assert_eq!(payment.status, payment_back.status);
    }

    #[test]
    fn payment_status_equality() {
        let status1 = PaymentStatus::Complete;
        let status2 = PaymentStatus::Complete;
        let status3 = PaymentStatus::Failed;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }
}