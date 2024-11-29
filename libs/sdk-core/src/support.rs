use std::time::SystemTime;

use crate::error::{SdkError, SdkResult};
use crate::{HealthCheckStatus, NodeState, Payment, ServiceHealthCheckResponse, SupportAPI};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use sdk_common::grpc::{BreezStatusRequest, ReportPaymentFailureRequest};
use sdk_common::prelude::BreezServer;
use sdk_common::tonic_wrap::with_connection_fallback;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PaymentFailureReport {
    pub node_state: NodeState,
    pub payment: Payment,
}

impl TryFrom<i32> for HealthCheckStatus {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HealthCheckStatus::Operational),
            1 => Ok(HealthCheckStatus::Maintenance),
            2 => Ok(HealthCheckStatus::ServiceDisruption),
            _ => Err(anyhow!("illegal value")),
        }
    }
}

#[tonic::async_trait]
impl SupportAPI for BreezServer {
    async fn service_health_check(&self) -> SdkResult<ServiceHealthCheckResponse> {
        let mut client = self.get_support_client().await?;
        let mut client_clone = client.clone();

        let request = BreezStatusRequest {};
        let response = with_connection_fallback(client.breez_status(request.clone()), || {
            client_clone.breez_status(request)
        })
        .await
        .map_err(|e| SdkError::ServiceConnectivity {
            err: format!("(Breez) Fetch status failed: {e}"),
        })?;
        let status = response.into_inner().status.try_into()?;
        Ok(ServiceHealthCheckResponse { status })
    }

    async fn report_payment_failure(
        &self,
        node_state: NodeState,
        payment: Payment,
        lsp_id: Option<String>,
        comment: Option<String>,
    ) -> SdkResult<()> {
        let mut client = self.get_support_client().await?;
        let mut client_clone = client.clone();

        let timestamp: DateTime<Utc> = SystemTime::now().into();
        let report = PaymentFailureReport {
            node_state: node_state.clone(),
            payment,
        };

        let request = ReportPaymentFailureRequest {
            sdk_version: option_env!("CARGO_PKG_VERSION")
                .unwrap_or_default()
                .to_string(),
            sdk_git_hash: option_env!("SDK_GIT_HASH").unwrap_or_default().to_string(),
            node_id: node_state.id,
            lsp_id: lsp_id.unwrap_or_default(),
            timestamp: timestamp.to_rfc3339(),
            comment: comment.unwrap_or_default(),
            report: serde_json::to_string(&report)?,
        };
        _ = with_connection_fallback(client.report_payment_failure(request.clone()), || {
            client_clone.report_payment_failure(request)
        })
        .await
        .map_err(|e| SdkError::ServiceConnectivity {
            err: format!("(Breez) Report payment failure failed: {e}"),
        })?;
        Ok(())
    }
}
