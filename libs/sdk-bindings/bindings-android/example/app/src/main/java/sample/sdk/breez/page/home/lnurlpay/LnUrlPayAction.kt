package sample.sdk.breez.page.home.lnurlpay

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import sample.sdk.breez.R
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun LnUrlPayAction(
    onLnUrlPayClick: (String, Long, String) -> Unit,
) {
    var amount by remember { mutableStateOf(TextFieldValue("3000")) }
    var description by remember { mutableStateOf(TextFieldValue("Paying 3000 sats")) }
    var payUrl by remember { mutableStateOf(TextFieldValue("lightning@address.com")) }

    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {

        TextField(
            modifier = Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 16.dp,
                bottom = 4.dp,
            ),
            value = amount,
            onValueChange = {
                amount = it
            },
            label = {
                Text(stringResource(R.string.ln_url_amount))
            },
        )

        TextField(
            modifier = Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 4.dp,
            ),
            value = description,
            onValueChange = {
                description = it
            },
            label = {
                Text(stringResource(R.string.ln_url_description))
            },
        )

        TextField(
            modifier = Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 4.dp,
            ),
            value = payUrl,
            onValueChange = {
                payUrl = it
            },
            label = {
                Text(stringResource(R.string.ln_url_pay_ln_url))
            },
        )

        Button(
            modifier = Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 16.dp,
            ),
            onClick = {
                onLnUrlPayClick(payUrl.text, amount.text.toLong(), description.text)
            },
        ) {
            Text(stringResource(R.string.ln_url_pay))
        }

    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 300,
    widthDp = 300,
)]
fun LnUrlPayActionPreview() {
    BreezSdkSampleTheme {
        LnUrlPayAction { _, _, _ ->
        }
    }
}
