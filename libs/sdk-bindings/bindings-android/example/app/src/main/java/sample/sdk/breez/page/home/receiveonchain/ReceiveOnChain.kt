package sample.sdk.breez.page.home.receiveonchain

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import breez_sdk.SwapInfo
import breez_sdk.SwapStatus
import sample.sdk.breez.R
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun ReceiveOnChain(
    swapInfo: SwapInfo,
) {
    Column {
        Text(
            swapInfo.bitcoinAddress,
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
            stringResource(
                R.string.receive_on_chain_range,
                swapInfo.minAllowedDeposit,
                swapInfo.maxAllowedDeposit,
            ),
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
fun ReceiveOnChainPreview() {
    BreezSdkSampleTheme {
        ReceiveOnChain(
            SwapInfo(
                "A bitcoin address",
                1689788251L,
                123L,
                emptyList(),
                emptyList(),
                emptyList(),
                emptyList(),
                emptyList(),
                emptyList(),
                "A Bolt11",
                1L.toULong(),
                2L.toULong(),
                3L.toULong(),
                SwapStatus.INITIAL,
                emptyList(),
                emptyList(),
                emptyList(),
                100L,
                200L,
                "A last redeem error",
            ),
        )
    }
}
