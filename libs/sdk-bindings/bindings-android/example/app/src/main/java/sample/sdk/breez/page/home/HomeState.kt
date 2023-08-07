package sample.sdk.breez.page.home

import breez_sdk.FiatCurrency
import breez_sdk.Rate
import breez_sdk.ReverseSwapInfo

sealed class HomeState {

    object Loading : HomeState()

    data class Error(
        val throwable: Throwable,
    ) : HomeState()

    data class Success(
        val lightningBalance: Long,
        val onchainBalance: Long,
        val inProgressSwapOuts: List<ReverseSwapInfo>,
        val fiatCurrencies: Map<FiatCurrency, Rate>,
    ) : HomeState()

}
