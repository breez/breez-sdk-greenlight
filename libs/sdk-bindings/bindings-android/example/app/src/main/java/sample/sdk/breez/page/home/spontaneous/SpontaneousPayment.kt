package sample.sdk.breez.page.home.spontaneous

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import breez_sdk.LnPaymentDetails
import breez_sdk.Payment
import breez_sdk.PaymentDetails
import breez_sdk.PaymentType
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun SpontaneousPayment(
    payment: Payment,
) {
    Column {
        Text(
            payment.id,
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 16.dp,
                bottom = 4.dp,
            ),
        )
        Text(
            payment.description.orEmpty(),
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 16.dp,
            ),
        )
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 200,
    widthDp = 300,
)]
fun SpontaneousPaymentPreview() {
    BreezSdkSampleTheme {
        SpontaneousPayment(
            Payment(
                "An id",
                PaymentType.SENT,
                1689788251L,
                3000L.toULong(),
                123L.toULong(),
                true,
                "A description",
                PaymentDetails.Ln(
                    data = LnPaymentDetails(
                        "A payment hash",
                        "A label",
                        "A destination pubkey",
                        "A payment pre image",
                        true,
                        "A bolt 11",
                        null,
                        null,
                        null,
                    ),
                ),
            ),
        )
    }
}
