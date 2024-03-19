package com.breez.breez_sdk

import breez_sdk.setLogStream
import kotlinx.coroutines.CoroutineScope

object SdkLogInitializer {
    private var nodeLogStream: SdkLogListener? = null

    fun initializeNodeLogStream(levelFilter: String = "TRACE"): SdkLogListener {
        if (nodeLogStream == null) {
            try {
                nodeLogStream = SdkLogListener(levelFilter)
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

    fun unsubscribeNodeLogStream(scope: CoroutineScope) {
        nodeLogStream?.unsubscribe(scope)
    }
}