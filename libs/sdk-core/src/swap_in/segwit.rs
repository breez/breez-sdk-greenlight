use gl_client::bitcoin::{Address, Transaction, TxIn, TxOut};

use crate::{SwapInfo, SwapStatus};

use super::{error::ReceiveSwapResult, swap::SwapOutput};

pub(super) struct SegwitReceiveSwap {}

impl SegwitReceiveSwap {
    pub fn new() -> Self {
        Self {}
    }

    pub fn payout_blocks_left(
        &self,
        swap_info: &SwapInfo,
        min_confirmation: u32,
        current_tip: u32,
    ) -> i32 {
        let confirmations = current_tip.saturating_sub(min_confirmation);
        swap_info.lock_height as i32 - confirmations as i32
    }

    pub fn create_fake_refund_tx(
        &self,
        swap_info: &SwapInfo,
        utxos: &[&SwapOutput],
        destination_address: Address,
    ) -> ReceiveSwapResult<Transaction> {
        Ok(Transaction {
            version: 2,
            lock_time: 0,
            input: utxos.iter().map(|utxo| {
                let mut tx_in  = utxo.try_into()?;
                tx_in.witness = vec![[1;73].to_vec(), Vec::new(), [1;33].to_vec()];
                Ok(tx_in)
        }).collect::<Result<_, _>>()?,
            output: TxOut {
                value: 0,
                script_pubkey: destination_address.script_pubkey(),
            },
        })
    }

    pub fn create_refund_tx(
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
