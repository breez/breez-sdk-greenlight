//! This is a thin layer that simplifies the use of `BreezServices` from Rust.

use std::sync::Arc;
use anyhow::{anyhow, Result};
use once_cell::sync::{Lazy, OnceCell};
use crate::{BreezEvent, BreezServices, EventListener, GreenlightCredentials, Network};

static BREEZ_SERVICES: OnceCell<Arc<BreezServices>> = OnceCell::new();
static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

pub async fn init_sdk_register(seed: &[u8]) -> Result<GreenlightCredentials> {
    let creds =
        BreezServices::register_node(Network::Bitcoin, seed.to_vec()).await?;
    init_sdk(seed, &creds).await?;
    Ok(creds)
}

pub async fn init_sdk_recover(seed: &[u8]) -> Result<GreenlightCredentials> {
    let creds =
        BreezServices::recover_node(Network::Bitcoin, seed.to_vec()).await?;
    init_sdk(seed, &creds).await?;
    Ok(creds)
}

pub async fn init_sdk(seed: &[u8], creds: &GreenlightCredentials) -> Result<()> {
    let service = BreezServices::init_services(
        None,
        seed.to_vec(),
        creds.clone(),
        Box::new(NoOpEventListener {}),
    )
        .await?;

    BREEZ_SERVICES
        .set(service)
        .map_err(|_| anyhow!("Failed to set Breez Service"))?;

    BreezServices::start(rt(), &sdk()?).await
}

pub fn sdk() -> Result<Arc<BreezServices>> {
    BREEZ_SERVICES
        .get()
        .ok_or("Breez Services not initialized")
        .map_err(|err| anyhow!(err))
        .cloned()
}

fn rt() -> &'static tokio::runtime::Runtime {
    &RT
}

struct NoOpEventListener {}
impl EventListener for NoOpEventListener {
    fn on_event(&self, _e: BreezEvent) { }
}