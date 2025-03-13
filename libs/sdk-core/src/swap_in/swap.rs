use std::{collections::HashMap, sync::Arc};

use gl_client::{
    bitcoin::{
        blockdata::constants::WITNESS_SCALE_FACTOR, consensus::encode, Address, AddressType, OutPoint, Script, Sequence, TxIn, Witness
    },
    lightning_invoice::Bolt11Invoice,
};
use tokio::sync::broadcast;

use crate::{
    breez_services::{OpenChannelParams, Receiver},
    chain::ChainService,
    error::ReceivePaymentError,
    node_api::FetchBolt11Result,
    persist::db::SqliteStorage,
    BreezEvent, ListSwapsRequest, OpeningFeeParams, PrepareRefundRequest, PrepareRefundResponse,
    ReceivePaymentRequest, RefundRequest, RefundResponse, SegwitSwapperAPI, SwapInfo, SwapStatus,
};

use super::{
    error::{GetPaymentRequestError, ReceiveSwapError, ReceiveSwapResult},
    segwit::SegwitReceiveSwap,
    taproot::TaprootReceiveSwap,
};

const EXPIRY_SECONDS_PER_BLOCK: u32 = 600;
const MIN_INVOICE_EXPIRY_SECONDS: u64 = 1800;
const MIN_OPENING_FEE_PARAMS_VALIDITY_SECONDS: u32 = 1800;

enum SwapAddress {
    Segwit(String),
    Taproot(String),
}

pub(crate) struct SwapChainData {
    pub outputs: Vec<SwapOutput>,
}

impl SwapChainData {
    pub fn compare(&self, other: &SwapChainData) -> bool {
        for output in self.outputs {
            if !other.outputs.contains(&output) {
                return false;
            }
        }

        for output in other.outputs {
            if !self.outputs.contains(&output) {
                return false;
            }
        }

        true
    }

    pub fn confirmed_outputs(&self) -> Vec<&SwapOutput> {
        self.outputs
            .iter()
            .filter(|o| o.confirmed_at_height.is_some())
            .collect()
    }

    pub fn confirmed_utxos(&self) -> Vec<&SwapOutput> {
        self.outputs
            .iter()
            .filter(|o| o.spend.is_none() && o.confirmed_at_height.is_some())
            .collect()
    }

    pub fn unconfirmed_utxos(&self) -> Vec<&SwapOutput> {
        self.outputs
            .iter()
            .filter(|o| o.spend.is_none() && o.confirmed_at_height.is_none())
            .collect()
    }

    pub fn utxos(&self) -> Vec<&SwapOutput> {
        self.outputs.iter().filter(|o| o.spend.is_none()).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SwapOutput {
    pub address: String,
    pub amount_sat: u64,
    pub tx_id: String,
    pub output_index: u32,
    pub confirmed_at_height: Option<u32>,
    pub block_hash: Option<String>,
    pub spend: Option<SwapSpend>,
}

impl TryInto<TxIn> for SwapOutput {
    type Error = ReceiveSwapError;

    fn try_into(self) -> Result<TxIn, Self::Error> {
        let previous_output = OutPoint {
            txid: self.tx_id.parse()?,
            vout: self.output_index,
        };
        let address: Address = self.address.parse()?;
        Ok(TxIn {
            previous_output,
            script_sig: Script::default(),
            sequence: Sequence::default(),
            witness: Witness::default(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SwapSpend {
    pub tx_id: String,
    pub output_index: u32,
    pub spending_tx_id: String,
    pub spending_input_index: u32,
    pub confirmed_at_height: Option<u32>,
    pub block_hash: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct SwapRefund {
    pub refund_tx_id: String,
    pub spent_tx_id: String,
    pub spent_output_index: u32,
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

pub(crate) struct ReceiveSwap {
    chain_service: Arc<dyn ChainService>,
    persister: Arc<SqliteStorage>,
    segwit: SegwitReceiveSwap,
    status_changes_notifier: broadcast::Sender<BreezEvent>,
    taproot: TaprootReceiveSwap,
}

impl ReceiveSwap {
    pub(crate) fn new(
        chain_service: Arc<dyn ChainService>,
        payment_receiver: Arc<dyn Receiver>,
        persister: Arc<SqliteStorage>,
        swapper_api: Arc<dyn SegwitSwapperAPI>,
    ) -> Self {
        ReceiveSwap {
            segwit: SegwitReceiveSwap::new(),
            taproot: TaprootReceiveSwap::new(),
            chain_service,
            persister,
            status_changes_notifier: broadcast::channel(100).0,
        }
    }
}

impl ReceiveSwap {
    pub(crate) async fn create_swap_address(
        &self,
        opening_fee_params: OpeningFeeParams,
    ) -> ReceiveSwapResult<String> {
        self.taproot.create_swap_address(opening_fee_params).await
    }

    pub(crate) async fn list_swaps(
        &self,
        req: ListSwapsRequest,
    ) -> ReceiveSwapResult<Vec<SwapInfo>> {
        self.persister.list_swaps(req)?
    }

    pub(crate) async fn list_in_progress_swaps(&self) -> ReceiveSwapResult<Vec<SwapInfo>> {
        self.list_swaps(ListSwapsRequest {
            status: Some(SwapStatus::in_progress()),
            ..Default::default()
        })
        .await
    }

    pub(crate) async fn list_refundables(&self) -> ReceiveSwapResult<Vec<SwapInfo>> {
        self.list_swaps(ListSwapsRequest {
            status: Some(SwapStatus::refundable()),
            ..Default::default()
        })
        .await
    }

    pub(crate) async fn prepare_refund(
        &self,
        req: PrepareRefundRequest,
    ) -> ReceiveSwapResult<PrepareRefundResponse> {
        let address = parse_address(&req.address)?;
        let swap_info = self
            .persister
            .get_swap_info_by_address(req.swap_address.clone())?
            .ok_or(ReceiveSwapError::SwapNotFound)?;
        let chain_data = self.persister.get_swap_chain_data(&req.swap_address)?;
        let utxos: Vec<_> = chain_data.confirmed_utxos();
        if utxos.is_empty() {
            return Err(ReceiveSwapError::NoUtxos);
        }

        // Sort UTXOs for deterministic transactions
        utxos.sort_by(|a, b| {
            a.tx_id
                .cmp(&b.tx_id)
                .then(a.output_index.cmp(&b.output_index))
        });

        let tx = match address {
            SwapAddress::Segwit(_) => self.segwit.create_fake_refund_tx(&swap_info, &utxos).await,
            SwapAddress::Taproot(_) => match req.unilateral {
                Some(true) => {
                    self.taproot
                        .create_fake_unilateral_refund_tx(&swap_info, &utxos)
                        .await
                }
                _ => {
                    self.taproot
                        .create_fake_cooperative_refund_tx(&swap_info, &utxos)
                        .await
                }
            },
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

    pub(crate) async fn refund(&self, req: RefundRequest) -> ReceiveSwapResult<RefundResponse> {
        let address = parse_address(&req.address)?;
        let swap_info = self
            .persister
            .get_swap_info_by_address(req.swap_address.clone())?
            .ok_or(ReceiveSwapError::SwapNotFound)?;
        let outputs = self.persister.get_swap_outputs(req.swap_address.clone())?;
        let utxos: Vec<_> = outputs.into_iter().filter(|o| o.spend.is_none()).collect();
        if utxos.is_empty() {
            return Err(ReceiveSwapError::NoUtxos);
        }

        // Sort UTXOs for deterministic transactions
        utxos.sort_by(|a, b| {
            a.tx_id
                .cmp(&b.tx_id)
                .then(a.output_index.cmp(&b.output_index))
        });

        let tx = match address {
            SwapAddress::Segwit(_) => self.segwit.create_refund_tx(&swap_info, &utxos).await,
            SwapAddress::Taproot(_) => match req.unilateral {
                Some(true) => {
                    self.taproot
                        .create_unilateral_refund_tx(&swap_info, &utxos)
                        .await
                }
                _ => {
                    self.taproot
                        .create_cooperative_refund_tx(&swap_info, &utxos)
                        .await
                }
            },
        };

        let refund_tx = encode::serialize(&tx);
        info!("broadcasting refund tx {:?}", hex::encode(&refund_tx));
        let tx_id = self.chain_service.broadcast_transaction(refund_tx).await?;
        self.persister
            .insert_swap_refund_tx_ids(swap_info.bitcoin_address, tx_id.clone())?;
        self.emit_swap_updated(&req.swap_address)?;

        Ok(RefundResponse {
            refund_tx_id: tx_id,
        })
    }

    pub(crate) async fn redeem_swap(&self, address: String) -> ReceiveSwapResult<()> {
        // TODO: Check whether this should use the full swap data.
        let swap_info = self
            .persister
            .get_swap_info_by_address(address)?
            .ok_or(ReceiveSwapError::SwapNotFound)?;
        let current_tip = self.chain_service.current_tip(true).await?;
        let (payment_request, is_new_payment_request) =
            self.get_payment_request(&swap_info, current_tip).await?;
        self.persister
            .update_swap_bolt11(address.clone(), payment_request.clone())?;
        if is_new_payment_request {
            self.emit_swap_updated(&swap_info.address)?;
        }
        let resp = match address {
            SwapAddress::Segwit(_) => {
                self.segwit
                    .get_swap_payment(&swap_info, payment_request)
                    .await
            }
            SwapAddress::Taproot(_) => {
                self.taproot
                    .get_swap_payment(&swap_info, payment_request)
                    .await
            }
        };

        let error_message = match resp {
            Ok(_) => {
                // Nothing to do here. Swap updated event will be emitted by the invoice paid event.
                return Ok(());
            }
            Err(message) => message,
        };

        debug!("Error getting paid for swap: {}", error_message);
        self.persister
            .update_swap_redeem_error(&swap_info.address, &error_message)?;
        self.emit_swap_updated(&swap_info.address)?;
        Err(ReceiveSwapError::PaymentError(error_message))
    }

    pub(crate) async fn rescan_swaps(&self, tip: u32) -> ReceiveSwapResult<()> {
        self.refresh_swaps(self.persister.list_swaps(ListSwapsRequest::default())?, tip)
            .await
    }

    pub(crate) async fn rescan_monitored_swaps(&self, tip: u32) -> ReceiveSwapResult<()> {
        self.refresh_swaps(self.list_monitored()?, tip).await
    }
}

/// ReceiveSwapper private functions
impl ReceiveSwap {
    async fn calculate_status(
        &self,
        swap_info: &SwapInfo,
        address: &SwapAddress,
        chain_data: &Option<SwapChainData>,
        current_tip: u32,
    ) -> SwapStatus {
        let chain_data = match chain_data {
            Some(cd) => cd,
            None => return self.calculate_status_without_chain_data(swap_info, address, current_tip),
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

        let payout_blocks_left = match address {
            SwapAddress::Segwit(_) => {
                self.segwit
                    .payout_blocks_left(swap_info, min_confirmation, current_tip)
            }
            SwapAddress::Taproot(_) => {
                self.taproot
                    .payout_blocks_left(swap_info, min_confirmation, current_tip)
            }
        };

        // If there are blocks left to be paid out and the swap was not redeemed yet, it is redeemable.
        if payout_blocks_left > 0 && swap_info.paid_msat == 0 {
            return SwapStatus::Redeemable;
        }

        // The swap is not redeemable. And there are confirmed or unconfirmed outputs.

        // Deduce the paid outputs by assuming the first confirmed outputs are the ones belonging to the payment.
        let all_outputs = chain_data.outputs.clone();
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

        let refundable_utxos = chain_data
            .utxos()
            .into_iter()
            .filter(|o| {
                paid_outputs
                    .iter()
                    .all(|po| po.tx_id != o.tx_id || po.output_index != o.output_index)
            })
            .collect();

        if refundable_utxos.is_empty() {
            return SwapStatus::Completed;
        }

        SwapStatus::Refundable
    }

    fn calculate_status_without_chain_data(
        &self,
        swap_info: &SwapInfo,
        address: &SwapAddress,
        current_tip: u32,
    ) -> SwapStatus {
        let mut passed_timelock = false;
        if let Some(confirmed_at) = self.confirmed_at {
            let payout_blocks_left = match address {
                SwapAddress::Segwit(_) => {
                    self.segwit
                        .payout_blocks_left(swap_info, confirmed_at, current_tip)
                }
                SwapAddress::Taproot(_) => {
                    self.taproot
                        .payout_blocks_left(swap_info, confirmed_at, current_tip)
                }
            };
            passed_timelock = payout_blocks_left <= 0;
        }

        // In case timelock has passed we can only be in the Refundable or Completed state.
        if passed_timelock {
            return match self.confirmed_sats {
                0 => SwapStatus::Completed,
                // This is to make sure we don't consider refundable in case we only have one transaction which was already
                // paid by the swapper.
                _ => match (self.paid_msat, self.total_incoming_txs) {
                    (paid, 1) if paid > 0 => SwapStatus::Completed,
                    _ => SwapStatus::Refundable,
                },
            };
        }

        match (
            self.confirmed_at,
            self.unconfirmed_sats,
            self.confirmed_sats,
            self.paid_msat,
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
        swap: &SwapInfo,
        bolt11_result: FetchBolt11Result,
    ) -> Result<Option<String>, GetPaymentRequestError> {
        let invoice: Bolt11Invoice = bolt11_result.bolt11.parse()?;
        let invoice_expires_at = match invoice.expires_at() {
            Some(expires_at) => expires_at,
            None => {
                debug!(
                    "Existing swap payment request has invalid expiry. Recreating payment request."
                );
                self.delete_invoice(swap, bolt11_result.bolt11).await?;
                return Ok(None);
            }
        };
        if invoice_expires_at.as_secs() < MIN_INVOICE_EXPIRY_SECONDS {
            debug!("Existing swap payment request has expired / will expire soon. Recreating payment request.");
            self.delete_invoice(swap, bolt11_result.bolt11).await?;
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
        if amount_msat != swap.confirmed_unspent_amount_msat() {
            debug!("Existing swap payment request amount is no longer correct. Recreating payment request.");
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
            debug!("Existing swap payment request is not an open channel invoice, but liquidity is no longer sufficient. Recreating payment request.");
            self.delete_invoice(swap, bolt11_result.bolt11).await?;
            return Ok(None);
        }

        Ok(Some(bolt11_result.bolt11))
    }

    async fn fetch_swap_onchain_data(
        &self,
        swap: &SwapInfo,
    ) -> ReceiveSwapResult<SwapChainData> {
        let txs = self
            .chain_service
            .address_transactions(swap.bitcoin_address.clone())
            .await?;

        let mut outputs = HashMap::new();

        // Collect all outputs that were sent to the swap address
        for tx in txs {
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
        for tx in txs {
            for (input_index, vin) in tx.vin.iter().enumerate() {
                let outpoint = format!("{}:{}", vin.txid, vin.vout);
                if let Some(mut output) = outputs.get_mut(&outpoint) {
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

    async fn get_payment_request(
        &self,
        swap: &SwapInfo,
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
        swap: &SwapInfo,
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
        let blocks_left = swap
            .blocks_left(current_tip)
            .ok_or(GetPaymentRequestError::generic(
                "Cannot create payment request for unconfirmed swap",
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
                // TODO: Substract fees here once swapper supports them.
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
    ) -> ReceiveSwapResult<()> {
        let (new_swap_info, new_chain_data) =
            match self.refresh_swap_onchain_data(swap_info, current_tip).await {
                Ok(s) => &s,
                Err(e) => {
                    error!(
                        "failed to refresh swap onchain status for address {}: {}",
                        swap_info.bitcoin_address, e
                    );
                    swap_info
                }
            };

        new_swap_info = match self
            .refresh_swap_payment_data(&new_swap_info, current_tip)
            .await
        {
            Ok(s) => &s,
            Err(e) => {
                error!(
                    "failed to refresh swap payment status for address {}: {}",
                    swap_info.bitcoin_address, e
                );
                new_swap_info
            }
        };

        if new_swap_info != swap_info {
            let address = parse_address(&swap_info.bitcoin_address)?;
            let status = self
                .calculate_status(swap_info, &address, &new_chain_data, current_tip)
                .await?;
            self.persister
                .set_swap_status(&swap_info.bitcoin_address, &status)?;
            self.emit_swap_updated(&swap_info.bitcoin_address)?;
        }

        Ok(())
    }

    async fn refresh_swaps(&self, swaps: Vec<SwapInfo>, tip: u32) -> ReceiveSwapResult<()> {
        for s in swaps {
            self.refresh_swap(&s, tip).await?;
        }
        Ok(())
    }

    async fn refresh_swap_onchain_data(
        &self,
        swap_info: &SwapInfo,
        current_tip: u32,
    ) -> ReceiveSwapResult<(SwapInfo, Option<SwapChainData>)> {
        let existing_chain_data = self
            .persister
            .get_swap_chain_data(&swap_info.bitcoin_address)?;
        let new_chain_data = match self.fetch_swap_onchain_data(swap_info).await {
            Ok(d) => d,
            Err(e) => {
                error!(
                    "failed to refresh swap onchain status for address {}: {}",
                    swap_info.bitcoin_address, e
                );
                Ok((swap_info.clone(), existing_chain_data))
            }
        };
        let changed = new_chain_data != existing_chain_data;
        let chain_info = new_chain_data.into();
        if changed {
            self.persister.set_swap_chain_data(
                &swap_info.bitcoin_address,
                &new_chain_data,
                &chain_info,
            )?;
        }
        let chain_info: SwapChainInfo = new_chain_data.into();
        Ok(SwapInfo {
            confirmed_at: chain_info.confirmed_at,
            confirmed_sats: chain_info.confirmed_sats,
            confirmed_tx_ids: chain_info.confirmed_tx_ids,
            total_incoming_txs: chain_info.total_incoming_txs,
            unconfirmed_sats: chain_info.unconfirmed_sats,
            unconfirmed_tx_ids: chain_info.unconfirmed_tx_ids,
            ..swap_info.clone()
        })
    }

    async fn refresh_swap_payment_data(
        &self,
        swap_info: &SwapInfo,
        current_tip: u32,
    ) -> ReceiveSwapResult<SwapInfo> {
        let payment = self
            .persister
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
        let mut swap_info = SwapInfo {
            paid_msat: amount_msat,
            ..swap_info.clone()
        };

        if amount_msat != swap_info.paid_msat {
            self.persister
                .update_swap_paid_amount(swap_info.bitcoin_address.clone(), amount_msat)?;
        }

        Ok(SwapInfo {
            paid_msat: amount_msat,
            ..swap_info.clone()
        })
    }
}

fn parse_address(address: &str) -> ReceiveSwapResult<SwapAddress> {
    let address: Address = address.parse()?;
    match address.address_type() {
        Some(AddressType::P2tr) => Ok(SwapAddress::Taproot(address.to_string())),
        Some(AddressType::P2wsh) => Ok(SwapAddress::Segwit(address.to_string())),
        _ => Err(ReceiveSwapError::InvalidAddressType),
    }
}
