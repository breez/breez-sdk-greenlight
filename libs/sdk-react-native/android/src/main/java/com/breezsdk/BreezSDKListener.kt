package com.breezsdk

import breez_sdk.BreezEvent
import breez_sdk.EventListener
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter

class BreezSDKListener(private val emitter: RCTDeviceEventEmitter): EventListener {
    companion object {
        var emitterName = "breezSdkEvent"
    }

    override fun onEvent(e: BreezEvent) {
        when (e) {
            is BreezEvent.InvoicePaid -> emitter.emit(emitterName, readableMapOf("type" to "invoicePaid", "data" to readableMapOf(e.details)))
            is BreezEvent.NewBlock -> emitter.emit(emitterName, readableMapOf("type" to "newBlock", "data" to e.block))
            is BreezEvent.PaymentFailed -> emitter.emit(emitterName, readableMapOf("type" to "paymentFailed", "data" to readableMapOf(e.details)))
            is BreezEvent.PaymentSucceed -> emitter.emit(emitterName, readableMapOf("type" to "paymentSucceed", "data" to readableMapOf(e.details)))
            is BreezEvent.Synced -> emitter.emit(emitterName, readableMapOf("type" to "synced"))
            is BreezEvent.BackupStarted -> emitter.emit(emitterName, readableMapOf("type" to "backupStarted"))
            is BreezEvent.BackupSucceeded -> emitter.emit(emitterName, readableMapOf("type" to "backupSucceeded"))
            is BreezEvent.BackupFailed -> emitter.emit(emitterName, readableMapOf("type" to "backupFailed", "data" to readableMapOf(e.details)))
        }
    }
}