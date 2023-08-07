package sample.sdk.breez.page.home.receiveonchain

import breez_sdk.SwapInfo

sealed class ReceiveOnChainState {

    object Initial : ReceiveOnChainState()

    object Loading : ReceiveOnChainState()

    data class Error(
        val throwable: Throwable,
    ) : ReceiveOnChainState()

    data class Success(
        val swapInfo: SwapInfo,
    ) : ReceiveOnChainState()

}
