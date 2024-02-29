package breez_sdk_notification

import android.Manifest
import android.annotation.SuppressLint
import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationChannelGroup
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.TaskStackBuilder
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import androidx.annotation.RequiresApi
import androidx.core.app.ActivityCompat
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import breez_sdk_notification.ResourceHelper.Companion.getColor
import breez_sdk_notification.ResourceHelper.Companion.getDrawable
import breez_sdk_notification.ResourceHelper.Companion.getString
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import org.tinylog.kotlin.Logger

@Suppress("unused")
class NotificationHelper {
    companion object {
        private const val TAG = "NotificationHelper"
        private var defaultClickAction: String? = null

        fun registerNotificationChannels(context: Context, defaultClickAction: String? = null) {
            this.defaultClickAction = defaultClickAction

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                val notificationManager =
                    context.getSystemService(Context.NOTIFICATION_SERVICE)
                            as NotificationManager
                if (notificationManager.areNotificationsEnabled()) {
                    createNotificationChannelGroup(context, notificationManager)
                    createNotificationChannels(context, notificationManager)
                }
                Logger.tag(TAG).debug { "Registered notification channels " }
            }
        }

        @RequiresApi(Build.VERSION_CODES.O)
        private fun createNotificationChannels(
            context: Context,
            notificationManager: NotificationManager,
        ) {
            val applicationId = context.applicationContext.packageName
            val foregroundServiceNotificationChannel = NotificationChannel(
                "${applicationId}.${Constants.NOTIFICATION_CHANNEL_FOREGROUND_SERVICE}",
                getString(
                    context,
                    Constants.FOREGROUND_SERVICE_NOTIFICATION_CHANNEL_NAME,
                    Constants.DEFAULT_FOREGROUND_SERVICE_NOTIFICATION_CHANNEL_NAME
                ),
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = getString(
                    context,
                    Constants.FOREGROUND_SERVICE_NOTIFICATION_CHANNEL_DESCRIPTION,
                    Constants.DEFAULT_FOREGROUND_SERVICE_NOTIFICATION_CHANNEL_DESCRIPTION
                )
            }
            val receivedPaymentsNotificationChannel = NotificationChannel(
                "${applicationId}.${Constants.NOTIFICATION_CHANNEL_PAYMENT_RECEIVED}",
                getString(
                    context,
                    Constants.PAYMENT_RECEIVED_NOTIFICATION_CHANNEL_NAME,
                    Constants.DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_CHANNEL_NAME
                ),
                NotificationManager.IMPORTANCE_DEFAULT
            ).apply {
                description = getString(
                    context,
                    Constants.PAYMENT_RECEIVED_NOTIFICATION_CHANNEL_DESCRIPTION,
                    Constants.DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_CHANNEL_DESCRIPTION
                )
                group = Constants.OFFLINE_PAYMENTS_WORKGROUP_ID
            }
            val lnurlPayNotificationChannel = NotificationChannel(
                "${applicationId}.${Constants.NOTIFICATION_CHANNEL_LNURL_PAY}",
                getString(
                    context,
                    Constants.LNURL_PAY_NOTIFICATION_CHANNEL_NAME,
                    Constants.DEFAULT_LNURL_PAY_NOTIFICATION_CHANNEL_NAME
                ),
                NotificationManager.IMPORTANCE_DEFAULT
            ).apply {
                description = getString(
                    context,
                    Constants.LNURL_PAY_NOTIFICATION_CHANNEL_DESCRIPTION,
                    Constants.DEFAULT_LNURL_PAY_NOTIFICATION_CHANNEL_DESCRIPTION
                )
                group = Constants.LNURL_PAY_WORKGROUP_ID
            }
            val swapTxConfirmedNotificationChannel = NotificationChannel(
                "${applicationId}.${Constants.NOTIFICATION_CHANNEL_SWAP_TX_CONFIRMED}",
                getString(
                    context,
                    Constants.SWAP_TX_CONFIRMED_NOTIFICATION_CHANNEL_NAME,
                    Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_CHANNEL_NAME
                ),
                NotificationManager.IMPORTANCE_DEFAULT
            ).apply {
                description = getString(
                    context,
                    Constants.SWAP_TX_CONFIRMED_NOTIFICATION_CHANNEL_DESCRIPTION,
                    Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_CHANNEL_DESCRIPTION
                )
                group = Constants.SWAP_TX_CONFIRMED_WORKGROUP_ID
            }
            notificationManager.createNotificationChannels(
                listOf(
                    foregroundServiceNotificationChannel,
                    receivedPaymentsNotificationChannel,
                    lnurlPayNotificationChannel,
                    swapTxConfirmedNotificationChannel
                )
            )
        }

        @RequiresApi(Build.VERSION_CODES.O)
        private fun createNotificationChannelGroup(
            context: Context,
            notificationManager: NotificationManager,
        ) {
            val offlinePaymentsNotificationChannelGroup = NotificationChannelGroup(
                Constants.OFFLINE_PAYMENTS_WORKGROUP_ID,
                getString(
                    context,
                    Constants.OFFLINE_PAYMENTS_WORKGROUP_NAME,
                    Constants.DEFAULT_OFFLINE_PAYMENTS_WORKGROUP_NAME
                ),
            )
            val lnurlPayNotificationChannelGroup = NotificationChannelGroup(
                Constants.LNURL_PAY_WORKGROUP_ID,
                getString(
                    context,
                    Constants.LNURL_PAY_WORKGROUP_NAME,
                    Constants.DEFAULT_LNURL_PAY_WORKGROUP_NAME
                ),
            )
            val swapTxConfirmedNotificationChannelGroup = NotificationChannelGroup(
                Constants.SWAP_TX_CONFIRMED_WORKGROUP_ID,
                getString(
                    context,
                    Constants.SWAP_TX_CONFIRMED_WORKGROUP_NAME,
                    Constants.DEFAULT_SWAP_TX_CONFIRMED_WORKGROUP_NAME
                ),
            )
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                offlinePaymentsNotificationChannelGroup.description = getString(
                    context,
                    Constants.OFFLINE_PAYMENTS_WORKGROUP_DESCRIPTION,
                    Constants.DEFAULT_OFFLINE_PAYMENTS_WORKGROUP_DESCRIPTION
                )
                lnurlPayNotificationChannelGroup.description = getString(
                    context,
                    Constants.LNURL_PAY_WORKGROUP_DESCRIPTION,
                    Constants.DEFAULT_LNURL_PAY_WORKGROUP_DESCRIPTION
                )
                swapTxConfirmedNotificationChannelGroup.description = getString(
                    context,
                    Constants.SWAP_TX_CONFIRMED_WORKGROUP_DESCRIPTION,
                    Constants.DEFAULT_SWAP_TX_CONFIRMED_WORKGROUP_DESCRIPTION
                )
            }

            notificationManager.createNotificationChannelGroups(
                listOf(
                    offlinePaymentsNotificationChannelGroup,
                    lnurlPayNotificationChannelGroup,
                    swapTxConfirmedNotificationChannelGroup
                )
            )
        }

        @SuppressLint("MissingPermission")
        fun notifyForegroundService(context: Context): Notification {
            val notificationColor =
                getColor(
                    context,
                    Constants.NOTIFICATION_COLOR,
                    Constants.DEFAULT_NOTIFICATION_COLOR
                )

            return NotificationCompat.Builder(
                context,
                "${context.applicationInfo.packageName}.${Constants.NOTIFICATION_CHANNEL_FOREGROUND_SERVICE}"
            )
                .apply {
                    setContentTitle(
                        getString(
                            context,
                            Constants.FOREGROUND_SERVICE_NOTIFICATION_TITLE,
                            Constants.DEFAULT_FOREGROUND_SERVICE_NOTIFICATION_TITLE
                        )
                    )
                    setSmallIcon(
                        getDrawable(
                            context,
                            Constants.NOTIFICATION_ICON,
                            android.R.drawable.sym_def_app_icon
                        )
                    )
                    setColorized(true)
                    setOngoing(true)
                    color = notificationColor
                }.build().also {
                    if (ActivityCompat.checkSelfPermission(
                            context,
                            Manifest.permission.POST_NOTIFICATIONS
                        ) == PackageManager.PERMISSION_GRANTED
                    ) {
                        NotificationManagerCompat.from(context)
                            .notify(Constants.NOTIFICATION_ID_FOREGROUND_SERVICE, it)
                    }
                }
        }

        @SuppressLint("MissingPermission")
        fun notifyChannel(
            context: Context,
            channelId: String,
            contentTitle: String,
            contentText: String? = null,
            clickAction: String? = defaultClickAction,
        ): Notification {
            val notificationID: Int = System.currentTimeMillis().toInt() / 1000
            val notificationColor =
                getColor(
                    context,
                    Constants.NOTIFICATION_COLOR,
                    Constants.DEFAULT_NOTIFICATION_COLOR
                )

            val notificationIntent =
                context.packageManager.getLaunchIntentForPackage(context.packageName) ?: Intent()
            notificationIntent.putExtra("click_action", clickAction)

            val flags =
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE else PendingIntent.FLAG_UPDATE_CURRENT
            val approvePendingIntent = PendingIntent.getActivity(
                context,
                0,
                notificationIntent,
                flags
            )

            val buttonTitle = "Open"
            val notificationAction = NotificationCompat.Action.Builder(
                android.R.drawable.ic_delete,
                buttonTitle,
                approvePendingIntent
            ).build()

            val contentIntent = TaskStackBuilder.create(context).run {
                addNextIntentWithParentStack(notificationIntent)
                approvePendingIntent
            }

            return NotificationCompat.Builder(
                context,
                "${context.applicationInfo.packageName}.${channelId}"
            )
                .apply {
                    setContentTitle(contentTitle)
                    setContentText(contentText)
                    setSmallIcon(
                        getDrawable(
                            context,
                            Constants.NOTIFICATION_ICON,
                            android.R.drawable.sym_def_app_icon
                        )
                    )
                    setContentIntent(contentIntent)
                    addAction(notificationAction)
                    setLights(notificationColor, 1000, 300)
                    // Dismiss on click
                    setOngoing(false)
                    setAutoCancel(true)
                }.build().also {
                    if (ActivityCompat.checkSelfPermission(
                            context,
                            Manifest.permission.POST_NOTIFICATIONS
                        ) == PackageManager.PERMISSION_GRANTED
                    ) {
                        // Required for notification to persist after work is complete
                        CoroutineScope(Dispatchers.Main).launch {
                            delay(200)
                            if (ActivityCompat.checkSelfPermission(
                                    context,
                                    Manifest.permission.POST_NOTIFICATIONS
                                ) == PackageManager.PERMISSION_GRANTED
                            ) {
                                // Use notificationID
                                NotificationManagerCompat.from(context)
                                    .notify(notificationID, it)
                            }
                        }
                    }
                }
        }
    }
}
