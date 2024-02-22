package breez_sdk_notification

import android.app.Service
import android.content.Intent
import android.os.Build
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import breez_sdk.BlockingBreezServices
import breez_sdk.ConnectRequest
import breez_sdk_notification.NotificationHelper.Companion.notifyForegroundService
import breez_sdk_notification.BreezSdkConnector.Companion.connectSDK
import breez_sdk_notification.job.Job
import breez_sdk_notification.job.LnurlPayInfoJob
import breez_sdk_notification.job.LnurlPayInvoiceJob
import breez_sdk_notification.job.ReceivePaymentJob
import kotlinx.coroutines.CoroutineExceptionHandler
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import org.tinylog.kotlin.Logger

abstract class ForegroundService : SdkForegroundService, Service() {
    private var breezSDK: BlockingBreezServices? = null
    val serviceScope = CoroutineScope(Dispatchers.Main.immediate + SupervisorJob())

    companion object {
        private const val TAG = "ForegroundService"
    }

    // =========================================================== //
    //                      SERVICE LIFECYCLE                      //
    // =========================================================== //

    override fun onBind(intent: Intent): IBinder? {
        return null
    }

    /** Stop the service */
    private val shutdownHandler = Handler(Looper.getMainLooper())
    private val shutdownRunnable: Runnable = Runnable {
        Logger.tag(TAG).debug { "Reached scheduled shutdown..." }
        shutdown()
    }

    override fun pushbackShutdown() {
        shutdownHandler.removeCallbacksAndMessages(null)
        shutdownHandler.postDelayed(shutdownRunnable, Constants.SHUTDOWN_DELAY_MS)
    }

    override fun shutdown() {
        Logger.tag(TAG).debug { "Shutting down foreground service" }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
            stopForeground(STOP_FOREGROUND_REMOVE)
        }
        stopSelf()
    }

    // =========================================================== //
    //                    START COMMAND HANDLER                    //
    // =========================================================== //

    /** Called when an intent is called for this service. */
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        super.onStartCommand(intent, flags, startId)
        val intentDetails = "[ intent=$intent, flag=$flags, startId=$startId ]"
        Logger.tag(TAG).debug { "Start foreground service from intent $intentDetails" }

        // Display foreground service notification
        val notification = notifyForegroundService(applicationContext)
        startForeground(Constants.NOTIFICATION_ID_FOREGROUND_SERVICE, notification)

        // Connect to SDK if source intent has data message with valid payload
        getConnectRequest()?.let { connectRequest ->
            getJobFromIntent(intent)?.also { job ->
                launchSdkConnection(connectRequest, job)
            } ?: run {
                Logger.tag(TAG).warn { "Received invalid data message." }
                shutdown()
            }
        } ?: run {
            Logger.tag(TAG).warn { "Missing ConnectRequest." }
            shutdown()
        }

        return START_NOT_STICKY
    }

    /** To be implemented by the application foreground service.
     *  It should retrieve the Breez API key and node mnemonic then construct
     *  a ConnectRequest to be used to call the Breez SDK connect function. */
    abstract fun getConnectRequest(): ConnectRequest?

    /** Get the job to be executed from the Message data in the Intent.
     *  This can be overridden to handle custom jobs. */
    open fun getJobFromIntent(intent: Intent?): Job? {
        return Message.createFromIntent(intent)?.let { message ->
            message.payload?.let { payload ->
                when (message.type) {
                    Constants.MESSAGE_TYPE_PAYMENT_RECEIVED -> ReceivePaymentJob(
                        applicationContext,
                        this,
                        payload
                    )

                    Constants.MESSAGE_TYPE_LNURL_PAY_INFO -> LnurlPayInfoJob(
                        applicationContext,
                        this,
                        payload
                    )

                    Constants.MESSAGE_TYPE_LNURL_PAY_INVOICE -> LnurlPayInvoiceJob(
                        applicationContext,
                        this,
                        payload
                    )

                    else -> null
                }
            }
        }
    }

    private fun launchSdkConnection(connectRequest: ConnectRequest, job: Job) {
        serviceScope.launch(Dispatchers.IO + CoroutineExceptionHandler { _, e ->
            Logger.tag(TAG).error { "Breez SDK connection failed $e" }
            shutdown()
        }) {
            breezSDK ?: run {
                breezSDK = connectSDK(connectRequest, job)
            }

            breezSDK?.let {
                job.start(breezSDK!!)

                // Push back shutdown by SHUTDOWN_DELAY_MS
                pushbackShutdown()
            }
        }
    }
}
