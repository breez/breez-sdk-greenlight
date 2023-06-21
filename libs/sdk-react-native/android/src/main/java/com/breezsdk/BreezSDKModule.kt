package com.breezsdk

import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.os.Message
import breez_sdk.*
import com.facebook.react.bridge.*
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter
import java.io.File
import java.util.*
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors


class BreezSDKModule(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {
    private lateinit var executor: ExecutorService
    private var breezServices: BlockingBreezServices? = null
    private val requests = HashMap<Int, Promise>()

    companion object {
        const val MSG_RESPONSE = 1
        const val MSG_ERROR = 2
        
        const val MAP_CHAR: Char = '{'
        const val ARRAY_CHAR: Char = '['

        const val TAG = "RNBreezSDK"
    }

    override fun initialize() {
        super.initialize()

        executor = Executors.newFixedThreadPool(3)
    }

    private val responseHandler: Handler = object : Handler(Looper.getMainLooper()) {
        override fun handleMessage(response: Message) {
            val responseData = response.data

            when (response.what) {
                MSG_RESPONSE -> {
                    val promise = requests.remove(response.arg1)

                    if (promise != null) {
                        val data = responseData.getString("data")

                        if (data != null && data.length > 1) {
                            if (data[0] == MAP_CHAR) {
                                return promise.resolve(deserializeMap(data))
                            } else if (data[0] == ARRAY_CHAR) {
                                return promise.resolve(deserializeArray(data))
                            }
                        }

                        return promise.resolve(data)
                    }
                }
                MSG_ERROR -> {
                    val promise = requests.remove(response.arg1)
                    val error = responseData.getString("error")

                    if (promise != null && error != null) {
                        promise.reject(TAG, error)
                    }
                }
                else -> super.handleMessage(response)
            }
        }
    }

    private fun getRequestId(promise: Promise): Int {
        val requestId: Int = Random().nextInt()
        requests[requestId] = promise

        return requestId
    }

    override fun getName(): String {
        return TAG
    }

    @Throws(SdkException::class)
    fun getBreezServices(): BlockingBreezServices {
        if (breezServices != null) {
            return breezServices!!
        }

        throw SdkException.Exception("BreezServices not initialized")
    }

    @ReactMethod
    fun addListener(eventName: String) {}

    @ReactMethod
    fun removeListeners(count: Int) {}

    @ReactMethod
    fun mnemonicToSeed(mnemonic: String, promise: Promise) {
        val requestId = getRequestId(promise)

        executor.execute {
            val message = Message.obtain(null, MSG_RESPONSE, requestId, 0)
            val data = Bundle()

            try {
                val seed = mnemonicToSeed(mnemonic)
                data.putString("data", serialize(readableArrayOf(seed)))
            } catch (e: SdkException) {
                e.printStackTrace()
                message.what = MSG_ERROR
                data.putString("error", e.message ?: "Error calling mnemonicToSeed")
            }

            message.data = data
            responseHandler.sendMessage(message)
        }
    }

    @ReactMethod
    fun parseInput(input: String, promise: Promise) {
        try {
            val inputType = parseInput(input)
            promise.resolve(readableMapOf(inputType))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling parseInput", e)
        }
    }

    @ReactMethod
    fun parseInvoice(invoice: String, promise: Promise) {
        try {
            val lnInvoice = parseInvoice(invoice)
            promise.resolve(readableMapOf(lnInvoice))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling parseInvoice", e)
        }
    }

    @ReactMethod
    fun registerNode(network: String, seed: ReadableArray, registerCredentials: ReadableMap, inviteCode: String, promise: Promise) {
        try {
            val registerCreds = asGreenlightCredentials(registerCredentials)
            val optionalInviteCode = inviteCode.takeUnless { it.isEmpty() }
            val creds = registerNode(asNetwork(network), asUByteList(seed), registerCreds, optionalInviteCode)
            promise.resolve(readableMapOf(creds))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling registerNode", e)
        }
    }

    @ReactMethod
    fun recoverNode(network: String, seed: ReadableArray, promise: Promise) {
        val requestId = getRequestId(promise)

        executor.execute {
            val message = Message.obtain(null, MSG_RESPONSE, requestId, 0)
            val data = Bundle()

            try {
                val creds = recoverNode(asNetwork(network), asUByteList(seed))
                data.putString("data", serialize(readableMapOf(creds)))
            } catch (e: SdkException) {
                e.printStackTrace()
                message.what = MSG_ERROR
                data.putString("error", e.message ?: "Error calling recoverNode")
            }

            message.data = data
            responseHandler.sendMessage(message)
        }
    }

    @ReactMethod
    fun startLogStream(promise: Promise) {
        try {
            val emitter = reactApplicationContext
                    .getJSModule(RCTDeviceEventEmitter::class.java)

            setLogStream(BreezSDKLogStream(emitter))
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling setLogStream", e)
        }
    }

    @ReactMethod
    fun defaultConfig(envType: String, promise: Promise) {
        try {
            val workingDir = File(reactApplicationContext.filesDir.toString() + "/breezSdk")

            if (!workingDir.exists()) {
                workingDir.mkdirs()
            }

            val config = defaultConfig(asEnvironmentType(envType))
            config.workingDir = workingDir.absolutePath

            promise.resolve(readableMapOf(config))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling defaultConfig", e)
        }
    }

    @ReactMethod
    fun initServices(config: ReadableMap, deviceKey: ReadableArray, deviceCert: ReadableArray, seed: ReadableArray, promise: Promise) {
        if (breezServices != null) {
            promise.reject(TAG, "BreezServices already initialized")
        }

        val configData = asConfig(config)

        if (configData == null) {
            promise.reject(TAG, "Invalid config")
        } else {
            val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)
            val creds = GreenlightCredentials(asUByteList(deviceKey), asUByteList(deviceCert))

            try {
                breezServices = initServices(configData, asUByteList(seed), creds, BreezSDKListener(emitter))
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling initServices", e)
            }
        }
    }

    @ReactMethod
    fun start(promise: Promise) {
        val requestId = getRequestId(promise)

        executor.execute {
            val message = Message.obtain(null, MSG_RESPONSE, requestId, 0)
            val data = Bundle()

            try {
                getBreezServices().start()
                data.putString("data", serialize(readableMapOf("status" to "ok")))
            } catch (e: SdkException) {
                e.printStackTrace()
                message.what = MSG_ERROR
                data.putString("error", e.message ?: "Error calling start")
            }

            message.data = data
            responseHandler.sendMessage(message)
        }
    }

    @ReactMethod
    fun sync(promise: Promise) {
        try {
            getBreezServices().sync()
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling sync", e)
        }
    }

    @ReactMethod
    fun stop(promise: Promise) {
        try {
            getBreezServices().stop()
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling stop", e)
        }
    }

    @ReactMethod
    fun sendPayment(bolt11: String, amountSats: Double, promise: Promise) {
        try {
            val optionalAmountSats = amountSats.takeUnless { it == 0.0 }
            val payment = getBreezServices().sendPayment(bolt11, optionalAmountSats?.toULong())
            promise.resolve(readableMapOf(payment))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling sendPayment", e)
        }
    }

    @ReactMethod
    fun sendSpontaneousPayment(nodeId: String, amountSats: Double, promise: Promise) {
        try {
            val payment = getBreezServices().sendSpontaneousPayment(nodeId, amountSats.toULong())
            promise.resolve(readableMapOf(payment))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling sendSpontaneousPayment", e)
        }
    }

    @ReactMethod
    fun receivePayment(amountSats: Double, description: String, promise: Promise) {
        try {
            val payment = getBreezServices().receivePayment(amountSats.toULong(), description)
            promise.resolve(readableMapOf(payment))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling receivePayment", e)
        }
    }

    @ReactMethod
    fun lnurlAuth(reqData: ReadableMap, promise: Promise) {
        val lnUrlAuthRequestData = asLnUrlAuthRequestData(reqData)

        if (lnUrlAuthRequestData == null) {
            promise.reject(TAG, "Invalid reqData")
        } else {
            try {
                val lnUrlCallbackStatus = getBreezServices().lnurlAuth(lnUrlAuthRequestData)
                promise.resolve(readableMapOf(lnUrlCallbackStatus))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling lnurlAuth", e)
            }
        }
    }

    @ReactMethod
    fun payLnurl(reqData: ReadableMap, amountSats: Double, comment: String, promise: Promise) {
        val lnUrlPayRequestData = asLnUrlPayRequestData(reqData)

        if (lnUrlPayRequestData == null) {
            promise.reject(TAG, "Invalid reqData")
        } else {
            try {
                val optionalComment = comment.takeUnless { it.isEmpty() }
                val lnUrlPayResult = getBreezServices().payLnurl(lnUrlPayRequestData, amountSats.toULong(), optionalComment)
                promise.resolve(readableMapOf(lnUrlPayResult))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling payLnurl", e)
            }
        }
    }

    @ReactMethod
    fun withdrawLnurl(reqData: ReadableMap, amountSats: Double, description: String, promise: Promise) {
        val lnUrlWithdrawRequestData = asLnUrlWithdrawRequestData(reqData)

        if (lnUrlWithdrawRequestData == null) {
            promise.reject(TAG, "Invalid reqData")
        } else {
            try {
                val optionalDescription = description.takeUnless { it.isEmpty() }
                val lnUrlCallbackStatus = getBreezServices().withdrawLnurl(lnUrlWithdrawRequestData, amountSats.toULong(), optionalDescription)
                promise.resolve(readableMapOf(lnUrlCallbackStatus))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling withdrawLnurl", e)
            }
        }
    }

    @ReactMethod
    fun nodeInfo(promise: Promise) {
        val requestId = getRequestId(promise)

        executor.execute {
            val message = Message.obtain(null, MSG_RESPONSE, requestId, 0)
            val data = Bundle()

            try {
                val nodeState = getBreezServices().nodeInfo()

                if (nodeState != null) {
                    data.putString("data", serialize(readableMapOf(nodeState)))
                } else {
                    data.putString("error", "No available node info")
                }
            } catch (e: SdkException) {
                e.printStackTrace()
                message.what = MSG_ERROR
                data.putString("error", e.message ?: "Error calling nodeInfo")
            }

            message.data = data
            responseHandler.sendMessage(message)
        }
    }

    @ReactMethod
    fun listPayments(filter: String, fromTimestamp: Double, toTimestamp: Double, promise: Promise) {
        try {
            val optionalFromTimestamp = fromTimestamp.takeUnless { it == 0.0 }
            val optionalToTimestamp = toTimestamp.takeUnless { it == 0.0 }
            val payments = getBreezServices().listPayments(asPaymentTypeFilter(filter), optionalFromTimestamp?.toLong(), optionalToTimestamp?.toLong())
            promise.resolve(readableArrayOf(payments))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling listPayments", e)
        }
    }

    @ReactMethod
    fun sweep(toAddress: String, feeRateSatsPerVbyte: Double, promise: Promise) {
        try {
            getBreezServices().sweep(toAddress, feeRateSatsPerVbyte.toULong())
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling sweep", e)
        }
    }

    @ReactMethod
    fun fetchFiatRates(promise: Promise) {
        try {
            val rates = getBreezServices().fetchFiatRates()
            promise.resolve(readableArrayOf(rates))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling fetchFiatRates", e)
        }
    }

    @ReactMethod
    fun listFiatCurrencies(promise: Promise) {
        try {
            val fiatCurrencies = getBreezServices().listFiatCurrencies()
            promise.resolve(readableArrayOf(fiatCurrencies))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling listFiatCurrencies", e)
        }
    }

    @ReactMethod
    fun listLsps(promise: Promise) {
        try {
            val lsps = getBreezServices().listLsps()
            promise.resolve(readableArrayOf(lsps))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling listLsps", e)
        }
    }

    @ReactMethod
    fun connectLsp(lspId: String, promise: Promise) {
        try {
            getBreezServices().connectLsp(lspId)
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling connectLsp", e)
        }
    }

    @ReactMethod
    fun fetchLspInfo(lspId: String, promise: Promise) {
        try {
            getBreezServices().fetchLspInfo(lspId)?.let {lspInformation->
                promise.resolve(readableMapOf(lspInformation))
            } ?: run {
                promise.reject(TAG, "No available lsp info")
            }
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling fetchLspInfo", e)
        }
    }

    @ReactMethod
    fun lspId(promise: Promise) {
        try {
            getBreezServices().lspId()?.let {lspId->
                promise.resolve(lspId)
            } ?: run {
                promise.reject(TAG, "No available lsp id")
            }
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling lspId", e)
        }
    }

    @ReactMethod
    fun closeLspChannels(promise: Promise) {
        try {
            getBreezServices().closeLspChannels()
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling closeLspChannels", e)
        }
    }

    @ReactMethod
    fun receiveOnchain(promise: Promise) {
        try {
            val swapInfo = getBreezServices().receiveOnchain()
            promise.resolve(readableMapOf(swapInfo))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling receiveOnchain", e)
        }
    }

    @ReactMethod
    fun inProgressSwap(promise: Promise) {
        try {
            getBreezServices().inProgressSwap()?.let {swapInfo->
                promise.resolve(readableMapOf(swapInfo))
            } ?: run {
                promise.reject(TAG, "No available in progress swap")
            }
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling inProgressSwap", e)
        }
    }

    @ReactMethod
    fun listRefundables(promise: Promise) {
        try {
            val swapInfos = getBreezServices().listRefundables()
            promise.resolve(readableArrayOf(swapInfos))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling listRefundables", e)
        }
    }

    @ReactMethod
    fun refund(swapAddress: String, toAddress: String, satPerVbyte: Double, promise: Promise) {
        try {
            val result = getBreezServices().refund(swapAddress, toAddress, satPerVbyte.toUInt())
            promise.resolve(result)
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling refund", e)
        }
    }

    @ReactMethod
    fun fetchReverseSwapFees(promise: Promise) {
        try {
            val reverseSwapFees = getBreezServices().fetchReverseSwapFees()
            promise.resolve(readableMapOf(reverseSwapFees))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling fetchReverseSwapFees", e)
        }
    }

    @ReactMethod
    fun inProgressReverseSwaps(promise: Promise) {
        try {
            val inProgressReverseSwaps = getBreezServices().inProgressReverseSwaps()
            promise.resolve(readableArrayOf(inProgressReverseSwaps))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling inProgressReverseSwaps", e)
        }
    }

    @ReactMethod
    fun sendOnchain(amountSat: Double, onchainRecipientAddress: String, pairHash: String, satPerVbyte: Double, promise: Promise) {
        try {
            val response = getBreezServices().sendOnchain(amountSat.toULong(), onchainRecipientAddress, pairHash, satPerVbyte.toULong())
            promise.resolve(readableMapOf(response))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling sendOnchain", e)
        }
    }

    @ReactMethod
    fun executeDevCommand(command: String, promise: Promise) {
        try {
            val result = getBreezServices().executeDevCommand(command)
            promise.resolve(result)
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling executeDevCommand", e)
        }
    }

    @ReactMethod
    fun recommendedFees(promise: Promise) {
        try {
            val fees = getBreezServices().recommendedFees()
            promise.resolve(readableMapOf(fees))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling recommendedFees", e)
        }
    }

    @ReactMethod
    fun buyBitcoin(provider: String, promise: Promise) {
        try {
            val buyBitcoinProvider = asBuyBitcoinProvider(provider)
            val result = getBreezServices().buyBitcoin(buyBitcoinProvider)
            promise.resolve(result)
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling buyBitcoin", e)
        }
    }

    @ReactMethod
    fun startBackup(promise: Promise) {
        try {
            getBreezServices().startBackup()
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling startBackup", e)
        }
    }

    @ReactMethod
    fun backupStatus(promise: Promise) {
        try {
            val status = getBreezServices().backupStatus()
            promise.resolve(readableMapOf(status))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(TAG, e.message ?: "Error calling backupStatus", e)
        }
    }
}
