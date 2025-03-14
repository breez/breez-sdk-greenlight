use std::sync::Arc;

use gl_client::bitcoin::{blockdata::{opcodes, script::Builder}, secp256k1::{Message, Secp256k1, SecretKey}, util::sighash::SighashCache, Address, EcdsaSighashType, PackedLockTime, Script, Transaction, TxIn, TxOut, Witness};
use ripemd::Ripemd160;
use sdk_common::{grpc::GetSwapPaymentRequest, prelude::BreezServer, with_connection_retry};

use crate::{SegwitSwapperAPI, SwapInfo};

use super::{error::{ReceiveSwapError, ReceiveSwapResult}, swap::{compute_tx_fee, SwapOutput}};

pub(super) struct SegwitReceiveSwap {
    swapper_api: Arc<dyn SegwitSwapperAPI>
}

impl SegwitReceiveSwap {
    pub fn new(swapper_api: Arc<dyn SegwitSwapperAPI>) -> Self {
        Self { swapper_api }
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
        destination_address: &Address,
    ) -> ReceiveSwapResult<Transaction> {
        Ok(Transaction {
            version: 2,
            lock_time: 0,
            input: utxos
                .iter()
                .map(|utxo| {
                    Ok(TxIn {
                        witness: vec![[1; 73].to_vec(), Vec::new(), [1; 100].to_vec()],
                        ..utxo.try_into()?
                    })
                })
                .collect::<Result<_, _>>()?,
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
        destination_address: &Address,
        sat_per_vbyte: u32,
    ) -> ReceiveSwapResult<Transaction> {
        let weight = self.create_fake_refund_tx(swap_info, utxos, destination_address)?.weight();
        let fee = compute_tx_fee(weight, sat_per_vbyte);
        let value: u64 = utxos.iter().map(|utxo| utxo.amount_sat).sum().saturating_sub(fee);
        if value == 0 {
            return Err(ReceiveSwapError::OutputValueBelowDust);
        }

        let lock_time = utxos.iter().fold(0, |accum, item| {
            let confirmed_height = item.confirmed_at_height.unwrap_or(0);
            if accum >= confirmed_height + swap_info.lock_height {
                accum
            } else {
                confirmed_height + swap_info.lock_height
            }
        });

        let input_script = create_submarine_swap_script(
            swap_info.payment_hash,
            swap_info.swapper_public_key,
            swap_info.public_key,
            swap_info.lock_height,
        )?;

        let mut tx = Transaction {
            version: 2,
            lock_time: PackedLockTime::from(lock_time),
            input: utxos
                .iter()
                .map(|utxo| utxo.try_into())
                .collect::<Result<_, _>>()?,
            output: TxOut {
                value,
                script_pubkey: destination_address.script_pubkey(),
            },
        };

        let scpt = Secp256k1::signing_only();
        let mut signer = SighashCache::new(&tx);
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
    
            let witness: Vec<Vec<u8>> = vec![sigvec, vec![], input_script.serialize()];
    
            let mut signed_input = input.clone();
            let w = Witness::from_vec(witness);
            input.witness = w;
        }

        Ok(tx)
    }

    pub async fn get_swap_payment(
        &self,
        swap_info: &SwapInfo,
        payment_request: String,
    ) -> ReceiveSwapResult<()> {
        self.swapper_api.complete_swap(payment_request).await
            .map_err(|e| ReceiveSwapError::PaymentError(e.to_string()))
    }
}

#[tonic::async_trait]
impl SegwitSwapperAPI for BreezServer {
    async fn complete_swap(&self, bolt11: String) -> anyhow::Result<()> {
        let mut client = self.get_swapper_client().await;
        let req = GetSwapPaymentRequest {
            payment_request: bolt11,
        };
        let resp = with_connection_retry!(client.get_swap_payment(req.clone()))
            .await?
            .into_inner();

        match resp.swap_error() {
            crate::grpc::get_swap_payment_reply::SwapError::NoError => Ok(()),
            err => Err(anyhow::anyhow!(err.as_str_name())),
        }
    }
}

fn create_submarine_swap_script(
    invoice_hash: Vec<u8>,
    swapper_pub_key: Vec<u8>,
    payer_pub_key: Vec<u8>,
    lock_height: i64,
) -> anyhow::Result<Script> {
    let mut hasher = Ripemd160::new();
    hasher.update(invoice_hash);
    let result = hasher.finalize();

    Ok(Builder::new()
        .push_opcode(opcodes::all::OP_HASH160)
        .push_slice(&result[..])
        .push_opcode(opcodes::all::OP_EQUAL)
        .push_opcode(opcodes::all::OP_IF)
        .push_slice(&swapper_pub_key[..])
        .push_opcode(opcodes::all::OP_ELSE)
        .push_int(lock_height)
        .push_opcode(opcodes::all::OP_CSV)
        .push_opcode(opcodes::all::OP_DROP)
        .push_slice(&payer_pub_key[..])
        .push_opcode(opcodes::all::OP_ENDIF)
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .into_script())
}