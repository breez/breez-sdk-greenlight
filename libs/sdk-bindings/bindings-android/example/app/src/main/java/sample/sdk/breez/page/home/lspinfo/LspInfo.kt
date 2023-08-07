package sample.sdk.breez.page.home.lspinfo

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import breez_sdk.LspInformation
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun LspInfo(
    lspInfo: LspInformation,
) {
    Column {
        Text(
            lspInfo.name,
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 16.dp,
                bottom = 4.dp,
            ),
        )
        Text(
            lspInfo.id,
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 4.dp,
            ),
        )
        Text(
            lspInfo.host,
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 4.dp,
            ),
        )
        Text(
            lspInfo.widgetUrl,
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
fun LspInfoPreview() {
    BreezSdkSampleTheme {
        LspInfo(
            LspInformation(
                "An id",
                "A name",
                "A widget URL",
                "A pubkey",
                "A host",
                123L,
                1,
                45L,
                1.2,
                1.toUInt(),
                67L,
                89L,
                emptyList(),
                4321L,
                65L,
            )
        )
    }
}
