use std::str::FromStr;
use std::sync::Arc;

use crate::boltzswap::{BoltzApiCreateReverseSwapResponse, BoltzApiReverseSwapStatus::*};
use crate::chain::{get_utxos, ChainService, MempoolSpace};
use crate::models::ReverseSwapperAPI;
use crate::ReverseSwapStatus::*;
use crate::{
    BreezEvent, Config, NodeAPI, ReverseSwapInfo, ReverseSwapInfoCached, ReverseSwapPairInfo,
    ReverseSwapStatus,
};
use anyhow::{anyhow, ensure, Result};
use bitcoin::blockdata::constants::WITNESS_SCALE_FACTOR;
use bitcoin::hashes::hex::{FromHex, ToHex};
use bitcoin::secp256k1::{Message, Secp256k1, SecretKey};
use bitcoin::util::sighash::SighashCache;
use bitcoin::{
    Address, AddressType, EcdsaSighashType, Script, Sequence, Transaction, TxIn, TxOut, Witness,
};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

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

enum TxStatus {
    Unknown,
    Mempool,
    Confirmed,
}

/// This struct is responsible for sending to an onchain address using lightning payments.
/// It uses internally an implementation of [ReverseSwapperAPI] that represents Boltz reverse swapper service.
pub(crate) struct BTCSendSwap {
    config: Config,
    pub(crate) reverse_swapper_api: Arc<dyn ReverseSwapperAPI>,
    persister: Arc<crate::persist::db::SqliteStorage>,
    chain_service: Arc<dyn ChainService>,
    node_api: Arc<dyn NodeAPI>,
}

impl BTCSendSwap {
    pub(crate) fn new(
        config: Config,
        reverse_swapper_api: Arc<dyn ReverseSwapperAPI>,
        persister: Arc<crate::persist::db::SqliteStorage>,
        chain_service: Arc<MempoolSpace>,
        node_api: Arc<dyn NodeAPI>,
    ) -> Self {
        Self {
            config,
            reverse_swapper_api,
            persister,
            chain_service,
            node_api,
        }
    }

    /// Validates the reverse swap arguments given by the user
    fn validate_rev_swap_args(claim_pubkey: &str) -> Result<()> {
        Address::from_str(claim_pubkey)
            .map(|_| ())
            .map_err(|_e| anyhow!("Invalid destination address"))
    }

    /// Creates and persists a reverse swap. If the initial payment fails, the reverse swap has the new
    /// status persisted.
    pub(crate) async fn create_reverse_swap(
        &self,
        amount_sat: u64,
        claim_pubkey: String,
        pair_hash: String,
        routing_node: String,
    ) -> Result<ReverseSwapInfo> {
        Self::validate_rev_swap_args(&claim_pubkey)?;

        let created_rsi = self
            .create_and_validate_rev_swap_on_remote(
                amount_sat,
                claim_pubkey,
                pair_hash,
                routing_node,
            )
            .await?;
        self.persister.insert_reverse_swap(&created_rsi)?;
        info!("Created and persisted reverse swap: {created_rsi:?}");

        // Wait until one of the following happens:
        // - trying to pay the HODL invoice explicitly fails from Greenlight
        // - the regular poll of the Breez API detects the status of this reverse swap advanced to LockTxMempool
        //   (meaning Boltz detected that we paid the HODL invoice)
        // - the max allowed duration of a payment is reached
        let res = tokio::select! {
            pay_thread_res = tokio::time::timeout(
                Duration::from_secs(self.config.payment_timeout_sec as u64),
                self.node_api.send_payment(created_rsi.invoice.clone(), None)
            ) => {
                // TODO It doesn't fail when trying to pay more sats than max_payable?
                match pay_thread_res {
                    // Paying a HODL invoice does not typically return, so if send_payment() returned, it's an abnormal situation
                    Ok(Ok(res)) => Err(anyhow!("Payment of HODL invoice unexpectedly returned: {res:?}")),

                    // send_payment() returned an error, so we know paying the HODL invoice failed
                    Ok(Err(e)) => Err(anyhow!("Failed to pay HODL invoice: {e}")),

                    // send_payment() has been trying to pay for longer than the payment timeout
                    Err(e) => Err(anyhow!("Trying to pay the HODL invoice timed out: {e}"))
                }
            },
            paid_invoice_res = self.poll_initial_boltz_status_transition(&created_rsi.id) => {
                paid_invoice_res.map(|_| created_rsi.clone())
            }
        };

        if res.is_err() {
            // Failed payment results in a cancelled state
            self.persister
                .update_reverse_swap_status(&created_rsi.id, &ReverseSwapStatus::Cancelled)?;
        }

        res
    }

    async fn poll_initial_boltz_status_transition(&self, id: &str) -> Result<()> {
        let mut i = 0;
        loop {
            sleep(Duration::from_secs(5)).await;

            info!("Checking reverse swap status, attempt {i}");
            let reverse_swap_boltz_status =
                self.reverse_swapper_api.get_boltz_status(id.into()).await?;
            if let LockTxMempool { transaction: _ } = reverse_swap_boltz_status {
                return Ok(());
            }
            i += 1;
        }
    }

    /// Create a new reverse swap on the remote service provider (Boltz), then validates its redeem script
    /// before returning it
    async fn create_and_validate_rev_swap_on_remote(
        &self,
        amount_sat: u64,
        claim_pubkey: String,
        pair_hash: String,
        routing_node: String,
    ) -> Result<ReverseSwapInfo> {
        let reverse_swap_keys = crate::swap::create_swap_keys()?;

        let boltz_response = self
            .reverse_swapper_api
            .create_reverse_swap_on_remote(
                amount_sat,
                reverse_swap_keys.preimage_hash_bytes().to_hex(),
                reverse_swap_keys.public_key()?.to_hex(),
                pair_hash.clone(),
                routing_node,
            )
            .await?;
        match boltz_response {
            BoltzApiCreateReverseSwapResponse::BoltzApiSuccess(response) => {
                let res = ReverseSwapInfo {
                    created_at_block_height: self.chain_service.current_tip().await?,
                    claim_pubkey,
                    invoice: response.invoice,
                    preimage: reverse_swap_keys.preimage,
                    private_key: reverse_swap_keys.priv_key,
                    timeout_block_height: response.timeout_block_height,
                    id: response.id,
                    onchain_amount_sat: response.onchain_amount,
                    redeem_script: response.redeem_script,
                    cache: ReverseSwapInfoCached {
                        status: ReverseSwapStatus::Initial,
                    },
                };

                res.validate_hodl_invoice(amount_sat * 1000)?;
                res.validate_redeem_script(response.lockup_address, self.config.network)?;
                Ok(res)
            }
            BoltzApiCreateReverseSwapResponse::BoltzApiError { error } => Err(anyhow!(error)),
        }
    }

    pub(crate) async fn on_event(&self, e: BreezEvent) -> Result<()> {
        match e {
            BreezEvent::Synced => {
                // Since this relies on the most up-to-date states of the reverse swap HODL invoice payments,
                // a fresh [BreezServices::sync] *must* be called before this method.
                // Therefore we specifically call this on the Synced event
                self.refresh_monitored_reverse_swaps().await?;

                // Expects the most up-to-date rev swap states to be in the cache DB, therefore the refresh above
                self.execute_pending_reverse_swaps().await
            }
            _ => Ok(()),
        }
    }

    /// Builds and signs claim tx
    async fn create_claim_tx(&self, rs: &ReverseSwapInfo) -> Result<Transaction> {
        let lockup_addr = rs.get_lockup_address(self.config.network)?;
        let claim_addr = Address::from_str(&rs.claim_pubkey)?;
        let redeem_script = Script::from_hex(&rs.redeem_script)?;

        match lockup_addr.address_type() {
            Some(AddressType::P2wsh) => {
                let txs = self
                    .chain_service
                    .address_transactions(lockup_addr.to_string())
                    .await?;
                let utxos = get_utxos(lockup_addr.to_string(), txs)?;

                let confirmed_amount: u64 = utxos
                    .confirmed
                    .iter()
                    .fold(0, |accum, item| accum + item.value);

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
                    value: confirmed_amount,
                    script_pubkey: claim_addr.script_pubkey(),
                }];

                // construct the transaction
                let mut tx = Transaction {
                    version: 2,
                    lock_time: bitcoin::PackedLockTime(0),
                    input: txins.clone(),
                    output: tx_out,
                };

                let recommended_fees = self.chain_service.recommended_fees().await?;
                let sat_per_vbyte = recommended_fees.half_hour_fee; // TODO Configurable

                let claim_script_bytes =
                    bitcoin::psbt::serialize::Serialize::serialize(&redeem_script);

                // Based on https://github.com/breez/boltz/blob/master/boltz.go#L31
                let claim_witness_input_size: u32 = 1 + 1 + 8 + 73 + 1 + 32 + 1 + 100;
                let tx_weight = tx.strippedsize() as u32 * WITNESS_SCALE_FACTOR as u32
                    + claim_witness_input_size * txins.len() as u32;
                let fees: u64 = (tx_weight * sat_per_vbyte / WITNESS_SCALE_FACTOR as u32) as u64;
                tx.output[0].value = confirmed_amount - fees;

                let scpt = Secp256k1::signing_only();

                // Sign inputs (iterate, even though we only have one input)
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
                    let secret_key = SecretKey::from_slice(rs.private_key.as_slice())?;
                    let sig = scpt.sign_ecdsa(&msg, &secret_key);

                    let mut sigvec = sig.serialize_der().to_vec();
                    sigvec.push(EcdsaSighashType::All as u8);

                    let witness: Vec<Vec<u8>> =
                        vec![sigvec, rs.preimage.clone(), claim_script_bytes.clone()];

                    let mut signed_input = input.clone();
                    let w = Witness::from_vec(witness);
                    signed_input.witness = w;
                    signed_inputs.push(signed_input);
                }
                tx.input = signed_inputs;

                Ok(tx)
            }
            Some(addr_type) => Err(anyhow!("Unexpected lock address type: {addr_type:?}")),
            None => Err(anyhow!("Could not determine lock address type")),
        }
    }

    /// Executes the corresponding next steps in the pending reverse swaps.
    ///
    /// Expects recently refreshed rev swap statuses to be present in the DB cache
    /// via [Self::refresh_monitored_reverse_swaps]
    pub(crate) async fn execute_pending_reverse_swaps(&self) -> Result<()> {
        let monitored = self.list_monitored().await?;
        debug!("Found {} monitored reverse swaps", monitored.len());

        // Depending on the new state, decide next steps and transition to the new state
        for rs in monitored {
            debug!("Checking monitored reverse swap {rs:?}");
            if matches!(self.get_lockup_tx_status(&rs).await?, TxStatus::Confirmed) {
                info!("Lock tx is confirmed, preparing claim tx");
                let claim_tx = self.create_claim_tx(&rs).await?;
                let claim_tx_broadcast_res = self
                    .chain_service
                    .broadcast_transaction(bitcoin::psbt::serialize::Serialize::serialize(
                        &claim_tx,
                    ))
                    .await;
                info!("Broadcast claim tx result: {claim_tx_broadcast_res:?}");
            }
        }

        Ok(())
    }

    /// The claim tx is considered confirmed when it has an incoming tx from the lockup address
    async fn get_claim_tx_status(&self, rsi: &ReverseSwapInfo) -> Result<TxStatus> {
        let lockup_addr = rsi.get_lockup_address(self.config.network)?;
        let maybe_claim_tx = self
            .chain_service
            .address_transactions(rsi.claim_pubkey.clone())
            .await?
            .into_iter()
            .find(|tx| {
                tx.vin
                    .iter()
                    .any(|vin| vin.prevout.scriptpubkey_address == lockup_addr.to_string())
            });

        match maybe_claim_tx {
            None => Ok(TxStatus::Unknown),
            Some(tx) => match tx.status.block_height {
                Some(_) => Ok(TxStatus::Confirmed),
                None => Ok(TxStatus::Mempool),
            },
        }
    }

    /// The lockup tx is seen when it has an incoming tx of the expected amount
    async fn get_lockup_tx_status(&self, rsi: &ReverseSwapInfo) -> Result<TxStatus> {
        let lockup_addr = rsi.get_lockup_address(self.config.network)?;
        let maybe_lockup_tx = self
            .chain_service
            .address_transactions(lockup_addr.to_string())
            .await?
            .into_iter()
            .find(|tx| {
                tx.vin
                    .iter()
                    .any(|vin| vin.prevout.value == rsi.onchain_amount_sat)
            });

        match maybe_lockup_tx {
            None => Ok(TxStatus::Unknown),
            Some(tx) => match tx.status.block_height {
                Some(_) => Ok(TxStatus::Confirmed),
                None => Ok(TxStatus::Mempool),
            },
        }
    }

    /// Determine the new active status of a monitored reverse swap.
    ///
    /// If the status has not changed, it will return [None].
    pub(crate) async fn get_status_update_for_monitored(
        &self,
        rsi: &ReverseSwapInfo,
    ) -> Result<Option<ReverseSwapStatus>> {
        let current_status = rsi.cache.status;
        ensure!(
            current_status.is_monitored_state(),
            "Tried to get status for non-monitored reverse swap"
        );

        let new_status = match &current_status {
            Initial => match self
                .persister
                .get_payment_by_hash(&rsi.get_preimage_hash().to_hex())?
            {
                Some(_) => Some(InProgress),
                None => match self
                    .reverse_swapper_api
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
            InProgress | CompletedSeen => match self.get_claim_tx_status(rsi).await? {
                TxStatus::Unknown => None,
                TxStatus::Mempool => Some(CompletedSeen),
                TxStatus::Confirmed => Some(CompletedConfirmed),
            },
            _ => None,
        };

        Ok(new_status)
    }

    /// Updates the state of monitored reverse swaps in the cache table. This includes the blocking
    /// reverse swaps as well, since the blocking statuses are a subset of the monitored statuses.
    async fn refresh_monitored_reverse_swaps(&self) -> Result<()> {
        for rsi in self.list_monitored().await? {
            self.refresh_reverse_swap(rsi).await?;
        }
        Ok(())
    }

    /// Updates the state of given reverse swap in the cache table, if the status has changed
    async fn refresh_reverse_swap(&self, rsi: ReverseSwapInfo) -> Result<()> {
        match self.get_status_update_for_monitored(&rsi).await? {
            None => Ok(()),
            Some(new_status) => self
                .persister
                .update_reverse_swap_status(&rsi.id, &new_status),
        }
    }

    /// Returns the ongoing reverse swaps which have a status that block the creation of new reverse swaps
    pub async fn list_blocking(&self) -> Result<Vec<ReverseSwapInfo>> {
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
    pub async fn list_monitored(&self) -> Result<Vec<ReverseSwapInfo>> {
        let mut matching_reverse_swaps = vec![];
        for rs in self.persister.list_reverse_swaps()? {
            if rs.cache.status.is_monitored_state() {
                matching_reverse_swaps.push(rs);
            }
        }
        Ok(matching_reverse_swaps)
    }

    /// See [ReverseSwapperAPI::fetch_reverse_swap_fees]
    pub(crate) async fn fetch_reverse_swap_fees(&self) -> Result<ReverseSwapPairInfo> {
        self.reverse_swapper_api.fetch_reverse_swap_fees().await
    }
}
