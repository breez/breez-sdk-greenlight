use crate::invoice::parse_invoice;
use crate::models::{
    Config, GreenlightCredentials, LnPaymentDetails, Network, NodeAPI, NodeState, PaymentDetails,
    PaymentType, SyncResponse, UnspentTransactionOutput,
};
use crate::persist::db::SqliteStorage;
use crate::{Channel, ChannelState, NodeConfig};

use anyhow::{anyhow, Result};
use bitcoin::bech32::{u5, ToBase32};
use bitcoin::secp256k1::ecdsa::{RecoverableSignature, RecoveryId};
use ecies::utils::{aes_decrypt, aes_encrypt};
use gl_client::node::ClnClient;
use gl_client::pb::amount::Unit;

use gl_client::pb::cln::{
    self, CloseRequest, ListclosedchannelsClosedchannels, ListclosedchannelsRequest,
    ListpeerchannelsRequest,
};
use gl_client::pb::{
    Amount, Invoice, InvoiceRequest, InvoiceStatus, OffChainPayment, PayStatus, WithdrawResponse,
};
use gl_client::scheduler::Scheduler;
use gl_client::signer::Signer;
use gl_client::tls::TlsConfig;
use gl_client::{node, pb, utils};

use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey};
use gl_client::pb::Peer;
use lightning_invoice::{RawInvoice, SignedRawInvoice};
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use strum_macros::{Display, EnumString};
use tokio::sync::{mpsc, Mutex};
use tonic::Streaming;

const MAX_PAYMENT_AMOUNT_MSAT: u64 = 4294967000;
const MAX_INBOUND_LIQUIDITY_MSAT: u64 = 4000000000;

pub(crate) struct Greenlight {
    sdk_config: Config,
    signer: Signer,
    tls_config: TlsConfig,
    gl_client: Mutex<Option<node::Client>>,
    node_client: Mutex<Option<ClnClient>>,
}

impl Greenlight {
    /// Connects to a live node using the provided seed and config.
    /// If the node is not registered, it will try to recover it using the seed.
    /// If the node is not created, it will register it using the provided partner credentials
    /// or invite code
    /// If the node is already registered and an existing credentials were found, it will try to
    /// connect to the node using these credentials.    
    pub async fn connect(
        config: Config,
        seed: Vec<u8>,
        persister: Arc<SqliteStorage>,
    ) -> Result<Self> {
        // Derive the encryption key from the seed
        let signer = Signer::new(seed.clone(), config.network.into(), TlsConfig::new()?)?;
        let encryption_key = Self::derive_bip32_key(
            config.network,
            &signer,
            vec![ChildNumber::from_hardened_idx(140)?, ChildNumber::from(0)],
        )?
        .to_priv()
        .to_bytes();
        let encryption_key_slice = encryption_key.as_slice();

        let register_credentials = match config.node_config.clone() {
            NodeConfig::Greenlight { config } => config,
        };

        // query for the existing credentials
        let credentials = persister.get_gl_credentials()?;
        let parsed_credentials: Result<GreenlightCredentials> = match credentials {
            // In case we found existing credentials, try to decrypt them and connect to the node
            Some(creds) => {
                let decrypted_credentials = aes_decrypt(encryption_key_slice, creds.as_slice());
                match decrypted_credentials {
                    Some(creds) => {
                        let built_credentials: GreenlightCredentials =
                            serde_json::from_slice(creds.as_slice())?;
                        info!("Initializing greenlight from existing credentials");
                        Ok(built_credentials)
                    }
                    None => {
                        return Err(anyhow!(
                            "Failed to decrypt credentials, seed doesn't match existing node"
                        ));
                    }
                }
            }
            // In case no credentials were found, try to recover the node
            None => {
                info!("No credentials found, trying to recover existing node");
                let recovered = Self::recover(config.network, seed.clone()).await;
                match recovered {
                    Ok(creds) => Ok(creds),
                    Err(_) => {
                        // If we got here it means we failed to recover so we need to register a new node
                        info!("Failed to recover node, registering new one");
                        let credentials = Self::register(
                            config.clone().network,
                            seed.clone(),
                            register_credentials.partner_credentials,
                            register_credentials.invite_code,
                        )
                        .await?;
                        Ok(credentials)
                    }
                }
            }
        };

        // Persist the connection credentials for future use and return the node instance
        let res = match parsed_credentials {
            Ok(creds) => {
                let json_creds = serde_json::to_string(&creds)?.as_bytes().to_vec();
                let encryptd_creds = aes_encrypt(encryption_key_slice, json_creds.as_slice());
                match encryptd_creds {
                    Some(c) => {
                        persister.set_gl_credentials(c)?;
                        Greenlight::new(config, seed, creds).await
                    }
                    None => {
                        return Err(anyhow!("Failed to encrypt credentials"));
                    }
                }
            }
            Err(_) => Err(anyhow!("Failed to get gl credentials")),
        };
        res
    }

    async fn new(
        sdk_config: Config,
        seed: Vec<u8>,
        connection_credentials: GreenlightCredentials,
    ) -> Result<Greenlight> {
        let greenlight_network = sdk_config.network.into();
        let tls_config = TlsConfig::new()?.identity(
            connection_credentials.device_cert,
            connection_credentials.device_key,
        );
        let signer = Signer::new(seed, greenlight_network, tls_config.clone())?;

        Ok(Greenlight {
            sdk_config,
            signer,
            tls_config,
            gl_client: Mutex::new(None),
            node_client: Mutex::new(None),
        })
    }

    fn derive_bip32_key(
        network: Network,
        signer: &Signer,
        path: Vec<ChildNumber>,
    ) -> Result<ExtendedPrivKey> {
        ExtendedPrivKey::new_master(network.into(), &signer.bip32_ext_key())?
            .derive_priv(&Secp256k1::new(), &path)
            .map_err(|e| anyhow!(e))
    }

    async fn register(
        network: Network,
        seed: Vec<u8>,
        register_credentials: Option<GreenlightCredentials>,
        invite_code: Option<String>,
    ) -> Result<GreenlightCredentials> {
        if invite_code.is_some() && register_credentials.is_some() {
            return Err(anyhow!("Cannot specify both invite code and credentials"));
        }
        let greenlight_network = network.into();
        let tls_config = match register_credentials {
            Some(creds) => {
                debug!("registering with credentials");
                TlsConfig::new()?.identity(creds.device_cert, creds.device_key)
            }
            None => TlsConfig::new()?,
        };

        let signer = Signer::new(seed, greenlight_network, tls_config.clone())?;
        let scheduler = Scheduler::with(
            signer.node_id(),
            greenlight_network,
            utils::scheduler_uri(),
            &tls_config,
        )
        .await?;
        let recover_res: pb::scheduler::RegistrationResponse =
            scheduler.register(&signer, invite_code).await?;

        Ok(GreenlightCredentials {
            device_key: recover_res.device_key.into(),
            device_cert: recover_res.device_cert.into(),
        })
    }

    async fn recover(network: Network, seed: Vec<u8>) -> Result<GreenlightCredentials> {
        let greenlight_network = network.into();
        let tls_config = TlsConfig::new()?;
        let signer = Signer::new(seed, greenlight_network, tls_config.clone())?;
        let scheduler = Scheduler::new(signer.node_id(), greenlight_network).await?;
        let recover_res: pb::scheduler::RecoveryResponse = scheduler.recover(&signer).await?;

        Ok(GreenlightCredentials {
            device_key: recover_res.device_key.as_bytes().to_vec(),
            device_cert: recover_res.device_cert.as_bytes().to_vec(),
        })
    }

    async fn get_client(&self) -> Result<node::Client> {
        let mut gl_client = self.gl_client.lock().await;
        if gl_client.is_none() {
            let scheduler =
                Scheduler::new(self.signer.node_id(), self.sdk_config.network.into()).await?;
            *gl_client = Some(scheduler.schedule(self.tls_config.clone()).await?);
        }
        Ok(gl_client.clone().unwrap())
    }

    pub(crate) async fn get_node_client(&self) -> Result<node::ClnClient> {
        let mut node_client = self.node_client.lock().await;
        if node_client.is_none() {
            let scheduler =
                Scheduler::new(self.signer.node_id(), self.sdk_config.network.into()).await?;
            *node_client = Some(scheduler.schedule(self.tls_config.clone()).await?);
        }
        Ok(node_client.clone().unwrap())
    }
}

#[tonic::async_trait]
impl NodeAPI for Greenlight {
    async fn create_invoice(
        &self,
        amount_sats: u64,
        description: String,
        preimage: Option<Vec<u8>>,
    ) -> Result<Invoice> {
        let mut client = self.get_client().await?;

        let request = InvoiceRequest {
            amount: Some(Amount {
                unit: Some(Unit::Satoshi(amount_sats)),
            }),
            label: format!(
                "breez-{}",
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
            ),
            description,
            preimage: preimage.unwrap_or_default(),
        };

        Ok(client.create_invoice(request).await?.into_inner())
    }

    // implemenet pull changes from greenlight
    async fn pull_changed(&self, since_timestamp: i64) -> Result<SyncResponse> {
        info!("pull changed since {}", since_timestamp);
        let mut client = self.get_client().await?;
        let mut node_client = self.get_node_client().await?;

        // list all peers
        let peers = client
            .list_peers(pb::ListPeersRequest::default())
            .await?
            .into_inner();

        // get node info
        let node_info = client
            .get_info(pb::GetInfoRequest::default())
            .await?
            .into_inner();

        // list both off chain funds and on chain fudns
        let funds = client
            .list_funds(pb::ListFundsRequest::default())
            .await?
            .into_inner();
        let offchain_funds = funds.channels;
        let onchain_funds = funds.outputs;

        // filter only connected peers
        let connected_peers: Vec<String> = peers
            .peers
            .clone()
            .iter()
            .filter(|p| p.connected)
            .map(|p| hex::encode(p.id.clone()))
            .collect();

        // make a vector of all channels by searching in peers
        let all_channels: &mut Vec<pb::Channel> = &mut Vec::new();
        peers.peers.clone().iter().for_each(|p| {
            let peer_channels = &mut p.channels.clone();
            all_channels.append(peer_channels);
        });

        // filter only opened channels
        let opened_channels: &mut Vec<&pb::Channel> = &mut all_channels
            .iter()
            .filter(|c| c.state == *"CHANNELD_NORMAL")
            .collect();

        // Fetch closed channels from greenlight
        let closed_channels = match node_client
            .list_closed_channels(ListclosedchannelsRequest { id: None })
            .await
        {
            Ok(c) => c.into_inner().closedchannels,
            Err(e) => {
                error!("list closed channels error {:?}", e);
                vec![]
            }
        };

        let forgotten_closed_channels: Result<Vec<Channel>> = closed_channels
            .into_iter()
            .filter(|c| {
                let hex_txid = hex::encode(c.funding_txid.clone());
                all_channels.iter().all(|c| c.funding_txid != hex_txid)
            })
            .map(TryInto::try_into)
            .collect();

        info!("forgotten_closed_channels {:?}", forgotten_closed_channels);

        let mut all_channel_models: Vec<Channel> =
            all_channels.clone().into_iter().map(|c| c.into()).collect();
        all_channel_models.extend(forgotten_closed_channels?);

        // calculate channels balance only from opened channels
        let channels_balance = offchain_funds.iter().fold(0, |a, b| {
            let hex_txid = hex::encode(b.funding_txid.clone());
            if opened_channels.iter().any(|c| c.funding_txid == hex_txid) {
                return a + b.our_amount_msat;
            }
            a
        });

        // calculate onchain balance
        let onchain_balance = onchain_funds.iter().fold(0, |a, b| {
            if b.reserved {
                return a;
            }
            a + amount_to_msat(&b.amount.clone().unwrap_or_default())
        });

        // Collect utxos from onchain funds
        let utxos = onchain_funds
            .iter()
            .filter_map(|list_funds_output| {
                list_funds_output
                    .output
                    .as_ref()
                    .map(|output| UnspentTransactionOutput {
                        txid: output.txid.clone(),
                        outnum: output.outnum,
                        amount_millisatoshi: list_funds_output
                            .amount
                            .as_ref()
                            .map(amount_to_msat)
                            .unwrap_or_default(),
                        address: list_funds_output.address.clone(),
                        reserved: list_funds_output.reserved,
                        reserved_to_block: list_funds_output.reserved_to_block,
                    })
            })
            .collect();

        // calculate payment limits and inbound liquidity
        let mut max_payable: u64 = 0;
        let mut max_receivable_single_channel: u64 = 0;
        opened_channels.iter().try_for_each(|c| -> Result<()> {
            max_payable += amount_to_msat(&parse_amount(c.spendable.clone())?);
            let receivable_amount = amount_to_msat(&parse_amount(c.receivable.clone())?);
            if receivable_amount > max_receivable_single_channel {
                max_receivable_single_channel = receivable_amount;
            }
            Ok(())
        })?;

        let max_allowed_to_receive_msats = max(MAX_INBOUND_LIQUIDITY_MSAT - channels_balance, 0);
        let node_pubkey = hex::encode(node_info.node_id);

        // construct the node state
        let node_state = NodeState {
            id: node_pubkey.clone(),
            block_height: node_info.blockheight,
            channels_balance_msat: channels_balance,
            onchain_balance_msat: onchain_balance,
            utxos,
            max_payable_msat: max_payable,
            max_receivable_msat: max_allowed_to_receive_msats,
            max_single_payment_amount_msat: MAX_PAYMENT_AMOUNT_MSAT,
            max_chan_reserve_msats: channels_balance - min(max_payable, channels_balance),
            connected_peers,
            inbound_liquidity_msats: max_receivable_single_channel,
        };

        Ok(SyncResponse {
            node_state,
            payments: pull_transactions(since_timestamp, client.clone()).await?,
            channels: all_channel_models,
        })
    }

    async fn send_payment(
        &self,
        bolt11: String,
        amount_sats: Option<u64>,
    ) -> Result<crate::models::PaymentResponse> {
        let mut description = None;
        if !bolt11.is_empty() {
            description = parse_invoice(&bolt11)?.description;
        }

        let mut client: node::ClnClient = self.get_node_client().await?;
        let request = pb::cln::PayRequest {
            bolt11,
            amount_msat: amount_sats.map(|amt| gl_client::pb::cln::Amount { msat: amt * 1000 }),
            maxfeepercent: Some(self.sdk_config.maxfee_percent),
            retry_for: Some(self.sdk_config.payment_timeout_sec),
            label: None,
            maxdelay: None,
            riskfactor: None,
            localinvreqid: None,
            exclude: vec![],
            maxfee: None,
            description,
            exemptfee: None,
        };
        client.pay(request).await?.into_inner().try_into()
    }

    async fn send_spontaneous_payment(
        &self,
        node_id: String,
        amount_sats: u64,
    ) -> Result<crate::models::PaymentResponse> {
        let mut client: node::ClnClient = self.get_node_client().await?;
        let request = pb::cln::KeysendRequest {
            destination: hex::decode(node_id)?,
            amount_msat: Some(gl_client::pb::cln::Amount {
                msat: amount_sats * 1000,
            }),
            label: Some(format!(
                "breez-{}",
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
            )),
            extratlvs: None,
            routehints: None,
            maxfeepercent: Some(self.sdk_config.maxfee_percent),
            exemptfee: None,
            retry_for: Some(self.sdk_config.payment_timeout_sec),
            maxdelay: None,
        };
        client.key_send(request).await?.into_inner().try_into()
    }

    async fn start(&self) -> Result<()> {
        self.get_node_client()
            .await?
            .getinfo(pb::cln::GetinfoRequest {})
            .await?;
        Ok(())
    }

    async fn sweep(
        &self,
        to_address: String,
        fee_rate_sats_per_vbyte: u64,
    ) -> Result<WithdrawResponse> {
        let mut client = self.get_client().await?;

        let request = pb::WithdrawRequest {
            feerate: Some(pb::Feerate {
                value: Some(pb::feerate::Value::Perkw(fee_rate_sats_per_vbyte * 250)),
            }),
            amount: Some(Amount {
                unit: Some(Unit::All(true)),
            }),
            destination: to_address,
            minconf: None,
            utxos: vec![],
        };

        Ok(client.withdraw(request).await?.into_inner())
    }

    async fn start_signer(&self, shutdown: mpsc::Receiver<()>) {
        _ = self.signer.run_forever(shutdown).await;
        error!("signer exited");
    }

    async fn list_peers(&self) -> Result<Vec<Peer>> {
        let mut client = self.get_client().await?;
        Ok(client
            .list_peers(pb::ListPeersRequest::default())
            .await?
            .into_inner()
            .peers)
    }

    async fn connect_peer(&self, node_id: String, addr: String) -> Result<()> {
        let mut client = self.get_client().await?;
        let connect_req = pb::ConnectRequest { node_id, addr };
        client.connect_peer(connect_req).await?;
        Ok(())
    }

    fn sign_invoice(&self, invoice: RawInvoice) -> Result<String> {
        let hrp_bytes = invoice.hrp.to_string().as_bytes().to_vec();
        let data_bytes = invoice.data.to_base32();

        // create the message for the signer
        let msg_type: u16 = 8;
        let data_len: u16 = data_bytes.len().try_into()?;
        let mut data_len_bytes = data_len.to_be_bytes().to_vec();
        let mut data_buf = data_bytes.iter().copied().map(u5::to_u8).collect();

        let hrp_len: u16 = hrp_bytes.len().try_into()?;
        let mut hrp_len_bytes = hrp_len.to_be_bytes().to_vec();
        let mut hrp_buf = hrp_bytes.to_vec();

        let mut buf = msg_type.to_be_bytes().to_vec();
        buf.append(&mut data_len_bytes);
        buf.append(&mut data_buf);
        buf.append(&mut hrp_len_bytes);
        buf.append(&mut hrp_buf);
        // Sign the invoice using the signer
        let raw_result = self.signer.sign_invoice(buf)?;
        info!(
            "recover id: {:?} raw = {:?}",
            raw_result, raw_result[64] as i32
        );
        // contruct the RecoveryId
        let rid = RecoveryId::from_i32(raw_result[64] as i32).expect("recovery ID");
        let sig = &raw_result[0..64];
        let recoverable_sig =
            RecoverableSignature::from_compact(sig, rid).map_err(|e| anyhow!(e))?;

        let signed_invoice: Result<SignedRawInvoice> = invoice.sign(|_| Ok(recoverable_sig));
        Ok(signed_invoice?.to_string())
    }

    async fn close_peer_channels(&self, node_id: String) -> Result<Vec<String>> {
        let mut client = self.get_node_client().await?;
        let closed_channels = client
            .list_peer_channels(ListpeerchannelsRequest {
                id: Some(hex::decode(node_id)?),
            })
            .await?
            .into_inner();
        let mut tx_ids = vec![];
        for channel in closed_channels.channels {
            let mut should_close = false;
            if let Some(state) = channel.state {
                match cln::ChannelState::from_i32(state) {
                    Some(cln::ChannelState::Openingd) => should_close = true,
                    Some(cln::ChannelState::ChanneldAwaitingLockin) => should_close = true,
                    Some(cln::ChannelState::ChanneldNormal) => should_close = true,
                    Some(cln::ChannelState::ChanneldShuttingDown) => should_close = true,
                    Some(cln::ChannelState::FundingSpendSeen) => should_close = true,
                    Some(cln::ChannelState::DualopendOpenInit) => should_close = true,
                    Some(cln::ChannelState::DualopendAwaitingLockin) => should_close = true,
                    Some(_) => should_close = false,
                    None => should_close = false,
                }
            }

            if should_close {
                let chan_id = channel.channel_id.ok_or(anyhow!("empty channel id"))?;
                let response = client
                    .close(CloseRequest {
                        id: hex::encode(chan_id),
                        unilateraltimeout: None,
                        destination: None,
                        fee_negotiation_step: None,
                        wrong_funding: None,
                        force_lease_closed: None,
                        feerange: vec![],
                    })
                    .await;
                match response {
                    Ok(res) => {
                        tx_ids.push(hex::encode(
                            res.into_inner()
                                .txid
                                .ok_or(anyhow!("empty txid in close response"))?,
                        ));
                    }
                    Err(e) => {
                        error!("error closing channel: {}", e);
                    }
                };
            }
        }
        Ok(tx_ids)
    }

    async fn stream_incoming_payments(&self) -> Result<Streaming<gl_client::pb::IncomingPayment>> {
        let mut client = self.get_client().await?;
        let stream = client
            .stream_incoming(gl_client::pb::StreamIncomingFilter {})
            .await?
            .into_inner();
        Ok(stream)
    }

    async fn stream_log_messages(&self) -> Result<Streaming<gl_client::pb::LogEntry>> {
        let mut client = self.get_client().await?;
        let stream = client
            .stream_log(gl_client::pb::StreamLogRequest {})
            .await?
            .into_inner();
        Ok(stream)
    }

    async fn execute_command(&self, command: String) -> Result<String> {
        let node_cmd = NodeCommand::from_str(&command)
            .map_err(|_| anyhow!(format!("command not found: {command}")))?;
        match node_cmd {
            NodeCommand::ListPeers => {
                let resp = self
                    .get_client()
                    .await?
                    .list_peers(pb::ListPeersRequest::default())
                    .await?
                    .into_inner();
                Ok(format!("{resp:?}"))
            }
            NodeCommand::ListFunds => {
                let resp = self
                    .get_client()
                    .await?
                    .list_funds(pb::ListFundsRequest::default())
                    .await?
                    .into_inner();
                Ok(format!("{resp:?}"))
            }
            NodeCommand::ListPayments => {
                let resp = self
                    .get_client()
                    .await?
                    .list_payments(pb::ListPaymentsRequest::default())
                    .await?
                    .into_inner();
                Ok(format!("{resp:?}"))
            }
            NodeCommand::ListInvoices => {
                let resp = self
                    .get_client()
                    .await?
                    .list_invoices(pb::ListInvoicesRequest::default())
                    .await?
                    .into_inner();
                Ok(format!("{resp:?}"))
            }
            NodeCommand::CloseAllChannels => {
                let peers_res = self
                    .get_client()
                    .await?
                    .list_peers(pb::ListPeersRequest::default())
                    .await?
                    .into_inner();
                for p in peers_res.peers {
                    self.close_peer_channels(hex::encode(p.id)).await?;
                }

                Ok("All channels were closed".to_string())
            }
        }
    }

    fn derive_bip32_key(&self, path: Vec<ChildNumber>) -> Result<ExtendedPrivKey> {
        Self::derive_bip32_key(self.sdk_config.network, &self.signer, path)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, EnumString, Display, Deserialize, Serialize)]
enum NodeCommand {
    #[strum(serialize = "listpeers")]
    ListPeers,

    #[strum(serialize = "listfunds")]
    ListFunds,

    #[strum(serialize = "listpayments")]
    ListPayments,

    #[strum(serialize = "listinvoices")]
    ListInvoices,

    #[strum(serialize = "closeallchannels")]
    CloseAllChannels,
}

// pulls transactions from greenlight based on last sync timestamp.
// greenlight gives us the payments via API and for received payments we are looking for settled invoices.
async fn pull_transactions(
    since_timestamp: i64,
    client: node::Client,
) -> Result<Vec<crate::models::Payment>> {
    let mut c = client.clone();

    // list invoices
    let invoices = c
        .list_invoices(pb::ListInvoicesRequest::default())
        .await?
        .into_inner();

    // construct the received transactions by filtering the invoices to those paid and beyond the filter timestamp
    let received_transactions: Result<Vec<crate::models::Payment>> = invoices
        .invoices
        .into_iter()
        .filter(|i| {
            i.payment_time > 0
                && i.status() == InvoiceStatus::Paid
                && i.payment_time as i64 > since_timestamp
        })
        .map(TryInto::try_into)
        .collect();

    // fetch payments from greenlight
    let payments = c
        .list_payments(pb::ListPaymentsRequest::default())
        .await?
        .into_inner();
    debug!("list payments: {:?}", payments);
    // construct the payment transactions (pending and complete)
    let outbound_transactions: Result<Vec<crate::models::Payment>> = payments
        .payments
        .into_iter()
        .filter(|p| {
            p.created_at as i64 > since_timestamp
                && (p.status() == PayStatus::Pending || p.status() == PayStatus::Complete)
        })
        .map(TryInto::try_into)
        .collect();

    let mut transactions: Vec<crate::models::Payment> = Vec::new();
    transactions.extend(received_transactions?);
    transactions.extend(outbound_transactions?);

    Ok(transactions)
}

//pub(crate) fn offchain_payment_to_transaction
impl TryFrom<OffChainPayment> for crate::models::Payment {
    type Error = anyhow::Error;

    fn try_from(p: OffChainPayment) -> std::result::Result<Self, Self::Error> {
        let ln_invoice = parse_invoice(&p.bolt11)?;
        Ok(crate::models::Payment {
            id: hex::encode(p.payment_hash.clone()),
            payment_type: PaymentType::Received,
            payment_time: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
            amount_msat: amount_to_msat(&p.amount.unwrap_or_default()),
            fee_msat: 0,
            pending: false,
            description: ln_invoice.description,
            details: PaymentDetails::Ln {
                data: LnPaymentDetails {
                    payment_hash: hex::encode(p.payment_hash),
                    label: p.label,
                    destination_pubkey: ln_invoice.payee_pubkey,
                    payment_preimage: hex::encode(p.preimage),
                    keysend: false,
                    bolt11: p.bolt11,
                    lnurl_success_action: None, // For received payments, this is None
                    lnurl_metadata: None,       // For received payments, this is None
                    ln_address: None,
                },
            },
        })
    }
    // fn from(p: OffChainPayment) -> Self {

    //}
}

/// Construct a lightning transaction from an invoice
impl TryFrom<pb::Invoice> for crate::models::Payment {
    type Error = anyhow::Error;

    fn try_from(invoice: pb::Invoice) -> std::result::Result<Self, Self::Error> {
        let ln_invoice = parse_invoice(&invoice.bolt11)?;
        Ok(crate::models::Payment {
            id: hex::encode(invoice.payment_hash.clone()),
            payment_type: PaymentType::Received,
            payment_time: invoice.payment_time as i64,
            amount_msat: amount_to_msat(&invoice.amount.unwrap_or_default()),
            fee_msat: 0,
            pending: false,
            description: ln_invoice.description,
            details: PaymentDetails::Ln {
                data: LnPaymentDetails {
                    payment_hash: hex::encode(invoice.payment_hash),
                    label: invoice.label,
                    destination_pubkey: ln_invoice.payee_pubkey,
                    payment_preimage: hex::encode(invoice.payment_preimage),
                    keysend: false,
                    bolt11: invoice.bolt11,
                    lnurl_success_action: None, // For received payments, this is None
                    lnurl_metadata: None,       // For received payments, this is None
                    ln_address: None,
                },
            },
        })
    }
}

impl TryFrom<pb::Payment> for crate::models::Payment {
    type Error = anyhow::Error;

    fn try_from(payment: pb::Payment) -> std::result::Result<Self, Self::Error> {
        let mut description = None;
        if !payment.bolt11.is_empty() {
            description = parse_invoice(&payment.bolt11)?.description;
        }

        let payment_amount = amount_to_msat(&payment.amount.unwrap_or_default());
        let payment_amount_sent = amount_to_msat(&payment.amount_sent.unwrap_or_default());

        Ok(crate::models::Payment {
            id: hex::encode(payment.payment_hash.clone()),
            payment_type: PaymentType::Sent,
            payment_time: payment.created_at as i64,
            amount_msat: payment_amount,
            fee_msat: payment_amount_sent - payment_amount,
            pending: pb::PayStatus::from_i32(payment.status) == Some(pb::PayStatus::Pending),
            description,
            details: PaymentDetails::Ln {
                data: LnPaymentDetails {
                    payment_hash: hex::encode(payment.payment_hash),
                    label: "".to_string(),
                    destination_pubkey: hex::encode(payment.destination),
                    payment_preimage: hex::encode(payment.payment_preimage),
                    keysend: payment.bolt11.is_empty(),
                    bolt11: payment.bolt11,
                    lnurl_success_action: None,
                    lnurl_metadata: None,
                    ln_address: None,
                },
            },
        })
    }
}

impl TryFrom<pb::cln::PayResponse> for crate::models::PaymentResponse {
    type Error = anyhow::Error;

    fn try_from(payment: pb::cln::PayResponse) -> std::result::Result<Self, Self::Error> {
        let payment_amount = payment.amount_msat.unwrap_or_default().msat;
        let payment_amount_sent = payment.amount_sent_msat.unwrap_or_default().msat;

        Ok(crate::models::PaymentResponse {
            payment_time: payment.created_at as i64,
            amount_msat: payment_amount,
            fee_msat: payment_amount_sent - payment_amount,
            payment_hash: hex::encode(payment.payment_hash),
            payment_preimage: hex::encode(payment.payment_preimage),
        })
    }
}

impl TryFrom<pb::cln::KeysendResponse> for crate::models::PaymentResponse {
    type Error = anyhow::Error;

    fn try_from(payment: pb::cln::KeysendResponse) -> std::result::Result<Self, Self::Error> {
        let payment_amount = payment.amount_msat.unwrap_or_default().msat;
        let payment_amount_sent = payment.amount_sent_msat.unwrap_or_default().msat;

        Ok(crate::models::PaymentResponse {
            payment_time: payment.created_at as i64,
            amount_msat: payment_amount,
            fee_msat: payment_amount_sent - payment_amount,
            payment_hash: hex::encode(payment.payment_hash),
            payment_preimage: hex::encode(payment.payment_preimage),
        })
    }
}

fn amount_to_msat(amount: &pb::Amount) -> u64 {
    match amount.unit {
        Some(pb::amount::Unit::Millisatoshi(val)) => val,
        Some(pb::amount::Unit::Satoshi(val)) => val * 1000,
        Some(pb::amount::Unit::Bitcoin(val)) => val * 100000000,
        Some(_) => 0,
        None => 0,
    }
}

fn parse_amount(amount_str: String) -> Result<pb::Amount> {
    let mut unit = pb::amount::Unit::Millisatoshi(0);
    if amount_str.ends_with("msat") {
        unit = pb::amount::Unit::Millisatoshi(
            amount_str
                .strip_suffix("msat")
                .ok_or_else(|| anyhow!("wrong amount format {}", amount_str))?
                .to_string()
                .parse::<u64>()?,
        );
    } else if amount_str.ends_with("sat") {
        unit = pb::amount::Unit::Satoshi(
            amount_str
                .strip_suffix("sat")
                .ok_or_else(|| anyhow!("wrong amount format {}", amount_str))?
                .to_string()
                .parse::<u64>()?,
        );
    } else if amount_str.ends_with("bitcoin") {
        unit = pb::amount::Unit::Bitcoin(
            amount_str
                .strip_suffix("bitcoin")
                .ok_or_else(|| anyhow!("wrong amount format {}", amount_str))?
                .to_string()
                .parse::<u64>()?,
        );
    };

    Ok(pb::Amount { unit: Some(unit) })
}

impl From<pb::Channel> for crate::models::Channel {
    fn from(c: pb::Channel) -> Self {
        let state = match c.state.as_str() {
            "OPENINGD" | "CHANNELD_AWAITING_LOCKIN" => crate::models::ChannelState::PendingOpen,
            "CHANNELD_NORMAL" => crate::models::ChannelState::Opened,
            "CLOSED" => crate::models::ChannelState::Closed,
            _ => crate::models::ChannelState::PendingClose,
        };

        crate::models::Channel {
            short_channel_id: c.short_channel_id,
            state,
            funding_txid: c.funding_txid,
            spendable_msat: amount_to_msat(&parse_amount(c.spendable).unwrap_or_default()),
            receivable_msat: amount_to_msat(&parse_amount(c.receivable).unwrap_or_default()),
            closed_at: None,
        }
    }
}

impl TryFrom<ListclosedchannelsClosedchannels> for crate::models::Channel {
    type Error = anyhow::Error;

    fn try_from(c: ListclosedchannelsClosedchannels) -> std::result::Result<Self, Self::Error> {
        let to_us = c
            .final_to_us_msat
            .ok_or(anyhow!("final_to_us_msat is missing"))?
            .msat;
        Ok(crate::models::Channel {
            short_channel_id: c
                .short_channel_id
                .ok_or(anyhow!("short_channel_id is missing"))?,
            state: ChannelState::Closed,
            funding_txid: hex::encode(c.funding_txid),
            spendable_msat: to_us,
            receivable_msat: 0,
            closed_at: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models;
    use anyhow::Result;
    use gl_client::pb;

    #[test]
    fn test_channel_states() -> Result<()> {
        for s in &["OPENINGD", "CHANNELD_AWAITING_LOCKIN"] {
            let c: models::Channel = gl_channel(s).into();
            assert_eq!(c.state, models::ChannelState::PendingOpen);
        }

        let s = &"CHANNELD_NORMAL";
        let c: models::Channel = gl_channel(s).into();
        assert_eq!(c.state, models::ChannelState::Opened);

        for s in &[
            "CHANNELD_SHUTTING_DOWN",
            "CLOSINGD_SIGEXCHANGE",
            "CLOSINGD_COMPLETE",
            "AWAITING_UNILATERAL",
            "FUNDING_SPEND_SEEN",
            "ONCHAIN",
        ] {
            let c: models::Channel = gl_channel(s).into();
            assert_eq!(c.state, models::ChannelState::PendingClose);
        }

        let s = &"CLOSED";
        let c: models::Channel = gl_channel(s).into();
        assert_eq!(c.state, models::ChannelState::Closed);

        Ok(())
        //let c =
    }

    fn gl_channel(state: &str) -> pb::Channel {
        pb::Channel {
            state: state.to_string(),
            owner: "".to_string(),
            short_channel_id: "".to_string(),
            direction: 0,
            channel_id: "".to_string(),
            funding_txid: "".to_string(),
            close_to_addr: "".to_string(),
            close_to: "".to_string(),
            private: true,
            total: "1000msat".to_string(),
            dust_limit: "10msat".to_string(),
            spendable: "20msat".to_string(),
            receivable: "960msat".to_string(),
            their_to_self_delay: 144,
            our_to_self_delay: 144,
            status: vec![],
            alias: None,
            htlcs: vec![],
        }
    }
}
