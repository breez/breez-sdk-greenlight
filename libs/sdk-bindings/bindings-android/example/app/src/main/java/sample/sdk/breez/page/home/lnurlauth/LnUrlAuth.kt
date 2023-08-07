package sample.sdk.breez.page.home.lnurlauth

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import sample.sdk.breez.page.BreezSdkSampleTheme
import sample.sdk.breez.R

@Composable
fun LnUrlAuth() {
    Column {
        Text(
            stringResource(
                R.string.ln_url_auth_success,
            ),
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 16.dp,
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
fun LnUrlAuthPreview() {
    BreezSdkSampleTheme {
        LnUrlAuth()
    }
}
