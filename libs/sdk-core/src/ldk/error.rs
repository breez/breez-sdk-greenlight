use sdk_common::prelude::InvoiceError;

use crate::node_api::NodeError;
use crate::persist::error::PersistError;

impl From<ldk_node::NodeError> for NodeError {
    fn from(err: ldk_node::NodeError) -> Self {
        let msg = |e: ldk_node::NodeError| format!("LDK Node error: {e}");
        let generic = |e: ldk_node::NodeError| NodeError::Generic(msg(e));

        match err {
            ldk_node::NodeError::AlreadyRunning => generic(err),
            ldk_node::NodeError::NotRunning => generic(err),

            ldk_node::NodeError::ConnectionFailed => NodeError::ServiceConnectivity(msg(err)),

            ldk_node::NodeError::OnchainTxCreationFailed => generic(err),
            ldk_node::NodeError::OnchainTxSigningFailed => generic(err),

            ldk_node::NodeError::InvoiceCreationFailed => generic(err),
            ldk_node::NodeError::InvoiceRequestCreationFailed => generic(err),
            ldk_node::NodeError::OfferCreationFailed => generic(err),
            ldk_node::NodeError::RefundCreationFailed => generic(err),

            ldk_node::NodeError::PaymentSendingFailed => NodeError::PaymentFailed(msg(err)),
            ldk_node::NodeError::InvalidCustomTlvs => generic(err),
            ldk_node::NodeError::ProbeSendingFailed => generic(err),

            ldk_node::NodeError::ChannelCreationFailed => generic(err),
            ldk_node::NodeError::ChannelClosingFailed => generic(err),
            ldk_node::NodeError::ChannelConfigUpdateFailed => generic(err),

            ldk_node::NodeError::PersistenceFailed => PersistError::Generic(msg(err)).into(),

            ldk_node::NodeError::FeerateEstimationUpdateFailed => {
                NodeError::ServiceConnectivity(msg(err))
            }
            ldk_node::NodeError::FeerateEstimationUpdateTimeout => {
                NodeError::ServiceConnectivity(msg(err))
            }

            ldk_node::NodeError::WalletOperationFailed => generic(err),
            ldk_node::NodeError::WalletOperationTimeout => NodeError::ServiceConnectivity(msg(err)),

            ldk_node::NodeError::TxSyncTimeout => NodeError::ServiceConnectivity(msg(err)),
            ldk_node::NodeError::TxSyncFailed => generic(err),

            ldk_node::NodeError::GossipUpdateTimeout => NodeError::ServiceConnectivity(msg(err)),
            ldk_node::NodeError::GossipUpdateFailed => NodeError::ServiceConnectivity(msg(err)),

            ldk_node::NodeError::LiquidityRequestFailed => generic(err),

            ldk_node::NodeError::UriParameterParsingFailed => generic(err),

            ldk_node::NodeError::InvalidAddress => generic(err),
            ldk_node::NodeError::InvalidAmount => generic(err),
            ldk_node::NodeError::InvalidChannelId => generic(err),
            ldk_node::NodeError::InvalidDateTime => generic(err),
            ldk_node::NodeError::InvalidFeeRate => generic(err),
            ldk_node::NodeError::InvalidInvoice => InvoiceError::Generic(msg(err)).into(),
            ldk_node::NodeError::InvalidNetwork => InvoiceError::InvalidNetwork(msg(err)).into(),
            ldk_node::NodeError::InvalidNodeAlias => generic(err),
            ldk_node::NodeError::InvalidNodeId => generic(err),
            ldk_node::NodeError::InvalidOffer => generic(err),
            ldk_node::NodeError::InvalidOfferId => generic(err),
            ldk_node::NodeError::InvalidPaymentHash => generic(err),
            ldk_node::NodeError::InvalidPaymentId => generic(err),
            ldk_node::NodeError::InvalidPaymentPreimage => generic(err),
            ldk_node::NodeError::InvalidPaymentSecret => generic(err),
            ldk_node::NodeError::InvalidPublicKey => generic(err),
            ldk_node::NodeError::InvalidQuantity => generic(err),
            ldk_node::NodeError::InvalidRefund => generic(err),
            ldk_node::NodeError::InvalidSecretKey => generic(err),
            ldk_node::NodeError::InvalidSocketAddress => generic(err),
            ldk_node::NodeError::InvalidUri => generic(err),

            ldk_node::NodeError::DuplicatePayment => NodeError::InvoiceAlreadyPaid,

            ldk_node::NodeError::UnsupportedCurrency => generic(err),

            ldk_node::NodeError::InsufficientFunds => NodeError::InsufficientFunds(msg(err)),

            ldk_node::NodeError::LiquiditySourceUnavailable => generic(err),
            ldk_node::NodeError::LiquidityFeeTooHigh => generic(err),
        }
    }
}
