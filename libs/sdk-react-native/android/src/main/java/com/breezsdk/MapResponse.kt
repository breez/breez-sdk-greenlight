package com.breezsdk

import com.facebook.react.bridge.Arguments
import com.facebook.react.bridge.WritableArray
import com.facebook.react.bridge.WritableMap

fun writableMapOf(vararg values: Pair<String, *>): WritableMap {
    val map = Arguments.createMap()
    for ((key, value) in values) {
        pushToMap(map, key, value)
    }
    return map
}

fun writableArrayOf(vararg values: Any?): WritableArray {
    val array = Arguments.createArray()
    for (value in values) {
        pushToArray(array, value)
    }
    return array
}

fun writableArrayOf(values: Iterable<*>): WritableArray {
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
        is Array<*> -> array.pushArray(writableArrayOf(value.asIterable()))
        is List<*> -> array.pushArray(writableArrayOf(value))
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
        is Array<*> -> map.putArray(key, writableArrayOf(value.asIterable()))
        is List<*> -> map.putArray(key, writableArrayOf(value))
        else -> throw IllegalArgumentException("Unsupported value type ${value::class.java.name} for key [$key]")
    }
}