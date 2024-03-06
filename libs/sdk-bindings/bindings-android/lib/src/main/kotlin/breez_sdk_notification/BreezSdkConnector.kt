package breez_sdk_notification

import breez_sdk.BlockingBreezServices
import breez_sdk.ConnectRequest
import breez_sdk.EventListener
import breez_sdk.LogEntry
import breez_sdk.LogStream
import breez_sdk.connect

class BreezSdkConnector {
    companion object {
        private const val TAG = "BreezSdkConnector"

        private var breezSDK: BlockingBreezServices? = null

        internal fun connectSDK(
            connectRequest: ConnectRequest,
            sdkListener: EventListener,
            logger: LogStream?
        ): BlockingBreezServices {
            synchronized(this) {
                if (breezSDK == null) {
                    logger?.log(
                        LogEntry(TAG, "Connecting to Breez SDK", "DEBUG")
                    )
                    breezSDK = connect(connectRequest, sdkListener)
                    logger?.log(LogEntry(TAG, "Connected to Breez SDK", "DEBUG"))
                } else logger?.log(LogEntry(TAG, "Already connected to Breez SDK", "DEBUG"))

                return breezSDK!!
            }
        }
    }
}
