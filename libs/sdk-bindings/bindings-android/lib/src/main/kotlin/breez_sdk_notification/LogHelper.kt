package breez_sdk_notification

import breez_sdk.setLogStream
import kotlinx.coroutines.CoroutineScope

@Suppress("unused")
object LogHelper {
    var nodeLogStream: SdkLogListener? = null

    fun initializeNodeLogStream(): SdkLogListener {
        if (nodeLogStream == null) {
            try {
                nodeLogStream = SdkLogListener()
                setLogStream(nodeLogStream!!)
            } catch (e: Throwable) {
                // Reset nodeLogStream if setting log stream fails
                e.printStackTrace()
                nodeLogStream = null
                throw e
            }
        }
        return nodeLogStream!!
    }

    fun unsubscribeNodeLogStream(scope: CoroutineScope) {
        nodeLogStream?.unsubscribe(scope)
    }
}
