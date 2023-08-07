package sample.sdk.breez.page.home.spontaneous

import breez_sdk.Payment

sealed class SpontaneousState {

    object Initial : SpontaneousState()

    object Loading : SpontaneousState()

    data class Error(
        val throwable: Throwable,
    ) : SpontaneousState()

    data class Success(
        val payment: Payment,
    ) : SpontaneousState()

}
