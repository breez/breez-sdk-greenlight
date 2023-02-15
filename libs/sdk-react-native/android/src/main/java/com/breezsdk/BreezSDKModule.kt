package com.breezsdk

import breez_sdk.*
import com.facebook.react.bridge.*
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter

class BreezSDKModule(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {
    private var breezServices: BlockingBreezServices? = null

    companion object {
        var TAG = "BreezSDK"
    }

    override fun getName(): String {
        return TAG
    }

    @ReactMethod
    fun addListener(eventName: String) {}

    @ReactMethod
    fun removeListeners(count: Int) {}

    @ReactMethod
    fun mnemonicToSeed(mnemonic: String, promise: Promise) {
        try {
            var seed = mnemonicToSeed(mnemonic)
            promise.resolve(readableArrayOf(seed))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, "Error calling mnemonicToSeed", e)
        }
    }

    @ReactMethod
    fun parseInput(input: String, promise: Promise) {
        try {
            var inputType = parseInput(input)
            promise.resolve(readableMapOf(inputType))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, "Error calling parseInput", e)
        }
    }

    @ReactMethod
    fun parseInvoice(invoice: String, promise: Promise) {
        try {
            var lnInvoice = parseInvoice(invoice)
            promise.resolve(readableMapOf(lnInvoice))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, "Error calling parseInvoice", e)
        }
    }

    @ReactMethod
    fun registerNode(network: String, seed: ReadableArray, promise: Promise) {
        try {
            var creds = registerNode(asNetwork(network), asUByteList(seed))
            promise.resolve(readableMapOf(creds))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, "Error calling registerNode", e)
        }
    }

    @ReactMethod
    fun recoverNode(network: String, seed: ReadableArray, promise: Promise) {
        try {
            var creds = recoverNode(asNetwork(network), asUByteList(seed))
            promise.resolve(readableMapOf(creds))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, "Error calling recoverNode", e)
        }
    }

    @ReactMethod
    fun startLogStream(promise: Promise) {
        try {
            var emitter = reactApplicationContext
                    .getJSModule(RCTDeviceEventEmitter::class.java)

            setLogStream(BreezSDKLogStream(emitter))
            promise.resolve("Log stream started")
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, "Error calling setLogStream", e)
        }
    }

    @ReactMethod
    fun initServices(apiKey: String, deviceKey: ReadableArray, deviceCert: ReadableArray, seed: ReadableArray, promise: Promise) {
        if (this.breezServices != null) {
            promise.reject(TAG, "BreezServices already initialized")
        }

        var emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)
        var creds = GreenlightCredentials(asUByteList(deviceKey), asUByteList(deviceCert))
        var config = defaultConfig(EnvironmentType.PRODUCTION)
        config.apiKey = apiKey

        try {
            this.breezServices = initServices(config, asUByteList(seed), creds, BreezSDKListener(emitter))
            promise.resolve("BreezServices initialized")
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, "Error calling initServices", e)
        }
    }

    @ReactMethod
    fun start(promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                breezServices.start()
                promise.resolve("BreezServices started")
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling start", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun sync(promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                breezServices.sync()
                promise.resolve("BreezServices syncing")
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling sync", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun stop(promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                breezServices.stop()
                promise.resolve("BreezServices stopped")
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling stop", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun sendPayment(bolt11: String, amountSats: String?, promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                var payment = breezServices.sendPayment(bolt11, amountSats?.toULong())

                promise.resolve(readableMapOf(payment))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling sendPayment", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun sendSpontaneousPayment(nodeId: String, amountSats: String, promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                var payment = breezServices.sendSpontaneousPayment(nodeId, amountSats.toULong())

                promise.resolve(readableMapOf(payment))
            } catch (e: NumberFormatException) {
                e.printStackTrace()
                promise.reject(TAG, "Invalid amountSats", e)
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling sendSpontaneousPayment", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun receivePayment(amountSats: String, description: String, promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                var payment = breezServices.receivePayment(amountSats.toULong(), description)

                promise.resolve(readableMapOf(payment))
            } catch (e: NumberFormatException) {
                e.printStackTrace()
                promise.reject(TAG, "Invalid amountSats", e)
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling receivePayment", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun withdrawLnurl(reqData: ReadableMap, amountSats: String, description: String?, promise: Promise) {
        this.breezServices?.let {breezServices->
            var lnUrlWithdrawRequestData = asLnUrlWithdrawRequestData(reqData)

            if (lnUrlWithdrawRequestData == null) {
                promise.reject(TAG, "Invalid reqData")
            } else {
                try {
                    var lnUrlWithdrawCallbackStatus = breezServices.withdrawLnurl(lnUrlWithdrawRequestData, amountSats.toULong(), description)

                    promise.resolve(readableMapOf(lnUrlWithdrawCallbackStatus))
                } catch (e: NumberFormatException) {
                    e.printStackTrace()
                    promise.reject(TAG, "Invalid amountSats", e)
                } catch (e: SdkException) {
                    e.printStackTrace()
                    promise.reject(TAG, "Error calling receivePayment", e)
                }
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }
}
