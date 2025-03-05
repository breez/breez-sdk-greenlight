use anyhow::Result;

pub type Transport = tonic_web_wasm_client::Client;

#[derive(Clone)]
pub struct GrpcClient {
    inner: Transport,
}

impl GrpcClient {
    pub fn new(url: String) -> Result<Self> {
        Ok(Self {
            inner: tonic_web_wasm_client::Client::new(url),
        })
    }

    pub fn into_inner(self) -> Transport {
        self.inner
    }
}
