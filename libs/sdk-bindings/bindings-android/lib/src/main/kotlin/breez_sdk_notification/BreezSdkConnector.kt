package breez_sdk_notification

import breez_sdk.*
import org.tinylog.kotlin.Logger

class BreezSdkConnector {
    companion object {
        private const val TAG = "BreezSdkConnector"

        private var breezSDK: BlockingBreezServices? = null

        internal fun connectSDK(
            connectRequest: ConnectRequest,
            sdkListener: EventListener,
        ): BlockingBreezServices {
            synchronized(this) {
                if (breezSDK == null) {
                    Logger.tag(TAG).debug { "Connecting to Breez SDK" }
                    breezSDK = connect(connectRequest, sdkListener)
                    Logger.tag(TAG).debug { "Connected to Breez SDK" }
                } else Logger.tag(TAG).debug { "Already connected to Breez SDK" }

                return breezSDK!!
            }
        }
    }
}