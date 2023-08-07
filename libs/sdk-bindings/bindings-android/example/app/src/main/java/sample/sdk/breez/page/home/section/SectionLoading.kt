package sample.sdk.breez.page.home.section

import androidx.compose.foundation.layout.padding
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun SectionLoading() {
    CircularProgressIndicator(
        modifier = Modifier.padding(
            start = 16.dp,
            end = 16.dp,
            top = 16.dp,
            bottom = 16.dp,
        ),
    )
}

@[Composable Preview(
    showBackground = true,
    heightDp = 100,
    widthDp = 100,
)]
fun SectionLoadingPreview() {
    BreezSdkSampleTheme {
        SectionLoading()
    }
}
