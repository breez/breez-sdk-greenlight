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
use crate::grpc::taproot_swapper_client::TaprootSwapperClient;
use crate::grpc::transport::{GrpcClient, Transport};
use crate::grpc::{ChainApiServersRequest, PingRequest};
use crate::model::BoltzSwapperUrls;
use crate::prelude::{ServiceConnectivityError, ServiceConnectivityErrorKind};
use crate::with_connection_retry;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub static PRODUCTION_BREEZSERVER_URL: &str = "https://bs1.breez.technology:443";
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub static PRODUCTION_BREEZSERVER_URL: &str = "https://bsw1.breez.technology";
pub static STAGING_BREEZSERVER_URL: &str = "https://bs1-st.breez.technology:443";
pub static REGTEST_BREEZSERVER_URL: &str = "http://localhost:8888";
pub static REGTEST_MEMPOOL_URL: &str = "http://localhost:8999/api/v1";

pub struct BreezServer {
    grpc_client: Mutex<GrpcClient>,
    api_key: Option<String>,
}

impl BreezServer {
    pub fn new(server_url: String, api_key: Option<String>) -> anyhow::Result<Self> {
        Ok(Self {
            grpc_client: Mutex::new(GrpcClient::new(server_url)?),
            api_key,
        })
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
            self.grpc_client.lock().await.clone().into_inner(),
            ApiKeyInterceptor { api_key_metadata },
        );
        Ok(with_interceptor)
    }

    pub async fn get_payment_notifier_client(&self) -> PaymentNotifierClient<Transport> {
        PaymentNotifierClient::new(self.grpc_client.lock().await.clone().into_inner())
    }

    pub async fn get_information_client(&self) -> InformationClient<Transport> {
        InformationClient::new(self.grpc_client.lock().await.clone().into_inner())
    }

    pub async fn get_signer_client(&self) -> SignerClient<Transport> {
        SignerClient::new(self.grpc_client.lock().await.clone().into_inner())
    }

    pub async fn get_support_client(
        &self,
    ) -> Result<
        SupportClient<InterceptedService<Transport, ApiKeyInterceptor>>,
        ServiceConnectivityError,
    > {
        let api_key_metadata = self.api_key_metadata()?;
        Ok(SupportClient::with_interceptor(
            self.grpc_client.lock().await.clone().into_inner(),
            ApiKeyInterceptor { api_key_metadata },
        ))
    }

    pub async fn get_swapper_client(&self) -> SwapperClient<Transport> {
        SwapperClient::new(self.grpc_client.lock().await.clone().into_inner())
    }

    pub async fn get_taproot_swapper_client(
        &self,
    ) -> Result<
        TaprootSwapperClient<InterceptedService<Transport, ApiKeyInterceptor>>,
        ServiceConnectivityError,
    > {
        let api_key_metadata = self.api_key_metadata()?;
        Ok(TaprootSwapperClient::with_interceptor(
            self.grpc_client.lock().await.clone().into_inner(),
            ApiKeyInterceptor { api_key_metadata },
        ))
    }

    pub async fn ping(&self) -> anyhow::Result<String> {
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
            .into_iter()
            .filter(|s| s.server_type == "MEMPOOL_SPACE")
            .map(|s| s.server_base_url)
            .collect();
        trace!("Received mempoolspace_urls: {mempoolspace_urls:?}");

        Ok(mempoolspace_urls)
    }

    pub async fn fetch_boltz_swapper_urls(
        &self,
    ) -> Result<BoltzSwapperUrls, ServiceConnectivityError> {
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

        let boltz_url = chain_api_servers
            .iter()
            .find(|s| s.server_type == "BOLTZ_SWAPPER")
            .map(|s| s.server_base_url.clone())
            .ok_or(ServiceConnectivityError::new(
                ServiceConnectivityErrorKind::Other,
                "Failed to find boltz url".to_string(),
            ))?;

        let proxy_url = chain_api_servers
            .iter()
            .find(|s| s.server_type == "GENERAL_SWAPPER")
            .map(|s| s.server_base_url.clone())
            .ok_or(ServiceConnectivityError::new(
                ServiceConnectivityErrorKind::Other,
                "Failed to find boltz proxy url".to_string(),
            ))?;
        trace!("Received boltz_url: {boltz_url:?}, proxy_url: {proxy_url:?}");

        Ok(BoltzSwapperUrls {
            boltz_url,
            proxy_url,
        })
    }
}

#[derive(Clone)]
pub struct ApiKeyInterceptor {
    api_key_metadata: Option<MetadataValue<Ascii>>,
}

impl Interceptor for ApiKeyInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        if let Some(api_key_metadata) = &self.api_key_metadata {
            req.metadata_mut()
                .insert("authorization", api_key_metadata.clone());
        }
        Ok(req)
    }
}
