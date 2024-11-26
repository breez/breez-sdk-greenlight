use anyhow::Result;
use bitcoin::hashes::hex::FromHex;
use serde::{Deserialize, Serialize};

use crate::bitcoin::{OutPoint, Txid};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OnchainTx {
    pub txid: String,
    pub version: u32,
    pub locktime: u32,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub size: u32,
    pub weight: u32,
    pub fee: u32,
    pub status: TxStatus,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TxStatus {
    pub confirmed: bool,
    pub block_height: Option<u32>,
    pub block_hash: Option<String>,
    pub block_time: Option<u64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Vout {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: String,
    pub value: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Vin {
    pub txid: String,
    pub vout: u32,
    pub prevout: Vout,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    pub witness: Option<Vec<String>>,
    pub is_coinbase: bool,
    pub sequence: u32,
}

#[derive(Clone)]
pub struct Utxo {
    pub out: OutPoint,
    pub value: u64,
    pub block_height: Option<u32>,
}

#[derive(Clone)]
pub struct AddressUtxos {
    pub unconfirmed: Vec<Utxo>,
    pub confirmed: Vec<Utxo>,
}

impl AddressUtxos {
    pub fn unconfirmed_sats(&self) -> u64 {
        self.unconfirmed
            .iter()
            .fold(0, |accum, item| accum + item.value)
    }

    pub fn unconfirmed_tx_ids(&self) -> Vec<String> {
        self.unconfirmed
            .iter()
            .map(|c| c.out.txid.to_string())
            .collect()
    }

    pub fn confirmed_sats(&self) -> u64 {
        self.confirmed
            .iter()
            .fold(0, |accum, item| accum + item.value)
    }

    pub fn confirmed_tx_ids(&self) -> Vec<String> {
        self.confirmed
            .iter()
            .map(|c| c.out.txid.to_string())
            .collect()
    }

    /// Get the highest block height of all confirmed transactions that paid to the given onchain address
    pub fn _confirmed_block(&self) -> u32 {
        self.confirmed.iter().fold(0, |b, item| {
            let confirmed_block = item.block_height.unwrap_or_default();
            if confirmed_block != 0 || confirmed_block < b {
                confirmed_block
            } else {
                b
            }
        })
    }
}

/// Gets unspent tx outputs. Specifically filters out inbound utxos that have been spent.
/// If include_unconfirmed_spends is true, then the result won't include utxos that were spent
/// in unconfirmed transactions.
pub fn get_utxos(
    address: String,
    transactions: Vec<OnchainTx>,
    include_unconfirmed_spends: bool,
) -> Result<AddressUtxos> {
    let mut spent_outputs: Vec<OutPoint> = Vec::new();
    let mut utxos: Vec<Utxo> = Vec::new();
    for tx in transactions.iter() {
        for vin in tx.vin.iter() {
            if vin.prevout.scriptpubkey_address == address.clone()
                && (include_unconfirmed_spends || tx.status.confirmed)
            {
                spent_outputs.push(OutPoint {
                    txid: Txid::from_hex(vin.txid.as_str())?,
                    vout: vin.vout,
                })
            }
        }
    }

    for tx in transactions.iter() {
        for (index, vout) in tx.vout.iter().enumerate() {
            if vout.scriptpubkey_address == address {
                let outpoint = OutPoint {
                    txid: Txid::from_hex(tx.txid.as_str())?,
                    vout: index as u32,
                };
                if !spent_outputs.contains(&outpoint) {
                    utxos.push(Utxo {
                        out: outpoint,
                        value: vout.value,
                        block_height: tx.status.block_height,
                    });
                }
            }
        }
    }
    let address_utxos = AddressUtxos {
        unconfirmed: utxos
            .clone()
            .into_iter()
            .filter(|u| u.block_height.is_none())
            .collect(),
        confirmed: utxos
            .clone()
            .into_iter()
            .filter(|u| u.block_height.is_some())
            .collect(),
    };
    Ok(address_utxos)
}
