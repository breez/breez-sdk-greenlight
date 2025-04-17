use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use gl_client::bitcoin::{
    self,
    blockdata::{
        opcodes::all::{OP_CHECKSIG, OP_CHECKSIGVERIFY, OP_CSV, OP_EQUALVERIFY, OP_HASH160},
        script,
    },
    consensus::serialize,
    hashes::{ripemd160, Hash},
    secp256k1::{Message, PublicKey, SecretKey},
    util::{
        sighash::{Prevouts, SighashCache},
        taproot::{LeafVersion, TapLeafHash, TaprootBuilder, TaprootSpendInfo},
    },
    Address, Network, PackedLockTime, SchnorrSighashType, Script, Sequence, Transaction, TxIn,
    TxOut, Witness, XOnlyPublicKey,
};
use rand::Rng;
use sdk_common::tonic_wrap;
use secp256k1::musig::{
    MusigAggNonce, MusigKeyAggCache, MusigPartialSignature, MusigPubNonce, MusigSecRand,
    MusigSession,
};

use crate::{NodeState, OpeningFeeParams, SwapInfo, SwapStatus};

use super::{
    error::{ReceiveSwapError, ReceiveSwapResult},
    swap::{compute_tx_fee, create_swap_keys, SwapOutput},
    taproot_server::TaprootSwapperAPI,
};

const SCHNORR_SIGNATURE_SIZE: usize = 64;
const TAPROOT_REFUND_SCRIPT_SIZE: usize = 65;
const TAPROOT_CONTROL_BLOCK_SIZE: usize = 37;
const PAYOUT_VALIDITY_BLOCKS: u32 = 360;

pub(super) struct TaprootReceiveSwap {
    musig_secp: secp256k1::Secp256k1<secp256k1::All>,
    network: Network,
    secp: bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>,
    swapper_api: Arc<dyn TaprootSwapperAPI>,
}

impl TaprootReceiveSwap {
    pub fn new(network: Network, swapper_api: Arc<dyn TaprootSwapperAPI>) -> Self {
        Self {
            musig_secp: secp256k1::Secp256k1::new(),
            network,
            secp: bitcoin::secp256k1::Secp256k1::new(),
            swapper_api,
        }
    }

    pub async fn create_swap(
        &self,
        node_state: &NodeState,
        opening_fee_params: OpeningFeeParams,
    ) -> ReceiveSwapResult<SwapInfo> {
        let keys = create_swap_keys()?;
        let refund_pubkey = keys.public_key()?;
        let payment_hash = keys.preimage_hash_bytes();
        let resp = self
            .swapper_api
            .create_swap(payment_hash.clone(), refund_pubkey.serialize().to_vec())
            .await?;

        let claim_pubkey = PublicKey::from_slice(&resp.claim_pubkey)
            .map_err(|_| ReceiveSwapError::generic("Received invalid claim pubkey from server"))?;
        let (x_only_claim_pubkey, _) = claim_pubkey.x_only_public_key();
        let (x_only_refund_pubkey, _) = refund_pubkey.x_only_public_key();
        let claim_script = claim_script(&x_only_claim_pubkey, &payment_hash);
        let refund_script = refund_script(&x_only_refund_pubkey, resp.lock_time);

        let taproot_spend_info = self.taproot_spend_info(
            &claim_pubkey.serialize(),
            &refund_pubkey.serialize(),
            claim_script,
            refund_script.clone(),
        )?;
        let expected_address =
            Address::p2tr_tweaked(taproot_spend_info.output_key(), self.network).to_string();
        if resp.address != expected_address {
            return Err(ReceiveSwapError::generic(
                "Received invalid taproot swap address from server",
            ));
        }

        let parameters = match resp.parameters {
            Some(parameters) => parameters,
            None => {
                return Err(ReceiveSwapError::generic(
                    "missing parameters in create_swap response",
                ))
            }
        };

        let swap_info = SwapInfo {
            bitcoin_address: resp.address,
            bolt11: None,
            channel_opening_fees: Some(opening_fee_params),
            confirmed_at: None,
            confirmed_sats: 0,
            confirmed_tx_ids: Vec::new(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
            last_redeem_error: None,
            lock_height: resp.lock_time as i64,
            max_allowed_deposit: std::cmp::min(
                node_state.max_receivable_msat / 1000,
                parameters.max_swap_amount_sat,
            ) as i64,
            max_swapper_payable: parameters.max_swap_amount_sat as i64,
            min_allowed_deposit: parameters.min_swap_amount_sat as i64,
            paid_msat: 0,
            preimage: keys.preimage,
            private_key: keys.priv_key,
            public_key: refund_pubkey.serialize().to_vec(),
            refund_tx_ids: Vec::new(),
            script: refund_script.to_bytes(),
            payment_hash,
            status: SwapStatus::Initial,
            swapper_public_key: resp.claim_pubkey,
            total_incoming_txs: 0,
            unconfirmed_sats: 0,
            unconfirmed_tx_ids: Vec::new(),
        };

        Ok(swap_info)
    }

    pub fn payout_blocks_left(
        &self,
        swap_info: &SwapInfo,
        min_confirmation: u32,
        current_tip: u32,
    ) -> u32 {
        let confirmations = current_tip.saturating_sub(min_confirmation);
        PAYOUT_VALIDITY_BLOCKS
            .min(swap_info.lock_height as u32)
            .saturating_sub(confirmations)
    }

    pub fn create_fake_cooperative_refund_tx(
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
                        witness: Witness::from_vec(vec![[1; SCHNORR_SIGNATURE_SIZE].to_vec()]),
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

    pub fn create_fake_unilateral_refund_tx(
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
                            [1; SCHNORR_SIGNATURE_SIZE].to_vec(),
                            [1; TAPROOT_REFUND_SCRIPT_SIZE].to_vec(),
                            [1; TAPROOT_CONTROL_BLOCK_SIZE].to_vec(),
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

    pub async fn create_cooperative_refund_tx(
        &self,
        swap_info: &SwapInfo,
        utxos: &[SwapOutput],
        destination_address: &Address,
        sat_per_vbyte: u32,
    ) -> ReceiveSwapResult<Transaction> {
        let weight = self
            .create_fake_cooperative_refund_tx(swap_info, utxos, destination_address)?
            .weight();
        let fee = compute_tx_fee(weight, sat_per_vbyte);
        let value: u64 = utxos
            .iter()
            .map(|utxo| utxo.amount_sat)
            .sum::<u64>()
            .saturating_sub(fee);
        let mut tx = Transaction {
            version: 2,
            lock_time: PackedLockTime::ZERO,
            input: utxos
                .iter()
                .map(|utxo| utxo.try_into())
                .collect::<Result<_, _>>()?,
            output: vec![TxOut {
                value,
                script_pubkey: destination_address.script_pubkey(),
            }],
        };

        let swap_address: Address = swap_info.bitcoin_address.parse()?;
        let swap_address_script_pubkey = swap_address.script_pubkey();
        let claim_pubkey = PublicKey::from_slice(&swap_info.swapper_public_key)
            .map_err(|_| ReceiveSwapError::generic("invalid claim pubkey"))?;
        let refund_privkey = SecretKey::from_slice(&swap_info.private_key)
            .map_err(|_| ReceiveSwapError::generic("invalid refund private key"))?;
        let refund_pubkey = refund_privkey.public_key(&self.secp);
        let secp_refund_privkey = secp256k1::SecretKey::from_slice(&swap_info.private_key)
            .map_err(|_| ReceiveSwapError::generic("invalid refund private key"))?;
        let secp_refund_pubkey = secp_refund_privkey.public_key(&self.musig_secp);
        let (x_only_claim_pubkey, _) = claim_pubkey.x_only_public_key();
        let (x_only_refund_pubkey, _) = refund_pubkey.x_only_public_key();
        let claim_script = claim_script(&x_only_claim_pubkey, &swap_info.payment_hash);
        let refund_script = refund_script(&x_only_refund_pubkey, swap_info.lock_height as u32);
        let tweak = self
            .taproot_spend_info(
                &swap_info.swapper_public_key,
                &swap_info.public_key,
                claim_script,
                refund_script,
            )?
            .tap_tweak();
        let tweak_scalar = tweak.to_scalar();
        let tweak_scalar = secp256k1::Scalar::from_be_bytes(tweak_scalar.to_be_bytes())?;
        let mut key_agg_cache =
            self.key_agg_cache(&swap_info.swapper_public_key, &swap_info.public_key)?;
        let _ = key_agg_cache.pubkey_xonly_tweak_add(&self.musig_secp, &tweak_scalar)?;

        let cloned_tx = tx.clone();
        let mut sighasher = SighashCache::new(&cloned_tx);
        let prevouts: Vec<_> = utxos
            .iter()
            .map(|u| TxOut {
                value: u.amount_sat,
                script_pubkey: swap_address_script_pubkey.clone(),
            })
            .collect();
        let prevouts = Prevouts::All(&prevouts);
        let serialized_tx = serialize(&tx);
        for (input_index, input) in tx.input.iter_mut().enumerate() {
            let session_id = MusigSecRand::assume_unique_per_nonce_gen(rand::thread_rng().gen());
            let sighash = sighasher.taproot_key_spend_signature_hash(
                input_index,
                &prevouts,
                SchnorrSighashType::Default,
            )?;
            let msg = secp256k1::Message::from_digest(
                sighash
                    .to_vec()
                    .try_into()
                    .map_err(|_| ReceiveSwapError::generic("invalid signature hash"))?,
            );
            let extra_rand = rand::thread_rng().gen();
            let (our_sec_nonce, our_pub_nonce) = key_agg_cache
                .nonce_gen(
                    &self.musig_secp,
                    session_id,
                    secp_refund_pubkey,
                    msg,
                    Some(extra_rand),
                )
                .map_err(|_| ReceiveSwapError::generic("failed to generate nonce"))?;

            let refund_resp = self
                .swapper_api
                .refund_swap(
                    swap_info.bitcoin_address.clone(),
                    input_index as u32,
                    our_pub_nonce.serialize().to_vec(),
                    serialized_tx.clone(),
                )
                .await?;

            let their_pub_nonce = MusigPubNonce::from_slice(&refund_resp.pub_nonce)?;
            let agg_nonce =
                MusigAggNonce::new(&self.musig_secp, &[&their_pub_nonce, &our_pub_nonce]);
            let musig_session = MusigSession::new(&self.musig_secp, &key_agg_cache, agg_nonce, msg);

            let their_partial_sig =
                MusigPartialSignature::from_slice(&refund_resp.partial_signature)?;
            let partial_sig = musig_session.partial_sign(
                &self.musig_secp,
                our_sec_nonce,
                &secp_refund_privkey.keypair(&self.musig_secp),
                &key_agg_cache,
            )?;

            let sig = musig_session.partial_sig_agg(&[&their_partial_sig, &partial_sig]);
            input.witness.clear();
            input.witness.push(sig.as_byte_array());
        }

        Ok(tx)
    }

    pub fn create_unilateral_refund_tx(
        &self,
        swap_info: &SwapInfo,
        utxos: &[SwapOutput],
        destination_address: &Address,
        sat_per_vbyte: u32,
    ) -> ReceiveSwapResult<Transaction> {
        let weight = self
            .create_fake_unilateral_refund_tx(swap_info, utxos, destination_address)?
            .weight();
        let fee = compute_tx_fee(weight, sat_per_vbyte);
        let value: u64 = utxos
            .iter()
            .map(|utxo| utxo.amount_sat)
            .sum::<u64>()
            .saturating_sub(fee);

        let mut tx = Transaction {
            version: 2,
            lock_time: PackedLockTime::ZERO,
            input: utxos
                .iter()
                .map(|utxo| {
                    Ok(TxIn {
                        sequence: Sequence::from_consensus(swap_info.lock_height as u32),
                        ..utxo.try_into()?
                    })
                })
                .collect::<Result<_, ReceiveSwapError>>()?,
            output: vec![TxOut {
                value,
                script_pubkey: destination_address.script_pubkey(),
            }],
        };
        let swap_address: Address = swap_info.bitcoin_address.parse()?;
        let swap_address_script_pubkey = swap_address.script_pubkey();
        let claim_pubkey = PublicKey::from_slice(&swap_info.swapper_public_key)
            .map_err(|_| ReceiveSwapError::generic("invalid claim pubkey"))?;
        let refund_privkey = SecretKey::from_slice(&swap_info.private_key)
            .map_err(|_| ReceiveSwapError::generic("invalid refund private key"))?;
        let refund_pubkey = refund_privkey.public_key(&self.secp);
        let (x_only_claim_pubkey, _) = claim_pubkey.x_only_public_key();
        let (x_only_refund_pubkey, _) = refund_pubkey.x_only_public_key();
        let prevouts: Vec<_> = utxos
            .iter()
            .map(|u| TxOut {
                value: u.amount_sat,
                script_pubkey: swap_address_script_pubkey.clone(),
            })
            .collect();
        let prevouts = Prevouts::All(&prevouts);

        let claim_script = claim_script(&x_only_claim_pubkey, &swap_info.payment_hash);
        let refund_script = refund_script(&x_only_refund_pubkey, swap_info.lock_height as u32);
        let cloned_tx = tx.clone();
        let mut sighasher = SighashCache::new(&cloned_tx);
        for (input_index, input) in tx.input.iter_mut().enumerate() {
            let leaf_hash = TapLeafHash::from_script(&refund_script, LeafVersion::TapScript);

            let sighash = sighasher.taproot_script_spend_signature_hash(
                input_index,
                &prevouts,
                leaf_hash,
                SchnorrSighashType::Default,
            )?;

            let rnd = rand::thread_rng().gen();
            let msg = Message::from(sighash);
            let signature = self.secp.sign_schnorr_with_aux_rand(
                &msg,
                &refund_privkey.keypair(&self.secp),
                &rnd,
            );

            let signature: Vec<u8> = signature.as_ref().to_vec();
            let control_block = self
                .taproot_spend_info(
                    &swap_info.swapper_public_key,
                    &swap_info.public_key,
                    claim_script.clone(),
                    refund_script.clone(),
                )?
                .control_block(&(refund_script.clone(), LeafVersion::TapScript))
                .ok_or(ReceiveSwapError::Taproot(
                    "missing control block".to_string(),
                ))?;
            let witness = vec![
                signature,
                refund_script.to_bytes(),
                control_block.serialize(),
            ];
            input.witness.clear();
            input.witness = Witness::from_vec(witness);
        }

        Ok(tx)
    }

    pub async fn payout_swap(
        &self,
        swap_info: &SwapInfo,
        payment_request: String,
    ) -> ReceiveSwapResult<()> {
        let resp = self.swapper_api.pay_swap(payment_request.clone()).await;
        let status = match resp {
            Ok(_) => return Ok(()),
            Err(status) => status,
        };

        let error_message = match status.code() {
            tonic::Code::InvalidArgument => {
                error!(
                    "Invalid argument calling pay_swap for address {} with payment request {}: {}",
                    swap_info.bitcoin_address,
                    payment_request,
                    status.message()
                );
                format!("Invalid argument: {}", status.message())
            }
            tonic::Code::DeadlineExceeded => "Deadline exceeded".to_string(),
            tonic::Code::NotFound => "Swap not found on remote server".to_string(),
            tonic::Code::FailedPrecondition => {
                format!("Failed precondition: {}", status.message())
            }
            _ => tonic_wrap::Status(status).to_string(),
        };

        Err(ReceiveSwapError::PaymentError(error_message))
    }
}

impl TaprootReceiveSwap {
    fn key_agg_cache(
        &self,
        claim_pubkey: &[u8],
        refund_pubkey: &[u8],
    ) -> ReceiveSwapResult<MusigKeyAggCache> {
        let cp = secp256k1::PublicKey::from_slice(claim_pubkey)?;
        let rp = secp256k1::PublicKey::from_slice(refund_pubkey)?;
        Ok(MusigKeyAggCache::new(&self.musig_secp, &[&cp, &rp]))
    }

    fn taproot_spend_info(
        &self,
        claim_pubkey: &[u8],
        refund_pubkey: &[u8],
        claim_script: Script,
        refund_script: Script,
    ) -> ReceiveSwapResult<TaprootSpendInfo> {
        let m = self.key_agg_cache(claim_pubkey, refund_pubkey)?;
        let internal_key = m.agg_pk();

        // Convert from one secp256k1 crate to the other.
        let internal_key = XOnlyPublicKey::from_slice(&internal_key.serialize())?;

        // claim and refund scripts go in a taptree.
        Ok(TaprootBuilder::new()
            .add_leaf(1, claim_script)?
            .add_leaf(1, refund_script)?
            .finalize(&self.secp, internal_key)?)
    }
}

fn claim_script(x_only_claim_pubkey: &XOnlyPublicKey, hash: &[u8]) -> Script {
    script::Builder::new()
        .push_opcode(OP_HASH160)
        .push_slice(&ripemd160::Hash::hash(hash))
        .push_opcode(OP_EQUALVERIFY)
        .push_x_only_key(x_only_claim_pubkey)
        .push_opcode(OP_CHECKSIG)
        .into_script()
}

fn refund_script(x_only_refund_pubkey: &XOnlyPublicKey, lock_time: u32) -> Script {
    script::Builder::new()
        .push_x_only_key(x_only_refund_pubkey)
        .push_opcode(OP_CHECKSIGVERIFY)
        .push_int(Sequence::from_height(lock_time as u16).to_consensus_u32() as i64)
        .push_opcode(OP_CSV)
        .into_script()
}
