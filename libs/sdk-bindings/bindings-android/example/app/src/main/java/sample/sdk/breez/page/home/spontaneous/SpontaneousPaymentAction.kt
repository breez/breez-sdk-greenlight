package sample.sdk.breez.page.home.spontaneous

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
fun SpontaneousPaymentAction(
    onPaymentClick: (String, Long) -> Unit,
) {
    var nodeId by remember { mutableStateOf(TextFieldValue("...")) }
    var amount by remember { mutableStateOf(TextFieldValue("3000")) }

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
                Text(stringResource(R.string.send_spontaneous_payment_amount))
            },
        )

        TextField(
            modifier = Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 4.dp,
            ),
            value = nodeId,
            onValueChange = {
                nodeId = it
            },
            label = {
                Text(stringResource(R.string.send_spontaneous_payment_node_id))
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
                onPaymentClick(nodeId.text, amount.text.toLong())
            },
        ) {
            Text(stringResource(R.string.send_spontaneous_payment))
        }

    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 200,
    widthDp = 300,
)]
fun SpontaneousPaymentActionPreview() {
    BreezSdkSampleTheme {
        SpontaneousPaymentAction { _, _ ->
        }
    }
}
