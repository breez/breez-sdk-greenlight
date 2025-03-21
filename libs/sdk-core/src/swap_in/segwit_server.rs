use sdk_common::{grpc::GetSwapPaymentRequest, prelude::BreezServer, with_connection_retry};

use crate::SwapperAPI;

#[cfg_attr(test, mockall::automock)]
#[tonic::async_trait]
impl SwapperAPI for BreezServer {
    async fn complete_swap(&self, bolt11: String) -> anyhow::Result<()> {
        let mut client = self.get_swapper_client().await;
        let req = GetSwapPaymentRequest {
            payment_request: bolt11,
        };
        let resp = with_connection_retry!(client.get_swap_payment(req.clone()))
            .await?
            .into_inner();

        match resp.swap_error() {
            crate::grpc::get_swap_payment_reply::SwapError::NoError => Ok(()),
            err => Err(anyhow::anyhow!(err.as_str_name())),
        }
    }
}
