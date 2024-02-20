package flutter_secured_storage

import android.content.Context
import android.util.Base64
import java.nio.charset.Charset

class FlutterSecuredStorageHelper {
    companion object {
        @Throws(java.lang.Exception::class)
        fun readSecuredValue(appContext: Context, key: String): String? {
            val preferences = appContext.getSharedPreferences("FlutterSecureStorage", Context.MODE_PRIVATE)
            val rawValue = preferences.getString(key, null)
            return decodeRawValue(appContext, rawValue)
        }

        @Throws(java.lang.Exception::class)
        private fun decodeRawValue(appContext: Context, value: String?): String? {
            if (value == null) {
                return null
            }
            val charset = Charset.forName("UTF-8")
            val data: ByteArray = Base64.decode(value, 0)
            val keyCipherAlgo = RSACipher18Implementation(context = appContext)
            val storageCipher = StorageCipher18Implementation(appContext, keyCipherAlgo)
            val result: ByteArray = storageCipher.decrypt(data)
            return String(result, charset)
        }
    }
}