package sample.sdk.breez.page.home.lnurlwithdraw

sealed class LnUrlWithdrawState {

    object Initial : LnUrlWithdrawState()

    object Loading : LnUrlWithdrawState()

    data class Error(
        val throwable: Throwable,
    ) : LnUrlWithdrawState()

    object Success : LnUrlWithdrawState()

}
