package sample.sdk.breez.page.home.fiatcurrency

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import breez_sdk.CurrencyInfo
import breez_sdk.FiatCurrency
import breez_sdk.Rate
import sample.sdk.breez.page.BreezSdkSampleTheme
import sample.sdk.breez.R

@Composable
fun FiatCurrency(
    fiatCurrency: FiatCurrency,
    rate: Rate,
) {
    Row(
        Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween,
    ) {
        Text(
            fiatCurrency.info.name,
            Modifier
                .padding(
                    start = 16.dp,
                    end = 4.dp,
                    top = 4.dp,
                    bottom = 4.dp,
                )
                .weight(1f),
            maxLines = 1,
            overflow = TextOverflow.Ellipsis,
        )
        Text(
            stringResource(
                R.string.fiat_currency_value,
                rate.value,
            ),
            Modifier.padding(
                start = 4.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 4.dp,
            ),
        )
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 100,
    widthDp = 300,
)]
fun FiatCurrencyPreview() {
    BreezSdkSampleTheme {
        FiatCurrency(
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
            ),
            Rate(
                "USD",
                30_000.00,
            ),
        )
    }
}
