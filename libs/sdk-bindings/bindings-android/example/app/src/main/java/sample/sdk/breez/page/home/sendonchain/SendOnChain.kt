package sample.sdk.breez.page.home.sendonchain

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import breez_sdk.ReverseSwapInfo
import breez_sdk.ReverseSwapStatus
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun SendOnChain(
    swapInfo: ReverseSwapInfo,
) {
    Column {
        Text(
            swapInfo.id,
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 16.dp,
                bottom = 4.dp,
            ),
        )
        Text(
            swapInfo.status.name,
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 4.dp,
            ),
        )
        Text(
            swapInfo.onchainAmountSat.toString(),
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 4.dp,
            ),
        )
        Text(
            swapInfo.claimPubkey,
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
fun SendOnChainPreview() {
    BreezSdkSampleTheme {
        SendOnChain(
            ReverseSwapInfo(
                "An id",
                "A public key",
                123L.toULong(),
                ReverseSwapStatus.INITIAL,
            ),
        )
    }
}
