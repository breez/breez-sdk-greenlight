package breez_sdk_notification

import android.app.ActivityManager
import android.app.ActivityManager.RunningAppProcessInfo.IMPORTANCE_FOREGROUND
import android.app.ActivityManager.RunningAppProcessInfo.IMPORTANCE_VISIBLE
import android.content.Context
import org.tinylog.kotlin.Logger

interface MessagingService {
    companion object {
        private const val TAG = "MessagingService"
    }

    /** To be implemented by the application messaging service.
     *  The implemented function should start the foreground with
     *  the provided Message in an Intent. */
    fun startForegroundService(message: Message)

    /** Check if the message has a data payload with high priority
     *  as we cannot start foreground service from low/normal priority message. */
    fun startServiceIfNeeded(context: Context, message: Message) {
        val isServiceNeeded = when (message.type) {
            Constants.MESSAGE_TYPE_PAYMENT_RECEIVED -> !isAppForeground(context)
            else -> true
        }
        if (isServiceNeeded && message.priority == Message.PRIORITY_HIGH) startForegroundService(message)
        else Logger.tag(TAG).warn { "Ignoring message ${message.type}: ${message.payload}" }
    }

    /** Basic implementation to check if the application is in the foreground. */
    fun isAppForeground(context: Context): Boolean {
        val appProcessInfo = ActivityManager.RunningAppProcessInfo()
        ActivityManager.getMyMemoryState(appProcessInfo)

        return (appProcessInfo.importance == IMPORTANCE_FOREGROUND || appProcessInfo.importance == IMPORTANCE_VISIBLE)
    }
}
