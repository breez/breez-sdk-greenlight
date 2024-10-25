use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::node_api::NodeAPI;
use crate::CustomMessage;
use anyhow::{anyhow, Result};
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use serde::de::DeserializeOwned;
use tokio::sync::watch;
use tokio::sync::{mpsc, oneshot, Mutex, RwLock};
use tokio::time::sleep;
use tokio_stream::StreamExt;

use super::error::Error;
use super::jsonrpc::RpcServerMessageBody;
use super::jsonrpc::{RpcError, RpcRequest, RpcServerMessage};

const LSPS0_MESSAGE_TYPE: u16 = 37913;
const JSONRPC_VERSION: &str = "2.0";

#[tonic::async_trait]
trait NotificationSender: Send + Sync {
    async fn send(&self, params: serde_json::Value) -> Result<(), Error>;
}
struct NotificationHandler<TNotification>
where
    TNotification: DeserializeOwned + Send,
{
    tx: mpsc::Sender<TNotification>,
}

#[tonic::async_trait]
impl<TNotification> NotificationSender for NotificationHandler<TNotification>
where
    TNotification: DeserializeOwned + Send,
{
    async fn send(&self, params: serde_json::Value) -> Result<(), Error> {
        let n = match serde_json::from_value::<TNotification>(params) {
            Ok(n) => n,
            Err(e) => return Err(Error::Deserialization(e)),
        };

        match self.tx.send(n).await {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::Local(anyhow!("receiver dropped"))),
        }
    }
}

struct ResponseOrError {
    response: Option<serde_json::Value>,
    error: Option<RpcError>,
}

/// Transport sends and receives LSPS0 messages to/from remote nodes. One
/// user node has one Transport.
pub struct Transport {
    node: Arc<dyn NodeAPI>,
    reconnect_interval: Duration,
    response_handlers: Mutex<HashMap<String, oneshot::Sender<ResponseOrError>>>,
    notification_handlers: RwLock<HashMap<String, Box<dyn NotificationSender>>>,
}

impl Transport {
    #[allow(dead_code)]
    pub fn new(node: Arc<dyn NodeAPI>) -> Transport {
        Transport {
            node,
            response_handlers: Mutex::new(HashMap::new()),
            notification_handlers: RwLock::new(HashMap::new()),
            reconnect_interval: Duration::from_secs(1),
        }
    }

    #[allow(dead_code)]
    pub fn start(self: &Arc<Transport>, cancel: watch::Receiver<()>) {
        debug!("starting lsps0 transport.");
        let cloned = self.clone();
        tokio::spawn(async move {
            loop {
                let mut cancel = cancel.clone();
                if cancel.has_changed().unwrap_or(true) {
                    return;
                }

                debug!("lsps0 transport connecting to custom message stream.");
                let mut stream = match cloned.node.stream_custom_messages().await {
                    Ok(s) => s,
                    Err(err) => {
                        warn!(
                            "lsps0 transport failed to connect to custom message stream: {}. Retrying in {:?}", 
                            err,
                            cloned.reconnect_interval);
                        break;
                    }
                };
                loop {
                    tokio::select! {
                        _ = cancel.changed() => {
                            debug!("lsps0 tranport cancelled.");
                            return;
                        }
                        msg = stream.next() => {
                            let msg = match msg {
                                Some(msg) => match msg {
                                    Ok(msg) => msg,
                                    Err(e) => {
                                        warn!("connection to custom message stream errored: {}", e);
                                        break;
                                    }
                                },
                                None => {
                                    warn!("connection to custom message stream dropped");
                                    break
                                }
                            };

                            cloned.handle_message(msg).await;
                        }
                    }
                }

                sleep(cloned.reconnect_interval).await;
            }
        });
    }

    async fn handle_message(&self, msg: CustomMessage) {
        if msg.message_type != LSPS0_MESSAGE_TYPE {
            debug!("received custom message that was not lsps0: {:?}", msg);
            return;
        }

        let v: RpcServerMessage = match serde_json::from_slice(&msg.payload) {
            Ok(v) => v,
            Err(e) => {
                warn!(
                    "error deserializing lsps0 payload {:?}: {}",
                    &msg.payload, e
                );
                return;
            }
        };

        if v.jsonrpc != JSONRPC_VERSION {
            warn!(
                "error deserializing lsps0 payload {:?}: Invalid jsonrpc version. Expected {:?}.",
                &msg.payload, JSONRPC_VERSION
            );
            return;
        }

        match v.body {
            RpcServerMessageBody::Notification { method, params } => {
                let id = get_notification_handler_id(&method, msg.peer_id.clone());
                if let Some(tx) = (*self.notification_handlers.read().await).get(&id) {
                    match tx.send(params).await {
                        Ok(_) => (),
                        Err(e) => match e {
                            Error::Deserialization(e) => {
                                // TODO: Drop connection to LSP?
                                warn!(
                                    "LSPS0: Got invalid notification {:?} for id {}: {}",
                                    msg, id, e
                                );
                            }
                            _ => {
                                debug!("LSPS0: Notification listener dropped for id {}", id);
                                let mut notification_handlers =
                                    self.notification_handlers.write().await;
                                (*notification_handlers).remove(&id);
                            }
                        },
                    }
                } else {
                    info!(
                        "LSPS0: got notification without listener: method: {}, params: {:?}",
                        method, params
                    );
                }
            }
            RpcServerMessageBody::Response { id, result } => {
                let handler_id = get_request_handler_id(&id, msg.peer_id);
                if let Some(tx) = (*self.response_handlers.lock().await).remove(&handler_id) {
                    if tx
                        .send(ResponseOrError {
                            response: Some(result),
                            error: None,
                        })
                        .is_err()
                    {
                        debug!("LSPS0: got response, but listener dropped");
                    }
                } else {
                    debug!(
                        "LSPS0: got response without listener: id: {}, result: {:?}",
                        id, result
                    );
                }
            }
            RpcServerMessageBody::Error { id, error } => {
                let handler_id = get_request_handler_id(&id, msg.peer_id);
                if let Some(tx) = (*self.response_handlers.lock().await).remove(&handler_id) {
                    if tx
                        .send(ResponseOrError {
                            response: None,
                            error: Some(error),
                        })
                        .is_err()
                    {
                        debug!("LSPS0: got error response, but listener dropped");
                    }
                } else {
                    debug!(
                        "LSPS0: got error without listener: id: {}, error: {:?}",
                        id, error
                    );
                }
            }
        }
    }

    pub async fn request_response<TRequest, TResponse>(
        &self,
        method: String,
        peer_id: Vec<u8>,
        req: &TRequest,
        timeout: Duration,
    ) -> Result<TResponse, Error>
    where
        TRequest: serde::Serialize,
        TResponse: serde::de::DeserializeOwned,
    {
        let request_id = generate_request_id();
        let wrapped_req = RpcRequest {
            id: request_id.clone(),
            jsonrpc: String::from(JSONRPC_VERSION),
            method,
            params: req,
        };
        let payload = serde_json::to_vec(&wrapped_req)?;
        let (tx, rx) = oneshot::channel();
        let handler_id = get_request_handler_id(&request_id, peer_id.clone());
        (*self.response_handlers.lock().await).insert(handler_id.clone(), tx);

        if let Err(e) = self
            .node
            .send_custom_message(CustomMessage {
                peer_id,
                message_type: LSPS0_MESSAGE_TYPE,
                payload,
            })
            .await
        {
            (*self.response_handlers.lock().await).remove(&handler_id);
            return Err(e.into());
        }

        let result_or = tokio::time::timeout(timeout, rx).await?;
        let response_or_error = result_or?;
        if let Some(response) = response_or_error.response {
            let resp = serde_json::from_value::<TResponse>(response)?;
            Ok(resp)
        } else if let Some(error) = response_or_error.error {
            Err(Error::Remote(error))
        } else {
            Err(Error::Local(anyhow!("did not get response or error")))
        }
    }

    pub async fn stream_notifications<TNotification>(
        &self,
        method: String,
        node_id: Vec<u8>,
    ) -> Result<mpsc::Receiver<TNotification>, Error>
    where
        TNotification: serde::de::DeserializeOwned + std::marker::Send + 'static,
    {
        let (tx, rx) = mpsc::channel(100);
        let id = get_notification_handler_id(&method, node_id);
        (*self.notification_handlers.write().await)
            .insert(id, Box::new(NotificationHandler::<TNotification> { tx }));

        Ok(rx)
    }
}

fn generate_request_id() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 21)
}

fn get_request_handler_id(request_id: &str, node_id: Vec<u8>) -> String {
    let mut id = hex::encode(node_id);
    id.push('|');
    id.push_str(request_id);
    id
}

fn get_notification_handler_id(method: &str, node_id: Vec<u8>) -> String {
    let mut id = hex::encode(node_id);
    id.push('|');
    id.push_str(method);
    id
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::{sync::Arc, time::Duration};
    use tokio::sync::{mpsc, watch};

    use crate::{
        internal_breez_services::tests::get_dummy_node_state,
        lsps0::{
            error::Error,
            jsonrpc::{RpcError, RpcRequest, RpcServerMessage, RpcServerMessageBody},
            transport::LSPS0_MESSAGE_TYPE,
        },
        test_utils::MockNodeAPI,
        CustomMessage,
    };

    use super::Transport;

    #[derive(Serialize, Deserialize)]
    pub struct Request {}

    #[derive(Serialize, Deserialize)]
    pub struct Response {}

    #[derive(Serialize, Deserialize)]
    pub struct Notification {}

    #[tokio::test]
    async fn test_request_response_success() {
        let peer_id = vec![21];
        let peer_id_clone = peer_id.clone();
        let (tx, rx) = mpsc::channel(1);
        let tx_arc = Arc::new(tx);
        let on_send_request = move |message: CustomMessage| {
            let req = serde_json::from_slice::<RpcRequest<Request>>(&message.payload).unwrap();
            let resp = RpcServerMessage {
                jsonrpc: req.jsonrpc,
                body: RpcServerMessageBody::Response {
                    id: req.id,
                    result: json!({}),
                },
            };
            let raw_resp = serde_json::to_vec(&resp).unwrap();
            let tx_arc = tx_arc.clone();
            let peer_id = peer_id_clone.clone();
            tokio::spawn(async move {
                tx_arc
                    .send(CustomMessage {
                        message_type: LSPS0_MESSAGE_TYPE,
                        payload: raw_resp,
                        peer_id,
                    })
                    .await
                    .unwrap();
            });
            Ok(())
        };

        let mut node_api = MockNodeAPI::new(get_dummy_node_state());
        node_api.set_on_send_custom_message(Box::new(on_send_request));
        node_api.set_on_stream_custom_messages(rx).await;

        let transport = Arc::new(Transport::new(Arc::new(node_api)));
        let (stop, cancel) = watch::channel(());
        transport.start(cancel);
        let timeout = Duration::from_millis(10);
        transport
            .request_response::<Request, Response>(
                String::from("test"),
                peer_id.clone(),
                &Request {},
                timeout,
            )
            .await
            .unwrap();
        let _ = stop.send(());
    }

    #[tokio::test]
    async fn test_request_response_timeout() {
        let peer_id = vec![21];
        let node_api = MockNodeAPI::new(get_dummy_node_state());
        let transport = Arc::new(Transport::new(Arc::new(node_api)));
        let (stop, cancel) = watch::channel(());
        transport.start(cancel);
        let timeout = Duration::from_millis(10);
        let result = transport
            .request_response::<Request, Response>(
                String::from("test"),
                peer_id.clone(),
                &Request {},
                timeout,
            )
            .await;

        assert!(matches!(result.err().unwrap(), Error::Timeout));
        let _ = stop.send(());
    }

    #[tokio::test]
    async fn test_response_from_different_node() {
        let (tx, rx) = mpsc::channel(1);
        let tx_arc = Arc::new(tx);
        let on_send_request = move |message: CustomMessage| {
            let req = serde_json::from_slice::<RpcRequest<Request>>(&message.payload).unwrap();
            let resp = RpcServerMessage {
                jsonrpc: req.jsonrpc,
                body: RpcServerMessageBody::Response {
                    id: req.id,
                    result: json!({}),
                },
            };
            let raw_resp = serde_json::to_vec(&resp).unwrap();
            let tx_arc = tx_arc.clone();
            tokio::spawn(async move {
                tx_arc
                    .send(CustomMessage {
                        message_type: LSPS0_MESSAGE_TYPE,
                        payload: raw_resp,
                        peer_id: vec![22],
                    })
                    .await
                    .unwrap();
            });
            Ok(())
        };

        let mut node_api = MockNodeAPI::new(get_dummy_node_state());
        node_api.set_on_send_custom_message(Box::new(on_send_request));
        node_api.set_on_stream_custom_messages(rx).await;

        let transport = Arc::new(Transport::new(Arc::new(node_api)));
        let (stop, cancel) = watch::channel(());
        transport.start(cancel);
        let timeout = Duration::from_millis(10);
        let result = transport
            .request_response::<Request, Response>(
                String::from("test"),
                vec![21],
                &Request {},
                timeout,
            )
            .await;

        assert!(matches!(result.err().unwrap(), Error::Timeout));
        let _ = stop.send(());
    }

    #[tokio::test]
    async fn test_request_response_remote_error() {
        let peer_id = vec![21];
        let peer_id_clone = peer_id.clone();
        let (tx, rx) = mpsc::channel(1);
        let tx_arc = Arc::new(tx);
        let on_send_request = move |message: CustomMessage| {
            let req = serde_json::from_slice::<RpcRequest<Request>>(&message.payload).unwrap();
            let resp = RpcServerMessage {
                jsonrpc: req.jsonrpc,
                body: RpcServerMessageBody::Error {
                    id: req.id,
                    error: RpcError {
                        code: 1,
                        data: Some(json!({})),
                        message: String::from("error occurred"),
                    },
                },
            };
            let raw_resp = serde_json::to_vec(&resp).unwrap();
            let tx_arc = tx_arc.clone();
            let peer_id = peer_id_clone.clone();
            tokio::spawn(async move {
                tx_arc
                    .send(CustomMessage {
                        message_type: LSPS0_MESSAGE_TYPE,
                        payload: raw_resp,
                        peer_id,
                    })
                    .await
                    .unwrap();
            });
            Ok(())
        };

        let mut node_api = MockNodeAPI::new(get_dummy_node_state());
        node_api.set_on_send_custom_message(Box::new(on_send_request));
        node_api.set_on_stream_custom_messages(rx).await;

        let transport = Arc::new(Transport::new(Arc::new(node_api)));
        let (stop, cancel) = watch::channel(());
        transport.start(cancel);
        let timeout = Duration::from_millis(10);
        let result = transport
            .request_response::<Request, Response>(
                String::from("test"),
                peer_id.clone(),
                &Request {},
                timeout,
            )
            .await;

        let err = result.err().unwrap();
        assert!(matches!(err, Error::Remote { .. }));
        match err {
            Error::Remote(e) => {
                assert_eq!(e.code, 1);
                assert_eq!(e.message, String::from("error occurred"));
                assert_eq!(e.data, Some(json!({})));
            }
            _ => unreachable!(),
        };
        let _ = stop.send(());
    }

    #[tokio::test]
    async fn test_request_response_deserialization_error() {
        let peer_id = vec![21];
        let peer_id_clone = peer_id.clone();
        let (tx, rx) = mpsc::channel(1);
        let tx_arc = Arc::new(tx);
        let on_send_request = move |message: CustomMessage| {
            let req = serde_json::from_slice::<RpcRequest<Request>>(&message.payload).unwrap();
            let resp = RpcServerMessage {
                jsonrpc: req.jsonrpc,
                body: RpcServerMessageBody::Response {
                    id: req.id,
                    result: json!("cannot deserialize this"),
                },
            };
            let raw_resp = serde_json::to_vec(&resp).unwrap();
            let tx_arc = tx_arc.clone();
            let peer_id = peer_id_clone.clone();
            tokio::spawn(async move {
                tx_arc
                    .send(CustomMessage {
                        message_type: LSPS0_MESSAGE_TYPE,
                        payload: raw_resp,
                        peer_id,
                    })
                    .await
                    .unwrap();
            });
            Ok(())
        };

        let mut node_api = MockNodeAPI::new(get_dummy_node_state());
        node_api.set_on_send_custom_message(Box::new(on_send_request));
        node_api.set_on_stream_custom_messages(rx).await;

        let transport = Arc::new(Transport::new(Arc::new(node_api)));
        let (stop, cancel) = watch::channel(());
        transport.start(cancel);
        let timeout = Duration::from_millis(10);
        let result = transport
            .request_response::<Request, Response>(
                String::from("test"),
                peer_id.clone(),
                &Request {},
                timeout,
            )
            .await;

        let err = result.err().unwrap();
        assert!(matches!(err, Error::Deserialization { .. }));
        let _ = stop.send(());
    }

    #[tokio::test]
    async fn test_request_response_different_id() {
        let peer_id = vec![21];
        let peer_id_clone = peer_id.clone();
        let (tx, rx) = mpsc::channel(1);
        let tx_arc = Arc::new(tx);
        let on_send_request = move |message: CustomMessage| {
            let req = serde_json::from_slice::<RpcRequest<Request>>(&message.payload).unwrap();
            let resp = RpcServerMessage {
                jsonrpc: req.jsonrpc,
                body: RpcServerMessageBody::Response {
                    id: String::from("different id"),
                    result: json!({}),
                },
            };
            let raw_resp = serde_json::to_vec(&resp).unwrap();
            let tx_arc = tx_arc.clone();
            let peer_id = peer_id_clone.clone();
            tokio::spawn(async move {
                tx_arc
                    .send(CustomMessage {
                        message_type: LSPS0_MESSAGE_TYPE,
                        payload: raw_resp,
                        peer_id,
                    })
                    .await
                    .unwrap();
            });
            Ok(())
        };

        let mut node_api = MockNodeAPI::new(get_dummy_node_state());
        node_api.set_on_send_custom_message(Box::new(on_send_request));
        node_api.set_on_stream_custom_messages(rx).await;

        let transport = Arc::new(Transport::new(Arc::new(node_api)));
        let (stop, cancel) = watch::channel(());
        transport.start(cancel);
        let timeout = Duration::from_millis(10);
        let result = transport
            .request_response::<Request, Response>(
                String::from("test"),
                peer_id.clone(),
                &Request {},
                timeout,
            )
            .await;
        assert!(matches!(result.err().unwrap(), Error::Timeout));
        let _ = stop.send(());
    }

    #[tokio::test]
    async fn test_request_response_not_lsps0() {
        let peer_id = vec![21];
        let peer_id_clone = peer_id.clone();
        let (tx, rx) = mpsc::channel(1);
        let tx_arc = Arc::new(tx);
        let on_send_request = move |message: CustomMessage| {
            let req = serde_json::from_slice::<RpcRequest<Request>>(&message.payload).unwrap();
            let resp = RpcServerMessage {
                jsonrpc: req.jsonrpc,
                body: RpcServerMessageBody::Response {
                    id: req.id,
                    result: json!({}),
                },
            };
            let raw_resp = serde_json::to_vec(&resp).unwrap();
            let tx_arc = tx_arc.clone();
            let peer_id = peer_id_clone.clone();
            tokio::spawn(async move {
                tx_arc
                    .send(CustomMessage {
                        message_type: LSPS0_MESSAGE_TYPE + 1,
                        payload: raw_resp,
                        peer_id,
                    })
                    .await
                    .unwrap();
            });
            Ok(())
        };

        let mut node_api = MockNodeAPI::new(get_dummy_node_state());
        node_api.set_on_send_custom_message(Box::new(on_send_request));
        node_api.set_on_stream_custom_messages(rx).await;

        let transport = Arc::new(Transport::new(Arc::new(node_api)));
        let (stop, cancel) = watch::channel(());
        transport.start(cancel);
        let timeout = Duration::from_millis(10);
        let result = transport
            .request_response::<Request, Response>(
                String::from("test"),
                peer_id.clone(),
                &Request {},
                timeout,
            )
            .await;
        assert!(matches!(result.err().unwrap(), Error::Timeout));
        let _ = stop.send(());
    }

    #[tokio::test]
    async fn test_notification_success() {
        let method = String::from("test");
        let peer_id = vec![21];
        let (tx, rx) = mpsc::channel(1);
        let mut node_api = MockNodeAPI::new(get_dummy_node_state());
        node_api.set_on_stream_custom_messages(rx).await;

        let transport = Arc::new(Transport::new(Arc::new(node_api)));
        let (stop, cancel) = watch::channel(());
        transport.start(cancel);
        let mut stream = transport
            .stream_notifications::<Notification>(method, peer_id.clone())
            .await
            .unwrap();
        let payload = RpcServerMessage {
            jsonrpc: String::from("2.0"),
            body: RpcServerMessageBody::Notification {
                method: String::from("test"),
                params: json!({}),
            },
        };
        let raw_payload = serde_json::to_vec(&payload).unwrap();
        tx.send(CustomMessage {
            message_type: LSPS0_MESSAGE_TYPE,
            payload: raw_payload,
            peer_id: peer_id.clone(),
        })
        .await
        .unwrap();
        stream.recv().await.unwrap();
        let _ = stop.send(());
    }

    #[tokio::test]
    async fn test_notification_different_node() {
        let method = String::from("test");
        let peer_id = vec![21];
        let (tx, rx) = mpsc::channel(1);
        let mut node_api = MockNodeAPI::new(get_dummy_node_state());
        node_api.set_on_stream_custom_messages(rx).await;

        let transport = Arc::new(Transport::new(Arc::new(node_api)));
        let (stop, cancel) = watch::channel(());
        transport.start(cancel);
        let mut stream = transport
            .stream_notifications::<Notification>(method, peer_id.clone())
            .await
            .unwrap();
        let payload = RpcServerMessage {
            jsonrpc: String::from("2.0"),
            body: RpcServerMessageBody::Notification {
                method: String::from("test"),
                params: json!({}),
            },
        };
        let raw_payload = serde_json::to_vec(&payload).unwrap();
        tx.send(CustomMessage {
            message_type: LSPS0_MESSAGE_TYPE,
            payload: raw_payload,
            peer_id: vec![22],
        })
        .await
        .unwrap();
        let a = stream.try_recv();
        assert!(a.is_err());
        let _ = stop.send(());
    }
}
