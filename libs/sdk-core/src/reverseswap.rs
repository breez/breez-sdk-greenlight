use std::sync::Arc;

use crate::chain::{ChainService, MempoolSpace};
use anyhow::Result;

use crate::breez_services::BreezServer;
use crate::models::{ReverseSwap, ReverseSwapperAPI};

#[tonic::async_trait]
impl ReverseSwapperAPI for BreezServer {
    async fn create_reverse_swap(&self) -> Result<ReverseSwap> {
        let m = String::from("");
        return Ok(ReverseSwap { error_message: m });
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
}
