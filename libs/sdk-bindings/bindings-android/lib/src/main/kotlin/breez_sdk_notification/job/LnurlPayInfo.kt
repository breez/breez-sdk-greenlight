package breez_sdk_notification.job

import android.content.Context
import breez_sdk.BlockingBreezServices
import breez_sdk.OpenChannelFeeRequest
import breez_sdk_notification.Constants.DEFAULT_LNURL_PAY_INFO_NOTIFICATION_TITLE
import breez_sdk_notification.Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT
import breez_sdk_notification.Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
import breez_sdk_notification.Constants.LNURL_PAY_INFO_NOTIFICATION_TITLE
import breez_sdk_notification.Constants.LNURL_PAY_METADATA_PLAIN_TEXT
import breez_sdk_notification.Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE
import breez_sdk_notification.Constants.NOTIFICATION_CHANNEL_LNURL_PAY
import breez_sdk_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_notification.ResourceHelper.Companion.getString
import breez_sdk_notification.SdkForegroundService
import breez_sdk_notification.ServiceConfig
import breez_sdk_notification.ServiceLogger
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

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
    private val logger: ServiceLogger,
    private val config: ServiceConfig,
) : LnurlPayJob {
    companion object {
        private const val TAG = "LnurlPayInfoJob"
    }

    override fun start(breezSDK: BlockingBreezServices) {
        var request: LnurlInfoRequest? = null
        try {
            request = Json.decodeFromString(LnurlInfoRequest.serializer(), payload)
            // Get the fee parameters offered by the LSP for opening a new channel
            val ofp =
                breezSDK.openChannelFee(OpenChannelFeeRequest(amountMsat = null)).feeParams
            // Calculate the maximum receivable amount within the fee limit (in millisatoshis)
            val feeLimitMsat: ULong = config.channelFeeLimitMsat
            val nodeInfo = breezSDK.nodeInfo()
            val proportionalPercent = ((ofp.proportional.toDouble() / 1000000.0))
            // Treat fee limit feature as disabled when it's set to 0
            val maxReceivableMsatFeeLimit = if (proportionalPercent != 0.0 && feeLimitMsat != 0UL) {
                minOf(
                    nodeInfo.maxReceivableMsat,
                    (feeLimitMsat.toDouble() / proportionalPercent).toULong()
                )
            } else {
                nodeInfo.maxReceivableMsat
            }
            // Calculate the maximum sendable amount (in millisatoshis)
            val maxSendable = maxOf(
                nodeInfo.inboundLiquidityMsats,
                maxReceivableMsatFeeLimit,
            )
            // Get the minimum sendable amount (in millisatoshis), can not be less than 1 or more than maxSendable
            val minSendable: ULong =
                if (nodeInfo.inboundLiquidityMsats < 1000UL) ofp.minMsat else 1000UL
            if (minSendable > maxSendable) {
                throw Exception("Minimum sendable amount can't be greater than maximum sendable amount.")
            }
            val plainTextMetadata = getString(
                context,
                LNURL_PAY_METADATA_PLAIN_TEXT,
                DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT
            )
            val response =
                LnurlPayInfoResponse(
                    request.callbackURL,
                    maxSendable,
                    minSendable,
                    "[[\"text/plain\",\"$plainTextMetadata\"]]",
                    "payRequest",
                )
            val success = replyServer(Json.encodeToString(response), request.replyURL)
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_LNURL_PAY,
                getString(
                    context,
                    if (success) LNURL_PAY_INFO_NOTIFICATION_TITLE else LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                    if (success) DEFAULT_LNURL_PAY_INFO_NOTIFICATION_TITLE else DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
                ),
            )
        } catch (e: Exception) {
            logger.log(TAG, "Failed to process lnurl: ${e.message}", "WARN")
            if (request != null) {
                fail(e.message, request.replyURL)
            }
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_LNURL_PAY,
                getString(
                    context,
                    LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                    DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
                ),
            )
        }

        fgService.shutdown()
    }
}
