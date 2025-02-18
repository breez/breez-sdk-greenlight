#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

use anyhow::Result;
use log::trace;
use tokio::sync::Mutex;
use tonic::codegen::InterceptedService;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
#[cfg(not(target_arch = "wasm32"))]
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Status};
#[cfg(target_arch = "wasm32")]
use tonic_web_wasm_client::Client;

use crate::grpc::channel_opener_client::ChannelOpenerClient;
use crate::grpc::information_client::InformationClient;
use crate::grpc::payment_notifier_client::PaymentNotifierClient;
use crate::grpc::signer_client::SignerClient;
use crate::grpc::support_client::SupportClient;
use crate::grpc::swapper_client::SwapperClient;
use crate::grpc::{ChainApiServersRequest, PingRequest};
use crate::prelude::{ServiceConnectivityError, ServiceConnectivityErrorKind};
use crate::with_connection_retry;

#[cfg(not(target_arch = "wasm32"))]
pub static PRODUCTION_BREEZSERVER_URL: &str = "https://bs1.breez.technology:443";
#[cfg(target_arch = "wasm32")]
pub static PRODUCTION_BREEZSERVER_URL: &str = "https://bsw1.breez.technology:443";
pub static STAGING_BREEZSERVER_URL: &str = "https://bs1-st.breez.technology:443";

pub struct BreezServer {
    #[cfg(not(target_arch = "wasm32"))]
    grpc_channel: Mutex<Channel>,
    #[cfg(target_arch = "wasm32")]
    grpc_client: Mutex<Client>,
    api_key: Option<String>,
}

impl BreezServer {
    pub fn new(server_url: String, api_key: Option<String>) -> Result<Self> {
        Ok(Self {
            #[cfg(not(target_arch = "wasm32"))]
            grpc_channel: Mutex::new(Self::create_endpoint(&server_url)?.connect_lazy()),
            #[cfg(target_arch = "wasm32")]
            grpc_client: Mutex::new(tonic_web_wasm_client::Client::new(server_url)),
            api_key,
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn create_endpoint(server_url: &str) -> Result<Endpoint> {
        Ok(Endpoint::from_shared(server_url.to_string())?
            .http2_keep_alive_interval(Duration::new(5, 0))
            .tcp_keepalive(Some(Duration::from_secs(5)))
            .keep_alive_timeout(Duration::from_secs(5))
            .keep_alive_while_idle(true))
    }

    fn api_key_metadata(&self) -> Result<Option<MetadataValue<Ascii>>, ServiceConnectivityError> {
        match &self.api_key {
            Some(key) => Ok(Some(format!("Bearer {key}").parse().map_err(
                |e: InvalidMetadataValue| {
                    ServiceConnectivityError::new(
                        ServiceConnectivityErrorKind::Other,
                        format!("(Breez: {:?}) Failed parse API key: {e}", self.api_key),
                    )
                },
            )?)),
            _ => Ok(None),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_channel_opener_client(
        &self,
    ) -> Result<
        ChannelOpenerClient<InterceptedService<Channel, ApiKeyInterceptor>>,
        ServiceConnectivityError,
    > {
        let api_key_metadata = self.api_key_metadata()?;
        let with_interceptor = ChannelOpenerClient::with_interceptor(
            self.grpc_channel.lock().await.clone(),
            ApiKeyInterceptor { api_key_metadata },
        );
        Ok(with_interceptor)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_channel_opener_client(
        &self,
    ) -> Result<
        ChannelOpenerClient<InterceptedService<Client, ApiKeyInterceptor>>,
        ServiceConnectivityError,
    > {
        let api_key_metadata = self.api_key_metadata()?;
        let with_interceptor = ChannelOpenerClient::with_interceptor(
            self.grpc_client.lock().await.clone(),
            ApiKeyInterceptor { api_key_metadata },
        );
        Ok(with_interceptor)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_payment_notifier_client(&self) -> PaymentNotifierClient<Channel> {
        PaymentNotifierClient::new(self.grpc_channel.lock().await.clone())
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_payment_notifier_client(&self) -> PaymentNotifierClient<Client> {
        PaymentNotifierClient::new(self.grpc_client.lock().await.clone())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_information_client(&self) -> InformationClient<Channel> {
        InformationClient::new(self.grpc_channel.lock().await.clone())
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_information_client(&self) -> InformationClient<Client> {
        InformationClient::new(self.grpc_client.lock().await.clone())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_signer_client(&self) -> SignerClient<Channel> {
        SignerClient::new(self.grpc_channel.lock().await.clone())
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_signer_client(&self) -> SignerClient<Client> {
        SignerClient::new(self.grpc_client.lock().await.clone())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_support_client(
        &self,
    ) -> Result<
        SupportClient<InterceptedService<Channel, ApiKeyInterceptor>>,
        ServiceConnectivityError,
    > {
        let api_key_metadata = self.api_key_metadata()?;
        Ok(SupportClient::with_interceptor(
            self.grpc_channel.lock().await.clone(),
            ApiKeyInterceptor { api_key_metadata },
        ))
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_support_client(
        &self,
    ) -> Result<
        SupportClient<InterceptedService<Client, ApiKeyInterceptor>>,
        ServiceConnectivityError,
    > {
        let api_key_metadata = self.api_key_metadata()?;
        Ok(SupportClient::with_interceptor(
            self.grpc_client.lock().await.clone(),
            ApiKeyInterceptor { api_key_metadata },
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_swapper_client(&self) -> SwapperClient<Channel> {
        SwapperClient::new(self.grpc_channel.lock().await.clone())
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_swapper_client(&self) -> SwapperClient<Client> {
        SwapperClient::new(self.grpc_client.lock().await.clone())
    }

    pub async fn ping(&self) -> Result<String> {
        let request = Request::new(PingRequest {});
        let response = self
            .get_information_client()
            .await
            .ping(request)
            .await?
            .into_inner()
            .version;
        Ok(response)
    }

    pub async fn fetch_mempoolspace_urls(&self) -> Result<Vec<String>, ServiceConnectivityError> {
        let mut client = self.get_information_client().await;
        let chain_api_servers =
            with_connection_retry!(client.chain_api_servers(ChainApiServersRequest {}))
                .await
                .map_err(|e| {
                    ServiceConnectivityError::new(
                        ServiceConnectivityErrorKind::Other,
                        format!("(Breez: {e:?}) Failed to fetch ChainApiServers"),
                    )
                })?
                .into_inner()
                .servers;
        trace!("Received chain_api_servers: {chain_api_servers:?}");

        let mempoolspace_urls = chain_api_servers
            .iter()
            .filter(|s| s.server_type == "MEMPOOL_SPACE")
            .map(|s| s.server_base_url.clone())
            .collect();
        trace!("Received mempoolspace_urls: {mempoolspace_urls:?}");

        Ok(mempoolspace_urls)
    }

    pub async fn fetch_boltz_swapper_urls(&self) -> Result<Vec<String>, ServiceConnectivityError> {
        let mut client = self.get_information_client().await;

        let chain_api_servers =
            with_connection_retry!(client.chain_api_servers(ChainApiServersRequest {}))
                .await
                .map_err(|e| {
                    ServiceConnectivityError::new(
                        ServiceConnectivityErrorKind::Other,
                        format!("(Breez: {e:?}) Failed to fetch ChainApiServers"),
                    )
                })?
                .into_inner()
                .servers;
        trace!("Received chain_api_servers: {chain_api_servers:?}");

        let boltz_swapper_urls = chain_api_servers
            .iter()
            .filter(|s| s.server_type == "BOLTZ_SWAPPER")
            .map(|s| s.server_base_url.clone())
            .collect();
        trace!("Received boltz_swapper_urls: {boltz_swapper_urls:?}");

        Ok(boltz_swapper_urls)
    }
}

#[derive(Clone)]
pub struct ApiKeyInterceptor {
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
