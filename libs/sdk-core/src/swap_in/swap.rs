use std::{collections::HashMap, sync::Arc};

use gl_client::{bitcoin::{blockdata::constants::WITNESS_SCALE_FACTOR, consensus::encode, Address, AddressType}, lightning::chain, lightning_invoice::Bolt11Invoice};
use tokio::sync::broadcast;

use crate::{breez_services::{OpenChannelParams, Receiver}, chain::{ChainService, OnchainTx}, error::ReceivePaymentError, node_api::FetchBolt11Result, persist::db::SqliteStorage, BreezEvent, ListSwapsRequest, OpeningFeeParams, PrepareRefundRequest, PrepareRefundResponse, ReceivePaymentRequest, RefundRequest, RefundResponse, SegwitSwapperAPI, SwapInfo, SwapStatus};

use super::{error::{GetPaymentRequestError, ReceiveSwapperError, ReceiveSwapperResult}, segwit::swap::SegwitReceiveSwap, taproot::TaprootReceiveSwap};

const EXPIRY_SECONDS_PER_BLOCK: u32 = 600;
const MIN_INVOICE_EXPIRY_SECONDS: u64 = 1800;
const MIN_OPENING_FEE_PARAMS_VALIDITY_SECONDS: u32 = 1800;

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

    pub fn confirmed_utxos(&self) -> Vec<&SwapOutput> {
        self.outputs.iter().filter(|o|o.spend.is_none() && o.confirmed_at_height.is_some()).collect()
    }

    pub fn utxos(&self) -> Vec<&SwapOutput> {
        self.outputs.iter().filter(|o|o.spend.is_none()).collect()
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

pub(crate) struct ReceiveSwapper {
    chain_service: Arc<dyn ChainService>,
    persister: Arc<SqliteStorage>,
    // segwit: SegwitReceiveSwap,
    status_changes_notifier: broadcast::Sender<BreezEvent>,
    // taproot: TaprootReceiveSwap,
}

impl ReceiveSwapper {
    pub(crate) fn new(chain_service: Arc<dyn ChainService>, payment_receiver: Arc<dyn Receiver>, persister: Arc<SqliteStorage>, swapper_api: Arc<dyn SegwitSwapperAPI>) -> Self {
        ReceiveSwapper {
            chain_service,
            persister,
            status_changes_notifier: broadcast::channel(100).0,
        }
    }
}

impl ReceiveSwapper {
    pub(crate) async fn create_swap_address(&self, opening_fee_params: OpeningFeeParams) -> ReceiveSwapperResult<String> {
        self.taproot.create_swap_address(opening_fee_params).await
    }

    pub(crate) async fn list_swaps(&self, req: ListSwapsRequest) -> ReceiveSwapperResult<Vec<SwapInfo>> {
        self.persister.list_swaps(req)?
    }

    pub(crate) async fn list_in_progress_swaps(&self) -> ReceiveSwapperResult<Vec<SwapInfo>> {
        self.list_swaps(ListSwapsRequest {
            status: Some(SwapStatus::in_progress()),
            ..Default::default()
        }).await
    }

    pub(crate) async fn list_refundables(&self) -> ReceiveSwapperResult<Vec<SwapInfo>> {
        self.list_swaps(ListSwapsRequest {
            status: Some(SwapStatus::refundable()),
            ..Default::default()
        }).await
    }

    pub(crate) async fn prepare_refund(&self, req: PrepareRefundRequest) -> ReceiveSwapperResult<PrepareRefundResponse> {
        let address = parse_address(&req.address)?;
        let swap_info = self.persister.get_swap_info_by_address(req.swap_address.clone())?.ok_or(ReceiveSwapperError::SwapNotFound)?;
        let outputs = self.persister.get_swap_outputs(req.swap_address.clone())?;
        let utxos: Vec<_> = outputs.into_iter().filter(|o|o.spend.is_none()).collect();
        if utxos.is_empty() {
            return Err(ReceiveSwapperError::NoUtxos);
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
            }
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

    pub(crate) async fn refund(&self, req: RefundRequest) -> ReceiveSwapperResult<RefundResponse> {
        let address = parse_address(&req.address)?;
        let swap_info = self.persister.get_swap_info_by_address(req.swap_address.clone())?.ok_or(ReceiveSwapperError::SwapNotFound)?;
        let outputs = self.persister.get_swap_outputs(req.swap_address.clone())?;
        let utxos: Vec<_> = outputs.into_iter().filter(|o|o.spend.is_none()).collect();
        if utxos.is_empty() {
            return Err(ReceiveSwapperError::NoUtxos);
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
            }
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

    pub(crate) async fn redeem_swap(&self, address: String) -> ReceiveSwapperResult<()> {
        let swap_info = self.persister.get_swap_info_by_address(address)?.ok_or(ReceiveSwapperError::SwapNotFound)?;
        let current_tip = self.chain_service.current_tip(true).await?;
        let (payment_request, is_new_payment_request) =
            self.get_payment_request(&swap_info, current_tip).await?;
        self.persister
            .update_swap_bolt11(address.clone(), payment_request.clone())?;
        if is_new_payment_request {
            self.emit_swap_updated(&swap_info.address)?;
        }
        let resp = match address {
            SwapAddress::Segwit(_) => self.segwit.get_swap_payment(&swap_info, payment_request).await,
            SwapAddress::Taproot(_) => self.taproot.get_swap_payment(&swap_info, payment_request).await,
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
            .set_taproot_swap_last_payment_error(&swap_info.address, &error_message)?;
        self.emit_swap_updated(&swap_info.address)?;
        Err(ReceiveSwapperError::PaymentError(error_message))
    }

    pub(crate) async fn rescan_swaps(&self, tip: u32) -> ReceiveSwapperResult<()> {
        self.refresh_swaps(self.persister.list_swaps(ListSwapsRequest::default())?, tip)
            .await
    }

    pub(crate) async fn rescan_monitored_swaps(&self, tip: u32) -> ReceiveSwapperResult<()> {
        self.refresh_swaps(self.list_monitored()?, tip)
            .await
    }
}

/// ReceiveSwapper private functions
impl ReceiveSwapper {
    async fn check_existing_payment_request(
        &self,
        swap: &SwapInfo,
        bolt11_result: FetchBolt11Result,
    ) -> Result<Option<String>, GetPaymentRequestError> {
        let invoice: Bolt11Invoice = bolt11_result.bolt11.parse()?;
        let invoice_expires_at = match invoice.expires_at() {
            Some(expires_at) => expires_at,
            None => {
                debug!("Existing swap payment request has invalid expiry. Recreating payment request.");
                self.delete_invoice(swap, bolt11_result.bolt11).await?;
                return Ok(None);
            }
        };
        if invoice_expires_at.as_secs() < MIN_INVOICE_EXPIRY_SECONDS {
            debug!("Existing swap payment request has expired / will expire soon. Recreating payment request.");
            self.delete_invoice(swap, bolt11_result.bolt11).await?;
            return Ok(None);
        }
        let invoice_amount_msat = invoice
            .amount_milli_satoshis()
            .ok_or(GetPaymentRequestError::generic("Found swap invoice without amount"))?;
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
        let blocks_left = swap.blocks_left(current_tip).ok_or(GetPaymentRequestError::generic(
            "Cannot create payment request for unconfirmed swap"
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

    async fn refresh_swaps(&self, swaps: Vec<SwapInfo>, tip: u32) -> ReceiveSwapperResult<()> {
        for s in swaps {
            let address = s.bitcoin_address.clone();
            let result = self
                .refresh_swap_on_chain_status(address.clone(), tip)
                .await;
            if let Err(err) = result {
                error!("failed to refresh swap status for address {address}: {err}")
            }
        }
        Ok(())
    }

    async fn refresh_swap(&self, swap: &SwapInfo, tip: u32) -> ReceiveSwapperResult<()> {
        let chain_data = self.fetch_swap_onchain_data(swap, tip).await?;
        let existing_chain_data = self.persister.get_swap_chain_data(&swap.bitcoin_address)?;
        let changed = chain_data != existing_chain_data;
        if changed {
            self.persister.set_swap_chain_data(&swap.bitcoin_address, &chain_data)?;
            self.emit_swap_updated(&swap.bitcoin_address)?;
            // TODO: Set the swapinfo values
        }

        
    }

    async fn fetch_swap_onchain_data(&self, swap:&SwapInfo, tip: u32) -> ReceiveSwapperResult<SwapChainData> {
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

    /// refreshes the on-chain status of the swap. This method updates the following information
    /// on a SwapInfo and save it to the persistent storage:
    /// confirmed_sats - the number of unspent satoshis that were sent to this address
    /// confirmed_txs - all utxo that are sent to this address
    /// swap_status - Either Initial or Expired.
    pub(crate) async fn refresh_swap_on_chain_status(
        &self,
        bitcoin_address: String,
        current_tip: u32,
    ) -> ReceiveSwapperResult<SwapInfo> {
        let mut swap_info = self
            .persister
            .get_swap_info_by_address(bitcoin_address.clone())?
            .ok_or(ReceiveSwapperError::SwapNotFound(bitcoin_address.clone()))?;
        let txs = self
            .chain_service
            .address_transactions(bitcoin_address.clone())
            .await?;
        let maybe_min_confirmed_block = txs
            .clone()
            .into_iter()
            .filter_map(|t| t.status.block_height)
            .filter(|height| *height > 0)
            .min();

        let utxos = get_utxos(bitcoin_address.clone(), txs.clone(), false)?;
        let total_incoming_txs = get_total_incoming_txs(bitcoin_address.clone(), txs);

        debug!(
            "updating swap on-chain info {:?}: confirmed_sats={:?} refund_tx_ids={:?}, confirmed_tx_ids={:?}",
            bitcoin_address.clone(), utxos.confirmed_sats(), swap_info.refund_tx_ids, utxos.confirmed_tx_ids(),
        );

        let payment = self
            .persister
            .get_completed_payment_by_hash(&hex::encode(swap_info.payment_hash.clone()))?;
        if let Some(payment) = payment {
            debug!(
                "found payment for hash {:?}, {:?}",
                &hex::encode(swap_info.payment_hash.clone()),
                payment
            );
            let amount_msat = payment.amount_msat;
            swap_info = swap_info.with_paid_amount(amount_msat, current_tip);
            self.persister.update_swap_paid_amount(
                bitcoin_address.clone(),
                amount_msat,
                swap_info.status.clone(),
            )?;
        }

        let chain_info = SwapChainInfo {
            unconfirmed_sats: utxos.unconfirmed_sats(),
            unconfirmed_tx_ids: utxos.unconfirmed_tx_ids(),
            confirmed_sats: utxos.confirmed_sats(),
            confirmed_tx_ids: utxos.confirmed_tx_ids(),
            confirmed_at: maybe_min_confirmed_block,
            total_incoming_txs,
        };
        let status = swap_info
            .with_chain_info(chain_info.clone(), current_tip)
            .status;
        let updated = self
            .persister
            .update_swap_chain_info(bitcoin_address, chain_info, status)?;
        self.emit_swap_updated(&swap_info.bitcoin_address)?;
        Ok(updated)
    }
}


enum SwapAddress {
    Segwit(String),
    Taproot(String),
}

fn parse_address(address: &str) -> ReceiveSwapperResult<SwapAddress> {
    let address: Address = address.parse()?;
    match address.address_type() {
        Some(AddressType::P2tr) => Ok(SwapAddress::Taproot(address.to_string())),
        Some(AddressType::P2wsh) => Ok(SwapAddress::Segwit(address.to_string())),
        _ => Err(ReceiveSwapperError::InvalidAddressType),
    }
}