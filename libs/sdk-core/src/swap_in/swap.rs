use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use gl_client::{
    bitcoin::{
        blockdata::constants::WITNESS_SCALE_FACTOR,
        consensus::encode,
        hashes::sha256,
        secp256k1::{Message, PublicKey, Secp256k1, SecretKey},
        Address, AddressType, Network, OutPoint, Script, Sequence, TxIn, Witness,
    },
    lightning_invoice::Bolt11Invoice,
};
use rand::Rng;
use sdk_common::ensure_sdk;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex};

use crate::{
    breez_services::{OpenChannelParams, Receiver},
    chain::ChainService,
    error::ReceivePaymentError,
    node_api::{FetchBolt11Result, NodeAPI},
    persist::{
        cache::NodeStateStorage, error::PersistResult, swap::SwapStorage,
        transactions::CompletedPaymentStorage,
    },
    BreezEvent, ListSwapsRequest, OpeningFeeParams, PrepareRefundRequest, PrepareRefundResponse,
    ReceivePaymentRequest, RefundRequest, RefundResponse, SwapInfo, SwapStatus, SwapperAPI,
};

use super::{
    error::{GetPaymentRequestError, ReceiveSwapError, ReceiveSwapResult},
    segwit::SegwitReceiveSwap,
    taproot::TaprootReceiveSwap,
    TaprootSwapperAPI,
};

const EXPIRY_SECONDS_PER_BLOCK: u32 = 600;
const MIN_INVOICE_EXPIRY_SECONDS: u64 = 1800;
const MIN_OPENING_FEE_PARAMS_VALIDITY_SECONDS: u32 = 1800;
const MONITOR_EXPIRED_SWAP_BLOCKS: u32 = 144 * 28;

pub(crate) fn create_swap_keys() -> anyhow::Result<SwapKeys> {
    let priv_key = rand::thread_rng().gen::<[u8; 32]>().to_vec();
    let preimage = rand::thread_rng().gen::<[u8; 32]>().to_vec();
    Ok(SwapKeys { priv_key, preimage })
}

pub(crate) struct SwapKeys {
    pub(crate) priv_key: Vec<u8>,
    pub(crate) preimage: Vec<u8>,
}

impl SwapKeys {
    pub(crate) fn secret_key(&self) -> anyhow::Result<SecretKey> {
        Ok(SecretKey::from_slice(&self.priv_key)?)
    }

    pub(crate) fn public_key(&self) -> anyhow::Result<PublicKey> {
        Ok(PublicKey::from_secret_key(
            &Secp256k1::new(),
            &self.secret_key()?,
        ))
    }

    pub(crate) fn preimage_hash_bytes(&self) -> Vec<u8> {
        Message::from_hashed_data::<sha256::Hash>(&self.preimage[..])
            .as_ref()
            .to_vec()
    }
}

#[derive(Eq, PartialEq)]
enum SwapAddressType {
    Segwit,
    Taproot,
}

#[derive(Default, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SwapChainData {
    pub outputs: Vec<SwapOutput>,
}

impl SwapChainData {
    pub fn confirmed_utxos(&self) -> Vec<SwapOutput> {
        self.utxos()
            .into_iter()
            .filter(|u| u.confirmed_at_height.is_some())
            .collect()
    }

    pub fn utxos(&self) -> Vec<SwapOutput> {
        self.outputs
            .iter()
            .filter(|o| {
                let spend = match &o.spend {
                    Some(spend) => spend,
                    None => return true,
                };

                spend.confirmed_at_height.is_none()
            })
            .cloned()
            .collect()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SwapOutput {
    pub address: String,
    pub amount_sat: u64,
    pub tx_id: String,
    pub output_index: u32,
    pub confirmed_at_height: Option<u32>,
    pub block_hash: Option<String>,
    pub spend: Option<SwapSpend>,
}

impl TryInto<TxIn> for &SwapOutput {
    type Error = ReceiveSwapError;

    fn try_into(self) -> Result<TxIn, Self::Error> {
        Ok(TxIn {
            previous_output: OutPoint {
                txid: self.tx_id.parse()?,
                vout: self.output_index,
            },
            script_sig: Script::default(),
            sequence: Sequence::default(),
            witness: Witness::default(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SwapSpend {
    pub tx_id: String,
    pub output_index: u32,
    pub spending_tx_id: String,
    pub spending_input_index: u32,
    pub confirmed_at_height: Option<u32>,
    pub block_hash: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct SwapChainInfo {
    pub(crate) unconfirmed_sats: u64,
    pub(crate) unconfirmed_tx_ids: Vec<String>,
    pub(crate) confirmed_sats: u64,
    pub(crate) confirmed_tx_ids: Vec<String>,
    pub(crate) confirmed_at: Option<u32>,
    pub(crate) total_incoming_txs: u64,
}

impl From<SwapChainData> for SwapChainInfo {
    fn from(value: SwapChainData) -> Self {
        SwapChainInfo {
            unconfirmed_sats: value
                .outputs
                .iter()
                .filter(|o| o.spend.is_none() && o.confirmed_at_height.is_none())
                .map(|o| o.amount_sat)
                .sum(),
            unconfirmed_tx_ids: value
                .outputs
                .iter()
                .filter(|o| o.spend.is_none() && o.confirmed_at_height.is_none())
                .map(|o| o.tx_id.clone())
                .collect(),
            confirmed_sats: value
                .outputs
                .iter()
                .filter(|o| o.spend.is_none() && o.confirmed_at_height.is_some())
                .map(|o| o.amount_sat)
                .sum(),
            confirmed_tx_ids: value
                .outputs
                .iter()
                .filter(|o| o.spend.is_none() && o.confirmed_at_height.is_some())
                .map(|o| o.tx_id.clone())
                .collect(),
            confirmed_at: value
                .outputs
                .iter()
                .filter_map(|o| o.confirmed_at_height)
                .min(),
            total_incoming_txs: value.outputs.len() as u64,
        }
    }
}

pub(crate) struct BTCReceiveSwap {
    chain_service: Arc<dyn ChainService>,
    completed_payment_storage: Arc<dyn CompletedPaymentStorage>,
    current_tip: Mutex<u32>,
    node_api: Arc<dyn NodeAPI>,
    node_state_storage: Arc<dyn NodeStateStorage>,
    payment_receiver: Arc<dyn Receiver>,
    segwit: SegwitReceiveSwap,
    status_changes_notifier: broadcast::Sender<BreezEvent>,
    swap_storage: Arc<dyn SwapStorage>,
    taproot: TaprootReceiveSwap,
}

pub(crate) struct BTCReceiveSwapParameters {
    pub chain_service: Arc<dyn ChainService>,
    pub completed_payment_storage: Arc<dyn CompletedPaymentStorage>,
    pub network: Network,
    pub node_api: Arc<dyn NodeAPI>,
    pub node_state_storage: Arc<dyn NodeStateStorage>,
    pub payment_receiver: Arc<dyn Receiver>,
    pub segwit_swapper_api: Arc<dyn SwapperAPI>,
    pub swap_storage: Arc<dyn SwapStorage>,
    pub taproot_swapper_api: Arc<dyn TaprootSwapperAPI>,
}

impl BTCReceiveSwap {
    pub(crate) fn new(params: BTCReceiveSwapParameters) -> Self {
        BTCReceiveSwap {
            chain_service: params.chain_service,
            completed_payment_storage: params.completed_payment_storage,
            current_tip: Mutex::new(0),
            node_api: params.node_api,
            node_state_storage: params.node_state_storage,
            payment_receiver: params.payment_receiver,
            segwit: SegwitReceiveSwap::new(params.segwit_swapper_api),
            status_changes_notifier: broadcast::channel(100).0,
            swap_storage: params.swap_storage,
            taproot: TaprootReceiveSwap::new(params.network, params.taproot_swapper_api),
        }
    }
}

impl BTCReceiveSwap {
    pub(crate) async fn create_swap(
        &self,
        opening_fee_params: OpeningFeeParams,
    ) -> ReceiveSwapResult<SwapInfo> {
        let node_state = self
            .node_state_storage
            .get_node_state()?
            .ok_or(ReceiveSwapError::NodeStateNotFound)?;
        // Calculate max_allowed_deposit based on absolute max and current node state
        let fn_max_allowed_deposit = |max_allowed_deposit_abs: i64| {
            std::cmp::min(
                (node_state.max_receivable_msat / 1000) as i64,
                max_allowed_deposit_abs,
            )
        };

        let unused_swaps = self.list_unused()?;
        let unused_swap = unused_swaps.into_iter().find(|s| {
            let address_type = match parse_address(&s.bitcoin_address) {
                Ok(address_type) => address_type,
                Err(_) => return false,
            };
            matches!(address_type, SwapAddressType::Taproot)
        });
        if let Some(mut unused_swap) = unused_swap {
            // Check max_allowed_deposit and, if it changed, persist and validate changes
            let old_max_allowed_deposit = unused_swap.max_allowed_deposit;
            unused_swap.max_allowed_deposit =
                fn_max_allowed_deposit(unused_swap.max_swapper_payable);
            if unused_swap.max_allowed_deposit != old_max_allowed_deposit {
                info!("max_allowed_deposit for this swap has changed, updating it");
                validate_swap_limits(&unused_swap)?;
                self.swap_storage.update_swap_max_allowed_deposit(
                    &unused_swap.bitcoin_address,
                    unused_swap.max_allowed_deposit,
                )?;
            }

            self.swap_storage
                .update_swap_fees(&unused_swap.bitcoin_address, &opening_fee_params)?;

            return Ok(unused_swap);
        }

        let swap_info = self
            .taproot
            .create_swap(&node_state, opening_fee_params)
            .await?;
        self.swap_storage.insert_swap(&swap_info)?;
        Ok(swap_info)
    }

    pub(crate) fn list_swaps(&self, req: ListSwapsRequest) -> ReceiveSwapResult<Vec<SwapInfo>> {
        Ok(self.swap_storage.list_swaps(req)?)
    }

    pub(crate) fn list_in_progress_swaps(&self) -> ReceiveSwapResult<Vec<SwapInfo>> {
        self.list_swaps(ListSwapsRequest {
            status: Some(SwapStatus::in_progress()),
            ..Default::default()
        })
    }

    pub fn list_monitored(&self) -> ReceiveSwapResult<Vec<SwapInfo>> {
        let monitored = self.list_swaps(ListSwapsRequest {
            status: Some(SwapStatus::monitored()),
            ..Default::default()
        })?;
        let recent = self.list_swaps(ListSwapsRequest {
            from_timestamp: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .saturating_sub(Duration::from_secs(
                        MONITOR_EXPIRED_SWAP_BLOCKS as u64 * EXPIRY_SECONDS_PER_BLOCK as u64,
                    ))
                    .as_secs() as i64,
            ),
            ..Default::default()
        })?;

        let mut result = HashMap::new();
        for monitored in monitored {
            result.insert(monitored.bitcoin_address.clone(), monitored);
        }

        for recent in recent {
            result.insert(recent.bitcoin_address.clone(), recent);
        }

        let mut result: Vec<_> = result.into_values().collect();
        result.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(result)
    }

    #[allow(dead_code)]
    pub(crate) fn list_redeemables(&self) -> ReceiveSwapResult<Vec<SwapInfo>> {
        Ok(self.swap_storage.list_swaps(ListSwapsRequest {
            status: Some(SwapStatus::redeemable()),
            ..Default::default()
        })?)
    }

    pub(crate) fn list_refundables(&self) -> ReceiveSwapResult<Vec<SwapInfo>> {
        self.list_swaps(ListSwapsRequest {
            status: Some(SwapStatus::refundable()),
            ..Default::default()
        })
    }

    pub(crate) fn list_unused(&self) -> ReceiveSwapResult<Vec<SwapInfo>> {
        self.list_swaps(ListSwapsRequest {
            status: Some(SwapStatus::unused()),
            ..Default::default()
        })
    }

    pub(crate) async fn on_event(&self, e: BreezEvent) -> ReceiveSwapResult<()> {
        match e {
            BreezEvent::NewBlock { block: tip } => {
                debug!("got chain event {:?}", e);
                self.set_tip(tip).await;
                if let Err(e) = self.execute_pending_swaps(tip).await {
                    error!("Failed to execute pending swaps: {}", e);
                }
            }

            // When invoice is paid we lookup for a swap that matches the same hash.
            // In case we find one, we update its paid amount.
            BreezEvent::InvoicePaid { details } => {
                debug!("swap InvoicePaid event!");
                let hash_raw = hex::decode(&details.payment_hash)?;
                let mut swap_info = match self.swap_storage.get_swap_info_by_hash(&hash_raw)? {
                    Some(swap_info) => swap_info,
                    None => return Ok(()),
                };

                let payment = match self
                    .completed_payment_storage
                    .get_completed_payment_by_hash(&details.payment_hash)?
                {
                    Some(payment) => payment,
                    None => return Ok(()),
                };

                let current_tip = self.tip().await;
                let chain_data = self
                    .swap_storage
                    .get_swap_chain_data(&swap_info.bitcoin_address)?;
                swap_info.paid_msat = payment.amount_msat;
                let address = parse_address(&swap_info.bitcoin_address)?;
                let new_status =
                    self.calculate_status(&swap_info, &address, &chain_data, current_tip);
                self.swap_storage
                    .update_swap_paid_amount(&swap_info.bitcoin_address, swap_info.paid_msat)?;
                self.swap_storage
                    .set_swap_status(&swap_info.bitcoin_address, &new_status)?;
                self.emit_swap_updated(&swap_info.bitcoin_address)?;
            }
            _ => {} // skip events were are not interested in
        }

        Ok(())
    }

    pub(crate) async fn prepare_refund(
        &self,
        req: PrepareRefundRequest,
    ) -> ReceiveSwapResult<PrepareRefundResponse> {
        let address_type = parse_address(&req.swap_address)?;
        let swap_info = self
            .swap_storage
            .get_swap_info_by_address(&req.swap_address)?
            .ok_or(ReceiveSwapError::SwapNotFound("".to_string()))?;
        let chain_data = match self.swap_storage.get_swap_chain_data(&req.swap_address)? {
            Some(chain_data) => chain_data,
            None => {
                let chain_data = self.fetch_swap_onchain_data(&swap_info).await?;
                self.swap_storage.set_swap_chain_data(
                    &req.swap_address,
                    &chain_data,
                    &chain_data.clone().into(),
                )?;
                chain_data
            }
        };

        let mut utxos: Vec<_> = chain_data.confirmed_utxos();
        if utxos.is_empty() {
            return Err(ReceiveSwapError::NoUtxos);
        }

        // Sort UTXOs for deterministic transactions
        utxos.sort_by(|a, b| {
            a.tx_id
                .cmp(&b.tx_id)
                .then(a.output_index.cmp(&b.output_index))
        });

        let destination_address = req.to_address.parse()?;
        let tx = match address_type {
            SwapAddressType::Segwit => self
                .segwit
                .create_fake_refund_tx(&utxos, &destination_address),
            SwapAddressType::Taproot => match req.unilateral {
                Some(true) => self.taproot.create_fake_unilateral_refund_tx(
                    &swap_info,
                    &utxos,
                    &destination_address,
                ),
                _ => self.taproot.create_fake_cooperative_refund_tx(
                    &swap_info,
                    &utxos,
                    &destination_address,
                ),
            },
        }?;

        let weight = tx.weight();
        let fee = compute_tx_fee(weight, req.sat_per_vbyte);
        Ok(PrepareRefundResponse {
            refund_tx_weight: weight as u32,
            refund_tx_fee_sat: fee,
        })
    }

    pub(crate) async fn refund(&self, req: RefundRequest) -> ReceiveSwapResult<RefundResponse> {
        let current_tip = self.chain_service.current_tip().await?;
        let address_type = parse_address(&req.swap_address)?;
        let swap_info = self
            .swap_storage
            .get_swap_info_by_address(&req.swap_address)?
            .ok_or(ReceiveSwapError::SwapNotFound("".to_string()))?;
        let chain_data = match self.swap_storage.get_swap_chain_data(&req.swap_address)? {
            Some(chain_data) => chain_data,
            None => {
                let chain_data = self.fetch_swap_onchain_data(&swap_info).await?;
                self.swap_storage.set_swap_chain_data(
                    &req.swap_address,
                    &chain_data,
                    &chain_data.clone().into(),
                )?;
                chain_data
            }
        };
        let mut utxos: Vec<_> =
            refundable_utxos(&swap_info, &chain_data, current_tip, &address_type);
        if utxos.is_empty() {
            return Err(ReceiveSwapError::NoUtxos);
        }

        // Sort UTXOs for deterministic transactions
        utxos.sort_by(|a, b| {
            a.tx_id
                .cmp(&b.tx_id)
                .then(a.output_index.cmp(&b.output_index))
        });

        // Function to check whether all utxos exceed the timelock. Only used for unilateral refunds.
        let has_passed_timelock = utxos.iter().all(|utxo| {
            let confirmed_at = match utxo.confirmed_at_height {
                Some(confirmed_at) => confirmed_at,
                None => return false,
            };

            current_tip >= confirmed_at.saturating_add(swap_info.lock_height as u32)
        });
        let destination_address = req.to_address.parse()?;
        let tx = match address_type {
            SwapAddressType::Segwit => {
                if !has_passed_timelock {
                    return Err(ReceiveSwapError::UtxosTimelocked);
                }
                self.segwit.create_refund_tx(
                    &swap_info,
                    &utxos,
                    &destination_address,
                    req.sat_per_vbyte,
                )
            }
            SwapAddressType::Taproot => match req.unilateral {
                Some(true) => {
                    if !has_passed_timelock {
                        return Err(ReceiveSwapError::UtxosTimelocked);
                    }
                    self.taproot.create_unilateral_refund_tx(
                        &swap_info,
                        &utxos,
                        &destination_address,
                        req.sat_per_vbyte,
                    )
                }
                _ => {
                    self.taproot
                        .create_cooperative_refund_tx(
                            &swap_info,
                            &utxos,
                            &destination_address,
                            req.sat_per_vbyte,
                        )
                        .await
                }
            },
        }?;

        let refund_tx = encode::serialize(&tx);
        info!("broadcasting refund tx {:?}", hex::encode(&refund_tx));
        let tx_id = self.chain_service.broadcast_transaction(refund_tx).await?;
        self.swap_storage
            .insert_swap_refund_tx_ids(swap_info.bitcoin_address, tx_id.clone())?;
        self.emit_swap_updated(&req.swap_address)?;

        Ok(RefundResponse {
            refund_tx_id: tx_id,
        })
    }

    pub(crate) async fn redeem_swap(&self, address: String) -> ReceiveSwapResult<()> {
        let swap_info = self
            .swap_storage
            .get_swap_info_by_address(&address)?
            .ok_or(ReceiveSwapError::SwapNotFound("".to_string()))?;
        let address_type = parse_address(&address)?;

        let current_tip = self.chain_service.current_tip().await?;

        // TODO: Handle NeedsNewFeeParams here.
        let (payment_request, is_new_payment_request) =
            self.get_payment_request(&swap_info, current_tip).await?;
        self.swap_storage
            .update_swap_bolt11(swap_info.bitcoin_address.clone(), payment_request.clone())?;
        if is_new_payment_request {
            self.emit_swap_updated(&swap_info.bitcoin_address.clone())?;
        }

        let resp = match address_type {
            SwapAddressType::Segwit => self.segwit.payout_swap(payment_request).await,
            SwapAddressType::Taproot => self.taproot.payout_swap(&swap_info, payment_request).await,
        };

        let message = match resp {
            Ok(_) => {
                // Nothing to do here. Swap updated event will be emitted by the invoice paid event.
                return Ok(());
            }
            Err(err) => match err {
                ReceiveSwapError::PaymentError(err) => err,
                _ => return Err(err),
            },
        };

        debug!("Error getting paid for swap: {}", message);
        self.swap_storage
            .update_swap_redeem_error(swap_info.bitcoin_address.clone(), message.clone())?;
        self.emit_swap_updated(&swap_info.bitcoin_address)?;
        Err(ReceiveSwapError::PaymentError(message))
    }

    pub(crate) async fn rescan_monitored_swaps(&self, tip: u32) -> ReceiveSwapResult<()> {
        self.refresh_swaps(
            self.swap_storage.list_swaps(ListSwapsRequest {
                status: Some(SwapStatus::monitored()),
                ..Default::default()
            })?,
            tip,
        )
        .await
    }

    pub(crate) async fn rescan_swap(&self, address: &str, tip: u32) -> ReceiveSwapResult<()> {
        let swap = self
            .swap_storage
            .get_swap_info_by_address(address)?
            .ok_or(ReceiveSwapError::SwapNotFound("".to_string()))?;
        self.refresh_swaps(vec![swap], tip).await
    }

    pub(crate) async fn rescan_swaps(&self, tip: u32) -> ReceiveSwapResult<()> {
        self.refresh_swaps(
            self.swap_storage.list_swaps(ListSwapsRequest::default())?,
            tip,
        )
        .await
    }

    pub(crate) fn subscribe_status_changes(&self) -> broadcast::Receiver<BreezEvent> {
        self.status_changes_notifier.subscribe()
    }
}

/// ReceiveSwapper private functions
impl BTCReceiveSwap {
    fn calculate_status(
        &self,
        swap_info: &SwapInfo,
        address_type: &SwapAddressType,
        chain_data: &Option<SwapChainData>,
        current_tip: u32,
    ) -> SwapStatus {
        let chain_data = match chain_data {
            Some(cd) => cd,
            None => {
                return self.calculate_status_without_chain_data(
                    swap_info,
                    address_type,
                    current_tip,
                )
            }
        };

        // No unconfirmed or confirmed outputs at all means initial state.
        if chain_data.outputs.is_empty() {
            return SwapStatus::Initial;
        }

        // Get the minimum confirmation height. If there are no confirmed outputs yet, we are waiting for confirmation.
        let min_confirmation = match chain_data
            .outputs
            .iter()
            .filter_map(|o| o.confirmed_at_height)
            .min()
        {
            Some(min) => min,
            None => return SwapStatus::WaitingConfirmation,
        };

        // If none of the outputs are unspent, confirmed or unconfirmed, the swap is completed.
        if chain_data.utxos().is_empty() {
            return SwapStatus::Completed;
        }

        // If there are utxos, but they all have unconfirmed spends, they are refundable.
        // If the spends were confirmed, they wouldn't be utxos at all.
        if chain_data.utxos().iter().all(|utxo| utxo.spend.is_some()) {
            return SwapStatus::Refundable;
        }

        if address_type == &SwapAddressType::Taproot
            && chain_data.confirmed_utxos().iter().any(|utxo| {
                utxo.amount_sat > swap_info.max_allowed_deposit as u64
                    || utxo.amount_sat < swap_info.min_allowed_deposit as u64
            })
        {
            return SwapStatus::Refundable;
        }

        let payout_blocks_left = match address_type {
            SwapAddressType::Segwit => {
                self.segwit
                    .payout_blocks_left(swap_info, min_confirmation, current_tip)
            }
            SwapAddressType::Taproot => {
                self.taproot
                    .payout_blocks_left(swap_info, min_confirmation, current_tip)
            }
        };

        // If there are blocks left to be paid out and the swap was not redeemed yet, it is redeemable.
        if payout_blocks_left > 0 && swap_info.paid_msat == 0 {
            return SwapStatus::Redeemable;
        }

        // The swap is not redeemable. And there are confirmed or unconfirmed outputs.

        let unpaid_utxos = unpaid_utxos(swap_info, chain_data);

        // If all utxos were used for payment, that means the swap server hasn't claimed them yet.
        // There are no pending utxos, so the swap has completed.
        if unpaid_utxos.is_empty() {
            return SwapStatus::Completed;
        }

        let refundable_utxos = refundable_utxos(swap_info, chain_data, current_tip, address_type);

        // There are utxos left, but they are not refundable yet. Mark the status as 'Redeemed' in that case.
        if refundable_utxos.is_empty() {
            return SwapStatus::Redeemed;
        }

        // There are refundable utxos.
        SwapStatus::Refundable
    }

    fn calculate_status_without_chain_data(
        &self,
        swap_info: &SwapInfo,
        address_type: &SwapAddressType,
        current_tip: u32,
    ) -> SwapStatus {
        let mut passed_timelock = false;
        if let Some(confirmed_at) = swap_info.confirmed_at {
            let payout_blocks_left = match address_type {
                SwapAddressType::Segwit => {
                    self.segwit
                        .payout_blocks_left(swap_info, confirmed_at, current_tip)
                }
                SwapAddressType::Taproot => {
                    self.taproot
                        .payout_blocks_left(swap_info, confirmed_at, current_tip)
                }
            };
            passed_timelock = payout_blocks_left == 0;
        }

        // In case timelock has passed we can only be in the Refundable or Completed state.
        if passed_timelock {
            return match swap_info.confirmed_sats {
                0 => SwapStatus::Completed,
                // This is to make sure we don't consider refundable in case we only have one transaction which was already
                // paid by the swapper.
                _ => match (swap_info.paid_msat, swap_info.total_incoming_txs) {
                    (paid, 1) if paid > 0 => SwapStatus::Completed,
                    _ => SwapStatus::Refundable,
                },
            };
        }

        match (
            swap_info.confirmed_at,
            swap_info.unconfirmed_sats,
            swap_info.confirmed_sats,
            swap_info.paid_msat,
        ) {
            // We have confirmation and both uconfirmed and confirmed balance are zero then we are done
            (Some(_), 0, 0, _) => SwapStatus::Completed,
            // We got lightning payment so we are in redeemed state.
            (_, _, _, paid) if paid > 0 => SwapStatus::Redeemed,
            // We have positive confirmed balance then we should redeem the funds.
            (_, _, confirmed, _) if confirmed > 0 => SwapStatus::Redeemable,
            // We have positive unconfirmed balance then we are waiting for confirmation.
            (_, unconfirmed, _, _) if unconfirmed > 0 => SwapStatus::WaitingConfirmation,
            _ => SwapStatus::Initial,
        }
    }

    async fn check_existing_payment_request(
        &self,
        swap_info: &SwapInfo,
        bolt11_result: FetchBolt11Result,
    ) -> Result<Option<String>, GetPaymentRequestError> {
        let invoice: Bolt11Invoice = bolt11_result.bolt11.parse()?;
        let invoice_expires_at = match invoice.expires_at() {
            Some(expires_at) => expires_at,
            None => {
                debug!(
                    "Existing swap payment request has invalid expiry. Recreating payment request."
                );
                self.node_api.delete_invoice(bolt11_result.bolt11).await?;
                return Ok(None);
            }
        };
        if invoice_expires_at.as_secs() < MIN_INVOICE_EXPIRY_SECONDS {
            debug!("Existing swap payment request has expired / will expire soon. Recreating payment request.");
            self.node_api.delete_invoice(bolt11_result.bolt11).await?;
            return Ok(None);
        }
        let invoice_amount_msat =
            invoice
                .amount_milli_satoshis()
                .ok_or(GetPaymentRequestError::generic(
                    "Found swap invoice without amount",
                ))?;
        let amount_msat = bolt11_result
            .payer_amount_msat
            .unwrap_or(invoice_amount_msat);
        if amount_msat != swap_info.confirmed_sats * 1000 {
            debug!("Existing swap payment request amount is no longer correct. Recreating payment request.");
            self.node_api.delete_invoice(bolt11_result.bolt11).await?;
            return Ok(None);
        }

        if let Some(payer_amount_msat) = bolt11_result.payer_amount_msat {
            // This is an open channel invoice, so liquidity won't be an issue.
            // TODO: Validate opening_fee_params validity.
            // TODO: Fetch opening_fee_params belonging to the invoice
            let opening_fee_params = swap_info
                .channel_opening_fees
                .clone()
                .ok_or(GetPaymentRequestError::MissingOpeningFeeParams)?;
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
            debug!("Existing swap payment request is not an open channel invoice, but liquidity is no longer sufficient. Recreating payment request.");
            self.node_api.delete_invoice(bolt11_result.bolt11).await?;
            return Ok(None);
        }

        Ok(Some(bolt11_result.bolt11))
    }

    fn emit_swap_updated(&self, bitcoin_address: &str) -> PersistResult<()> {
        let swap_info = self
            .swap_storage
            .get_swap_info_by_address(bitcoin_address)?
            .ok_or_else(|| {
                anyhow::anyhow!(format!("swap address {} was not found", bitcoin_address))
            })?;
        debug!("Emitting swap updated event");
        self.status_changes_notifier
            .send(BreezEvent::SwapUpdated { details: swap_info })
            .map_err(anyhow::Error::msg)?;
        Ok(())
    }

    async fn execute_pending_swaps(&self, tip: u32) -> ReceiveSwapResult<()> {
        let monitored_swaps = self.list_monitored()?;
        debug!("Refreshing {} monitored swaps", monitored_swaps.len());

        // first refresh all swaps we monitor
        self.refresh_swaps(monitored_swaps, tip).await?;

        // redeem swaps
        let redeemable_swaps = self.list_redeemables()?;
        debug!("Processing {} redeemable swaps", redeemable_swaps.len());
        for s in redeemable_swaps {
            let swap_address = s.bitcoin_address;
            let bolt11 = s.bolt11.unwrap_or_default();

            match self.redeem_swap(swap_address.clone()).await {
                Ok(_) => info!("succeed to redeem swap {swap_address}: {bolt11}"),
                Err(err) => error!("failed to redeem swap {err:?}: {swap_address} {bolt11}"),
            }
        }

        Ok(())
    }

    async fn fetch_swap_onchain_data(&self, swap: &SwapInfo) -> ReceiveSwapResult<SwapChainData> {
        let txs = self
            .chain_service
            .address_transactions(swap.bitcoin_address.clone())
            .await?;

        let mut outputs = HashMap::new();

        // Collect all outputs that were sent to the swap address
        for tx in &txs {
            for (output_index, vout) in tx.vout.iter().enumerate() {
                if vout.scriptpubkey_address != swap.bitcoin_address {
                    continue;
                }

                let output = SwapOutput {
                    address: swap.bitcoin_address.clone(),
                    amount_sat: vout.value,
                    tx_id: tx.txid.clone(),
                    output_index: output_index as u32,
                    confirmed_at_height: tx.status.block_height,
                    block_hash: tx.status.block_hash.clone(),
                    spend: None,
                };

                let outpoint = format!("{}:{}", tx.txid, output_index);
                outputs.insert(outpoint, output);
            }
        }

        // Collect all spends of the swap outputs
        for tx in &txs {
            for (input_index, vin) in tx.vin.iter().enumerate() {
                let outpoint = format!("{}:{}", vin.txid, vin.vout);
                if let Some(output) = outputs.get_mut(&outpoint) {
                    output.spend = Some(SwapSpend {
                        tx_id: vin.txid.clone(),
                        output_index: vin.vout,
                        spending_tx_id: tx.txid.clone(),
                        spending_input_index: input_index as u32,
                        confirmed_at_height: tx.status.block_height,
                        block_hash: tx.status.block_hash.clone(),
                    });
                }
            }
        }

        let chain_data = SwapChainData {
            outputs: outputs.into_values().collect(),
        };

        Ok(chain_data)
    }

    /// Gets or creates a payment request for the current swap, given the passed timeout in blocks.
    /// The first return value is the payment request, the second a value indicating whether this payment
    /// request was newly created.
    async fn get_payment_request(
        &self,
        swap: &SwapInfo,
        blocks: u32,
    ) -> Result<(String, bool), GetPaymentRequestError> {
        match self.get_payment_request_inner(swap, blocks).await {
            Ok(s) => return Ok(s),
            Err(e) => match e {
                GetPaymentRequestError::InvoiceAlreadyExists => {}
                _ => return Err(e),
            },
        }

        debug!("Retrying to get payment request because invoice already existed.");
        // Retry getting the payment request once if it returned 'Invoice already exists' on the first try.
        self.get_payment_request_inner(swap, blocks).await
    }

    /// Gets or creates a payment request for the current swap, given the passed timeout in blocks.
    /// The first return value is the payment request, the second a value indicating whether this payment
    /// request was newly created.
    async fn get_payment_request_inner(
        &self,
        swap_info: &SwapInfo,
        blocks: u32,
    ) -> Result<(String, bool), GetPaymentRequestError> {
        let maybe_bolt11_result = self
            .node_api
            .fetch_bolt11(swap_info.payment_hash.clone())
            .await?;
        let accepted_opening_fee_params = swap_info
            .channel_opening_fees
            .as_ref()
            .ok_or(GetPaymentRequestError::MissingOpeningFeeParams)?;
        let initial_fee_params_valid =
            accepted_opening_fee_params.valid_for(MIN_OPENING_FEE_PARAMS_VALIDITY_SECONDS)?;
        let opening_fee_params = match initial_fee_params_valid {
            true => Some(accepted_opening_fee_params.clone()),
            false => None,
        };

        // If a payment was requested before, the could be an existing invoice.
        // Validate the existing invoice, it may need to be recreated.
        if let Some(bolt11_result) = maybe_bolt11_result {
            let maybe_bolt11 = self
                .check_existing_payment_request(swap_info, bolt11_result)
                .await?;
            if let Some(bolt11) = maybe_bolt11 {
                return Ok((bolt11, false));
            }
        };

        let amount_msat = swap_info.confirmed_sats * 1000;
        // Note that if the accepted opening fee params is no longer valid, a new one will be issued by the
        // receive_payment function. It is checked in the response.
        let receive_resp = self
            .payment_receiver
            .receive_payment(ReceivePaymentRequest {
                // TODO: Substract fees here once swapper supports them.
                amount_msat,
                cltv: Some(144),
                description: format!("taproot swap {}", swap_info.bitcoin_address),
                expiry: Some(blocks.saturating_mul(EXPIRY_SECONDS_PER_BLOCK)),
                opening_fee_params,
                preimage: Some(swap_info.preimage.clone()),
                use_description_hash: None,
            })
            .await;
        match receive_resp {
            Ok(resp) => {
                if let Some(opening_fee_params) = resp.opening_fee_params {
                    if opening_fee_params.get_channel_fees_msat_for(amount_msat)
                        > accepted_opening_fee_params.get_channel_fees_msat_for(amount_msat)
                    {
                        return Err(GetPaymentRequestError::NeedsNewFeeParams);
                    }
                }

                // TODO: Save the new opening_fee_params? Like 'last' opening_fee_params?
                return Ok((resp.ln_invoice.bolt11, true));
            }
            Err(e) => match e {
                ReceivePaymentError::InvoicePreimageAlreadyExists { err: _ } => {
                    debug!("Tried to create swap invoice, but invoice preimage already exists.")
                }
                _ => return Err(e.into()),
            },
        };

        // Ending up here means the invoice already exists, even though it was checked above.
        // Retry this whole operation again if this is the first try.
        Err(GetPaymentRequestError::InvoiceAlreadyExists)
    }

    async fn refresh_swap(
        &self,
        swap_info: &SwapInfo,
        current_tip: u32,
    ) -> ReceiveSwapResult<SwapInfo> {
        let (mut new_swap_info, new_chain_data) =
            match self.refresh_swap_onchain_data(swap_info).await {
                Ok((s, cd)) => (s, cd),
                Err(e) => {
                    error!(
                        "failed to refresh swap onchain status for address {}: {}",
                        swap_info.bitcoin_address, e
                    );
                    (swap_info.clone(), None)
                }
            };

        new_swap_info = match self.refresh_swap_payment_data(&new_swap_info).await {
            Ok(s) => s,
            Err(e) => {
                error!(
                    "failed to refresh swap payment status for address {}: {}",
                    swap_info.bitcoin_address, e
                );
                new_swap_info
            }
        };

        let address = parse_address(&swap_info.bitcoin_address)?;
        new_swap_info.status =
            self.calculate_status(&new_swap_info, &address, &new_chain_data, current_tip);
        if &new_swap_info != swap_info {
            self.swap_storage
                .set_swap_status(&swap_info.bitcoin_address, &new_swap_info.status)?;
            self.emit_swap_updated(&swap_info.bitcoin_address)?;
        }

        Ok(new_swap_info)
    }

    async fn refresh_swaps(&self, swaps: Vec<SwapInfo>, tip: u32) -> ReceiveSwapResult<()> {
        for s in swaps {
            match self.refresh_swap(&s, tip).await {
                Ok(_) => debug!("refreshed swap {}", s.bitcoin_address),
                Err(e) => error!("failed to refresh swap {}: {}", s.bitcoin_address, e),
            };
        }
        Ok(())
    }

    async fn refresh_swap_onchain_data(
        &self,
        swap_info: &SwapInfo,
    ) -> ReceiveSwapResult<(SwapInfo, Option<SwapChainData>)> {
        let existing_chain_data = self
            .swap_storage
            .get_swap_chain_data(&swap_info.bitcoin_address)?;
        let new_chain_data = match self.fetch_swap_onchain_data(swap_info).await {
            Ok(d) => d,
            Err(e) => {
                error!(
                    "failed to refresh swap onchain status for address {}: {}",
                    swap_info.bitcoin_address, e
                );
                return Ok((swap_info.clone(), existing_chain_data));
            }
        };
        let changed = match existing_chain_data {
            Some(e) => e != new_chain_data,
            None => true,
        };
        let chain_info = new_chain_data.clone().into();
        if changed {
            self.swap_storage.set_swap_chain_data(
                &swap_info.bitcoin_address,
                &new_chain_data,
                &chain_info,
            )?;
        }
        Ok((
            SwapInfo {
                confirmed_at: chain_info.confirmed_at,
                confirmed_sats: chain_info.confirmed_sats,
                confirmed_tx_ids: chain_info.confirmed_tx_ids,
                total_incoming_txs: chain_info.total_incoming_txs,
                unconfirmed_sats: chain_info.unconfirmed_sats,
                unconfirmed_tx_ids: chain_info.unconfirmed_tx_ids,
                ..swap_info.clone()
            },
            Some(new_chain_data),
        ))
    }

    async fn refresh_swap_payment_data(&self, swap_info: &SwapInfo) -> ReceiveSwapResult<SwapInfo> {
        let payment = self
            .completed_payment_storage
            .get_completed_payment_by_hash(&hex::encode(swap_info.payment_hash.clone()))?;
        let payment = match payment {
            Some(p) => p,
            None => return Ok(swap_info.clone()),
        };
        debug!(
            "found payment for hash {:?}, {:?}",
            &hex::encode(swap_info.payment_hash.clone()),
            payment
        );
        let amount_msat = payment.amount_msat;
        if amount_msat != swap_info.paid_msat {
            self.swap_storage
                .update_swap_paid_amount(&swap_info.bitcoin_address, amount_msat)?;
        }

        Ok(SwapInfo {
            paid_msat: amount_msat,
            ..swap_info.clone()
        })
    }

    async fn set_tip(&self, tip: u32) {
        *self.current_tip.lock().await = tip;
    }

    async fn tip(&self) -> u32 {
        *self.current_tip.lock().await
    }
}

fn parse_address(address: &str) -> ReceiveSwapResult<SwapAddressType> {
    let address: Address = address.parse()?;
    match address.address_type() {
        Some(AddressType::P2tr) => Ok(SwapAddressType::Taproot),
        Some(AddressType::P2wsh) => Ok(SwapAddressType::Segwit),
        _ => Err(ReceiveSwapError::InvalidAddressType),
    }
}

pub(super) fn compute_tx_fee(tx_weight: usize, sat_per_vbyte: u32) -> u64 {
    (tx_weight as u32 * sat_per_vbyte / WITNESS_SCALE_FACTOR as u32) as u64
}

fn refundable_utxos(
    swap_info: &SwapInfo,
    chain_data: &SwapChainData,
    current_tip: u32,
    address_type: &SwapAddressType,
) -> Vec<SwapOutput> {
    unpaid_utxos(swap_info, chain_data)
        .into_iter()
        .filter(|o| match address_type {
            // segwit utxos are refundable after the locktime expires.
            SwapAddressType::Segwit => o
                .confirmed_at_height
                .map(|height| {
                    current_tip.saturating_sub(height.saturating_add(swap_info.lock_height as u32))
                        == 0
                })
                .unwrap_or(false),
            // Taproot utxos are always refundable.
            SwapAddressType::Taproot => true,
        })
        .collect()
}

/// Deduces the paid outputs by assuming the first confirmed outputs are the ones belonging to the payment.
fn unpaid_utxos(swap_info: &SwapInfo, chain_data: &SwapChainData) -> Vec<SwapOutput> {
    let mut all_outputs = chain_data.outputs.clone();
    all_outputs.sort_by(|a, b| a.confirmed_at_height.cmp(&b.confirmed_at_height));
    let mut sum = 0;
    let paid_outputs: Vec<_> = all_outputs
        .iter()
        .take_while(|o| {
            if sum >= swap_info.paid_msat {
                return false;
            }

            sum += o.amount_sat * 1000;
            true
        })
        .collect();

    chain_data
        .utxos()
        .into_iter()
        .filter(|o| {
            paid_outputs
                .iter()
                .all(|po| po.tx_id != o.tx_id || po.output_index != o.output_index)
        })
        .collect()
}

fn validate_swap_limits(swap_info: &SwapInfo) -> ReceiveSwapResult<()> {
    ensure_sdk!(
        swap_info.max_allowed_deposit >= swap_info.min_allowed_deposit,
        ReceiveSwapError::unsupported_swap_limits("No allowed deposit amounts")
    );
    Ok(())
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use gl_client::bitcoin::Network;
    use mockall::predicate;

    use crate::{
        chain::{OnchainTx, TxStatus, Vin, Vout},
        persist::{
            cache::MockNodeStateStorage, swap::MockSwapStorage,
            transactions::MockCompletedPaymentStorage,
        },
        swap_in::{
            swap::{SwapOutput, SwapSpend},
            taproot_server::MockTaprootSwapperAPI,
            BTCReceiveSwap, BTCReceiveSwapParameters,
        },
        test_utils::{
            MockBreezServer, MockChainService, MockNodeAPI, MockReceiver, MockSwapperAPI,
        },
        ListSwapsRequest, NodeState, OpeningFeeParams, Payment, SwapInfo, SwapStatus,
    };

    use super::SwapChainData;
    const BITCOIN_ADDRESS: &str = "bc1puvuup6jctnk0v3e3qhvr69c73e8a90vytw06hfyn8ahhvf0l63hqd30tzd";

    #[tokio::test]
    async fn test_create_swap_uses_unused_taproot_swap() {
        let mut swap_storage = MockSwapStorage::new();
        let mut node_state_storage = MockNodeStateStorage::new();
        let completed_payment_storage = MockCompletedPaymentStorage::new();
        let node_state = NodeState {
            max_receivable_msat: 1_000_000_000,
            ..Default::default()
        };

        let node_state_clone = node_state.clone();
        // Setup persister expectations
        node_state_storage
            .expect_get_node_state()
            .return_once(move || Ok(Some(node_state_clone)));

        let unused_swap = SwapInfo {
            bitcoin_address: BITCOIN_ADDRESS.to_string(),
            max_allowed_deposit: 1_000_000,
            max_swapper_payable: 1_000_000,
            ..Default::default()
        };

        swap_storage
            .expect_list_swaps()
            .with(predicate::eq(ListSwapsRequest {
                status: Some(SwapStatus::unused()),
                ..Default::default()
            }))
            .return_once(move |_| Ok(vec![unused_swap.clone()]));

        swap_storage
            .expect_update_swap_fees()
            .return_once(|_, _| Ok(()));

        let swap = BTCReceiveSwap::new(BTCReceiveSwapParameters {
            chain_service: Arc::new(MockChainService::default()),
            completed_payment_storage: Arc::new(completed_payment_storage),
            network: Network::Bitcoin,
            node_api: Arc::new(MockNodeAPI::new(node_state)),
            node_state_storage: Arc::new(node_state_storage),
            payment_receiver: Arc::new(MockReceiver::default()),
            segwit_swapper_api: Arc::new(MockSwapperAPI {}),
            swap_storage: Arc::new(swap_storage),
            taproot_swapper_api: Arc::new(MockTaprootSwapperAPI::new()),
        });

        let opening_fee_params = OpeningFeeParams {
            min_msat: 1000,
            ..Default::default()
        };

        let result = swap.create_swap(opening_fee_params).await.unwrap();

        assert_eq!(&result.bitcoin_address, BITCOIN_ADDRESS);
        assert_eq!(result.max_allowed_deposit, 1_000_000);
    }

    #[tokio::test]
    async fn test_create_swap_uses_unused_taproot_swap_new_balance() {
        let mut swap_storage = MockSwapStorage::new();
        let mut node_state_storage = MockNodeStateStorage::new();
        let completed_payment_storage = MockCompletedPaymentStorage::new();
        let node_state = NodeState {
            max_receivable_msat: 100_000_000,
            ..Default::default()
        };

        let node_state_clone = node_state.clone();
        // Setup persister expectations
        node_state_storage
            .expect_get_node_state()
            .return_once(move || Ok(Some(node_state_clone)));

        let unused_swap = SwapInfo {
            bitcoin_address: BITCOIN_ADDRESS.to_string(),
            max_allowed_deposit: 1_000_000,
            max_swapper_payable: 1_000_000,
            ..Default::default()
        };

        swap_storage
            .expect_list_swaps()
            .with(predicate::eq(ListSwapsRequest {
                status: Some(SwapStatus::unused()),
                ..Default::default()
            }))
            .return_once(move |_| Ok(vec![unused_swap.clone()]));

        swap_storage
            .expect_update_swap_max_allowed_deposit()
            .return_once(|_, _| Ok(()));
        swap_storage
            .expect_update_swap_fees()
            .return_once(|_, _| Ok(()));

        let swap = BTCReceiveSwap::new(BTCReceiveSwapParameters {
            chain_service: Arc::new(MockChainService::default()),
            completed_payment_storage: Arc::new(completed_payment_storage),
            network: Network::Bitcoin,
            node_api: Arc::new(MockNodeAPI::new(node_state)),
            node_state_storage: Arc::new(node_state_storage),
            payment_receiver: Arc::new(MockReceiver::default()),
            segwit_swapper_api: Arc::new(MockSwapperAPI {}),
            swap_storage: Arc::new(swap_storage),
            taproot_swapper_api: Arc::new(MockTaprootSwapperAPI::new()),
        });

        let opening_fee_params = OpeningFeeParams {
            min_msat: 1000,
            ..Default::default()
        };

        let result = swap.create_swap(opening_fee_params).await.unwrap();

        assert_eq!(&result.bitcoin_address, BITCOIN_ADDRESS);
        assert_eq!(result.max_allowed_deposit, 100_000);
    }

    #[tokio::test]
    async fn test_create_swap_creates_new_when_no_unused() {
        let mut swap_storage = MockSwapStorage::new();
        let mut node_state_storage = MockNodeStateStorage::new();
        let completed_payment_storage = MockCompletedPaymentStorage::new();
        let node_state = NodeState {
            max_receivable_msat: 100_000_000,
            ..Default::default()
        };

        let node_state_clone = node_state.clone();
        // Setup persister expectations
        node_state_storage
            .expect_get_node_state()
            .return_once(move || Ok(Some(node_state_clone)));

        swap_storage
            .expect_list_swaps()
            .return_once(move |_| Ok(vec![]));

        swap_storage
            .expect_insert_swap()
            .return_once(move |_| Ok(()));

        // Setup server

        let swap = BTCReceiveSwap::new(BTCReceiveSwapParameters {
            chain_service: Arc::new(MockChainService::default()),
            completed_payment_storage: Arc::new(completed_payment_storage),
            network: Network::Bitcoin,
            node_api: Arc::new(MockNodeAPI::new(node_state)),
            node_state_storage: Arc::new(node_state_storage),
            payment_receiver: Arc::new(MockReceiver::default()),
            segwit_swapper_api: Arc::new(MockSwapperAPI {}),
            swap_storage: Arc::new(swap_storage),
            taproot_swapper_api: Arc::new(MockBreezServer {}),
        });

        let opening_fee_params = OpeningFeeParams {
            min_msat: 1000,
            ..Default::default()
        };

        let result = swap.create_swap(opening_fee_params).await.unwrap();

        assert_eq!(result.max_allowed_deposit, 100_000);
    }

    async fn test_swap_state_transition(
        swap_info: &SwapInfo,
        chain_data: &SwapChainData,
        payment: Option<Payment>,
        current_tip: u32,
    ) -> SwapInfo {
        let mut chain_service = MockChainService::default();
        chain_service.address_to_transactions.clear();
        chain_service.address_to_transactions.insert(
            swap_info.bitcoin_address.clone(),
            chain_data
                .outputs
                .iter()
                .map(|o| OnchainTx {
                    vout: vec![Vout {
                        scriptpubkey_address: swap_info.bitcoin_address.clone(),
                        value: o.amount_sat,
                        ..Default::default()
                    }],
                    status: TxStatus {
                        block_hash: o.block_hash.clone(),
                        block_height: o.confirmed_at_height,
                        confirmed: o.confirmed_at_height.is_some(),
                        ..Default::default()
                    },
                    txid: o.tx_id.clone(),
                    ..Default::default()
                })
                .collect(),
        );
        chain_service
            .address_to_transactions
            .get_mut(&swap_info.bitcoin_address)
            .unwrap()
            .append(
                &mut chain_data
                    .outputs
                    .iter()
                    .filter(|o| o.spend.is_some())
                    .map(|o| OnchainTx {
                        vin: vec![Vin {
                            prevout: Vout {
                                scriptpubkey_address: swap_info.bitcoin_address.clone(),
                                value: o.amount_sat,
                                ..Default::default()
                            },
                            txid: o.tx_id.clone(),
                            vout: o.output_index,
                            ..Default::default()
                        }],
                        status: TxStatus {
                            block_hash: o.spend.as_ref().unwrap().block_hash.clone(),
                            block_height: o.spend.as_ref().unwrap().confirmed_at_height,
                            confirmed: o.spend.as_ref().unwrap().confirmed_at_height.is_some(),
                            ..Default::default()
                        },
                        txid: o.spend.as_ref().unwrap().tx_id.clone(),
                        ..Default::default()
                    })
                    .collect(),
            );

        let swap_clone = swap_info.clone();
        let mut completed_payment_storage = MockCompletedPaymentStorage::new();
        completed_payment_storage
            .expect_get_completed_payment_by_hash()
            .returning(move |_| Ok(payment.clone()));
        let mut swap_storage = MockSwapStorage::new();
        swap_storage
            .expect_get_swap_chain_data()
            .returning(|_| Ok(None));
        swap_storage
            .expect_set_swap_chain_data()
            .returning(|_, _, _| Ok(()));
        swap_storage
            .expect_set_swap_status()
            .returning(|_, _| Ok(()));
        swap_storage
            .expect_get_swap_info_by_address()
            .returning(move |_| Ok(Some(swap_clone.clone())));
        swap_storage
            .expect_update_swap_paid_amount()
            .returning(|_, _| Ok(()));
        let node_state = NodeState {
            max_receivable_msat: 100_000_000,
            ..Default::default()
        };

        let swapper = BTCReceiveSwap::new(BTCReceiveSwapParameters {
            chain_service: Arc::new(chain_service),
            completed_payment_storage: Arc::new(completed_payment_storage),
            network: Network::Bitcoin,
            node_api: Arc::new(MockNodeAPI::new(node_state)),
            node_state_storage: Arc::new(MockNodeStateStorage::new()),
            payment_receiver: Arc::new(MockReceiver::default()),
            segwit_swapper_api: Arc::new(MockSwapperAPI {}),
            swap_storage: Arc::new(swap_storage),
            taproot_swapper_api: Arc::new(MockBreezServer {}),
        });
        let _receiver = swapper.subscribe_status_changes();

        swapper.refresh_swap(swap_info, current_tip).await.unwrap()
    }

    #[tokio::test]
    async fn test_swap_state_transitions() {
        let swap = SwapInfo {
            lock_height: 288,
            bitcoin_address: BITCOIN_ADDRESS.to_string(),
            min_allowed_deposit: 1_000,
            max_allowed_deposit: 1_000_000,
            ..Default::default()
        };

        let result = test_swap_state_transition(&swap, &SwapChainData::default(), None, 1).await;
        assert_eq!(result.status, SwapStatus::Initial);

        let result = test_swap_state_transition(
            &swap,
            &SwapChainData {
                outputs: vec![SwapOutput {
                    address: swap.bitcoin_address.clone(),
                    tx_id: "tx1".to_string(),
                    amount_sat: 1_000_000,
                    confirmed_at_height: None,
                    ..Default::default()
                }],
            },
            None,
            1,
        )
        .await;
        assert_eq!(result.status, SwapStatus::WaitingConfirmation);

        let result = test_swap_state_transition(
            &swap,
            &SwapChainData {
                outputs: vec![SwapOutput {
                    address: swap.bitcoin_address.clone(),
                    tx_id: "tx1".to_string(),
                    amount_sat: 1_000_000,
                    confirmed_at_height: Some(1),
                    ..Default::default()
                }],
            },
            None,
            1,
        )
        .await;
        assert_eq!(result.status, SwapStatus::Redeemable);

        let result = test_swap_state_transition(
            &swap,
            &SwapChainData {
                outputs: vec![SwapOutput {
                    address: swap.bitcoin_address.clone(),
                    tx_id: "tx1".to_string(),
                    amount_sat: 999,
                    confirmed_at_height: Some(1),
                    ..Default::default()
                }],
            },
            None,
            1,
        )
        .await;
        assert_eq!(result.status, SwapStatus::Refundable);

        let result = test_swap_state_transition(
            &swap,
            &SwapChainData {
                outputs: vec![SwapOutput {
                    address: swap.bitcoin_address.clone(),
                    tx_id: "tx1".to_string(),
                    amount_sat: 1_000_001,
                    confirmed_at_height: Some(1),
                    ..Default::default()
                }],
            },
            None,
            1,
        )
        .await;
        assert_eq!(result.status, SwapStatus::Refundable);

        let result = test_swap_state_transition(
            &swap,
            &SwapChainData {
                outputs: vec![SwapOutput {
                    address: swap.bitcoin_address.clone(),
                    tx_id: "tx1".to_string(),
                    amount_sat: 1_000_000,
                    confirmed_at_height: Some(1),
                    ..Default::default()
                }],
            },
            None,
            361,
        )
        .await;
        assert_eq!(result.status, SwapStatus::Refundable);

        let result = test_swap_state_transition(
            &swap,
            &SwapChainData {
                outputs: vec![SwapOutput {
                    address: swap.bitcoin_address.clone(),
                    tx_id: "tx1".to_string(),
                    amount_sat: 1_000_000,
                    confirmed_at_height: Some(1),
                    ..Default::default()
                }],
            },
            Some(Payment {
                amount_msat: 1_000_000,
                ..Default::default()
            }),
            361,
        )
        .await;
        assert_eq!(result.status, SwapStatus::Completed);

        let result = test_swap_state_transition(
            &swap,
            &SwapChainData {
                outputs: vec![
                    SwapOutput {
                        address: swap.bitcoin_address.clone(),
                        tx_id: "tx1".to_string(),
                        amount_sat: 1_000_000,
                        confirmed_at_height: Some(1),
                        ..Default::default()
                    },
                    SwapOutput {
                        address: swap.bitcoin_address.clone(),
                        tx_id: "tx2".to_string(),
                        amount_sat: 1_000_000,
                        confirmed_at_height: Some(2),
                        ..Default::default()
                    },
                ],
            },
            Some(Payment {
                amount_msat: 1_000_000,
                ..Default::default()
            }),
            1,
        )
        .await;
        assert_eq!(result.status, SwapStatus::Refundable);

        let result = test_swap_state_transition(
            &swap,
            &SwapChainData {
                outputs: vec![
                    SwapOutput {
                        address: swap.bitcoin_address.clone(),
                        tx_id: "tx1".to_string(),
                        amount_sat: 1_000_000,
                        confirmed_at_height: Some(1),
                        ..Default::default()
                    },
                    SwapOutput {
                        address: swap.bitcoin_address.clone(),
                        tx_id: "tx2".to_string(),
                        amount_sat: 1_000_000,
                        confirmed_at_height: None,
                        ..Default::default()
                    },
                ],
            },
            Some(Payment {
                amount_msat: 1_000_000,
                ..Default::default()
            }),
            1,
        )
        .await;
        assert_eq!(result.status, SwapStatus::Refundable);

        let result = test_swap_state_transition(
            &swap,
            &SwapChainData {
                outputs: vec![SwapOutput {
                    address: swap.bitcoin_address.clone(),
                    tx_id: "tx1".to_string(),
                    output_index: 0,
                    amount_sat: 1_000_000,
                    confirmed_at_height: Some(1),
                    spend: Some(SwapSpend {
                        confirmed_at_height: None,
                        block_hash: None,
                        tx_id: "tx1".to_string(),
                        output_index: 0,
                        spending_tx_id: "tx2".to_string(),
                        spending_input_index: 0,
                    }),
                    ..Default::default()
                }],
            },
            None,
            1,
        )
        .await;
        assert_eq!(result.status, SwapStatus::Refundable);

        let result = test_swap_state_transition(
            &swap,
            &SwapChainData {
                outputs: vec![SwapOutput {
                    address: swap.bitcoin_address.clone(),
                    tx_id: "tx1".to_string(),
                    output_index: 0,
                    amount_sat: 1_000_000,
                    confirmed_at_height: Some(1),
                    spend: Some(SwapSpend {
                        confirmed_at_height: Some(1),
                        block_hash: Some("hash".to_string()),
                        tx_id: "tx1".to_string(),
                        output_index: 0,
                        spending_tx_id: "tx2".to_string(),
                        spending_input_index: 0,
                    }),
                    ..Default::default()
                }],
            },
            None,
            1,
        )
        .await;
        assert_eq!(result.status, SwapStatus::Completed);
    }
}
