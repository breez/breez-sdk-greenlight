import BreezSDK
import Foundation

class BreezSDKMapper {
    static func asAesSuccessActionDataDecrypted(data: [String: Any?]) throws -> AesSuccessActionDataDecrypted {
        guard let description = data["description"] as? String else { throw SdkError.Generic(message: "Missing mandatory field description for type AesSuccessActionDataDecrypted") }
        guard let plaintext = data["plaintext"] as? String else { throw SdkError.Generic(message: "Missing mandatory field plaintext for type AesSuccessActionDataDecrypted") }

        return AesSuccessActionDataDecrypted(
            description: description,
            plaintext: plaintext
        )
    }

    static func dictionaryOf(aesSuccessActionDataDecrypted: AesSuccessActionDataDecrypted) -> [String: Any?] {
        return [
            "description": aesSuccessActionDataDecrypted.description,
            "plaintext": aesSuccessActionDataDecrypted.plaintext,
        ]
    }

    static func asAesSuccessActionDataDecryptedList(arr: [Any]) throws -> [AesSuccessActionDataDecrypted] {
        var list = [AesSuccessActionDataDecrypted]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var aesSuccessActionDataDecrypted = try asAesSuccessActionDataDecrypted(data: val)
                list.append(aesSuccessActionDataDecrypted)
            } else {
                throw SdkError.Generic(message: "Invalid element type AesSuccessActionDataDecrypted")
            }
        }
        return list
    }

    static func arrayOf(aesSuccessActionDataDecryptedList: [AesSuccessActionDataDecrypted]) -> [Any] {
        return aesSuccessActionDataDecryptedList.map { v -> [String: Any?] in dictionaryOf(aesSuccessActionDataDecrypted: v) }
    }

    static func asBackupFailedData(data: [String: Any?]) throws -> BackupFailedData {
        guard let error = data["error"] as? String else { throw SdkError.Generic(message: "Missing mandatory field error for type BackupFailedData") }

        return BackupFailedData(
            error: error)
    }

    static func dictionaryOf(backupFailedData: BackupFailedData) -> [String: Any?] {
        return [
            "error": backupFailedData.error,
        ]
    }

    static func asBackupFailedDataList(arr: [Any]) throws -> [BackupFailedData] {
        var list = [BackupFailedData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var backupFailedData = try asBackupFailedData(data: val)
                list.append(backupFailedData)
            } else {
                throw SdkError.Generic(message: "Invalid element type BackupFailedData")
            }
        }
        return list
    }

    static func arrayOf(backupFailedDataList: [BackupFailedData]) -> [Any] {
        return backupFailedDataList.map { v -> [String: Any?] in dictionaryOf(backupFailedData: v) }
    }

    static func asBackupStatus(data: [String: Any?]) throws -> BackupStatus {
        guard let backedUp = data["backedUp"] as? Bool else { throw SdkError.Generic(message: "Missing mandatory field backedUp for type BackupStatus") }
        let lastBackupTime = data["lastBackupTime"] as? UInt64

        return BackupStatus(
            backedUp: backedUp,
            lastBackupTime: lastBackupTime
        )
    }

    static func dictionaryOf(backupStatus: BackupStatus) -> [String: Any?] {
        return [
            "backedUp": backupStatus.backedUp,
            "lastBackupTime": backupStatus.lastBackupTime == nil ? nil : backupStatus.lastBackupTime,
        ]
    }

    static func asBackupStatusList(arr: [Any]) throws -> [BackupStatus] {
        var list = [BackupStatus]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var backupStatus = try asBackupStatus(data: val)
                list.append(backupStatus)
            } else {
                throw SdkError.Generic(message: "Invalid element type BackupStatus")
            }
        }
        return list
    }

    static func arrayOf(backupStatusList: [BackupStatus]) -> [Any] {
        return backupStatusList.map { v -> [String: Any?] in dictionaryOf(backupStatus: v) }
    }

    static func asBitcoinAddressData(data: [String: Any?]) throws -> BitcoinAddressData {
        guard let address = data["address"] as? String else { throw SdkError.Generic(message: "Missing mandatory field address for type BitcoinAddressData") }
        guard let networkTmp = data["network"] as? String else { throw SdkError.Generic(message: "Missing mandatory field network for type BitcoinAddressData") }
        let network = try asNetwork(type: networkTmp)

        let amountSat = data["amountSat"] as? UInt64
        let label = data["label"] as? String
        let message = data["message"] as? String

        return BitcoinAddressData(
            address: address,
            network: network,
            amountSat: amountSat,
            label: label,
            message: message
        )
    }

    static func dictionaryOf(bitcoinAddressData: BitcoinAddressData) -> [String: Any?] {
        return [
            "address": bitcoinAddressData.address,
            "network": valueOf(network: bitcoinAddressData.network),
            "amountSat": bitcoinAddressData.amountSat == nil ? nil : bitcoinAddressData.amountSat,
            "label": bitcoinAddressData.label == nil ? nil : bitcoinAddressData.label,
            "message": bitcoinAddressData.message == nil ? nil : bitcoinAddressData.message,
        ]
    }

    static func asBitcoinAddressDataList(arr: [Any]) throws -> [BitcoinAddressData] {
        var list = [BitcoinAddressData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var bitcoinAddressData = try asBitcoinAddressData(data: val)
                list.append(bitcoinAddressData)
            } else {
                throw SdkError.Generic(message: "Invalid element type BitcoinAddressData")
            }
        }
        return list
    }

    static func arrayOf(bitcoinAddressDataList: [BitcoinAddressData]) -> [Any] {
        return bitcoinAddressDataList.map { v -> [String: Any?] in dictionaryOf(bitcoinAddressData: v) }
    }

    static func asBuyBitcoinRequest(data: [String: Any?]) throws -> BuyBitcoinRequest {
        guard let providerTmp = data["provider"] as? String else { throw SdkError.Generic(message: "Missing mandatory field provider for type BuyBitcoinRequest") }
        let provider = try asBuyBitcoinProvider(type: providerTmp)

        var openingFeeParams: OpeningFeeParams?
        if let openingFeeParamsTmp = data["openingFeeParams"] as? [String: Any?] {
            openingFeeParams = try asOpeningFeeParams(data: openingFeeParamsTmp)
        }

        return BuyBitcoinRequest(
            provider: provider,
            openingFeeParams: openingFeeParams
        )
    }

    static func dictionaryOf(buyBitcoinRequest: BuyBitcoinRequest) -> [String: Any?] {
        return [
            "provider": valueOf(buyBitcoinProvider: buyBitcoinRequest.provider),
            "openingFeeParams": buyBitcoinRequest.openingFeeParams == nil ? nil : dictionaryOf(openingFeeParams: buyBitcoinRequest.openingFeeParams!),
        ]
    }

    static func asBuyBitcoinRequestList(arr: [Any]) throws -> [BuyBitcoinRequest] {
        var list = [BuyBitcoinRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var buyBitcoinRequest = try asBuyBitcoinRequest(data: val)
                list.append(buyBitcoinRequest)
            } else {
                throw SdkError.Generic(message: "Invalid element type BuyBitcoinRequest")
            }
        }
        return list
    }

    static func arrayOf(buyBitcoinRequestList: [BuyBitcoinRequest]) -> [Any] {
        return buyBitcoinRequestList.map { v -> [String: Any?] in dictionaryOf(buyBitcoinRequest: v) }
    }

    static func asBuyBitcoinResponse(data: [String: Any?]) throws -> BuyBitcoinResponse {
        guard let url = data["url"] as? String else { throw SdkError.Generic(message: "Missing mandatory field url for type BuyBitcoinResponse") }
        var openingFeeParams: OpeningFeeParams?
        if let openingFeeParamsTmp = data["openingFeeParams"] as? [String: Any?] {
            openingFeeParams = try asOpeningFeeParams(data: openingFeeParamsTmp)
        }

        return BuyBitcoinResponse(
            url: url,
            openingFeeParams: openingFeeParams
        )
    }

    static func dictionaryOf(buyBitcoinResponse: BuyBitcoinResponse) -> [String: Any?] {
        return [
            "url": buyBitcoinResponse.url,
            "openingFeeParams": buyBitcoinResponse.openingFeeParams == nil ? nil : dictionaryOf(openingFeeParams: buyBitcoinResponse.openingFeeParams!),
        ]
    }

    static func asBuyBitcoinResponseList(arr: [Any]) throws -> [BuyBitcoinResponse] {
        var list = [BuyBitcoinResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var buyBitcoinResponse = try asBuyBitcoinResponse(data: val)
                list.append(buyBitcoinResponse)
            } else {
                throw SdkError.Generic(message: "Invalid element type BuyBitcoinResponse")
            }
        }
        return list
    }

    static func arrayOf(buyBitcoinResponseList: [BuyBitcoinResponse]) -> [Any] {
        return buyBitcoinResponseList.map { v -> [String: Any?] in dictionaryOf(buyBitcoinResponse: v) }
    }

    static func asCheckMessageRequest(data: [String: Any?]) throws -> CheckMessageRequest {
        guard let message = data["message"] as? String else { throw SdkError.Generic(message: "Missing mandatory field message for type CheckMessageRequest") }
        guard let pubkey = data["pubkey"] as? String else { throw SdkError.Generic(message: "Missing mandatory field pubkey for type CheckMessageRequest") }
        guard let signature = data["signature"] as? String else { throw SdkError.Generic(message: "Missing mandatory field signature for type CheckMessageRequest") }

        return CheckMessageRequest(
            message: message,
            pubkey: pubkey,
            signature: signature
        )
    }

    static func dictionaryOf(checkMessageRequest: CheckMessageRequest) -> [String: Any?] {
        return [
            "message": checkMessageRequest.message,
            "pubkey": checkMessageRequest.pubkey,
            "signature": checkMessageRequest.signature,
        ]
    }

    static func asCheckMessageRequestList(arr: [Any]) throws -> [CheckMessageRequest] {
        var list = [CheckMessageRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var checkMessageRequest = try asCheckMessageRequest(data: val)
                list.append(checkMessageRequest)
            } else {
                throw SdkError.Generic(message: "Invalid element type CheckMessageRequest")
            }
        }
        return list
    }

    static func arrayOf(checkMessageRequestList: [CheckMessageRequest]) -> [Any] {
        return checkMessageRequestList.map { v -> [String: Any?] in dictionaryOf(checkMessageRequest: v) }
    }

    static func asCheckMessageResponse(data: [String: Any?]) throws -> CheckMessageResponse {
        guard let isValid = data["isValid"] as? Bool else { throw SdkError.Generic(message: "Missing mandatory field isValid for type CheckMessageResponse") }

        return CheckMessageResponse(
            isValid: isValid)
    }

    static func dictionaryOf(checkMessageResponse: CheckMessageResponse) -> [String: Any?] {
        return [
            "isValid": checkMessageResponse.isValid,
        ]
    }

    static func asCheckMessageResponseList(arr: [Any]) throws -> [CheckMessageResponse] {
        var list = [CheckMessageResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var checkMessageResponse = try asCheckMessageResponse(data: val)
                list.append(checkMessageResponse)
            } else {
                throw SdkError.Generic(message: "Invalid element type CheckMessageResponse")
            }
        }
        return list
    }

    static func arrayOf(checkMessageResponseList: [CheckMessageResponse]) -> [Any] {
        return checkMessageResponseList.map { v -> [String: Any?] in dictionaryOf(checkMessageResponse: v) }
    }

    static func asClosedChannelPaymentDetails(data: [String: Any?]) throws -> ClosedChannelPaymentDetails {
        guard let shortChannelId = data["shortChannelId"] as? String else { throw SdkError.Generic(message: "Missing mandatory field shortChannelId for type ClosedChannelPaymentDetails") }
        guard let stateTmp = data["state"] as? String else { throw SdkError.Generic(message: "Missing mandatory field state for type ClosedChannelPaymentDetails") }
        let state = try asChannelState(type: stateTmp)

        guard let fundingTxid = data["fundingTxid"] as? String else { throw SdkError.Generic(message: "Missing mandatory field fundingTxid for type ClosedChannelPaymentDetails") }
        let closingTxid = data["closingTxid"] as? String

        return ClosedChannelPaymentDetails(
            shortChannelId: shortChannelId,
            state: state,
            fundingTxid: fundingTxid,
            closingTxid: closingTxid
        )
    }

    static func dictionaryOf(closedChannelPaymentDetails: ClosedChannelPaymentDetails) -> [String: Any?] {
        return [
            "shortChannelId": closedChannelPaymentDetails.shortChannelId,
            "state": valueOf(channelState: closedChannelPaymentDetails.state),
            "fundingTxid": closedChannelPaymentDetails.fundingTxid,
            "closingTxid": closedChannelPaymentDetails.closingTxid == nil ? nil : closedChannelPaymentDetails.closingTxid,
        ]
    }

    static func asClosedChannelPaymentDetailsList(arr: [Any]) throws -> [ClosedChannelPaymentDetails] {
        var list = [ClosedChannelPaymentDetails]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var closedChannelPaymentDetails = try asClosedChannelPaymentDetails(data: val)
                list.append(closedChannelPaymentDetails)
            } else {
                throw SdkError.Generic(message: "Invalid element type ClosedChannelPaymentDetails")
            }
        }
        return list
    }

    static func arrayOf(closedChannelPaymentDetailsList: [ClosedChannelPaymentDetails]) -> [Any] {
        return closedChannelPaymentDetailsList.map { v -> [String: Any?] in dictionaryOf(closedChannelPaymentDetails: v) }
    }

    static func asConfig(data: [String: Any?]) throws -> Config {
        guard let breezserver = data["breezserver"] as? String else { throw SdkError.Generic(message: "Missing mandatory field breezserver for type Config") }
        guard let mempoolspaceUrl = data["mempoolspaceUrl"] as? String else { throw SdkError.Generic(message: "Missing mandatory field mempoolspaceUrl for type Config") }
        guard let workingDir = data["workingDir"] as? String else { throw SdkError.Generic(message: "Missing mandatory field workingDir for type Config") }
        guard let networkTmp = data["network"] as? String else { throw SdkError.Generic(message: "Missing mandatory field network for type Config") }
        let network = try asNetwork(type: networkTmp)

        guard let paymentTimeoutSec = data["paymentTimeoutSec"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field paymentTimeoutSec for type Config") }
        let defaultLspId = data["defaultLspId"] as? String
        let apiKey = data["apiKey"] as? String
        guard let maxfeePercent = data["maxfeePercent"] as? Double else { throw SdkError.Generic(message: "Missing mandatory field maxfeePercent for type Config") }
        guard let exemptfeeMsat = data["exemptfeeMsat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field exemptfeeMsat for type Config") }
        guard let nodeConfigTmp = data["nodeConfig"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field nodeConfig for type Config") }
        let nodeConfig = try asNodeConfig(data: nodeConfigTmp)

        return Config(
            breezserver: breezserver,
            mempoolspaceUrl: mempoolspaceUrl,
            workingDir: workingDir,
            network: network,
            paymentTimeoutSec: paymentTimeoutSec,
            defaultLspId: defaultLspId,
            apiKey: apiKey,
            maxfeePercent: maxfeePercent,
            exemptfeeMsat: exemptfeeMsat,
            nodeConfig: nodeConfig
        )
    }

    static func dictionaryOf(config: Config) -> [String: Any?] {
        return [
            "breezserver": config.breezserver,
            "mempoolspaceUrl": config.mempoolspaceUrl,
            "workingDir": config.workingDir,
            "network": valueOf(network: config.network),
            "paymentTimeoutSec": config.paymentTimeoutSec,
            "defaultLspId": config.defaultLspId == nil ? nil : config.defaultLspId,
            "apiKey": config.apiKey == nil ? nil : config.apiKey,
            "maxfeePercent": config.maxfeePercent,
            "exemptfeeMsat": config.exemptfeeMsat,
            "nodeConfig": dictionaryOf(nodeConfig: config.nodeConfig),
        ]
    }

    static func asConfigList(arr: [Any]) throws -> [Config] {
        var list = [Config]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var config = try asConfig(data: val)
                list.append(config)
            } else {
                throw SdkError.Generic(message: "Invalid element type Config")
            }
        }
        return list
    }

    static func arrayOf(configList: [Config]) -> [Any] {
        return configList.map { v -> [String: Any?] in dictionaryOf(config: v) }
    }

    static func asCurrencyInfo(data: [String: Any?]) throws -> CurrencyInfo {
        guard let name = data["name"] as? String else { throw SdkError.Generic(message: "Missing mandatory field name for type CurrencyInfo") }
        guard let fractionSize = data["fractionSize"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field fractionSize for type CurrencyInfo") }
        let spacing = data["spacing"] as? UInt32
        var symbol: Symbol?
        if let symbolTmp = data["symbol"] as? [String: Any?] {
            symbol = try asSymbol(data: symbolTmp)
        }

        var uniqSymbol: Symbol?
        if let uniqSymbolTmp = data["uniqSymbol"] as? [String: Any?] {
            uniqSymbol = try asSymbol(data: uniqSymbolTmp)
        }

        var localizedName: [LocalizedName]?
        if let localizedNameTmp = data["localizedName"] as? [[String: Any?]] {
            localizedName = try asLocalizedNameList(arr: localizedNameTmp)
        }

        var localeOverrides: [LocaleOverrides]?
        if let localeOverridesTmp = data["localeOverrides"] as? [[String: Any?]] {
            localeOverrides = try asLocaleOverridesList(arr: localeOverridesTmp)
        }

        return CurrencyInfo(
            name: name,
            fractionSize: fractionSize,
            spacing: spacing,
            symbol: symbol,
            uniqSymbol: uniqSymbol,
            localizedName: localizedName,
            localeOverrides: localeOverrides
        )
    }

    static func dictionaryOf(currencyInfo: CurrencyInfo) -> [String: Any?] {
        return [
            "name": currencyInfo.name,
            "fractionSize": currencyInfo.fractionSize,
            "spacing": currencyInfo.spacing == nil ? nil : currencyInfo.spacing,
            "symbol": currencyInfo.symbol == nil ? nil : dictionaryOf(symbol: currencyInfo.symbol!),
            "uniqSymbol": currencyInfo.uniqSymbol == nil ? nil : dictionaryOf(symbol: currencyInfo.uniqSymbol!),
            "localizedName": currencyInfo.localizedName == nil ? nil : arrayOf(localizedNameList: currencyInfo.localizedName!),
            "localeOverrides": currencyInfo.localeOverrides == nil ? nil : arrayOf(localeOverridesList: currencyInfo.localeOverrides!),
        ]
    }

    static func asCurrencyInfoList(arr: [Any]) throws -> [CurrencyInfo] {
        var list = [CurrencyInfo]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var currencyInfo = try asCurrencyInfo(data: val)
                list.append(currencyInfo)
            } else {
                throw SdkError.Generic(message: "Invalid element type CurrencyInfo")
            }
        }
        return list
    }

    static func arrayOf(currencyInfoList: [CurrencyInfo]) -> [Any] {
        return currencyInfoList.map { v -> [String: Any?] in dictionaryOf(currencyInfo: v) }
    }

    static func asFiatCurrency(data: [String: Any?]) throws -> FiatCurrency {
        guard let id = data["id"] as? String else { throw SdkError.Generic(message: "Missing mandatory field id for type FiatCurrency") }
        guard let infoTmp = data["info"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field info for type FiatCurrency") }
        let info = try asCurrencyInfo(data: infoTmp)

        return FiatCurrency(
            id: id,
            info: info
        )
    }

    static func dictionaryOf(fiatCurrency: FiatCurrency) -> [String: Any?] {
        return [
            "id": fiatCurrency.id,
            "info": dictionaryOf(currencyInfo: fiatCurrency.info),
        ]
    }

    static func asFiatCurrencyList(arr: [Any]) throws -> [FiatCurrency] {
        var list = [FiatCurrency]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var fiatCurrency = try asFiatCurrency(data: val)
                list.append(fiatCurrency)
            } else {
                throw SdkError.Generic(message: "Invalid element type FiatCurrency")
            }
        }
        return list
    }

    static func arrayOf(fiatCurrencyList: [FiatCurrency]) -> [Any] {
        return fiatCurrencyList.map { v -> [String: Any?] in dictionaryOf(fiatCurrency: v) }
    }

    static func asGreenlightCredentials(data: [String: Any?]) throws -> GreenlightCredentials {
        guard let deviceKey = data["deviceKey"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field deviceKey for type GreenlightCredentials") }
        guard let deviceCert = data["deviceCert"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field deviceCert for type GreenlightCredentials") }

        return GreenlightCredentials(
            deviceKey: deviceKey,
            deviceCert: deviceCert
        )
    }

    static func dictionaryOf(greenlightCredentials: GreenlightCredentials) -> [String: Any?] {
        return [
            "deviceKey": greenlightCredentials.deviceKey,
            "deviceCert": greenlightCredentials.deviceCert,
        ]
    }

    static func asGreenlightCredentialsList(arr: [Any]) throws -> [GreenlightCredentials] {
        var list = [GreenlightCredentials]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var greenlightCredentials = try asGreenlightCredentials(data: val)
                list.append(greenlightCredentials)
            } else {
                throw SdkError.Generic(message: "Invalid element type GreenlightCredentials")
            }
        }
        return list
    }

    static func arrayOf(greenlightCredentialsList: [GreenlightCredentials]) -> [Any] {
        return greenlightCredentialsList.map { v -> [String: Any?] in dictionaryOf(greenlightCredentials: v) }
    }

    static func asGreenlightNodeConfig(data: [String: Any?]) throws -> GreenlightNodeConfig {
        var partnerCredentials: GreenlightCredentials?
        if let partnerCredentialsTmp = data["partnerCredentials"] as? [String: Any?] {
            partnerCredentials = try asGreenlightCredentials(data: partnerCredentialsTmp)
        }

        let inviteCode = data["inviteCode"] as? String

        return GreenlightNodeConfig(
            partnerCredentials: partnerCredentials,
            inviteCode: inviteCode
        )
    }

    static func dictionaryOf(greenlightNodeConfig: GreenlightNodeConfig) -> [String: Any?] {
        return [
            "partnerCredentials": greenlightNodeConfig.partnerCredentials == nil ? nil : dictionaryOf(greenlightCredentials: greenlightNodeConfig.partnerCredentials!),
            "inviteCode": greenlightNodeConfig.inviteCode == nil ? nil : greenlightNodeConfig.inviteCode,
        ]
    }

    static func asGreenlightNodeConfigList(arr: [Any]) throws -> [GreenlightNodeConfig] {
        var list = [GreenlightNodeConfig]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var greenlightNodeConfig = try asGreenlightNodeConfig(data: val)
                list.append(greenlightNodeConfig)
            } else {
                throw SdkError.Generic(message: "Invalid element type GreenlightNodeConfig")
            }
        }
        return list
    }

    static func arrayOf(greenlightNodeConfigList: [GreenlightNodeConfig]) -> [Any] {
        return greenlightNodeConfigList.map { v -> [String: Any?] in dictionaryOf(greenlightNodeConfig: v) }
    }

    static func asInvoicePaidDetails(data: [String: Any?]) throws -> InvoicePaidDetails {
        guard let paymentHash = data["paymentHash"] as? String else { throw SdkError.Generic(message: "Missing mandatory field paymentHash for type InvoicePaidDetails") }
        guard let bolt11 = data["bolt11"] as? String else { throw SdkError.Generic(message: "Missing mandatory field bolt11 for type InvoicePaidDetails") }

        return InvoicePaidDetails(
            paymentHash: paymentHash,
            bolt11: bolt11
        )
    }

    static func dictionaryOf(invoicePaidDetails: InvoicePaidDetails) -> [String: Any?] {
        return [
            "paymentHash": invoicePaidDetails.paymentHash,
            "bolt11": invoicePaidDetails.bolt11,
        ]
    }

    static func asInvoicePaidDetailsList(arr: [Any]) throws -> [InvoicePaidDetails] {
        var list = [InvoicePaidDetails]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var invoicePaidDetails = try asInvoicePaidDetails(data: val)
                list.append(invoicePaidDetails)
            } else {
                throw SdkError.Generic(message: "Invalid element type InvoicePaidDetails")
            }
        }
        return list
    }

    static func arrayOf(invoicePaidDetailsList: [InvoicePaidDetails]) -> [Any] {
        return invoicePaidDetailsList.map { v -> [String: Any?] in dictionaryOf(invoicePaidDetails: v) }
    }

    static func asLnInvoice(data: [String: Any?]) throws -> LnInvoice {
        guard let bolt11 = data["bolt11"] as? String else { throw SdkError.Generic(message: "Missing mandatory field bolt11 for type LnInvoice") }
        guard let payeePubkey = data["payeePubkey"] as? String else { throw SdkError.Generic(message: "Missing mandatory field payeePubkey for type LnInvoice") }
        guard let paymentHash = data["paymentHash"] as? String else { throw SdkError.Generic(message: "Missing mandatory field paymentHash for type LnInvoice") }
        let description = data["description"] as? String
        let descriptionHash = data["descriptionHash"] as? String
        let amountMsat = data["amountMsat"] as? UInt64
        guard let timestamp = data["timestamp"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field timestamp for type LnInvoice") }
        guard let expiry = data["expiry"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field expiry for type LnInvoice") }
        guard let routingHintsTmp = data["routingHints"] as? [[String: Any?]] else { throw SdkError.Generic(message: "Missing mandatory field routingHints for type LnInvoice") }
        let routingHints = try asRouteHintList(arr: routingHintsTmp)

        guard let paymentSecret = data["paymentSecret"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field paymentSecret for type LnInvoice") }

        return LnInvoice(
            bolt11: bolt11,
            payeePubkey: payeePubkey,
            paymentHash: paymentHash,
            description: description,
            descriptionHash: descriptionHash,
            amountMsat: amountMsat,
            timestamp: timestamp,
            expiry: expiry,
            routingHints: routingHints,
            paymentSecret: paymentSecret
        )
    }

    static func dictionaryOf(lnInvoice: LnInvoice) -> [String: Any?] {
        return [
            "bolt11": lnInvoice.bolt11,
            "payeePubkey": lnInvoice.payeePubkey,
            "paymentHash": lnInvoice.paymentHash,
            "description": lnInvoice.description == nil ? nil : lnInvoice.description,
            "descriptionHash": lnInvoice.descriptionHash == nil ? nil : lnInvoice.descriptionHash,
            "amountMsat": lnInvoice.amountMsat == nil ? nil : lnInvoice.amountMsat,
            "timestamp": lnInvoice.timestamp,
            "expiry": lnInvoice.expiry,
            "routingHints": arrayOf(routeHintList: lnInvoice.routingHints),
            "paymentSecret": lnInvoice.paymentSecret,
        ]
    }

    static func asLnInvoiceList(arr: [Any]) throws -> [LnInvoice] {
        var list = [LnInvoice]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnInvoice = try asLnInvoice(data: val)
                list.append(lnInvoice)
            } else {
                throw SdkError.Generic(message: "Invalid element type LnInvoice")
            }
        }
        return list
    }

    static func arrayOf(lnInvoiceList: [LnInvoice]) -> [Any] {
        return lnInvoiceList.map { v -> [String: Any?] in dictionaryOf(lnInvoice: v) }
    }

    static func asListPaymentsRequest(data: [String: Any?]) throws -> ListPaymentsRequest {
        guard let filterTmp = data["filter"] as? String else { throw SdkError.Generic(message: "Missing mandatory field filter for type ListPaymentsRequest") }
        let filter = try asPaymentTypeFilter(type: filterTmp)

        let fromTimestamp = data["fromTimestamp"] as? Int64
        let toTimestamp = data["toTimestamp"] as? Int64
        let includeFailures = data["includeFailures"] as? Bool
        let offset = data["offset"] as? UInt32
        let limit = data["limit"] as? UInt32

        return ListPaymentsRequest(
            filter: filter,
            fromTimestamp: fromTimestamp,
            toTimestamp: toTimestamp,
            includeFailures: includeFailures,
            offset: offset,
            limit: limit
        )
    }

    static func dictionaryOf(listPaymentsRequest: ListPaymentsRequest) -> [String: Any?] {
        return [
            "filter": valueOf(paymentTypeFilter: listPaymentsRequest.filter),
            "fromTimestamp": listPaymentsRequest.fromTimestamp == nil ? nil : listPaymentsRequest.fromTimestamp,
            "toTimestamp": listPaymentsRequest.toTimestamp == nil ? nil : listPaymentsRequest.toTimestamp,
            "includeFailures": listPaymentsRequest.includeFailures == nil ? nil : listPaymentsRequest.includeFailures,
            "offset": listPaymentsRequest.offset == nil ? nil : listPaymentsRequest.offset,
            "limit": listPaymentsRequest.limit == nil ? nil : listPaymentsRequest.limit,
        ]
    }

    static func asListPaymentsRequestList(arr: [Any]) throws -> [ListPaymentsRequest] {
        var list = [ListPaymentsRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var listPaymentsRequest = try asListPaymentsRequest(data: val)
                list.append(listPaymentsRequest)
            } else {
                throw SdkError.Generic(message: "Invalid element type ListPaymentsRequest")
            }
        }
        return list
    }

    static func arrayOf(listPaymentsRequestList: [ListPaymentsRequest]) -> [Any] {
        return listPaymentsRequestList.map { v -> [String: Any?] in dictionaryOf(listPaymentsRequest: v) }
    }

    static func asLnPaymentDetails(data: [String: Any?]) throws -> LnPaymentDetails {
        guard let paymentHash = data["paymentHash"] as? String else { throw SdkError.Generic(message: "Missing mandatory field paymentHash for type LnPaymentDetails") }
        guard let label = data["label"] as? String else { throw SdkError.Generic(message: "Missing mandatory field label for type LnPaymentDetails") }
        guard let destinationPubkey = data["destinationPubkey"] as? String else { throw SdkError.Generic(message: "Missing mandatory field destinationPubkey for type LnPaymentDetails") }
        guard let paymentPreimage = data["paymentPreimage"] as? String else { throw SdkError.Generic(message: "Missing mandatory field paymentPreimage for type LnPaymentDetails") }
        guard let keysend = data["keysend"] as? Bool else { throw SdkError.Generic(message: "Missing mandatory field keysend for type LnPaymentDetails") }
        guard let bolt11 = data["bolt11"] as? String else { throw SdkError.Generic(message: "Missing mandatory field bolt11 for type LnPaymentDetails") }
        var lnurlSuccessAction: SuccessActionProcessed?
        if let lnurlSuccessActionTmp = data["lnurlSuccessAction"] as? [String: Any?] {
            lnurlSuccessAction = try asSuccessActionProcessed(data: lnurlSuccessActionTmp)
        }

        let lnurlMetadata = data["lnurlMetadata"] as? String
        let lnAddress = data["lnAddress"] as? String
        let lnurlWithdrawEndpoint = data["lnurlWithdrawEndpoint"] as? String

        return LnPaymentDetails(
            paymentHash: paymentHash,
            label: label,
            destinationPubkey: destinationPubkey,
            paymentPreimage: paymentPreimage,
            keysend: keysend,
            bolt11: bolt11,
            lnurlSuccessAction: lnurlSuccessAction,
            lnurlMetadata: lnurlMetadata,
            lnAddress: lnAddress,
            lnurlWithdrawEndpoint: lnurlWithdrawEndpoint
        )
    }

    static func dictionaryOf(lnPaymentDetails: LnPaymentDetails) -> [String: Any?] {
        return [
            "paymentHash": lnPaymentDetails.paymentHash,
            "label": lnPaymentDetails.label,
            "destinationPubkey": lnPaymentDetails.destinationPubkey,
            "paymentPreimage": lnPaymentDetails.paymentPreimage,
            "keysend": lnPaymentDetails.keysend,
            "bolt11": lnPaymentDetails.bolt11,
            "lnurlSuccessAction": lnPaymentDetails.lnurlSuccessAction == nil ? nil : dictionaryOf(successActionProcessed: lnPaymentDetails.lnurlSuccessAction!),
            "lnurlMetadata": lnPaymentDetails.lnurlMetadata == nil ? nil : lnPaymentDetails.lnurlMetadata,
            "lnAddress": lnPaymentDetails.lnAddress == nil ? nil : lnPaymentDetails.lnAddress,
            "lnurlWithdrawEndpoint": lnPaymentDetails.lnurlWithdrawEndpoint == nil ? nil : lnPaymentDetails.lnurlWithdrawEndpoint,
        ]
    }

    static func asLnPaymentDetailsList(arr: [Any]) throws -> [LnPaymentDetails] {
        var list = [LnPaymentDetails]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnPaymentDetails = try asLnPaymentDetails(data: val)
                list.append(lnPaymentDetails)
            } else {
                throw SdkError.Generic(message: "Invalid element type LnPaymentDetails")
            }
        }
        return list
    }

    static func arrayOf(lnPaymentDetailsList: [LnPaymentDetails]) -> [Any] {
        return lnPaymentDetailsList.map { v -> [String: Any?] in dictionaryOf(lnPaymentDetails: v) }
    }

    static func asLnUrlAuthRequestData(data: [String: Any?]) throws -> LnUrlAuthRequestData {
        guard let k1 = data["k1"] as? String else { throw SdkError.Generic(message: "Missing mandatory field k1 for type LnUrlAuthRequestData") }
        let action = data["action"] as? String
        guard let domain = data["domain"] as? String else { throw SdkError.Generic(message: "Missing mandatory field domain for type LnUrlAuthRequestData") }
        guard let url = data["url"] as? String else { throw SdkError.Generic(message: "Missing mandatory field url for type LnUrlAuthRequestData") }

        return LnUrlAuthRequestData(
            k1: k1,
            action: action,
            domain: domain,
            url: url
        )
    }

    static func dictionaryOf(lnUrlAuthRequestData: LnUrlAuthRequestData) -> [String: Any?] {
        return [
            "k1": lnUrlAuthRequestData.k1,
            "action": lnUrlAuthRequestData.action == nil ? nil : lnUrlAuthRequestData.action,
            "domain": lnUrlAuthRequestData.domain,
            "url": lnUrlAuthRequestData.url,
        ]
    }

    static func asLnUrlAuthRequestDataList(arr: [Any]) throws -> [LnUrlAuthRequestData] {
        var list = [LnUrlAuthRequestData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlAuthRequestData = try asLnUrlAuthRequestData(data: val)
                list.append(lnUrlAuthRequestData)
            } else {
                throw SdkError.Generic(message: "Invalid element type LnUrlAuthRequestData")
            }
        }
        return list
    }

    static func arrayOf(lnUrlAuthRequestDataList: [LnUrlAuthRequestData]) -> [Any] {
        return lnUrlAuthRequestDataList.map { v -> [String: Any?] in dictionaryOf(lnUrlAuthRequestData: v) }
    }

    static func asLnUrlErrorData(data: [String: Any?]) throws -> LnUrlErrorData {
        guard let reason = data["reason"] as? String else { throw SdkError.Generic(message: "Missing mandatory field reason for type LnUrlErrorData") }

        return LnUrlErrorData(
            reason: reason)
    }

    static func dictionaryOf(lnUrlErrorData: LnUrlErrorData) -> [String: Any?] {
        return [
            "reason": lnUrlErrorData.reason,
        ]
    }

    static func asLnUrlErrorDataList(arr: [Any]) throws -> [LnUrlErrorData] {
        var list = [LnUrlErrorData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlErrorData = try asLnUrlErrorData(data: val)
                list.append(lnUrlErrorData)
            } else {
                throw SdkError.Generic(message: "Invalid element type LnUrlErrorData")
            }
        }
        return list
    }

    static func arrayOf(lnUrlErrorDataList: [LnUrlErrorData]) -> [Any] {
        return lnUrlErrorDataList.map { v -> [String: Any?] in dictionaryOf(lnUrlErrorData: v) }
    }

    static func asLnUrlPayRequestData(data: [String: Any?]) throws -> LnUrlPayRequestData {
        guard let callback = data["callback"] as? String else { throw SdkError.Generic(message: "Missing mandatory field callback for type LnUrlPayRequestData") }
        guard let minSendable = data["minSendable"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field minSendable for type LnUrlPayRequestData") }
        guard let maxSendable = data["maxSendable"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field maxSendable for type LnUrlPayRequestData") }
        guard let metadataStr = data["metadataStr"] as? String else { throw SdkError.Generic(message: "Missing mandatory field metadataStr for type LnUrlPayRequestData") }
        guard let commentAllowed = data["commentAllowed"] as? UInt16 else { throw SdkError.Generic(message: "Missing mandatory field commentAllowed for type LnUrlPayRequestData") }
        guard let domain = data["domain"] as? String else { throw SdkError.Generic(message: "Missing mandatory field domain for type LnUrlPayRequestData") }
        let lnAddress = data["lnAddress"] as? String

        return LnUrlPayRequestData(
            callback: callback,
            minSendable: minSendable,
            maxSendable: maxSendable,
            metadataStr: metadataStr,
            commentAllowed: commentAllowed,
            domain: domain,
            lnAddress: lnAddress
        )
    }

    static func dictionaryOf(lnUrlPayRequestData: LnUrlPayRequestData) -> [String: Any?] {
        return [
            "callback": lnUrlPayRequestData.callback,
            "minSendable": lnUrlPayRequestData.minSendable,
            "maxSendable": lnUrlPayRequestData.maxSendable,
            "metadataStr": lnUrlPayRequestData.metadataStr,
            "commentAllowed": lnUrlPayRequestData.commentAllowed,
            "domain": lnUrlPayRequestData.domain,
            "lnAddress": lnUrlPayRequestData.lnAddress == nil ? nil : lnUrlPayRequestData.lnAddress,
        ]
    }

    static func asLnUrlPayRequestDataList(arr: [Any]) throws -> [LnUrlPayRequestData] {
        var list = [LnUrlPayRequestData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlPayRequestData = try asLnUrlPayRequestData(data: val)
                list.append(lnUrlPayRequestData)
            } else {
                throw SdkError.Generic(message: "Invalid element type LnUrlPayRequestData")
            }
        }
        return list
    }

    static func arrayOf(lnUrlPayRequestDataList: [LnUrlPayRequestData]) -> [Any] {
        return lnUrlPayRequestDataList.map { v -> [String: Any?] in dictionaryOf(lnUrlPayRequestData: v) }
    }

    static func asLnUrlWithdrawRequestData(data: [String: Any?]) throws -> LnUrlWithdrawRequestData {
        guard let callback = data["callback"] as? String else { throw SdkError.Generic(message: "Missing mandatory field callback for type LnUrlWithdrawRequestData") }
        guard let k1 = data["k1"] as? String else { throw SdkError.Generic(message: "Missing mandatory field k1 for type LnUrlWithdrawRequestData") }
        guard let defaultDescription = data["defaultDescription"] as? String else { throw SdkError.Generic(message: "Missing mandatory field defaultDescription for type LnUrlWithdrawRequestData") }
        guard let minWithdrawable = data["minWithdrawable"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field minWithdrawable for type LnUrlWithdrawRequestData") }
        guard let maxWithdrawable = data["maxWithdrawable"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field maxWithdrawable for type LnUrlWithdrawRequestData") }

        return LnUrlWithdrawRequestData(
            callback: callback,
            k1: k1,
            defaultDescription: defaultDescription,
            minWithdrawable: minWithdrawable,
            maxWithdrawable: maxWithdrawable
        )
    }

    static func dictionaryOf(lnUrlWithdrawRequestData: LnUrlWithdrawRequestData) -> [String: Any?] {
        return [
            "callback": lnUrlWithdrawRequestData.callback,
            "k1": lnUrlWithdrawRequestData.k1,
            "defaultDescription": lnUrlWithdrawRequestData.defaultDescription,
            "minWithdrawable": lnUrlWithdrawRequestData.minWithdrawable,
            "maxWithdrawable": lnUrlWithdrawRequestData.maxWithdrawable,
        ]
    }

    static func asLnUrlWithdrawRequestDataList(arr: [Any]) throws -> [LnUrlWithdrawRequestData] {
        var list = [LnUrlWithdrawRequestData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlWithdrawRequestData = try asLnUrlWithdrawRequestData(data: val)
                list.append(lnUrlWithdrawRequestData)
            } else {
                throw SdkError.Generic(message: "Invalid element type LnUrlWithdrawRequestData")
            }
        }
        return list
    }

    static func arrayOf(lnUrlWithdrawRequestDataList: [LnUrlWithdrawRequestData]) -> [Any] {
        return lnUrlWithdrawRequestDataList.map { v -> [String: Any?] in dictionaryOf(lnUrlWithdrawRequestData: v) }
    }

    static func asLnUrlWithdrawSuccessData(data: [String: Any?]) throws -> LnUrlWithdrawSuccessData {
        guard let invoiceTmp = data["invoice"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field invoice for type LnUrlWithdrawSuccessData") }
        let invoice = try asLnInvoice(data: invoiceTmp)

        return LnUrlWithdrawSuccessData(
            invoice: invoice)
    }

    static func dictionaryOf(lnUrlWithdrawSuccessData: LnUrlWithdrawSuccessData) -> [String: Any?] {
        return [
            "invoice": dictionaryOf(lnInvoice: lnUrlWithdrawSuccessData.invoice),
        ]
    }

    static func asLnUrlWithdrawSuccessDataList(arr: [Any]) throws -> [LnUrlWithdrawSuccessData] {
        var list = [LnUrlWithdrawSuccessData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlWithdrawSuccessData = try asLnUrlWithdrawSuccessData(data: val)
                list.append(lnUrlWithdrawSuccessData)
            } else {
                throw SdkError.Generic(message: "Invalid element type LnUrlWithdrawSuccessData")
            }
        }
        return list
    }

    static func arrayOf(lnUrlWithdrawSuccessDataList: [LnUrlWithdrawSuccessData]) -> [Any] {
        return lnUrlWithdrawSuccessDataList.map { v -> [String: Any?] in dictionaryOf(lnUrlWithdrawSuccessData: v) }
    }

    static func asLocaleOverrides(data: [String: Any?]) throws -> LocaleOverrides {
        guard let locale = data["locale"] as? String else { throw SdkError.Generic(message: "Missing mandatory field locale for type LocaleOverrides") }
        let spacing = data["spacing"] as? UInt32
        guard let symbolTmp = data["symbol"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field symbol for type LocaleOverrides") }
        let symbol = try asSymbol(data: symbolTmp)

        return LocaleOverrides(
            locale: locale,
            spacing: spacing,
            symbol: symbol
        )
    }

    static func dictionaryOf(localeOverrides: LocaleOverrides) -> [String: Any?] {
        return [
            "locale": localeOverrides.locale,
            "spacing": localeOverrides.spacing == nil ? nil : localeOverrides.spacing,
            "symbol": dictionaryOf(symbol: localeOverrides.symbol),
        ]
    }

    static func asLocaleOverridesList(arr: [Any]) throws -> [LocaleOverrides] {
        var list = [LocaleOverrides]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var localeOverrides = try asLocaleOverrides(data: val)
                list.append(localeOverrides)
            } else {
                throw SdkError.Generic(message: "Invalid element type LocaleOverrides")
            }
        }
        return list
    }

    static func arrayOf(localeOverridesList: [LocaleOverrides]) -> [Any] {
        return localeOverridesList.map { v -> [String: Any?] in dictionaryOf(localeOverrides: v) }
    }

    static func asLocalizedName(data: [String: Any?]) throws -> LocalizedName {
        guard let locale = data["locale"] as? String else { throw SdkError.Generic(message: "Missing mandatory field locale for type LocalizedName") }
        guard let name = data["name"] as? String else { throw SdkError.Generic(message: "Missing mandatory field name for type LocalizedName") }

        return LocalizedName(
            locale: locale,
            name: name
        )
    }

    static func dictionaryOf(localizedName: LocalizedName) -> [String: Any?] {
        return [
            "locale": localizedName.locale,
            "name": localizedName.name,
        ]
    }

    static func asLocalizedNameList(arr: [Any]) throws -> [LocalizedName] {
        var list = [LocalizedName]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var localizedName = try asLocalizedName(data: val)
                list.append(localizedName)
            } else {
                throw SdkError.Generic(message: "Invalid element type LocalizedName")
            }
        }
        return list
    }

    static func arrayOf(localizedNameList: [LocalizedName]) -> [Any] {
        return localizedNameList.map { v -> [String: Any?] in dictionaryOf(localizedName: v) }
    }

    static func asLogEntry(data: [String: Any?]) throws -> LogEntry {
        guard let line = data["line"] as? String else { throw SdkError.Generic(message: "Missing mandatory field line for type LogEntry") }
        guard let level = data["level"] as? String else { throw SdkError.Generic(message: "Missing mandatory field level for type LogEntry") }

        return LogEntry(
            line: line,
            level: level
        )
    }

    static func dictionaryOf(logEntry: LogEntry) -> [String: Any?] {
        return [
            "line": logEntry.line,
            "level": logEntry.level,
        ]
    }

    static func asLogEntryList(arr: [Any]) throws -> [LogEntry] {
        var list = [LogEntry]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var logEntry = try asLogEntry(data: val)
                list.append(logEntry)
            } else {
                throw SdkError.Generic(message: "Invalid element type LogEntry")
            }
        }
        return list
    }

    static func arrayOf(logEntryList: [LogEntry]) -> [Any] {
        return logEntryList.map { v -> [String: Any?] in dictionaryOf(logEntry: v) }
    }

    static func asLspInformation(data: [String: Any?]) throws -> LspInformation {
        guard let id = data["id"] as? String else { throw SdkError.Generic(message: "Missing mandatory field id for type LspInformation") }
        guard let name = data["name"] as? String else { throw SdkError.Generic(message: "Missing mandatory field name for type LspInformation") }
        guard let widgetUrl = data["widgetUrl"] as? String else { throw SdkError.Generic(message: "Missing mandatory field widgetUrl for type LspInformation") }
        guard let pubkey = data["pubkey"] as? String else { throw SdkError.Generic(message: "Missing mandatory field pubkey for type LspInformation") }
        guard let host = data["host"] as? String else { throw SdkError.Generic(message: "Missing mandatory field host for type LspInformation") }
        guard let channelCapacity = data["channelCapacity"] as? Int64 else { throw SdkError.Generic(message: "Missing mandatory field channelCapacity for type LspInformation") }
        guard let targetConf = data["targetConf"] as? Int32 else { throw SdkError.Generic(message: "Missing mandatory field targetConf for type LspInformation") }
        guard let baseFeeMsat = data["baseFeeMsat"] as? Int64 else { throw SdkError.Generic(message: "Missing mandatory field baseFeeMsat for type LspInformation") }
        guard let feeRate = data["feeRate"] as? Double else { throw SdkError.Generic(message: "Missing mandatory field feeRate for type LspInformation") }
        guard let timeLockDelta = data["timeLockDelta"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field timeLockDelta for type LspInformation") }
        guard let minHtlcMsat = data["minHtlcMsat"] as? Int64 else { throw SdkError.Generic(message: "Missing mandatory field minHtlcMsat for type LspInformation") }
        guard let lspPubkey = data["lspPubkey"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field lspPubkey for type LspInformation") }
        guard let openingFeeParamsListTmp = data["openingFeeParamsList"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field openingFeeParamsList for type LspInformation") }
        let openingFeeParamsList = try asOpeningFeeParamsMenu(data: openingFeeParamsListTmp)

        return LspInformation(
            id: id,
            name: name,
            widgetUrl: widgetUrl,
            pubkey: pubkey,
            host: host,
            channelCapacity: channelCapacity,
            targetConf: targetConf,
            baseFeeMsat: baseFeeMsat,
            feeRate: feeRate,
            timeLockDelta: timeLockDelta,
            minHtlcMsat: minHtlcMsat,
            lspPubkey: lspPubkey,
            openingFeeParamsList: openingFeeParamsList
        )
    }

    static func dictionaryOf(lspInformation: LspInformation) -> [String: Any?] {
        return [
            "id": lspInformation.id,
            "name": lspInformation.name,
            "widgetUrl": lspInformation.widgetUrl,
            "pubkey": lspInformation.pubkey,
            "host": lspInformation.host,
            "channelCapacity": lspInformation.channelCapacity,
            "targetConf": lspInformation.targetConf,
            "baseFeeMsat": lspInformation.baseFeeMsat,
            "feeRate": lspInformation.feeRate,
            "timeLockDelta": lspInformation.timeLockDelta,
            "minHtlcMsat": lspInformation.minHtlcMsat,
            "lspPubkey": lspInformation.lspPubkey,
            "openingFeeParamsList": dictionaryOf(openingFeeParamsMenu: lspInformation.openingFeeParamsList),
        ]
    }

    static func asLspInformationList(arr: [Any]) throws -> [LspInformation] {
        var list = [LspInformation]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lspInformation = try asLspInformation(data: val)
                list.append(lspInformation)
            } else {
                throw SdkError.Generic(message: "Invalid element type LspInformation")
            }
        }
        return list
    }

    static func arrayOf(lspInformationList: [LspInformation]) -> [Any] {
        return lspInformationList.map { v -> [String: Any?] in dictionaryOf(lspInformation: v) }
    }

    static func asMessageSuccessActionData(data: [String: Any?]) throws -> MessageSuccessActionData {
        guard let message = data["message"] as? String else { throw SdkError.Generic(message: "Missing mandatory field message for type MessageSuccessActionData") }

        return MessageSuccessActionData(
            message: message)
    }

    static func dictionaryOf(messageSuccessActionData: MessageSuccessActionData) -> [String: Any?] {
        return [
            "message": messageSuccessActionData.message,
        ]
    }

    static func asMessageSuccessActionDataList(arr: [Any]) throws -> [MessageSuccessActionData] {
        var list = [MessageSuccessActionData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var messageSuccessActionData = try asMessageSuccessActionData(data: val)
                list.append(messageSuccessActionData)
            } else {
                throw SdkError.Generic(message: "Invalid element type MessageSuccessActionData")
            }
        }
        return list
    }

    static func arrayOf(messageSuccessActionDataList: [MessageSuccessActionData]) -> [Any] {
        return messageSuccessActionDataList.map { v -> [String: Any?] in dictionaryOf(messageSuccessActionData: v) }
    }

    static func asMetadataItem(data: [String: Any?]) throws -> MetadataItem {
        guard let key = data["key"] as? String else { throw SdkError.Generic(message: "Missing mandatory field key for type MetadataItem") }
        guard let value = data["value"] as? String else { throw SdkError.Generic(message: "Missing mandatory field value for type MetadataItem") }

        return MetadataItem(
            key: key,
            value: value
        )
    }

    static func dictionaryOf(metadataItem: MetadataItem) -> [String: Any?] {
        return [
            "key": metadataItem.key,
            "value": metadataItem.value,
        ]
    }

    static func asMetadataItemList(arr: [Any]) throws -> [MetadataItem] {
        var list = [MetadataItem]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var metadataItem = try asMetadataItem(data: val)
                list.append(metadataItem)
            } else {
                throw SdkError.Generic(message: "Invalid element type MetadataItem")
            }
        }
        return list
    }

    static func arrayOf(metadataItemList: [MetadataItem]) -> [Any] {
        return metadataItemList.map { v -> [String: Any?] in dictionaryOf(metadataItem: v) }
    }

    static func asNodeState(data: [String: Any?]) throws -> NodeState {
        guard let id = data["id"] as? String else { throw SdkError.Generic(message: "Missing mandatory field id for type NodeState") }
        guard let blockHeight = data["blockHeight"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field blockHeight for type NodeState") }
        guard let channelsBalanceMsat = data["channelsBalanceMsat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field channelsBalanceMsat for type NodeState") }
        guard let onchainBalanceMsat = data["onchainBalanceMsat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field onchainBalanceMsat for type NodeState") }
        guard let utxosTmp = data["utxos"] as? [[String: Any?]] else { throw SdkError.Generic(message: "Missing mandatory field utxos for type NodeState") }
        let utxos = try asUnspentTransactionOutputList(arr: utxosTmp)

        guard let maxPayableMsat = data["maxPayableMsat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field maxPayableMsat for type NodeState") }
        guard let maxReceivableMsat = data["maxReceivableMsat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field maxReceivableMsat for type NodeState") }
        guard let maxSinglePaymentAmountMsat = data["maxSinglePaymentAmountMsat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field maxSinglePaymentAmountMsat for type NodeState") }
        guard let maxChanReserveMsats = data["maxChanReserveMsats"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field maxChanReserveMsats for type NodeState") }
        guard let connectedPeers = data["connectedPeers"] as? [String] else { throw SdkError.Generic(message: "Missing mandatory field connectedPeers for type NodeState") }
        guard let inboundLiquidityMsats = data["inboundLiquidityMsats"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field inboundLiquidityMsats for type NodeState") }

        return NodeState(
            id: id,
            blockHeight: blockHeight,
            channelsBalanceMsat: channelsBalanceMsat,
            onchainBalanceMsat: onchainBalanceMsat,
            utxos: utxos,
            maxPayableMsat: maxPayableMsat,
            maxReceivableMsat: maxReceivableMsat,
            maxSinglePaymentAmountMsat: maxSinglePaymentAmountMsat,
            maxChanReserveMsats: maxChanReserveMsats,
            connectedPeers: connectedPeers,
            inboundLiquidityMsats: inboundLiquidityMsats
        )
    }

    static func dictionaryOf(nodeState: NodeState) -> [String: Any?] {
        return [
            "id": nodeState.id,
            "blockHeight": nodeState.blockHeight,
            "channelsBalanceMsat": nodeState.channelsBalanceMsat,
            "onchainBalanceMsat": nodeState.onchainBalanceMsat,
            "utxos": arrayOf(unspentTransactionOutputList: nodeState.utxos),
            "maxPayableMsat": nodeState.maxPayableMsat,
            "maxReceivableMsat": nodeState.maxReceivableMsat,
            "maxSinglePaymentAmountMsat": nodeState.maxSinglePaymentAmountMsat,
            "maxChanReserveMsats": nodeState.maxChanReserveMsats,
            "connectedPeers": nodeState.connectedPeers,
            "inboundLiquidityMsats": nodeState.inboundLiquidityMsats,
        ]
    }

    static func asNodeStateList(arr: [Any]) throws -> [NodeState] {
        var list = [NodeState]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var nodeState = try asNodeState(data: val)
                list.append(nodeState)
            } else {
                throw SdkError.Generic(message: "Invalid element type NodeState")
            }
        }
        return list
    }

    static func arrayOf(nodeStateList: [NodeState]) -> [Any] {
        return nodeStateList.map { v -> [String: Any?] in dictionaryOf(nodeState: v) }
    }

    static func asOpenChannelFeeRequest(data: [String: Any?]) throws -> OpenChannelFeeRequest {
        guard let amountMsat = data["amountMsat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field amountMsat for type OpenChannelFeeRequest") }
        let expiry = data["expiry"] as? UInt32

        return OpenChannelFeeRequest(
            amountMsat: amountMsat,
            expiry: expiry
        )
    }

    static func dictionaryOf(openChannelFeeRequest: OpenChannelFeeRequest) -> [String: Any?] {
        return [
            "amountMsat": openChannelFeeRequest.amountMsat,
            "expiry": openChannelFeeRequest.expiry == nil ? nil : openChannelFeeRequest.expiry,
        ]
    }

    static func asOpenChannelFeeRequestList(arr: [Any]) throws -> [OpenChannelFeeRequest] {
        var list = [OpenChannelFeeRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var openChannelFeeRequest = try asOpenChannelFeeRequest(data: val)
                list.append(openChannelFeeRequest)
            } else {
                throw SdkError.Generic(message: "Invalid element type OpenChannelFeeRequest")
            }
        }
        return list
    }

    static func arrayOf(openChannelFeeRequestList: [OpenChannelFeeRequest]) -> [Any] {
        return openChannelFeeRequestList.map { v -> [String: Any?] in dictionaryOf(openChannelFeeRequest: v) }
    }

    static func asOpenChannelFeeResponse(data: [String: Any?]) throws -> OpenChannelFeeResponse {
        guard let feeMsat = data["feeMsat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field feeMsat for type OpenChannelFeeResponse") }
        var usedFeeParams: OpeningFeeParams?
        if let usedFeeParamsTmp = data["usedFeeParams"] as? [String: Any?] {
            usedFeeParams = try asOpeningFeeParams(data: usedFeeParamsTmp)
        }

        return OpenChannelFeeResponse(
            feeMsat: feeMsat,
            usedFeeParams: usedFeeParams
        )
    }

    static func dictionaryOf(openChannelFeeResponse: OpenChannelFeeResponse) -> [String: Any?] {
        return [
            "feeMsat": openChannelFeeResponse.feeMsat,
            "usedFeeParams": openChannelFeeResponse.usedFeeParams == nil ? nil : dictionaryOf(openingFeeParams: openChannelFeeResponse.usedFeeParams!),
        ]
    }

    static func asOpenChannelFeeResponseList(arr: [Any]) throws -> [OpenChannelFeeResponse] {
        var list = [OpenChannelFeeResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var openChannelFeeResponse = try asOpenChannelFeeResponse(data: val)
                list.append(openChannelFeeResponse)
            } else {
                throw SdkError.Generic(message: "Invalid element type OpenChannelFeeResponse")
            }
        }
        return list
    }

    static func arrayOf(openChannelFeeResponseList: [OpenChannelFeeResponse]) -> [Any] {
        return openChannelFeeResponseList.map { v -> [String: Any?] in dictionaryOf(openChannelFeeResponse: v) }
    }

    static func asOpeningFeeParams(data: [String: Any?]) throws -> OpeningFeeParams {
        guard let minMsat = data["minMsat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field minMsat for type OpeningFeeParams") }
        guard let proportional = data["proportional"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field proportional for type OpeningFeeParams") }
        guard let validUntil = data["validUntil"] as? String else { throw SdkError.Generic(message: "Missing mandatory field validUntil for type OpeningFeeParams") }
        guard let maxIdleTime = data["maxIdleTime"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field maxIdleTime for type OpeningFeeParams") }
        guard let maxClientToSelfDelay = data["maxClientToSelfDelay"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field maxClientToSelfDelay for type OpeningFeeParams") }
        guard let promise = data["promise"] as? String else { throw SdkError.Generic(message: "Missing mandatory field promise for type OpeningFeeParams") }

        return OpeningFeeParams(
            minMsat: minMsat,
            proportional: proportional,
            validUntil: validUntil,
            maxIdleTime: maxIdleTime,
            maxClientToSelfDelay: maxClientToSelfDelay,
            promise: promise
        )
    }

    static func dictionaryOf(openingFeeParams: OpeningFeeParams) -> [String: Any?] {
        return [
            "minMsat": openingFeeParams.minMsat,
            "proportional": openingFeeParams.proportional,
            "validUntil": openingFeeParams.validUntil,
            "maxIdleTime": openingFeeParams.maxIdleTime,
            "maxClientToSelfDelay": openingFeeParams.maxClientToSelfDelay,
            "promise": openingFeeParams.promise,
        ]
    }

    static func asOpeningFeeParamsList(arr: [Any]) throws -> [OpeningFeeParams] {
        var list = [OpeningFeeParams]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var openingFeeParams = try asOpeningFeeParams(data: val)
                list.append(openingFeeParams)
            } else {
                throw SdkError.Generic(message: "Invalid element type OpeningFeeParams")
            }
        }
        return list
    }

    static func arrayOf(openingFeeParamsList: [OpeningFeeParams]) -> [Any] {
        return openingFeeParamsList.map { v -> [String: Any?] in dictionaryOf(openingFeeParams: v) }
    }

    static func asOpeningFeeParamsMenu(data: [String: Any?]) throws -> OpeningFeeParamsMenu {
        guard let valuesTmp = data["values"] as? [[String: Any?]] else { throw SdkError.Generic(message: "Missing mandatory field values for type OpeningFeeParamsMenu") }
        let values = try asOpeningFeeParamsList(arr: valuesTmp)

        return OpeningFeeParamsMenu(
            values: values)
    }

    static func dictionaryOf(openingFeeParamsMenu: OpeningFeeParamsMenu) -> [String: Any?] {
        return [
            "values": arrayOf(openingFeeParamsList: openingFeeParamsMenu.values),
        ]
    }

    static func asOpeningFeeParamsMenuList(arr: [Any]) throws -> [OpeningFeeParamsMenu] {
        var list = [OpeningFeeParamsMenu]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var openingFeeParamsMenu = try asOpeningFeeParamsMenu(data: val)
                list.append(openingFeeParamsMenu)
            } else {
                throw SdkError.Generic(message: "Invalid element type OpeningFeeParamsMenu")
            }
        }
        return list
    }

    static func arrayOf(openingFeeParamsMenuList: [OpeningFeeParamsMenu]) -> [Any] {
        return openingFeeParamsMenuList.map { v -> [String: Any?] in dictionaryOf(openingFeeParamsMenu: v) }
    }

    static func asPayment(data: [String: Any?]) throws -> Payment {
        guard let id = data["id"] as? String else { throw SdkError.Generic(message: "Missing mandatory field id for type Payment") }
        guard let paymentTypeTmp = data["paymentType"] as? String else { throw SdkError.Generic(message: "Missing mandatory field paymentType for type Payment") }
        let paymentType = try asPaymentType(type: paymentTypeTmp)

        guard let paymentTime = data["paymentTime"] as? Int64 else { throw SdkError.Generic(message: "Missing mandatory field paymentTime for type Payment") }
        guard let amountMsat = data["amountMsat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field amountMsat for type Payment") }
        guard let feeMsat = data["feeMsat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field feeMsat for type Payment") }
        guard let statusTmp = data["status"] as? String else { throw SdkError.Generic(message: "Missing mandatory field status for type Payment") }
        let status = try asPaymentStatus(type: statusTmp)

        let description = data["description"] as? String
        guard let detailsTmp = data["details"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field details for type Payment") }
        let details = try asPaymentDetails(data: detailsTmp)

        return Payment(
            id: id,
            paymentType: paymentType,
            paymentTime: paymentTime,
            amountMsat: amountMsat,
            feeMsat: feeMsat,
            status: status,
            description: description,
            details: details
        )
    }

    static func dictionaryOf(payment: Payment) -> [String: Any?] {
        return [
            "id": payment.id,
            "paymentType": valueOf(paymentType: payment.paymentType),
            "paymentTime": payment.paymentTime,
            "amountMsat": payment.amountMsat,
            "feeMsat": payment.feeMsat,
            "status": valueOf(paymentStatus: payment.status),
            "description": payment.description == nil ? nil : payment.description,
            "details": dictionaryOf(paymentDetails: payment.details),
        ]
    }

    static func asPaymentList(arr: [Any]) throws -> [Payment] {
        var list = [Payment]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var payment = try asPayment(data: val)
                list.append(payment)
            } else {
                throw SdkError.Generic(message: "Invalid element type Payment")
            }
        }
        return list
    }

    static func arrayOf(paymentList: [Payment]) -> [Any] {
        return paymentList.map { v -> [String: Any?] in dictionaryOf(payment: v) }
    }

    static func asPaymentFailedData(data: [String: Any?]) throws -> PaymentFailedData {
        guard let error = data["error"] as? String else { throw SdkError.Generic(message: "Missing mandatory field error for type PaymentFailedData") }
        guard let nodeId = data["nodeId"] as? String else { throw SdkError.Generic(message: "Missing mandatory field nodeId for type PaymentFailedData") }
        var invoice: LnInvoice?
        if let invoiceTmp = data["invoice"] as? [String: Any?] {
            invoice = try asLnInvoice(data: invoiceTmp)
        }

        return PaymentFailedData(
            error: error,
            nodeId: nodeId,
            invoice: invoice
        )
    }

    static func dictionaryOf(paymentFailedData: PaymentFailedData) -> [String: Any?] {
        return [
            "error": paymentFailedData.error,
            "nodeId": paymentFailedData.nodeId,
            "invoice": paymentFailedData.invoice == nil ? nil : dictionaryOf(lnInvoice: paymentFailedData.invoice!),
        ]
    }

    static func asPaymentFailedDataList(arr: [Any]) throws -> [PaymentFailedData] {
        var list = [PaymentFailedData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var paymentFailedData = try asPaymentFailedData(data: val)
                list.append(paymentFailedData)
            } else {
                throw SdkError.Generic(message: "Invalid element type PaymentFailedData")
            }
        }
        return list
    }

    static func arrayOf(paymentFailedDataList: [PaymentFailedData]) -> [Any] {
        return paymentFailedDataList.map { v -> [String: Any?] in dictionaryOf(paymentFailedData: v) }
    }

    static func asRate(data: [String: Any?]) throws -> Rate {
        guard let coin = data["coin"] as? String else { throw SdkError.Generic(message: "Missing mandatory field coin for type Rate") }
        guard let value = data["value"] as? Double else { throw SdkError.Generic(message: "Missing mandatory field value for type Rate") }

        return Rate(
            coin: coin,
            value: value
        )
    }

    static func dictionaryOf(rate: Rate) -> [String: Any?] {
        return [
            "coin": rate.coin,
            "value": rate.value,
        ]
    }

    static func asRateList(arr: [Any]) throws -> [Rate] {
        var list = [Rate]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var rate = try asRate(data: val)
                list.append(rate)
            } else {
                throw SdkError.Generic(message: "Invalid element type Rate")
            }
        }
        return list
    }

    static func arrayOf(rateList: [Rate]) -> [Any] {
        return rateList.map { v -> [String: Any?] in dictionaryOf(rate: v) }
    }

    static func asReceiveOnchainRequest(data: [String: Any?]) throws -> ReceiveOnchainRequest {
        var openingFeeParams: OpeningFeeParams?
        if let openingFeeParamsTmp = data["openingFeeParams"] as? [String: Any?] {
            openingFeeParams = try asOpeningFeeParams(data: openingFeeParamsTmp)
        }

        return ReceiveOnchainRequest(
            openingFeeParams: openingFeeParams)
    }

    static func dictionaryOf(receiveOnchainRequest: ReceiveOnchainRequest) -> [String: Any?] {
        return [
            "openingFeeParams": receiveOnchainRequest.openingFeeParams == nil ? nil : dictionaryOf(openingFeeParams: receiveOnchainRequest.openingFeeParams!),
        ]
    }

    static func asReceiveOnchainRequestList(arr: [Any]) throws -> [ReceiveOnchainRequest] {
        var list = [ReceiveOnchainRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var receiveOnchainRequest = try asReceiveOnchainRequest(data: val)
                list.append(receiveOnchainRequest)
            } else {
                throw SdkError.Generic(message: "Invalid element type ReceiveOnchainRequest")
            }
        }
        return list
    }

    static func arrayOf(receiveOnchainRequestList: [ReceiveOnchainRequest]) -> [Any] {
        return receiveOnchainRequestList.map { v -> [String: Any?] in dictionaryOf(receiveOnchainRequest: v) }
    }

    static func asReceivePaymentRequest(data: [String: Any?]) throws -> ReceivePaymentRequest {
        guard let amountSats = data["amountSats"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field amountSats for type ReceivePaymentRequest") }
        guard let description = data["description"] as? String else { throw SdkError.Generic(message: "Missing mandatory field description for type ReceivePaymentRequest") }
        let preimage = data["preimage"] as? [UInt8]
        var openingFeeParams: OpeningFeeParams?
        if let openingFeeParamsTmp = data["openingFeeParams"] as? [String: Any?] {
            openingFeeParams = try asOpeningFeeParams(data: openingFeeParamsTmp)
        }

        let useDescriptionHash = data["useDescriptionHash"] as? Bool
        let expiry = data["expiry"] as? UInt32
        let cltv = data["cltv"] as? UInt32

        return ReceivePaymentRequest(
            amountSats: amountSats,
            description: description,
            preimage: preimage,
            openingFeeParams: openingFeeParams,
            useDescriptionHash: useDescriptionHash,
            expiry: expiry,
            cltv: cltv
        )
    }

    static func dictionaryOf(receivePaymentRequest: ReceivePaymentRequest) -> [String: Any?] {
        return [
            "amountSats": receivePaymentRequest.amountSats,
            "description": receivePaymentRequest.description,
            "preimage": receivePaymentRequest.preimage == nil ? nil : receivePaymentRequest.preimage,
            "openingFeeParams": receivePaymentRequest.openingFeeParams == nil ? nil : dictionaryOf(openingFeeParams: receivePaymentRequest.openingFeeParams!),
            "useDescriptionHash": receivePaymentRequest.useDescriptionHash == nil ? nil : receivePaymentRequest.useDescriptionHash,
            "expiry": receivePaymentRequest.expiry == nil ? nil : receivePaymentRequest.expiry,
            "cltv": receivePaymentRequest.cltv == nil ? nil : receivePaymentRequest.cltv,
        ]
    }

    static func asReceivePaymentRequestList(arr: [Any]) throws -> [ReceivePaymentRequest] {
        var list = [ReceivePaymentRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var receivePaymentRequest = try asReceivePaymentRequest(data: val)
                list.append(receivePaymentRequest)
            } else {
                throw SdkError.Generic(message: "Invalid element type ReceivePaymentRequest")
            }
        }
        return list
    }

    static func arrayOf(receivePaymentRequestList: [ReceivePaymentRequest]) -> [Any] {
        return receivePaymentRequestList.map { v -> [String: Any?] in dictionaryOf(receivePaymentRequest: v) }
    }

    static func asReceivePaymentResponse(data: [String: Any?]) throws -> ReceivePaymentResponse {
        guard let lnInvoiceTmp = data["lnInvoice"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field lnInvoice for type ReceivePaymentResponse") }
        let lnInvoice = try asLnInvoice(data: lnInvoiceTmp)

        var openingFeeParams: OpeningFeeParams?
        if let openingFeeParamsTmp = data["openingFeeParams"] as? [String: Any?] {
            openingFeeParams = try asOpeningFeeParams(data: openingFeeParamsTmp)
        }

        let openingFeeMsat = data["openingFeeMsat"] as? UInt64

        return ReceivePaymentResponse(
            lnInvoice: lnInvoice,
            openingFeeParams: openingFeeParams,
            openingFeeMsat: openingFeeMsat
        )
    }

    static func dictionaryOf(receivePaymentResponse: ReceivePaymentResponse) -> [String: Any?] {
        return [
            "lnInvoice": dictionaryOf(lnInvoice: receivePaymentResponse.lnInvoice),
            "openingFeeParams": receivePaymentResponse.openingFeeParams == nil ? nil : dictionaryOf(openingFeeParams: receivePaymentResponse.openingFeeParams!),
            "openingFeeMsat": receivePaymentResponse.openingFeeMsat == nil ? nil : receivePaymentResponse.openingFeeMsat,
        ]
    }

    static func asReceivePaymentResponseList(arr: [Any]) throws -> [ReceivePaymentResponse] {
        var list = [ReceivePaymentResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var receivePaymentResponse = try asReceivePaymentResponse(data: val)
                list.append(receivePaymentResponse)
            } else {
                throw SdkError.Generic(message: "Invalid element type ReceivePaymentResponse")
            }
        }
        return list
    }

    static func arrayOf(receivePaymentResponseList: [ReceivePaymentResponse]) -> [Any] {
        return receivePaymentResponseList.map { v -> [String: Any?] in dictionaryOf(receivePaymentResponse: v) }
    }

    static func asRecommendedFees(data: [String: Any?]) throws -> RecommendedFees {
        guard let fastestFee = data["fastestFee"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field fastestFee for type RecommendedFees") }
        guard let halfHourFee = data["halfHourFee"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field halfHourFee for type RecommendedFees") }
        guard let hourFee = data["hourFee"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field hourFee for type RecommendedFees") }
        guard let economyFee = data["economyFee"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field economyFee for type RecommendedFees") }
        guard let minimumFee = data["minimumFee"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field minimumFee for type RecommendedFees") }

        return RecommendedFees(
            fastestFee: fastestFee,
            halfHourFee: halfHourFee,
            hourFee: hourFee,
            economyFee: economyFee,
            minimumFee: minimumFee
        )
    }

    static func dictionaryOf(recommendedFees: RecommendedFees) -> [String: Any?] {
        return [
            "fastestFee": recommendedFees.fastestFee,
            "halfHourFee": recommendedFees.halfHourFee,
            "hourFee": recommendedFees.hourFee,
            "economyFee": recommendedFees.economyFee,
            "minimumFee": recommendedFees.minimumFee,
        ]
    }

    static func asRecommendedFeesList(arr: [Any]) throws -> [RecommendedFees] {
        var list = [RecommendedFees]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var recommendedFees = try asRecommendedFees(data: val)
                list.append(recommendedFees)
            } else {
                throw SdkError.Generic(message: "Invalid element type RecommendedFees")
            }
        }
        return list
    }

    static func arrayOf(recommendedFeesList: [RecommendedFees]) -> [Any] {
        return recommendedFeesList.map { v -> [String: Any?] in dictionaryOf(recommendedFees: v) }
    }

    static func asReverseSwapFeesRequest(data: [String: Any?]) throws -> ReverseSwapFeesRequest {
        let sendAmountSat = data["sendAmountSat"] as? UInt64

        return ReverseSwapFeesRequest(
            sendAmountSat: sendAmountSat)
    }

    static func dictionaryOf(reverseSwapFeesRequest: ReverseSwapFeesRequest) -> [String: Any?] {
        return [
            "sendAmountSat": reverseSwapFeesRequest.sendAmountSat == nil ? nil : reverseSwapFeesRequest.sendAmountSat,
        ]
    }

    static func asReverseSwapFeesRequestList(arr: [Any]) throws -> [ReverseSwapFeesRequest] {
        var list = [ReverseSwapFeesRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var reverseSwapFeesRequest = try asReverseSwapFeesRequest(data: val)
                list.append(reverseSwapFeesRequest)
            } else {
                throw SdkError.Generic(message: "Invalid element type ReverseSwapFeesRequest")
            }
        }
        return list
    }

    static func arrayOf(reverseSwapFeesRequestList: [ReverseSwapFeesRequest]) -> [Any] {
        return reverseSwapFeesRequestList.map { v -> [String: Any?] in dictionaryOf(reverseSwapFeesRequest: v) }
    }

    static func asReverseSwapInfo(data: [String: Any?]) throws -> ReverseSwapInfo {
        guard let id = data["id"] as? String else { throw SdkError.Generic(message: "Missing mandatory field id for type ReverseSwapInfo") }
        guard let claimPubkey = data["claimPubkey"] as? String else { throw SdkError.Generic(message: "Missing mandatory field claimPubkey for type ReverseSwapInfo") }
        let lockupTxid = data["lockupTxid"] as? String
        let claimTxid = data["claimTxid"] as? String
        guard let onchainAmountSat = data["onchainAmountSat"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field onchainAmountSat for type ReverseSwapInfo") }
        guard let statusTmp = data["status"] as? String else { throw SdkError.Generic(message: "Missing mandatory field status for type ReverseSwapInfo") }
        let status = try asReverseSwapStatus(type: statusTmp)

        return ReverseSwapInfo(
            id: id,
            claimPubkey: claimPubkey,
            lockupTxid: lockupTxid,
            claimTxid: claimTxid,
            onchainAmountSat: onchainAmountSat,
            status: status
        )
    }

    static func dictionaryOf(reverseSwapInfo: ReverseSwapInfo) -> [String: Any?] {
        return [
            "id": reverseSwapInfo.id,
            "claimPubkey": reverseSwapInfo.claimPubkey,
            "lockupTxid": reverseSwapInfo.lockupTxid == nil ? nil : reverseSwapInfo.lockupTxid,
            "claimTxid": reverseSwapInfo.claimTxid == nil ? nil : reverseSwapInfo.claimTxid,
            "onchainAmountSat": reverseSwapInfo.onchainAmountSat,
            "status": valueOf(reverseSwapStatus: reverseSwapInfo.status),
        ]
    }

    static func asReverseSwapInfoList(arr: [Any]) throws -> [ReverseSwapInfo] {
        var list = [ReverseSwapInfo]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var reverseSwapInfo = try asReverseSwapInfo(data: val)
                list.append(reverseSwapInfo)
            } else {
                throw SdkError.Generic(message: "Invalid element type ReverseSwapInfo")
            }
        }
        return list
    }

    static func arrayOf(reverseSwapInfoList: [ReverseSwapInfo]) -> [Any] {
        return reverseSwapInfoList.map { v -> [String: Any?] in dictionaryOf(reverseSwapInfo: v) }
    }

    static func asReverseSwapPairInfo(data: [String: Any?]) throws -> ReverseSwapPairInfo {
        guard let min = data["min"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field min for type ReverseSwapPairInfo") }
        guard let max = data["max"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field max for type ReverseSwapPairInfo") }
        guard let feesHash = data["feesHash"] as? String else { throw SdkError.Generic(message: "Missing mandatory field feesHash for type ReverseSwapPairInfo") }
        guard let feesPercentage = data["feesPercentage"] as? Double else { throw SdkError.Generic(message: "Missing mandatory field feesPercentage for type ReverseSwapPairInfo") }
        guard let feesLockup = data["feesLockup"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field feesLockup for type ReverseSwapPairInfo") }
        guard let feesClaim = data["feesClaim"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field feesClaim for type ReverseSwapPairInfo") }
        let totalEstimatedFees = data["totalEstimatedFees"] as? UInt64

        return ReverseSwapPairInfo(
            min: min,
            max: max,
            feesHash: feesHash,
            feesPercentage: feesPercentage,
            feesLockup: feesLockup,
            feesClaim: feesClaim,
            totalEstimatedFees: totalEstimatedFees
        )
    }

    static func dictionaryOf(reverseSwapPairInfo: ReverseSwapPairInfo) -> [String: Any?] {
        return [
            "min": reverseSwapPairInfo.min,
            "max": reverseSwapPairInfo.max,
            "feesHash": reverseSwapPairInfo.feesHash,
            "feesPercentage": reverseSwapPairInfo.feesPercentage,
            "feesLockup": reverseSwapPairInfo.feesLockup,
            "feesClaim": reverseSwapPairInfo.feesClaim,
            "totalEstimatedFees": reverseSwapPairInfo.totalEstimatedFees == nil ? nil : reverseSwapPairInfo.totalEstimatedFees,
        ]
    }

    static func asReverseSwapPairInfoList(arr: [Any]) throws -> [ReverseSwapPairInfo] {
        var list = [ReverseSwapPairInfo]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var reverseSwapPairInfo = try asReverseSwapPairInfo(data: val)
                list.append(reverseSwapPairInfo)
            } else {
                throw SdkError.Generic(message: "Invalid element type ReverseSwapPairInfo")
            }
        }
        return list
    }

    static func arrayOf(reverseSwapPairInfoList: [ReverseSwapPairInfo]) -> [Any] {
        return reverseSwapPairInfoList.map { v -> [String: Any?] in dictionaryOf(reverseSwapPairInfo: v) }
    }

    static func asRouteHint(data: [String: Any?]) throws -> RouteHint {
        guard let hopsTmp = data["hops"] as? [[String: Any?]] else { throw SdkError.Generic(message: "Missing mandatory field hops for type RouteHint") }
        let hops = try asRouteHintHopList(arr: hopsTmp)

        return RouteHint(
            hops: hops)
    }

    static func dictionaryOf(routeHint: RouteHint) -> [String: Any?] {
        return [
            "hops": arrayOf(routeHintHopList: routeHint.hops),
        ]
    }

    static func asRouteHintList(arr: [Any]) throws -> [RouteHint] {
        var list = [RouteHint]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var routeHint = try asRouteHint(data: val)
                list.append(routeHint)
            } else {
                throw SdkError.Generic(message: "Invalid element type RouteHint")
            }
        }
        return list
    }

    static func arrayOf(routeHintList: [RouteHint]) -> [Any] {
        return routeHintList.map { v -> [String: Any?] in dictionaryOf(routeHint: v) }
    }

    static func asRouteHintHop(data: [String: Any?]) throws -> RouteHintHop {
        guard let srcNodeId = data["srcNodeId"] as? String else { throw SdkError.Generic(message: "Missing mandatory field srcNodeId for type RouteHintHop") }
        guard let shortChannelId = data["shortChannelId"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field shortChannelId for type RouteHintHop") }
        guard let feesBaseMsat = data["feesBaseMsat"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field feesBaseMsat for type RouteHintHop") }
        guard let feesProportionalMillionths = data["feesProportionalMillionths"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field feesProportionalMillionths for type RouteHintHop") }
        guard let cltvExpiryDelta = data["cltvExpiryDelta"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field cltvExpiryDelta for type RouteHintHop") }
        let htlcMinimumMsat = data["htlcMinimumMsat"] as? UInt64
        let htlcMaximumMsat = data["htlcMaximumMsat"] as? UInt64

        return RouteHintHop(
            srcNodeId: srcNodeId,
            shortChannelId: shortChannelId,
            feesBaseMsat: feesBaseMsat,
            feesProportionalMillionths: feesProportionalMillionths,
            cltvExpiryDelta: cltvExpiryDelta,
            htlcMinimumMsat: htlcMinimumMsat,
            htlcMaximumMsat: htlcMaximumMsat
        )
    }

    static func dictionaryOf(routeHintHop: RouteHintHop) -> [String: Any?] {
        return [
            "srcNodeId": routeHintHop.srcNodeId,
            "shortChannelId": routeHintHop.shortChannelId,
            "feesBaseMsat": routeHintHop.feesBaseMsat,
            "feesProportionalMillionths": routeHintHop.feesProportionalMillionths,
            "cltvExpiryDelta": routeHintHop.cltvExpiryDelta,
            "htlcMinimumMsat": routeHintHop.htlcMinimumMsat == nil ? nil : routeHintHop.htlcMinimumMsat,
            "htlcMaximumMsat": routeHintHop.htlcMaximumMsat == nil ? nil : routeHintHop.htlcMaximumMsat,
        ]
    }

    static func asRouteHintHopList(arr: [Any]) throws -> [RouteHintHop] {
        var list = [RouteHintHop]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var routeHintHop = try asRouteHintHop(data: val)
                list.append(routeHintHop)
            } else {
                throw SdkError.Generic(message: "Invalid element type RouteHintHop")
            }
        }
        return list
    }

    static func arrayOf(routeHintHopList: [RouteHintHop]) -> [Any] {
        return routeHintHopList.map { v -> [String: Any?] in dictionaryOf(routeHintHop: v) }
    }

    static func asSignMessageRequest(data: [String: Any?]) throws -> SignMessageRequest {
        guard let message = data["message"] as? String else { throw SdkError.Generic(message: "Missing mandatory field message for type SignMessageRequest") }

        return SignMessageRequest(
            message: message)
    }

    static func dictionaryOf(signMessageRequest: SignMessageRequest) -> [String: Any?] {
        return [
            "message": signMessageRequest.message,
        ]
    }

    static func asSignMessageRequestList(arr: [Any]) throws -> [SignMessageRequest] {
        var list = [SignMessageRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var signMessageRequest = try asSignMessageRequest(data: val)
                list.append(signMessageRequest)
            } else {
                throw SdkError.Generic(message: "Invalid element type SignMessageRequest")
            }
        }
        return list
    }

    static func arrayOf(signMessageRequestList: [SignMessageRequest]) -> [Any] {
        return signMessageRequestList.map { v -> [String: Any?] in dictionaryOf(signMessageRequest: v) }
    }

    static func asSignMessageResponse(data: [String: Any?]) throws -> SignMessageResponse {
        guard let signature = data["signature"] as? String else { throw SdkError.Generic(message: "Missing mandatory field signature for type SignMessageResponse") }

        return SignMessageResponse(
            signature: signature)
    }

    static func dictionaryOf(signMessageResponse: SignMessageResponse) -> [String: Any?] {
        return [
            "signature": signMessageResponse.signature,
        ]
    }

    static func asSignMessageResponseList(arr: [Any]) throws -> [SignMessageResponse] {
        var list = [SignMessageResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var signMessageResponse = try asSignMessageResponse(data: val)
                list.append(signMessageResponse)
            } else {
                throw SdkError.Generic(message: "Invalid element type SignMessageResponse")
            }
        }
        return list
    }

    static func arrayOf(signMessageResponseList: [SignMessageResponse]) -> [Any] {
        return signMessageResponseList.map { v -> [String: Any?] in dictionaryOf(signMessageResponse: v) }
    }

    static func asStaticBackupRequest(data: [String: Any?]) throws -> StaticBackupRequest {
        guard let workingDir = data["workingDir"] as? String else { throw SdkError.Generic(message: "Missing mandatory field workingDir for type StaticBackupRequest") }

        return StaticBackupRequest(
            workingDir: workingDir)
    }

    static func dictionaryOf(staticBackupRequest: StaticBackupRequest) -> [String: Any?] {
        return [
            "workingDir": staticBackupRequest.workingDir,
        ]
    }

    static func asStaticBackupRequestList(arr: [Any]) throws -> [StaticBackupRequest] {
        var list = [StaticBackupRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var staticBackupRequest = try asStaticBackupRequest(data: val)
                list.append(staticBackupRequest)
            } else {
                throw SdkError.Generic(message: "Invalid element type StaticBackupRequest")
            }
        }
        return list
    }

    static func arrayOf(staticBackupRequestList: [StaticBackupRequest]) -> [Any] {
        return staticBackupRequestList.map { v -> [String: Any?] in dictionaryOf(staticBackupRequest: v) }
    }

    static func asStaticBackupResponse(data: [String: Any?]) throws -> StaticBackupResponse {
        let backup = data["backup"] as? [String]

        return StaticBackupResponse(
            backup: backup)
    }

    static func dictionaryOf(staticBackupResponse: StaticBackupResponse) -> [String: Any?] {
        return [
            "backup": staticBackupResponse.backup == nil ? nil : staticBackupResponse.backup,
        ]
    }

    static func asStaticBackupResponseList(arr: [Any]) throws -> [StaticBackupResponse] {
        var list = [StaticBackupResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var staticBackupResponse = try asStaticBackupResponse(data: val)
                list.append(staticBackupResponse)
            } else {
                throw SdkError.Generic(message: "Invalid element type StaticBackupResponse")
            }
        }
        return list
    }

    static func arrayOf(staticBackupResponseList: [StaticBackupResponse]) -> [Any] {
        return staticBackupResponseList.map { v -> [String: Any?] in dictionaryOf(staticBackupResponse: v) }
    }

    static func asSwapInfo(data: [String: Any?]) throws -> SwapInfo {
        guard let bitcoinAddress = data["bitcoinAddress"] as? String else { throw SdkError.Generic(message: "Missing mandatory field bitcoinAddress for type SwapInfo") }
        guard let createdAt = data["createdAt"] as? Int64 else { throw SdkError.Generic(message: "Missing mandatory field createdAt for type SwapInfo") }
        guard let lockHeight = data["lockHeight"] as? Int64 else { throw SdkError.Generic(message: "Missing mandatory field lockHeight for type SwapInfo") }
        guard let paymentHash = data["paymentHash"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field paymentHash for type SwapInfo") }
        guard let preimage = data["preimage"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field preimage for type SwapInfo") }
        guard let privateKey = data["privateKey"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field privateKey for type SwapInfo") }
        guard let publicKey = data["publicKey"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field publicKey for type SwapInfo") }
        guard let swapperPublicKey = data["swapperPublicKey"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field swapperPublicKey for type SwapInfo") }
        guard let script = data["script"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field script for type SwapInfo") }
        let bolt11 = data["bolt11"] as? String
        guard let paidSats = data["paidSats"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field paidSats for type SwapInfo") }
        guard let unconfirmedSats = data["unconfirmedSats"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field unconfirmedSats for type SwapInfo") }
        guard let confirmedSats = data["confirmedSats"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field confirmedSats for type SwapInfo") }
        guard let statusTmp = data["status"] as? String else { throw SdkError.Generic(message: "Missing mandatory field status for type SwapInfo") }
        let status = try asSwapStatus(type: statusTmp)

        guard let refundTxIds = data["refundTxIds"] as? [String] else { throw SdkError.Generic(message: "Missing mandatory field refundTxIds for type SwapInfo") }
        guard let unconfirmedTxIds = data["unconfirmedTxIds"] as? [String] else { throw SdkError.Generic(message: "Missing mandatory field unconfirmedTxIds for type SwapInfo") }
        guard let confirmedTxIds = data["confirmedTxIds"] as? [String] else { throw SdkError.Generic(message: "Missing mandatory field confirmedTxIds for type SwapInfo") }
        guard let minAllowedDeposit = data["minAllowedDeposit"] as? Int64 else { throw SdkError.Generic(message: "Missing mandatory field minAllowedDeposit for type SwapInfo") }
        guard let maxAllowedDeposit = data["maxAllowedDeposit"] as? Int64 else { throw SdkError.Generic(message: "Missing mandatory field maxAllowedDeposit for type SwapInfo") }
        let lastRedeemError = data["lastRedeemError"] as? String
        var channelOpeningFees: OpeningFeeParams?
        if let channelOpeningFeesTmp = data["channelOpeningFees"] as? [String: Any?] {
            channelOpeningFees = try asOpeningFeeParams(data: channelOpeningFeesTmp)
        }

        return SwapInfo(
            bitcoinAddress: bitcoinAddress,
            createdAt: createdAt,
            lockHeight: lockHeight,
            paymentHash: paymentHash,
            preimage: preimage,
            privateKey: privateKey,
            publicKey: publicKey,
            swapperPublicKey: swapperPublicKey,
            script: script,
            bolt11: bolt11,
            paidSats: paidSats,
            unconfirmedSats: unconfirmedSats,
            confirmedSats: confirmedSats,
            status: status,
            refundTxIds: refundTxIds,
            unconfirmedTxIds: unconfirmedTxIds,
            confirmedTxIds: confirmedTxIds,
            minAllowedDeposit: minAllowedDeposit,
            maxAllowedDeposit: maxAllowedDeposit,
            lastRedeemError: lastRedeemError,
            channelOpeningFees: channelOpeningFees
        )
    }

    static func dictionaryOf(swapInfo: SwapInfo) -> [String: Any?] {
        return [
            "bitcoinAddress": swapInfo.bitcoinAddress,
            "createdAt": swapInfo.createdAt,
            "lockHeight": swapInfo.lockHeight,
            "paymentHash": swapInfo.paymentHash,
            "preimage": swapInfo.preimage,
            "privateKey": swapInfo.privateKey,
            "publicKey": swapInfo.publicKey,
            "swapperPublicKey": swapInfo.swapperPublicKey,
            "script": swapInfo.script,
            "bolt11": swapInfo.bolt11 == nil ? nil : swapInfo.bolt11,
            "paidSats": swapInfo.paidSats,
            "unconfirmedSats": swapInfo.unconfirmedSats,
            "confirmedSats": swapInfo.confirmedSats,
            "status": valueOf(swapStatus: swapInfo.status),
            "refundTxIds": swapInfo.refundTxIds,
            "unconfirmedTxIds": swapInfo.unconfirmedTxIds,
            "confirmedTxIds": swapInfo.confirmedTxIds,
            "minAllowedDeposit": swapInfo.minAllowedDeposit,
            "maxAllowedDeposit": swapInfo.maxAllowedDeposit,
            "lastRedeemError": swapInfo.lastRedeemError == nil ? nil : swapInfo.lastRedeemError,
            "channelOpeningFees": swapInfo.channelOpeningFees == nil ? nil : dictionaryOf(openingFeeParams: swapInfo.channelOpeningFees!),
        ]
    }

    static func asSwapInfoList(arr: [Any]) throws -> [SwapInfo] {
        var list = [SwapInfo]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var swapInfo = try asSwapInfo(data: val)
                list.append(swapInfo)
            } else {
                throw SdkError.Generic(message: "Invalid element type SwapInfo")
            }
        }
        return list
    }

    static func arrayOf(swapInfoList: [SwapInfo]) -> [Any] {
        return swapInfoList.map { v -> [String: Any?] in dictionaryOf(swapInfo: v) }
    }

    static func asSweepRequest(data: [String: Any?]) throws -> SweepRequest {
        guard let toAddress = data["toAddress"] as? String else { throw SdkError.Generic(message: "Missing mandatory field toAddress for type SweepRequest") }
        guard let feeRateSatsPerVbyte = data["feeRateSatsPerVbyte"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field feeRateSatsPerVbyte for type SweepRequest") }

        return SweepRequest(
            toAddress: toAddress,
            feeRateSatsPerVbyte: feeRateSatsPerVbyte
        )
    }

    static func dictionaryOf(sweepRequest: SweepRequest) -> [String: Any?] {
        return [
            "toAddress": sweepRequest.toAddress,
            "feeRateSatsPerVbyte": sweepRequest.feeRateSatsPerVbyte,
        ]
    }

    static func asSweepRequestList(arr: [Any]) throws -> [SweepRequest] {
        var list = [SweepRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var sweepRequest = try asSweepRequest(data: val)
                list.append(sweepRequest)
            } else {
                throw SdkError.Generic(message: "Invalid element type SweepRequest")
            }
        }
        return list
    }

    static func arrayOf(sweepRequestList: [SweepRequest]) -> [Any] {
        return sweepRequestList.map { v -> [String: Any?] in dictionaryOf(sweepRequest: v) }
    }

    static func asSweepResponse(data: [String: Any?]) throws -> SweepResponse {
        guard let txid = data["txid"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field txid for type SweepResponse") }

        return SweepResponse(
            txid: txid)
    }

    static func dictionaryOf(sweepResponse: SweepResponse) -> [String: Any?] {
        return [
            "txid": sweepResponse.txid,
        ]
    }

    static func asSweepResponseList(arr: [Any]) throws -> [SweepResponse] {
        var list = [SweepResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var sweepResponse = try asSweepResponse(data: val)
                list.append(sweepResponse)
            } else {
                throw SdkError.Generic(message: "Invalid element type SweepResponse")
            }
        }
        return list
    }

    static func arrayOf(sweepResponseList: [SweepResponse]) -> [Any] {
        return sweepResponseList.map { v -> [String: Any?] in dictionaryOf(sweepResponse: v) }
    }

    static func asSymbol(data: [String: Any?]) throws -> Symbol {
        let grapheme = data["grapheme"] as? String
        let template = data["template"] as? String
        let rtl = data["rtl"] as? Bool
        let position = data["position"] as? UInt32

        return Symbol(
            grapheme: grapheme,
            template: template,
            rtl: rtl,
            position: position
        )
    }

    static func dictionaryOf(symbol: Symbol) -> [String: Any?] {
        return [
            "grapheme": symbol.grapheme == nil ? nil : symbol.grapheme,
            "template": symbol.template == nil ? nil : symbol.template,
            "rtl": symbol.rtl == nil ? nil : symbol.rtl,
            "position": symbol.position == nil ? nil : symbol.position,
        ]
    }

    static func asSymbolList(arr: [Any]) throws -> [Symbol] {
        var list = [Symbol]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var symbol = try asSymbol(data: val)
                list.append(symbol)
            } else {
                throw SdkError.Generic(message: "Invalid element type Symbol")
            }
        }
        return list
    }

    static func arrayOf(symbolList: [Symbol]) -> [Any] {
        return symbolList.map { v -> [String: Any?] in dictionaryOf(symbol: v) }
    }

    static func asUnspentTransactionOutput(data: [String: Any?]) throws -> UnspentTransactionOutput {
        guard let txid = data["txid"] as? [UInt8] else { throw SdkError.Generic(message: "Missing mandatory field txid for type UnspentTransactionOutput") }
        guard let outnum = data["outnum"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field outnum for type UnspentTransactionOutput") }
        guard let amountMillisatoshi = data["amountMillisatoshi"] as? UInt64 else { throw SdkError.Generic(message: "Missing mandatory field amountMillisatoshi for type UnspentTransactionOutput") }
        guard let address = data["address"] as? String else { throw SdkError.Generic(message: "Missing mandatory field address for type UnspentTransactionOutput") }
        guard let reserved = data["reserved"] as? Bool else { throw SdkError.Generic(message: "Missing mandatory field reserved for type UnspentTransactionOutput") }

        return UnspentTransactionOutput(
            txid: txid,
            outnum: outnum,
            amountMillisatoshi: amountMillisatoshi,
            address: address,
            reserved: reserved
        )
    }

    static func dictionaryOf(unspentTransactionOutput: UnspentTransactionOutput) -> [String: Any?] {
        return [
            "txid": unspentTransactionOutput.txid,
            "outnum": unspentTransactionOutput.outnum,
            "amountMillisatoshi": unspentTransactionOutput.amountMillisatoshi,
            "address": unspentTransactionOutput.address,
            "reserved": unspentTransactionOutput.reserved,
        ]
    }

    static func asUnspentTransactionOutputList(arr: [Any]) throws -> [UnspentTransactionOutput] {
        var list = [UnspentTransactionOutput]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var unspentTransactionOutput = try asUnspentTransactionOutput(data: val)
                list.append(unspentTransactionOutput)
            } else {
                throw SdkError.Generic(message: "Invalid element type UnspentTransactionOutput")
            }
        }
        return list
    }

    static func arrayOf(unspentTransactionOutputList: [UnspentTransactionOutput]) -> [Any] {
        return unspentTransactionOutputList.map { v -> [String: Any?] in dictionaryOf(unspentTransactionOutput: v) }
    }

    static func asUrlSuccessActionData(data: [String: Any?]) throws -> UrlSuccessActionData {
        guard let description = data["description"] as? String else { throw SdkError.Generic(message: "Missing mandatory field description for type UrlSuccessActionData") }
        guard let url = data["url"] as? String else { throw SdkError.Generic(message: "Missing mandatory field url for type UrlSuccessActionData") }

        return UrlSuccessActionData(
            description: description,
            url: url
        )
    }

    static func dictionaryOf(urlSuccessActionData: UrlSuccessActionData) -> [String: Any?] {
        return [
            "description": urlSuccessActionData.description,
            "url": urlSuccessActionData.url,
        ]
    }

    static func asUrlSuccessActionDataList(arr: [Any]) throws -> [UrlSuccessActionData] {
        var list = [UrlSuccessActionData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var urlSuccessActionData = try asUrlSuccessActionData(data: val)
                list.append(urlSuccessActionData)
            } else {
                throw SdkError.Generic(message: "Invalid element type UrlSuccessActionData")
            }
        }
        return list
    }

    static func arrayOf(urlSuccessActionDataList: [UrlSuccessActionData]) -> [Any] {
        return urlSuccessActionDataList.map { v -> [String: Any?] in dictionaryOf(urlSuccessActionData: v) }
    }

    static func asBreezEvent(data: [String: Any?]) throws -> BreezEvent {
        let type = data["type"] as! String
        if type == "newBlock" {
            guard let _block = data["block"] as? UInt32 else { throw SdkError.Generic(message: "Missing mandatory field block for type BreezEvent") }
            return BreezEvent.newBlock(block: _block)
        }
        if type == "invoicePaid" {
            guard let detailsTmp = data["details"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field details for type BreezEvent") }
            let _details = try asInvoicePaidDetails(data: detailsTmp)

            return BreezEvent.invoicePaid(details: _details)
        }
        if type == "synced" {
            return BreezEvent.synced
        }
        if type == "paymentSucceed" {
            guard let detailsTmp = data["details"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field details for type BreezEvent") }
            let _details = try asPayment(data: detailsTmp)

            return BreezEvent.paymentSucceed(details: _details)
        }
        if type == "paymentFailed" {
            guard let detailsTmp = data["details"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field details for type BreezEvent") }
            let _details = try asPaymentFailedData(data: detailsTmp)

            return BreezEvent.paymentFailed(details: _details)
        }
        if type == "backupStarted" {
            return BreezEvent.backupStarted
        }
        if type == "backupSucceeded" {
            return BreezEvent.backupSucceeded
        }
        if type == "backupFailed" {
            guard let detailsTmp = data["details"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field details for type BreezEvent") }
            let _details = try asBackupFailedData(data: detailsTmp)

            return BreezEvent.backupFailed(details: _details)
        }

        throw SdkError.Generic(message: "Invalid enum variant \(type) for enum BreezEvent")
    }

    static func dictionaryOf(breezEvent: BreezEvent) -> [String: Any?] {
        switch breezEvent {
        case let .newBlock(
            block
        ):
            return [
                "type": "newBlock",
                "block": block,
            ]

        case let .invoicePaid(
            details
        ):
            return [
                "type": "invoicePaid",
                "details": dictionaryOf(invoicePaidDetails: details),
            ]

        case .synced:
            return [
                "type": "synced",
            ]

        case let .paymentSucceed(
            details
        ):
            return [
                "type": "paymentSucceed",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentFailed(
            details
        ):
            return [
                "type": "paymentFailed",
                "details": dictionaryOf(paymentFailedData: details),
            ]

        case .backupStarted:
            return [
                "type": "backupStarted",
            ]

        case .backupSucceeded:
            return [
                "type": "backupSucceeded",
            ]

        case let .backupFailed(
            details
        ):
            return [
                "type": "backupFailed",
                "details": dictionaryOf(backupFailedData: details),
            ]
        }
    }

    static func asBuyBitcoinProvider(type: String) throws -> BuyBitcoinProvider {
        switch type {
        case "moonpay":
            return BuyBitcoinProvider.moonpay

        default: throw SdkError.Generic(message: "Invalid variant \(type) for enum BuyBitcoinProvider")
        }
    }

    static func valueOf(buyBitcoinProvider: BuyBitcoinProvider) -> String {
        switch buyBitcoinProvider {
        case .moonpay:
            return "moonpay"
        }
    }

    static func asChannelState(type: String) throws -> ChannelState {
        switch type {
        case "pendingOpen":
            return ChannelState.pendingOpen

        case "opened":
            return ChannelState.opened

        case "pendingClose":
            return ChannelState.pendingClose

        case "closed":
            return ChannelState.closed

        default: throw SdkError.Generic(message: "Invalid variant \(type) for enum ChannelState")
        }
    }

    static func valueOf(channelState: ChannelState) -> String {
        switch channelState {
        case .pendingOpen:
            return "pendingOpen"

        case .opened:
            return "opened"

        case .pendingClose:
            return "pendingClose"

        case .closed:
            return "closed"
        }
    }

    static func asEnvironmentType(type: String) throws -> EnvironmentType {
        switch type {
        case "production":
            return EnvironmentType.production

        case "staging":
            return EnvironmentType.staging

        default: throw SdkError.Generic(message: "Invalid variant \(type) for enum EnvironmentType")
        }
    }

    static func valueOf(environmentType: EnvironmentType) -> String {
        switch environmentType {
        case .production:
            return "production"

        case .staging:
            return "staging"
        }
    }

    static func asFeeratePreset(type: String) throws -> FeeratePreset {
        switch type {
        case "regular":
            return FeeratePreset.regular

        case "economy":
            return FeeratePreset.economy

        case "priority":
            return FeeratePreset.priority

        default: throw SdkError.Generic(message: "Invalid variant \(type) for enum FeeratePreset")
        }
    }

    static func valueOf(feeratePreset: FeeratePreset) -> String {
        switch feeratePreset {
        case .regular:
            return "regular"

        case .economy:
            return "economy"

        case .priority:
            return "priority"
        }
    }

    static func asInputType(data: [String: Any?]) throws -> InputType {
        let type = data["type"] as! String
        if type == "bitcoinAddress" {
            guard let addressTmp = data["address"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field address for type InputType") }
            let _address = try asBitcoinAddressData(data: addressTmp)

            return InputType.bitcoinAddress(address: _address)
        }
        if type == "bolt11" {
            guard let invoiceTmp = data["invoice"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field invoice for type InputType") }
            let _invoice = try asLnInvoice(data: invoiceTmp)

            return InputType.bolt11(invoice: _invoice)
        }
        if type == "nodeId" {
            guard let _nodeId = data["nodeId"] as? String else { throw SdkError.Generic(message: "Missing mandatory field nodeId for type InputType") }
            return InputType.nodeId(nodeId: _nodeId)
        }
        if type == "url" {
            guard let _url = data["url"] as? String else { throw SdkError.Generic(message: "Missing mandatory field url for type InputType") }
            return InputType.url(url: _url)
        }
        if type == "lnUrlPay" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type InputType") }
            let _data = try asLnUrlPayRequestData(data: dataTmp)

            return InputType.lnUrlPay(data: _data)
        }
        if type == "lnUrlWithdraw" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type InputType") }
            let _data = try asLnUrlWithdrawRequestData(data: dataTmp)

            return InputType.lnUrlWithdraw(data: _data)
        }
        if type == "lnUrlAuth" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type InputType") }
            let _data = try asLnUrlAuthRequestData(data: dataTmp)

            return InputType.lnUrlAuth(data: _data)
        }
        if type == "lnUrlError" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type InputType") }
            let _data = try asLnUrlErrorData(data: dataTmp)

            return InputType.lnUrlError(data: _data)
        }

        throw SdkError.Generic(message: "Invalid enum variant \(type) for enum InputType")
    }

    static func dictionaryOf(inputType: InputType) -> [String: Any?] {
        switch inputType {
        case let .bitcoinAddress(
            address
        ):
            return [
                "type": "bitcoinAddress",
                "address": dictionaryOf(bitcoinAddressData: address),
            ]

        case let .bolt11(
            invoice
        ):
            return [
                "type": "bolt11",
                "invoice": dictionaryOf(lnInvoice: invoice),
            ]

        case let .nodeId(
            nodeId
        ):
            return [
                "type": "nodeId",
                "nodeId": nodeId,
            ]

        case let .url(
            url
        ):
            return [
                "type": "url",
                "url": url,
            ]

        case let .lnUrlPay(
            data
        ):
            return [
                "type": "lnUrlPay",
                "data": dictionaryOf(lnUrlPayRequestData: data),
            ]

        case let .lnUrlWithdraw(
            data
        ):
            return [
                "type": "lnUrlWithdraw",
                "data": dictionaryOf(lnUrlWithdrawRequestData: data),
            ]

        case let .lnUrlAuth(
            data
        ):
            return [
                "type": "lnUrlAuth",
                "data": dictionaryOf(lnUrlAuthRequestData: data),
            ]

        case let .lnUrlError(
            data
        ):
            return [
                "type": "lnUrlError",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]
        }
    }

    static func asLnUrlCallbackStatus(data: [String: Any?]) throws -> LnUrlCallbackStatus {
        let type = data["type"] as! String
        if type == "ok" {
            return LnUrlCallbackStatus.ok
        }
        if type == "errorStatus" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type LnUrlCallbackStatus") }
            let _data = try asLnUrlErrorData(data: dataTmp)

            return LnUrlCallbackStatus.errorStatus(data: _data)
        }

        throw SdkError.Generic(message: "Invalid enum variant \(type) for enum LnUrlCallbackStatus")
    }

    static func dictionaryOf(lnUrlCallbackStatus: LnUrlCallbackStatus) -> [String: Any?] {
        switch lnUrlCallbackStatus {
        case .ok:
            return [
                "type": "ok",
            ]

        case let .errorStatus(
            data
        ):
            return [
                "type": "errorStatus",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]
        }
    }

    static func asLnUrlPayResult(data: [String: Any?]) throws -> LnUrlPayResult {
        let type = data["type"] as! String
        if type == "endpointSuccess" {
            var _data: SuccessActionProcessed?
            if let dataTmp = data["data"] as? [String: Any?] {
                _data = try asSuccessActionProcessed(data: dataTmp)
            }

            return LnUrlPayResult.endpointSuccess(data: _data)
        }
        if type == "endpointError" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type LnUrlPayResult") }
            let _data = try asLnUrlErrorData(data: dataTmp)

            return LnUrlPayResult.endpointError(data: _data)
        }

        throw SdkError.Generic(message: "Invalid enum variant \(type) for enum LnUrlPayResult")
    }

    static func dictionaryOf(lnUrlPayResult: LnUrlPayResult) -> [String: Any?] {
        switch lnUrlPayResult {
        case let .endpointSuccess(
            data
        ):
            return [
                "type": "endpointSuccess",
                "data": data == nil ? nil : dictionaryOf(successActionProcessed: data!),
            ]

        case let .endpointError(
            data
        ):
            return [
                "type": "endpointError",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]
        }
    }

    static func asLnUrlWithdrawResult(data: [String: Any?]) throws -> LnUrlWithdrawResult {
        let type = data["type"] as! String
        if type == "ok" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type LnUrlWithdrawResult") }
            let _data = try asLnUrlWithdrawSuccessData(data: dataTmp)

            return LnUrlWithdrawResult.ok(data: _data)
        }
        if type == "errorStatus" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type LnUrlWithdrawResult") }
            let _data = try asLnUrlErrorData(data: dataTmp)

            return LnUrlWithdrawResult.errorStatus(data: _data)
        }

        throw SdkError.Generic(message: "Invalid enum variant \(type) for enum LnUrlWithdrawResult")
    }

    static func dictionaryOf(lnUrlWithdrawResult: LnUrlWithdrawResult) -> [String: Any?] {
        switch lnUrlWithdrawResult {
        case let .ok(
            data
        ):
            return [
                "type": "ok",
                "data": dictionaryOf(lnUrlWithdrawSuccessData: data),
            ]

        case let .errorStatus(
            data
        ):
            return [
                "type": "errorStatus",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]
        }
    }

    static func asNetwork(type: String) throws -> Network {
        switch type {
        case "bitcoin":
            return Network.bitcoin

        case "testnet":
            return Network.testnet

        case "signet":
            return Network.signet

        case "regtest":
            return Network.regtest

        default: throw SdkError.Generic(message: "Invalid variant \(type) for enum Network")
        }
    }

    static func valueOf(network: Network) -> String {
        switch network {
        case .bitcoin:
            return "bitcoin"

        case .testnet:
            return "testnet"

        case .signet:
            return "signet"

        case .regtest:
            return "regtest"
        }
    }

    static func asNodeConfig(data: [String: Any?]) throws -> NodeConfig {
        let type = data["type"] as! String
        if type == "greenlight" {
            guard let configTmp = data["config"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field config for type NodeConfig") }
            let _config = try asGreenlightNodeConfig(data: configTmp)

            return NodeConfig.greenlight(config: _config)
        }

        throw SdkError.Generic(message: "Invalid enum variant \(type) for enum NodeConfig")
    }

    static func dictionaryOf(nodeConfig: NodeConfig) -> [String: Any?] {
        switch nodeConfig {
        case let .greenlight(
            config
        ):
            return [
                "type": "greenlight",
                "config": dictionaryOf(greenlightNodeConfig: config),
            ]
        }
    }

    static func asPaymentDetails(data: [String: Any?]) throws -> PaymentDetails {
        let type = data["type"] as! String
        if type == "ln" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type PaymentDetails") }
            let _data = try asLnPaymentDetails(data: dataTmp)

            return PaymentDetails.ln(data: _data)
        }
        if type == "closedChannel" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type PaymentDetails") }
            let _data = try asClosedChannelPaymentDetails(data: dataTmp)

            return PaymentDetails.closedChannel(data: _data)
        }

        throw SdkError.Generic(message: "Invalid enum variant \(type) for enum PaymentDetails")
    }

    static func dictionaryOf(paymentDetails: PaymentDetails) -> [String: Any?] {
        switch paymentDetails {
        case let .ln(
            data
        ):
            return [
                "type": "ln",
                "data": dictionaryOf(lnPaymentDetails: data),
            ]

        case let .closedChannel(
            data
        ):
            return [
                "type": "closedChannel",
                "data": dictionaryOf(closedChannelPaymentDetails: data),
            ]
        }
    }

    static func asPaymentStatus(type: String) throws -> PaymentStatus {
        switch type {
        case "pending":
            return PaymentStatus.pending

        case "complete":
            return PaymentStatus.complete

        case "failed":
            return PaymentStatus.failed

        default: throw SdkError.Generic(message: "Invalid variant \(type) for enum PaymentStatus")
        }
    }

    static func valueOf(paymentStatus: PaymentStatus) -> String {
        switch paymentStatus {
        case .pending:
            return "pending"

        case .complete:
            return "complete"

        case .failed:
            return "failed"
        }
    }

    static func asPaymentType(type: String) throws -> PaymentType {
        switch type {
        case "sent":
            return PaymentType.sent

        case "received":
            return PaymentType.received

        case "closedChannel":
            return PaymentType.closedChannel

        default: throw SdkError.Generic(message: "Invalid variant \(type) for enum PaymentType")
        }
    }

    static func valueOf(paymentType: PaymentType) -> String {
        switch paymentType {
        case .sent:
            return "sent"

        case .received:
            return "received"

        case .closedChannel:
            return "closedChannel"
        }
    }

    static func asPaymentTypeFilter(type: String) throws -> PaymentTypeFilter {
        switch type {
        case "sent":
            return PaymentTypeFilter.sent

        case "received":
            return PaymentTypeFilter.received

        case "all":
            return PaymentTypeFilter.all

        default: throw SdkError.Generic(message: "Invalid variant \(type) for enum PaymentTypeFilter")
        }
    }

    static func valueOf(paymentTypeFilter: PaymentTypeFilter) -> String {
        switch paymentTypeFilter {
        case .sent:
            return "sent"

        case .received:
            return "received"

        case .all:
            return "all"
        }
    }

    static func asReverseSwapStatus(type: String) throws -> ReverseSwapStatus {
        switch type {
        case "initial":
            return ReverseSwapStatus.initial

        case "inProgress":
            return ReverseSwapStatus.inProgress

        case "cancelled":
            return ReverseSwapStatus.cancelled

        case "completedSeen":
            return ReverseSwapStatus.completedSeen

        case "completedConfirmed":
            return ReverseSwapStatus.completedConfirmed

        default: throw SdkError.Generic(message: "Invalid variant \(type) for enum ReverseSwapStatus")
        }
    }

    static func valueOf(reverseSwapStatus: ReverseSwapStatus) -> String {
        switch reverseSwapStatus {
        case .initial:
            return "initial"

        case .inProgress:
            return "inProgress"

        case .cancelled:
            return "cancelled"

        case .completedSeen:
            return "completedSeen"

        case .completedConfirmed:
            return "completedConfirmed"
        }
    }

    static func asSuccessActionProcessed(data: [String: Any?]) throws -> SuccessActionProcessed {
        let type = data["type"] as! String
        if type == "aes" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type SuccessActionProcessed") }
            let _data = try asAesSuccessActionDataDecrypted(data: dataTmp)

            return SuccessActionProcessed.aes(data: _data)
        }
        if type == "message" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type SuccessActionProcessed") }
            let _data = try asMessageSuccessActionData(data: dataTmp)

            return SuccessActionProcessed.message(data: _data)
        }
        if type == "url" {
            guard let dataTmp = data["data"] as? [String: Any?] else { throw SdkError.Generic(message: "Missing mandatory field data for type SuccessActionProcessed") }
            let _data = try asUrlSuccessActionData(data: dataTmp)

            return SuccessActionProcessed.url(data: _data)
        }

        throw SdkError.Generic(message: "Invalid enum variant \(type) for enum SuccessActionProcessed")
    }

    static func dictionaryOf(successActionProcessed: SuccessActionProcessed) -> [String: Any?] {
        switch successActionProcessed {
        case let .aes(
            data
        ):
            return [
                "type": "aes",
                "data": dictionaryOf(aesSuccessActionDataDecrypted: data),
            ]

        case let .message(
            data
        ):
            return [
                "type": "message",
                "data": dictionaryOf(messageSuccessActionData: data),
            ]

        case let .url(
            data
        ):
            return [
                "type": "url",
                "data": dictionaryOf(urlSuccessActionData: data),
            ]
        }
    }

    static func asSwapStatus(type: String) throws -> SwapStatus {
        switch type {
        case "initial":
            return SwapStatus.initial

        case "expired":
            return SwapStatus.expired

        default: throw SdkError.Generic(message: "Invalid variant \(type) for enum SwapStatus")
        }
    }

    static func valueOf(swapStatus: SwapStatus) -> String {
        switch swapStatus {
        case .initial:
            return "initial"

        case .expired:
            return "expired"
        }
    }
}
