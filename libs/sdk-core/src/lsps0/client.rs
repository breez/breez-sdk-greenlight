use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;

use super::{error::Error, transport::Transport};

#[derive(Debug, Serialize)]
pub struct ListProtocolsRequest {}

#[derive(Debug, Deserialize)]
pub struct ListProtocolsResponse {
    #[allow(dead_code)]
    pub protocols: Vec<i32>,
}

pub struct Client {
    transport: Arc<Transport>,
    peer_id: Vec<u8>,
    timeout: Duration,
}

impl Client {
    #[allow(dead_code)]
    pub fn new(transport: Arc<Transport>, peer_id: Vec<u8>, timeout: Duration) -> Self {
        Self {
            transport,
            peer_id,
            timeout,
        }
    }

    pub async fn call<TRequest, TResponse>(
        &self,
        method: String,
        req: TRequest,
    ) -> Result<TResponse, Error>
    where
        TRequest: serde::Serialize,
        TResponse: serde::de::DeserializeOwned,
    {
        self.transport
            .request_response(method, self.peer_id.clone(), &req, self.timeout)
            .await
    }

    #[allow(dead_code)]
    pub async fn stream_notifications<TNotification>(
        &self,
        method: String,
    ) -> Result<mpsc::Receiver<TNotification>, Error>
    where
        TNotification: serde::de::DeserializeOwned + std::marker::Send + 'static,
    {
        self.transport
            .stream_notifications(method, self.peer_id.clone())
            .await
    }

    #[allow(dead_code)]
    pub async fn list_protocols(&self) -> Result<ListProtocolsResponse, Error> {
        self.call(
            String::from("lsps0.list_protocols"),
            ListProtocolsRequest {},
        )
        .await
    }
}
