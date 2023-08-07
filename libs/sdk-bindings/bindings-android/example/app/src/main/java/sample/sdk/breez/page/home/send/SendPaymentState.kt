package sample.sdk.breez.page.home.send

import breez_sdk.Payment

sealed class SendPaymentState {

    object Initial : SendPaymentState()

    object Loading : SendPaymentState()

    data class Error(
        val throwable: Throwable,
    ) : SendPaymentState()

    data class Success(
        val payment: Payment,
    ) : SendPaymentState()

}
