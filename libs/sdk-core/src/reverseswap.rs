use std::sync::Arc;

use crate::chain::{ChainService, MempoolSpace};
use crate::models::{ReverseSwap, ReverseSwapperAPI};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateReverseSwapResponse {
    id: String,
    invoice: String,
    redeem_script: String,
    onchain_amount: u64,
    timeout_block_height: u64,
    lockup_address: String,
}

/// This struct is responsible for sending to an onchain address using lightning payments.
/// It uses internally an implementation of [ReverseSwapperAPI] that represents Boltz reverse swapper service.
pub(crate) struct BTCSendSwap {
    network: bitcoin::Network,
    pub(crate) reverse_swapper_api: Arc<dyn ReverseSwapperAPI>,
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

    pub(crate) async fn create_reverse_swap(
        &self,
        amount_sat: u64,
        onchain_claim_address: String,
        pair_hash: String,
        routing_node: String,
    ) -> Result<ReverseSwap> {
        self.reverse_swapper_api
            .create_reverse_swap(amount_sat, onchain_claim_address, pair_hash, routing_node)
            .await
    }
}
