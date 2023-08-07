package sample.sdk.breez.page.home.inprogressreverseswap

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import breez_sdk.ReverseSwapInfo
import breez_sdk.ReverseSwapStatus
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun InProgressReverseSwap(
    swap: ReverseSwapInfo,
) {
    Row(
        Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween,
    ) {
        Text(
            swap.id,
            Modifier
                .padding(
                    start = 16.dp,
                    end = 4.dp,
                    top = 4.dp,
                    bottom = 4.dp,
                )
                .weight(1f),
            maxLines = 1,
            overflow = TextOverflow.Ellipsis,
        )
        Text(
            swap.status.name,
            Modifier.padding(
                start = 4.dp,
                end = 16.dp,
                top = 4.dp,
                bottom = 4.dp,
            ),
        )
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 100,
    widthDp = 300,
)]
fun InProgressReverseSwapPreview() {
    BreezSdkSampleTheme {
        InProgressReverseSwap(
            ReverseSwapInfo(
                "An id",
                "A claim pub key",
                12345L.toULong(),
                ReverseSwapStatus.INITIAL,
            ),
        )
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 100,
    widthDp = 300,
)]
fun InProgressReverseSwapPreviewForBigId() {
    BreezSdkSampleTheme {
        InProgressReverseSwap(
            ReverseSwapInfo(
                "A big biiig biiig huge huuuge huuuuuuge id",
                "A claim pub key",
                12345L.toULong(),
                ReverseSwapStatus.INITIAL,
            ),
        )
    }
}
