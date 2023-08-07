package sample.sdk.breez.page.home.sendonchain

import breez_sdk.ReverseSwapInfo

sealed class SendOnChainState {

    object Initial : SendOnChainState()

    object Loading : SendOnChainState()

    data class Error(
        val throwable: Throwable,
    ) : SendOnChainState()

    data class Success(
        val swapInfo: ReverseSwapInfo,
    ) : SendOnChainState()

}
