package breez_sdk_notification

import android.app.ActivityManager
import android.app.ActivityManager.RunningAppProcessInfo.IMPORTANCE_FOREGROUND
import android.app.ActivityManager.RunningAppProcessInfo.IMPORTANCE_VISIBLE
import android.content.Context
import breez_sdk_notification.Constants.MESSAGE_TYPE_ADDRESS_TXS_CONFIRMED
import breez_sdk_notification.Constants.MESSAGE_TYPE_PAYMENT_RECEIVED
import breez_sdk_notification.LogHelper.nodeLogStream

@Suppress("unused")
interface MessagingService {
    companion object {
        private const val TAG = "MessagingService"
    }

    /** To be implemented by the application messaging service.
     *  The implemented function should start the foreground with
     *  the provided Message in an Intent. */
    fun startForegroundService(message: Message)

    /** Check if the foreground service is needed depending on the
     *  message type and foreground state of the application. */
    fun startServiceIfNeeded(context: Context, message: Message) {
        val isServiceNeeded = when (message.type) {
            MESSAGE_TYPE_ADDRESS_TXS_CONFIRMED -> !isAppForeground(context)
            MESSAGE_TYPE_PAYMENT_RECEIVED -> !isAppForeground(context)
            else -> true
        }
        if (isServiceNeeded) startForegroundService(message)
        else nodeLogStream?.log(TAG, "Ignoring message ${message.type}: ${message.payload}", "WARN")
    }

    /** Basic implementation to check if the application is in the foreground */
    fun isAppForeground(context: Context): Boolean {
        val appProcessInfo = ActivityManager.RunningAppProcessInfo()
        ActivityManager.getMyMemoryState(appProcessInfo)

        return (appProcessInfo.importance == IMPORTANCE_FOREGROUND || appProcessInfo.importance == IMPORTANCE_VISIBLE)
    }
}
