package sample.sdk.breez.page.home.lspinfo

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import sample.sdk.breez.R
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun LspInfoAction(
    onLspInfoClick: () -> Unit,
) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {

        Button(
            modifier = Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 16.dp,
                bottom = 16.dp,
            ),
            onClick = {
                onLspInfoClick()
            },
        ) {
            Text(stringResource(R.string.lsp_info))
        }

    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 200,
    widthDp = 300,
)]
fun LspInfoActionPreview() {
    BreezSdkSampleTheme {
        LspInfoAction {
        }
    }
}
