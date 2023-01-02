use crate::chain::{ChainService, MempoolSpace, RecommendedFees};
use crate::fiat::{FiatCurrency, Rate};
use crate::greenlight::Greenlight;
use crate::grpc::channel_opener_client::ChannelOpenerClient;
use crate::grpc::fund_manager_client::FundManagerClient;
use crate::grpc::information_client::InformationClient;
use crate::grpc::PaymentInformation;
use crate::input_parser::LnUrlPayRequestData;
use crate::invoice::{add_routing_hints, parse_invoice, LNInvoice, RouteHint, RouteHintHop};
use crate::lnurl::pay::model::{LnUrlPayResult, ValidatedCallbackResponse};
use crate::lnurl::pay::validate_lnurl_pay;
use crate::lnurl::withdraw::model::LnUrlWithdrawCallbackStatus;
use crate::lnurl::withdraw::validate_lnurl_withdraw;
use crate::lsp::LspInformation;
use crate::models::{
    parse_short_channel_id, ChannelState, ClosesChannelPaymentDetails, Config, FeeratePreset,
    FiatAPI, GreenlightCredentials, LspAPI, Network, NodeAPI, NodeState, Payment, PaymentDetails,
    PaymentType, PaymentTypeFilter, SwapInfo, SwapperAPI,
};
use crate::persist::db::SqliteStorage;
use crate::swap::BTCReceiveSwap;
use crate::{persist, LnUrlWithdrawRequestData};
use anyhow::{anyhow, Result};
use bip39::*;
use core::time;
use std::cmp::max;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tonic::codegen::InterceptedService;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
use tonic::transport::{Channel, Uri};
use tonic::{Request, Status};

pub trait EventListener: Send + Sync {
    fn on_event(&self, e: BreezEvent);
}

#[derive(Clone, Debug)]
pub enum BreezEvent {
    NewBlock { block: u32 },
    InvoicePaid { details: InvoicePaidDetails },
    Synced,
}

#[derive(Clone, Debug)]
pub struct InvoicePaidDetails {
    pub payment_hash: String,
    pub bolt11: String,
}

/// starts the BreezServices background threads.
pub(crate) async fn start(
    rt: &Runtime,
    breez_services: Arc<BreezServices>,
    mut shutdown_receiver: mpsc::Receiver<()>,
) -> Result<()> {
    // start the signer
    let (shutdown_signer_sender, signer_signer_receiver) = mpsc::channel(1);
    let signer_api = breez_services.clone();
    rt.spawn(async move {
        signer_api
            .node_api
            .start_signer(signer_signer_receiver)
            .await;
    });

    // sync with remote state
    breez_services.clone().sync().await?;

    // poll sdk events
    rt.spawn(async move {
        let current_block: u32 = 0;
        loop {
            tokio::select! {

              poll_result = poll_events(breez_services.clone(), current_block) => {
               match poll_result {
                Ok(()) => {
                 return;
                },
                Err(err) => {
                 debug!("poll_events returned with error: {:?} waiting...", err);
                 sleep(Duration::from_secs(1)).await;
                 continue
                }
               }
              },
              _ = shutdown_receiver.recv() => {
               _ = shutdown_signer_sender.send(()).await;
               debug!("Received the signal to exit the chain monitoring loop");
               return;
             }
            }
        }
    });

    Ok(())
}

/// BreezServices is a facade and the single entry point for the sdk use cases providing
/// by exposing a simplified API
pub struct BreezServices {
    config: Config,
    node_api: Arc<dyn NodeAPI>,
    lsp_api: Arc<dyn LspAPI>,
    fiat_api: Arc<dyn FiatAPI>,
    chain_service: Arc<dyn ChainService>,
    persister: Arc<persist::db::SqliteStorage>,
    payment_receiver: Arc<PaymentReceiver>,
    btc_receive_swapper: Arc<BTCReceiveSwap>,
    event_listener: Option<Box<dyn EventListener>>,
    shutdown_sender: Mutex<Option<mpsc::Sender<()>>>,
}

impl BreezServices {
    pub async fn register_node(network: Network, seed: Vec<u8>) -> Result<GreenlightCredentials> {
        let creds = Greenlight::register(network, seed.clone()).await?;
        Ok(creds)
    }

    pub async fn recover_node(network: Network, seed: Vec<u8>) -> Result<GreenlightCredentials> {
        let creds = Greenlight::recover(network, seed.clone()).await?;
        Ok(creds)
    }

    pub async fn init_services(
        config: Option<Config>,
        seed: Vec<u8>,
        creds: GreenlightCredentials,
        event_listener: Box<dyn EventListener>,
    ) -> Result<Arc<BreezServices>> {
        let sdk_config = config.unwrap_or(Config::default());

        // create the node services instance and set it globally
        let breez_services = BreezServicesBuilder::new(sdk_config.clone())
            .greenlight_credentials(creds, seed)
            .build(Some(event_listener))?;
        Ok(breez_services.clone())
    }

    pub async fn start(runtime: &Runtime, breez_services: &Arc<BreezServices>) -> Result<()> {
        // create a shutdown channel (sender and receiver)
        let (stop_sender, stop_receiver) = mpsc::channel(1);
        breez_services.set_shutdown_sender(stop_sender);

        crate::breez_services::start(&runtime, breez_services.clone(), stop_receiver).await
    }

    pub async fn stop(&self) -> Result<()> {
        let unlocked = self.shutdown_sender.lock().unwrap();
        if unlocked.is_none() {
            return Err(anyhow!("node has not been started"));
        }
        let sender = unlocked.as_ref().unwrap();
        sender.send(()).await.map_err(anyhow::Error::msg)
    }

    pub async fn send_payment(&self, bolt11: String, amount_sats: Option<u64>) -> Result<()> {
        self.start_node().await?;
        self.node_api.send_payment(bolt11, amount_sats).await?;
        self.sync().await?;
        Ok(())
    }

    pub async fn send_spontaneous_payment(&self, node_id: String, amount_sats: u64) -> Result<()> {
        self.start_node().await?;
        self.node_api
            .send_spontaneous_payment(node_id, amount_sats)
            .await?;
        self.sync().await?;
        Ok(())
    }

    pub async fn pay_lnurl(
        &self,
        user_amount_sat: u64,
        comment: Option<String>,
        req_data: LnUrlPayRequestData,
    ) -> Result<LnUrlPayResult> {
        match validate_lnurl_pay(user_amount_sat, comment, req_data).await? {
            ValidatedCallbackResponse::EndpointError { data: e } => {
                Ok(LnUrlPayResult::EndpointError { data: e })
            }
            ValidatedCallbackResponse::EndpointSuccess { data: cb } => {
                self.send_payment(cb.pr, None).await?;
                Ok(LnUrlPayResult::EndpointSuccess {
                    data: cb.success_action,
                })
            }
        }
    }

    pub async fn withdraw_lnurl(
        &self,
        req_data: LnUrlWithdrawRequestData,
        amount_sats: u64,
        description: Option<String>,
    ) -> Result<LnUrlWithdrawCallbackStatus> {
        let invoice = self
            .receive_payment(amount_sats, description.unwrap_or_default())
            .await?;
        validate_lnurl_withdraw(req_data, invoice).await
    }

    pub async fn receive_payment(
        &self,
        amount_sats: u64,
        description: String,
    ) -> Result<LNInvoice> {
        self.payment_receiver
            .receive_payment(amount_sats, description, None)
            .await
    }

    pub fn node_info(&self) -> Result<Option<NodeState>> {
        self.persister.get_node_state()
    }

    pub async fn list_payments(
        &self,
        filter: PaymentTypeFilter,
        from_timestamp: Option<i64>,
        to_timestamp: Option<i64>,
    ) -> Result<Vec<Payment>> {
        self.persister
            .list_payments(filter, from_timestamp, to_timestamp)
            .map_err(|err| anyhow!(err))
    }

    pub async fn sweep(&self, to_address: String, fee_rate_sats_per_byte: u64) -> Result<()> {
        self.start_node().await?;
        self.node_api
            .sweep(to_address, fee_rate_sats_per_byte)
            .await?;
        self.sync().await?;
        Ok(())
    }

    pub async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>> {
        self.fiat_api.fetch_fiat_rates().await
    }

    pub fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>> {
        self.fiat_api.list_fiat_currencies()
    }

    pub async fn list_lsps(&self) -> Result<Vec<LspInformation>> {
        self.lsp_api
            .list_lsps(self.node_info()?.ok_or(anyhow!("err"))?.id)
            .await
    }

    pub async fn connect_lsp(&self, lsp_id: String) -> Result<()> {
        self.persister.set_lsp_id(lsp_id)?;
        self.sync().await?;
        Ok(())
    }

    /// Convenience method to look up LSP info based on current LSP ID
    pub async fn lsp_info(&self) -> Result<LspInformation> {
        get_lsp(self.persister.clone(), self.lsp_api.clone()).await
    }

    pub async fn close_lsp_channels(&self) -> Result<()> {
        self.start_node().await?;
        let lsp = self.lsp_info().await?;
        self.node_api
            .close_peer_channels(lsp.pubkey)
            .await
            .map(|_| ())?;
        self.sync().await
    }

    /// Onchain receive swap API
    pub async fn receive_onchain(&self) -> Result<SwapInfo> {
        self.btc_receive_swapper.create_swap_address().await
    }

    // list swaps history (all of them: expired, refunded and active)
    pub async fn list_refundables(&self) -> Result<Vec<SwapInfo>> {
        self.btc_receive_swapper.list_refundables()
    }

    // construct and broadcast a refund transaction for a faile/expired swap
    pub async fn refund(
        &self,
        swap_address: String,
        to_address: String,
        sat_per_vbyte: u32,
    ) -> Result<String> {
        self.btc_receive_swapper
            .refund_swap(swap_address, to_address, sat_per_vbyte)
            .await
    }

    /// Excute a command directly on the NodeAPI interface.
    /// Mainly used to debugging.
    pub async fn execute_dev_command(&self, command: &String) -> Result<String> {
        self.node_api.execute_command(command).await
    }

    /// This method sync the local state with the remote node state.
    /// The synced items are as follows:
    /// * node state - General information about the node and its liquidity status
    /// * channels - The list of channels and their status
    /// * payments - The incoming/outgoing payments
    async fn sync(&self) -> Result<()> {
        self.start_node().await?;
        self.connect_lsp_peer().await?;

        // First query the changes since last sync time.
        let since_timestamp = self.persister.last_payment_timestamp().unwrap_or(0);
        let new_data = &self.node_api.pull_changed(since_timestamp).await?;

        debug!(
            "pull changed time={:?} {:?}",
            since_timestamp, new_data.payments
        );

        // update node state and channels state
        self.persister.set_node_state(&new_data.node_state)?;
        self.persister.update_channels(&new_data.channels)?;

        //fetch closed_channel and convert them to Payment items.
        let closed_channel_payments_res: Result<Vec<crate::models::Payment>> = self
            .persister
            .list_channels()?
            .into_iter()
            .filter(|c| c.state == ChannelState::Closed || c.state == ChannelState::PendingClose)
            .map(|c| closed_channel_to_transaction(c))
            .collect();

        // update both closed channels and lightning transation payments
        let mut payments = closed_channel_payments_res?;
        payments.extend(new_data.payments.clone());
        self.persister.insert_payments(&payments)?;
        self.notify_event_listeners(BreezEvent::Synced).await?;
        Ok(())
    }

    async fn connect_lsp_peer(&self) -> Result<()> {
        let lsp = self.lsp_info().await.ok();
        if !lsp.is_none() {
            let lsp_info = lsp.unwrap().clone();
            let node_id = lsp_info.pubkey;
            let address = lsp_info.host;
            debug!("connecting to lsp {}@{}", node_id.clone(), address.clone());
            self.node_api
                .connect_peer(node_id.clone(), address.clone())
                .await
                .map_err(anyhow::Error::msg)?;
            debug!("connected to lsp {}@{}", node_id.clone(), address.clone());
        }
        Ok(())
    }

    async fn on_event(&self, e: BreezEvent) -> Result<()> {
        debug!("breez services got event {:?}", e);
        match e {
            BreezEvent::InvoicePaid { details: _ } => self.sync().await?,
            BreezEvent::NewBlock { block: _ } => self.sync().await?,
            _ => {}
        };

        self.notify_event_listeners(e.clone()).await
    }

    async fn notify_event_listeners(&self, e: BreezEvent) -> Result<()> {
        if let Err(err) = self.btc_receive_swapper.on_event(e.clone()).await {
            debug!(
                "btc_receive_swapper failed to processed event {:?}: {:?}",
                e, err
            )
        };

        if !self.event_listener.is_none() {
            self.event_listener.as_ref().unwrap().on_event(e.clone())
        }
        Ok(())
    }

    pub fn set_shutdown_sender(&self, sender: mpsc::Sender<()>) {
        *self.shutdown_sender.lock().unwrap() = Some(sender);
    }

    pub(crate) async fn start_node(&self) -> Result<()> {
        self.node_api.start().await
    }

    pub async fn recommended_fees(&self) -> Result<RecommendedFees> {
        self.chain_service.recommended_fees().await
    }
}

async fn poll_events(breez_services: Arc<BreezServices>, mut current_block: u32) -> Result<()> {
    let mut interval = tokio::time::interval(time::Duration::from_secs(30));
    let mut invoice_stream = breez_services.node_api.stream_incoming_payments().await?;
    let mut log_stream = breez_services.node_api.stream_log_messages().await?;

    loop {
        tokio::select! {

         // handle chain events
         _ = interval.tick() => {
          let tip_res = breez_services.chain_service.current_tip().await;
          match tip_res {
           Ok(next_block) => {
            debug!("got tip {:?}", next_block);
            if next_block > current_block {
             _  = breez_services.on_event(BreezEvent::NewBlock{block: next_block}).await;
            }
            current_block = next_block
           },
           Err(e) => {
            error!("failed to fetch next block {}", e)
           }
          };
         },
         paid_invoice_res = invoice_stream.message() => {
          match paid_invoice_res {
           Ok(Some(i)) => {
            debug!("invoice stream got new invoice");
            match i.details {
             Some(gl_client::pb::incoming_payment::Details::Offchain(p)) => {
              _  = breez_services.on_event(BreezEvent::InvoicePaid{details: InvoicePaidDetails {
                    payment_hash: hex::encode(p.payment_hash),
                    bolt11: p.bolt11,
                }}).await;
             },
             None => {}
            }
           }
           // stream is closed, renew it
           Ok(None) => {
            debug!("invoice stream closed, renewing");
            invoice_stream = breez_services.node_api.stream_incoming_payments().await?;
           }
           Err(err) => {
            debug!("failed to process incoming payment {:?}", err);
            invoice_stream = breez_services.node_api.stream_incoming_payments().await?;
           }
          };
        },
         log_message_res = log_stream.message() => {
          match log_message_res {
           Ok(Some(l)) => {
            debug!("{}", l.line);
           },
           // stream is closed, renew it
           Ok(None) => {
            //debug!("log stream closed, renewing");
            log_stream = breez_services.node_api.stream_log_messages().await?;
           }
           Err(err) => {
            debug!("failed to process log entry {:?}", err);
            log_stream = breez_services.node_api.stream_log_messages().await?;
           }
          };
         }
        }
    }
}

fn closed_channel_to_transaction(
    channel: crate::models::Channel,
) -> Result<crate::models::Payment> {
    let now = SystemTime::now();
    Ok(crate::models::Payment {
        id: channel.funding_txid.clone(),
        payment_type: PaymentType::ClosedChannel,
        payment_time: channel
            .closed_at
            .unwrap_or(now.duration_since(UNIX_EPOCH)?.as_secs()) as i64,
        amount_msat: -1 * channel.spendable_msat as i64,
        fee_msat: 0,
        pending: channel.state == ChannelState::PendingClose,
        description: Some("Closed Channel".to_string()),
        details: PaymentDetails::ClosedChannel {
            data: ClosesChannelPaymentDetails {
                short_channel_id: channel.short_channel_id,
                state: channel.state,
                funding_txid: channel.funding_txid,
            },
        },
    })
}

/// A helper struct to configure and build BreezServices
pub struct BreezServicesBuilder {
    config: Config,
    node_api: Option<Arc<dyn NodeAPI>>,
    creds: Option<GreenlightCredentials>,
    seed: Option<Vec<u8>>,
    lsp_api: Option<Arc<dyn LspAPI>>,
    fiat_api: Option<Arc<dyn FiatAPI>>,
    swapper_api: Option<Arc<dyn SwapperAPI>>,
}

impl BreezServicesBuilder {
    pub fn new(config: Config) -> BreezServicesBuilder {
        BreezServicesBuilder {
            config: config,
            node_api: None,
            creds: None,
            seed: None,
            lsp_api: None,
            fiat_api: None,
            swapper_api: None,
        }
    }

    pub fn node_api(&mut self, node_api: Arc<dyn NodeAPI>) -> &mut Self {
        self.node_api = Some(node_api);
        self
    }

    pub fn lsp_api(&mut self, lsp_api: Arc<dyn LspAPI>) -> &mut Self {
        self.lsp_api = Some(lsp_api.clone());
        self
    }

    pub fn fiat_api(&mut self, fiat_api: Arc<dyn FiatAPI>) -> &mut Self {
        self.fiat_api = Some(fiat_api.clone());
        self
    }

    pub fn swapper_api(&mut self, swapper_api: Arc<dyn SwapperAPI>) -> &mut Self {
        self.swapper_api = Some(swapper_api.clone());
        self
    }

    pub fn greenlight_credentials(
        &mut self,
        creds: GreenlightCredentials,
        seed: Vec<u8>,
    ) -> &mut Self {
        self.creds = Some(creds);
        self.seed = Some(seed);
        self
    }

    pub fn build(&self, listener: Option<Box<dyn EventListener>>) -> Result<Arc<BreezServices>> {
        if self.node_api.is_none() && (self.creds.is_none() || self.seed.is_none()) {
            return Err(anyhow!(
                "Either node_api or both credentials and seed should be provided"
            ));
        }

        let mut node_api = self.node_api.clone();
        if node_api.is_none() {
            if self.creds.is_none() || self.seed.is_none() {
                return Err(anyhow!(
                    "Either node_api or both credentials and seed should be provided"
                ));
            }
            let greenlight = Greenlight::new(
                self.config.clone(),
                self.seed.clone().unwrap(),
                self.creds.clone().unwrap(),
            )?;
            node_api = Some(Arc::new(greenlight));
        }
        let unwrapped_node_api = node_api.unwrap();

        // breez_server provides both FiatAPI & LspAPI implementations
        let breez_server = Arc::new(BreezServer::new(
            self.config.breezserver.clone(),
            self.config.api_key.clone(),
        ));

        // mempool space is used to monitor the chain
        let chain_service = Arc::new(MempoolSpace {
            base_url: self.config.mempoolspace_url.clone(),
        });

        // The storage is implemented via sqlite.
        let persister = Arc::new(crate::persist::db::SqliteStorage::from_file(format!(
            "{}/storage.sql",
            self.config.working_dir
        )));

        persister.init().unwrap();
        let current_lsp_id = persister.get_lsp_id()?;
        if current_lsp_id.is_none() && self.config.default_lsp_id.is_some() {
            persister.set_lsp_id(self.config.default_lsp_id.clone().unwrap())?;
        }

        let payment_receiver = Arc::new(PaymentReceiver {
            node_api: unwrapped_node_api.clone(),
            lsp: breez_server.clone(),
            persister: persister.clone(),
        });

        let btc_receive_swapper = Arc::new(BTCReceiveSwap::new(
            self.config.network.clone().into(),
            self.swapper_api.clone().unwrap_or(breez_server.clone()),
            persister.clone(),
            chain_service.clone(),
            payment_receiver.clone(),
        ));

        // Create the node services and it them statically
        let breez_services = Arc::new(BreezServices {
            config: self.config.clone(),
            node_api: unwrapped_node_api.clone(),
            lsp_api: self.lsp_api.clone().unwrap_or(breez_server.clone()),
            fiat_api: self.fiat_api.clone().unwrap_or(breez_server.clone()),
            chain_service: chain_service.clone(),
            persister: persister.clone(),
            btc_receive_swapper: btc_receive_swapper.clone(),
            payment_receiver,
            event_listener: listener,
            shutdown_sender: Mutex::new(None),
        });

        Ok(breez_services)
    }
}

#[derive(Clone)]
pub struct BreezServer {
    server_url: String,
    api_key: Option<String>,
}

impl BreezServer {
    pub fn new(server_url: String, api_key: Option<String>) -> Self {
        Self {
            server_url,
            api_key,
        }
    }

    pub(crate) async fn get_channel_opener_client(
        &self,
    ) -> Result<ChannelOpenerClient<InterceptedService<Channel, ApiKeyInterceptor>>> {
        let s = self.server_url.clone();
        let channel = Channel::from_shared(s)?.connect().await?;

        let api_key_metadata: Option<MetadataValue<Ascii>> = match &self.api_key {
            Some(key) => Some(format!("Bearer {}", key).parse()?),
            _ => None,
        };
        let client =
            ChannelOpenerClient::with_interceptor(channel, ApiKeyInterceptor { api_key_metadata });
        Ok(client)
    }

    pub(crate) async fn get_information_client(&self) -> Result<InformationClient<Channel>> {
        InformationClient::connect(Uri::from_str(&self.server_url)?)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub(crate) async fn get_fund_manager_client(&self) -> Result<FundManagerClient<Channel>> {
        FundManagerClient::connect(Uri::from_str(&self.server_url)?)
            .await
            .map_err(|e| anyhow!(e))
    }
}

pub(crate) struct ApiKeyInterceptor {
    api_key_metadata: Option<MetadataValue<Ascii>>,
}
impl Interceptor for ApiKeyInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        if self.api_key_metadata.clone().is_some() {
            req.metadata_mut()
                .insert("authorization", self.api_key_metadata.clone().unwrap());
        }
        Ok(req)
    }
}

/// Attempts to convert the phrase to a mnemonic, then to a seed.
///
/// If the phrase is not a valid mnemonic, an error is returned.
pub fn mnemonic_to_seed(phrase: String) -> Result<Vec<u8>> {
    let mnemonic = Mnemonic::from_phrase(&phrase, Language::English)?;
    let seed = Seed::new(&mnemonic, "");
    Ok(seed.as_bytes().to_vec())
}

#[tonic::async_trait]
pub trait Receiver: Send + Sync {
    async fn receive_payment(
        &self,
        amount_sats: u64,
        description: String,
        preimage: Option<Vec<u8>>,
    ) -> Result<LNInvoice>;
}

pub(crate) struct PaymentReceiver {
    node_api: Arc<dyn NodeAPI>,
    lsp: Arc<dyn LspAPI>,
    persister: Arc<persist::db::SqliteStorage>,
}

#[tonic::async_trait]
impl Receiver for PaymentReceiver {
    async fn receive_payment(
        &self,
        amount_sats: u64,
        description: String,
        preimage: Option<Vec<u8>>,
    ) -> Result<LNInvoice> {
        self.node_api.start().await?;
        let lsp_info = get_lsp(self.persister.clone(), self.lsp.clone()).await?;
        let node_state = self
            .persister
            .get_node_state()?
            .ok_or("Failed to retrieve node state")
            .map_err(|err| anyhow!(err))?;

        let amount_msats = amount_sats * 1000;

        let mut short_channel_id = parse_short_channel_id("1x0x0")?;
        let mut destination_invoice_amount_sats = amount_msats;

        // check if we need to open channel
        if node_state.inbound_liquidity_msats < amount_msats {
            info!("We need to open a channel");

            // we need to open channel so we are calculating the fees for the LSP
            let channel_fees_msat_calculated =
                amount_msats * lsp_info.channel_fee_permyriad as u64 / 10_000 / 1_000_000;
            let channel_fees_msat = max(
                channel_fees_msat_calculated,
                lsp_info.channel_minimum_fee_msat as u64,
            );

            if amount_msats < channel_fees_msat + 1000 {
                return Err(anyhow!(
                    "requestPayment: Amount should be more than the minimum fees {} sats",
                    lsp_info.channel_minimum_fee_msat / 1000
                ));
            }

            // remove the fees from the amount to get the small amount on the current node invoice.
            destination_invoice_amount_sats = amount_sats - channel_fees_msat / 1000;
        } else {
            // not opening a channel so we need to get the real channel id into the routing hints
            info!("Finding channel ID for routing hint");
            for peer in self.node_api.list_peers().await? {
                if hex::encode(peer.id) == lsp_info.pubkey && !peer.channels.is_empty() {
                    let active_channel = peer
                        .channels
                        .iter()
                        .find(|&c| c.state == "CHANNELD_NORMAL")
                        .ok_or("No open channel found")
                        .map_err(|err| anyhow!(err))?;
                    short_channel_id = parse_short_channel_id(&active_channel.short_channel_id)?;
                    info!(
                        "Found channel ID: {} {:?}",
                        short_channel_id, active_channel
                    );
                    break;
                }
            }
        }

        info!("Creating invoice on NodeAPI");
        let invoice = &self
            .node_api
            .create_invoice(amount_sats, description, preimage)
            .await?;
        info!("Invoice created {}", invoice.bolt11);

        info!("Adding routing hint");
        let lsp_hop = RouteHintHop {
            src_node_id: lsp_info.pubkey.clone(), // TODO correct?
            short_channel_id: short_channel_id as u64,
            fees_base_msat: lsp_info.base_fee_msat as u32,
            fees_proportional_millionths: 10, // TODO
            cltv_expiry_delta: lsp_info.time_lock_delta as u64,
            htlc_minimum_msat: Some(lsp_info.min_htlc_msat as u64), // TODO correct?
            htlc_maximum_msat: Some(1000000000),                    // TODO ?
        };

        info!("lsp hop = {:?}", lsp_hop);

        let raw_invoice_with_hint = add_routing_hints(
            &invoice.bolt11,
            vec![RouteHint {
                hops: vec![lsp_hop],
            }],
            amount_sats * 1000,
        )?;
        info!("Routing hint added");
        let signed_invoice_with_hint = self.node_api.sign_invoice(raw_invoice_with_hint)?;
        let parsed_invoice = parse_invoice(&signed_invoice_with_hint.to_string())?;

        // register the payment at the lsp if needed
        if destination_invoice_amount_sats < amount_sats {
            info!("Registering payment with LSP");
            self.lsp
                .register_payment(
                    lsp_info.id.clone(),
                    lsp_info.lsp_pubkey.clone(),
                    PaymentInformation {
                        payment_hash: hex::decode(parsed_invoice.payment_hash.clone())?,
                        payment_secret: parsed_invoice.payment_secret.clone(),
                        destination: hex::decode(parsed_invoice.payee_pubkey.clone())?,
                        incoming_amount_msat: amount_msats as i64,
                        outgoing_amount_msat: (destination_invoice_amount_sats * 1000) as i64,
                    },
                )
                .await?;
            info!("Payment registered");
        }

        // return the signed, converted invoice with hints
        Ok(parsed_invoice)
    }
}

/// Convenience method to look up LSP info based on current LSP ID
async fn get_lsp(persister: Arc<SqliteStorage>, lsp: Arc<dyn LspAPI>) -> Result<LspInformation> {
    let lsp_id = persister
        .get_lsp_id()?
        .ok_or("No LSP ID found")
        .map_err(|err| anyhow!(err))?;

    let node_pubkey = persister
        .get_node_state()?
        .ok_or("No NodeState found")
        .map_err(|err| anyhow!(err))?
        .id;

    lsp.list_lsps(node_pubkey)
        .await?
        .iter()
        .find(|&lsp| lsp.id == lsp_id)
        .ok_or("No LSP found for given LSP ID")
        .map_err(|err| anyhow!(err))
        .cloned()
}

pub(crate) mod test {
    use rand::Rng;
    use std::sync::Arc;
    use std::time::SystemTime;
    use tokio::sync::mpsc;

    use anyhow::anyhow;

    use crate::breez_services::{BreezServices, BreezServicesBuilder, Config};
    use crate::chain::MempoolSpace;
    use crate::fiat::Rate;
    use crate::models::{
        ClosesChannelPaymentDetails, LnPaymentDetails, NodeState, Payment, PaymentDetails,
        PaymentTypeFilter,
    };
    use crate::test_utils::*;
    use crate::{persist, PaymentType};

    #[test]
    fn test_config() {
        // Before the state is initialized, the config defaults to using ::default() for its values
        let config = Config::default();
        assert_eq!(config.breezserver, "https://bs1-st.breez.technology:443");
        assert_eq!(config.mempoolspace_url, "https://mempool.space");
    }

    #[tokio::test]
    async fn test_node_state() -> Result<(), Box<dyn std::error::Error>> {
        let storage_path = format!("{}/storage.sql", get_test_working_dir());
        std::fs::remove_file(storage_path).ok();

        let dummy_node_state = get_dummy_node_state();

        let dummy_transactions = vec![
            Payment {
                id: "1111".to_string(),
                payment_type: PaymentType::Received,
                payment_time: 100000,
                amount_msat: 10,
                fee_msat: 0,
                pending: false,
                description: Some("test receive".to_string()),
                details: PaymentDetails::Ln {
                    data: LnPaymentDetails {
                        payment_hash: "1111".to_string(),
                        label: "".to_string(),
                        destination_pubkey: "1111".to_string(),
                        payment_preimage: "2222".to_string(),
                        keysend: false,
                        bolt11: "1111".to_string(),
                    },
                },
            },
            Payment {
                id: "3333".to_string(),
                payment_type: PaymentType::Sent,
                payment_time: 200000,
                amount_msat: 8,
                fee_msat: 2,
                pending: false,
                description: Some("test payment".to_string()),
                details: PaymentDetails::Ln {
                    data: LnPaymentDetails {
                        payment_hash: "3333".to_string(),
                        label: "".to_string(),
                        destination_pubkey: "123".to_string(),
                        payment_preimage: "4444".to_string(),
                        keysend: false,
                        bolt11: "123".to_string(),
                    },
                },
            },
        ];
        let node_api = Arc::new(MockNodeAPI {
            node_state: dummy_node_state.clone(),
            transactions: dummy_transactions.clone(),
        });

        let mut builder = BreezServicesBuilder::new(create_test_config());
        let breez_services = builder
            .lsp_api(Arc::new(MockBreezServer {}))
            .fiat_api(Arc::new(MockBreezServer {}))
            .node_api(node_api)
            .build(None)?;

        breez_services.sync().await?;
        let fetched_state = breez_services
            .node_info()?
            .ok_or("No NodeState found")
            .map_err(|err| anyhow!(err))?;
        assert_eq!(fetched_state, dummy_node_state);

        let all = breez_services
            .list_payments(PaymentTypeFilter::All, None, None)
            .await?;
        let mut cloned = all.clone();

        // test the right order
        cloned.reverse();
        assert_eq!(dummy_transactions, cloned);

        let received = breez_services
            .list_payments(PaymentTypeFilter::Received, None, None)
            .await?;
        assert_eq!(received, vec![cloned[0].clone()]);

        let sent = breez_services
            .list_payments(PaymentTypeFilter::Sent, None, None)
            .await?;
        assert_eq!(sent, vec![cloned[1].clone()]);

        Ok(())
    }

    #[tokio::test]
    async fn test_list_lsps() -> Result<(), Box<dyn std::error::Error>> {
        let storage_path = format!("{}/storage.sql", get_test_working_dir());
        std::fs::remove_file(storage_path).ok();

        let breez_services = breez_services().await;
        breez_services.sync().await?;

        let node_pubkey = breez_services
            .node_info()?
            .ok_or("No NodeState found")
            .map_err(|err| anyhow!(err))?
            .id;
        let lsps = breez_services.lsp_api.list_lsps(node_pubkey).await?;
        assert!(lsps.is_empty()); // The mock returns an empty list

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_rates() -> Result<(), Box<dyn std::error::Error>> {
        let breez_services = breez_services().await;
        breez_services.sync().await?;

        let rates = breez_services.fiat_api.fetch_fiat_rates().await?;
        assert_eq!(rates.len(), 1);
        assert_eq!(
            rates[0],
            Rate {
                coin: "USD".to_string(),
                value: 20_000.00
            }
        );

        Ok(())
    }

    /// build node service for tests
    pub(crate) async fn breez_services() -> Arc<BreezServices> {
        let node_api = Arc::new(MockNodeAPI {
            node_state: get_dummy_node_state(),
            transactions: vec![],
        });

        let mut builder = BreezServicesBuilder::new(create_test_config());
        let breez_services = builder
            .lsp_api(Arc::new(MockBreezServer {}))
            .fiat_api(Arc::new(MockBreezServer {}))
            .node_api(node_api)
            .build(None)
            .unwrap();

        breez_services
    }

    /// Build dummy NodeState for tests
    pub(crate) fn get_dummy_node_state() -> NodeState {
        NodeState {
            id: "tx1".to_string(),
            block_height: 1,
            channels_balance_msat: 100,
            onchain_balance_msat: 1000,
            max_payable_msat: 95,
            max_receivable_msat: 1000,
            max_single_payment_amount_msat: 1000,
            max_chan_reserve_msats: 0,
            connected_peers: vec!["1111".to_string()],
            inbound_liquidity_msats: 2000,
        }
    }
}
