package com.breezsdk

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
        const val GENERIC_CODE = "Generic"
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

        throw SdkException.Generic("BreezServices not initialized")
    }

    @ReactMethod
    fun addListener(eventName: String) {}

    @ReactMethod
    fun removeListeners(count: Int) {}

    @ReactMethod
    fun parseInvoice(
        invoice: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = parseInvoice(invoice)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun parseInput(
        s: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = parseInput(s)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun mnemonicToSeed(
        phrase: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = mnemonicToSeed(phrase)
                promise.resolve(readableArrayOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun defaultConfig(
        envType: String,
        apiKey: String,
        nodeConfig: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val envTypeTmp = asEnvironmentType(envType)
                val nodeConfigTmp = asNodeConfig(nodeConfig) ?: run { throw SdkException.Generic("Missing mandatory field nodeConfig of type NodeConfig") }
                val res = defaultConfig(envTypeTmp, apiKey, nodeConfigTmp)
                val workingDir = File(reactApplicationContext.filesDir.toString() + "/breezSdk")

                if (!workingDir.exists()) {
                    workingDir.mkdirs()
                }

                res.workingDir = workingDir.absolutePath
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun staticBackup(
        request: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val staticBackupRequest =
                    asStaticBackupRequest(request) ?: run {
                        throw SdkException.Generic("Missing mandatory field request of type StaticBackupRequest")
                    }
                val res = staticBackup(staticBackupRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun setLogStream(promise: Promise) {
        try {
            val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)

            setLogStream(BreezSDKLogStream(emitter))
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(e.javaClass.simpleName, e.message, e)
        }
    }

    @ReactMethod
    fun connect(
        config: ReadableMap,
        seed: ReadableArray,
        promise: Promise,
    ) {
        if (breezServices != null) {
            promise.reject(TAG, "BreezServices already initialized")
            return
        }

        try {
            val configTmp = asConfig(config) ?: run { throw SdkException.Generic("Missing mandatory field config of type Config") }
            val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)

            breezServices = connect(configTmp, asUByteList(seed), BreezSDKListener(emitter))
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(e.javaClass.simpleName, e.message, e)
        }
    }

    @ReactMethod
    fun disconnect(promise: Promise) {
        executor.execute {
            try {
                getBreezServices().disconnect()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun sendPayment(
        bolt11: String,
        amountMsat: Double,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val amountMsatTmp = amountMsat.toULong().takeUnless { it == 0UL }
                val res = getBreezServices().sendPayment(bolt11, amountMsatTmp)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun sendSpontaneousPayment(
        nodeId: String,
        amountSats: Double,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = getBreezServices().sendSpontaneousPayment(nodeId, amountSats.toULong())
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun receivePayment(
        reqData: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val receivePaymentRequest =
                    asReceivePaymentRequest(reqData) ?: run {
                        throw SdkException.Generic("Missing mandatory field reqData of type ReceivePaymentRequest")
                    }
                val res = getBreezServices().receivePayment(receivePaymentRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun payLnurl(
        reqData: ReadableMap,
        amountSats: Double,
        comment: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val lnUrlPayRequestData =
                    asLnUrlPayRequestData(reqData) ?: run {
                        throw SdkException.Generic("Missing mandatory field reqData of type LnUrlPayRequestData")
                    }
                val commentTmp = comment.takeUnless { it.isEmpty() }
                val res = getBreezServices().payLnurl(lnUrlPayRequestData, amountSats.toULong(), commentTmp)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun withdrawLnurl(
        reqData: ReadableMap,
        amountSats: Double,
        description: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val lnUrlWithdrawRequestData =
                    asLnUrlWithdrawRequestData(reqData) ?: run {
                        throw SdkException.Generic("Missing mandatory field reqData of type LnUrlWithdrawRequestData")
                    }
                val descriptionTmp = description.takeUnless { it.isEmpty() }
                val res = getBreezServices().withdrawLnurl(lnUrlWithdrawRequestData, amountSats.toULong(), descriptionTmp)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun lnurlAuth(
        reqData: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val lnUrlAuthRequestData =
                    asLnUrlAuthRequestData(reqData) ?: run {
                        throw SdkException.Generic("Missing mandatory field reqData of type LnUrlAuthRequestData")
                    }
                val res = getBreezServices().lnurlAuth(lnUrlAuthRequestData)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun nodeInfo(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().nodeInfo()
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun signMessage(
        request: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val signMessageRequest =
                    asSignMessageRequest(request) ?: run {
                        throw SdkException.Generic("Missing mandatory field request of type SignMessageRequest")
                    }
                val res = getBreezServices().signMessage(signMessageRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun checkMessage(
        request: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val checkMessageRequest =
                    asCheckMessageRequest(request) ?: run {
                        throw SdkException.Generic("Missing mandatory field request of type CheckMessageRequest")
                    }
                val res = getBreezServices().checkMessage(checkMessageRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun backupStatus(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().backupStatus()
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
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
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun paymentByHash(
        hash: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = getBreezServices().paymentByHash(hash)
                promise.resolve(res?.let { readableMapOf(res) })
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun listPayments(
        request: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val listPaymentsRequest =
                    asListPaymentsRequest(request) ?: run {
                        throw SdkException.Generic("Missing mandatory field request of type ListPaymentsRequest")
                    }
                val res = getBreezServices().listPayments(listPaymentsRequest)
                promise.resolve(readableArrayOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun sweep(
        request: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val sweepRequest = asSweepRequest(request) ?: run { throw SdkException.Generic("Missing mandatory field request of type SweepRequest") }
                val res = getBreezServices().sweep(sweepRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun fetchFiatRates(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().fetchFiatRates()
                promise.resolve(readableArrayOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun listFiatCurrencies(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().listFiatCurrencies()
                promise.resolve(readableArrayOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun listLsps(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().listLsps()
                promise.resolve(readableArrayOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun connectLsp(
        lspId: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                getBreezServices().connectLsp(lspId)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun fetchLspInfo(
        lspId: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = getBreezServices().fetchLspInfo(lspId)
                promise.resolve(res?.let { readableMapOf(res) })
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun openChannelFee(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val openChannelFeeRequest =
                    asOpenChannelFeeRequest(req) ?: run {
                        throw SdkException.Generic("Missing mandatory field req of type OpenChannelFeeRequest")
                    }
                val res = getBreezServices().openChannelFee(openChannelFeeRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun lspId(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().lspId()
                promise.resolve(res?.let { res })
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun lspInfo(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().lspInfo()
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
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
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun receiveOnchain(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val receiveOnchainRequest =
                    asReceiveOnchainRequest(req) ?: run {
                        throw SdkException.Generic("Missing mandatory field req of type ReceiveOnchainRequest")
                    }
                val res = getBreezServices().receiveOnchain(receiveOnchainRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun inProgressSwap(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().inProgressSwap()
                promise.resolve(res?.let { readableMapOf(res) })
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun listRefundables(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().listRefundables()
                promise.resolve(readableArrayOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun refund(
        swapAddress: String,
        toAddress: String,
        satPerVbyte: Int,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = getBreezServices().refund(swapAddress, toAddress, satPerVbyte.toUInt())
                promise.resolve(res)
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun fetchReverseSwapFees(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val reverseSwapFeesRequest =
                    asReverseSwapFeesRequest(req) ?: run {
                        throw SdkException.Generic("Missing mandatory field req of type ReverseSwapFeesRequest")
                    }
                val res = getBreezServices().fetchReverseSwapFees(reverseSwapFeesRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun inProgressReverseSwaps(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().inProgressReverseSwaps()
                promise.resolve(readableArrayOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun sendOnchain(
        amountSat: Double,
        onchainRecipientAddress: String,
        pairHash: String,
        satPerVbyte: Double,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = getBreezServices().sendOnchain(amountSat.toULong(), onchainRecipientAddress, pairHash, satPerVbyte.toULong())
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun executeDevCommand(
        command: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = getBreezServices().executeDevCommand(command)
                promise.resolve(res)
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
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
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun recommendedFees(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().recommendedFees()
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }

    @ReactMethod
    fun buyBitcoin(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val buyBitcoinRequest =
                    asBuyBitcoinRequest(req) ?: run {
                        throw SdkException.Generic("Missing mandatory field req of type BuyBitcoinRequest")
                    }
                val res = getBreezServices().buyBitcoin(buyBitcoinRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }
}
