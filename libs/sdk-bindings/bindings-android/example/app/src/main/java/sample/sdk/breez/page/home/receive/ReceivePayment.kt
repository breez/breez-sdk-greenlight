package sample.sdk.breez.page.home.receive

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import breez_sdk.LnInvoice
import sample.sdk.breez.R
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun ReceivePayment(
    invoice: LnInvoice,
    openingChannelFee: Long?,
) {
    Column {
        Text(
            invoice.bolt11,
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 16.dp,
                bottom = 4.dp,
            ),
        )
        Text(
            invoice.paymentHash,
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 4.dp,
            ),
        )
        Text(
            invoice.description.orEmpty(),
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = if (openingChannelFee == null) 16.dp else 4.dp,
            ),
        )
        if (openingChannelFee != null) {
            Text(
                stringResource(
                    R.string.receiving_lightning_payment_open_channel,
                    openingChannelFee,
                ),
                Modifier.padding(
                    start = 16.dp,
                    end = 16.dp,
                    top = 4.dp,
                    bottom = 16.dp,
                ),
            )
        }
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 200,
    widthDp = 300,
)]
fun ReceivePaymentPreview() {
    BreezSdkSampleTheme {
        ReceivePayment(
            LnInvoice(
                "A Bolt11",
                "A payee pubkey",
                "A payment hash",
                "A Description",
                "A description hash",
                123L.toULong(),
                1689788251L.toULong(),
                1689874651L.toULong(),
                emptyList(),
                emptyList(),
            ),
            null,
        )
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 200,
    widthDp = 300,
)]
fun ReceivePaymentPreviewWithOpeningChannel() {
    BreezSdkSampleTheme {
        ReceivePayment(
            LnInvoice(
                "A Bolt11",
                "A payee pubkey",
                "A payment hash",
                "A Description",
                "A description hash",
                234L.toULong(),
                1689788251L.toULong(),
                1689874651L.toULong(),
                emptyList(),
                emptyList(),
            ),
            123L,
        )
    }
}
