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

    companion object {
        const val TAG = "RNBreezSDK"
    }

    override fun initialize() {
        super.initialize()

        executor = Executors.newFixedThreadPool(3)
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
        executor.execute {
            try {
                val seed = mnemonicToSeed(mnemonic)
                promise.resolve(readableArrayOf(seed))
            } catch (e: SdkException) {
                promise.reject(TAG, e.message ?: "Error calling mnemonicToSeed", e)
            }
        }
    }

    @ReactMethod
    fun parseInput(input: String, promise: Promise) {
        executor.execute {
            try {
                val inputType = parseInput(input)
                promise.resolve(readableMapOf(inputType))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling parseInput", e)
            }
        }
    }

    @ReactMethod
    fun parseInvoice(invoice: String, promise: Promise) {
        executor.execute {
            try {
                val lnInvoice = parseInvoice(invoice)
                promise.resolve(readableMapOf(lnInvoice))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling parseInvoice", e)
            }
        }
    }

    @ReactMethod
    fun registerNode(network: String, seed: ReadableArray, registerCredentials: ReadableMap, inviteCode: String, promise: Promise) {
        executor.execute {
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
    }

    @ReactMethod
    fun recoverNode(network: String, seed: ReadableArray, promise: Promise) {
        executor.execute {
            try {
                val creds = recoverNode(asNetwork(network), asUByteList(seed))
                promise.resolve(readableMapOf(creds))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling recoverNode", e)
            }
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
        executor.execute {
            try {
                getBreezServices().start()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling initServices", e)
            }
        }
    }

    @ReactMethod
    fun sync(promise: Promise) {
        executor.execute {
            try {
                getBreezServices().sync()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling sync", e)
            }
        }
    }

    @ReactMethod
    fun stop(promise: Promise) {
        executor.execute {
            try {
                getBreezServices().stop()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling stop", e)
            }
        }
    }

    @ReactMethod
    fun sendPayment(bolt11: String, amountSats: Double, promise: Promise) {
        executor.execute {
            try {
                val optionalAmountSats = amountSats.takeUnless { it == 0.0 }
                val payment = getBreezServices().sendPayment(bolt11, optionalAmountSats?.toULong())
                promise.resolve(readableMapOf(payment))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling sendPayment", e)
            }
        }
    }

    @ReactMethod
    fun sendSpontaneousPayment(nodeId: String, amountSats: Double, promise: Promise) {
        executor.execute {
            try {
                val payment = getBreezServices().sendSpontaneousPayment(nodeId, amountSats.toULong())
                promise.resolve(readableMapOf(payment))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling sendSpontaneousPayment", e)
            }
        }
    }

    @ReactMethod
    fun receivePayment(amountSats: Double, description: String, promise: Promise) {
        executor.execute {
            try {
                val payment = getBreezServices().receivePayment(amountSats.toULong(), description)
                promise.resolve(readableMapOf(payment))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling receivePayment", e)
            }
        }
    }

    @ReactMethod
    fun lnurlAuth(reqData: ReadableMap, promise: Promise) {
        executor.execute {
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
    }

    @ReactMethod
    fun payLnurl(reqData: ReadableMap, amountSats: Double, comment: String, promise: Promise) {
        executor.execute {
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
    }

    @ReactMethod
    fun withdrawLnurl(reqData: ReadableMap, amountSats: Double, description: String, promise: Promise) {
        executor.execute {
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
    }

    @ReactMethod
    fun nodeInfo(promise: Promise) {
        executor.execute {
            try {
                getBreezServices().nodeInfo()?.let {nodeState->
                    promise.resolve(readableMapOf(nodeState))
                } ?: run {
                    promise.reject(TAG, "No available node info")
                }
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling nodeInfo", e)
            }
        }
    }

    @ReactMethod
    fun listPayments(filter: String, fromTimestamp: Double, toTimestamp: Double, promise: Promise) {
        executor.execute {
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
    }

    @ReactMethod
    fun sweep(toAddress: String, feeRateSatsPerVbyte: Double, promise: Promise) {
        executor.execute {
            try {
                getBreezServices().sweep(toAddress, feeRateSatsPerVbyte.toULong())
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling sweep", e)
            }
        }
    }

    @ReactMethod
    fun fetchFiatRates(promise: Promise) {
        executor.execute {
            try {
                val rates = getBreezServices().fetchFiatRates()
                promise.resolve(readableArrayOf(rates))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling fetchFiatRates", e)
            }
        }
    }

    @ReactMethod
    fun listFiatCurrencies(promise: Promise) {
        executor.execute {
            try {
                val fiatCurrencies = getBreezServices().listFiatCurrencies()
                promise.resolve(readableArrayOf(fiatCurrencies))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling listFiatCurrencies", e)
            }
        }
    }

    @ReactMethod
    fun listLsps(promise: Promise) {
        executor.execute {
            try {
                val lsps = getBreezServices().listLsps()
                promise.resolve(readableArrayOf(lsps))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling listLsps", e)
            }
        }
    }

    @ReactMethod
    fun connectLsp(lspId: String, promise: Promise) {
        executor.execute {
            try {
                getBreezServices().connectLsp(lspId)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling connectLsp", e)
            }
        }
    }

    @ReactMethod
    fun fetchLspInfo(lspId: String, promise: Promise) {
        executor.execute {
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
    }

    @ReactMethod
    fun lspId(promise: Promise) {
        executor.execute {
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
    }

    @ReactMethod
    fun closeLspChannels(promise: Promise) {
        executor.execute {
            try {
                getBreezServices().closeLspChannels()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling closeLspChannels", e)
            }
        }
    }

    @ReactMethod
    fun receiveOnchain(promise: Promise) {
        executor.execute {
            try {
                val swapInfo = getBreezServices().receiveOnchain()
                promise.resolve(readableMapOf(swapInfo))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling receiveOnchain", e)
            }
        }
    }

    @ReactMethod
    fun inProgressSwap(promise: Promise) {
        executor.execute {
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
    }

    @ReactMethod
    fun listRefundables(promise: Promise) {
        executor.execute {
            try {
                val swapInfos = getBreezServices().listRefundables()
                promise.resolve(readableArrayOf(swapInfos))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling listRefundables", e)
            }
        }
    }

    @ReactMethod
    fun refund(swapAddress: String, toAddress: String, satPerVbyte: Double, promise: Promise) {
        executor.execute {
            try {
                val result = getBreezServices().refund(swapAddress, toAddress, satPerVbyte.toUInt())
                promise.resolve(result)
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling refund", e)
            }
        }
    }

    @ReactMethod
    fun fetchReverseSwapFees(promise: Promise) {
        executor.execute {
            try {
                val reverseSwapFees = getBreezServices().fetchReverseSwapFees()
                promise.resolve(readableMapOf(reverseSwapFees))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling fetchReverseSwapFees", e)
            }
        }
    }

    @ReactMethod
    fun inProgressReverseSwaps(promise: Promise) {
        executor.execute {
            try {
                val inProgressReverseSwaps = getBreezServices().inProgressReverseSwaps()
                promise.resolve(readableArrayOf(inProgressReverseSwaps))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling inProgressReverseSwaps", e)
            }
        }
    }

    @ReactMethod
    fun sendOnchain(amountSat: Double, onchainRecipientAddress: String, pairHash: String, satPerVbyte: Double, promise: Promise) {
        executor.execute {
            try {
                val response = getBreezServices().sendOnchain(amountSat.toULong(), onchainRecipientAddress, pairHash, satPerVbyte.toULong())
                promise.resolve(readableMapOf(response))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling sendOnchain", e)
            }
        }
    }

    @ReactMethod
    fun executeDevCommand(command: String, promise: Promise) {
        executor.execute {
            try {
                val result = getBreezServices().executeDevCommand(command)
                promise.resolve(result)
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling executeDevCommand", e)
            }
        }
    }

    @ReactMethod
    fun recommendedFees(promise: Promise) {
        executor.execute {
            try {
                val fees = getBreezServices().recommendedFees()
                promise.resolve(readableMapOf(fees))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling recommendedFees", e)
            }
        }
    }

    @ReactMethod
    fun buyBitcoin(provider: String, promise: Promise) {
        executor.execute {
            try {
                val buyBitcoinProvider = asBuyBitcoinProvider(provider)
                val result = getBreezServices().buyBitcoin(buyBitcoinProvider)
                promise.resolve(result)
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling buyBitcoin", e)
            }
        }
    }

    @ReactMethod
    fun backup(promise: Promise) {
        executor.execute {
            try {
                getBreezServices().backup()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling backup", e)
            }
        }
    }

    @ReactMethod
    fun backupStatus(promise: Promise) {
        executor.execute {
            try {
                val status = getBreezServices().backupStatus()
                promise.resolve(readableMapOf(status))
            } catch (e: SdkException) {
                e.printStackTrace()
                promise.reject(TAG, e.message ?: "Error calling backupStatus", e)
            }
        }
    }
}
