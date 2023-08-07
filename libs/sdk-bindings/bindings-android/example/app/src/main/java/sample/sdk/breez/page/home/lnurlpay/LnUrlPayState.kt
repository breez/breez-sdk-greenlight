package sample.sdk.breez.page.home.lnurlpay

import breez_sdk.SuccessActionProcessed

sealed class LnUrlPayState {

    object Initial : LnUrlPayState()

    object Loading : LnUrlPayState()

    data class Error(
        val throwable: Throwable,
    ) : LnUrlPayState()

    data class Success(
        val result: SuccessActionProcessed?,
    ) : LnUrlPayState()

}
