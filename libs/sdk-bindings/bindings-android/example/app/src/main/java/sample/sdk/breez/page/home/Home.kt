package sample.sdk.breez.page.home

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import breez_sdk.CurrencyInfo
import breez_sdk.FiatCurrency
import breez_sdk.Rate
import breez_sdk.ReverseSwapInfo
import breez_sdk.ReverseSwapStatus
import sample.sdk.breez.page.BreezSdkSampleTheme
import sample.sdk.breez.R
import sample.sdk.breez.page.home.fiatcurrency.FiatCurrency
import sample.sdk.breez.page.home.inprogressreverseswap.InProgressReverseSwap
import sample.sdk.breez.page.home.section.Section

@Composable
fun Home(
    onchainBalance: Long,
    lightningBalance: Long,
    inProgressSwapOuts: List<ReverseSwapInfo>,
    fiatCurrencies: Map<FiatCurrency, Rate>,
    receivePaymentComposable: @Composable ColumnScope.() -> Unit,
    sendPaymentComposable: @Composable ColumnScope.() -> Unit,
    spontaneousComposable: @Composable ColumnScope.() -> Unit,
    receiveOnChainComposable: @Composable ColumnScope.() -> Unit,
    sendOnChainComposable: @Composable ColumnScope.() -> Unit,
    lspInfoComposable: @Composable ColumnScope.() -> Unit,
    lnUrlPayComposable: @Composable ColumnScope.() -> Unit,
    lnUrlWithdrawComposable: @Composable ColumnScope.() -> Unit,
    lnUrlAuthComposable: @Composable ColumnScope.() -> Unit,
) {
    Column(
        Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState()),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Top,
    ) {
        Text(
            stringResource(
                R.string.onchain_balance,
                onchainBalance,
            ),
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 16.dp,
                bottom = 4.dp,
            ),
        )

        Text(
            stringResource(
                R.string.lightning_balance,
                lightningBalance,
            ),
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 16.dp,
            ),
        )

        Section(
            stringResource(R.string.section_receiving_lightning_payments),
            receivePaymentComposable,
        )

        Section(
            stringResource(R.string.section_sending_lightning_payments),
            sendPaymentComposable,
        )

        Section(
            stringResource(R.string.section_sending_spontaneous_payments),
            spontaneousComposable,
        )

        Section(
            stringResource(R.string.section_receive_on_chain),
            receiveOnChainComposable,
        )

        Section(
            stringResource(R.string.section_send_on_chain),
            sendOnChainComposable,
        )

        Section(
            stringResource(R.string.section_send_on_chain_in_progress),
        ) {
            Spacer(Modifier.padding(top = 12.dp))
            for (swapInfo in inProgressSwapOuts) {
                InProgressReverseSwap(swapInfo)
            }
            Spacer(Modifier.padding(bottom = 12.dp))
        }

        Section(
            stringResource(R.string.section_lsp_info),
            lspInfoComposable,
        )

        Section(
            stringResource(R.string.section_ln_url_pay),
            lnUrlPayComposable,
        )

        Section(
            stringResource(R.string.section_ln_url_withdraw),
            lnUrlWithdrawComposable,
        )

        Section(
            stringResource(R.string.section_ln_url_auth),
            lnUrlAuthComposable,
        )

        Section(
            stringResource(R.string.section_fiat_currencies),
        ) {
            Spacer(Modifier.padding(top = 12.dp))
            for ((currency, rate) in fiatCurrencies) {
                FiatCurrency(currency, rate)
            }
            Spacer(Modifier.padding(bottom = 12.dp))
        }
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 700,
    widthDp = 300,
)]
fun HomePreview() {
    BreezSdkSampleTheme {
        Home(
            lightningBalance = 12_345L,
            onchainBalance = 34_567L,
            inProgressSwapOuts = listOf(
                ReverseSwapInfo(
                    "An id",
                    "A claim pub key",
                    12345L.toULong(),
                    ReverseSwapStatus.INITIAL,
                ),
            ),
            fiatCurrencies = mapOf(
                FiatCurrency(
                    "USD",
                    CurrencyInfo(
                        "United States Dollar",
                        2.toUInt(),
                        null,
                        null,
                        null,
                        null,
                        null,
                    )
                ) to Rate(
                    "USD",
                    30_000.00,
                ),
            ),
            receivePaymentComposable = {},
            sendPaymentComposable = {},
            spontaneousComposable = {},
            receiveOnChainComposable = {},
            sendOnChainComposable = {},
            lspInfoComposable = {},
            lnUrlPayComposable = {},
            lnUrlWithdrawComposable = {},
            lnUrlAuthComposable = {},
        )
    }
}
