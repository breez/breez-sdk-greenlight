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
            promise.resolve(readableMapOf("status" to "ok"))
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
            promise.resolve(readableMapOf("status" to "ok"))
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
                promise.resolve(readableMapOf("status" to "ok"))
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
                promise.resolve(readableMapOf("status" to "ok"))
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
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling stop", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun sendPayment(bolt11: String, amountSats: Double, promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                var optionalAmountSats = amountSats.takeUnless { it == 0.0 }
                var payment = breezServices.sendPayment(bolt11, optionalAmountSats?.toULong())

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
    fun sendSpontaneousPayment(nodeId: String, amountSats: Double, promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                var payment = breezServices.sendSpontaneousPayment(nodeId, amountSats.toULong())

                promise.resolve(readableMapOf(payment))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling sendSpontaneousPayment", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun receivePayment(amountSats: Double, description: String, promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                var payment = breezServices.receivePayment(amountSats.toULong(), description)

                promise.resolve(readableMapOf(payment))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling receivePayment", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun withdrawLnurl(reqData: ReadableMap, amountSats: Double, description: String?, promise: Promise) {
        this.breezServices?.let {breezServices->
            var lnUrlWithdrawRequestData = asLnUrlWithdrawRequestData(reqData)

            if (lnUrlWithdrawRequestData == null) {
                promise.reject(TAG, "Invalid reqData")
            } else {
                try {
                    var lnUrlWithdrawCallbackStatus = breezServices.withdrawLnurl(lnUrlWithdrawRequestData, amountSats.toULong(), description)

                    promise.resolve(readableMapOf(lnUrlWithdrawCallbackStatus))
                } catch (e: SdkException) {
                    e.printStackTrace()
                    promise.reject(TAG, "Error calling withdrawLnurl", e)
                }
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun nodeInfo(promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                breezServices.nodeInfo()?.let {nodeState->
                    promise.resolve(readableMapOf(nodeState))
                } ?: run {
                    promise.reject(TAG, "No available node info")
                }
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling nodeInfo", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun listPayments(filter: String, fromTimestamp: Double, toTimestamp: Double, promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                var optionalFromTimestamp = fromTimestamp.takeUnless { it == 0.0 }
                var optionalToTimestamp = toTimestamp.takeUnless { it == 0.0 }
                var payments = breezServices.listPayments(asPaymentTypeFilter(filter), optionalFromTimestamp?.toLong(), optionalToTimestamp?.toLong())

                promise.resolve(readableArrayOf(payments))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling listPayments", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun sweep(toAddress: String, feeRateSatsPerByte: Double, promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                breezServices.sweep(toAddress, feeRateSatsPerByte.toULong())

                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling sweep", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun fetchFiatRates(promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                var rates = breezServices.fetchFiatRates()

                promise.resolve(readableArrayOf(rates))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling fetchFiatRates", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun listFiatCurrencies(promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                var fiatCurrencies = breezServices.listFiatCurrencies()

                promise.resolve(readableArrayOf(fiatCurrencies))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling listFiatCurrencies", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun listLsps(promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                var lsps = breezServices.listLsps()

                promise.resolve(readableArrayOf(lsps))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling listLsps", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun connectLsp(lspId: String, promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                breezServices.connectLsp(lspId)

                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling connectLsp", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun fetchLspInfo(lspId: String, promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                breezServices.fetchLspInfo(lspId)?.let {lspInformation->
                    promise.resolve(readableMapOf(lspInformation))
                } ?: run {
                    promise.reject(TAG, "No available lsp info")
                }
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling fetchLspInfo", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun lspId(promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                breezServices.lspId()?.let {lspId->
                    promise.resolve(lspId)
                } ?: run {
                    promise.reject(TAG, "No available lsp id")
                }
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling lspId", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }

    @ReactMethod
    fun closeLspChannels(promise: Promise) {
        this.breezServices?.let {breezServices->
            try {
                breezServices.closeLspChannels()

                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, "Error calling closeLspChannels", e)
            }
        } ?: run {
            promise.reject(TAG, "BreezServices not initialized")
        }
    }
}
