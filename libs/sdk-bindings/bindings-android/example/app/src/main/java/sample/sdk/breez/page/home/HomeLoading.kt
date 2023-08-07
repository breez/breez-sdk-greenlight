package sample.sdk.breez.page.home

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import sample.sdk.breez.R
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun HomeLoading() {
    Column(
        Modifier.fillMaxSize(),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center,
    ) {
        Text(stringResource(R.string.loading))
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 100,
    widthDp = 100,
)]
fun HomeLoadingPreview() {
    BreezSdkSampleTheme {
        HomeLoading()
    }
}
