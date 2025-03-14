use std::{collections::HashMap, sync::Arc};

use crate::{
    chain::ChainService,
    error::{SdkError, SdkResult},
    persist::db::SqliteStorage,
    ListPaymentsRequest, Payment, PaymentDetails,
};

pub(crate) struct PaymentService {
    chain: Arc<dyn ChainService>,
    persister: Arc<SqliteStorage>,
}

impl PaymentService {
    pub(crate) fn new(chain: Arc<dyn ChainService>, persister: Arc<SqliteStorage>) -> Self {
        Self { chain, persister }
    }

    pub(crate) async fn get_payment_by_hash(&self, hash: &String) -> SdkResult<Option<Payment>> {
        Ok(self.persister.get_payment_by_hash(hash)?)
    }

    pub(crate) async fn list_payments(&self, req: ListPaymentsRequest) -> SdkResult<Vec<Payment>> {
        let mut payments = self.persister.list_payments(req)?;
        let hashes: Vec<_> = payments.iter().map(|p| p.id.clone()).collect();
        let taproot_swaps = self.persister.list_taproot_swaps_by_hash(&hashes)?;
        if taproot_swaps.is_empty() {
            return Ok(payments);
        }

        let current_tip = self.chain.current_tip(true).await?;
        let node_state = self.persister.get_node_state()?.ok_or(SdkError::Generic {
            err: "Node info not found".into(),
        })?;
        let taproot_swaps: HashMap<_, _> = taproot_swaps
            .into_iter()
            .map(|s| (hex::encode(&s.swap.payment_hash), s))
            .collect();
        for p in &mut payments {
            if let PaymentDetails::Ln { data } = &mut p.details {
                if let Some(swap) = taproot_swaps.get(&p.id) {
                    data.swap_info = Some(swap.to_swap_info(&node_state, current_tip))
                }
            }
        }
        Ok(payments)
    }
}
