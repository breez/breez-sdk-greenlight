package sample.sdk.breez.page.home.lnurlpay

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import breez_sdk.AesSuccessActionDataDecrypted
import breez_sdk.MessageSuccessActionData
import breez_sdk.SuccessActionProcessed
import breez_sdk.UrlSuccessActionData
import sample.sdk.breez.R
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun LnUrlPay(
    result: SuccessActionProcessed?,
) {
    Column {
        Text(
            when (result) {
                null -> stringResource(
                    R.string.ln_url_pay_no_result,
                )
                is SuccessActionProcessed.Aes -> stringResource(
                    R.string.ln_url_pay_aes,
                    result.data.description,
                    result.data.plaintext,
                )
                is SuccessActionProcessed.Message -> stringResource(
                    R.string.ln_url_pay_message,
                    result.data.message,
                )
                is SuccessActionProcessed.Url -> stringResource(
                    R.string.ln_url_pay_url,
                    result.data.url,
                    result.data.description,
                )
            },
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
fun LnUrlPayPreview() {
    BreezSdkSampleTheme {
        LnUrlPay(
            null,
        )
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 200,
    widthDp = 300,
)]
fun LnUrlPayPreviewForAes() {
    BreezSdkSampleTheme {
        LnUrlPay(
            SuccessActionProcessed.Aes(
                data = AesSuccessActionDataDecrypted(
                    description = "A description",
                    plaintext = "A plain text",
                )
            )
        )
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 200,
    widthDp = 300,
)]
fun LnUrlPayPreviewForMessage() {
    BreezSdkSampleTheme {
        LnUrlPay(
            SuccessActionProcessed.Message(
                data = MessageSuccessActionData(
                    message = "A message",
                )
            )
        )
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 200,
    widthDp = 300,
)]
fun LnUrlPayPreviewForUrl() {
    BreezSdkSampleTheme {
        LnUrlPay(
            SuccessActionProcessed.Url(
                data = UrlSuccessActionData(
                    url = "An URL",
                    description = "A description",
                )
            )
        )
    }
}
