use gl_client::bitcoin::Transaction;

use crate::{SwapInfo, SwapStatus};

use super::{error::ReceiveSwapResult, swap::SwapOutput};

pub(super) struct TaprootReceiveSwap {}

impl TaprootReceiveSwap {
    pub fn new() -> Self {
        Self {}
    }

    pub fn payout_blocks_left(
        &self,
        swap_info: &SwapInfo,
        min_confirmation: u32,
        current_tip: u32,
    ) -> ReceiveSwapResult<SwapStatus> {
        todo!()
    }

    pub fn create_fake_cooperative_refund_tx(
        &self,
        swap_info: &SwapInfo,
        utxos: &[&SwapOutput],
    ) -> ReceiveSwapResult<Transaction> {
        todo!()
    }

    pub fn create_fake_unilateral_refund_tx(
        &self,
        swap_info: &SwapInfo,
        utxos: &[&SwapOutput],
    ) -> ReceiveSwapResult<Transaction> {
        todo!()
    }

    pub fn create_cooperative_refund_tx(
        &self,
        swap_info: &SwapInfo,
        utxos: &[&SwapOutput],
    ) -> ReceiveSwapResult<Transaction> {
        todo!()
    }

    pub fn create_unilateral_refund_tx(
        &self,
        swap_info: &SwapInfo,
        utxos: &[&SwapOutput],
    ) -> ReceiveSwapResult<Transaction> {
        todo!()
    }

    pub fn get_swap_payment(
        swap_info: &SwapInfo,
        payment_request: String,
    ) -> ReceiveSwapResult<()> {
        todo!()
    }
}
