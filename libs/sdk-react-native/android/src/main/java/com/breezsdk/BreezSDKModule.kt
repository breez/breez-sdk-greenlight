package com.breezsdk

import breez_sdk.*
import com.facebook.react.bridge.*
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter
import java.io.File
import java.util.*
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors

class BreezSDKModule(
    reactContext: ReactApplicationContext,
) : ReactContextBaseJavaModule(reactContext) {
    private lateinit var executor: ExecutorService
    private var breezServices: BlockingBreezServices? = null

    companion object {
        const val TAG = "RNBreezSDK"
    }

    override fun initialize() {
        super.initialize()

        executor = Executors.newFixedThreadPool(3)
    }

    override fun getName(): String = TAG

    @Throws(SdkException::class)
    fun getBreezServices(): BlockingBreezServices {
        if (breezServices != null) {
            return breezServices!!
        }

        throw SdkException.Generic("BreezServices not initialized")
    }

    @Throws(SdkException::class)
    private fun ensureWorkingDir(workingDir: String) {
        try {
            val workingDirFile = File(workingDir)

            if (!workingDirFile.exists() && !workingDirFile.mkdirs()) {
                throw SdkException.Generic("Mandatory field workingDir must contain a writable directory")
            }
        } catch (e: SecurityException) {
            throw SdkException.Generic("Mandatory field workingDir must contain a writable directory")
        }
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
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
<<<<<<< HEAD
                val nodeConfigTmp =
                    asNodeConfig(nodeConfig) ?: run { throw SdkException.Generic(errMissingMandatoryField("nodeConfig", "NodeConfig")) }
=======
                val nodeConfigTmp = asNodeConfig(nodeConfig) ?: run { throw SdkException.Generic(errMissingMandatoryField("nodeConfig", "NodeConfig")) }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = defaultConfig(envTypeTmp, apiKey, nodeConfigTmp)
                val workingDir = File(reactApplicationContext.filesDir.toString() + "/breezSdk")

                res.workingDir = workingDir.absolutePath
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun staticBackup(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val staticBackupRequest =
<<<<<<< HEAD
                    asStaticBackupRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "StaticBackupRequest")) }
=======
                    asStaticBackupRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "StaticBackupRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = staticBackup(staticBackupRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun serviceHealthCheck(
        apiKey: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = serviceHealthCheck(apiKey)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun setLogStream(promise: Promise) {
        executor.execute {
            try {
                val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)

                setLogStream(BreezSDKLogStream(emitter))
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                e.printStackTrace()
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun connect(
        req: ReadableMap,
        promise: Promise,
    ) {
        if (breezServices != null) {
            promise.reject("Generic", "BreezServices already initialized")
            return
        }

<<<<<<< HEAD
        executor.execute {
            try {
                val connectRequest =
                    asConnectRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "ConnectRequest")) }
                val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)

                ensureWorkingDir(connectRequest.config.workingDir)

                breezServices = connect(connectRequest, BreezSDKListener(emitter))
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                e.printStackTrace()
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
=======
        try {
            val configTmp = asConfig(config) ?: run { throw SdkException.Generic(errMissingMandatoryField("config", "Config")) }
            val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)

            ensureWorkingDir(configTmp.workingDir)

            breezServices = connect(configTmp, asUByteList(seed), BreezSDKListener(emitter))
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: Exception) {
            e.printStackTrace()
            promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
        }
    }

    @ReactMethod
    fun disconnect(promise: Promise) {
        executor.execute {
            try {
                getBreezServices().disconnect()
                breezServices = null
<<<<<<< HEAD
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun configureNode(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val configureNodeRequest =
                    asConfigureNodeRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "ConfigureNodeRequest")) }
                getBreezServices().configureNode(configureNodeRequest)
=======
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun sendPayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val sendPaymentRequest =
<<<<<<< HEAD
                    asSendPaymentRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "SendPaymentRequest")) }
=======
                    asSendPaymentRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "SendPaymentRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().sendPayment(sendPaymentRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun sendSpontaneousPayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val sendSpontaneousPaymentRequest =
<<<<<<< HEAD
                    asSendSpontaneousPaymentRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "SendSpontaneousPaymentRequest")) }
=======
                    asSendSpontaneousPaymentRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "SendSpontaneousPaymentRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().sendSpontaneousPayment(sendSpontaneousPaymentRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun receivePayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val receivePaymentRequest =
<<<<<<< HEAD
                    asReceivePaymentRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "ReceivePaymentRequest")) }
=======
                    asReceivePaymentRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "ReceivePaymentRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().receivePayment(receivePaymentRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun payLnurl(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
<<<<<<< HEAD
                val lnUrlPayRequest =
                    asLnUrlPayRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "LnUrlPayRequest")) }
=======
                val lnUrlPayRequest = asLnUrlPayRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "LnUrlPayRequest")) }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().payLnurl(lnUrlPayRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun withdrawLnurl(
        request: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val lnUrlWithdrawRequest =
<<<<<<< HEAD
                    asLnUrlWithdrawRequest(request)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("request", "LnUrlWithdrawRequest")) }
=======
                    asLnUrlWithdrawRequest(request) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("request", "LnUrlWithdrawRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().withdrawLnurl(lnUrlWithdrawRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
<<<<<<< HEAD
                    asLnUrlAuthRequestData(reqData)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("reqData", "LnUrlAuthRequestData")) }
=======
                    asLnUrlAuthRequestData(reqData) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("reqData", "LnUrlAuthRequestData"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().lnurlAuth(lnUrlAuthRequestData)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun reportIssue(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val reqTmp =
                    asReportIssueRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "ReportIssueRequest")) }
                getBreezServices().reportIssue(reqTmp)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun nodeCredentials(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().nodeCredentials()
                promise.resolve(res?.let { readableMapOf(res) })
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun nodeInfo(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().nodeInfo()
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun signMessage(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val signMessageRequest =
<<<<<<< HEAD
                    asSignMessageRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "SignMessageRequest")) }
=======
                    asSignMessageRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "SignMessageRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().signMessage(signMessageRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun checkMessage(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val checkMessageRequest =
<<<<<<< HEAD
                    asCheckMessageRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "CheckMessageRequest")) }
=======
                    asCheckMessageRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "CheckMessageRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().checkMessage(checkMessageRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun backupStatus(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().backupStatus()
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun backup(promise: Promise) {
        executor.execute {
            try {
                getBreezServices().backup()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun listPayments(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val listPaymentsRequest =
                    asListPaymentsRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "ListPaymentsRequest")) }
                val res = getBreezServices().listPayments(listPaymentsRequest)
                promise.resolve(readableArrayOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun setPaymentMetadata(
        hash: String,
        metadata: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
<<<<<<< HEAD
                getBreezServices().setPaymentMetadata(hash, metadata)
                promise.resolve(readableMapOf("status" to "ok"))
=======
                val listPaymentsRequest =
                    asListPaymentsRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "ListPaymentsRequest"))
                    }
                val res = getBreezServices().listPayments(listPaymentsRequest)
                promise.resolve(readableArrayOf(res))
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun redeemOnchainFunds(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
<<<<<<< HEAD
                val redeemOnchainFundsRequest =
                    asRedeemOnchainFundsRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "RedeemOnchainFundsRequest")) }
                val res = getBreezServices().redeemOnchainFunds(redeemOnchainFundsRequest)
=======
                val sweepRequest = asSweepRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "SweepRequest")) }
                val res = getBreezServices().sweep(sweepRequest)
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun fetchFiatRates(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().fetchFiatRates()
                promise.resolve(readableArrayOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun listFiatCurrencies(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().listFiatCurrencies()
                promise.resolve(readableArrayOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun listLsps(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().listLsps()
                promise.resolve(readableArrayOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
<<<<<<< HEAD
                    asOpenChannelFeeRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "OpenChannelFeeRequest")) }
=======
                    asOpenChannelFeeRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "OpenChannelFeeRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().openChannelFee(openChannelFeeRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun lspId(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().lspId()
                promise.resolve(res?.let { res })
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun lspInfo(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().lspInfo()
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun closeLspChannels(promise: Promise) {
        executor.execute {
            try {
                getBreezServices().closeLspChannels()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun registerWebhook(
        webhookUrl: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                getBreezServices().registerWebhook(webhookUrl)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun unregisterWebhook(
        webhookUrl: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                getBreezServices().unregisterWebhook(webhookUrl)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
<<<<<<< HEAD
                    asReceiveOnchainRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "ReceiveOnchainRequest")) }
=======
                    asReceiveOnchainRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "ReceiveOnchainRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().receiveOnchain(receiveOnchainRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun inProgressSwap(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().inProgressSwap()
                promise.resolve(res?.let { readableMapOf(res) })
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun rescanSwaps(promise: Promise) {
        executor.execute {
            try {
                getBreezServices().rescanSwaps()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun redeemSwap(
        swapAddress: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                getBreezServices().redeemSwap(swapAddress)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun listRefundables(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().listRefundables()
                promise.resolve(readableArrayOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun prepareRefund(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareRefundRequest =
<<<<<<< HEAD
                    asPrepareRefundRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PrepareRefundRequest")) }
=======
                    asPrepareRefundRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "PrepareRefundRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().prepareRefund(prepareRefundRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun refund(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
<<<<<<< HEAD
                val refundRequest =
                    asRefundRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "RefundRequest")) }
=======
                val refundRequest = asRefundRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "RefundRequest")) }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().refund(refundRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
<<<<<<< HEAD
                    asReverseSwapFeesRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "ReverseSwapFeesRequest")) }
=======
                    asReverseSwapFeesRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "ReverseSwapFeesRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().fetchReverseSwapFees(reverseSwapFeesRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun onchainPaymentLimits(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().onchainPaymentLimits()
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun prepareOnchainPayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareOnchainPaymentRequest =
                    asPrepareOnchainPaymentRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PrepareOnchainPaymentRequest")) }
                val res = getBreezServices().prepareOnchainPayment(prepareOnchainPaymentRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun inProgressOnchainPayments(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().inProgressOnchainPayments()
                promise.resolve(readableArrayOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun inProgressReverseSwaps(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().inProgressReverseSwaps()
                promise.resolve(readableArrayOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun maxReverseSwapAmount(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().maxReverseSwapAmount()
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun sendOnchain(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val sendOnchainRequest =
<<<<<<< HEAD
                    asSendOnchainRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "SendOnchainRequest")) }
=======
                    asSendOnchainRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "SendOnchainRequest"))
                    }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().sendOnchain(sendOnchainRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun payOnchain(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val payOnchainRequest =
                    asPayOnchainRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PayOnchainRequest")) }
                val res = getBreezServices().payOnchain(payOnchainRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun generateDiagnosticData(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().generateDiagnosticData()
                promise.resolve(res)
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun sync(promise: Promise) {
        executor.execute {
            try {
                getBreezServices().sync()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun recommendedFees(promise: Promise) {
        executor.execute {
            try {
                val res = getBreezServices().recommendedFees()
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
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
<<<<<<< HEAD
                val buyBitcoinRequest =
                    asBuyBitcoinRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "BuyBitcoinRequest")) }
=======
                val buyBitcoinRequest = asBuyBitcoinRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "BuyBitcoinRequest")) }
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                val res = getBreezServices().buyBitcoin(buyBitcoinRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun prepareRedeemOnchainFunds(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
<<<<<<< HEAD
                val prepareRedeemOnchainFundsRequest =
                    asPrepareRedeemOnchainFundsRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PrepareRedeemOnchainFundsRequest")) }
                val res = getBreezServices().prepareRedeemOnchainFunds(prepareRedeemOnchainFundsRequest)
=======
                val prepareSweepRequest =
                    asPrepareSweepRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "PrepareSweepRequest"))
                    }
                val res = getBreezServices().prepareSweep(prepareSweepRequest)
>>>>>>> 76cb54aa (Squash and rebase Bolt12 implementation)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun createOffer(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val createOfferRequest =
                    asCreateOfferRequest(req) ?: run {
                        throw SdkException.Generic(errMissingMandatoryField("req", "CreateOfferRequest"))
                    }
                val res = getBreezServices().createOffer(createOfferRequest)
                promise.resolve(res)
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun payOffer(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val payOfferRequest = asPayOfferRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PayOfferRequest")) }
                val res = getBreezServices().payOffer(payOfferRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }
}
