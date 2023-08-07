package sample.sdk.breez.page.home.lnurlauth

sealed class LnUrlAuthState {

    object Initial : LnUrlAuthState()

    object Loading : LnUrlAuthState()

    data class Error(
        val throwable: Throwable,
    ) : LnUrlAuthState()

    object Success : LnUrlAuthState()

}
