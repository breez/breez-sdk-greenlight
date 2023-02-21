package com.breezsdk

import breez_sdk.LogEntry
import breez_sdk.LogStream
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter

class BreezSDKLogStream(private val emitter: RCTDeviceEventEmitter): LogStream {
    companion object {
        var emitterName = "breezSdkLog"
    }

    override fun log(l: LogEntry) {
        emitter.emit(emitterName, readableMapOf("line" to l.line, "level" to l.level))
    }
}