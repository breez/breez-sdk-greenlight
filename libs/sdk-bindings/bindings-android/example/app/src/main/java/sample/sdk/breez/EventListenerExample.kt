package sample.sdk.breez

import android.util.Log
import breez_sdk.BreezEvent
import breez_sdk.EventListener

private const val TAG = "EventListenerExample"

class EventListenerExample : EventListener {

    override fun onEvent(e: BreezEvent) {
        when (e) {
            is BreezEvent.NewBlock -> newBlock(e)
            is BreezEvent.InvoicePaid -> invoicePaid(e)
            is BreezEvent.Synced -> synced(e)
            is BreezEvent.PaymentSucceed -> paymentSucceed(e)
            is BreezEvent.PaymentFailed -> paymentFailed(e)
            is BreezEvent.BackupStarted -> backupStarted(e)
            is BreezEvent.BackupSucceeded -> backupSucceeded(e)
            is BreezEvent.BackupFailed -> backupFailed(e)
        }
    }

    private fun newBlock(event: BreezEvent.NewBlock) {
        Log.d(TAG, "newBlock: $event")
    }

    private fun invoicePaid(event: BreezEvent.InvoicePaid) {
        Log.d(TAG, "invoicePaid: $event")
    }

    private fun synced(event: BreezEvent.Synced) {
        Log.d(TAG, "synced: $event")
    }

    private fun paymentSucceed(event: BreezEvent.PaymentSucceed) {
        Log.d(TAG, "paymentSucceed: $event")
    }

    private fun paymentFailed(event: BreezEvent.PaymentFailed) {
        Log.d(TAG, "paymentFailed: $event")
    }

    private fun backupStarted(event: BreezEvent.BackupStarted) {
        Log.d(TAG, "backupStarted: $event")
    }

    private fun backupSucceeded(event: BreezEvent.BackupSucceeded) {
        Log.d(TAG, "backupSucceeded: $event")
    }

    private fun backupFailed(event: BreezEvent.BackupFailed) {
        Log.d(TAG, "backupFailed: $event")
    }

}
