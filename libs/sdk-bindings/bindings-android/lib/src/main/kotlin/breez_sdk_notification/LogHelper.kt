package breez_sdk_notification

import org.tinylog.kotlin.Logger
import java.io.File

@Suppress("unused")
class LogHelper {
    companion object {
        private const val TAG = "LogHelper"

        private var isInit: Boolean? = null

        fun configureLogger(
            loggingDir: File,
        ): Boolean? {
            synchronized(this) {
                System.setProperty("tinylog.directory", loggingDir.absolutePath)
                System.setProperty("tinylog.timestamp", System.currentTimeMillis().toString())

                if (isInit == false) {
                    Logger.tag(TAG).debug { "Logs directory: '$loggingDir'" }
                    isInit = true
                }
                return isInit
            }
        }
    }
}
