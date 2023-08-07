package sample.sdk.breez.page.home.lnurlauth

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
import sample.sdk.breez.page.BreezSdkSampleTheme
import sample.sdk.breez.R

@Composable
fun LnUrlAuthAction(
    onLnUrlAuthClick: (String) -> Unit,
) {
    var authUrl by remember { mutableStateOf(TextFieldValue("lnur…")) }

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
            value = authUrl,
            onValueChange = {
                authUrl = it
            },
            label = {
                Text(stringResource(R.string.ln_url_auth_ln_url))
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
                onLnUrlAuthClick(authUrl.text)
            },
        ) {
            Text(stringResource(R.string.ln_url_auth))
        }

    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 300,
    widthDp = 300,
)]
fun LnUrlAuthActionPreview() {
    BreezSdkSampleTheme {
        LnUrlAuthAction {
        }
    }
}
