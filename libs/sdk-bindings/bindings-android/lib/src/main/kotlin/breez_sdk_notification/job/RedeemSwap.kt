package breez_sdk_notification.job

import android.content.Context
import breez_sdk.BlockingBreezServices
import breez_sdk.BreezEvent
import breez_sdk_notification.Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_FAILURE_TITLE
import breez_sdk_notification.Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_TITLE
import breez_sdk_notification.Constants.NOTIFICATION_CHANNEL_SWAP_TX_CONFIRMED
import breez_sdk_notification.Constants.SWAP_TX_CONFIRMED_NOTIFICATION_FAILURE_TITLE
import breez_sdk_notification.Constants.SWAP_TX_CONFIRMED_NOTIFICATION_TITLE
import breez_sdk_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_notification.ResourceHelper.Companion.getString
import breez_sdk_notification.SdkForegroundService
import breez_sdk_notification.ServiceLogger
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class AddressTxsConfirmedRequest(
    val address: String,
)

class RedeemSwapJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
    private val payload: String,
    private val logger: ServiceLogger,
    private var bitcoinAddress: String? = null,
) : Job {
    companion object {
        private const val TAG = "RedeemSwapJob"
    }

    override fun start(breezSDK: BlockingBreezServices) {
        try {
            val request = Json.decodeFromString(AddressTxsConfirmedRequest.serializer(), payload)
            this.bitcoinAddress = request.address
            breezSDK.redeemSwap(request.address)
            logger.log(TAG, "Found swap for ${request.address}", "INFO")
        } catch (e: Exception) {
            logger.log(TAG, "Failed to manually reedeem swap notification: ${e.message}", "WARN")
        }
    }

    override fun onEvent(e: BreezEvent) {
        this.bitcoinAddress?.let {address ->
            when (e) {
                is BreezEvent.SwapUpdated -> {
                    val swapInfo = e.details
                    logger.log(TAG, "Received swap updated event: ${swapInfo.bitcoinAddress} current address: ${address} status: ${swapInfo.status}", "TRACE")
                    if (swapInfo.bitcoinAddress == address) {
                        if (swapInfo.paidMsat.toLong() > 0) {
                            notifySuccessAndShutdown(address)
                        }
                    }
                }

                else -> {}
            }
        }
    }

    private  fun notifySuccessAndShutdown(address: String) {
        logger.log(TAG, "Swap address ${address} redeemed succesfully", "INFO")
        notifyChannel(
            context,
            NOTIFICATION_CHANNEL_SWAP_TX_CONFIRMED,
            getString(
                context,
                SWAP_TX_CONFIRMED_NOTIFICATION_TITLE,
                DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_TITLE
            ),
        )
        fgService.shutdown()
    }
}
