#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

use anyhow::Result;
use log::trace;
use tokio::sync::Mutex;
use tonic::codegen::InterceptedService;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
use tonic::{Request, Status};

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
pub static PRODUCTION_BREEZSERVER_URL: &str = "https://bsw1.breez.technology";
pub static STAGING_BREEZSERVER_URL: &str = "https://bs1-st.breez.technology:443";

#[cfg(not(target_arch = "wasm32"))]
type Transport = tonic::transport::Channel;
#[cfg(target_arch = "wasm32")]
type Transport = tonic_web_wasm_client::Client;

pub struct BreezServer {
    transport: Mutex<Transport>,
    api_key: Option<String>,
}

impl BreezServer {
    pub fn new(server_url: String, api_key: Option<String>) -> Result<Self> {
        Ok(Self {
            #[cfg(not(target_arch = "wasm32"))]
            transport: Mutex::new(Self::create_endpoint(&server_url)?.connect_lazy()),
            #[cfg(target_arch = "wasm32")]
            transport: Mutex::new(tonic_web_wasm_client::Client::new(server_url)),
            api_key,
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn create_endpoint(server_url: &str) -> Result<tonic::transport::Endpoint> {
        Ok(
            tonic::transport::Endpoint::from_shared(server_url.to_string())?
                .http2_keep_alive_interval(Duration::new(5, 0))
                .tcp_keepalive(Some(Duration::from_secs(5)))
                .keep_alive_timeout(Duration::from_secs(5))
                .keep_alive_while_idle(true),
        )
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

    pub async fn get_channel_opener_client(
        &self,
    ) -> Result<
        ChannelOpenerClient<InterceptedService<Transport, ApiKeyInterceptor>>,
        ServiceConnectivityError,
    > {
        let api_key_metadata = self.api_key_metadata()?;
        let with_interceptor = ChannelOpenerClient::with_interceptor(
            self.transport.lock().await.clone(),
            ApiKeyInterceptor { api_key_metadata },
        );
        Ok(with_interceptor)
    }

    pub async fn get_payment_notifier_client(&self) -> PaymentNotifierClient<Transport> {
        PaymentNotifierClient::new(self.transport.lock().await.clone())
    }

    pub async fn get_information_client(&self) -> InformationClient<Transport> {
        InformationClient::new(self.transport.lock().await.clone())
    }

    pub async fn get_signer_client(&self) -> SignerClient<Transport> {
        SignerClient::new(self.transport.lock().await.clone())
    }

    pub async fn get_support_client(
        &self,
    ) -> Result<
        SupportClient<InterceptedService<Transport, ApiKeyInterceptor>>,
        ServiceConnectivityError,
    > {
        let api_key_metadata = self.api_key_metadata()?;
        Ok(SupportClient::with_interceptor(
            self.transport.lock().await.clone(),
            ApiKeyInterceptor { api_key_metadata },
        ))
    }

    pub async fn get_swapper_client(&self) -> SwapperClient<Transport> {
        SwapperClient::new(self.transport.lock().await.clone())
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
