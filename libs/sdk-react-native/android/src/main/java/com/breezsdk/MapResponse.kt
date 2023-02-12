package com.breezsdk

import com.facebook.react.bridge.Arguments
import com.facebook.react.bridge.ReadableArray
import com.facebook.react.bridge.ReadableMap
import com.facebook.react.bridge.WritableArray
import com.facebook.react.bridge.WritableMap

fun readableMapOf(vararg values: Pair<String, *>): ReadableMap {
    val map = Arguments.createMap()
    for ((key, value) in values) {
        pushToMap(map, key, value)
    }
    return map
}

fun readableArrayOf(vararg values: Any?): ReadableArray {
    val array = Arguments.createArray()
    for (value in values) {
        pushToArray(array, value)
    }
    return array
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
        is String -> array.pushString(value)
        is UByte -> array.pushInt(value.toInt())
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
        is WritableMap -> map.putMap(key, value)
        is WritableArray -> map.putArray(key, value)
        is Array<*> -> map.putArray(key, readableArrayOf(value.asIterable()))
        is List<*> -> map.putArray(key, readableArrayOf(value))
        else -> throw IllegalArgumentException("Unsupported value type ${value::class.java.name} for key [$key]")
    }
}