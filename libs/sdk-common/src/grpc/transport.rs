use anyhow::Result;
use std::time::Duration;

pub type Transport = tonic::transport::Channel;

#[derive(Clone)]
pub struct GrpcClient {
    inner: Transport,
}

impl GrpcClient {
    pub fn new(url: String) -> Result<Self> {
        Ok(Self {
            inner: Self::create_endpoint(&url)?.connect_lazy(),
        })
    }

    pub fn into_inner(self) -> Transport {
        self.inner
    }

    fn create_endpoint(server_url: &str) -> Result<tonic::transport::Endpoint> {
        Ok(
            tonic::transport::Endpoint::from_shared(server_url.to_string())?
                .http2_keep_alive_interval(Duration::new(5, 0))
                .tcp_keepalive(Some(Duration::from_secs(5)))
                .keep_alive_timeout(Duration::from_secs(5))
                .keep_alive_while_idle(true),
        )
    }
}
