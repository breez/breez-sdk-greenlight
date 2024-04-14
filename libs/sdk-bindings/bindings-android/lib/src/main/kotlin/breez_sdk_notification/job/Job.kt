package breez_sdk_notification.job

import breez_sdk.BlockingBreezServices
import breez_sdk.EventListener

interface Job : EventListener {
    /** When the notification service is connected to the Breez SDK
     *  it calls `start` to initiate the job.
     */
    fun start(breezSDK: BlockingBreezServices)

    /** When the short service timeout is reached it calls `onShutdown` 
     *  to cleanup the job.
     */
    fun onShutdown()
}
