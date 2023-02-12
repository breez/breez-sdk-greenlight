package com.breezsdk

import breez_sdk.LnInvoice
import breez_sdk.RouteHint
import breez_sdk.RouteHintHop
import com.facebook.react.bridge.Arguments
import com.facebook.react.bridge.ReadableArray
import com.facebook.react.bridge.ReadableMap
import com.facebook.react.bridge.WritableArray
import com.facebook.react.bridge.WritableMap

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

fun readableMapOf(routeHint: RouteHint): ReadableMap {
    return readableMapOf(
            "hops" to readableArrayOf(routeHint.hops),
    )
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