package com.breezsdk

import breez_sdk.*
import com.facebook.react.bridge.*

fun asConfig(config: ReadableMap): Config? {
    val breezserver = config.getString("breezserver")
    val mempoolspaceUrl = config.getString("mempoolspaceUrl")
    val workingDir = config.getString("workingDir")
    val network = config.getString("network")

    if (breezserver != null && mempoolspaceUrl != null && workingDir != null && network != null && hasNonNullKey(config, "paymentTimeoutSec") && hasNonNullKey(config, "maxfeePercent")) {
        val paymentTimeoutSec = config.getInt("paymentTimeoutSec")
        val defaultLspId = config.getString("defaultLspId")
        val apiKey = config.getString("apiKey")
        val maxfeePercent = config.getDouble("maxfeePercent")

        return Config(breezserver, mempoolspaceUrl, workingDir, asNetwork(network), paymentTimeoutSec.toUInt(), defaultLspId, apiKey, maxfeePercent)
    }

    return null
}

fun asEnvironmentType(envType: String): EnvironmentType {
    return EnvironmentType.valueOf(envType.uppercase())
}

fun asBuyBitcoinProvider(envType: String): BuyBitcoinProvider {
    return BuyBitcoinProvider.valueOf(envType.uppercase())
}

fun asLnUrlAuthRequestData(reqData: ReadableMap): LnUrlAuthRequestData? {
    val k1 = reqData.getString("k1")
    val action = reqData.getString("action")
    val domain = reqData.getString("domain")
    val url = reqData.getString("url")

    if (k1 != null && domain != null && url != null) {
        return LnUrlAuthRequestData(k1, action, domain, url)
    }

    return null
}

fun asLnUrlPayRequestData(reqData: ReadableMap): LnUrlPayRequestData? {
    val callback = reqData.getString("callback")
    val metadataStr = reqData.getString("metadataStr")
    val domain = reqData.getString("domain")
    val lnAddress = reqData.getString("lnAddress")

    if (callback != null && metadataStr != null && domain != null && hasNonNullKey(reqData, "minSendable") && hasNonNullKey(reqData, "minSendable") && hasNonNullKey(reqData, "commentAllowed")) {
        val minSendable = reqData.getDouble("minSendable")
        val maxSendable = reqData.getDouble("maxSendable")
        val commentAllowed = reqData.getInt("commentAllowed")

        return LnUrlPayRequestData(callback, minSendable.toULong(), maxSendable.toULong(), metadataStr, commentAllowed.toUShort(), domain, lnAddress)
    }

    return null
}

fun asLnUrlWithdrawRequestData(reqData: ReadableMap): LnUrlWithdrawRequestData? {
    val callback = reqData.getString("callback")
    val k1 = reqData.getString("k1")
    val defaultDescription = reqData.getString("defaultDescription")

    if (callback != null && k1 != null && defaultDescription != null && hasNonNullKey(reqData, "minWithdrawable") && hasNonNullKey(reqData, "maxWithdrawable")) {
        val minWithdrawable = reqData.getDouble("minWithdrawable")
        val maxWithdrawable = reqData.getDouble("maxWithdrawable")

        return LnUrlWithdrawRequestData(callback, k1, defaultDescription, minWithdrawable.toULong(), maxWithdrawable.toULong())
    }

    return null
}

fun asPaymentTypeFilter(filter: String): PaymentTypeFilter {
    return PaymentTypeFilter.valueOf(filter.uppercase())
}

fun asNetwork(network: String): Network {
    return Network.valueOf(network.uppercase())
}

fun asGreenlightCredentials(creds: ReadableMap) : GreenlightCredentials? {
    if (hasNonNullKey(creds, "deviceKey") && hasNonNullKey(creds, "deviceCert")) {
         val deviceKeyArray = creds.getArray("deviceKey")
         val deviceCertArray = creds.getArray("deviceCert")
         return GreenlightCredentials(asUByteList(deviceKeyArray!!), asUByteList(deviceCertArray!!))
    }

    return null
}

fun asUByteList(arr: ReadableArray): List<UByte> {
    val list = ArrayList<UByte>()
    for (value in arr.toArrayList()) {
        when (value) {
            is Double -> list.add(value.toInt().toUByte())
            is Int -> list.add(value.toUByte())
            is UByte -> list.add(value)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }

    return list
}

fun hasNonNullKey(map: ReadableMap, key: String): Boolean {
    return map.hasKey(key) && !map.isNull(key)
}

fun pushToArray(array: WritableArray, value: Any?) {
    when (value) {
        null -> array.pushNull()
        is Boolean -> array.pushBoolean(value)
        is Double -> array.pushDouble(value)
        is FiatCurrency -> array.pushMap(readableMapOf(value))
        is Int -> array.pushInt(value)
        is LocaleOverrides -> array.pushMap(readableMapOf(value))
        is LocalizedName -> array.pushMap(readableMapOf(value))
        is LspInformation -> array.pushMap(readableMapOf(value))
        is Payment -> array.pushMap(readableMapOf(value))
        is Rate -> array.pushMap(readableMapOf(value))
        is ReadableArray -> array.pushArray(value)
        is ReadableMap -> array.pushMap(value)
        is ReverseSwapInfo -> array.pushMap(readableMapOf(value))
        is ReverseSwapPairInfo -> array.pushMap(readableMapOf(value))
        is RouteHint -> array.pushMap(readableMapOf(value))
        is RouteHintHop -> array.pushMap(readableMapOf(value))
        is String -> array.pushString(value)
        is SwapInfo -> array.pushMap(readableMapOf(value))
        is UByte -> array.pushInt(value.toInt())
        is UInt -> array.pushInt(value.toInt())
        is UShort -> array.pushInt(value.toInt())
        is ULong -> array.pushDouble(value.toDouble())
        is UnspentTransactionOutput -> array.pushMap(readableMapOf(value))
        is Array<*> -> array.pushArray(readableArrayOf(value.asIterable()))
        is List<*> -> array.pushArray(readableArrayOf(value))
        else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
    }
}

fun pushToMap(map: WritableMap, key: String, value: Any?) {
    when (value) {
        null -> map.putNull(key)
        is Boolean -> map.putBoolean(key, value)
        is Byte -> map.putInt(key, value.toInt())
        is Double -> map.putDouble(key, value)
        is Int -> map.putInt(key, value)
        is Long -> map.putDouble(key, value.toDouble())
        is ReadableArray -> map.putArray(key, value)
        is ReadableMap -> map.putMap(key, value)
        is String -> map.putString(key, value)
        is UByte -> map.putInt(key, value.toInt())
        is UInt -> map.putInt(key, value.toInt())
        is UShort -> map.putInt(key, value.toInt())
        is ULong -> map.putDouble(key, value.toDouble())
        is Array<*> -> map.putArray(key, readableArrayOf(value.asIterable()))
        is List<*> -> map.putArray(key, readableArrayOf(value))
        else -> throw IllegalArgumentException("Unsupported value type ${value::class.java.name} for key [$key]")
    }
}

fun readableArrayOf(values: Iterable<*>?): ReadableArray {
    val array = Arguments.createArray()
    if (values != null) {
        for (value in values) {
            pushToArray(array, value)
        }
    }

    return array
}

fun readableMapOf(aesSuccessActionDataDecrypted: AesSuccessActionDataDecrypted): ReadableMap {
    return readableMapOf(
            "type" to "aes",
            "description" to aesSuccessActionDataDecrypted.description,
            "plaintext" to aesSuccessActionDataDecrypted.plaintext
    )
}

fun readableMapOf(bitcoinAddressData: BitcoinAddressData): ReadableMap {
    return readableMapOf(
            "address" to bitcoinAddressData.address,
            "network" to bitcoinAddressData.network.name.lowercase(),
            "amountSat" to bitcoinAddressData.amountSat,
            "label" to bitcoinAddressData.label,
            "message" to bitcoinAddressData.message
    )
}

fun readableMapOf(closedChannelPaymentDetails: ClosedChannelPaymentDetails): ReadableMap {
    return readableMapOf(
            "type" to "closed_channel",
            "shortChannelId" to closedChannelPaymentDetails.shortChannelId,
            "state" to closedChannelPaymentDetails.state.name.lowercase(),
            "fundingTxid" to closedChannelPaymentDetails.fundingTxid
    )
}

fun readableMapOf(config: Config): ReadableMap {
    return readableMapOf(
            "breezserver" to config.breezserver,
            "mempoolspaceUrl" to config.mempoolspaceUrl,
            "workingDir" to config.workingDir,
            "network" to config.network.name.lowercase(),
            "paymentTimeoutSec" to config.paymentTimeoutSec,
            "defaultLspId" to config.defaultLspId,
            "apiKey" to config.apiKey,            
            "maxfeePercent" to config.maxfeePercent
    )
}

fun readableMapOf(currencyInfo: CurrencyInfo): ReadableMap {
    return readableMapOf(
            "name" to currencyInfo.name,
            "fractionSize" to currencyInfo.fractionSize,
            "spacing" to currencyInfo.spacing,
            "symbol" to readableMapOf(currencyInfo.symbol),
            "uniqSymbol" to readableMapOf(currencyInfo.uniqSymbol),
            "localizedName" to readableArrayOf(currencyInfo.localizedName),
            "localeOverrides" to readableArrayOf(currencyInfo.localeOverrides)
    )
}

fun readableMapOf(fiatCurrency: FiatCurrency): ReadableMap {
    return readableMapOf(
            "id" to fiatCurrency.id,
            "info" to readableMapOf(fiatCurrency.info)
    )
}

fun readableMapOf(greenlightCredentials: GreenlightCredentials): ReadableMap {
    return readableMapOf(
            "deviceKey" to readableArrayOf(greenlightCredentials.deviceKey),
            "deviceCert" to readableArrayOf(greenlightCredentials.deviceCert)
    )
}

fun readableMapOf(inputType: InputType): ReadableMap {
    return when (inputType) {
        is InputType.BitcoinAddress -> readableMapOf("type" to "bitcoinAddress", "data" to readableMapOf(inputType.address))
        is InputType.Bolt11 -> readableMapOf("type" to "bolt11", "data" to readableMapOf(inputType.invoice))
        is InputType.LnUrlAuth -> readableMapOf("type" to "lnUrlAuth", "data" to readableMapOf(inputType.data))
        is InputType.LnUrlError -> readableMapOf("type" to "lnUrlError", "data" to readableMapOf(inputType.data))
        is InputType.LnUrlPay -> readableMapOf("type" to "lnUrlPay", "data" to readableMapOf(inputType.data))
        is InputType.LnUrlWithdraw -> readableMapOf("type" to "lnUrlWithdraw", "data" to readableMapOf(inputType.data))
        is InputType.NodeId -> readableMapOf("type" to "nodeId", "data" to inputType.nodeId)
        is InputType.Url -> readableMapOf("type" to "url", "data" to inputType.url)
    }
}

fun readableMapOf(invoicePaidDetails: InvoicePaidDetails): ReadableMap {
    return readableMapOf(
            "paymentHash" to invoicePaidDetails.paymentHash,
            "bolt11" to invoicePaidDetails.bolt11
    )
}

fun readableMapOf(backupStatus: BackupStatus): ReadableMap {
    return readableMapOf(
            "backedUp" to backupStatus.backedUp,
            "lastBackupTime" to backupStatus.lastBackupTime
    )
}

fun readableMapOf(backupFailedData: BackupFailedData): ReadableMap {
    return readableMapOf(
            "error" to backupFailedData.error
    )
}

fun readableMapOf(paymentFailedData: PaymentFailedData): ReadableMap {
    return readableMapOf(
            "error" to paymentFailedData.error,
            "bolt11" to if (paymentFailedData.invoice == null) null else readableMapOf(paymentFailedData.invoice!!),
            "nodeId" to paymentFailedData.nodeId
    )
}

fun readableMapOf(lnInvoice: LnInvoice): ReadableMap {
    return readableMapOf(
            "bolt11" to lnInvoice.bolt11,
            "payeePubkey" to lnInvoice.payeePubkey,
            "paymentHash" to lnInvoice.paymentHash,
            "description" to lnInvoice.description,
            "descriptionHash" to lnInvoice.descriptionHash,
            "amountMsat" to lnInvoice.amountMsat,
            "timestamp" to lnInvoice.timestamp,
            "expiry" to lnInvoice.expiry,
            "routingHints" to readableArrayOf(lnInvoice.routingHints),
            "paymentSecret" to readableArrayOf(lnInvoice.paymentSecret)
    )
}

fun readableMapOf(lnPaymentDetails: LnPaymentDetails): ReadableMap {
    return readableMapOf(
            "type" to "ln",
            "paymentHash" to lnPaymentDetails.paymentHash,
            "label" to lnPaymentDetails.label,
            "destinationPubkey" to lnPaymentDetails.destinationPubkey,
            "paymentPreimage" to lnPaymentDetails.paymentPreimage,
            "keysend" to lnPaymentDetails.keysend,
            "bolt11" to lnPaymentDetails.bolt11,
            "lnurlSuccessAction" to readableMapOf(lnPaymentDetails.lnurlSuccessAction),
            "lnurlMetadata" to lnPaymentDetails.lnurlMetadata,
            "lnAddress" to lnPaymentDetails.lnAddress
    )
}

fun readableMapOf(lnUrlAuthRequestData: LnUrlAuthRequestData): ReadableMap {
    return readableMapOf(
            "k1" to lnUrlAuthRequestData.k1,
            "action" to lnUrlAuthRequestData.action,
            "domain" to lnUrlAuthRequestData.domain,
            "url" to lnUrlAuthRequestData.url
    )
}

fun readableMapOf(lnUrlErrorData: LnUrlErrorData): ReadableMap {
    return readableMapOf("reason" to lnUrlErrorData.reason)
}

fun readableMapOf(lnUrlCallbackStatus: LnUrlCallbackStatus): ReadableMap {
    return when (lnUrlCallbackStatus) {
        is LnUrlCallbackStatus.Ok -> readableMapOf("status" to "ok")
        is LnUrlCallbackStatus.ErrorStatus -> {
            val response = Arguments.createMap()
            response.putString("status", "error")
            response.merge(readableMapOf(lnUrlCallbackStatus.data))
            response
        }
    }
}

fun readableMapOf(lnUrlPayRequestData: LnUrlPayRequestData): ReadableMap {
    return readableMapOf(
            "callback" to lnUrlPayRequestData.callback,
            "minSendable" to lnUrlPayRequestData.minSendable,
            "maxSendable" to lnUrlPayRequestData.maxSendable,
            "metadataStr" to lnUrlPayRequestData.metadataStr,
            "commentAllowed" to lnUrlPayRequestData.commentAllowed,
            "domain" to lnUrlPayRequestData.domain,
            "lnAddress" to lnUrlPayRequestData.lnAddress
    )
}

fun readableMapOf(lnUrlPayResult: LnUrlPayResult): ReadableMap {
    return when (lnUrlPayResult) {
        is LnUrlPayResult.EndpointSuccess -> {
            readableMapOf(
                    "type" to "endpointSuccess",
                    "data" to readableMapOf(lnUrlPayResult.data)
            )
        }
        is LnUrlPayResult.EndpointError -> {
            readableMapOf(
                    "type" to "endpointError",
                    "data" to readableMapOf(lnUrlPayResult.data)
            )
        }
    }
}

fun readableMapOf(lnUrlWithdrawRequestData: LnUrlWithdrawRequestData): ReadableMap {
    return readableMapOf(
            "callback" to lnUrlWithdrawRequestData.callback,
            "k1" to lnUrlWithdrawRequestData.k1,
            "defaultDescription" to lnUrlWithdrawRequestData.defaultDescription,
            "minWithdrawable" to lnUrlWithdrawRequestData.minWithdrawable,
            "maxWithdrawable" to lnUrlWithdrawRequestData.maxWithdrawable
    )
}

fun readableMapOf(localeOverride: LocaleOverrides): ReadableMap {
    return readableMapOf(
            "locale" to localeOverride.locale,
            "spacing" to localeOverride.spacing,
            "symbol" to readableMapOf(localeOverride.symbol)
    )
}

fun readableMapOf(localizedName: LocalizedName): ReadableMap {
    return readableMapOf(
            "name" to localizedName.name,
            "locale" to localizedName.locale
    )
}

fun readableMapOf(lspInformation: LspInformation): ReadableMap {
    return readableMapOf(
            "id" to lspInformation.id,
            "name" to lspInformation.name,
            "widgetUrl" to lspInformation.widgetUrl,
            "pubkey" to lspInformation.pubkey,
            "host" to lspInformation.host,
            "channelCapacity" to lspInformation.channelCapacity,
            "targetConf" to lspInformation.targetConf,
            "baseFeeMsat" to lspInformation.baseFeeMsat,
            "feeRate" to lspInformation.feeRate,
            "timeLockDelta" to lspInformation.timeLockDelta,
            "minHtlcMsat" to lspInformation.minHtlcMsat,
            "channelFeePermyriad" to lspInformation.channelFeePermyriad,
            "lspPubkey" to lspInformation.lspPubkey,
            "maxInactiveDuration" to lspInformation.maxInactiveDuration,
            "channelMinimumFeeMsat" to lspInformation.channelMinimumFeeMsat
    )
}

fun readableMapOf(messageSuccessActionData: MessageSuccessActionData): ReadableMap {
    return readableMapOf(
            "type" to "message",
            "message" to messageSuccessActionData.message
    )
}

fun readableMapOf(nodeState: NodeState): ReadableMap {
    return readableMapOf(
            "id" to nodeState.id,
            "blockHeight" to nodeState.blockHeight,
            "channelsBalanceMsat" to nodeState.channelsBalanceMsat,
            "onchainBalanceMsat" to nodeState.onchainBalanceMsat,
            "utxos" to readableArrayOf(nodeState.utxos),
            "maxPayableMsat" to nodeState.maxPayableMsat,
            "maxReceivableMsat" to nodeState.maxReceivableMsat,
            "maxSinglePaymentAmountMsat" to nodeState.maxSinglePaymentAmountMsat,
            "maxChanReserveMsats" to nodeState.maxChanReserveMsats,
            "connectedPeers" to readableArrayOf(nodeState.connectedPeers),
            "inboundLiquidityMsats" to nodeState.inboundLiquidityMsats
    )
}

fun readableMapOf(payment: Payment): ReadableMap {
    return readableMapOf(
            "id" to payment.id,
            "paymentType" to payment.paymentType.name.lowercase(),
            "paymentTime" to payment.paymentTime,
            "amountMsat" to payment.amountMsat,
            "feeMsat" to payment.feeMsat,
            "pending" to payment.pending,
            "description" to payment.description,
            "details" to readableMapOf(payment.details)
    )
}

fun readableMapOf(paymentDetails: PaymentDetails): ReadableMap {
    return when (paymentDetails) {
        is PaymentDetails.Ln -> readableMapOf(paymentDetails.data)
        is PaymentDetails.ClosedChannel -> readableMapOf(paymentDetails.data)
    }
}

fun readableMapOf(rate: Rate): ReadableMap {
    return readableMapOf(
            "coin" to rate.coin,
            "value" to rate.value
    )
}

fun readableMapOf(recommendedFees: RecommendedFees): ReadableMap {
    return readableMapOf(
            "fastestFee" to recommendedFees.fastestFee,
            "halfHourFee" to recommendedFees.halfHourFee,
            "hourFee" to recommendedFees.hourFee,
            "economyFee" to recommendedFees.economyFee,
            "minimumFee" to recommendedFees.minimumFee
    )
}

fun readableMapOf(routeHint: RouteHint): ReadableMap {
    return readableMapOf("hops" to readableArrayOf(routeHint.hops))
}

fun readableMapOf(routeHintHop: RouteHintHop): ReadableMap {
    return readableMapOf(
            "srcNodeId" to routeHintHop.srcNodeId,
            "shortChannelId" to routeHintHop.shortChannelId,
            "feesBaseMsat" to routeHintHop.feesBaseMsat,
            "feesProportionalMillionths" to routeHintHop.feesProportionalMillionths,
            "cltvExpiryDelta" to routeHintHop.cltvExpiryDelta,
            "htlcMinimumMsat" to routeHintHop.htlcMinimumMsat,
            "htlcMaximumMsat" to routeHintHop.htlcMaximumMsat
    )
}

fun readableMapOf(symbol: Symbol?): ReadableMap? {
    if (symbol != null) {
        return readableMapOf(
                "grapheme" to symbol.grapheme,
                "template" to symbol.template,
                "rtl" to symbol.rtl,
                "position" to symbol.position
        )
    }
    return null
}

fun readableMapOf(successActionProcessed: SuccessActionProcessed?): ReadableMap? {
    if (successActionProcessed != null) {
        return when (successActionProcessed) {
            is SuccessActionProcessed.Aes -> readableMapOf(successActionProcessed.data)
            is SuccessActionProcessed.Message -> readableMapOf(successActionProcessed.data)
            is SuccessActionProcessed.Url -> readableMapOf(successActionProcessed.data)
        }
    }
    return null
}

fun readableMapOf(swapInfo: SwapInfo): ReadableMap {
    return readableMapOf(
            "bitcoinAddress" to swapInfo.bitcoinAddress,
            "createdAt" to swapInfo.createdAt,
            "lockHeight" to swapInfo.lockHeight,
            "paymentHash" to swapInfo.paymentHash,
            "preimage" to swapInfo.preimage,
            "privateKey" to swapInfo.privateKey,
            "publicKey" to swapInfo.publicKey,
            "swapperPublicKey" to swapInfo.swapperPublicKey,
            "script" to swapInfo.script,
            "bolt11" to swapInfo.bolt11,
            "paidSats" to swapInfo.paidSats,
            "unconfirmedSats" to swapInfo.unconfirmedSats,
            "confirmedSats" to swapInfo.confirmedSats,
            "status" to swapInfo.status.name.lowercase(),
            "refundTxIds" to swapInfo.refundTxIds,
            "unconfirmedTxIds" to swapInfo.unconfirmedTxIds,
            "confirmedTxIds" to swapInfo.confirmedTxIds,
            "minAllowedDeposit" to swapInfo.minAllowedDeposit,
            "maxAllowedDeposit" to swapInfo.maxAllowedDeposit,
            "lastRedeemError" to swapInfo.lastRedeemError
    )
}

fun readableMapOf(reverseSwapPairInfo: ReverseSwapPairInfo): ReadableMap {
    return readableMapOf(
            "min" to reverseSwapPairInfo.min,
            "max" to reverseSwapPairInfo.max,
            "feesHash" to reverseSwapPairInfo.feesHash,
            "feesPercentage" to reverseSwapPairInfo.feesPercentage,
            "feesLockup" to reverseSwapPairInfo.feesLockup,
            "feesClaim" to reverseSwapPairInfo.feesClaim
    )
}

fun readableMapOf(reverseSwapInfo: ReverseSwapInfo): ReadableMap {
    return readableMapOf(
            "id" to reverseSwapInfo.id,
            "claimPubkey" to reverseSwapInfo.claimPubkey,
            "onchainAmountSat" to reverseSwapInfo.onchainAmountSat,
            "status" to reverseSwapInfo.status.name.lowercase()
    )
}

fun readableMapOf(unspentTransactionOutput: UnspentTransactionOutput): ReadableMap {
    return readableMapOf(
            "txid" to unspentTransactionOutput.txid,
            "outnum" to unspentTransactionOutput.outnum,
            "amountMillisatoshi" to unspentTransactionOutput.amountMillisatoshi,
            "address" to unspentTransactionOutput.address,
            "reserved" to unspentTransactionOutput.reserved,
            "reservedToBlock" to unspentTransactionOutput.reservedToBlock
    )
}

fun readableMapOf(urlSuccessActionData: UrlSuccessActionData): ReadableMap {
    return readableMapOf(
            "type" to "url",
            "description" to urlSuccessActionData.description,
            "url" to urlSuccessActionData.url
    )
}

fun readableMapOf(vararg values: Pair<String, *>): ReadableMap {
    val map = Arguments.createMap()
    for ((key, value) in values) {
        pushToMap(map, key, value)
    }
    return map
}
