use crate::{
    breez_services::{OpenChannelParams, Receiver},
    chain::ChainService,
    error::{ReceivePaymentError, SdkError},
    node_api::{FetchBolt11Result, NodeAPI, NodeError},
    persist::{db::SqliteStorage, error::PersistError},
    swap_in_segwit::swap::create_swap_keys,
    swap_in_taproot::TaprootSwapperAPI,
    BreezEvent, InvoicePaidDetails, ListSwapsRequest, OpeningFeeParams, PrepareRefundRequest,
    PrepareRefundResponse, ReceivePaymentRequest, RefundRequest, RefundResponse,
};
use anyhow::{anyhow, Result};
use gl_client::{
    bitcoin::{
        blockdata::{
            constants::WITNESS_SCALE_FACTOR,
            opcodes::all::{OP_CHECKSIG, OP_CHECKSIGVERIFY, OP_CSV, OP_EQUALVERIFY, OP_HASH160},
            script,
        },
        consensus::serialize,
        hashes::{ripemd160, Hash},
        psbt::Prevouts,
        secp256k1::{All, Message, PublicKey, Secp256k1, SecretKey},
        util::{
            sighash::SighashCache,
            taproot::{
                LeafVersion, TapLeafHash, TaprootBuilder, TaprootBuilderError, TaprootSpendInfo,
            },
        },
        Address, Network, PackedLockTime, SchnorrSighashType, Script, Sequence, Transaction, TxIn,
        TxOut, Witness, XOnlyPublicKey,
    },
    lightning_invoice::Bolt11Invoice,
};
use rand::Rng;
use sdk_common::tonic_wrap;
use secp256k1::musig::{
    MusigAggNonce, MusigKeyAggCache, MusigPartialSignature, MusigPubNonce, MusigSecRand,
    MusigSession,
};
use std::{
    sync::Arc,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};
use thiserror::Error;
use tokio::sync::broadcast;

const PAYOUT_VALIDITY_BLOCKS: u32 = 360;
const EXPIRY_SECONDS_PER_BLOCK: u32 = 600;
const MIN_OPENING_FEE_PARAMS_VALIDITY_SECONDS: u32 = 1800;
const MIN_INVOICE_EXPIRY_SECONDS: u64 = 1800;
const MONITOR_EXPIRED_SWAP_BLOCKS: u32 = 144 * 28;
const DUST_LIMIT_SAT: u64 = 330;

#[derive(Debug, Clone, Default)]
pub(crate) struct TaprootSwap {
    /// Script address to send onchain funds to swap-in.
    pub address: String,

    /// Time when the swap was created. In seconds since unix epoch.
    pub created_at: u64,

    /// Relative time in blocks after which the swap has expired after onchain confirmation
    /// and is eligible for unilateral refund.
    pub lock_time: u32,

    /// Preimage to unlock the swap.
    pub preimage: Vec<u8>,

    /// Payment hash the swap is locked to. It can be unlocked with the preimage.
    pub payment_hash: Vec<u8>,

    /// Private key used for refunds and cooperative spends of the swap outputs.
    pub refund_private_key: Vec<u8>,

    /// Public key used by the swap server to claim the funds after payout, and
    /// used in cooperative spends.
    pub claim_public_key: Vec<u8>,

    /// The initial opening fee params when the swap was created. These may be
    /// used to create an open channel invoice with for the swap. It's also
    /// possible the opening fee params are re-requested at the time of payout,
    /// if they're equally cheap or cheaper at that point.
    pub accepted_opening_fee_params: OpeningFeeParams,
}

impl TaprootSwap {
    pub fn refund_public_key(&self) -> Result<PublicKey, TaprootSwapError> {
        let secret = SecretKey::from_slice(&self.refund_private_key)
            .map_err(|_| TaprootSwapError::generic("invalid refund private key"))?;
        Ok(secret.public_key(&Secp256k1::new()))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TaprootSwapParameters {
    pub max_swap_amount_sat: u64,
    pub min_swap_amount_sat: u64,
    pub min_utxo_amount_sat: u64,
}

impl From<sdk_common::grpc::SwapParameters> for TaprootSwapParameters {
    fn from(value: sdk_common::grpc::SwapParameters) -> Self {
        TaprootSwapParameters {
            max_swap_amount_sat: value.max_swap_amount_sat,
            min_swap_amount_sat: value.min_swap_amount_sat,
            min_utxo_amount_sat: value.min_utxo_amount_sat,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct FullTaprootSwapData {
    pub swap: TaprootSwap,
    pub outputs: Vec<TaprootSwapOutput>,
    pub paid_amount_msat: Option<u64>,
    pub bolt11: Option<String>,
    pub last_payment_error: Option<String>,
    pub parameters: Option<TaprootSwapParameters>,
    pub refund_transactions: Vec<String>,
}

impl FullTaprootSwapData {
    pub fn blocks_left(&self, current_tip: u32) -> Option<i32> {
        let min_confirmation_height = self.min_confirmation_height()?;
        let blocks_passed = current_tip.saturating_sub(min_confirmation_height);
        Some(
            (self.swap.lock_time as i32)
                .checked_sub_unsigned(blocks_passed)
                .unwrap_or(i32::MIN),
        )
    }

    pub fn confirmed_unspent_amount_msat(&self) -> u64 {
        self.confirmed_unspent_amount_sat() * 1000
    }

    pub fn confirmed_unspent_amount_sat(&self) -> u64 {
        self.confirmed_utxos()
            .iter()
            .map(|o| o.amount_sat)
            .sum::<u64>()
    }

    pub fn is_payable(&self, current_tip: u32) -> bool {
        if self.confirmed_unspent_amount_sat() == 0 {
            return false;
        }

        if self.paid_amount_msat.is_some() {
            return false;
        }

        let min_confirmation = match self.min_confirmation_height() {
            Some(min) => min,
            None => return false,
        };

        current_tip.saturating_sub(min_confirmation)
            < std::cmp::min(self.swap.lock_time, PAYOUT_VALIDITY_BLOCKS)
    }

    pub fn confirmed_outputs(&self) -> Vec<TaprootSwapOutput> {
        self.outputs
            .iter()
            .filter(|o| o.confirmed_at_height.is_some())
            .cloned()
            .collect()
    }

    pub fn confirmed_utxos(&self) -> Vec<TaprootSwapOutput> {
        self.confirmed_outputs()
            .into_iter()
            .filter(|o| o.spend.is_none())
            .collect()
    }

    pub fn is_marked_refundable(&self, current_tip: u32) -> bool {
        !self.refundable_utxos(current_tip).is_empty()
    }

    pub fn min_confirmation_height(&self) -> Option<u32> {
        self.confirmed_outputs()
            .iter()
            .filter_map(|o| o.confirmed_at_height)
            .min()
    }

    pub fn refundable_utxos(&self, current_tip: u32) -> Vec<TaprootSwapOutput> {
        if self.is_payable(current_tip) {
            return Vec::new();
        }

        let mut txos: Vec<_> = self.confirmed_outputs();
        txos.sort_by(|a, b| a.confirmed_at_height.cmp(&b.confirmed_at_height));
        let utxos: Vec<_> = self.confirmed_utxos();

        // If the swap is not payable, and not paid, all utxos are refundable.
        let paid_amount_msat = match self.paid_amount_msat {
            Some(p) => p,
            None => return utxos,
        };

        // If the swap is paid, only the unpaid outputs are refundable.
        // Deduce the paid outputs by assuming the first confirmed outputs are the ones belonging to the payment.
        let mut sum = 0;
        let paid_outputs: Vec<_> = txos
            .iter()
            .take_while(|o| {
                sum += o.amount_sat * 1000;

                // TODO: Change this logic once fees are involved in swapd.
                sum < paid_amount_msat
            })
            .collect();

        let refundable_utxos = utxos
            .into_iter()
            .filter(|o| {
                paid_outputs
                    .iter()
                    .all(|po| po.tx_id != o.tx_id || po.output_index != o.output_index)
            })
            .collect();

        refundable_utxos
    }

    pub fn unconfirmed_amount_sat(&self) -> u64 {
        self.unconfirmed_outputs()
            .iter()
            .map(|o| o.amount_sat)
            .sum::<u64>()
    }

    pub fn unconfirmed_outputs(&self) -> Vec<TaprootSwapOutput> {
        self.outputs
            .iter()
            .filter(|o| o.confirmed_at_height.is_none())
            .cloned()
            .collect()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TaprootSwapOutput {
    pub address: String,
    pub amount_sat: u64,
    pub tx_id: String,
    pub output_index: u32,
    pub confirmed_at_height: Option<u32>,
    pub block_hash: Option<String>,
    pub spend: Option<TaprootSwapSpend>,
}

#[derive(Clone, Debug)]
pub(crate) struct TaprootSwapSpend {
    pub tx_id: String,
    pub output_index: u32,
    pub spending_tx_id: String,
    pub spending_input_index: u32,
    pub confirmed_at_height: Option<u32>,
    pub block_hash: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct TaprootSwapRefund {
    pub refund_tx_id: String,
    pub spent_tx_id: String,
    pub spent_output_index: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum TaprootSwapError {
    #[error("{0}")]
    Generic(String),

    #[error("swap not found")]
    NotFound,

    #[error("swap has no utxos")]
    NoUtxos,

    #[error("invalid address")]
    InvalidAddress,

    #[error("insufficient funds to pay for tx fee")]
    InsufficientFunds,

    #[error(transparent)]
    Persistance(#[from] PersistError),

    #[error("{0}")]
    ServiceConnectivity(String),

    #[error("{0}")]
    Taproot(String),
}

impl TaprootSwapError {
    pub fn generic(msg: &str) -> Self {
        Self::Generic(msg.to_string())
    }
}

pub(crate) struct TaprootReceiveSwap {
    swapper_api: Arc<dyn TaprootSwapperAPI>,
    chain: Arc<dyn ChainService>,
    network: Network,
    node: Arc<dyn NodeAPI>,
    payment_receiver: Arc<dyn Receiver>,
    persister: Arc<SqliteStorage>,
    secp: Secp256k1<All>,
    musig_secp: secp256k1::Secp256k1<secp256k1::All>,
    status_changes_notifier: broadcast::Sender<FullTaprootSwapData>,
}

#[derive(Clone, Debug, Error)]
enum GetPaymentRequestError {
    #[error("needs new fee params")]
    NeedsNewFeeParams,
    #[error("invoice already exists")]
    InvoiceAlreadyExists,
    #[error("{0}")]
    Generic(String),
}

impl TaprootReceiveSwap {
    pub fn new(
        swapper_api: Arc<dyn TaprootSwapperAPI>,
        chain: Arc<dyn ChainService>,
        network: Network,
        node: Arc<dyn NodeAPI>,
        payment_receiver: Arc<dyn Receiver>,
        persister: Arc<SqliteStorage>,
    ) -> Self {
        let (status_changes_notifier, _) = broadcast::channel::<FullTaprootSwapData>(100);
        Self {
            swapper_api,
            chain,
            network,
            node,
            payment_receiver,
            persister,
            secp: Secp256k1::new(),
            musig_secp: secp256k1::Secp256k1::new(),
            status_changes_notifier,
        }
    }

    pub(crate) fn active_swaps(&self, current_tip: u32) -> Result<Vec<FullTaprootSwapData>> {
        Ok(self
            .persister
            .list_taproot_swaps()?
            .into_iter()
            .filter(|s| {
                // no outputs means initial state, or state needs to be restored.
                if s.outputs.is_empty() {
                    return true;
                }

                // unconfirmed outputs need monitoring.
                if s.outputs.iter().any(|o| o.confirmed_at_height.is_none()) {
                    return true;
                }

                // if the output is not spent or the spend unconfirmed.
                if s.outputs.iter().any(|o| {
                    o.spend
                        .as_ref()
                        .and_then(|s| s.confirmed_at_height)
                        .is_none()
                }) {
                    return true;
                }

                // Keep monitoring completed swaps for a while.
                if let Some(min_confirmation_height) = s.min_confirmation_height() {
                    if min_confirmation_height
                        .saturating_add(s.swap.lock_time)
                        .saturating_add(MONITOR_EXPIRED_SWAP_BLOCKS)
                        < current_tip
                    {
                        return true;
                    }
                }

                false
            })
            .collect())
    }

    async fn broadcast_and_persist_refund(
        &self,
        tx: Transaction,
        utxos: &[TaprootSwapOutput],
    ) -> Result<String, TaprootSwapError> {
        let tx_bytes = serialize(&tx);
        info!("broadcasting refund tx {:?}", hex::encode(&tx_bytes));
        let refund_tx_id = self.chain.broadcast_transaction(tx_bytes).await?;

        for utxo in utxos {
            self.persister.add_taproot_swap_refund(TaprootSwapRefund {
                refund_tx_id: refund_tx_id.clone(),
                spent_tx_id: utxo.tx_id.clone(),
                spent_output_index: utxo.output_index,
            })?;
        }

        Ok(refund_tx_id)
    }

    pub(crate) async fn check_active_swap(
        &self,
        swap: &mut FullTaprootSwapData,
        current_tip: u32,
    ) -> Result<()> {
        if let Err(e) = self.update_swap_onchain_data(swap).await {
            warn!(
                "Failed to update swap onchain data: {}, continuing swap check 
                with potentially outdated chain data.",
                e
            );
        }

        if !swap.is_payable(current_tip) {
            return Ok(());
        }

        self.get_swap_payment(swap, current_tip).await?;

        Ok(())
    }

    async fn check_existing_payment_request(
        &self,
        swap: &FullTaprootSwapData,
        bolt11_result: FetchBolt11Result,
    ) -> Result<Option<String>> {
        let invoice: Bolt11Invoice = bolt11_result.bolt11.parse()?;
        let invoice_expires_at = match invoice.expires_at() {
            Some(expires_at) => expires_at,
            None => {
                debug!("Existing taproot swap payment request has invalid expiry. Recreating payment request.");
                self.delete_invoice(swap, bolt11_result.bolt11).await?;
                return Ok(None);
            }
        };
        if invoice_expires_at.as_secs() < MIN_INVOICE_EXPIRY_SECONDS {
            debug!("Existing taproot swap payment request has expired / will expire soon. Recreating payment request.");
            self.delete_invoice(swap, bolt11_result.bolt11).await?;
            return Ok(None);
        }
        let invoice_amount_msat = invoice
            .amount_milli_satoshis()
            .ok_or(anyhow!("Found swap invoice without amount"))?;
        let amount_msat = bolt11_result
            .payer_amount_msat
            .unwrap_or(invoice_amount_msat);
        if amount_msat != swap.confirmed_unspent_amount_msat() {
            debug!("Existing taproot swap payment request amount is no longer correct. Recreating payment request.");
            self.delete_invoice(swap, bolt11_result.bolt11).await?;
            return Ok(None);
        }

        if let Some(payer_amount_msat) = bolt11_result.payer_amount_msat {
            // This is an open channel invoice, so liquidity won't be an issue.
            // TODO: Validate opening_fee_params validity.
            // TODO: Fetch opening_fee_params belonging to the invoice
            let opening_fee_params = swap.swap.accepted_opening_fee_params.clone();
            let wrapped_invoice = self
                .payment_receiver
                .wrap_node_invoice(
                    &bolt11_result.bolt11,
                    Some(OpenChannelParams {
                        payer_amount_msat,
                        opening_fee_params,
                    }),
                    None,
                )
                .await?;
            return Ok(Some(wrapped_invoice));
        }

        // This is not an open channel invoice, check liquidity.
        if self.payment_receiver.open_channel_needed(amount_msat)? {
            debug!("Existing taproot swap payment request is not an open channel invoice, but liquidity is no longer sufficient. Recreating payment request.");
            self.delete_invoice(swap, bolt11_result.bolt11).await?;
            return Ok(None);
        }

        Ok(Some(bolt11_result.bolt11))
    }

    pub async fn create_swap_address(
        &self,
        opening_fee_params: OpeningFeeParams,
    ) -> Result<FullTaprootSwapData, TaprootSwapError> {
        if let Some(mut unused_swap) = self.get_unused()? {
            info!(
                "Found unused swap when creating new swap address. Address: {}",
                unused_swap.swap.address
            );
            let resp = self.swapper_api.swap_parameters().await?;
            let parameters = match resp {
                Some(parameters) => parameters,
                None => {
                    return Err(TaprootSwapError::generic(
                        "missing parameters in swap_parameters response",
                    ))
                }
            }
            .into();

            self.persister
                .set_taproot_swap_parameters(&unused_swap.swap.address, &parameters)?;
            unused_swap.parameters = Some(parameters);
            return Ok(unused_swap);
        }

        let keys = create_swap_keys()?;
        let refund_pubkey = keys.public_key()?;
        let hash = keys.preimage_hash_bytes();
        let resp = self
            .swapper_api
            .create_swap(hash.clone(), refund_pubkey.serialize().to_vec())
            .await?;

        let claim_pubkey = PublicKey::from_slice(&resp.claim_pubkey)
            .map_err(|_| TaprootSwapError::generic("Received invalid claim pubkey from server"))?;
        let (x_only_claim_pubkey, _) = claim_pubkey.x_only_public_key();
        let (x_only_refund_pubkey, _) = refund_pubkey.x_only_public_key();
        let claim_script = claim_script(&x_only_claim_pubkey, &hash);
        let refund_script = refund_script(&x_only_refund_pubkey, resp.lock_time);

        let taproot_spend_info = self.taproot_spend_info(
            &claim_pubkey.serialize(),
            &refund_pubkey.serialize(),
            claim_script,
            refund_script,
        )?;
        let expected_address =
            Address::p2tr_tweaked(taproot_spend_info.output_key(), self.network).to_string();
        if resp.address != expected_address {
            return Err(TaprootSwapError::generic(
                "Received invalid taproot swap address from server",
            ));
        }

        let swap = TaprootSwap {
            address: expected_address,
            claim_public_key: resp.claim_pubkey,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            lock_time: resp.lock_time,
            payment_hash: hash,
            preimage: keys.preimage,
            refund_private_key: keys.priv_key,
            accepted_opening_fee_params: opening_fee_params,
        };
        let parameters = match resp.parameters {
            Some(parameters) => parameters,
            None => {
                return Err(TaprootSwapError::generic(
                    "missing parameters in create_swap response",
                ))
            }
        }
        .into();

        self.persister.add_taproot_swap(&swap)?;
        self.persister
            .set_taproot_swap_parameters(&swap.address, &parameters)?;

        Ok(FullTaprootSwapData {
            swap,
            parameters: Some(parameters),
            ..Default::default()
        })
    }

    async fn delete_invoice(&self, _swap: &FullTaprootSwapData, bolt11: String) -> Result<()> {
        self.node.delete_invoice(bolt11).await?;
        Ok(())
    }

    fn emit_swap_updated(&self, bitcoin_address: &str) -> Result<()> {
        let swap_data = self
            .persister
            .get_full_taproot_swap(bitcoin_address)?
            .ok_or_else(|| {
                anyhow!(format!(
                    "taproot swap address {} was not found",
                    bitcoin_address
                ))
            })?;
        self.status_changes_notifier
            .send(swap_data)
            .map_err(anyhow::Error::msg)?;
        Ok(())
    }

    async fn get_payment_request(
        &self,
        swap: &FullTaprootSwapData,
        current_tip: u32,
    ) -> Result<(String, bool), GetPaymentRequestError> {
        match self.get_payment_request_inner(swap, current_tip).await {
            Ok(s) => return Ok(s),
            Err(e) => match e {
                GetPaymentRequestError::InvoiceAlreadyExists => {}
                _ => return Err(e),
            },
        }

        debug!("Retrying to get payment request because invoice already existed.");
        // Retry getting the payment request once if it returned 'Invoice already exists' on the first try.
        self.get_payment_request_inner(swap, current_tip).await
    }

    async fn get_payment_request_inner(
        &self,
        swap: &FullTaprootSwapData,
        current_tip: u32,
    ) -> Result<(String, bool), GetPaymentRequestError> {
        let maybe_bolt11_result = self
            .node
            .fetch_bolt11(swap.swap.payment_hash.clone())
            .await?;
        let initial_fee_params_valid = swap
            .swap
            .accepted_opening_fee_params
            .valid_for(MIN_OPENING_FEE_PARAMS_VALIDITY_SECONDS)?;
        let opening_fee_params = match initial_fee_params_valid {
            true => Some(swap.swap.accepted_opening_fee_params.clone()),
            false => None,
        };

        // If a payment was requested before, the could be an existing invoice.
        // Validate the existing invoice, it may need to be recreated.
        if let Some(bolt11_result) = maybe_bolt11_result {
            let maybe_bolt11 = self
                .check_existing_payment_request(swap, bolt11_result)
                .await?;
            if let Some(bolt11) = maybe_bolt11 {
                return Ok((bolt11, false));
            }
        };

        let amount_msat = swap.confirmed_unspent_amount_msat();
        let blocks_left = swap.blocks_left(current_tip).ok_or(anyhow!(
            "cannot create payment request for unconfirmed swap"
        ))?;
        let blocks_left = match blocks_left {
            b if b <= 0 => 1,
            _ => blocks_left as u32,
        };
        // Note that if the accepted opening fee params is no longer valid, a new one will be issued by the
        // receive_payment function. It is checked in the response.
        let receive_resp = self
            .payment_receiver
            .receive_payment(ReceivePaymentRequest {
                // TODO: Substract fees here once swapd supports them.
                amount_msat,
                cltv: Some(144),
                description: format!("taproot swap {}", swap.swap.address),
                expiry: Some(blocks_left.saturating_mul(EXPIRY_SECONDS_PER_BLOCK)),
                opening_fee_params,
                preimage: Some(swap.swap.preimage.clone()),
                use_description_hash: None,
            })
            .await;
        match receive_resp {
            Ok(resp) => {
                if let Some(opening_fee_params) = resp.opening_fee_params {
                    if opening_fee_params.get_channel_fees_msat_for(amount_msat)
                        > swap
                            .swap
                            .accepted_opening_fee_params
                            .get_channel_fees_msat_for(amount_msat)
                    {
                        return Err(GetPaymentRequestError::NeedsNewFeeParams);
                    }
                }

                // TODO: Save the new opening_fee_params? Like 'last' opening_fee_params?
                return Ok((resp.ln_invoice.bolt11, true));
            }
            Err(e) => match e {
                ReceivePaymentError::InvoicePreimageAlreadyExists { err: _ } => {
                    debug!("Tried to create taproot swap invoice, but invoice preimage already exists.")
                }
                _ => return Err(e.into()),
            },
        };

        // Ending up here means the invoice already exists, even though it was checked above.
        // Retry this whole operation again if this is the first try.
        Err(GetPaymentRequestError::InvoiceAlreadyExists)
    }

    async fn get_swap_payment(&self, swap: &FullTaprootSwapData, current_tip: u32) -> Result<()> {
        // TODO: Handle NeedsNewFeeParams here
        let (payment_request, is_new_payment_request) =
            self.get_payment_request(swap, current_tip).await?;
        self.persister
            .set_taproot_swap_bolt11(&swap.swap.address, &payment_request)?;
        if is_new_payment_request {
            self.emit_swap_updated(&swap.swap.address)?;
        }
        let resp = self.swapper_api.pay_swap(payment_request.clone()).await;
        let status = match resp {
            Ok(_) => {
                // Nothing to do here. Swap updated event will be emitted by the invoice paid event.
                return Ok(());
            }
            Err(status) => status,
        };

        let error_message = match status.code() {
            tonic::Code::InvalidArgument => {
                error!(
                    "Invalid argument calling pay_swap for address {} with payment request {}: {}",
                    swap.swap.address,
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
        debug!("Error getting paid for taproot swap: {}", error_message);
        self.persister
            .set_taproot_swap_last_payment_error(&swap.swap.address, &error_message)?;
        self.emit_swap_updated(&swap.swap.address)?;
        Err(anyhow!(error_message))
    }

    fn key_agg_cache(
        &self,
        claim_pubkey: &[u8],
        refund_pubkey: &[u8],
    ) -> Result<MusigKeyAggCache, TaprootSwapError> {
        let cp = secp256k1::PublicKey::from_slice(claim_pubkey)?;
        let rp = secp256k1::PublicKey::from_slice(refund_pubkey)?;
        Ok(MusigKeyAggCache::new(&self.musig_secp, &[&cp, &rp]))
    }

    pub async fn list_refundables(&self) -> Result<Vec<FullTaprootSwapData>, TaprootSwapError> {
        let current_tip = self.chain.current_tip(true).await?;
        let swaps = self.persister.list_taproot_swaps()?;
        Ok(swaps
            .into_iter()
            .filter(|s| !s.refundable_utxos(current_tip).is_empty())
            .collect())
    }

    pub async fn list_swaps(
        &self,
        req: &ListSwapsRequest,
    ) -> Result<(Vec<FullTaprootSwapData>, usize), TaprootSwapError> {
        let current_tip = match req.status {
            Some(_) => self.chain.current_tip(true).await?,
            None => 0,
        };
        let swaps = self.persister.list_taproot_swaps()?;
        let filtered: Vec<_> = swaps
            .into_iter()
            .filter(|swap| {
                if let Some(from_timestamp) = req.from_timestamp {
                    if swap.swap.created_at < from_timestamp as u64 {
                        return false;
                    }
                }

                if let Some(to_timestamp) = req.to_timestamp {
                    if swap.swap.created_at >= to_timestamp as u64 {
                        return false;
                    }
                }

                if let Some(statuses) = &req.status {
                    let status = swap.to_swap_status(current_tip);
                    if !statuses.contains(&status) {
                        return false;
                    }
                }

                true
            })
            .collect();

        let len = filtered.len();
        let mut sized: Box<dyn Iterator<Item = _>> = Box::new(filtered.into_iter());
        if let Some(offset) = req.offset {
            sized = Box::new(sized.skip(offset as usize))
        }

        if let Some(limit) = req.limit {
            sized = Box::new(sized.take(limit as usize))
        }

        Ok((sized.collect(), len))
    }

    fn get_unused(&self) -> Result<Option<FullTaprootSwapData>, PersistError> {
        Ok(self
            .persister
            .list_unused_taproot_swaps()?
            .into_iter()
            .nth(0))
    }

    pub(crate) async fn on_event(&self, event: BreezEvent) -> Result<()> {
        match event {
            BreezEvent::NewBlock { block } => self.on_new_block(block).await?,
            BreezEvent::InvoicePaid { details } => self.on_invoice_paid(details)?,
            _ => {}
        };

        Ok(())
    }

    fn on_invoice_paid(&self, details: InvoicePaidDetails) -> Result<()> {
        let maybe_swap = self
            .persister
            .list_taproot_swaps()?
            .into_iter()
            .find(|s| hex::encode(&s.swap.payment_hash) == details.payment_hash);
        let swap = match maybe_swap {
            Some(s) => s,
            None => return Ok(()),
        };
        self.emit_swap_updated(&swap.swap.address)?;
        Ok(())
    }

    async fn on_new_block(&self, block: u32) -> Result<()> {
        let mut active_swaps = self.active_swaps(block)?;
        let mut futures = Vec::new();
        for swap in &mut active_swaps {
            futures.push(self.check_active_swap(swap, block));
        }

        let results = futures::future::join_all(futures).await;
        for result in results {
            if let Err(e) = result {
                error!("Error checking active swap: {}", e);
            }
        }
        Ok(())
    }

    async fn prepare_refund(
        &self,
        req: &PrepareRefundRequest,
        txin_fn: &mut impl FnMut(gl_client::bitcoin::OutPoint, u32) -> TxIn,
    ) -> Result<PrepareRefundResponse, TaprootSwapError> {
        let (swap_info, utxos) = self.prepare_refund_utxos(&req.swap_address).await?;
        let address: Address = req.to_address.parse()?;
        let tx = Transaction {
            version: 2,
            lock_time: PackedLockTime::ZERO,
            input: utxos
                .iter()
                .map(|u| {
                    Ok::<_, gl_client::bitcoin::hashes::hex::Error>(txin_fn(
                        gl_client::bitcoin::OutPoint {
                            txid: u.tx_id.parse()?,
                            vout: u.output_index,
                        },
                        swap_info.swap.lock_time,
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?,
            output: vec![TxOut {
                script_pubkey: address.script_pubkey(),
                value: 0,
            }],
        };

        let weight = tx.weight() as u32;
        let fee = (weight as u64)
            .saturating_mul(req.sat_per_vbyte as u64)
            .saturating_mul(WITNESS_SCALE_FACTOR as u64);
        Ok(PrepareRefundResponse {
            refund_tx_weight: weight,
            refund_tx_fee_sat: fee,
        })
    }

    async fn prepare_refund_utxos(
        &self,
        swap_address: &str,
    ) -> Result<(FullTaprootSwapData, Vec<TaprootSwapOutput>), TaprootSwapError> {
        let swap_info = self
            .persister
            .get_full_taproot_swap(swap_address)?
            .ok_or(TaprootSwapError::NotFound)?;
        let current_tip = self.chain.current_tip(true).await?;
        let mut utxos = swap_info.refundable_utxos(current_tip);
        if utxos.is_empty() {
            return Err(TaprootSwapError::NoUtxos);
        }

        // Sort UTXOs for deterministic transactions
        utxos.sort_by(|a, b| {
            a.tx_id
                .cmp(&b.tx_id)
                .then(a.output_index.cmp(&b.output_index))
        });

        Ok((swap_info, utxos))
    }

    pub(crate) async fn prepare_cooperative_refund(
        &self,
        req: &PrepareRefundRequest,
    ) -> Result<PrepareRefundResponse, TaprootSwapError> {
        self.prepare_refund(req, &mut |previous_output, _| TxIn {
            previous_output,
            script_sig: Script::default(),
            sequence: Sequence::ZERO,
            // Mock a 64 byte schnorr signature
            witness: Witness::from_vec(vec![[1; 64].to_vec()]),
        })
        .await
    }

    pub(crate) async fn prepare_unilateral_refund(
        &self,
        req: &PrepareRefundRequest,
    ) -> Result<PrepareRefundResponse, TaprootSwapError> {
        self.prepare_refund(req, &mut |previous_output, lock_time| TxIn {
            previous_output,
            script_sig: Script::default(),
            sequence: Sequence::from_consensus(lock_time),
            // Mock a 64 byte schnorr signature, 65 byte script and 37 byte control block
            witness: Witness::from_vec(vec![[1; 64].to_vec(), [1; 65].to_vec(), [1; 37].to_vec()]),
        })
        .await
    }

    async fn create_refund_tx(
        &self,
        swap_info: &FullTaprootSwapData,
        utxos: &[TaprootSwapOutput],
        destination_address: &str,
        txin_fn: &mut impl FnMut(gl_client::bitcoin::OutPoint, u32) -> TxIn,
    ) -> Result<Transaction, TaprootSwapError> {
        let address: Address = destination_address.parse()?;
        Ok(Transaction {
            version: 2,
            lock_time: PackedLockTime::ZERO,
            input: utxos
                .iter()
                .map(|u| {
                    Ok::<_, gl_client::bitcoin::hashes::hex::Error>(txin_fn(
                        gl_client::bitcoin::OutPoint {
                            txid: u.tx_id.parse()?,
                            vout: u.output_index,
                        },
                        swap_info.swap.lock_time,
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?,
            output: vec![TxOut {
                script_pubkey: address.script_pubkey(),
                value: 0,
            }],
        })
    }

    pub(crate) async fn refund_cooperative(
        &self,
        req: &RefundRequest,
    ) -> Result<RefundResponse, TaprootSwapError> {
        let (swap_info, utxos) = self.prepare_refund_utxos(&req.swap_address).await?;
        let mut tx = self
            .create_refund_tx(
                &swap_info,
                &utxos,
                &req.to_address,
                &mut |previous_output, _| TxIn {
                    previous_output,
                    script_sig: Script::default(),
                    sequence: Sequence::ZERO,
                    // Mock a 64 byte schnorr signature
                    witness: Witness::from_vec(vec![[1; 64].to_vec()]),
                },
            )
            .await?;

        let weight = tx.weight();
        let fee = (weight as u64)
            .saturating_mul(req.sat_per_vbyte as u64)
            .saturating_mul(WITNESS_SCALE_FACTOR as u64);

        let amount: u64 = self.validate_refund_amount(&utxos, fee)?;

        tx.output[0].value = amount;
        let swap_address: Address = swap_info.swap.address.parse()?;
        let swap_address_script_pubkey = swap_address.script_pubkey();
        let refund_privkey =
            secp256k1::SecretKey::from_slice(&swap_info.swap.refund_private_key)
                .map_err(|_| TaprootSwapError::generic("invalid refund private key"))?;
        let refund_pubkey = refund_privkey.public_key(&self.musig_secp);
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
        for input_index in 0..tx.input.len() {
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
                    .map_err(|_| TaprootSwapError::generic("invalid signature hash"))?,
            );
            let extra_rand = rand::thread_rng().gen();
            let key_agg_cache = self.key_agg_cache(
                &swap_info.swap.claim_public_key,
                &swap_info.swap.refund_public_key()?.serialize(),
            )?;
            let (our_sec_nonce, our_pub_nonce) = key_agg_cache
                .nonce_gen(
                    &self.musig_secp,
                    session_id,
                    refund_pubkey,
                    msg,
                    Some(extra_rand),
                )
                .map_err(|_| TaprootSwapError::generic("failed to generate nonce"))?;

            let refund_resp = self
                .swapper_api
                .refund_swap(
                    swap_info.swap.address.clone(),
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
                &refund_privkey.keypair(&self.musig_secp),
                &key_agg_cache,
            )?;

            let sig = musig_session.partial_sig_agg(&[&their_partial_sig, &partial_sig]);
            tx.input[input_index].witness.clear();
            tx.input[input_index].witness.push(sig.as_byte_array());
        }

        let refund_tx_id = self.broadcast_and_persist_refund(tx, &utxos).await?;
        self.emit_swap_updated(&swap_info.swap.address)?;
        Ok(RefundResponse { refund_tx_id })
    }

    pub(crate) async fn refund_unilateral(
        &self,
        req: &RefundRequest,
    ) -> Result<RefundResponse, TaprootSwapError> {
        let (swap_info, utxos) = self.prepare_refund_utxos(&req.swap_address).await?;
        let mut tx = self
            .create_refund_tx(
                &swap_info,
                &utxos,
                &req.to_address,
                &mut |previous_output, _| TxIn {
                    previous_output,
                    script_sig: Script::default(),
                    sequence: Sequence::from_consensus(swap_info.swap.lock_time),
                    witness: Witness::from_vec(vec![
                        [1; 64].to_vec(),
                        [1; 65].to_vec(),
                        [1; 37].to_vec(),
                    ]),
                },
            )
            .await?;

        let weight = tx.weight();
        let fee = (weight as u64)
            .saturating_mul(req.sat_per_vbyte as u64)
            .saturating_mul(WITNESS_SCALE_FACTOR as u64);

        let amount: u64 = self.validate_refund_amount(&utxos, fee)?;

        tx.output[0].value = amount;
        let swap_address: Address = swap_info.swap.address.parse()?;
        let swap_address_script_pubkey = swap_address.script_pubkey();
        let claim_pubkey = PublicKey::from_slice(&swap_info.swap.claim_public_key)
            .map_err(|_| TaprootSwapError::generic("invalid claim pubkey"))?;
        let refund_privkey = SecretKey::from_slice(&swap_info.swap.refund_private_key)
            .map_err(|_| TaprootSwapError::generic("invalid refund private key"))?;
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

        let claim_script = claim_script(&x_only_claim_pubkey, &swap_info.swap.payment_hash);
        let refund_script = refund_script(&x_only_refund_pubkey, swap_info.swap.lock_time);
        let cloned_tx = tx.clone();
        let mut sighasher = SighashCache::new(&cloned_tx);
        for input_index in 0..tx.input.len() {
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
                    &swap_info.swap.claim_public_key,
                    swap_info.swap.refund_public_key()?.serialize().as_ref(),
                    claim_script.clone(),
                    refund_script.clone(),
                )?
                .control_block(&(refund_script.clone(), LeafVersion::TapScript))
                .ok_or(TaprootSwapError::Taproot(
                    "missing control block".to_string(),
                ))?;
            let witness = vec![
                signature,
                serialize(&refund_script),
                control_block.serialize(),
            ];
            tx.input[input_index].witness.clear();
            tx.input[input_index].witness = Witness::from_vec(witness);
        }

        let refund_tx_id = self.broadcast_and_persist_refund(tx, &utxos).await?;
        self.emit_swap_updated(&swap_info.swap.address)?;

        Ok(RefundResponse { refund_tx_id })
    }

    pub(crate) async fn rescan_swaps(&self) -> Result<(), TaprootSwapError> {
        let swaps = self.persister.list_taproot_swaps()?;
        for mut swap in swaps {
            if let Err(e) = self.update_swap_onchain_data(&mut swap).await {
                error!(
                    "Failed to update swap onchain data during rescan for swap {}: {}",
                    swap.swap.address, e
                );
            }
        }

        Ok(())
    }

    pub(crate) fn subscribe_status_changes(&self) -> broadcast::Receiver<FullTaprootSwapData> {
        self.status_changes_notifier.subscribe()
    }

    fn taproot_spend_info(
        &self,
        claim_pubkey: &[u8],
        refund_pubkey: &[u8],
        claim_script: Script,
        refund_script: Script,
    ) -> Result<TaprootSpendInfo, TaprootSwapError> {
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

    async fn update_swap_onchain_data(&self, swap: &mut FullTaprootSwapData) -> Result<()> {
        let address_transactions = self
            .chain
            .address_transactions(swap.swap.address.clone())
            .await?;

        let mut changed_outputs = Vec::new();
        let mut new_outputs = Vec::new();
        let mut spends = Vec::new();
        for address_transaction in &address_transactions {
            for (output_index, vout) in address_transaction.vout.iter().enumerate() {
                if vout.scriptpubkey_address != swap.swap.address {
                    continue;
                }

                // This is a swap output. Check whether it was already stored,
                // to fire an event if so.
                for existing_output in &mut swap.outputs {
                    if existing_output.tx_id == address_transaction.txid
                        && existing_output.output_index == output_index as u32
                    {
                        if existing_output.confirmed_at_height
                            == address_transaction.status.block_height
                            && existing_output.block_hash == address_transaction.status.block_hash
                        {
                            continue;
                        }

                        existing_output.confirmed_at_height =
                            address_transaction.status.block_height;
                        existing_output.block_hash = address_transaction.status.block_hash.clone();

                        changed_outputs.push(existing_output.clone());
                    } else {
                        let output = TaprootSwapOutput {
                            address: swap.swap.address.clone(),
                            amount_sat: vout.value,
                            block_hash: address_transaction.status.block_hash.clone(),
                            confirmed_at_height: address_transaction.status.block_height,
                            output_index: output_index as u32,
                            tx_id: address_transaction.txid.clone(),
                            spend: None,
                        };
                        changed_outputs.push(output.clone());
                        new_outputs.push(output);
                    }
                }
            }
        }

        for output in &new_outputs {
            swap.outputs.push(output.clone());
        }

        for output in &changed_outputs {
            swap.outputs
                .retain(|o| !(o.tx_id == output.tx_id && o.output_index == output.output_index));
            swap.outputs.push(output.clone());
        }

        // Enumerate spends after enumerating the outputs, so there are no
        // missing outputs when handling the spends.
        for address_transaction in &address_transactions {
            for (input_index, vin) in address_transaction.vin.iter().enumerate() {
                if vin.prevout.scriptpubkey_address != swap.swap.address {
                    continue;
                }

                for existing_output in &mut swap.outputs {
                    if existing_output.tx_id != vin.txid || existing_output.output_index != vin.vout
                    {
                        continue;
                    }

                    let spend = TaprootSwapSpend {
                        tx_id: vin.txid.clone(),
                        output_index: vin.vout,
                        block_hash: address_transaction.status.block_hash.clone(),
                        confirmed_at_height: address_transaction.status.block_height,
                        spending_input_index: input_index as u32,
                        spending_tx_id: address_transaction.txid.clone(),
                    };
                    existing_output.spend = Some(spend.clone());
                    spends.push(spend);
                }
            }
        }

        let mut changed = false;
        if !new_outputs.is_empty() {
            self.persister.add_taproot_swap_outputs(&new_outputs)?;
            changed = true;
        }

        if !changed_outputs.is_empty() {
            self.persister.add_taproot_swap_outputs(&changed_outputs)?;
            changed = true;
        }

        if !spends.is_empty() {
            self.persister.add_taproot_swap_spends(&spends)?;
            changed = true;
        }

        if changed {
            self.emit_swap_updated(&swap.swap.address)?;
        }

        Ok(())
    }

    fn validate_refund_amount(
        &self,
        utxos: &[TaprootSwapOutput],
        fee: u64,
    ) -> Result<u64, TaprootSwapError> {
        let amount: u64 = utxos
            .iter()
            .map(|u| u.amount_sat)
            .sum::<u64>()
            .saturating_sub(fee);

        if amount < DUST_LIMIT_SAT {
            return Err(TaprootSwapError::InsufficientFunds);
        }

        Ok(amount)
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

impl From<TaprootBuilderError> for TaprootSwapError {
    fn from(value: TaprootBuilderError) -> Self {
        TaprootSwapError::Taproot(value.to_string())
    }
}

impl From<TaprootBuilder> for TaprootSwapError {
    fn from(_value: TaprootBuilder) -> Self {
        TaprootSwapError::Taproot("could not finalize taproot spend info".to_string())
    }
}

impl From<gl_client::bitcoin::secp256k1::Error> for TaprootSwapError {
    fn from(value: gl_client::bitcoin::secp256k1::Error) -> Self {
        TaprootSwapError::Taproot(value.to_string())
    }
}

impl From<secp256k1::Error> for TaprootSwapError {
    fn from(value: secp256k1::Error) -> Self {
        TaprootSwapError::Taproot(value.to_string())
    }
}

impl From<gl_client::bitcoin::locktime::Error> for TaprootSwapError {
    fn from(value: gl_client::bitcoin::locktime::Error) -> Self {
        TaprootSwapError::Taproot(value.to_string())
    }
}

impl From<gl_client::bitcoin::util::address::Error> for TaprootSwapError {
    fn from(_value: gl_client::bitcoin::util::address::Error) -> Self {
        TaprootSwapError::InvalidAddress
    }
}

impl From<gl_client::bitcoin::hashes::hex::Error> for TaprootSwapError {
    fn from(value: gl_client::bitcoin::hashes::hex::Error) -> Self {
        TaprootSwapError::Taproot(value.to_string())
    }
}

impl From<anyhow::Error> for TaprootSwapError {
    fn from(value: anyhow::Error) -> Self {
        TaprootSwapError::Generic(value.to_string())
    }
}

impl From<tonic::Status> for TaprootSwapError {
    fn from(value: tonic::Status) -> Self {
        TaprootSwapError::ServiceConnectivity(tonic_wrap::Status(value).to_string())
    }
}

impl From<SdkError> for TaprootSwapError {
    fn from(value: SdkError) -> Self {
        TaprootSwapError::Generic(value.to_string())
    }
}

impl From<SystemTimeError> for TaprootSwapError {
    fn from(_value: SystemTimeError) -> Self {
        TaprootSwapError::Generic("invalid system time".to_string())
    }
}

impl From<NodeError> for GetPaymentRequestError {
    fn from(value: NodeError) -> Self {
        GetPaymentRequestError::Generic(value.to_string())
    }
}

impl From<anyhow::Error> for GetPaymentRequestError {
    fn from(value: anyhow::Error) -> Self {
        GetPaymentRequestError::Generic(value.to_string())
    }
}

impl From<ReceivePaymentError> for GetPaymentRequestError {
    fn from(value: ReceivePaymentError) -> Self {
        GetPaymentRequestError::Generic(value.to_string())
    }
}

impl From<gl_client::bitcoin::consensus::encode::Error> for TaprootSwapError {
    fn from(value: gl_client::bitcoin::consensus::encode::Error) -> Self {
        TaprootSwapError::Generic(value.to_string())
    }
}

impl From<gl_client::bitcoin::util::sighash::Error> for TaprootSwapError {
    fn from(value: gl_client::bitcoin::util::sighash::Error) -> Self {
        TaprootSwapError::Generic(value.to_string())
    }
}

impl From<secp256k1::musig::ParseError> for TaprootSwapError {
    fn from(value: secp256k1::musig::ParseError) -> Self {
        TaprootSwapError::Taproot(value.to_string())
    }
}

impl From<secp256k1::musig::MusigSignError> for TaprootSwapError {
    fn from(value: secp256k1::musig::MusigSignError) -> Self {
        TaprootSwapError::Taproot(value.to_string())
    }
}
