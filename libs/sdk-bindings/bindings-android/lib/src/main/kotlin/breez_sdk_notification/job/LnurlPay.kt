package breez_sdk_notification.job

import breez_sdk.BreezEvent
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import java.io.DataOutputStream
import java.net.HttpURLConnection
import java.net.URL

@Serializable
data class LnurlErrorResponse(
    @SerialName("status") val status: String,
    @SerialName("reason") val reason: String?,
)

interface LnurlPayJob : Job {
    override fun onEvent(e: BreezEvent) {}

    fun replyServer(
        payload: String,
        replyURL: String,
    ): Boolean {
        val url = URL(replyURL)
        val response = payload.toByteArray()

        with(url.openConnection() as HttpURLConnection) {
            requestMethod = "POST"
            doOutput = true
            useCaches = false
            setRequestProperty("Content-Type", "application/json")
            setRequestProperty("Content-Length", response.size.toString())
            DataOutputStream(outputStream).use { it.write(response, 0, response.size) }

            return responseCode == 200
        }
    }

    fun fail(
        withError: String?,
        replyURL: String,
    ) {
        val url = URL(replyURL)
        val response = Json.encodeToString(LnurlErrorResponse("ERROR", withError)).toByteArray()

        with(url.openConnection() as HttpURLConnection) {
            requestMethod = "POST"
            doOutput = true
            useCaches = false
            setRequestProperty("Content-Type", "application/json")
            setRequestProperty("Content-Length", response.size.toString())
            DataOutputStream(outputStream).use { it.write(response, 0, response.size) }
        }
    }
}
