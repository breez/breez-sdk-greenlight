package com.breezsdk
import breez_sdk.*
import com.facebook.react.bridge.*
import java.util.*

fun asAesSuccessActionDataDecrypted(data: ReadableMap): AesSuccessActionDataDecrypted? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "description",
                "plaintext",
            ),
        )
    ) {
        return null
    }
    val description = data.getString("description")!!
    val plaintext = data.getString("plaintext")!!
    return AesSuccessActionDataDecrypted(
        description,
        plaintext,
    )
}

fun readableMapOf(aesSuccessActionDataDecrypted: AesSuccessActionDataDecrypted): ReadableMap {
    return readableMapOf(
        "description" to aesSuccessActionDataDecrypted.description,
        "plaintext" to aesSuccessActionDataDecrypted.plaintext,
    )
}

fun asAesSuccessActionDataDecryptedList(arr: ReadableArray): List<AesSuccessActionDataDecrypted> {
    val list = ArrayList<AesSuccessActionDataDecrypted>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asAesSuccessActionDataDecrypted(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asBackupFailedData(data: ReadableMap): BackupFailedData? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "error",
            ),
        )
    ) {
        return null
    }
    val error = data.getString("error")!!
    return BackupFailedData(
        error,
    )
}

fun readableMapOf(backupFailedData: BackupFailedData): ReadableMap {
    return readableMapOf(
        "error" to backupFailedData.error,
    )
}

fun asBackupFailedDataList(arr: ReadableArray): List<BackupFailedData> {
    val list = ArrayList<BackupFailedData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asBackupFailedData(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asBackupStatus(data: ReadableMap): BackupStatus? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "backedUp",
            ),
        )
    ) {
        return null
    }
    val backedUp = data.getBoolean("backedUp")
    val lastBackupTime = if (hasNonNullKey(data, "lastBackupTime")) data.getDouble("lastBackupTime").toULong() else null
    return BackupStatus(
        backedUp,
        lastBackupTime,
    )
}

fun readableMapOf(backupStatus: BackupStatus): ReadableMap {
    return readableMapOf(
        "backedUp" to backupStatus.backedUp,
        "lastBackupTime" to backupStatus.lastBackupTime,
    )
}

fun asBackupStatusList(arr: ReadableArray): List<BackupStatus> {
    val list = ArrayList<BackupStatus>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asBackupStatus(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asBitcoinAddressData(data: ReadableMap): BitcoinAddressData? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "address",
                "network",
            ),
        )
    ) {
        return null
    }
    val address = data.getString("address")!!
    val network = data.getString("network")?.let { asNetwork(it) }!!
    val amountSat = if (hasNonNullKey(data, "amountSat")) data.getDouble("amountSat").toULong() else null
    val label = if (hasNonNullKey(data, "label")) data.getString("label") else null
    val message = if (hasNonNullKey(data, "message")) data.getString("message") else null
    return BitcoinAddressData(
        address,
        network,
        amountSat,
        label,
        message,
    )
}

fun readableMapOf(bitcoinAddressData: BitcoinAddressData): ReadableMap {
    return readableMapOf(
        "address" to bitcoinAddressData.address,
        "network" to bitcoinAddressData.network.name.lowercase(),
        "amountSat" to bitcoinAddressData.amountSat,
        "label" to bitcoinAddressData.label,
        "message" to bitcoinAddressData.message,
    )
}

fun asBitcoinAddressDataList(arr: ReadableArray): List<BitcoinAddressData> {
    val list = ArrayList<BitcoinAddressData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asBitcoinAddressData(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asBuyBitcoinRequest(data: ReadableMap): BuyBitcoinRequest? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "provider",
            ),
        )
    ) {
        return null
    }
    val provider = data.getString("provider")?.let { asBuyBitcoinProvider(it) }!!
    val openingFeeParams =
        if (hasNonNullKey(data, "openingFeeParams")) {
            data.getMap("openingFeeParams")?.let {
                asOpeningFeeParams(it)
            }
        } else {
            null
        }
    return BuyBitcoinRequest(
        provider,
        openingFeeParams,
    )
}

fun readableMapOf(buyBitcoinRequest: BuyBitcoinRequest): ReadableMap {
    return readableMapOf(
        "provider" to buyBitcoinRequest.provider.name.lowercase(),
        "openingFeeParams" to buyBitcoinRequest.openingFeeParams?.let { readableMapOf(it) },
    )
}

fun asBuyBitcoinRequestList(arr: ReadableArray): List<BuyBitcoinRequest> {
    val list = ArrayList<BuyBitcoinRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asBuyBitcoinRequest(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asBuyBitcoinResponse(data: ReadableMap): BuyBitcoinResponse? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "url",
            ),
        )
    ) {
        return null
    }
    val url = data.getString("url")!!
    val openingFeeParams =
        if (hasNonNullKey(data, "openingFeeParams")) {
            data.getMap("openingFeeParams")?.let {
                asOpeningFeeParams(it)
            }
        } else {
            null
        }
    return BuyBitcoinResponse(
        url,
        openingFeeParams,
    )
}

fun readableMapOf(buyBitcoinResponse: BuyBitcoinResponse): ReadableMap {
    return readableMapOf(
        "url" to buyBitcoinResponse.url,
        "openingFeeParams" to buyBitcoinResponse.openingFeeParams?.let { readableMapOf(it) },
    )
}

fun asBuyBitcoinResponseList(arr: ReadableArray): List<BuyBitcoinResponse> {
    val list = ArrayList<BuyBitcoinResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asBuyBitcoinResponse(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asCheckMessageRequest(data: ReadableMap): CheckMessageRequest? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "message",
                "pubkey",
                "signature",
            ),
        )
    ) {
        return null
    }
    val message = data.getString("message")!!
    val pubkey = data.getString("pubkey")!!
    val signature = data.getString("signature")!!
    return CheckMessageRequest(
        message,
        pubkey,
        signature,
    )
}

fun readableMapOf(checkMessageRequest: CheckMessageRequest): ReadableMap {
    return readableMapOf(
        "message" to checkMessageRequest.message,
        "pubkey" to checkMessageRequest.pubkey,
        "signature" to checkMessageRequest.signature,
    )
}

fun asCheckMessageRequestList(arr: ReadableArray): List<CheckMessageRequest> {
    val list = ArrayList<CheckMessageRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asCheckMessageRequest(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asCheckMessageResponse(data: ReadableMap): CheckMessageResponse? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "isValid",
            ),
        )
    ) {
        return null
    }
    val isValid = data.getBoolean("isValid")
    return CheckMessageResponse(
        isValid,
    )
}

fun readableMapOf(checkMessageResponse: CheckMessageResponse): ReadableMap {
    return readableMapOf(
        "isValid" to checkMessageResponse.isValid,
    )
}

fun asCheckMessageResponseList(arr: ReadableArray): List<CheckMessageResponse> {
    val list = ArrayList<CheckMessageResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asCheckMessageResponse(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asClosedChannelPaymentDetails(data: ReadableMap): ClosedChannelPaymentDetails? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "shortChannelId",
                "state",
                "fundingTxid",
            ),
        )
    ) {
        return null
    }
    val shortChannelId = data.getString("shortChannelId")!!
    val state = data.getString("state")?.let { asChannelState(it) }!!
    val fundingTxid = data.getString("fundingTxid")!!
    return ClosedChannelPaymentDetails(
        shortChannelId,
        state,
        fundingTxid,
    )
}

fun readableMapOf(closedChannelPaymentDetails: ClosedChannelPaymentDetails): ReadableMap {
    return readableMapOf(
        "shortChannelId" to closedChannelPaymentDetails.shortChannelId,
        "state" to closedChannelPaymentDetails.state.name.lowercase(),
        "fundingTxid" to closedChannelPaymentDetails.fundingTxid,
    )
}

fun asClosedChannelPaymentDetailsList(arr: ReadableArray): List<ClosedChannelPaymentDetails> {
    val list = ArrayList<ClosedChannelPaymentDetails>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asClosedChannelPaymentDetails(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asConfig(data: ReadableMap): Config? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "breezserver",
                "mempoolspaceUrl",
                "workingDir",
                "network",
                "paymentTimeoutSec",
                "maxfeePercent",
                "exemptfeeMsat",
                "nodeConfig",
            ),
        )
    ) {
        return null
    }
    val breezserver = data.getString("breezserver")!!
    val mempoolspaceUrl = data.getString("mempoolspaceUrl")!!
    val workingDir = data.getString("workingDir")!!
    val network = data.getString("network")?.let { asNetwork(it) }!!
    val paymentTimeoutSec = data.getInt("paymentTimeoutSec").toUInt()
    val defaultLspId = if (hasNonNullKey(data, "defaultLspId")) data.getString("defaultLspId") else null
    val apiKey = if (hasNonNullKey(data, "apiKey")) data.getString("apiKey") else null
    val maxfeePercent = data.getDouble("maxfeePercent")
    val exemptfeeMsat = data.getDouble("exemptfeeMsat").toULong()
    val nodeConfig = data.getMap("nodeConfig")?.let { asNodeConfig(it) }!!
    return Config(
        breezserver,
        mempoolspaceUrl,
        workingDir,
        network,
        paymentTimeoutSec,
        defaultLspId,
        apiKey,
        maxfeePercent,
        exemptfeeMsat,
        nodeConfig,
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
        "maxfeePercent" to config.maxfeePercent,
        "exemptfeeMsat" to config.exemptfeeMsat,
        "nodeConfig" to readableMapOf(config.nodeConfig),
    )
}

fun asConfigList(arr: ReadableArray): List<Config> {
    val list = ArrayList<Config>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asConfig(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asCurrencyInfo(data: ReadableMap): CurrencyInfo? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "name",
                "fractionSize",
            ),
        )
    ) {
        return null
    }
    val name = data.getString("name")!!
    val fractionSize = data.getInt("fractionSize").toUInt()
    val spacing = if (hasNonNullKey(data, "spacing")) data.getInt("spacing").toUInt() else null
    val symbol = if (hasNonNullKey(data, "symbol")) data.getMap("symbol")?.let { asSymbol(it) } else null
    val uniqSymbol = if (hasNonNullKey(data, "uniqSymbol")) data.getMap("uniqSymbol")?.let { asSymbol(it) } else null
    val localizedName = if (hasNonNullKey(data, "localizedName")) data.getArray("localizedName")?.let { asLocalizedNameList(it) } else null
    val localeOverrides =
        if (hasNonNullKey(data, "localeOverrides")) {
            data.getArray("localeOverrides")?.let {
                asLocaleOverridesList(it)
            }
        } else {
            null
        }
    return CurrencyInfo(
        name,
        fractionSize,
        spacing,
        symbol,
        uniqSymbol,
        localizedName,
        localeOverrides,
    )
}

fun readableMapOf(currencyInfo: CurrencyInfo): ReadableMap {
    return readableMapOf(
        "name" to currencyInfo.name,
        "fractionSize" to currencyInfo.fractionSize,
        "spacing" to currencyInfo.spacing,
        "symbol" to currencyInfo.symbol?.let { readableMapOf(it) },
        "uniqSymbol" to currencyInfo.uniqSymbol?.let { readableMapOf(it) },
        "localizedName" to currencyInfo.localizedName?.let { readableArrayOf(it) },
        "localeOverrides" to currencyInfo.localeOverrides?.let { readableArrayOf(it) },
    )
}

fun asCurrencyInfoList(arr: ReadableArray): List<CurrencyInfo> {
    val list = ArrayList<CurrencyInfo>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asCurrencyInfo(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asFiatCurrency(data: ReadableMap): FiatCurrency? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "id",
                "info",
            ),
        )
    ) {
        return null
    }
    val id = data.getString("id")!!
    val info = data.getMap("info")?.let { asCurrencyInfo(it) }!!
    return FiatCurrency(
        id,
        info,
    )
}

fun readableMapOf(fiatCurrency: FiatCurrency): ReadableMap {
    return readableMapOf(
        "id" to fiatCurrency.id,
        "info" to readableMapOf(fiatCurrency.info),
    )
}

fun asFiatCurrencyList(arr: ReadableArray): List<FiatCurrency> {
    val list = ArrayList<FiatCurrency>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asFiatCurrency(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asGreenlightCredentials(data: ReadableMap): GreenlightCredentials? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "deviceKey",
                "deviceCert",
            ),
        )
    ) {
        return null
    }
    val deviceKey = data.getArray("deviceKey")?.let { asUByteList(it) }!!
    val deviceCert = data.getArray("deviceCert")?.let { asUByteList(it) }!!
    return GreenlightCredentials(
        deviceKey,
        deviceCert,
    )
}

fun readableMapOf(greenlightCredentials: GreenlightCredentials): ReadableMap {
    return readableMapOf(
        "deviceKey" to readableArrayOf(greenlightCredentials.deviceKey),
        "deviceCert" to readableArrayOf(greenlightCredentials.deviceCert),
    )
}

fun asGreenlightCredentialsList(arr: ReadableArray): List<GreenlightCredentials> {
    val list = ArrayList<GreenlightCredentials>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asGreenlightCredentials(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asGreenlightNodeConfig(data: ReadableMap): GreenlightNodeConfig? {
    if (!validateMandatoryFields(
            data,
            arrayOf(),
        )
    ) {
        return null
    }
    val partnerCredentials =
        if (hasNonNullKey(data, "partnerCredentials")) {
            data.getMap("partnerCredentials")?.let {
                asGreenlightCredentials(it)
            }
        } else {
            null
        }
    val inviteCode = if (hasNonNullKey(data, "inviteCode")) data.getString("inviteCode") else null
    return GreenlightNodeConfig(
        partnerCredentials,
        inviteCode,
    )
}

fun readableMapOf(greenlightNodeConfig: GreenlightNodeConfig): ReadableMap {
    return readableMapOf(
        "partnerCredentials" to greenlightNodeConfig.partnerCredentials?.let { readableMapOf(it) },
        "inviteCode" to greenlightNodeConfig.inviteCode,
    )
}

fun asGreenlightNodeConfigList(arr: ReadableArray): List<GreenlightNodeConfig> {
    val list = ArrayList<GreenlightNodeConfig>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asGreenlightNodeConfig(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asInvoicePaidDetails(data: ReadableMap): InvoicePaidDetails? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "paymentHash",
                "bolt11",
            ),
        )
    ) {
        return null
    }
    val paymentHash = data.getString("paymentHash")!!
    val bolt11 = data.getString("bolt11")!!
    return InvoicePaidDetails(
        paymentHash,
        bolt11,
    )
}

fun readableMapOf(invoicePaidDetails: InvoicePaidDetails): ReadableMap {
    return readableMapOf(
        "paymentHash" to invoicePaidDetails.paymentHash,
        "bolt11" to invoicePaidDetails.bolt11,
    )
}

fun asInvoicePaidDetailsList(arr: ReadableArray): List<InvoicePaidDetails> {
    val list = ArrayList<InvoicePaidDetails>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asInvoicePaidDetails(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asLnInvoice(data: ReadableMap): LnInvoice? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "bolt11",
                "payeePubkey",
                "paymentHash",
                "timestamp",
                "expiry",
                "routingHints",
                "paymentSecret",
            ),
        )
    ) {
        return null
    }
    val bolt11 = data.getString("bolt11")!!
    val payeePubkey = data.getString("payeePubkey")!!
    val paymentHash = data.getString("paymentHash")!!
    val description = if (hasNonNullKey(data, "description")) data.getString("description") else null
    val descriptionHash = if (hasNonNullKey(data, "descriptionHash")) data.getString("descriptionHash") else null
    val amountMsat = if (hasNonNullKey(data, "amountMsat")) data.getDouble("amountMsat").toULong() else null
    val timestamp = data.getDouble("timestamp").toULong()
    val expiry = data.getDouble("expiry").toULong()
    val routingHints = data.getArray("routingHints")?.let { asRouteHintList(it) }!!
    val paymentSecret = data.getArray("paymentSecret")?.let { asUByteList(it) }!!
    return LnInvoice(
        bolt11,
        payeePubkey,
        paymentHash,
        description,
        descriptionHash,
        amountMsat,
        timestamp,
        expiry,
        routingHints,
        paymentSecret,
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
        "paymentSecret" to readableArrayOf(lnInvoice.paymentSecret),
    )
}

fun asLnInvoiceList(arr: ReadableArray): List<LnInvoice> {
    val list = ArrayList<LnInvoice>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnInvoice(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asListPaymentsRequest(data: ReadableMap): ListPaymentsRequest? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "filter",
            ),
        )
    ) {
        return null
    }
    val filter = data.getString("filter")?.let { asPaymentTypeFilter(it) }!!
    val fromTimestamp = if (hasNonNullKey(data, "fromTimestamp")) data.getDouble("fromTimestamp").toLong() else null
    val toTimestamp = if (hasNonNullKey(data, "toTimestamp")) data.getDouble("toTimestamp").toLong() else null
    val includeFailures = if (hasNonNullKey(data, "includeFailures")) data.getBoolean("includeFailures") else null
    return ListPaymentsRequest(
        filter,
        fromTimestamp,
        toTimestamp,
        includeFailures,
    )
}

fun readableMapOf(listPaymentsRequest: ListPaymentsRequest): ReadableMap {
    return readableMapOf(
        "filter" to listPaymentsRequest.filter.name.lowercase(),
        "fromTimestamp" to listPaymentsRequest.fromTimestamp,
        "toTimestamp" to listPaymentsRequest.toTimestamp,
        "includeFailures" to listPaymentsRequest.includeFailures,
    )
}

fun asListPaymentsRequestList(arr: ReadableArray): List<ListPaymentsRequest> {
    val list = ArrayList<ListPaymentsRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asListPaymentsRequest(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asLnPaymentDetails(data: ReadableMap): LnPaymentDetails? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "paymentHash",
                "label",
                "destinationPubkey",
                "paymentPreimage",
                "keysend",
                "bolt11",
            ),
        )
    ) {
        return null
    }
    val paymentHash = data.getString("paymentHash")!!
    val label = data.getString("label")!!
    val destinationPubkey = data.getString("destinationPubkey")!!
    val paymentPreimage = data.getString("paymentPreimage")!!
    val keysend = data.getBoolean("keysend")
    val bolt11 = data.getString("bolt11")!!
    val lnurlSuccessAction =
        if (hasNonNullKey(data, "lnurlSuccessAction")) {
            data.getMap("lnurlSuccessAction")?.let {
                asSuccessActionProcessed(it)
            }
        } else {
            null
        }
    val lnurlMetadata = if (hasNonNullKey(data, "lnurlMetadata")) data.getString("lnurlMetadata") else null
    val lnAddress = if (hasNonNullKey(data, "lnAddress")) data.getString("lnAddress") else null
    return LnPaymentDetails(
        paymentHash,
        label,
        destinationPubkey,
        paymentPreimage,
        keysend,
        bolt11,
        lnurlSuccessAction,
        lnurlMetadata,
        lnAddress,
    )
}

fun readableMapOf(lnPaymentDetails: LnPaymentDetails): ReadableMap {
    return readableMapOf(
        "paymentHash" to lnPaymentDetails.paymentHash,
        "label" to lnPaymentDetails.label,
        "destinationPubkey" to lnPaymentDetails.destinationPubkey,
        "paymentPreimage" to lnPaymentDetails.paymentPreimage,
        "keysend" to lnPaymentDetails.keysend,
        "bolt11" to lnPaymentDetails.bolt11,
        "lnurlSuccessAction" to lnPaymentDetails.lnurlSuccessAction?.let { readableMapOf(it) },
        "lnurlMetadata" to lnPaymentDetails.lnurlMetadata,
        "lnAddress" to lnPaymentDetails.lnAddress,
    )
}

fun asLnPaymentDetailsList(arr: ReadableArray): List<LnPaymentDetails> {
    val list = ArrayList<LnPaymentDetails>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnPaymentDetails(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asLnUrlAuthRequestData(data: ReadableMap): LnUrlAuthRequestData? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "k1",
                "domain",
                "url",
            ),
        )
    ) {
        return null
    }
    val k1 = data.getString("k1")!!
    val action = if (hasNonNullKey(data, "action")) data.getString("action") else null
    val domain = data.getString("domain")!!
    val url = data.getString("url")!!
    return LnUrlAuthRequestData(
        k1,
        action,
        domain,
        url,
    )
}

fun readableMapOf(lnUrlAuthRequestData: LnUrlAuthRequestData): ReadableMap {
    return readableMapOf(
        "k1" to lnUrlAuthRequestData.k1,
        "action" to lnUrlAuthRequestData.action,
        "domain" to lnUrlAuthRequestData.domain,
        "url" to lnUrlAuthRequestData.url,
    )
}

fun asLnUrlAuthRequestDataList(arr: ReadableArray): List<LnUrlAuthRequestData> {
    val list = ArrayList<LnUrlAuthRequestData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlAuthRequestData(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asLnUrlErrorData(data: ReadableMap): LnUrlErrorData? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "reason",
            ),
        )
    ) {
        return null
    }
    val reason = data.getString("reason")!!
    return LnUrlErrorData(
        reason,
    )
}

fun readableMapOf(lnUrlErrorData: LnUrlErrorData): ReadableMap {
    return readableMapOf(
        "reason" to lnUrlErrorData.reason,
    )
}

fun asLnUrlErrorDataList(arr: ReadableArray): List<LnUrlErrorData> {
    val list = ArrayList<LnUrlErrorData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlErrorData(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asLnUrlPayRequestData(data: ReadableMap): LnUrlPayRequestData? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "callback",
                "minSendable",
                "maxSendable",
                "metadataStr",
                "commentAllowed",
                "domain",
            ),
        )
    ) {
        return null
    }
    val callback = data.getString("callback")!!
    val minSendable = data.getDouble("minSendable").toULong()
    val maxSendable = data.getDouble("maxSendable").toULong()
    val metadataStr = data.getString("metadataStr")!!
    val commentAllowed = data.getInt("commentAllowed").toUShort()
    val domain = data.getString("domain")!!
    val lnAddress = if (hasNonNullKey(data, "lnAddress")) data.getString("lnAddress") else null
    return LnUrlPayRequestData(
        callback,
        minSendable,
        maxSendable,
        metadataStr,
        commentAllowed,
        domain,
        lnAddress,
    )
}

fun readableMapOf(lnUrlPayRequestData: LnUrlPayRequestData): ReadableMap {
    return readableMapOf(
        "callback" to lnUrlPayRequestData.callback,
        "minSendable" to lnUrlPayRequestData.minSendable,
        "maxSendable" to lnUrlPayRequestData.maxSendable,
        "metadataStr" to lnUrlPayRequestData.metadataStr,
        "commentAllowed" to lnUrlPayRequestData.commentAllowed,
        "domain" to lnUrlPayRequestData.domain,
        "lnAddress" to lnUrlPayRequestData.lnAddress,
    )
}

fun asLnUrlPayRequestDataList(arr: ReadableArray): List<LnUrlPayRequestData> {
    val list = ArrayList<LnUrlPayRequestData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlPayRequestData(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asLnUrlWithdrawRequestData(data: ReadableMap): LnUrlWithdrawRequestData? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "callback",
                "k1",
                "defaultDescription",
                "minWithdrawable",
                "maxWithdrawable",
            ),
        )
    ) {
        return null
    }
    val callback = data.getString("callback")!!
    val k1 = data.getString("k1")!!
    val defaultDescription = data.getString("defaultDescription")!!
    val minWithdrawable = data.getDouble("minWithdrawable").toULong()
    val maxWithdrawable = data.getDouble("maxWithdrawable").toULong()
    return LnUrlWithdrawRequestData(
        callback,
        k1,
        defaultDescription,
        minWithdrawable,
        maxWithdrawable,
    )
}

fun readableMapOf(lnUrlWithdrawRequestData: LnUrlWithdrawRequestData): ReadableMap {
    return readableMapOf(
        "callback" to lnUrlWithdrawRequestData.callback,
        "k1" to lnUrlWithdrawRequestData.k1,
        "defaultDescription" to lnUrlWithdrawRequestData.defaultDescription,
        "minWithdrawable" to lnUrlWithdrawRequestData.minWithdrawable,
        "maxWithdrawable" to lnUrlWithdrawRequestData.maxWithdrawable,
    )
}

fun asLnUrlWithdrawRequestDataList(arr: ReadableArray): List<LnUrlWithdrawRequestData> {
    val list = ArrayList<LnUrlWithdrawRequestData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlWithdrawRequestData(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asLocaleOverrides(data: ReadableMap): LocaleOverrides? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "locale",
                "symbol",
            ),
        )
    ) {
        return null
    }
    val locale = data.getString("locale")!!
    val spacing = if (hasNonNullKey(data, "spacing")) data.getInt("spacing").toUInt() else null
    val symbol = data.getMap("symbol")?.let { asSymbol(it) }!!
    return LocaleOverrides(
        locale,
        spacing,
        symbol,
    )
}

fun readableMapOf(localeOverrides: LocaleOverrides): ReadableMap {
    return readableMapOf(
        "locale" to localeOverrides.locale,
        "spacing" to localeOverrides.spacing,
        "symbol" to readableMapOf(localeOverrides.symbol),
    )
}

fun asLocaleOverridesList(arr: ReadableArray): List<LocaleOverrides> {
    val list = ArrayList<LocaleOverrides>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLocaleOverrides(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asLocalizedName(data: ReadableMap): LocalizedName? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "locale",
                "name",
            ),
        )
    ) {
        return null
    }
    val locale = data.getString("locale")!!
    val name = data.getString("name")!!
    return LocalizedName(
        locale,
        name,
    )
}

fun readableMapOf(localizedName: LocalizedName): ReadableMap {
    return readableMapOf(
        "locale" to localizedName.locale,
        "name" to localizedName.name,
    )
}

fun asLocalizedNameList(arr: ReadableArray): List<LocalizedName> {
    val list = ArrayList<LocalizedName>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLocalizedName(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asLogEntry(data: ReadableMap): LogEntry? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "line",
                "level",
            ),
        )
    ) {
        return null
    }
    val line = data.getString("line")!!
    val level = data.getString("level")!!
    return LogEntry(
        line,
        level,
    )
}

fun readableMapOf(logEntry: LogEntry): ReadableMap {
    return readableMapOf(
        "line" to logEntry.line,
        "level" to logEntry.level,
    )
}

fun asLogEntryList(arr: ReadableArray): List<LogEntry> {
    val list = ArrayList<LogEntry>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLogEntry(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asLspInformation(data: ReadableMap): LspInformation? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "id",
                "name",
                "widgetUrl",
                "pubkey",
                "host",
                "channelCapacity",
                "targetConf",
                "baseFeeMsat",
                "feeRate",
                "timeLockDelta",
                "minHtlcMsat",
                "lspPubkey",
                "openingFeeParamsList",
            ),
        )
    ) {
        return null
    }
    val id = data.getString("id")!!
    val name = data.getString("name")!!
    val widgetUrl = data.getString("widgetUrl")!!
    val pubkey = data.getString("pubkey")!!
    val host = data.getString("host")!!
    val channelCapacity = data.getDouble("channelCapacity").toLong()
    val targetConf = data.getInt("targetConf")
    val baseFeeMsat = data.getDouble("baseFeeMsat").toLong()
    val feeRate = data.getDouble("feeRate")
    val timeLockDelta = data.getInt("timeLockDelta").toUInt()
    val minHtlcMsat = data.getDouble("minHtlcMsat").toLong()
    val lspPubkey = data.getArray("lspPubkey")?.let { asUByteList(it) }!!
    val openingFeeParamsList = data.getMap("openingFeeParamsList")?.let { asOpeningFeeParamsMenu(it) }!!
    return LspInformation(
        id,
        name,
        widgetUrl,
        pubkey,
        host,
        channelCapacity,
        targetConf,
        baseFeeMsat,
        feeRate,
        timeLockDelta,
        minHtlcMsat,
        lspPubkey,
        openingFeeParamsList,
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
        "lspPubkey" to readableArrayOf(lspInformation.lspPubkey),
        "openingFeeParamsList" to readableMapOf(lspInformation.openingFeeParamsList),
    )
}

fun asLspInformationList(arr: ReadableArray): List<LspInformation> {
    val list = ArrayList<LspInformation>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLspInformation(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asMessageSuccessActionData(data: ReadableMap): MessageSuccessActionData? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "message",
            ),
        )
    ) {
        return null
    }
    val message = data.getString("message")!!
    return MessageSuccessActionData(
        message,
    )
}

fun readableMapOf(messageSuccessActionData: MessageSuccessActionData): ReadableMap {
    return readableMapOf(
        "message" to messageSuccessActionData.message,
    )
}

fun asMessageSuccessActionDataList(arr: ReadableArray): List<MessageSuccessActionData> {
    val list = ArrayList<MessageSuccessActionData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asMessageSuccessActionData(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asMetadataItem(data: ReadableMap): MetadataItem? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "key",
                "value",
            ),
        )
    ) {
        return null
    }
    val key = data.getString("key")!!
    val value = data.getString("value")!!
    return MetadataItem(
        key,
        value,
    )
}

fun readableMapOf(metadataItem: MetadataItem): ReadableMap {
    return readableMapOf(
        "key" to metadataItem.key,
        "value" to metadataItem.value,
    )
}

fun asMetadataItemList(arr: ReadableArray): List<MetadataItem> {
    val list = ArrayList<MetadataItem>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asMetadataItem(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asNodeState(data: ReadableMap): NodeState? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "id",
                "blockHeight",
                "channelsBalanceMsat",
                "onchainBalanceMsat",
                "utxos",
                "maxPayableMsat",
                "maxReceivableMsat",
                "maxSinglePaymentAmountMsat",
                "maxChanReserveMsats",
                "connectedPeers",
                "inboundLiquidityMsats",
            ),
        )
    ) {
        return null
    }
    val id = data.getString("id")!!
    val blockHeight = data.getInt("blockHeight").toUInt()
    val channelsBalanceMsat = data.getDouble("channelsBalanceMsat").toULong()
    val onchainBalanceMsat = data.getDouble("onchainBalanceMsat").toULong()
    val utxos = data.getArray("utxos")?.let { asUnspentTransactionOutputList(it) }!!
    val maxPayableMsat = data.getDouble("maxPayableMsat").toULong()
    val maxReceivableMsat = data.getDouble("maxReceivableMsat").toULong()
    val maxSinglePaymentAmountMsat = data.getDouble("maxSinglePaymentAmountMsat").toULong()
    val maxChanReserveMsats = data.getDouble("maxChanReserveMsats").toULong()
    val connectedPeers = data.getArray("connectedPeers")?.let { asStringList(it) }!!
    val inboundLiquidityMsats = data.getDouble("inboundLiquidityMsats").toULong()
    return NodeState(
        id,
        blockHeight,
        channelsBalanceMsat,
        onchainBalanceMsat,
        utxos,
        maxPayableMsat,
        maxReceivableMsat,
        maxSinglePaymentAmountMsat,
        maxChanReserveMsats,
        connectedPeers,
        inboundLiquidityMsats,
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
        "inboundLiquidityMsats" to nodeState.inboundLiquidityMsats,
    )
}

fun asNodeStateList(arr: ReadableArray): List<NodeState> {
    val list = ArrayList<NodeState>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asNodeState(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asOpenChannelFeeRequest(data: ReadableMap): OpenChannelFeeRequest? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "amountMsat",
            ),
        )
    ) {
        return null
    }
    val amountMsat = data.getDouble("amountMsat").toULong()
    val expiry = if (hasNonNullKey(data, "expiry")) data.getInt("expiry").toUInt() else null
    return OpenChannelFeeRequest(
        amountMsat,
        expiry,
    )
}

fun readableMapOf(openChannelFeeRequest: OpenChannelFeeRequest): ReadableMap {
    return readableMapOf(
        "amountMsat" to openChannelFeeRequest.amountMsat,
        "expiry" to openChannelFeeRequest.expiry,
    )
}

fun asOpenChannelFeeRequestList(arr: ReadableArray): List<OpenChannelFeeRequest> {
    val list = ArrayList<OpenChannelFeeRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asOpenChannelFeeRequest(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asOpenChannelFeeResponse(data: ReadableMap): OpenChannelFeeResponse? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "feeMsat",
            ),
        )
    ) {
        return null
    }
    val feeMsat = data.getDouble("feeMsat").toULong()
    val usedFeeParams = if (hasNonNullKey(data, "usedFeeParams")) data.getMap("usedFeeParams")?.let { asOpeningFeeParams(it) } else null
    return OpenChannelFeeResponse(
        feeMsat,
        usedFeeParams,
    )
}

fun readableMapOf(openChannelFeeResponse: OpenChannelFeeResponse): ReadableMap {
    return readableMapOf(
        "feeMsat" to openChannelFeeResponse.feeMsat,
        "usedFeeParams" to openChannelFeeResponse.usedFeeParams?.let { readableMapOf(it) },
    )
}

fun asOpenChannelFeeResponseList(arr: ReadableArray): List<OpenChannelFeeResponse> {
    val list = ArrayList<OpenChannelFeeResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asOpenChannelFeeResponse(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asOpeningFeeParams(data: ReadableMap): OpeningFeeParams? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "minMsat",
                "proportional",
                "validUntil",
                "maxIdleTime",
                "maxClientToSelfDelay",
                "promise",
            ),
        )
    ) {
        return null
    }
    val minMsat = data.getDouble("minMsat").toULong()
    val proportional = data.getInt("proportional").toUInt()
    val validUntil = data.getString("validUntil")!!
    val maxIdleTime = data.getInt("maxIdleTime").toUInt()
    val maxClientToSelfDelay = data.getInt("maxClientToSelfDelay").toUInt()
    val promise = data.getString("promise")!!
    return OpeningFeeParams(
        minMsat,
        proportional,
        validUntil,
        maxIdleTime,
        maxClientToSelfDelay,
        promise,
    )
}

fun readableMapOf(openingFeeParams: OpeningFeeParams): ReadableMap {
    return readableMapOf(
        "minMsat" to openingFeeParams.minMsat,
        "proportional" to openingFeeParams.proportional,
        "validUntil" to openingFeeParams.validUntil,
        "maxIdleTime" to openingFeeParams.maxIdleTime,
        "maxClientToSelfDelay" to openingFeeParams.maxClientToSelfDelay,
        "promise" to openingFeeParams.promise,
    )
}

fun asOpeningFeeParamsList(arr: ReadableArray): List<OpeningFeeParams> {
    val list = ArrayList<OpeningFeeParams>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asOpeningFeeParams(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asOpeningFeeParamsMenu(data: ReadableMap): OpeningFeeParamsMenu? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "values",
            ),
        )
    ) {
        return null
    }
    val values = data.getArray("values")?.let { asOpeningFeeParamsList(it) }!!
    return OpeningFeeParamsMenu(
        values,
    )
}

fun readableMapOf(openingFeeParamsMenu: OpeningFeeParamsMenu): ReadableMap {
    return readableMapOf(
        "values" to readableArrayOf(openingFeeParamsMenu.values),
    )
}

fun asOpeningFeeParamsMenuList(arr: ReadableArray): List<OpeningFeeParamsMenu> {
    val list = ArrayList<OpeningFeeParamsMenu>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asOpeningFeeParamsMenu(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asPayment(data: ReadableMap): Payment? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "id",
                "paymentType",
                "paymentTime",
                "amountMsat",
                "feeMsat",
                "status",
                "details",
            ),
        )
    ) {
        return null
    }
    val id = data.getString("id")!!
    val paymentType = data.getString("paymentType")?.let { asPaymentType(it) }!!
    val paymentTime = data.getDouble("paymentTime").toLong()
    val amountMsat = data.getDouble("amountMsat").toULong()
    val feeMsat = data.getDouble("feeMsat").toULong()
    val status = data.getString("status")?.let { asPaymentStatus(it) }!!
    val description = if (hasNonNullKey(data, "description")) data.getString("description") else null
    val details = data.getMap("details")?.let { asPaymentDetails(it) }!!
    return Payment(
        id,
        paymentType,
        paymentTime,
        amountMsat,
        feeMsat,
        status,
        description,
        details,
    )
}

fun readableMapOf(payment: Payment): ReadableMap {
    return readableMapOf(
        "id" to payment.id,
        "paymentType" to payment.paymentType.name.lowercase(),
        "paymentTime" to payment.paymentTime,
        "amountMsat" to payment.amountMsat,
        "feeMsat" to payment.feeMsat,
        "status" to payment.status.name.lowercase(),
        "description" to payment.description,
        "details" to readableMapOf(payment.details),
    )
}

fun asPaymentList(arr: ReadableArray): List<Payment> {
    val list = ArrayList<Payment>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPayment(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asPaymentFailedData(data: ReadableMap): PaymentFailedData? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "error",
                "nodeId",
            ),
        )
    ) {
        return null
    }
    val error = data.getString("error")!!
    val nodeId = data.getString("nodeId")!!
    val invoice = if (hasNonNullKey(data, "invoice")) data.getMap("invoice")?.let { asLnInvoice(it) } else null
    return PaymentFailedData(
        error,
        nodeId,
        invoice,
    )
}

fun readableMapOf(paymentFailedData: PaymentFailedData): ReadableMap {
    return readableMapOf(
        "error" to paymentFailedData.error,
        "nodeId" to paymentFailedData.nodeId,
        "invoice" to paymentFailedData.invoice?.let { readableMapOf(it) },
    )
}

fun asPaymentFailedDataList(arr: ReadableArray): List<PaymentFailedData> {
    val list = ArrayList<PaymentFailedData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPaymentFailedData(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asRate(data: ReadableMap): Rate? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "coin",
                "value",
            ),
        )
    ) {
        return null
    }
    val coin = data.getString("coin")!!
    val value = data.getDouble("value")
    return Rate(
        coin,
        value,
    )
}

fun readableMapOf(rate: Rate): ReadableMap {
    return readableMapOf(
        "coin" to rate.coin,
        "value" to rate.value,
    )
}

fun asRateList(arr: ReadableArray): List<Rate> {
    val list = ArrayList<Rate>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRate(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asReceiveOnchainRequest(data: ReadableMap): ReceiveOnchainRequest? {
    if (!validateMandatoryFields(
            data,
            arrayOf(),
        )
    ) {
        return null
    }
    val openingFeeParams =
        if (hasNonNullKey(data, "openingFeeParams")) {
            data.getMap("openingFeeParams")?.let {
                asOpeningFeeParams(it)
            }
        } else {
            null
        }
    return ReceiveOnchainRequest(
        openingFeeParams,
    )
}

fun readableMapOf(receiveOnchainRequest: ReceiveOnchainRequest): ReadableMap {
    return readableMapOf(
        "openingFeeParams" to receiveOnchainRequest.openingFeeParams?.let { readableMapOf(it) },
    )
}

fun asReceiveOnchainRequestList(arr: ReadableArray): List<ReceiveOnchainRequest> {
    val list = ArrayList<ReceiveOnchainRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asReceiveOnchainRequest(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asReceivePaymentRequest(data: ReadableMap): ReceivePaymentRequest? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "amountSats",
                "description",
            ),
        )
    ) {
        return null
    }
    val amountSats = data.getDouble("amountSats").toULong()
    val description = data.getString("description")!!
    val preimage = if (hasNonNullKey(data, "preimage")) data.getArray("preimage")?.let { asUByteList(it) } else null
    val openingFeeParams =
        if (hasNonNullKey(data, "openingFeeParams")) {
            data.getMap("openingFeeParams")?.let {
                asOpeningFeeParams(it)
            }
        } else {
            null
        }
    val useDescriptionHash = if (hasNonNullKey(data, "useDescriptionHash")) data.getBoolean("useDescriptionHash") else null
    val expiry = if (hasNonNullKey(data, "expiry")) data.getInt("expiry").toUInt() else null
    val cltv = if (hasNonNullKey(data, "cltv")) data.getInt("cltv").toUInt() else null
    return ReceivePaymentRequest(
        amountSats,
        description,
        preimage,
        openingFeeParams,
        useDescriptionHash,
        expiry,
        cltv,
    )
}

fun readableMapOf(receivePaymentRequest: ReceivePaymentRequest): ReadableMap {
    return readableMapOf(
        "amountSats" to receivePaymentRequest.amountSats,
        "description" to receivePaymentRequest.description,
        "preimage" to receivePaymentRequest.preimage?.let { readableArrayOf(it) },
        "openingFeeParams" to receivePaymentRequest.openingFeeParams?.let { readableMapOf(it) },
        "useDescriptionHash" to receivePaymentRequest.useDescriptionHash,
        "expiry" to receivePaymentRequest.expiry,
        "cltv" to receivePaymentRequest.cltv,
    )
}

fun asReceivePaymentRequestList(arr: ReadableArray): List<ReceivePaymentRequest> {
    val list = ArrayList<ReceivePaymentRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asReceivePaymentRequest(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asReceivePaymentResponse(data: ReadableMap): ReceivePaymentResponse? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "lnInvoice",
            ),
        )
    ) {
        return null
    }
    val lnInvoice = data.getMap("lnInvoice")?.let { asLnInvoice(it) }!!
    val openingFeeParams =
        if (hasNonNullKey(data, "openingFeeParams")) {
            data.getMap("openingFeeParams")?.let {
                asOpeningFeeParams(it)
            }
        } else {
            null
        }
    val openingFeeMsat = if (hasNonNullKey(data, "openingFeeMsat")) data.getDouble("openingFeeMsat").toULong() else null
    return ReceivePaymentResponse(
        lnInvoice,
        openingFeeParams,
        openingFeeMsat,
    )
}

fun readableMapOf(receivePaymentResponse: ReceivePaymentResponse): ReadableMap {
    return readableMapOf(
        "lnInvoice" to readableMapOf(receivePaymentResponse.lnInvoice),
        "openingFeeParams" to receivePaymentResponse.openingFeeParams?.let { readableMapOf(it) },
        "openingFeeMsat" to receivePaymentResponse.openingFeeMsat,
    )
}

fun asReceivePaymentResponseList(arr: ReadableArray): List<ReceivePaymentResponse> {
    val list = ArrayList<ReceivePaymentResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asReceivePaymentResponse(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asRecommendedFees(data: ReadableMap): RecommendedFees? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "fastestFee",
                "halfHourFee",
                "hourFee",
                "economyFee",
                "minimumFee",
            ),
        )
    ) {
        return null
    }
    val fastestFee = data.getDouble("fastestFee").toULong()
    val halfHourFee = data.getDouble("halfHourFee").toULong()
    val hourFee = data.getDouble("hourFee").toULong()
    val economyFee = data.getDouble("economyFee").toULong()
    val minimumFee = data.getDouble("minimumFee").toULong()
    return RecommendedFees(
        fastestFee,
        halfHourFee,
        hourFee,
        economyFee,
        minimumFee,
    )
}

fun readableMapOf(recommendedFees: RecommendedFees): ReadableMap {
    return readableMapOf(
        "fastestFee" to recommendedFees.fastestFee,
        "halfHourFee" to recommendedFees.halfHourFee,
        "hourFee" to recommendedFees.hourFee,
        "economyFee" to recommendedFees.economyFee,
        "minimumFee" to recommendedFees.minimumFee,
    )
}

fun asRecommendedFeesList(arr: ReadableArray): List<RecommendedFees> {
    val list = ArrayList<RecommendedFees>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRecommendedFees(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asReverseSwapFeesRequest(data: ReadableMap): ReverseSwapFeesRequest? {
    if (!validateMandatoryFields(
            data,
            arrayOf(),
        )
    ) {
        return null
    }
    val sendAmountSat = if (hasNonNullKey(data, "sendAmountSat")) data.getDouble("sendAmountSat").toULong() else null
    return ReverseSwapFeesRequest(
        sendAmountSat,
    )
}

fun readableMapOf(reverseSwapFeesRequest: ReverseSwapFeesRequest): ReadableMap {
    return readableMapOf(
        "sendAmountSat" to reverseSwapFeesRequest.sendAmountSat,
    )
}

fun asReverseSwapFeesRequestList(arr: ReadableArray): List<ReverseSwapFeesRequest> {
    val list = ArrayList<ReverseSwapFeesRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asReverseSwapFeesRequest(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asReverseSwapInfo(data: ReadableMap): ReverseSwapInfo? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "id",
                "claimPubkey",
                "onchainAmountSat",
                "status",
            ),
        )
    ) {
        return null
    }
    val id = data.getString("id")!!
    val claimPubkey = data.getString("claimPubkey")!!
    val onchainAmountSat = data.getDouble("onchainAmountSat").toULong()
    val status = data.getString("status")?.let { asReverseSwapStatus(it) }!!
    return ReverseSwapInfo(
        id,
        claimPubkey,
        onchainAmountSat,
        status,
    )
}

fun readableMapOf(reverseSwapInfo: ReverseSwapInfo): ReadableMap {
    return readableMapOf(
        "id" to reverseSwapInfo.id,
        "claimPubkey" to reverseSwapInfo.claimPubkey,
        "onchainAmountSat" to reverseSwapInfo.onchainAmountSat,
        "status" to reverseSwapInfo.status.name.lowercase(),
    )
}

fun asReverseSwapInfoList(arr: ReadableArray): List<ReverseSwapInfo> {
    val list = ArrayList<ReverseSwapInfo>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asReverseSwapInfo(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asReverseSwapPairInfo(data: ReadableMap): ReverseSwapPairInfo? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "min",
                "max",
                "feesHash",
                "feesPercentage",
                "feesLockup",
                "feesClaim",
            ),
        )
    ) {
        return null
    }
    val min = data.getDouble("min").toULong()
    val max = data.getDouble("max").toULong()
    val feesHash = data.getString("feesHash")!!
    val feesPercentage = data.getDouble("feesPercentage")
    val feesLockup = data.getDouble("feesLockup").toULong()
    val feesClaim = data.getDouble("feesClaim").toULong()
    val totalEstimatedFees = if (hasNonNullKey(data, "totalEstimatedFees")) data.getDouble("totalEstimatedFees").toULong() else null
    return ReverseSwapPairInfo(
        min,
        max,
        feesHash,
        feesPercentage,
        feesLockup,
        feesClaim,
        totalEstimatedFees,
    )
}

fun readableMapOf(reverseSwapPairInfo: ReverseSwapPairInfo): ReadableMap {
    return readableMapOf(
        "min" to reverseSwapPairInfo.min,
        "max" to reverseSwapPairInfo.max,
        "feesHash" to reverseSwapPairInfo.feesHash,
        "feesPercentage" to reverseSwapPairInfo.feesPercentage,
        "feesLockup" to reverseSwapPairInfo.feesLockup,
        "feesClaim" to reverseSwapPairInfo.feesClaim,
        "totalEstimatedFees" to reverseSwapPairInfo.totalEstimatedFees,
    )
}

fun asReverseSwapPairInfoList(arr: ReadableArray): List<ReverseSwapPairInfo> {
    val list = ArrayList<ReverseSwapPairInfo>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asReverseSwapPairInfo(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asRouteHint(data: ReadableMap): RouteHint? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "hops",
            ),
        )
    ) {
        return null
    }
    val hops = data.getArray("hops")?.let { asRouteHintHopList(it) }!!
    return RouteHint(
        hops,
    )
}

fun readableMapOf(routeHint: RouteHint): ReadableMap {
    return readableMapOf(
        "hops" to readableArrayOf(routeHint.hops),
    )
}

fun asRouteHintList(arr: ReadableArray): List<RouteHint> {
    val list = ArrayList<RouteHint>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRouteHint(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asRouteHintHop(data: ReadableMap): RouteHintHop? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "srcNodeId",
                "shortChannelId",
                "feesBaseMsat",
                "feesProportionalMillionths",
                "cltvExpiryDelta",
            ),
        )
    ) {
        return null
    }
    val srcNodeId = data.getString("srcNodeId")!!
    val shortChannelId = data.getDouble("shortChannelId").toULong()
    val feesBaseMsat = data.getInt("feesBaseMsat").toUInt()
    val feesProportionalMillionths = data.getInt("feesProportionalMillionths").toUInt()
    val cltvExpiryDelta = data.getDouble("cltvExpiryDelta").toULong()
    val htlcMinimumMsat = if (hasNonNullKey(data, "htlcMinimumMsat")) data.getDouble("htlcMinimumMsat").toULong() else null
    val htlcMaximumMsat = if (hasNonNullKey(data, "htlcMaximumMsat")) data.getDouble("htlcMaximumMsat").toULong() else null
    return RouteHintHop(
        srcNodeId,
        shortChannelId,
        feesBaseMsat,
        feesProportionalMillionths,
        cltvExpiryDelta,
        htlcMinimumMsat,
        htlcMaximumMsat,
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
        "htlcMaximumMsat" to routeHintHop.htlcMaximumMsat,
    )
}

fun asRouteHintHopList(arr: ReadableArray): List<RouteHintHop> {
    val list = ArrayList<RouteHintHop>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRouteHintHop(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asSignMessageRequest(data: ReadableMap): SignMessageRequest? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "message",
            ),
        )
    ) {
        return null
    }
    val message = data.getString("message")!!
    return SignMessageRequest(
        message,
    )
}

fun readableMapOf(signMessageRequest: SignMessageRequest): ReadableMap {
    return readableMapOf(
        "message" to signMessageRequest.message,
    )
}

fun asSignMessageRequestList(arr: ReadableArray): List<SignMessageRequest> {
    val list = ArrayList<SignMessageRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSignMessageRequest(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asSignMessageResponse(data: ReadableMap): SignMessageResponse? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "signature",
            ),
        )
    ) {
        return null
    }
    val signature = data.getString("signature")!!
    return SignMessageResponse(
        signature,
    )
}

fun readableMapOf(signMessageResponse: SignMessageResponse): ReadableMap {
    return readableMapOf(
        "signature" to signMessageResponse.signature,
    )
}

fun asSignMessageResponseList(arr: ReadableArray): List<SignMessageResponse> {
    val list = ArrayList<SignMessageResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSignMessageResponse(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asStaticBackupRequest(data: ReadableMap): StaticBackupRequest? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "workingDir",
            ),
        )
    ) {
        return null
    }
    val workingDir = data.getString("workingDir")!!
    return StaticBackupRequest(
        workingDir,
    )
}

fun readableMapOf(staticBackupRequest: StaticBackupRequest): ReadableMap {
    return readableMapOf(
        "workingDir" to staticBackupRequest.workingDir,
    )
}

fun asStaticBackupRequestList(arr: ReadableArray): List<StaticBackupRequest> {
    val list = ArrayList<StaticBackupRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asStaticBackupRequest(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asStaticBackupResponse(data: ReadableMap): StaticBackupResponse? {
    if (!validateMandatoryFields(
            data,
            arrayOf(),
        )
    ) {
        return null
    }
    val backup = if (hasNonNullKey(data, "backup")) data.getArray("backup")?.let { asStringList(it) } else null
    return StaticBackupResponse(
        backup,
    )
}

fun readableMapOf(staticBackupResponse: StaticBackupResponse): ReadableMap {
    return readableMapOf(
        "backup" to staticBackupResponse.backup?.let { readableArrayOf(it) },
    )
}

fun asStaticBackupResponseList(arr: ReadableArray): List<StaticBackupResponse> {
    val list = ArrayList<StaticBackupResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asStaticBackupResponse(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asSwapInfo(data: ReadableMap): SwapInfo? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "bitcoinAddress",
                "createdAt",
                "lockHeight",
                "paymentHash",
                "preimage",
                "privateKey",
                "publicKey",
                "swapperPublicKey",
                "script",
                "paidSats",
                "unconfirmedSats",
                "confirmedSats",
                "status",
                "refundTxIds",
                "unconfirmedTxIds",
                "confirmedTxIds",
                "minAllowedDeposit",
                "maxAllowedDeposit",
            ),
        )
    ) {
        return null
    }
    val bitcoinAddress = data.getString("bitcoinAddress")!!
    val createdAt = data.getDouble("createdAt").toLong()
    val lockHeight = data.getDouble("lockHeight").toLong()
    val paymentHash = data.getArray("paymentHash")?.let { asUByteList(it) }!!
    val preimage = data.getArray("preimage")?.let { asUByteList(it) }!!
    val privateKey = data.getArray("privateKey")?.let { asUByteList(it) }!!
    val publicKey = data.getArray("publicKey")?.let { asUByteList(it) }!!
    val swapperPublicKey = data.getArray("swapperPublicKey")?.let { asUByteList(it) }!!
    val script = data.getArray("script")?.let { asUByteList(it) }!!
    val bolt11 = if (hasNonNullKey(data, "bolt11")) data.getString("bolt11") else null
    val paidSats = data.getDouble("paidSats").toULong()
    val unconfirmedSats = data.getDouble("unconfirmedSats").toULong()
    val confirmedSats = data.getDouble("confirmedSats").toULong()
    val status = data.getString("status")?.let { asSwapStatus(it) }!!
    val refundTxIds = data.getArray("refundTxIds")?.let { asStringList(it) }!!
    val unconfirmedTxIds = data.getArray("unconfirmedTxIds")?.let { asStringList(it) }!!
    val confirmedTxIds = data.getArray("confirmedTxIds")?.let { asStringList(it) }!!
    val minAllowedDeposit = data.getDouble("minAllowedDeposit").toLong()
    val maxAllowedDeposit = data.getDouble("maxAllowedDeposit").toLong()
    val lastRedeemError = if (hasNonNullKey(data, "lastRedeemError")) data.getString("lastRedeemError") else null
    val channelOpeningFees =
        if (hasNonNullKey(data, "channelOpeningFees")) {
            data.getMap("channelOpeningFees")?.let {
                asOpeningFeeParams(it)
            }
        } else {
            null
        }
    return SwapInfo(
        bitcoinAddress,
        createdAt,
        lockHeight,
        paymentHash,
        preimage,
        privateKey,
        publicKey,
        swapperPublicKey,
        script,
        bolt11,
        paidSats,
        unconfirmedSats,
        confirmedSats,
        status,
        refundTxIds,
        unconfirmedTxIds,
        confirmedTxIds,
        minAllowedDeposit,
        maxAllowedDeposit,
        lastRedeemError,
        channelOpeningFees,
    )
}

fun readableMapOf(swapInfo: SwapInfo): ReadableMap {
    return readableMapOf(
        "bitcoinAddress" to swapInfo.bitcoinAddress,
        "createdAt" to swapInfo.createdAt,
        "lockHeight" to swapInfo.lockHeight,
        "paymentHash" to readableArrayOf(swapInfo.paymentHash),
        "preimage" to readableArrayOf(swapInfo.preimage),
        "privateKey" to readableArrayOf(swapInfo.privateKey),
        "publicKey" to readableArrayOf(swapInfo.publicKey),
        "swapperPublicKey" to readableArrayOf(swapInfo.swapperPublicKey),
        "script" to readableArrayOf(swapInfo.script),
        "bolt11" to swapInfo.bolt11,
        "paidSats" to swapInfo.paidSats,
        "unconfirmedSats" to swapInfo.unconfirmedSats,
        "confirmedSats" to swapInfo.confirmedSats,
        "status" to swapInfo.status.name.lowercase(),
        "refundTxIds" to readableArrayOf(swapInfo.refundTxIds),
        "unconfirmedTxIds" to readableArrayOf(swapInfo.unconfirmedTxIds),
        "confirmedTxIds" to readableArrayOf(swapInfo.confirmedTxIds),
        "minAllowedDeposit" to swapInfo.minAllowedDeposit,
        "maxAllowedDeposit" to swapInfo.maxAllowedDeposit,
        "lastRedeemError" to swapInfo.lastRedeemError,
        "channelOpeningFees" to swapInfo.channelOpeningFees?.let { readableMapOf(it) },
    )
}

fun asSwapInfoList(arr: ReadableArray): List<SwapInfo> {
    val list = ArrayList<SwapInfo>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSwapInfo(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asSymbol(data: ReadableMap): Symbol? {
    if (!validateMandatoryFields(
            data,
            arrayOf(),
        )
    ) {
        return null
    }
    val grapheme = if (hasNonNullKey(data, "grapheme")) data.getString("grapheme") else null
    val template = if (hasNonNullKey(data, "template")) data.getString("template") else null
    val rtl = if (hasNonNullKey(data, "rtl")) data.getBoolean("rtl") else null
    val position = if (hasNonNullKey(data, "position")) data.getInt("position").toUInt() else null
    return Symbol(
        grapheme,
        template,
        rtl,
        position,
    )
}

fun readableMapOf(symbol: Symbol): ReadableMap {
    return readableMapOf(
        "grapheme" to symbol.grapheme,
        "template" to symbol.template,
        "rtl" to symbol.rtl,
        "position" to symbol.position,
    )
}

fun asSymbolList(arr: ReadableArray): List<Symbol> {
    val list = ArrayList<Symbol>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSymbol(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asUnspentTransactionOutput(data: ReadableMap): UnspentTransactionOutput? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "txid",
                "outnum",
                "amountMillisatoshi",
                "address",
                "reserved",
                "reservedToBlock",
            ),
        )
    ) {
        return null
    }
    val txid = data.getArray("txid")?.let { asUByteList(it) }!!
    val outnum = data.getInt("outnum").toUInt()
    val amountMillisatoshi = data.getDouble("amountMillisatoshi").toULong()
    val address = data.getString("address")!!
    val reserved = data.getBoolean("reserved")
    val reservedToBlock = data.getInt("reservedToBlock").toUInt()
    return UnspentTransactionOutput(
        txid,
        outnum,
        amountMillisatoshi,
        address,
        reserved,
        reservedToBlock,
    )
}

fun readableMapOf(unspentTransactionOutput: UnspentTransactionOutput): ReadableMap {
    return readableMapOf(
        "txid" to readableArrayOf(unspentTransactionOutput.txid),
        "outnum" to unspentTransactionOutput.outnum,
        "amountMillisatoshi" to unspentTransactionOutput.amountMillisatoshi,
        "address" to unspentTransactionOutput.address,
        "reserved" to unspentTransactionOutput.reserved,
        "reservedToBlock" to unspentTransactionOutput.reservedToBlock,
    )
}

fun asUnspentTransactionOutputList(arr: ReadableArray): List<UnspentTransactionOutput> {
    val list = ArrayList<UnspentTransactionOutput>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asUnspentTransactionOutput(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asUrlSuccessActionData(data: ReadableMap): UrlSuccessActionData? {
    if (!validateMandatoryFields(
            data,
            arrayOf(
                "description",
                "url",
            ),
        )
    ) {
        return null
    }
    val description = data.getString("description")!!
    val url = data.getString("url")!!
    return UrlSuccessActionData(
        description,
        url,
    )
}

fun readableMapOf(urlSuccessActionData: UrlSuccessActionData): ReadableMap {
    return readableMapOf(
        "description" to urlSuccessActionData.description,
        "url" to urlSuccessActionData.url,
    )
}

fun asUrlSuccessActionDataList(arr: ReadableArray): List<UrlSuccessActionData> {
    val list = ArrayList<UrlSuccessActionData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asUrlSuccessActionData(value)!!)
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}

fun asBreezEvent(data: ReadableMap): BreezEvent? {
    val type = data.getString("type")

    if (type == "newBlock") {
        return BreezEvent.NewBlock(data.getInt("block").toUInt())
    }
    if (type == "invoicePaid") {
        return BreezEvent.InvoicePaid(data.getMap("details")?.let { asInvoicePaidDetails(it) }!!)
    }
    if (type == "synced") {
        return BreezEvent.Synced
    }
    if (type == "paymentSucceed") {
        return BreezEvent.PaymentSucceed(data.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "paymentFailed") {
        return BreezEvent.PaymentFailed(data.getMap("details")?.let { asPaymentFailedData(it) }!!)
    }
    if (type == "backupStarted") {
        return BreezEvent.BackupStarted
    }
    if (type == "backupSucceeded") {
        return BreezEvent.BackupSucceeded
    }
    if (type == "backupFailed") {
        return BreezEvent.BackupFailed(data.getMap("details")?.let { asBackupFailedData(it) }!!)
    }
    return null
}

fun readableMapOf(breezEvent: BreezEvent): ReadableMap? {
    val map = Arguments.createMap()
    when (breezEvent) {
        is BreezEvent.NewBlock -> {
            pushToMap(map, "type", "newBlock")
            pushToMap(map, "block", breezEvent.block)
        }
        is BreezEvent.InvoicePaid -> {
            pushToMap(map, "type", "invoicePaid")
            pushToMap(map, "details", readableMapOf(breezEvent.details))
        }
        is BreezEvent.Synced -> {
            pushToMap(map, "type", "synced")
        }
        is BreezEvent.PaymentSucceed -> {
            pushToMap(map, "type", "paymentSucceed")
            pushToMap(map, "details", readableMapOf(breezEvent.details))
        }
        is BreezEvent.PaymentFailed -> {
            pushToMap(map, "type", "paymentFailed")
            pushToMap(map, "details", readableMapOf(breezEvent.details))
        }
        is BreezEvent.BackupStarted -> {
            pushToMap(map, "type", "backupStarted")
        }
        is BreezEvent.BackupSucceeded -> {
            pushToMap(map, "type", "backupSucceeded")
        }
        is BreezEvent.BackupFailed -> {
            pushToMap(map, "type", "backupFailed")
            pushToMap(map, "details", readableMapOf(breezEvent.details))
        }
    }
    return map
}

fun asBuyBitcoinProvider(type: String): BuyBitcoinProvider {
    return BuyBitcoinProvider.valueOf(type.uppercase())
}

fun asChannelState(type: String): ChannelState {
    return ChannelState.valueOf(type.uppercase())
}

fun asEnvironmentType(type: String): EnvironmentType {
    return EnvironmentType.valueOf(type.uppercase())
}

fun asFeeratePreset(type: String): FeeratePreset {
    return FeeratePreset.valueOf(type.uppercase())
}

fun asInputType(data: ReadableMap): InputType? {
    val type = data.getString("type")

    if (type == "bitcoinAddress") {
        return InputType.BitcoinAddress(data.getMap("address")?.let { asBitcoinAddressData(it) }!!)
    }
    if (type == "bolt11") {
        return InputType.Bolt11(data.getMap("invoice")?.let { asLnInvoice(it) }!!)
    }
    if (type == "nodeId") {
        return InputType.NodeId(data.getString("nodeId")!!)
    }
    if (type == "url") {
        return InputType.Url(data.getString("url")!!)
    }
    if (type == "lnUrlPay") {
        return InputType.LnUrlPay(data.getMap("data")?.let { asLnUrlPayRequestData(it) }!!)
    }
    if (type == "lnUrlWithdraw") {
        return InputType.LnUrlWithdraw(data.getMap("data")?.let { asLnUrlWithdrawRequestData(it) }!!)
    }
    if (type == "lnUrlAuth") {
        return InputType.LnUrlAuth(data.getMap("data")?.let { asLnUrlAuthRequestData(it) }!!)
    }
    if (type == "lnUrlError") {
        return InputType.LnUrlError(data.getMap("data")?.let { asLnUrlErrorData(it) }!!)
    }
    return null
}

fun readableMapOf(inputType: InputType): ReadableMap? {
    val map = Arguments.createMap()
    when (inputType) {
        is InputType.BitcoinAddress -> {
            pushToMap(map, "type", "bitcoinAddress")
            pushToMap(map, "address", readableMapOf(inputType.address))
        }
        is InputType.Bolt11 -> {
            pushToMap(map, "type", "bolt11")
            pushToMap(map, "invoice", readableMapOf(inputType.invoice))
        }
        is InputType.NodeId -> {
            pushToMap(map, "type", "nodeId")
            pushToMap(map, "nodeId", inputType.nodeId)
        }
        is InputType.Url -> {
            pushToMap(map, "type", "url")
            pushToMap(map, "url", inputType.url)
        }
        is InputType.LnUrlPay -> {
            pushToMap(map, "type", "lnUrlPay")
            pushToMap(map, "data", readableMapOf(inputType.data))
        }
        is InputType.LnUrlWithdraw -> {
            pushToMap(map, "type", "lnUrlWithdraw")
            pushToMap(map, "data", readableMapOf(inputType.data))
        }
        is InputType.LnUrlAuth -> {
            pushToMap(map, "type", "lnUrlAuth")
            pushToMap(map, "data", readableMapOf(inputType.data))
        }
        is InputType.LnUrlError -> {
            pushToMap(map, "type", "lnUrlError")
            pushToMap(map, "data", readableMapOf(inputType.data))
        }
    }
    return map
}

fun asLnUrlCallbackStatus(data: ReadableMap): LnUrlCallbackStatus? {
    val type = data.getString("type")

    if (type == "ok") {
        return LnUrlCallbackStatus.Ok
    }
    if (type == "errorStatus") {
        return LnUrlCallbackStatus.ErrorStatus(data.getMap("data")?.let { asLnUrlErrorData(it) }!!)
    }
    return null
}

fun readableMapOf(lnUrlCallbackStatus: LnUrlCallbackStatus): ReadableMap? {
    val map = Arguments.createMap()
    when (lnUrlCallbackStatus) {
        is LnUrlCallbackStatus.Ok -> {
            pushToMap(map, "type", "ok")
        }
        is LnUrlCallbackStatus.ErrorStatus -> {
            pushToMap(map, "type", "errorStatus")
            pushToMap(map, "data", readableMapOf(lnUrlCallbackStatus.data))
        }
    }
    return map
}

fun asLnUrlPayResult(data: ReadableMap): LnUrlPayResult? {
    val type = data.getString("type")

    if (type == "endpointSuccess") {
        return LnUrlPayResult.EndpointSuccess(
            if (hasNonNullKey(data, "data")) data.getMap("data")?.let { asSuccessActionProcessed(it) } else null,
        )
    }
    if (type == "endpointError") {
        return LnUrlPayResult.EndpointError(data.getMap("data")?.let { asLnUrlErrorData(it) }!!)
    }
    return null
}

fun readableMapOf(lnUrlPayResult: LnUrlPayResult): ReadableMap? {
    val map = Arguments.createMap()
    when (lnUrlPayResult) {
        is LnUrlPayResult.EndpointSuccess -> {
            pushToMap(map, "type", "endpointSuccess")
            pushToMap(map, "data", lnUrlPayResult.data?.let { readableMapOf(it) })
        }
        is LnUrlPayResult.EndpointError -> {
            pushToMap(map, "type", "endpointError")
            pushToMap(map, "data", readableMapOf(lnUrlPayResult.data))
        }
    }
    return map
}

fun asNetwork(type: String): Network {
    return Network.valueOf(type.uppercase())
}

fun asNodeConfig(data: ReadableMap): NodeConfig? {
    val type = data.getString("type")

    if (type == "greenlight") {
        return NodeConfig.Greenlight(data.getMap("config")?.let { asGreenlightNodeConfig(it) }!!)
    }
    return null
}

fun readableMapOf(nodeConfig: NodeConfig): ReadableMap? {
    val map = Arguments.createMap()
    when (nodeConfig) {
        is NodeConfig.Greenlight -> {
            pushToMap(map, "type", "greenlight")
            pushToMap(map, "config", readableMapOf(nodeConfig.config))
        }
    }
    return map
}

fun asPaymentDetails(data: ReadableMap): PaymentDetails? {
    val type = data.getString("type")

    if (type == "ln") {
        return PaymentDetails.Ln(data.getMap("data")?.let { asLnPaymentDetails(it) }!!)
    }
    if (type == "closedChannel") {
        return PaymentDetails.ClosedChannel(data.getMap("data")?.let { asClosedChannelPaymentDetails(it) }!!)
    }
    return null
}

fun readableMapOf(paymentDetails: PaymentDetails): ReadableMap? {
    val map = Arguments.createMap()
    when (paymentDetails) {
        is PaymentDetails.Ln -> {
            pushToMap(map, "type", "ln")
            pushToMap(map, "data", readableMapOf(paymentDetails.data))
        }
        is PaymentDetails.ClosedChannel -> {
            pushToMap(map, "type", "closedChannel")
            pushToMap(map, "data", readableMapOf(paymentDetails.data))
        }
    }
    return map
}

fun asPaymentStatus(type: String): PaymentStatus {
    return PaymentStatus.valueOf(type.uppercase())
}

fun asPaymentType(type: String): PaymentType {
    return PaymentType.valueOf(type.uppercase())
}

fun asPaymentTypeFilter(type: String): PaymentTypeFilter {
    return PaymentTypeFilter.valueOf(type.uppercase())
}

fun asReverseSwapStatus(type: String): ReverseSwapStatus {
    return ReverseSwapStatus.valueOf(type.uppercase())
}

fun asSuccessActionProcessed(data: ReadableMap): SuccessActionProcessed? {
    val type = data.getString("type")

    if (type == "aes") {
        return SuccessActionProcessed.Aes(data.getMap("data")?.let { asAesSuccessActionDataDecrypted(it) }!!)
    }
    if (type == "message") {
        return SuccessActionProcessed.Message(data.getMap("data")?.let { asMessageSuccessActionData(it) }!!)
    }
    if (type == "url") {
        return SuccessActionProcessed.Url(data.getMap("data")?.let { asUrlSuccessActionData(it) }!!)
    }
    return null
}

fun readableMapOf(successActionProcessed: SuccessActionProcessed): ReadableMap? {
    val map = Arguments.createMap()
    when (successActionProcessed) {
        is SuccessActionProcessed.Aes -> {
            pushToMap(map, "type", "aes")
            pushToMap(map, "data", readableMapOf(successActionProcessed.data))
        }
        is SuccessActionProcessed.Message -> {
            pushToMap(map, "type", "message")
            pushToMap(map, "data", readableMapOf(successActionProcessed.data))
        }
        is SuccessActionProcessed.Url -> {
            pushToMap(map, "type", "url")
            pushToMap(map, "data", readableMapOf(successActionProcessed.data))
        }
    }
    return map
}

fun asSwapStatus(type: String): SwapStatus {
    return SwapStatus.valueOf(type.uppercase())
}

fun readableMapOf(vararg values: Pair<String, *>): ReadableMap {
    val map = Arguments.createMap()
    for ((key, value) in values) {
        pushToMap(map, key, value)
    }
    return map
}

fun hasNonNullKey(
    map: ReadableMap,
    key: String,
): Boolean {
    return map.hasKey(key) && !map.isNull(key)
}

fun validateMandatoryFields(
    map: ReadableMap,
    keys: Array<String>,
): Boolean {
    for (k in keys) {
        if (!hasNonNullKey(map, k)) return false
    }

    return true
}

fun pushToArray(
    array: WritableArray,
    value: Any?,
) {
    when (value) {
        null -> array.pushNull()
        is FiatCurrency -> array.pushMap(readableMapOf(value))
        is LocaleOverrides -> array.pushMap(readableMapOf(value))
        is LocalizedName -> array.pushMap(readableMapOf(value))
        is LspInformation -> array.pushMap(readableMapOf(value))
        is OpeningFeeParams -> array.pushMap(readableMapOf(value))
        is Payment -> array.pushMap(readableMapOf(value))
        is Rate -> array.pushMap(readableMapOf(value))
        is ReverseSwapInfo -> array.pushMap(readableMapOf(value))
        is RouteHint -> array.pushMap(readableMapOf(value))
        is RouteHintHop -> array.pushMap(readableMapOf(value))
        is String -> array.pushString(value)
        is SwapInfo -> array.pushMap(readableMapOf(value))
        is UByte -> array.pushInt(value.toInt())
        is UnspentTransactionOutput -> array.pushMap(readableMapOf(value))
        is Array<*> -> array.pushArray(readableArrayOf(value.asIterable()))
        is List<*> -> array.pushArray(readableArrayOf(value))
        else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
    }
}

fun pushToMap(
    map: WritableMap,
    key: String,
    value: Any?,
) {
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

fun asStringList(arr: ReadableArray): List<String> {
    val list = ArrayList<String>()
    for (value in arr.toArrayList()) {
        list.add(value.toString())
    }
    return list
}
