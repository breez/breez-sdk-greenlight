package sample.sdk.breez.page.home.section

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Divider
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import sample.sdk.breez.page.BreezSdkSampleTheme

@Composable
fun Section(
    title: String,
    content: @Composable ColumnScope.() -> Unit,
) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Divider(
            color = Color.Gray,
            thickness = 1.dp,
        )
        Text(
            title,
            Modifier.padding(
                start = 16.dp,
                end = 16.dp,
                top = 8.dp,
                bottom = 4.dp,
            ),
        )
        content()
    }
}

@[Composable Preview(
    showBackground = true,
    heightDp = 200,
    widthDp = 300,
)]
fun poSectionPreview() {
    BreezSdkSampleTheme {
        Section(
            title = "A title",
            content = {
                Text("An example")
            }
        )
    }
}
