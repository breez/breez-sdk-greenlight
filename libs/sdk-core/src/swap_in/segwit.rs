use std::sync::Arc;

use gl_client::bitcoin::{
    blockdata::{opcodes, script::Builder},
    consensus::serialize,
    secp256k1::{Message, Secp256k1, SecretKey},
    util::sighash::SighashCache,
    Address, EcdsaSighashType, PackedLockTime, Script, Transaction, TxIn, TxOut, Witness,
};
use ripemd::{Digest, Ripemd160};

use crate::{SwapInfo, SwapperAPI};

use super::{
    error::{ReceiveSwapError, ReceiveSwapResult},
    swap::{compute_tx_fee, SwapOutput},
};

const MAX_ECDSA_SIGNATURE_SIZE: usize = 73;
const SEGWIT_SWAP_SCRIPT_SIZE: usize = 100;

pub(super) struct SegwitReceiveSwap {
    swapper_api: Arc<dyn SwapperAPI>,
}

impl SegwitReceiveSwap {
    pub fn new(swapper_api: Arc<dyn SwapperAPI>) -> Self {
        Self { swapper_api }
    }

    pub fn payout_blocks_left(
        &self,
        swap_info: &SwapInfo,
        min_confirmation: u32,
        current_tip: u32,
    ) -> u32 {
        let confirmations = current_tip.saturating_sub(min_confirmation);
        (swap_info.lock_height as u32).saturating_sub(confirmations)
    }

    pub fn create_fake_refund_tx(
        &self,
        _swap_info: &SwapInfo,
        utxos: &[SwapOutput],
        destination_address: &Address,
    ) -> ReceiveSwapResult<Transaction> {
        Ok(Transaction {
            version: 2,
            lock_time: PackedLockTime::ZERO,
            input: utxos
                .iter()
                .map(|utxo| {
                    Ok(TxIn {
                        witness: Witness::from_vec(vec![
                            [1; MAX_ECDSA_SIGNATURE_SIZE].to_vec(),
                            Vec::new(),
                            [1; SEGWIT_SWAP_SCRIPT_SIZE].to_vec(),
                        ]),
                        ..utxo.try_into()?
                    })
                })
                .collect::<Result<_, ReceiveSwapError>>()?,
            output: vec![TxOut {
                value: 0,
                script_pubkey: destination_address.script_pubkey(),
            }],
        })
    }

    pub fn create_refund_tx(
        &self,
        swap_info: &SwapInfo,
        utxos: &[SwapOutput],
        destination_address: &Address,
        sat_per_vbyte: u32,
    ) -> ReceiveSwapResult<Transaction> {
        let weight = self
            .create_fake_refund_tx(swap_info, utxos, destination_address)?
            .weight();
        let fee = compute_tx_fee(weight, sat_per_vbyte);
        let value: u64 = utxos
            .iter()
            .map(|utxo| utxo.amount_sat)
            .sum::<u64>()
            .saturating_sub(fee);
        if value == 0 {
            return Err(ReceiveSwapError::OutputValueBelowDust);
        }

        let lock_time = utxos.iter().fold(0, |accum, item| {
            let confirmed_height = item.confirmed_at_height.unwrap_or(0);
            if accum >= confirmed_height + (swap_info.lock_height as u32) {
                accum
            } else {
                confirmed_height + (swap_info.lock_height as u32)
            }
        });

        let input_script = create_submarine_swap_script(
            &swap_info.payment_hash,
            &swap_info.swapper_public_key,
            &swap_info.public_key,
            swap_info.lock_height,
        )?;

        let mut tx = Transaction {
            version: 2,
            lock_time: PackedLockTime(lock_time),
            input: utxos
                .iter()
                .map(|utxo| utxo.try_into())
                .collect::<Result<_, _>>()?,
            output: vec![TxOut {
                value,
                script_pubkey: destination_address.script_pubkey(),
            }],
        };

        let scpt = Secp256k1::signing_only();
        let cloned_tx = tx.clone();
        let mut signer = SighashCache::new(&cloned_tx);
        for (input_index, input) in tx.input.iter_mut().enumerate() {
            let sig = signer.segwit_signature_hash(
                input_index,
                &input_script,
                utxos[input_index].amount_sat,
                EcdsaSighashType::All,
            )?;
            let msg = Message::from_slice(&sig[..])?;
            let secret_key = SecretKey::from_slice(&swap_info.private_key)?;
            let sig = scpt.sign_ecdsa(&msg, &secret_key);

            let mut sigvec = sig.serialize_der().to_vec();
            sigvec.push(EcdsaSighashType::All as u8);

            let witness: Vec<Vec<u8>> = vec![sigvec, vec![], serialize(&input_script)];

            let w = Witness::from_vec(witness);
            input.witness = w;
        }

        Ok(tx)
    }

    pub async fn get_swap_payment(&self, payment_request: String) -> ReceiveSwapResult<()> {
        self.swapper_api
            .complete_swap(payment_request)
            .await
            .map_err(|e| ReceiveSwapError::PaymentError(e.to_string()))
    }
}

fn create_submarine_swap_script(
    payment_hash: &[u8],
    swapper_pub_key: &[u8],
    payer_pub_key: &[u8],
    lock_height: i64,
) -> anyhow::Result<Script> {
    let mut hasher = Ripemd160::new();
    hasher.update(payment_hash);
    let result = hasher.finalize();

    Ok(Builder::new()
        .push_opcode(opcodes::all::OP_HASH160)
        .push_slice(&result)
        .push_opcode(opcodes::all::OP_EQUAL)
        .push_opcode(opcodes::all::OP_IF)
        .push_slice(swapper_pub_key)
        .push_opcode(opcodes::all::OP_ELSE)
        .push_int(lock_height)
        .push_opcode(opcodes::all::OP_CSV)
        .push_opcode(opcodes::all::OP_DROP)
        .push_slice(payer_pub_key)
        .push_opcode(opcodes::all::OP_ENDIF)
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .into_script())
}
