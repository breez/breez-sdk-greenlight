use std::time::SystemTime;

use crate::error::SdkResult;
use crate::grpc::ReportPaymentFailureRequest;
use crate::{breez_services::BreezServer, error::SdkError};
use crate::{NodeState, Payment, SupportAPI};
use bitcoin::hashes::hex::ToHex;
use bitcoin::hashes::{sha256, Hash};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tonic::Request;

#[derive(Serialize, Deserialize)]
struct PaymentFailureReport {
    pub node_state: NodeState,
    pub payment: Payment,
}

#[tonic::async_trait]
impl SupportAPI for BreezServer {
    async fn report_payment_failure(
        &self,
        node_state: NodeState,
        payment: Payment,
        api_key: Option<String>,
        comment: Option<String>,
    ) -> SdkResult<()> {
        let mut client = self.get_support_client().await?;
        let timestamp: DateTime<Utc> = SystemTime::now().into();
        let report = PaymentFailureReport {
            node_state: node_state.clone(),
            payment,
        };

        let request = Request::new(ReportPaymentFailureRequest {
            node_id: node_state.id,
            api_key_hash: api_key
                .map(|api_key| sha256::Hash::hash(api_key.as_bytes()).to_hex())
                .unwrap_or_default(),
            timestamp: timestamp.to_rfc3339(),
            comment: comment.unwrap_or_default(),
            report: serde_json::to_string(&report)?,
        });
        _ = client.report_payment_failure(request).await.map_err(|e| {
            SdkError::ServiceConnectivity {
                err: format!("Report payment failure failed: {e}"),
            }
        })?;
        Ok(())
    }
}
