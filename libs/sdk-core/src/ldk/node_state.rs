use ldk_node::{Node, PendingSweepBalance};

use crate::NodeState;

impl From<&Node> for NodeState {
    fn from(node: &Node) -> Self {
        const MAX_PAYMENT_AMOUNT_MSAT: u64 = 4_294_967_000;

        let balances = node.list_balances();
        let pending_onchain_balance_sats: u64 = balances
            .pending_balances_from_channel_closures
            .iter()
            .map(get_balance)
            .sum();

        let connected_peers = node
            .list_peers()
            .iter()
            .filter(|p| p.is_connected)
            .map(|p| p.node_id.to_string())
            .collect();

        let channels = node.list_channels();
        let max_payable_msat = channels
            .iter()
            .map(|c| c.next_outbound_htlc_limit_msat)
            .sum();
        let max_chan_reserve_sats: u64 = channels
            .iter()
            .flat_map(|c| c.unspendable_punishment_reserve)
            .sum();
        let inbound_capacity_msats = channels.iter().map(|c| c.inbound_capacity_msat).sum();

        Self {
            id: node.node_id().to_string(),
            block_height: node.status().current_best_block.height,
            channels_balance_msat: balances.total_lightning_balance_sats * 1000,
            onchain_balance_msat: balances.total_onchain_balance_sats * 1000,
            pending_onchain_balance_msat: pending_onchain_balance_sats * 1000,
            utxos: Vec::new(), // Not available in LDK Node.
            max_payable_msat,
            max_receivable_msat: MAX_PAYMENT_AMOUNT_MSAT,
            max_single_payment_amount_msat: MAX_PAYMENT_AMOUNT_MSAT,
            max_chan_reserve_msats: max_chan_reserve_sats * 1000,
            connected_peers,
            // TODO: Calculate a better approximation.
            max_receivable_single_payment_amount_msat: inbound_capacity_msats,
            total_inbound_liquidity_msats: inbound_capacity_msats,
        }
    }
}

fn get_balance(balance: &PendingSweepBalance) -> u64 {
    match balance {
        PendingSweepBalance::PendingBroadcast {
            amount_satoshis, ..
        }
        | PendingSweepBalance::BroadcastAwaitingConfirmation {
            amount_satoshis, ..
        }
        | PendingSweepBalance::AwaitingThresholdConfirmations {
            amount_satoshis, ..
        } => *amount_satoshis,
    }
}
