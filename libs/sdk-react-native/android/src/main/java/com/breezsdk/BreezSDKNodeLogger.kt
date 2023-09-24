package com.breezsdk

import breez_sdk.LogMessage
import breez_sdk.Logger
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter

class BreezSDKNodeLogger(private val emitter: RCTDeviceEventEmitter): Logger {
    companion object {
        var emitterName = "breezSdkNodeLog"
    }

    override fun log(logMessage: LogMessage) {
        emitter.emit(emitterName, readableMapOf(logMessage))
    }
}