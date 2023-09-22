import BreezSDK
import Foundation

@objc(RNBreezSDK)
class RNBreezSDK: RCTEventEmitter {
    static let TAG: String = "BreezSDK"

    private var breezServices: BlockingBreezServices!

    static var breezSdkDirectory: URL {
        let applicationDirectory = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
        let breezSdkDirectory = applicationDirectory.appendingPathComponent("breezSdk", isDirectory: true)

        if !FileManager.default.fileExists(atPath: breezSdkDirectory.path) {
            try! FileManager.default.createDirectory(atPath: breezSdkDirectory.path, withIntermediateDirectories: true)
        }

        return breezSdkDirectory
    }

    @objc
    override static func moduleName() -> String! {
        TAG
    }

    override func supportedEvents() -> [String]! {
        return [BreezSDKListener.emitterName, BreezSDKLogStream.emitterName]
    }

    @objc
    override static func requiresMainQueueSetup() -> Bool {
        return false
    }

    func getBreezServices() throws -> BlockingBreezServices {
        if breezServices != nil {
            return breezServices
        }

        throw SdkError.Generic(message: "BreezServices not initialized")
    }

    @objc(parseInvoice:resolve:reject:)
    func parseInvoice(_ invoice: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try BreezSDK.parseInvoice(invoice: invoice)
            resolve(BreezSDKMapper.dictionaryOf(lnInvoice: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(parseInput:resolve:reject:)
    func parseInput(_ s: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try BreezSDK.parseInput(s: s)
            resolve(BreezSDKMapper.dictionaryOf(inputType: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(mnemonicToSeed:resolve:reject:)
    func mnemonicToSeed(_ phrase: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try BreezSDK.mnemonicToSeed(phrase: phrase)
            resolve(res)
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(defaultConfig:apiKey:nodeConfig:resolve:reject:)
    func defaultConfig(_ envType: String, apiKey: String, nodeConfig: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let envTypeTmp = try BreezSDKMapper.asEnvironmentType(type: envType)
            let nodeConfigTmp = try BreezSDKMapper.asNodeConfig(data: nodeConfig)
            var res = BreezSDK.defaultConfig(envType: envTypeTmp, apiKey: apiKey, nodeConfig: nodeConfigTmp)
            res.workingDir = RNBreezSDK.breezSdkDirectory.path
            resolve(BreezSDKMapper.dictionaryOf(config: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(staticBackup:resolve:reject:)
    func staticBackup(_ request: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let staticBackupRequest = try BreezSDKMapper.asStaticBackupRequest(data: request)
            var res = try BreezSDK.staticBackup(request: staticBackupRequest)
            resolve(BreezSDKMapper.dictionaryOf(staticBackupResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(setLogStream:reject:)
    func setLogStream(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try BreezSDK.setLogStream(logStream: BreezSDKLogStream(emitter: self))
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(connect:seed:resolve:reject:)
    func connect(_ config: [String: Any], seed: [UInt8], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        if breezServices != nil {
            reject(RNBreezSDK.TAG, "BreezServices already initialized", nil)
            return
        }

        do {
            let configTmp = try BreezSDKMapper.asConfig(data: config)
            breezServices = try BreezSDK.connect(config: configTmp, seed: seed, listener: BreezSDKListener(emitter: self))
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(disconnect:reject:)
    func disconnect(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBreezServices().disconnect()
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sendPayment:amountSats:resolve:reject:)
    func sendPayment(_ bolt11: String, amountSats: UInt64, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let amountSatsTmp = amountSats == 0 ? nil : amountSats
            var res = try getBreezServices().sendPayment(bolt11: bolt11, amountSats: amountSatsTmp)
            resolve(BreezSDKMapper.dictionaryOf(payment: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sendSpontaneousPayment:amountSats:resolve:reject:)
    func sendSpontaneousPayment(_ nodeId: String, amountSats: UInt64, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().sendSpontaneousPayment(nodeId: nodeId, amountSats: amountSats)
            resolve(BreezSDKMapper.dictionaryOf(payment: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(receivePayment:resolve:reject:)
    func receivePayment(_ reqData: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let receivePaymentRequest = try BreezSDKMapper.asReceivePaymentRequest(data: reqData)
            var res = try getBreezServices().receivePayment(reqData: receivePaymentRequest)
            resolve(BreezSDKMapper.dictionaryOf(receivePaymentResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(payLnurl:amountSats:comment:resolve:reject:)
    func payLnurl(_ reqData: [String: Any], amountSats: UInt64, comment: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let lnUrlPayRequestData = try BreezSDKMapper.asLnUrlPayRequestData(data: reqData)
            let commentTmp = comment.isEmpty ? nil : comment
            var res = try getBreezServices().payLnurl(reqData: lnUrlPayRequestData, amountSats: amountSats, comment: commentTmp)
            resolve(BreezSDKMapper.dictionaryOf(lnUrlPayResult: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(withdrawLnurl:amountSats:description:resolve:reject:)
    func withdrawLnurl(_ reqData: [String: Any], amountSats: UInt64, description: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let lnUrlWithdrawRequestData = try BreezSDKMapper.asLnUrlWithdrawRequestData(data: reqData)
            let descriptionTmp = description.isEmpty ? nil : description
            var res = try getBreezServices().withdrawLnurl(reqData: lnUrlWithdrawRequestData, amountSats: amountSats, description: descriptionTmp)
            resolve(BreezSDKMapper.dictionaryOf(lnUrlWithdrawResult: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(lnurlAuth:resolve:reject:)
    func lnurlAuth(_ reqData: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let lnUrlAuthRequestData = try BreezSDKMapper.asLnUrlAuthRequestData(data: reqData)
            var res = try getBreezServices().lnurlAuth(reqData: lnUrlAuthRequestData)
            resolve(BreezSDKMapper.dictionaryOf(lnUrlCallbackStatus: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(nodeInfo:reject:)
    func nodeInfo(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().nodeInfo()
            resolve(BreezSDKMapper.dictionaryOf(nodeState: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(signMessage:resolve:reject:)
    func signMessage(_ request: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let signMessageRequest = try BreezSDKMapper.asSignMessageRequest(data: request)
            var res = try getBreezServices().signMessage(request: signMessageRequest)
            resolve(BreezSDKMapper.dictionaryOf(signMessageResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(checkMessage:resolve:reject:)
    func checkMessage(_ request: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let checkMessageRequest = try BreezSDKMapper.asCheckMessageRequest(data: request)
            var res = try getBreezServices().checkMessage(request: checkMessageRequest)
            resolve(BreezSDKMapper.dictionaryOf(checkMessageResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(backupStatus:reject:)
    func backupStatus(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().backupStatus()
            resolve(BreezSDKMapper.dictionaryOf(backupStatus: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(backup:reject:)
    func backup(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBreezServices().backup()
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(paymentByHash:resolve:reject:)
    func paymentByHash(_ hash: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().paymentByHash(hash: hash)
            if res != nil {
                resolve(BreezSDKMapper.dictionaryOf(payment: res!))
            } else {
                resolve(nil)
            }
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listPayments:resolve:reject:)
    func listPayments(_ request: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let listPaymentsRequest = try BreezSDKMapper.asListPaymentsRequest(data: request)
            var res = try getBreezServices().listPayments(request: listPaymentsRequest)
            resolve(BreezSDKMapper.arrayOf(paymentList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sweep:feeRateSatsPerVbyte:resolve:reject:)
    func sweep(_ toAddress: String, feeRateSatsPerVbyte: UInt64, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().sweep(toAddress: toAddress, feeRateSatsPerVbyte: feeRateSatsPerVbyte)
            resolve(BreezSDKMapper.dictionaryOf(sweepResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(fetchFiatRates:reject:)
    func fetchFiatRates(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().fetchFiatRates()
            resolve(BreezSDKMapper.arrayOf(rateList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listFiatCurrencies:reject:)
    func listFiatCurrencies(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().listFiatCurrencies()
            resolve(BreezSDKMapper.arrayOf(fiatCurrencyList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listLsps:reject:)
    func listLsps(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().listLsps()
            resolve(BreezSDKMapper.arrayOf(lspInformationList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(connectLsp:resolve:reject:)
    func connectLsp(_ lspId: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBreezServices().connectLsp(lspId: lspId)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(fetchLspInfo:resolve:reject:)
    func fetchLspInfo(_ lspId: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().fetchLspInfo(lspId: lspId)
            if res != nil {
                resolve(BreezSDKMapper.dictionaryOf(lspInformation: res!))
            } else {
                resolve(nil)
            }
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(openChannelFee:resolve:reject:)
    func openChannelFee(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let openChannelFeeRequest = try BreezSDKMapper.asOpenChannelFeeRequest(data: req)
            var res = try getBreezServices().openChannelFee(req: openChannelFeeRequest)
            resolve(BreezSDKMapper.dictionaryOf(openChannelFeeResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(lspId:reject:)
    func lspId(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().lspId()
            if res != nil {
                resolve(res!)
            } else {
                resolve(nil)
            }
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(lspInfo:reject:)
    func lspInfo(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().lspInfo()
            resolve(BreezSDKMapper.dictionaryOf(lspInformation: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(closeLspChannels:reject:)
    func closeLspChannels(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBreezServices().closeLspChannels()
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(receiveOnchain:resolve:reject:)
    func receiveOnchain(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let receiveOnchainRequest = try BreezSDKMapper.asReceiveOnchainRequest(data: req)
            var res = try getBreezServices().receiveOnchain(req: receiveOnchainRequest)
            resolve(BreezSDKMapper.dictionaryOf(swapInfo: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(inProgressSwap:reject:)
    func inProgressSwap(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().inProgressSwap()
            if res != nil {
                resolve(BreezSDKMapper.dictionaryOf(swapInfo: res!))
            } else {
                resolve(nil)
            }
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listRefundables:reject:)
    func listRefundables(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().listRefundables()
            resolve(BreezSDKMapper.arrayOf(swapInfoList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(refund:toAddress:satPerVbyte:resolve:reject:)
    func refund(_ swapAddress: String, toAddress: String, satPerVbyte: UInt32, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().refund(swapAddress: swapAddress, toAddress: toAddress, satPerVbyte: satPerVbyte)
            resolve(res)
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(fetchReverseSwapFees:resolve:reject:)
    func fetchReverseSwapFees(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let reverseSwapFeesRequest = try BreezSDKMapper.asReverseSwapFeesRequest(data: req)
            var res = try getBreezServices().fetchReverseSwapFees(req: reverseSwapFeesRequest)
            resolve(BreezSDKMapper.dictionaryOf(reverseSwapPairInfo: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(inProgressReverseSwaps:reject:)
    func inProgressReverseSwaps(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().inProgressReverseSwaps()
            resolve(BreezSDKMapper.arrayOf(reverseSwapInfoList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sendOnchain:onchainRecipientAddress:pairHash:satPerVbyte:resolve:reject:)
    func sendOnchain(_ amountSat: UInt64, onchainRecipientAddress: String, pairHash: String, satPerVbyte: UInt64, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().sendOnchain(amountSat: amountSat, onchainRecipientAddress: onchainRecipientAddress, pairHash: pairHash, satPerVbyte: satPerVbyte)
            resolve(BreezSDKMapper.dictionaryOf(reverseSwapInfo: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(executeDevCommand:resolve:reject:)
    func executeDevCommand(_ command: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().executeDevCommand(command: command)
            resolve(res)
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sync:reject:)
    func sync(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBreezServices().sync()
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(recommendedFees:reject:)
    func recommendedFees(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBreezServices().recommendedFees()
            resolve(BreezSDKMapper.dictionaryOf(recommendedFees: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(buyBitcoin:resolve:reject:)
    func buyBitcoin(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let buyBitcoinRequest = try BreezSDKMapper.asBuyBitcoinRequest(data: req)
            var res = try getBreezServices().buyBitcoin(req: buyBitcoinRequest)
            resolve(BreezSDKMapper.dictionaryOf(buyBitcoinResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    func rejectErr(err: Error, reject: @escaping RCTPromiseRejectBlock) {
        var errorCode = "Generic"
        var message = "\(err)"
        if let sdkErr = err as? SdkError {
            if let sdkErrAssociated = Mirror(reflecting: sdkErr).children.first {
                if let associatedMessage = Mirror(reflecting: sdkErrAssociated.value).children.first {
                    message = associatedMessage.value as! String
                }
            }
        }
        reject(errorCode, message, err)
    }
}
