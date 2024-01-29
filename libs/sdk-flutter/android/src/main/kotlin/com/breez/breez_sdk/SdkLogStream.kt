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

class SdkLogListener(eventSink: EventChannel.EventSink) : LogStream {
    // Event has to be handled on main thread.
    private var handler = Handler(Looper.getMainLooper())
    private val _sharedFlow = MutableSharedFlow<LogEntry>() // private mutable shared flow
    private var _eventSink: EventChannel.EventSink? = null

    init {
        _eventSink = eventSink
    }

    override fun log(l: LogEntry) {
        produceLog(l)

        val runnable = Runnable {
            val data = mapOf("level" to l.level, "line" to l.line)
            _eventSink?.success(data)
        }

        handler.post(runnable)
    }

    private fun produceLog(log: LogEntry) = _sharedFlow.tryEmit(log)

    fun subscribe(scope: CoroutineScope, block: suspend (LogEntry) -> Unit) = _sharedFlow.onEach(block).launchIn(scope)
}