package flutter_secured_storage

import android.content.Context
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.Base64
import android.util.Log
import com.it_nomads.fluttersecurestorage.ciphers.KeyCipher
import java.math.BigInteger
import java.security.Key
import java.security.KeyPairGenerator
import java.security.KeyStore
import java.security.PrivateKey
import java.security.PublicKey
import java.security.spec.AlgorithmParameterSpec
import java.util.Calendar
import java.util.Locale
import javax.crypto.Cipher
import javax.crypto.spec.IvParameterSpec
import javax.security.auth.x500.X500Principal


internal class RSACipher18Implementation(private val context: Context) : KeyCipher {
    private val keyAlias: String

    init {
        keyAlias = createKeyAlias()
        createRSAKeysIfNeeded()
    }

    private fun createKeyAlias(): String {
        return context.packageName + ".FlutterSecureStoragePluginKey"
    }

    @Throws(Exception::class)
    override fun wrap(key: Key): ByteArray {
        val publicKey = publicKey
        val cipher = rSACipher
        cipher.init(Cipher.WRAP_MODE, publicKey, algorithmParameterSpec)
        return cipher.wrap(key)
    }

    @Throws(Exception::class)
    override fun unwrap(wrappedKey: ByteArray, algorithm: String): Key {
        val privateKey = privateKey
        val cipher = rSACipher
        cipher.init(Cipher.UNWRAP_MODE, privateKey, algorithmParameterSpec)
        return cipher.unwrap(wrappedKey, algorithm, Cipher.SECRET_KEY)
    }

    @get:Throws(Exception::class)
    private val privateKey: PrivateKey
        get() {
            val ks = KeyStore.getInstance(KEYSTORE_PROVIDER_ANDROID)
            ks.load(null)
            return (ks.getKey(keyAlias, null)
                ?: throw Exception("No key found under alias: $keyAlias")) as? PrivateKey
                ?: throw Exception("Not an instance of a PrivateKey")
        }

    @get:Throws(Exception::class)
    private val publicKey: PublicKey
        get() {
            val ks =
                KeyStore.getInstance(KEYSTORE_PROVIDER_ANDROID)
            ks.load(null)
            val cert = ks.getCertificate(keyAlias)
                ?: throw Exception("No certificate found under alias: $keyAlias")
            return cert.publicKey ?: throw Exception("No key found under alias: $keyAlias")
        }

    @get:Throws(Exception::class)
    private val rSACipher: Cipher
        get() = Cipher.getInstance(
            "RSA/ECB/PKCS1Padding",
            "AndroidKeyStoreBCWorkaround"
        ) // error in android 5: NoSuchProviderException: Provider not available: AndroidKeyStoreBCWorkaround
    private val algorithmParameterSpec: AlgorithmParameterSpec?
        get() = null

    @Throws(Exception::class)
    private fun createRSAKeysIfNeeded() {
        val ks = KeyStore.getInstance(KEYSTORE_PROVIDER_ANDROID)
        ks.load(null)
        val privateKey = ks.getKey(keyAlias, null)
        if (privateKey == null) {
            createKeys()
        }
    }

    /**
     * Sets default locale.
     */
    private fun setLocale(locale: Locale) {
        Locale.setDefault(locale)
        val config = context.resources.configuration
        config.setLocale(locale)
        context.createConfigurationContext(config)
    }

    @Throws(Exception::class)
    private fun createKeys() {
        val localeBeforeFakingEnglishLocale = Locale.getDefault()
        try {
            setLocale(Locale.ENGLISH)
            val start = Calendar.getInstance()
            val end = Calendar.getInstance()
            end.add(Calendar.YEAR, 25)
            val kpGenerator = KeyPairGenerator.getInstance(TYPE_RSA, KEYSTORE_PROVIDER_ANDROID)
            val spec: AlgorithmParameterSpec = makeAlgorithmParameterSpec(start, end)
            kpGenerator.initialize(spec)
            kpGenerator.generateKeyPair()
        } finally {
            setLocale(localeBeforeFakingEnglishLocale)
        }
    }

    private fun makeAlgorithmParameterSpec(
        start: Calendar,
        end: Calendar
    ): AlgorithmParameterSpec {
        val builder = KeyGenParameterSpec.Builder(
            keyAlias,
            KeyProperties.PURPOSE_DECRYPT or KeyProperties.PURPOSE_ENCRYPT
        )
            .setCertificateSubject(X500Principal("CN=$keyAlias"))
            .setDigests(KeyProperties.DIGEST_SHA256)
            .setBlockModes(KeyProperties.BLOCK_MODE_ECB)
            .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_RSA_PKCS1)
            .setCertificateSerialNumber(BigInteger.valueOf(1))
            .setCertificateNotBefore(start.time)
            .setCertificateNotAfter(end.time)
        return builder.build()
    }

    companion object {
        private const val KEYSTORE_PROVIDER_ANDROID = "AndroidKeyStore"
        private const val TYPE_RSA = "RSA"
    }
}


class StorageCipher18Implementation(context: Context, rsaCipher: KeyCipher) {
    private val context: Context
    private val rsaCipher: KeyCipher

    init {
        this.context = context
        this.rsaCipher = rsaCipher
    }

    private fun getKey () : Key  {
        val aesPreferencesKey = aESPreferencesKey
        val preferences =
            context.getSharedPreferences(SHARED_PREFERENCES_NAME, Context.MODE_PRIVATE)
        val aesKey = preferences.getString(aesPreferencesKey, null)
        if (aesKey != null) {
            val encrypted: ByteArray
            try {
                encrypted = Base64.decode(aesKey, Base64.DEFAULT)
                return rsaCipher.unwrap(encrypted, KEY_ALGORITHM)
            } catch (e: java.lang.Exception) {
                Log.e("Unwrap key failed" + e.message, e.toString())
                throw e
            }
        }
        throw java.lang.Exception("key not found")
    }

    private val aESPreferencesKey: String
        get() = "VGhpcyBpcyB0aGUga2V5IGZvciBhIHNlY3VyZSBzdG9yYWdlIEFFUyBLZXkK"

    @Throws(java.lang.Exception::class)
    private fun getCipher(): Cipher {
        return Cipher.getInstance("AES/CBC/PKCS7Padding")
    }

    @Throws(java.lang.Exception::class)
    fun decrypt(input: ByteArray): ByteArray {
        val iv = ByteArray(ivSize)
        System.arraycopy(input, 0, iv, 0, iv.size)
        val ivParameterSpec = getParameterSpec(iv)
        val payloadSize = input.size - ivSize
        val payload = ByteArray(payloadSize)
        System.arraycopy(input, iv.size, payload, 0, payloadSize)
        val cipher = getCipher()
        cipher.init(Cipher.DECRYPT_MODE, getKey(), ivParameterSpec)
        return cipher.doFinal(payload)
    }

    private val ivSize: Int
        get() = keySize

    private fun getParameterSpec(iv: ByteArray?): AlgorithmParameterSpec {
        return IvParameterSpec(iv)
    }

    companion object {
        private const val TAG = "StorageCipher18Implementation"

        private const val keySize = 16
        private const val KEY_ALGORITHM = "AES"
        private const val SHARED_PREFERENCES_NAME = "FlutterSecureKeyStorage"
    }
}