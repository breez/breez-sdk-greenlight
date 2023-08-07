package sample.sdk.breez.page.home.section

import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import sample.sdk.breez.R
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun SectionError(
    throwable: Throwable,
) {
    Text(
        modifier = Modifier.padding(
            start = 16.dp,
            end = 16.dp,
            top = 16.dp,
            bottom = 16.dp,
        ),
        text = throwable.message ?: stringResource(R.string.error_unknown),
    )
}

@[Composable Preview(
    showBackground = true,
    heightDp = 100,
    widthDp = 100,
)]
fun SectionErrorPreview() {
    BreezSdkSampleTheme {
        SectionError(Exception("An exception"))
    }
}
