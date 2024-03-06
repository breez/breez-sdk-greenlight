package breez_sdk_notification.job

import android.content.Context
import breez_sdk.BlockingBreezServices
import breez_sdk.BreezEvent
import breez_sdk.LogEntry
import breez_sdk.Payment
import breez_sdk_notification.Constants.DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TEXT
import breez_sdk_notification.Constants.DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TITLE
import breez_sdk_notification.Constants.NOTIFICATION_CHANNEL_PAYMENT_RECEIVED
import breez_sdk_notification.Constants.PAYMENT_RECEIVED_NOTIFICATION_TEXT
import breez_sdk_notification.Constants.PAYMENT_RECEIVED_NOTIFICATION_TITLE
import breez_sdk_notification.ForegroundService.Companion.logger
import breez_sdk_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_notification.ResourceHelper.Companion.getString
import breez_sdk_notification.SdkForegroundService

class ReceivePaymentJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
) : Job {
    private var receivedPayment: Payment? = null

    companion object {
        private const val TAG = "ReceivePaymentJob"
    }

    override fun start(breezSDK: BlockingBreezServices) {}

    override fun onEvent(e: BreezEvent) {
        logger?.log(LogEntry(TAG, "Received event $e", "TRACE"))
        when (e) {
            is BreezEvent.InvoicePaid -> {
                val pd = e.details
                handleReceivedPayment(pd.bolt11, pd.paymentHash, pd.payment?.amountMsat)
                receivedPayment = pd.payment

                // Push back shutdown by SHUTDOWN_DELAY_MS for payments synced event
                fgService.pushbackShutdown()
            }

            is BreezEvent.Synced -> {
                receivedPayment?.let {
                    logger?.log(LogEntry(TAG, "Got synced event for received payment", "INFO"))
                    fgService.shutdown()
                }
            }

            else -> {}
        }
    }

    private fun handleReceivedPayment(
        bolt11: String,
        paymentHash: String,
        amountMsat: ULong?,
    ) {
        logger?.log(
            LogEntry(
                TAG,
                "Received payment. Bolt11:${bolt11}\nPayment Hash:${paymentHash}",
                "INFO"
            )
        )
        val amountSat = (amountMsat ?: ULong.MIN_VALUE) / 1000u
        notifyChannel(
            context,
            NOTIFICATION_CHANNEL_PAYMENT_RECEIVED,
            getString(
                context,
                PAYMENT_RECEIVED_NOTIFICATION_TITLE,
                DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TITLE
            ),
            String.format(
                getString(
                    context,
                    PAYMENT_RECEIVED_NOTIFICATION_TEXT,
                    "%d",
                    DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TEXT
                ), amountSat.toLong()
            )
        )
    }
}
