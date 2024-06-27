fun readableMapOf(vararg values: Pair<String, *>): ReadableMap {
    val map = Arguments.createMap()
    for ((key, value) in values) {
        pushToMap(map, key, value)
    }
    return map
}

fun hasNonNullKey(map: ReadableMap, key: String): Boolean {
    return map.hasKey(key) && !map.isNull(key)
}

fun validateMandatoryFields(map: ReadableMap, keys: Array<String>): Boolean {
    for (k in keys) {
        if (!hasNonNullKey(map, k)) return false
    }

    return true
}

fun pushToArray(array: WritableArray, value: Any?) {
    when (value) {
        null -> array.pushNull()
        {%- for sequence_type in self.sequence_types() %}
	    is {{ sequence_type }} -> {{sequence_type|render_to_array(ci)}}
	    {%- endfor %}
        is Array<*> -> array.pushArray(readableArrayOf(value.asIterable()))
        is List<*> -> array.pushArray(readableArrayOf(value))
        else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
    }
}

fun pushToMap(map: WritableMap, key: String, value: Any?) {
    when (value) {
        null -> map.putNull(key)
        is Boolean -> map.putBoolean(key, value)
        is Byte -> map.putInt(key, value.toInt())
        is Double -> map.putDouble(key, value)
        is Float -> map.putDouble(key, value.toDouble())
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
        else -> throw SdkException.Generic("Unexpected type ${value::class.java.name} for key [$key]")
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

fun asUByteList(arr: ReadableArray): List<UByte> {
    val list = ArrayList<UByte>()
    for (value in arr.toArrayList()) {
        when (value) {
            is Double -> list.add(value.toInt().toUByte())
            is Int -> list.add(value.toUByte())
            is UByte -> list.add(value)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asStringList(arr: ReadableArray): List<String> {
    val list = ArrayList<String>()
    for (value in arr.toArrayList()) {
        list.add(value.toString())
    }
    return list
}

fun errMissingMandatoryField(fieldName: String, typeName: String): String {
        return "Missing mandatory field ${fieldName} for type ${typeName}"
    }

fun errUnexpectedType(typeName: String): String {
        return "Unexpected type ${typeName}"
    }

fun errUnexpectedValue(fieldName: String): String {
    return "Unexpected value for optional field ${fieldName}"
}

fun camelToUpperSnakeCase(str: String): String {
    val pattern = "(?<=.)[A-Z]".toRegex()
    return str.replace(pattern, "_$0").uppercase()
}
