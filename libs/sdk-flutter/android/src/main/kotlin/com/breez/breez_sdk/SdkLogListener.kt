package com.breez.breez_sdk

import android.os.Handler
import android.os.Looper
import breez_sdk.LogEntry
import breez_sdk.LogStream
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import io.flutter.plugin.common.EventChannel

class SdkLogListener : LogStream {
    private val _sharedFlow = MutableSharedFlow<LogEntry>() // private mutable shared flow

    override fun log(l: LogEntry) {
        produceLog(l)
    }

    private fun produceLog(log: LogEntry) = _sharedFlow.tryEmit(log)

    fun subscribe(scope: CoroutineScope, block: suspend (LogEntry) -> Unit) = _sharedFlow.onEach(block).launchIn(scope)
}