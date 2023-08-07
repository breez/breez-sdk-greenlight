package sample.sdk.breez.page.home.receive

import breez_sdk.LnInvoice

sealed class ReceiveState {

    object Initial : ReceiveState()

    object Loading : ReceiveState()

    data class Error(
        val throwable: Throwable,
    ) : ReceiveState()

    data class Success(
        val invoice: LnInvoice,
        val openingChannelFee: Long?,
    ) : ReceiveState()

}
