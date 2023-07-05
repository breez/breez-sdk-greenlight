import Foundation
import BreezSDK

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
    
    func getBreezServices() throws ->
    BlockingBreezServices {
        if breezServices != nil {
            return breezServices
        }
        
        throw SdkError.Error(message: "BreezServices not initialized")
    }
    
    @objc(mnemonicToSeed:resolver:rejecter:)
    func mnemonicToSeed(_ phrase: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let seed = try BreezSDK.mnemonicToSeed(phrase: phrase)
            
            resolve(seed)
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling mnemonicToSeed", err)
        }
    }
    
    @objc(parseInput:resolver:rejecter:)
    func parseInput(_ input: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let inputType = try BreezSDK.parseInput(s: input)
            
            resolve(BreezSDKMapper.dictionaryOf(inputType: inputType))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling parseInput", err)
        }
    }
    
    @objc(parseInvoice:resolver:rejecter:)
    func parseInvoice(_ invoice: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let lnInvoice = try BreezSDK.parseInvoice(invoice: invoice)
            
            resolve(BreezSDKMapper.dictionaryOf(lnInvoice: lnInvoice))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling parseInvoice", err)
        }
    }
    
    @objc(registerNode:seed:registerCredentials:inviteCode:resolver:rejecter:)
    func registerNode(_ network:String, seed:[UInt8], registerCredentials: [String: Any], inviteCode: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let registerCreds = BreezSDKMapper.asGreenlightCredentials(reqData: registerCredentials)
            let optionalInviteCode = inviteCode.count == 0 ? nil : inviteCode
            let greenlightCredentials = try BreezSDK.registerNode(network: BreezSDKMapper.asNetwork(network: network), seed: seed, registerCredentials: registerCreds, inviteCode: optionalInviteCode)
            
            resolve(BreezSDKMapper.dictionaryOf(greenlightCredentials: greenlightCredentials))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling registerNode", err)
        }
    }
    
    @objc(recoverNode:seed:resolver:rejecter:)
    func recoverNode(_ network:String, seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let greenlightCredentials = try BreezSDK.recoverNode(network: BreezSDKMapper.asNetwork(network: network), seed: seed)
            
            resolve(BreezSDKMapper.dictionaryOf(greenlightCredentials: greenlightCredentials))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling recoverNode", err)
        }
    }
    
    @objc(startLogStream:rejecter:)
    func startLogStream(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try BreezSDK.setLogStream(logStream: BreezSDKLogStream(emitter: self))
            
            resolve(["status": "ok"])
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling setLogStream", err)
        }
    }
    
    @objc(defaultConfig:resolver:rejecter:)
    func defaultConfig(_ envType: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            var config = try BreezSDK.defaultConfig(envType: BreezSDKMapper.asEnvironmentType(envType: envType))
            config.workingDir = RNBreezSDK.breezSdkDirectory.path

            resolve(BreezSDKMapper.dictionaryOf(config: config))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling defaultConfig", err)
        }
    }
    
    @objc(initServices:deviceKey:deviceCert:seed:resolver:rejecter:)
    func initServices(_ config:[String: Any], deviceKey:[UInt8], deviceCert:[UInt8], seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if self.breezServices != nil {
            reject(RNBreezSDK.TAG, "BreezServices already initialized", nil)
        } else if let config = BreezSDKMapper.asConfig(config: config) {
            let greenlightCredentials = GreenlightCredentials(deviceKey: deviceKey, deviceCert: deviceCert)
            
            do {
                self.breezServices = try BreezSDK.initServices(config: config, seed: seed, creds: greenlightCredentials, listener: BreezSDKListener(emitter: self))
                
                resolve(["status": "ok"])
            } catch SdkError.Error(let message) {
                reject(RNBreezSDK.TAG, message, nil)
            } catch let err {
                reject(RNBreezSDK.TAG, "Error calling initServices", err)
            }
        } else {
            reject(RNBreezSDK.TAG, "Invalid config", nil)
        }
    }
    
    @objc(start:rejecter:)
    func start(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().start()
            resolve(["status": "ok"])
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling start", err)
        }
    }
    
    @objc(sync:rejecter:)
    func sync(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().sync()
            resolve(["status": "ok"])
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling sync", err)
        }
   }
    
    @objc(stop:rejecter:)
    func stop(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().stop()
            resolve(["status": "ok"])
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling stop", err)
        }
    }
    
    @objc(sendPayment:amountSats:resolver:rejecter:)
    func sendPayment(_ bolt11:String, amountSats:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let optionalAmountSats = amountSats == 0 ? nil : amountSats
            let payment = try getBreezServices().sendPayment(bolt11: bolt11, amountSats: optionalAmountSats)
            resolve(BreezSDKMapper.dictionaryOf(payment: payment))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling sendPayment", err)
        }
    }
    
    @objc(sendSpontaneousPayment:amountSats:resolver:rejecter:)
    func sendSpontaneousPayment(_ nodeId:String, amountSats:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let payment = try getBreezServices().sendSpontaneousPayment(nodeId: nodeId, amountSats: amountSats)
            resolve(BreezSDKMapper.dictionaryOf(payment: payment))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling sendSpontaneousPayment", err)
        }
    }
    
    @objc(receivePayment:description:resolver:rejecter:)
    func receivePayment(_ amountSats:UInt64, description:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let lnInvoice = try getBreezServices().receivePayment(amountSats: amountSats, description: description)
            resolve(BreezSDKMapper.dictionaryOf(lnInvoice: lnInvoice))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling receivePayment", err)
        }
    }
    
    @objc(lnurlAuth:resolver:rejecter:)
    func lnurlAuth(_ reqData:[String: Any], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let lnUrlAuthRequestData = BreezSDKMapper.asLnUrlAuthRequestData(reqData: reqData) {
            do {
                let lnUrlCallbackStatus = try getBreezServices().lnurlAuth(reqData: lnUrlAuthRequestData)
                
                resolve(BreezSDKMapper.dictionaryOf(lnUrlCallbackStatus: lnUrlCallbackStatus))
            } catch SdkError.Error(let message) {
                reject(RNBreezSDK.TAG, message, nil)
            } catch let err {
                reject(RNBreezSDK.TAG, "Error calling lnurlAuth", err)
            }
        } else {
            reject(RNBreezSDK.TAG, "Invalid reqData", nil)
        }
    }
    
    @objc(payLnurl:amountSats:comment:resolver:rejecter:)
    func payLnurl(_ reqData:[String: Any], amountSats:UInt64, comment:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let lnUrlPayRequestData = BreezSDKMapper.asLnUrlPayRequestData(reqData: reqData) {
            do {
                let optionalComment = comment.count == 0 ? nil : comment
                let lnUrlPayResult = try getBreezServices().payLnurl(reqData: lnUrlPayRequestData, amountSats: amountSats, comment: optionalComment)
                
                resolve(BreezSDKMapper.dictionaryOf(lnUrlPayResult: lnUrlPayResult))
            } catch SdkError.Error(let message) {
                reject(RNBreezSDK.TAG, message, nil)
            } catch let err {
                reject(RNBreezSDK.TAG, "Error calling payLnurl", err)
            }
        } else {
            reject(RNBreezSDK.TAG, "Invalid reqData", nil)
        }
    }
    
    @objc(withdrawLnurl:amountSats:description:resolver:rejecter:)
    func withdrawLnurl(_ reqData:[String: Any], amountSats:UInt64, description:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let lnUrlWithdrawRequestData = BreezSDKMapper.asLnUrlWithdrawRequestData(reqData: reqData) {
            do {
                let optionalDescription = description.count == 0 ? nil : description
                let lnUrlCallbackStatus = try getBreezServices().withdrawLnurl(reqData: lnUrlWithdrawRequestData, amountSats: amountSats, description: optionalDescription)
                
                resolve(BreezSDKMapper.dictionaryOf(lnUrlCallbackStatus: lnUrlCallbackStatus))
            } catch SdkError.Error(let message) {
                reject(RNBreezSDK.TAG, message, nil)
            } catch let err {
                reject(RNBreezSDK.TAG, "Error calling withdrawLnurl", err)
            }
        } else {
            reject(RNBreezSDK.TAG, "Invalid reqData", nil)
        }
    }
    
    @objc(nodeInfo:rejecter:)
    func nodeInfo(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let nodeState = try getBreezServices().nodeInfo() {
                resolve(BreezSDKMapper.dictionaryOf(nodeState: nodeState))
            } else {
                reject(RNBreezSDK.TAG, "No available node info", nil)
            }
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling nodeInfo", err)
        }
    }
    
    @objc(listPayments:fromTimestamp:toTimestamp:resolver:rejecter:)
    func listPayments(_ filter:String, fromTimestamp:Int64, toTimestamp:Int64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let optionalFromTimestamp = fromTimestamp == 0 ? nil : fromTimestamp
            let optionalToTimestamp = toTimestamp == 0 ? nil : toTimestamp
            let payments = try getBreezServices().listPayments(filter: BreezSDKMapper.asPaymentTypeFilter(filter: filter), fromTimestamp: optionalFromTimestamp, toTimestamp: optionalToTimestamp)
            resolve(BreezSDKMapper.arrayOf(payments: payments))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling listPayments", err)
        }
    }
    
    @objc(sweep:feeRateSatsPerVbyte:resolver:rejecter:)
    func sweep(_ toAddress:String, feeRateSatsPerVbyte:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().sweep(toAddress: toAddress, feeRateSatsPerVbyte: feeRateSatsPerVbyte)
            resolve(["status": "ok"])
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling sweep", err)
        }
    }
    
    @objc(fetchFiatRates:rejecter:)
    func fetchFiatRates(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let rates = try getBreezServices().fetchFiatRates()
            resolve(BreezSDKMapper.arrayOf(rates: rates))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling fetchFiatRates", err)
        }
    }
    
    @objc(listFiatCurrencies:rejecter:)
    func listFiatCurrencies(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let fiatCurrencies = try getBreezServices().listFiatCurrencies()
            resolve(BreezSDKMapper.arrayOf(fiatCurrencies: fiatCurrencies))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling listFiatCurrencies", err)
        }
    }
    
    @objc(listLsps:rejecter:)
    func listLsps(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let lsps = try getBreezServices().listLsps()
            resolve(BreezSDKMapper.arrayOf(lsps: lsps))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling listLsps", err)
        }
    }
    
    @objc(connectLsp:resolver:rejecter:)
    func connectLsp(_ lspId:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().connectLsp(lspId: lspId)
            resolve(["status": "ok"])
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling connectLsp", err)
        }
    }
    
    @objc(fetchLspInfo:resolver:rejecter:)
    func fetchLspInfo(_ lspId:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let lspInformation = try getBreezServices().fetchLspInfo(lspId: lspId) {
                resolve(BreezSDKMapper.dictionaryOf(lspInformation: lspInformation))
            } else {
                reject(RNBreezSDK.TAG, "No available lsp info", nil)
            }
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling fetchLspInfo", err)
        }
    }
    
    @objc(lspId:rejecter:)
    func lspId(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let lspId = try getBreezServices().lspId() {
                resolve(lspId)
            } else {
                reject(RNBreezSDK.TAG, "No available lsp id", nil)
            }
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling lspId", err)
        }
    }
    
    @objc(closeLspChannels:rejecter:)
    func closeLspChannels(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().closeLspChannels()
            resolve(["status": "ok"])
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling closeLspChannels", err)
        }

    }
    
    @objc(receiveOnchain:rejecter:)
    func receiveOnchain(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let swapInfo = try getBreezServices().receiveOnchain()
            resolve(BreezSDKMapper.dictionaryOf(swapInfo: swapInfo))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling receiveOnchain", err)
        }
    }
    
    @objc(inProgressSwap:rejecter:)
    func inProgressSwap(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let swapInfo = try getBreezServices().inProgressSwap() {
                resolve(BreezSDKMapper.dictionaryOf(swapInfo: swapInfo))
            } else {
                reject(RNBreezSDK.TAG, "No available in progress swap", nil)
            }
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling inProgressSwap", err)
        }
    }
    
    @objc(listRefundables:rejecter:)
    func listRefundables(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let swapInfos = try getBreezServices().listRefundables()
            resolve(BreezSDKMapper.arrayOf(swapInfos: swapInfos))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling listRefundables", err)
        }
    }
    
    @objc(refund:fromTimestamp:toTimestamp:resolver:rejecter:)
    func refund(_ swapAddress:String, toAddress:String, satPerVbyte:UInt32, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let result = try getBreezServices().refund(swapAddress: swapAddress, toAddress: toAddress, satPerVbyte: satPerVbyte)
            resolve(result)
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling refund", err)
        }
    }

    @objc(fetchReverseSwapFees:rejecter:)
    func fetchReverseSwapFees(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let fees = try getBreezServices().fetchReverseSwapFees()
            resolve(fees)
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling fetchReverseSwapFees", err)
        }
    }

    @objc(inProgressReverseSwaps:rejecter:)
    func inProgressReverseSwaps(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let swaps = try getBreezServices().inProgressReverseSwaps()
            resolve(BreezSDKMapper.arrayOf(reverseSwapInfos: swaps))            
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling inProgressReverseSwaps", err)
        }
    }

    @objc(sendOnchain:onchainRecipientAddress:pairHash:satPerVbyte:resolver:rejecter:)
    func sendOnchain(_ amountSat:UInt64, onchainRecipientAddress:String, pairHash:String, satPerVbyte:UInt64, resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let swapInfo = try getBreezServices().sendOnchain(amountSat: amountSat, onchainRecipientAddress: onchainRecipientAddress, pairHash: pairHash, satPerVbyte: satPerVbyte)
            resolve(BreezSDKMapper.dictionaryOf(reverseSwapInfo: swapInfo))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling sendOnchain", err)
        }
    }
    
    @objc(executeDevCommand:resolver:rejecter:)
    func executeDevCommand(_ command:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let result = try getBreezServices().executeDevCommand(command: command)
            resolve(result)
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling executeDevCommand", err)
        }
    }
    
    @objc(recommendedFees:rejecter:)
    func recommendedFees(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let fees = try getBreezServices().recommendedFees()
            resolve(BreezSDKMapper.dictionaryOf(recommendedFees: fees))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling recommendedFees", err)
        }
    }

    @objc(buyBitcoin:resolver:rejecter:)
    func buyBitcoin(_ provider: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let buyBitcoinProvider = try BreezSDKMapper.asBitcoinProvider(provider: provider)
            let result = try getBreezServices().buyBitcoin(provider: buyBitcoinProvider)
            resolve(result)
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling buyBitcoin", err)
        }
    }
    
    @objc(backup:rejecter:)
    func backup(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().backup()
            resolve(["status": "ok"])
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling backup", err)
        }
    }
    
    @objc(backupStatus:rejecter:)
    func backupStatus(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let status = try getBreezServices().backupStatus()
            resolve(BreezSDKMapper.dictionaryOf(backupStatus: status))
        } catch SdkError.Error(let message) {
            reject(RNBreezSDK.TAG, message, nil)
        } catch let err {
            reject(RNBreezSDK.TAG, "Error calling backupStatus", err)
        }
    }
}
