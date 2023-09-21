extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcServerMessage {
    pub jsonrpc: String,

    #[serde(flatten)]
    pub body: RpcServerMessageBody,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RpcServerMessageBody {
    Notification { method: String, params: Value },
    Response { id: String, result: Value },
    Error { id: String, error: RpcError },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest<TParams> {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: TParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<Value>,
}

#[cfg(test)]
mod tests {
    use crate::lsps0::jsonrpc::{RpcServerMessage, RpcServerMessageBody};

    #[test]
    fn test_deserialize_notification() {
        let json = r#"{"jsonrpc":"2.0","method":"test","params":{}}"#;
        let notification = serde_json::from_str::<RpcServerMessage>(json).unwrap();
        assert!(matches!(
            notification.body,
            RpcServerMessageBody::Notification { .. }
        ))
    }

    #[test]
    fn test_deserialize_response() {
        let json = r#"{"jsonrpc":"2.0","id":"test","result":{}}"#;
        let notification = serde_json::from_str::<RpcServerMessage>(json).unwrap();
        assert!(matches!(
            notification.body,
            RpcServerMessageBody::Response { .. }
        ))
    }

    #[test]
    fn test_deserialize_error() {
        let json = r#"{"jsonrpc":"2.0","id":"test","error":{"code":1,"message":"test","data":{}}}"#;
        let error = serde_json::from_str::<RpcServerMessage>(json).unwrap();
        assert!(matches!(error.body, RpcServerMessageBody::Error { .. }))
    }

    #[test]
    fn test_deserialize_error_without_data() {
        let json = r#"{"jsonrpc":"2.0","id":"test","error":{"code":1,"message":"test"}}"#;
        let error = serde_json::from_str::<RpcServerMessage>(json).unwrap();
        assert!(matches!(error.body, RpcServerMessageBody::Error { .. }))
    }
}
