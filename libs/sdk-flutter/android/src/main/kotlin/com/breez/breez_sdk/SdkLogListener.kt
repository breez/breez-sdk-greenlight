package com.breez.breez_sdk

import breez_sdk.LogEntry
import breez_sdk.LogStream
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch

enum class LevelFilter(val weight: Int) {
    TRACE(4),
    DEBUG(3),
    INFO(2),
    WARNING(1),
    ERROR(0)
}

class SdkLogListener(private val levelFilter: String = "TRACE") : LogStream {
    private val scope = CoroutineScope(SupervisorJob())

    private val _logEvents = MutableSharedFlow<LogEntry>()
    private val logEvents: SharedFlow<LogEntry> = _logEvents.asSharedFlow()

    override fun log(l: LogEntry) {
        scope.launch {
            if (LevelFilter.valueOf(levelFilter).weight >= LevelFilter.valueOf(l.level).weight) {
                _logEvents.emit(l)
            }
        }
    }

    fun subscribe(scope: CoroutineScope, block: suspend (LogEntry) -> Unit) =
        logEvents.onEach(block).launchIn(scope)

    fun unsubscribe(scope: CoroutineScope) = scope.cancel()
}