use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, ensure, Result};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};

use super::boltzswap::{BoltzApiCreateReverseSwapResponse, BoltzApiReverseSwapStatus::*};
use super::error::{ReverseSwapError, ReverseSwapResult};
use crate::bitcoin::blockdata::constants::WITNESS_SCALE_FACTOR;
use crate::bitcoin::consensus::serialize;
use crate::bitcoin::hashes::hex::{FromHex, ToHex};
use crate::bitcoin::hashes::{sha256, Hash};
use crate::bitcoin::psbt::serialize::Serialize as PsbtSerialize;
use crate::bitcoin::secp256k1::{Message, Secp256k1, SecretKey};
use crate::bitcoin::util::sighash::SighashCache;
use crate::bitcoin::{
    Address, AddressType, EcdsaSighashType, KeyPair, Network, OutPoint, Script, Sequence,
    Transaction, TxIn, TxOut, Txid, Witness,
};
use crate::chain::{get_utxos, AddressUtxos, ChainService, OnchainTx, Utxo};
use crate::error::SdkResult;
use crate::models::{ReverseSwapServiceAPI, ReverseSwapperRoutingAPI};
use crate::node_api::{NodeAPI, NodeError};
use crate::swap_in::create_swap_keys;
use crate::{
    ensure_sdk, BreezEvent, Config, FullReverseSwapInfo, PayOnchainRequest, PaymentStatus,
    ReverseSwapInfo, ReverseSwapInfoCached, ReverseSwapPairInfo, ReverseSwapStatus,
    ReverseSwapStatus::*, RouteHintHop,
};

// Estimates based on https://github.com/BoltzExchange/boltz-backend/blob/master/lib/rates/FeeProvider.ts#L31-L42
pub const ESTIMATED_CLAIM_TX_VSIZE: u64 = 138;
pub const ESTIMATED_LOCKUP_TX_VSIZE: u64 = 153;
pub(crate) const MAX_PAYMENT_PATH_HOPS: u32 = 3;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateReverseSwapResponse {
    id: String,

    /// HODL invoice that has to be paid, for the Boltz service to lock up the funds
    invoice: String,

    /// Redeem script from which the lock address is derived. Can be used to check that the Boltz
    /// service didn't create an address without an HTLC.
    redeem_script: String,

    /// Amount of sats which will be locked
    onchain_amount: u64,

    /// Block height at which the reverse swap will be considered cancelled
    timeout_block_height: u32,

    /// Address to which the funds will be locked
    lockup_address: String,
}

#[derive(Debug)]
enum TxStatus {
    Unknown,
    Mempool,
    Confirmed,
}

impl From<&Option<OnchainTx>> for TxStatus {
    fn from(value: &Option<OnchainTx>) -> Self {
        match value {
            None => TxStatus::Unknown,
            Some(tx) => match tx.status.block_height {
                Some(_) => TxStatus::Confirmed,
                None => TxStatus::Mempool,
            },
        }
    }
}

/// This struct is responsible for sending to an onchain address using lightning payments.
/// It uses internally an implementation of [ReverseSwapServiceAPI] that represents Boltz reverse swapper service.
pub(crate) struct BTCSendSwap {
    config: Config,
    pub(crate) reverse_swapper_api: Arc<dyn ReverseSwapperRoutingAPI>,
    pub(crate) reverse_swap_service_api: Arc<dyn ReverseSwapServiceAPI>,
    persister: Arc<crate::persist::db::SqliteStorage>,
    chain_service: Arc<dyn ChainService>,
    node_api: Arc<dyn NodeAPI>,
    status_changes_notifier: broadcast::Sender<BreezEvent>,
}

impl BTCSendSwap {
    pub(crate) fn new(
        config: Config,
        reverse_swapper_api: Arc<dyn ReverseSwapperRoutingAPI>,
        reverse_swap_service_api: Arc<dyn ReverseSwapServiceAPI>,
        persister: Arc<crate::persist::db::SqliteStorage>,
        chain_service: Arc<dyn ChainService>,
        node_api: Arc<dyn NodeAPI>,
    ) -> Self {
        let (status_changes_notifier, _) = broadcast::channel::<BreezEvent>(100);
        Self {
            config,
            reverse_swapper_api,
            reverse_swap_service_api,
            persister,
            chain_service,
            node_api,
            status_changes_notifier,
        }
    }

    pub(crate) fn subscribe_status_changes(&self) -> broadcast::Receiver<BreezEvent> {
        self.status_changes_notifier.subscribe()
    }

    async fn emit_reverse_swap_updated(&self, id: &str) -> Result<()> {
        let full_rsi = self
            .persister
            .get_reverse_swap(id)?
            .ok_or_else(|| anyhow!(format!("reverse swap {} was not found", id)))?;
        self.status_changes_notifier
            .send(BreezEvent::ReverseSwapUpdated {
                details: self.convert_reverse_swap_info(full_rsi).await?,
            })
            .map_err(anyhow::Error::msg)?;
        Ok(())
    }

    pub(crate) async fn on_event(&self, e: BreezEvent) -> Result<()> {
        match e {
            BreezEvent::Synced => {
                // Since this relies on the most up-to-date states of the reverse swap HODL invoice payments,
                // a fresh [BreezServices::sync] *must* be called before this method.
                // Therefore we specifically call this on the Synced event
                self.process_monitored_reverse_swaps().await
            }
            _ => Ok(()),
        }
    }

    /// Validates the reverse swap arguments given by the user
    fn validate_recipient_address(claim_pubkey: &str) -> ReverseSwapResult<()> {
        Address::from_str(claim_pubkey)
            .map(|_| ())
            .map_err(|e| ReverseSwapError::InvalidDestinationAddress(e.to_string()))
    }

    pub(crate) fn validate_claim_tx_fee(claim_fee: u64) -> ReverseSwapResult<()> {
        let min_claim_fee = Self::calculate_claim_tx_fee(1)?;
        ensure_sdk!(
            claim_fee >= min_claim_fee,
            ReverseSwapError::ClaimFeerateTooLow
        );
        Ok(())
    }

    pub(crate) async fn last_hop_for_payment(&self) -> ReverseSwapResult<RouteHintHop> {
        let reverse_routing_node = self
            .reverse_swapper_api
            .fetch_reverse_routing_node()
            .await?;
        let routing_hints = self
            .reverse_swap_service_api
            .get_route_hints(hex::encode(reverse_routing_node.clone()))
            .await?;
        routing_hints
            .first()
            .ok_or_else(|| {
                ReverseSwapError::RouteNotFound(format!(
                    "No route hints found for reverse routing node {reverse_routing_node:?}"
                ))
            })?
            .hops
            .first()
            .ok_or_else(|| {
                ReverseSwapError::RouteNotFound(format!(
                    "No hops found for reverse routing node {reverse_routing_node:?}"
                ))
            })
            .cloned()
    }

    /// Creates and persists a reverse swap. If the initial payment fails, the reverse swap has the new
    /// status persisted.
    pub(crate) async fn create_reverse_swap(
        &self,
        req: PayOnchainRequest,
    ) -> ReverseSwapResult<FullReverseSwapInfo> {
        Self::validate_recipient_address(&req.recipient_address)?;

        let routing_node = self
            .reverse_swapper_api
            .fetch_reverse_routing_node()
            .await
            .map(hex::encode)?;
        let created_rsi = self
            .create_and_validate_rev_swap_on_remote(req.clone(), routing_node)
            .await?;

        // Perform validation on the created swap
        trace!("create_rev_swap v2 request: {req:?}");
        trace!("create_rev_swap v2 created_rsi: {created_rsi:?}");

        // Validate send_amount
        let request_send_amount_sat = req.prepare_res.sender_amount_sat;
        let request_send_amount_msat = request_send_amount_sat * 1_000;
        created_rsi.validate_invoice_amount(request_send_amount_msat)?;

        // Validate onchain_amount
        let lockup_fee_sat = req.prepare_res.fees_lockup;
        let service_fee_sat = super::get_service_fee_sat(
            req.prepare_res.sender_amount_sat,
            req.prepare_res.fees_percentage,
        );
        trace!("create_rev_swap v2 service_fee_sat: {service_fee_sat} sat");
        let expected_onchain_amount = request_send_amount_sat - service_fee_sat - lockup_fee_sat;
        ensure_sdk!(
            created_rsi.onchain_amount_sat == expected_onchain_amount,
            ReverseSwapError::generic("Unexpected onchain amount (lockup fee or service fee)")
        );

        // Validate claim_fee. If onchain_amount and claim_fee are both valid, receive_amount is also valid.
        ensure_sdk!(
            created_rsi.onchain_amount_sat > req.prepare_res.recipient_amount_sat,
            ReverseSwapError::generic("Unexpected receive amount")
        );
        let claim_fee = created_rsi.onchain_amount_sat - req.prepare_res.recipient_amount_sat;
        Self::validate_claim_tx_fee(claim_fee)?;

        self.persister.insert_reverse_swap(&created_rsi)?;
        info!("Created and persisted reverse swap {}", created_rsi.id);

        // Wait until one of the following happens:
        // - trying to pay the HODL invoice explicitly fails from Greenlight
        // - the regular poll of the Breez API detects the status of this reverse swap advanced to LockTxMempool
        //   (meaning Boltz detected that we paid the HODL invoice)
        // - the max allowed duration of a payment is reached
        let res = tokio::select! {
            pay_thread_res = tokio::time::timeout(
                Duration::from_secs(self.config.payment_timeout_sec as u64),
                self.node_api.send_pay(created_rsi.invoice.clone(), MAX_PAYMENT_PATH_HOPS)
            ) => {
                // TODO It doesn't fail when trying to pay more sats than max_payable?
                match pay_thread_res {
                    // Paying a HODL invoice does not typically return, so if send_payment() returned, it's an abnormal situation
                    Ok(Ok(res)) => Err(NodeError::PaymentFailed(format!("Payment of HODL invoice unexpectedly returned: {res:?}"))),

                    // send_payment() returned an error, so we know paying the HODL invoice failed
                    Ok(Err(e)) => Err(NodeError::PaymentFailed(format!("Failed to pay HODL invoice: {e}"))),

                    // send_payment() has been trying to pay for longer than the payment timeout
                    Err(e) => Err(NodeError::PaymentTimeout(format!("Trying to pay the HODL invoice timed out: {e}")))
                }
            },
            paid_invoice_res = self.poll_initial_boltz_status_transition(&created_rsi.id) => {
                paid_invoice_res.map(|_| created_rsi.clone()).map_err(|e| NodeError::Generic(e.to_string()))
            }
        };

        // The result of the creation call can succeed or fail
        // We update the rev swap status accordingly, which would otherwise have needed a fully fledged sync() call
        match res {
            Ok(_) => {
                let lockup_txid = self.get_lockup_tx(&created_rsi).await?.map(|tx| tx.txid);
                self.persister
                    .update_reverse_swap_status(&created_rsi.id, &InProgress)?;
                self.persister
                    .update_reverse_swap_lockup_txid(&created_rsi.id, lockup_txid)?;
                self.emit_reverse_swap_updated(&created_rsi.id).await?
            }
            Err(_) => {
                self.persister
                    .update_reverse_swap_status(&created_rsi.id, &Cancelled)?;
                self.emit_reverse_swap_updated(&created_rsi.id).await?
            }
        }

        Ok(res?)
    }

    /// Endless loop that periodically polls whether the reverse swap transitioned away from the
    /// initial status.
    ///
    /// The loop returns as soon as the lock tx is seen by Boltz. In other words, it returns as soon as
    /// the reverse swap status, as reported by Boltz, is [BoltzApiReverseSwapStatus::LockTxMempool]
    /// or [BoltzApiReverseSwapStatus::LockTxConfirmed]
    async fn poll_initial_boltz_status_transition(&self, id: &str) -> Result<()> {
        let mut i = 0;
        loop {
            sleep(Duration::from_secs(5)).await;

            info!("Checking Boltz status for reverse swap {id}, attempt {i}");
            let reverse_swap_boltz_status = self
                .reverse_swap_service_api
                .get_boltz_status(id.into())
                .await?;
            info!("Got Boltz status {reverse_swap_boltz_status:?}");

            // Return when lock tx is seen in the mempool or onchain
            // Typically we first detect when the lock tx is in the mempool
            // However, if the tx is broadcast and the block is mined between the iterations of this loop,
            // we might not see the LockTxMempool state and instead directly get the LockTxConfirmed
            if let LockTxMempool { .. } | LockTxConfirmed { .. } = reverse_swap_boltz_status {
                return Ok(());
            }
            i += 1;
        }
    }

    /// Create a new reverse swap on the remote service provider (Boltz), then validates its redeem script
    /// before returning it
    async fn create_and_validate_rev_swap_on_remote(
        &self,
        req: PayOnchainRequest,
        routing_node: String,
    ) -> ReverseSwapResult<FullReverseSwapInfo> {
        let reverse_swap_keys = create_swap_keys()?;

        let boltz_response = self
            .reverse_swap_service_api
            .create_reverse_swap_on_remote(
                req.prepare_res.sender_amount_sat,
                reverse_swap_keys.preimage_hash_bytes().to_hex(),
                reverse_swap_keys.public_key()?.to_hex(),
                hex::encode(&req.prepare_res.fees_hash),
                routing_node,
            )
            .await?;
        match boltz_response {
            BoltzApiCreateReverseSwapResponse::BoltzApiSuccess(response) => {
                let res = FullReverseSwapInfo {
                    created_at_block_height: self.chain_service.current_tip().await?,
                    claim_pubkey: req.recipient_address.as_bytes().to_vec(),
                    invoice: response.invoice,
                    preimage: reverse_swap_keys.preimage,
                    private_key: reverse_swap_keys.priv_key,
                    timeout_block_height: response.timeout_block_height,
                    id: response.id,
                    onchain_amount_sat: response.onchain_amount,
                    sat_per_vbyte: None,
                    receive_amount_sat: Some(req.prepare_res.recipient_amount_sat),
                    redeem_script: hex::decode(&response.redeem_script)?,
                    cache: ReverseSwapInfoCached {
                        status: Initial,
                        lockup_txid: None,
                        claim_txid: None,
                    },
                };

                res.validate_invoice(req.prepare_res.sender_amount_sat * 1_000)?;
                res.validate_redeem_script(response.lockup_address, self.config.network)?;
                Ok(res)
            }
            BoltzApiCreateReverseSwapResponse::BoltzApiError { error } => {
                Err(ReverseSwapError::ServiceConnectivity(format!(
                    "(Boltz) Failed to create reverse swap: {error}"
                )))
            }
        }
    }

    /// Builds and signs claim tx
    async fn create_claim_tx(&self, rs: &FullReverseSwapInfo) -> Result<Transaction> {
        let lockup_addr = rs.get_lockup_address(self.config.network)?;
        // claim_pubkey is actually the Bitcoin address in binary form, so we convert it to a string
        let claim_pubkey_str = String::from_utf8(rs.claim_pubkey.clone())?;
        let claim_addr = Address::from_str(&claim_pubkey_str)?;
        let redeem_script = Script::from_hex(&hex::encode(&rs.redeem_script))?;

        match lockup_addr.address_type() {
            Some(AddressType::P2wsh) => {
                // We explicitly only get the confirmed onchain transactions
                //
                // Otherwise, if we had gotten all txs, we risk a race condition when we try
                // to re-broadcast the claim tx. On re-broadcast, the claim tx is already in the
                // mempool, so it would be returned in the list below. This however would mark
                // the utxos as spent, so this address would have a confirmed amount of 0. When
                // building the claim tx below and trying to subtract fees from the confirmed amount,
                // this would lead to creating a tx with a negative amount. This doesn't happen
                // if we restrict this to confirmed txs, because then the mempool claim tx is not returned.
                //
                // If the claim tx is confirmed, we would not try to re-broadcast it, so the race
                // condition only exists when a re-broadcast is tried and the claim tx is unconfirmed.
                let confirmed_txs = self
                    .chain_service
                    .address_transactions(lockup_addr.to_string())
                    .await?
                    .into_iter()
                    .filter(|tx| tx.status.confirmed)
                    .collect();
                debug!("Found confirmed txs for lockup address {lockup_addr}: {confirmed_txs:?}");
                let utxos = get_utxos(lockup_addr.to_string(), confirmed_txs, true)?;

                // The amount locked in the claim address
                let claim_amount_sat = rs.onchain_amount_sat;
                debug!("Claim tx amount: {claim_amount_sat} sat");

                // Calculate amount sent in a backward compatible way
                let tx_out_value = match rs.sat_per_vbyte {
                    Some(claim_tx_feerate) => {
                        claim_amount_sat - Self::calculate_claim_tx_fee(claim_tx_feerate)?
                    }
                    None => rs.receive_amount_sat.ok_or(anyhow!(
                        "Cannot create claim tx: no claim feerate or receive amount found"
                    ))?,
                };
                debug!("Tx out amount: {tx_out_value} sat");

                Self::build_claim_tx_inner(
                    SecretKey::from_slice(rs.private_key.as_slice())?,
                    rs.preimage.clone(),
                    utxos,
                    claim_addr,
                    redeem_script,
                    tx_out_value,
                )
            }
            Some(addr_type) => Err(anyhow!("Unexpected lock address type: {addr_type:?}")),
            None => Err(anyhow!("Could not determine lock address type")),
        }
    }

    fn build_claim_tx_inner(
        secret_key: SecretKey,
        preimage: Vec<u8>,
        utxos: AddressUtxos,
        claim_addr: Address,
        redeem_script: Script,
        tx_out_value: u64,
    ) -> Result<Transaction> {
        let txins: Vec<TxIn> = utxos
            .confirmed
            .iter()
            .map(|utxo| TxIn {
                previous_output: utxo.out,
                script_sig: Script::new(),
                sequence: Sequence(0),
                witness: Witness::default(),
            })
            .collect();

        let tx_out: Vec<TxOut> = vec![TxOut {
            value: tx_out_value,
            script_pubkey: claim_addr.script_pubkey(),
        }];

        // construct the transaction
        let mut tx = Transaction {
            version: 2,
            lock_time: crate::bitcoin::PackedLockTime(0),
            input: txins.clone(),
            output: tx_out,
        };

        let claim_script_bytes = PsbtSerialize::serialize(&redeem_script);

        // Sign inputs (iterate, even though we only have one input)
        let scpt = Secp256k1::signing_only();
        let mut signed_inputs: Vec<TxIn> = Vec::new();
        for (index, input) in tx.input.iter().enumerate() {
            let mut signer = SighashCache::new(&tx);
            let sig = signer.segwit_signature_hash(
                index,
                &redeem_script,
                utxos.confirmed[index].value,
                EcdsaSighashType::All,
            )?;
            let msg = Message::from_slice(&sig[..])?;
            let sig = scpt.sign_ecdsa(&msg, &secret_key);

            let mut sigvec = sig.serialize_der().to_vec();
            sigvec.push(EcdsaSighashType::All as u8);

            let witness: Vec<Vec<u8>> = vec![sigvec, preimage.clone(), claim_script_bytes.clone()];

            let mut signed_input = input.clone();
            let w = Witness::from_vec(witness);
            signed_input.witness = w;
            signed_inputs.push(signed_input);
        }
        tx.input = signed_inputs;

        Ok(tx)
    }

    pub(crate) fn calculate_claim_tx_fee(claim_tx_feerate: u32) -> SdkResult<u64> {
        let tx = build_fake_claim_tx()?;

        // Based on https://github.com/breez/boltz/blob/master/boltz.go#L32
        let claim_witness_input_size: u32 = 1 + 1 + 8 + 73 + 1 + 32 + 1 + 100;
        let tx_weight =
            tx.strippedsize() as u32 * WITNESS_SCALE_FACTOR as u32 + claim_witness_input_size;
        let fees: u64 = (tx_weight * claim_tx_feerate / WITNESS_SCALE_FACTOR as u32) as u64;

        Ok(fees)
    }

    async fn get_claim_tx(&self, rsi: &FullReverseSwapInfo) -> Result<Option<OnchainTx>> {
        let lockup_addr = rsi.get_lockup_address(self.config.network)?;
        Ok(self
            .chain_service
            .address_transactions(lockup_addr.to_string())
            .await?
            .into_iter()
            .find(|tx| {
                let claim_pubkey_hex = hex::encode(&rsi.claim_pubkey);
                tx.vin
                    .iter()
                    .any(|vin| vin.prevout.scriptpubkey_address == lockup_addr.to_string())
                    && tx
                        .vout
                        .iter()
                        .any(|vout| vout.scriptpubkey_address == claim_pubkey_hex)
            }))
    }

    async fn get_lockup_tx(&self, rsi: &FullReverseSwapInfo) -> Result<Option<OnchainTx>> {
        let lockup_addr = rsi.get_lockup_address(self.config.network)?;
        let maybe_lockup_tx = self
            .chain_service
            .address_transactions(lockup_addr.to_string())
            .await?
            .into_iter()
            .find(|tx| {
                // Lockup tx is identified by having a vout matching the expected rev swap amount
                // going to the lockup address (P2WSH)
                trace!("Checking potential lock tx {tx:#?}");
                tx.vout.iter().any(|vout| {
                    vout.value == rsi.onchain_amount_sat
                        && vout.scriptpubkey_address == lockup_addr.to_string()
                })
            });

        Ok(maybe_lockup_tx)
    }

    /// Determine the new active status of a monitored reverse swap.
    ///
    /// If the status has not changed, it will return [None].
    async fn get_status_update_for_monitored(
        &self,
        rsi: &FullReverseSwapInfo,
        claim_tx_status: TxStatus,
    ) -> Result<Option<ReverseSwapStatus>> {
        let current_status = rsi.cache.status;
        ensure!(
            current_status.is_monitored_state(),
            "Tried to get status for non-monitored reverse swap"
        );

        let payment_hash_hex = &rsi.get_preimage_hash().to_hex();
        let payment_status = self.persister.get_payment_by_hash(payment_hash_hex)?;
        if let Some(ref payment) = payment_status {
            if payment.status == PaymentStatus::Failed {
                warn!("Payment failed for reverse swap {}", rsi.id);
                return Ok(Some(Cancelled));
            }
        }

        let new_status = match &current_status {
            Initial => match payment_status {
                Some(_) => Some(InProgress),
                None => match self
                    .reverse_swap_service_api
                    .get_boltz_status(rsi.id.clone())
                    .await?
                {
                    SwapExpired | LockTxFailed | LockTxRefunded { .. } | InvoiceExpired => {
                        // We only mark a reverse swap as Cancelled if Boltz also reports it in a cancelled or error state
                        // We do this to avoid race conditions in the edge-case when a reverse swap status update
                        // is triggered after creation succeeds, but before the payment is persisted in the DB
                        Some(Cancelled)
                    }
                    _ => None,
                },
            },
            InProgress => match claim_tx_status {
                TxStatus::Unknown => {
                    let block_height = self.chain_service.current_tip().await?;
                    match block_height >= rsi.timeout_block_height {
                        true => {
                            warn!("Reverse swap {} crossed the timeout block height", rsi.id);
                            Some(Cancelled)
                        }
                        false => None,
                    }
                }
                TxStatus::Mempool => Some(CompletedSeen),
                TxStatus::Confirmed => Some(CompletedConfirmed),
            },
            CompletedSeen => match claim_tx_status {
                TxStatus::Confirmed => Some(CompletedConfirmed),
                _ => None,
            },
            _ => None,
        };

        Ok(new_status)
    }

    /// Updates the cached values of monitored reverse swaps in the cache table and executes the
    /// corresponding next steps for the pending reverse swaps. This includes the blocking
    /// reverse swaps as well, since the blocking statuses are a subset of the monitored statuses.
    async fn process_monitored_reverse_swaps(&self) -> Result<()> {
        let monitored = self.list_monitored().await?;
        debug!("Found {} monitored reverse swaps", monitored.len());
        self.claim_reverse_swaps(monitored).await
    }

    async fn claim_reverse_swaps(&self, reverse_swaps: Vec<FullReverseSwapInfo>) -> Result<()> {
        for rsi in reverse_swaps {
            debug!("Processing reverse swap {rsi:?}");

            // Look for lockup and claim txs on chain
            let lockup_tx = self.get_lockup_tx(&rsi).await?;
            let lock_tx_status = TxStatus::from(&lockup_tx);
            let claim_tx = self.get_claim_tx(&rsi).await?;
            let claim_tx_status = TxStatus::from(&claim_tx);

            if let Some(tx) = &claim_tx {
                info!(
                    "Found claim tx for reverse swap {:?}: {:?}, status: {:?}",
                    rsi.id, tx.txid, claim_tx_status
                );
            }
            // Update cached state when new state is detected
            if let Some(new_status) = self
                .get_status_update_for_monitored(&rsi, claim_tx_status)
                .await?
            {
                self.persister
                    .update_reverse_swap_status(&rsi.id, &new_status)?;
                self.emit_reverse_swap_updated(&rsi.id).await?;
            }

            // (Re-)Broadcast the claim tx for monitored reverse swaps that have a confirmed lockup tx
            let broadcasted_claim_tx = if matches!(lock_tx_status, TxStatus::Confirmed) {
                info!("Lock tx is confirmed, preparing claim tx");
                let claim_tx = self.create_claim_tx(&rsi).await?;
                let claim_tx_broadcast_res = self
                    .chain_service
                    .broadcast_transaction(serialize(&claim_tx))
                    .await;
                match claim_tx_broadcast_res {
                    Ok(txid) => info!("Claim tx was broadcast with txid {txid}"),
                    Err(e) => error!("Claim tx failed to broadcast: {e}"),
                };
                Some(claim_tx)
            } else {
                None
            };

            // Cache lockup and claim tx txids if not cached yet
            if rsi.cache.lockup_txid.is_none() {
                self.persister
                    .update_reverse_swap_lockup_txid(&rsi.id, lockup_tx.map(|tx| tx.txid))?;
                self.emit_reverse_swap_updated(&rsi.id).await?;
            }
            if rsi.cache.claim_txid.is_none() {
                self.persister.update_reverse_swap_claim_txid(
                    &rsi.id,
                    claim_tx
                        .map(|tx| tx.txid)
                        .or(broadcasted_claim_tx.map(|tx| tx.txid().to_string())),
                )?;
                self.emit_reverse_swap_updated(&rsi.id).await?;
            }
        }

        Ok(())
    }

    pub async fn claim_reverse_swap(&self, lockup_address: String) -> ReverseSwapResult<()> {
        let rsis: Vec<FullReverseSwapInfo> = self
            .list_monitored()
            .await?
            .into_iter()
            .filter(|rev_swap| {
                lockup_address
                    == rev_swap
                        .get_lockup_address(self.config.network)
                        .map(|a| a.to_string())
                        .unwrap_or_default()
            })
            .collect();
        match rsis.is_empty() {
            true => Err(ReverseSwapError::Generic(format!(
                "Reverse swap address {lockup_address} was not found"
            ))),
            false => Ok(self.claim_reverse_swaps(rsis).await?),
        }
    }

    /// Returns the ongoing reverse swaps which have a status that block the creation of new reverse swaps
    pub async fn list_blocking(&self) -> Result<Vec<FullReverseSwapInfo>> {
        let mut matching_reverse_swaps = vec![];
        for rs in self.persister.list_reverse_swaps()? {
            debug!("Reverse swap {} has status {:?}", rs.id, rs.cache.status);
            if rs.cache.status.is_blocking_state() {
                matching_reverse_swaps.push(rs);
            }
        }
        Ok(matching_reverse_swaps)
    }

    /// Returns the reverse swaps for which we expect the status to change, and therefore need
    /// to be monitored.
    pub async fn list_monitored(&self) -> Result<Vec<FullReverseSwapInfo>> {
        let mut matching_reverse_swaps = vec![];
        for rs in self.persister.list_reverse_swaps()? {
            if rs.cache.status.is_monitored_state() {
                matching_reverse_swaps.push(rs);
            }
        }
        Ok(matching_reverse_swaps)
    }

    /// See [ReverseSwapServiceAPI::fetch_reverse_swap_fees]
    pub(crate) async fn fetch_reverse_swap_fees(&self) -> ReverseSwapResult<ReverseSwapPairInfo> {
        self.reverse_swap_service_api
            .fetch_reverse_swap_fees()
            .await
    }

    /// Converts the internal [FullReverseSwapInfo] into the user-facing [ReverseSwapInfo]
    pub(crate) async fn convert_reverse_swap_info(
        &self,
        full_rsi: FullReverseSwapInfo,
    ) -> Result<ReverseSwapInfo> {
        Ok(ReverseSwapInfo {
            id: full_rsi.id.clone(),
            claim_pubkey: hex::encode(&full_rsi.claim_pubkey),
            lockup_txid: self
                .get_lockup_tx(&full_rsi)
                .await?
                .map(|lockup_tx| lockup_tx.txid),
            claim_txid: match full_rsi.cache.status {
                CompletedSeen | CompletedConfirmed => self
                    .create_claim_tx(&full_rsi)
                    .await
                    .ok()
                    .map(|claim_tx| claim_tx.txid().to_hex()),
                _ => None,
            },
            onchain_amount_sat: full_rsi.onchain_amount_sat,
            status: full_rsi.cache.status,
        })
    }
}

/// Internal utility to create a fake claim tx: a tx that has the claim tx structure (input,
/// output, witness, etc) but with random values.
///
/// This is used to get the claim tx size, in order to then estimate the claim tx fee, before
/// knowing the actual claim tx.
fn build_fake_claim_tx() -> Result<Transaction> {
    let keys = KeyPair::new(&Secp256k1::new(), &mut thread_rng());

    let sk = keys.secret_key();
    let pk_compressed_bytes = keys.public_key().serialize().to_vec();
    let preimage_bytes = sha256::Hash::hash("123".as_bytes()).to_vec();
    let redeem_script = FullReverseSwapInfo::build_expected_reverse_swap_script(
        sha256::Hash::hash(&preimage_bytes).to_vec(), // 32 bytes
        pk_compressed_bytes.clone(),                  // 33 bytes
        pk_compressed_bytes,                          // 33 bytes
        840_000,
    )?;

    // Use a P2TR output, which is slightly larger than a P2WPKH (native segwit) output
    // This means we will slightly overpay when claiming to segwit addresses
    let claim_addr = Address::p2tr(
        &Secp256k1::new(),
        keys.public_key().x_only_public_key().0,
        None,
        Network::Bitcoin,
    );

    BTCSendSwap::build_claim_tx_inner(
        sk,
        preimage_bytes,
        AddressUtxos {
            confirmed: vec![Utxo {
                out: OutPoint {
                    txid: Txid::all_zeros(),
                    vout: 1,
                },
                value: 1_000,
                block_height: Some(123),
            }],
        },
        claim_addr,
        redeem_script,
        1_000,
    )
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::swap_out::get_service_fee_sat;
    use crate::test_utils::{MOCK_REVERSE_SWAP_MAX, MOCK_REVERSE_SWAP_MIN};
    use crate::{PrepareOnchainPaymentRequest, PrepareOnchainPaymentResponse, SwapAmountType};

    #[tokio::test]
    async fn test_prepare_onchain_payment_in_range() -> Result<()> {
        let sdk = crate::breez_services::tests::breez_services().await?;

        // User-specified send amount is within range
        assert_in_range_prep_payment_response(
            sdk.prepare_onchain_payment(PrepareOnchainPaymentRequest {
                amount_sat: MOCK_REVERSE_SWAP_MIN,
                amount_type: SwapAmountType::Receive,
                claim_tx_feerate: 1,
            })
            .await?,
        )?;

        // Derived send amount is within range
        assert_in_range_prep_payment_response(
            sdk.prepare_onchain_payment(PrepareOnchainPaymentRequest {
                amount_sat: MOCK_REVERSE_SWAP_MIN,
                amount_type: SwapAmountType::Receive,
                claim_tx_feerate: 1,
            })
            .await?,
        )?;

        Ok(())
    }

    #[tokio::test]
    async fn test_prepare_onchain_payment_out_of_range() -> Result<()> {
        let sdk = crate::breez_services::tests::breez_services().await?;

        // User-specified send amount is out of range (below min)
        assert!(sdk
            .prepare_onchain_payment(PrepareOnchainPaymentRequest {
                amount_sat: MOCK_REVERSE_SWAP_MIN - 1,
                amount_type: SwapAmountType::Send,
                claim_tx_feerate: 1,
            })
            .await
            .is_err());

        // User-specified send amount is out of range (above max)
        assert!(sdk
            .prepare_onchain_payment(PrepareOnchainPaymentRequest {
                amount_sat: MOCK_REVERSE_SWAP_MAX + 1,
                amount_type: SwapAmountType::Send,
                claim_tx_feerate: 1,
            })
            .await
            .is_err());

        // Derived send amount is out of range (below min: specified receive amount is 0)
        assert!(sdk
            .prepare_onchain_payment(PrepareOnchainPaymentRequest {
                amount_sat: 0,
                amount_type: SwapAmountType::Receive,
                claim_tx_feerate: 1,
            })
            .await
            .is_err());

        // Derived send amount is out of range (above max)
        assert!(sdk
            .prepare_onchain_payment(PrepareOnchainPaymentRequest {
                amount_sat: MOCK_REVERSE_SWAP_MAX,
                amount_type: SwapAmountType::Receive,
                claim_tx_feerate: 1,
            })
            .await
            .is_err());

        // Derived send amount is out of range (above max because the chosen claim tx feerate pushes the send above max)
        assert!(sdk
            .prepare_onchain_payment(PrepareOnchainPaymentRequest {
                amount_sat: MOCK_REVERSE_SWAP_MIN,
                amount_type: SwapAmountType::Receive,
                claim_tx_feerate: 1_000_000,
            })
            .await
            .is_err());

        Ok(())
    }

    /// Validates a [PrepareOnchainPaymentResponse] with all fields set.
    ///
    /// This is the case when the requested amount is within the reverse swap range.
    fn assert_in_range_prep_payment_response(res: PrepareOnchainPaymentResponse) -> Result<()> {
        dbg!(&res);

        let send_amount_sat = res.sender_amount_sat;
        let receive_amount_sat = res.recipient_amount_sat;
        let total_fees = res.total_fees;
        assert_eq!(send_amount_sat - total_fees, receive_amount_sat);

        let service_fees = get_service_fee_sat(send_amount_sat, res.fees_percentage);
        let expected_total_fees = res.fees_lockup + res.fees_claim + service_fees;
        assert_eq!(expected_total_fees, total_fees);

        Ok(())
    }
}
