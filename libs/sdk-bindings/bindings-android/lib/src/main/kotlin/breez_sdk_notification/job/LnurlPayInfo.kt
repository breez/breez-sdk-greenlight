package breez_sdk_notification.job

import android.content.Context
import breez_sdk.BlockingBreezServices
import breez_sdk_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_notification.Constants
import breez_sdk_notification.ResourceHelper.Companion.getString
import breez_sdk_notification.SdkForegroundService
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.tinylog.kotlin.Logger

@Serializable
data class LnurlInfoRequest(
    @SerialName("callback_url") val callbackURL: String,
    @SerialName("reply_url") val replyURL: String,
)

// Serialize the response according to to LUD-06 payRequest base specification:
// https://github.com/lnurl/luds/blob/luds/06.md
@Serializable
data class LnurlPayInfoResponse(
    val callback: String,
    val maxSendable: ULong,
    val minSendable: ULong,
    val metadata: String,
    val tag: String,
)

class LnurlPayInfoJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
    private val payload: String,
) : LnurlPayJob {
    companion object {
        private const val TAG = "LnurlPayInfoJob"
    }

    override fun start(breezSDK: BlockingBreezServices) {
        var request: LnurlInfoRequest? = null
        try {
            request = Json.decodeFromString<LnurlInfoRequest>(LnurlInfoRequest.serializer(), payload)
            val nodeState = breezSDK.nodeInfo()
            val plainTextMetadata = getString(
                context,
                Constants.LNURL_PAY_METADATA_PLAIN_TEXT,
                Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT
            )
            val response =
                LnurlPayInfoResponse(
                    request.callbackURL,
                    nodeState.inboundLiquidityMsats,
                    1000UL,
                    "[[\"text/plain\",\"$plainTextMetadata\"]]",
                    "payRequest",
                )
            val success = replyServer(Json.encodeToString(response), request.replyURL)
            notifyChannel(
                context,
                Constants.NOTIFICATION_CHANNEL_LNURL_PAY,
                getString(
                    context,
                    if (success) Constants.LNURL_PAY_INFO_NOTIFICATION_TITLE else Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                    if (success) Constants.DEFAULT_LNURL_PAY_INFO_NOTIFICATION_TITLE else Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
                ),
            )
        } catch (e: Exception) {
            Logger.tag(TAG).warn { "Failed to process lnurl: ${e.message}" }
            if (request != null) {
                fail(e.message, request.replyURL)
            }
            notifyChannel(
                context,
                Constants.NOTIFICATION_CHANNEL_LNURL_PAY,
                getString(
                    context,
                    Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                    Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
                ),
            )
        }

        fgService.shutdown()
    }
}
