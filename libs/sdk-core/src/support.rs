use std::time::SystemTime;

use crate::error::SdkResult;
use crate::grpc::{BreezStatusRequest, ReportPaymentFailureRequest};
use crate::{breez_services::BreezServer, error::SdkError};
use crate::{BreezStatus, BreezStatusResponse, NodeState, Payment, SupportAPI};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tonic::Request;

#[derive(Serialize, Deserialize)]
struct PaymentFailureReport {
    pub node_state: NodeState,
    pub payment: Payment,
}

impl TryFrom<i32> for BreezStatus {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BreezStatus::Operational),
            1 => Ok(BreezStatus::Maintenance),
            2 => Ok(BreezStatus::ServiceDisruption),
            _ => Err(anyhow!("illegal value")),
        }
    }
}

#[tonic::async_trait]
impl SupportAPI for BreezServer {
    async fn fetch_breez_status(&self) -> SdkResult<BreezStatusResponse> {
        let mut client = self.get_support_client().await?;

        let request = Request::new(BreezStatusRequest {});
        let response =
            client
                .breez_status(request)
                .await
                .map_err(|e| SdkError::ServiceConnectivity {
                    err: format!("(Breez) Fetch status failed: {e}"),
                })?;
        let status = response.into_inner().status.try_into()?;
        Ok(BreezStatusResponse { status })
    }

    async fn report_payment_failure(
        &self,
        node_state: NodeState,
        payment: Payment,
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
            timestamp: timestamp.to_rfc3339(),
            comment: comment.unwrap_or_default(),
            report: serde_json::to_string(&report)?,
        });
        _ = client.report_payment_failure(request).await.map_err(|e| {
            SdkError::ServiceConnectivity {
                err: format!("(Breez) Report payment failure failed: {e}"),
            }
        })?;
        Ok(())
    }
}
