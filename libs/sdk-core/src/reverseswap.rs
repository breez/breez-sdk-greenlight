use std::sync::Arc;

use crate::chain::{ChainService, MempoolSpace};
use anyhow::Result;
use bitcoin_hashes::{Hash, sha256};
use bitcoin_hashes::hex::ToHex;
use rand::random;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use crate::boltzswap::CREATE_REVERSE_SWAP_ENDPOINT;
use serde::{Deserialize, Serialize};

use crate::breez_services::BreezServer;
use crate::models::{ReverseSwap, ReverseSwapperAPI};
use crate::ReverseSwapInfo;

fn get_boltz_reverse_swap_args(amount_sat: u64, preimage_hash_hex: String, claim_pubkey: String) -> String {
    json!({
        "type": "reversesubmarine",
        "pairId": "BTC/BTC",
        "orderSide": "buy",
        "invoiceAmount": amount_sat,
        "preimageHash": preimage_hash_hex,
        "claimPublicKey": claim_pubkey
    }).to_string()
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateReverseSwapResponse {
    id: String,
    invoice: String,
    redeem_script: String,
    onchain_amount: u64,
    timeout_block_height: u64,
    lockup_address: String
}

#[tonic::async_trait]
impl ReverseSwapperAPI for BreezServer {
    async fn create_reverse_swap(&self, amount_sat: u64, onchain_claim_address: String) -> Result<ReverseSwap> {
        let rand_bytes : [u8; 32] = random();
        let preimage = sha256::Hash::hash(&rand_bytes);
        let preimage_hash = sha256::Hash::hash(&preimage);
        let preimage_hash_hex = preimage_hash.to_hex();

        // TODO Ensure onchain address is in compressed format

        let temp_res = Client::new()
            .post(CREATE_REVERSE_SWAP_ENDPOINT)
            .header(CONTENT_TYPE, "application/json")
            .body(get_boltz_reverse_swap_args(amount_sat, preimage_hash_hex.into(), onchain_claim_address))
            .send()
            .await?
            .text()
            .await?;
        info!("received: {temp_res}");
        let response : CreateReverseSwapResponse = serde_json::from_str(&temp_res)?;

        return Ok(ReverseSwap {
            error_message: None,
            response
        });
    }
}

/// This struct is responsible for sending to an onchain address using lightning payments.
/// It uses internally an implementation of ReverseSwapperAPI that represents Boltz reverse swapper service.
pub(crate) struct BTCSendSwap {
    network: bitcoin::Network,
    reverse_swapper_api: Arc<dyn ReverseSwapperAPI>,
    persister: Arc<crate::persist::db::SqliteStorage>,
    chain_service: Arc<dyn ChainService>,
    //payment_sender: Arc<dyn Sender>,
}

impl BTCSendSwap {
    pub(crate) fn new(
        network: bitcoin::Network,
        reverse_swapper_api: Arc<dyn ReverseSwapperAPI>,
        persister: Arc<crate::persist::db::SqliteStorage>,
        chain_service: Arc<MempoolSpace>,
        //payment_sender: Arc<PaymentSender>,
    ) -> Self {
        Self {
            network,
            reverse_swapper_api,
            persister,
            chain_service,
            //payment_sender,
        }
    }

    pub(crate) async fn create_reverse_swap(&self, amount_sat: u64, onchain_claim_address: String) -> Result<ReverseSwap> {
        self.reverse_swapper_api.create_reverse_swap(amount_sat, onchain_claim_address).await
    }
}
