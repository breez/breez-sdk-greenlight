use std::sync::Arc;

use ldk_node::{Event, Node};
use tokio::sync::{broadcast, mpsc};

use crate::ldk::node_api::PreimageStore;
use crate::node_api::IncomingPayment;

pub async fn start_event_handling(
    node: Arc<Node>,
    preimages: PreimageStore,
    incoming_payments_tx: broadcast::Sender<IncomingPayment>,
    mut shutdown: mpsc::Receiver<()>,
) {
    loop {
        let event = tokio::select! {
            event = node.next_event_async() => event,
            _ = shutdown.recv() => {
                info!("Received shutdown signal, stopping event handling loop");
                return;
            },
        };
        debug!("Event: {event:?}");

        match event {
            Event::PaymentReceived {
                payment_id,
                payment_hash,
                amount_msat,
                ..
            } => {
                let preimage = preimages.lock().unwrap().remove(&payment_hash);
                match preimage {
                    Some(preimage) => {
                        // TODO: Load bolt11 from the store.
                        let bolt11 = String::new();
                        let payment = IncomingPayment {
                            label: String::new(),
                            payment_hash: payment_hash.0.to_vec(),
                            preimage: preimage.0.to_vec(),
                            amount_msat,
                            bolt11,
                        };
                        if let Err(e) = incoming_payments_tx.send(payment) {
                            warn!("Failed to send payment to incoming_payments_tx: {e}");
                        }
                    }
                    None => {
                        error!("Payment received but preimage not found for payment with id={payment_id:?}");
                    }
                }
            }
            Event::PaymentSuccessful { .. } => (),
            Event::PaymentFailed { .. } => (),
            Event::PaymentClaimable {
                payment_id,
                payment_hash,
                claimable_amount_msat,
                ..
            } => {
                let preimage = preimages.lock().unwrap().get(&payment_hash).cloned();
                match preimage {
                    Some(preimage) => {
                        if let Err(e) = node.bolt11_payment().claim_for_hash(
                            payment_hash,
                            claimable_amount_msat,
                            preimage,
                        ) {
                            error!("Failed to claim payment: {e}");
                        }
                    }
                    None => {
                        error!("Payment claimable but preimage not found for payment with id={payment_id:?}");
                        if let Err(e) = node.bolt11_payment().fail_for_hash(payment_hash) {
                            error!("Failed to fail payment: {e}");
                        }
                    }
                }
            }
            Event::PaymentForwarded { .. } => (),
            Event::ChannelPending { .. } => (),
            Event::ChannelReady { .. } => (),
            Event::ChannelClosed { .. } => (),
        }

        if let Err(e) = node.event_handled() {
            error!("Failed to report that event was handled: {e}");
        }
    }
}
