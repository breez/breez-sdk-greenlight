package com.breezsdk

import breez_sdk.BreezEvent
import breez_sdk.EventListener
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter

class BreezSDKListener(private val emitter: RCTDeviceEventEmitter): EventListener {
    companion object {
        var emitterName = "breezSdkEvent"
    }

    override fun onEvent(e: BreezEvent) {
        emitter.emit(emitterName, readableMapOf(e))
    }
}