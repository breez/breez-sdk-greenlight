package breez_sdk_notification.job

import android.content.Context
import breez_sdk.BlockingBreezServices
import breez_sdk.BreezEvent
import breez_sdk_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_notification.Constants
import breez_sdk_notification.ResourceHelper.Companion.getString
import breez_sdk_notification.SdkForegroundService
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import org.tinylog.kotlin.Logger

@Serializable
data class AddressTxsConfirmedRequest(
    val address: String,
)

class RedeemSwapJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
    private val payload: String,
) : Job {
    companion object {
        private const val TAG = "RedeemSwapJob"
    }

    override fun start(breezSDK: BlockingBreezServices) {
        var request: AddressTxsConfirmedRequest? = null
        try {
            val request = Json.decodeFromString<AddressTxsConfirmedRequest>(AddressTxsConfirmedRequest.serializer(), payload)
            breezSDK.redeemSwap(request!!.address)
            Logger.tag(TAG).info { "Found swap for ${request.address}" }
            notifyChannel(
                context,
                Constants.NOTIFICATION_CHANNEL_SWAP_TX_CONFIRMED,
                getString(
                    context,
                    Constants.SWAP_TX_CONFIRMED_NOTIFICATION_TITLE,
                    Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_TITLE
                ),
            )
        } catch (e: Exception) {
            Logger.tag(TAG).warn { "Failed to process swap notification: ${e.message}" }
            notifyChannel(
                context,
                Constants.NOTIFICATION_CHANNEL_SWAP_TX_CONFIRMED,
                getString(
                    context,
                    Constants.SWAP_TX_CONFIRMED_NOTIFICATION_FAILURE_TITLE,
                    Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_FAILURE_TITLE
                ),
            )
        }

        fgService.shutdown()
    }

    override fun onEvent(e: BreezEvent) {}
}
