package breez_sdk_notification

import android.content.Context
import io.flutter.util.PathUtils
import org.tinylog.kotlin.Logger
import java.io.File

class LogHelper {
    companion object {
        private const val TAG = "LogHelper"

        private var isInit: Boolean? = null

        fun configureLogger(applicationContext: Context): Boolean? {
            synchronized(this) {
                val loggingDir =
                    File(PathUtils.getDataDirectory(applicationContext), "/logs/").apply {
                        mkdirs()
                    }

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
