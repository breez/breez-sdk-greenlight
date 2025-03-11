use sdk_common::{
    grpc::{
        CreateSwapRequest, CreateSwapResponse, PaySwapRequest, PaySwapResponse, RefundSwapRequest,
        RefundSwapResponse, SwapParameters, SwapParametersRequest,
    },
    prelude::BreezServer,
    with_connection_retry,
};
use tonic::{async_trait, Status};

use crate::error::SdkResult;

#[async_trait]
pub(crate) trait TaprootSwapperAPI: Send + Sync {
    async fn create_swap(
        &self,
        hash: Vec<u8>,
        refund_pubkey: Vec<u8>,
    ) -> SdkResult<CreateSwapResponse>;
    async fn pay_swap(&self, payment_request: String) -> Result<PaySwapResponse, Status>;
    async fn refund_swap(
        &self,
        address: String,
        input_index: u32,
        pub_nonce: Vec<u8>,
        transaction: Vec<u8>,
    ) -> SdkResult<RefundSwapResponse>;
    async fn swap_parameters(&self) -> SdkResult<Option<SwapParameters>>;
}

#[async_trait]
impl TaprootSwapperAPI for BreezServer {
    async fn create_swap(
        &self,
        hash: Vec<u8>,
        refund_pubkey: Vec<u8>,
    ) -> SdkResult<CreateSwapResponse> {
        let mut client = self.get_taproot_swapper_client().await;
        let req = CreateSwapRequest {
            hash,
            refund_pubkey,
        };
        Ok(with_connection_retry!(client.create_swap(req.clone()))
            .await?
            .into_inner())
    }
    async fn pay_swap(&self, payment_request: String) -> Result<PaySwapResponse, Status> {
        let mut client = self.get_taproot_swapper_client().await;
        let req = PaySwapRequest { payment_request };
        Ok(with_connection_retry!(client.pay_swap(req.clone()))
            .await?
            .into_inner())
    }
    async fn refund_swap(
        &self,
        address: String,
        input_index: u32,
        pub_nonce: Vec<u8>,
        transaction: Vec<u8>,
    ) -> SdkResult<RefundSwapResponse> {
        let mut client = self.get_taproot_swapper_client().await;
        let req = RefundSwapRequest {
            address,
            input_index,
            pub_nonce,
            transaction,
        };
        Ok(with_connection_retry!(client.refund_swap(req.clone()))
            .await?
            .into_inner())
    }
    async fn swap_parameters(&self) -> SdkResult<Option<SwapParameters>> {
        let mut client = self.get_taproot_swapper_client().await;

        Ok(
            with_connection_retry!(client.swap_parameters(SwapParametersRequest {}))
                .await?
                .into_inner()
                .parameters,
        )
    }
}
