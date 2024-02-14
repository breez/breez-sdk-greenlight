package com.breez.breez_sdk

import android.os.Handler
import android.os.Looper
import breez_sdk.LogEntry
import breez_sdk.setLogStream
import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.plugin.common.EventChannel
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers

/** BreezSDKPlugin */
class BreezSDKPlugin : FlutterPlugin, MethodCallHandler, EventChannel.StreamHandler {
    // Event has to be handled on main thread.
    private var handler = Handler(Looper.getMainLooper())

    /// The MethodChannel that will the communication between Flutter and native Android
    ///
    /// This local reference serves to register the plugin with the Flutter Engine and unregister it
    /// when the Flutter Engine is detached from the Activity
    private lateinit var channel: MethodChannel
    private var eventChannel: EventChannel? = null
    private var eventSink: EventChannel.EventSink? = null

    override fun onAttachedToEngine(flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
        channel = MethodChannel(flutterPluginBinding.binaryMessenger, "breez_sdk")
        channel.setMethodCallHandler(this)

        eventChannel =
            EventChannel(flutterPluginBinding.binaryMessenger, "breez_sdk_node_logs")
        val nodeLogStream = setNodeLogStream()
        nodeLogStream.subscribe(CoroutineScope(Dispatchers.Main)) { l: LogEntry ->
            val runnable = Runnable {
                val data = mapOf("level" to l.level, "line" to l.line)
                eventSink?.success(data)
            }

            handler.post(runnable)
        }
        eventChannel?.setStreamHandler(this)
    }

    override fun onMethodCall(call: MethodCall, result: Result) {
        if (call.method == "getPlatformVersion") {
            result.success("Android ${android.os.Build.VERSION.RELEASE}")
        } else {
            result.notImplemented()
        }
    }

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        channel.setMethodCallHandler(null)
    }

    override fun onListen(arguments: Any?, events: EventChannel.EventSink?) {
        eventSink = events
    }

    override fun onCancel(arguments: Any?) {
        eventSink = null
        eventChannel = null
    }

    companion object {
        private var nodeLogStream: SdkLogListener? = null

        fun setNodeLogStream(): SdkLogListener {
            // Set Log Stream
            if (nodeLogStream == null) {
                try {
                    nodeLogStream = SdkLogListener()
                    setLogStream(nodeLogStream!!)
                } catch (e: Throwable) {
                    // Reset nodeLogStream if setting log stream fails
                    e.printStackTrace()
                    nodeLogStream = null
                    throw e
                }
            }
            return nodeLogStream!!
        }
    }
}
