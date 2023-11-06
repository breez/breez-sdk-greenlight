use anyhow::anyhow;

use crate::node_api::NodeError;

use super::jsonrpc::RpcError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Local lsps0 error: {0}")]
    Local(anyhow::Error),

    #[error("Lsps0 deserialization error: {0}")]
    Deserialization(serde_json::Error),

    #[error("Lsps0 node: {0}")]
    Node(NodeError),

    #[error("Lsps0 request timed out")]
    Timeout,

    #[error("Lsps0 remote error: {0:?}")]
    Remote(RpcError),
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Local(value)
    }
}

impl From<NodeError> for Error {
    fn from(value: NodeError) -> Self {
        Self::Node(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Deserialization(value)
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(_value: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::Local(anyhow!("server lost"))
    }
}

impl From<tokio::time::error::Elapsed> for Error {
    fn from(_value: tokio::time::error::Elapsed) -> Self {
        Self::Timeout
    }
}
