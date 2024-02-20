package breez_sdk_notification.job

import breez_sdk.BlockingBreezServices
import breez_sdk.EventListener

interface Job : EventListener {
    fun start(breezSDK: BlockingBreezServices)
}
