package breez_sdk_notification.job

import android.content.Context
import breez_sdk.BlockingBreezServices
import breez_sdk.OpenChannelFeeRequest
import breez_sdk.ReceivePaymentRequest
import breez_sdk_notification.Constants.DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE
import breez_sdk_notification.Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT
import breez_sdk_notification.Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
import breez_sdk_notification.Constants.LNURL_PAY_INVOICE_NOTIFICATION_TITLE
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
data class LnurlInvoiceRequest(
    @SerialName("amount") val amount: ULong,
    @SerialName("reply_url") val replyURL: String,
)

// Serialize the response according to to LUD-06 payRequest base specification:
// https://github.com/lnurl/luds/blob/luds/06.md
@Serializable
data class LnurlPayInvoiceResponse(
    val pr: String,
    val routes: List<String>,
)

class LnurlPayInvoiceJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
    private val payload: String,
    private val logger: ServiceLogger,
    private val config: ServiceConfig,
) : LnurlPayJob {
    companion object {
        private const val TAG = "LnurlPayInvoiceJob"
    }

    override fun start(breezSDK: BlockingBreezServices) {
        var request: LnurlInvoiceRequest? = null
        try {
            request = Json.decodeFromString<LnurlInvoiceRequest>(LnurlInvoiceRequest.serializer(), payload)
            // Get channel setup fee for invoice amount
            val ofpResp =
                breezSDK.openChannelFee(OpenChannelFeeRequest(amountMsat = request.amount))
            // Check if channel setup fee is within fee limit
            val feeLimitMsat: ULong = config.channelFeeLimitMsat
            val isFeeWithinLimit = ofpResp.feeMsat?.let { it == 0UL || it <= feeLimitMsat }
            // Get minimum amount LN service is willing to receive
            val minMsat =
                ofpResp.feeMsat?.let { if (it == 0UL) 1000UL else ofpResp.feeParams.minMsat }!!
            // Check if the invoice amount is larger than minimum accepted amount and if its fee is within fee limit
            if (request.amount < minMsat || isFeeWithinLimit != true) {
                fail("Invalid amount requested ${request.amount}", request.replyURL)
                notifyChannel(
                    context,
                    NOTIFICATION_CHANNEL_LNURL_PAY,
                    getString(
                        context,
                        LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                        DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
                    ),
                )
                return
            }
            val plainTextMetadata = getString(
                context,
                LNURL_PAY_METADATA_PLAIN_TEXT,
                DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT
            )
            val receivePaymentResponse = breezSDK.receivePayment(
                ReceivePaymentRequest(
                    request.amount,
                    description = "[[\"text/plain\",\"$plainTextMetadata\"]]",
                    useDescriptionHash = true
                )
            )
            val response =
                LnurlPayInvoiceResponse(
                    receivePaymentResponse.lnInvoice.bolt11,
                    listOf(),
                )
            val success = replyServer(Json.encodeToString(response), request.replyURL)
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_LNURL_PAY,
                getString(
                    context,
                    if (success) LNURL_PAY_INVOICE_NOTIFICATION_TITLE else LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                    if (success) DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE else DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
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

        fgService.onFinished(this)
    }

    override fun onShutdown() {}
}
