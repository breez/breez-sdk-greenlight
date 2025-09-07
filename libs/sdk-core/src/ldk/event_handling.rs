use std::sync::Arc;

use ldk_node::lightning::ln::channelmanager::PaymentId;
use ldk_node::payment::PaymentKind;
use ldk_node::{Event, Node};
use tokio::sync::{broadcast, mpsc};

use crate::node_api::IncomingPayment;

pub async fn start_event_handling(
    node: Arc<Node>,
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
            Event::PaymentReceived { payment_id, .. } => {
                match handle_received_payment(&node, payment_id) {
                    Ok(Some(payment)) => {
                        if let Err(e) = incoming_payments_tx.send(payment) {
                            warn!("Failed to send payment to incoming_payments_tx: {e}");
                        }
                    }
                    Ok(None) => (),
                    Err(e) => {
                        error!("Failed to handle PaymentReceived event for payment with id={payment_id:?}: {e}")
                    }
                };
            }
            Event::PaymentSuccessful { .. } => (),
            Event::PaymentFailed { .. } => (),
            Event::PaymentClaimable { .. } => (),
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

fn handle_received_payment(
    node: &Node,
    payment_id: Option<PaymentId>,
) -> Result<Option<IncomingPayment>, &'static str> {
    let payment_details = node
        .list_payments_with_filter(|p| Some(p.id) == payment_id)
        .into_iter()
        .next()
        .ok_or("Payment not found")?;

    let (hash, preimage) = match payment_details.kind {
        PaymentKind::Bolt11 { hash, preimage, .. }
        | PaymentKind::Bolt11Jit { hash, preimage, .. } => (hash, preimage),
        _ => return Ok(None),
    };
    let preimage = preimage.ok_or("Empty preimage")?;
    let amount_msat = payment_details.amount_msat.ok_or("Empty amount")?;
    // TODO: Load bolt11 from the store.
    let bolt11 = String::new();
    Ok(Some(IncomingPayment {
        label: String::new(),
        payment_hash: hash.0.to_vec(),
        preimage: preimage.0.to_vec(),
        amount_msat,
        bolt11,
    }))
}
