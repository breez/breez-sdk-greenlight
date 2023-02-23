package com.breezsdk

import breez_sdk.*
import com.facebook.react.bridge.Arguments
import com.facebook.react.bridge.ReadableArray
import com.facebook.react.bridge.ReadableMap
import com.facebook.react.bridge.WritableArray
import com.facebook.react.bridge.WritableMap

fun asLnUrlWithdrawRequestData(reqData: ReadableMap): LnUrlWithdrawRequestData? {
    var callback = reqData.getString("callback")
    var k1 = reqData.getString("k1")
    var defaultDescription = reqData.getString("defaultDescription")

    if (callback != null && k1 != null && defaultDescription != null && reqData.hasKey("minWithdrawable") && reqData.hasKey("maxWithdrawable")) {
        var minWithdrawable = reqData.getDouble("minWithdrawable")
        var maxWithdrawable = reqData.getDouble("maxWithdrawable")

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

fun asUByteList(arr: ReadableArray): List<UByte> {
    var list = ArrayList<UByte>()
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
        is RouteHint -> array.pushMap(readableMapOf(value))
        is RouteHintHop -> array.pushMap(readableMapOf(value))
        is String -> array.pushString(value)
        is SwapInfo -> array.pushMap(readableMapOf(value))
        is UByte -> array.pushInt(value.toInt())
        is ULong -> array.pushDouble(value.toDouble())
        is UnspentTransactionOutput -> array.pushMap(readableMapOf(value))
        is WritableArray -> array.pushArray(value)
        is WritableMap -> array.pushMap(value)
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
        is String -> map.putString(key, value)
        is UByte -> map.putInt(key, value.toInt())
        is UInt -> map.putInt(key, value.toInt())
        is ULong -> map.putDouble(key, value.toDouble())
        is WritableMap -> map.putMap(key, value)
        is WritableArray -> map.putArray(key, value)
        is Array<*> -> map.putArray(key, readableArrayOf(value.asIterable()))
        is List<*> -> map.putArray(key, readableArrayOf(value))
        else -> throw IllegalArgumentException("Unsupported value type ${value::class.java.name} for key [$key]")
    }
}

fun readableArrayOf(values: Iterable<*>?): ReadableArray? {
    if (values != null) {
        val array = Arguments.createArray()
        for (value in values) {
            pushToArray(array, value)
        }
        return array
    }

    return null
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

fun readableMapOf(currencyInfo: CurrencyInfo): ReadableMap {
    return readableMapOf(
            "name" to currencyInfo.name,
            "fractionSize" to currencyInfo.fractionSize,
            "spacing" to currencyInfo.spacing,
            "symbol" to readableMapOf(currencyInfo.symbol),
            "uniqSymbol" to readableMapOf(currencyInfo.uniqSymbol),
            "fractionSize" to currencyInfo.fractionSize,
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
            "deviceKey" to greenlightCredentials.deviceKey,
            "deviceCert" to greenlightCredentials.deviceCert
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
            "lnurlSuccessAction" to readableMapOf(lnPaymentDetails.lnurlSuccessAction)
    )
}

fun readableMapOf(lnUrlAuthRequestData: LnUrlAuthRequestData): ReadableMap {
    return readableMapOf("k1" to lnUrlAuthRequestData.k1)
}

fun readableMapOf(lnUrlErrorData: LnUrlErrorData): ReadableMap {
    return readableMapOf("reason" to lnUrlErrorData.reason)
}

fun readableMapOf(lnUrlPayRequestData: LnUrlPayRequestData): ReadableMap {
    return readableMapOf(
            "callback" to lnUrlPayRequestData.callback,
            "minSendable" to lnUrlPayRequestData.minSendable,
            "maxSendable" to lnUrlPayRequestData.maxSendable,
            "metadataStr" to lnUrlPayRequestData.metadataStr,
            "commentAllowed" to lnUrlPayRequestData.commentAllowed,
            "domain" to lnUrlPayRequestData.domain
    )
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

fun readableMapOf(lnUrlWithdrawCallbackStatus: LnUrlWithdrawCallbackStatus): ReadableMap {
    return when (lnUrlWithdrawCallbackStatus) {
        is LnUrlWithdrawCallbackStatus.Ok -> readableMapOf("status" to "ok")
        is LnUrlWithdrawCallbackStatus.ErrorStatus -> {
            var response = Arguments.createMap()
            response.putString("status", "error")
            response.merge(readableMapOf(lnUrlWithdrawCallbackStatus.data))
            response
        }
    }
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

