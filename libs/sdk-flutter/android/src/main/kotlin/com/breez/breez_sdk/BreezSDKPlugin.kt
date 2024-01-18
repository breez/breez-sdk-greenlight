package com.breez.breez_sdk

import android.os.Handler
import android.os.Looper
import androidx.annotation.NonNull
import breez_sdk.LogEntry
import breez_sdk.LogStream
import breez_sdk.setLogStream

import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.plugin.common.EventChannel
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach

/** BreezSDKPlugin */
class BreezSDKPlugin : FlutterPlugin, MethodCallHandler, EventChannel.StreamHandler {
    /// The MethodChannel that will the communication between Flutter and native Android
    ///
    /// This local reference serves to register the plugin with the Flutter Engine and unregister it
    /// when the Flutter Engine is detached from the Activity
    private lateinit var channel: MethodChannel
    private var eventChannel: EventChannel? = null
    private var eventSink: EventChannel.EventSink? = null
    private var nodeLogStream: LogStream? = null

    override fun onAttachedToEngine(@NonNull flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
        channel = MethodChannel(flutterPluginBinding.binaryMessenger, "breez_sdk")
        channel.setMethodCallHandler(this)

        eventChannel =
            EventChannel(flutterPluginBinding.binaryMessenger, "breez_sdk_node_logs")
        setNodeLogStream()
        eventChannel?.setStreamHandler(this)
    }

    override fun onMethodCall(@NonNull call: MethodCall, @NonNull result: Result) {
        if (call.method == "getPlatformVersion") {
            result.success("Android ${android.os.Build.VERSION.RELEASE}")
        } else {
            result.notImplemented()
        }
    }

    override fun onDetachedFromEngine(@NonNull binding: FlutterPlugin.FlutterPluginBinding) {
        channel.setMethodCallHandler(null)
    }


    override fun onListen(arguments: Any?, events: EventChannel.EventSink?) {
        eventSink = events
    }

    override fun onCancel(arguments: Any?) {
        eventSink = null
        eventChannel = null
    }

    private fun setNodeLogStream(): LogStream? {
        // Set Log Stream
        if (nodeLogStream == null) {
            try {
                nodeLogStream = SdkLogListener()
                setLogStream(nodeLogStream!!)
            } catch (ex: Throwable) {
            }
        }
        return nodeLogStream!!
    }

    inner class SdkLogListener : LogStream {
        // Event has to be handled on main thread.
        private var handler = Handler(Looper.getMainLooper())
        private val _sharedFlow = MutableSharedFlow<LogEntry>() // private mutable shared flow

        override fun log(l: LogEntry) {
            produceLog(l)

            val runnable = Runnable {
                val data = mapOf("level" to l.level, "line" to l.line)
                eventSink?.success(data)
            }

            handler.post(runnable)
        }

        private fun produceLog(log: LogEntry) = _sharedFlow.tryEmit(log)

        fun subscribe(scope: CoroutineScope, block: suspend (LogEntry) -> Unit) = _sharedFlow.onEach(block).launchIn(scope)
    }
}
