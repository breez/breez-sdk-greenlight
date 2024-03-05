package breez_sdk_notification

import breez_sdk.BlockingBreezServices
import breez_sdk.ConnectRequest
import breez_sdk.EventListener
import breez_sdk.connect
import breez_sdk_notification.LogHelper.nodeLogStream

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
                    nodeLogStream?.log(TAG, "Connecting to Breez SDK", "DEBUG")
                    breezSDK = connect(connectRequest, sdkListener)
                    nodeLogStream?.log(TAG, "Connected to Breez SDK", "DEBUG")
                } else nodeLogStream?.log(TAG, "Already connected to Breez SDK", "DEBUG")

                return breezSDK!!
            }
        }
    }
}
