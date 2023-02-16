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
    var minWithdrawable = reqData.getString("minWithdrawable")
    var maxWithdrawable = reqData.getString("maxWithdrawable")

    try {
        if (callback != null && k1 != null && defaultDescription != null && minWithdrawable != null && maxWithdrawable != null) {
            return LnUrlWithdrawRequestData(callback, k1, defaultDescription, minWithdrawable.toULong(), maxWithdrawable.toULong())
        }
    } catch (e: NumberFormatException) {}

    return null
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
            "commentAllowed" to lnUrlPayRequestData.commentAllowed
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

fun readableArrayOf(values: Iterable<*>): ReadableArray {
    val array = Arguments.createArray()
    for (value in values) {
        pushToArray(array, value)
    }
    return array
}

fun pushToArray(array: WritableArray, value: Any?) {
    when (value) {
        null -> array.pushNull()
        is Boolean -> array.pushBoolean(value)
        is Double -> array.pushDouble(value)
        is Int -> array.pushInt(value)
        is RouteHint -> array.pushMap(readableMapOf(value))
        is RouteHintHop -> array.pushMap(readableMapOf(value))
        is String -> array.pushString(value)
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
        is Double -> map.putDouble(key, value)
        is Int -> map.putInt(key, value)
        is String -> map.putString(key, value)
        is UByte -> map.putInt(key, value.toInt())
        is ULong -> map.putDouble(key, value.toDouble())
        is WritableMap -> map.putMap(key, value)
        is WritableArray -> map.putArray(key, value)
        is Array<*> -> map.putArray(key, readableArrayOf(value.asIterable()))
        is List<*> -> map.putArray(key, readableArrayOf(value))
        else -> throw IllegalArgumentException("Unsupported value type ${value::class.java.name} for key [$key]")
    }
}
