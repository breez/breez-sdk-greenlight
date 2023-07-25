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
        
        throw SdkError.Generic(message: "BreezServices not initialized")
    }
    
    @objc(mnemonicToSeed:resolver:rejecter:)
    func mnemonicToSeed(_ phrase: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let seed = try BreezSDK.mnemonicToSeed(phrase: phrase)        
            resolve(seed)        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(parseInput:resolver:rejecter:)
    func parseInput(_ input: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let inputType = try BreezSDK.parseInput(s: input)            
            resolve(BreezSDKMapper.dictionaryOf(inputType: inputType))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(parseInvoice:resolver:rejecter:)
    func parseInvoice(_ invoice: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let lnInvoice = try BreezSDK.parseInvoice(invoice: invoice)            
            resolve(BreezSDKMapper.dictionaryOf(lnInvoice: lnInvoice))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(startLogStream:rejecter:)
    func startLogStream(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try BreezSDK.setLogStream(logStream: BreezSDKLogStream(emitter: self))            
            resolve(["status": "ok"])        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(defaultConfig:apiKey:nodeConfigMap:resolver:rejecter:)
    func defaultConfig(_ envType: String, apiKey: String, nodeConfigMap: [String: Any], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let nodeConfig = BreezSDKMapper.asNodeConfig(nodeConfig: nodeConfigMap) {
                var config = try BreezSDK.defaultConfig(envType: BreezSDKMapper.asEnvironmentType(envType: envType), apiKey: apiKey, nodeConfig: nodeConfig)
                config.workingDir = RNBreezSDK.breezSdkDirectory.path                
                resolve(BreezSDKMapper.dictionaryOf(config: config))
            } else {
                reject(RNBreezSDK.TAG, "Invalid node config", nil)
            }        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(connect:seed:resolver:rejecter:)
    func connect(_ config:[String: Any], seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if self.breezServices != nil {
            reject(RNBreezSDK.TAG, "BreezServices already initialized", nil)
        } else if let config = BreezSDKMapper.asConfig(config: config) {
            do {
                self.breezServices = try BreezSDK.connect(config: config, seed: seed, listener: BreezSDKListener(emitter: self))                
                resolve(["status": "ok"])
            } catch let err {
                rejectErr(err: err, reject: reject)
            }
        } else {
            rejectErr(err: SdkError.Generic(message:"Invalid config"), reject: reject)            
        }
    }
    
    @objc(sync:rejecter:)
    func sync(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().sync()
            resolve(["status": "ok"])        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
   }
    
    @objc(disconnect:rejecter:)
    func disconnect(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().disconnect()
            resolve(["status": "ok"])        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(sendPayment:amountSats:resolver:rejecter:)
    func sendPayment(_ bolt11:String, amountSats:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let optionalAmountSats = amountSats == 0 ? nil : amountSats
            let payment = try getBreezServices().sendPayment(bolt11: bolt11, amountSats: optionalAmountSats)
            resolve(BreezSDKMapper.dictionaryOf(payment: payment))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(sendSpontaneousPayment:amountSats:resolver:rejecter:)
    func sendSpontaneousPayment(_ nodeId:String, amountSats:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let payment = try getBreezServices().sendSpontaneousPayment(nodeId: nodeId, amountSats: amountSats)
            resolve(BreezSDKMapper.dictionaryOf(payment: payment))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(receivePayment:description:resolver:rejecter:)
    func receivePayment(_ amountSats:UInt64, description:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let lnInvoice = try getBreezServices().receivePayment(amountSats: amountSats, description: description)
            resolve(BreezSDKMapper.dictionaryOf(lnInvoice: lnInvoice))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(lnurlAuth:resolver:rejecter:)
    func lnurlAuth(_ reqData:[String: Any], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let lnUrlAuthRequestData = BreezSDKMapper.asLnUrlAuthRequestData(reqData: reqData) {
            do {
                let lnUrlCallbackStatus = try getBreezServices().lnurlAuth(reqData: lnUrlAuthRequestData)                
                resolve(BreezSDKMapper.dictionaryOf(lnUrlCallbackStatus: lnUrlCallbackStatus))
            } catch let err {
                reject(RNBreezSDK.TAG, "Error calling lnurlAuth \(err)", err)
            }
        } else {
            rejectErr(err: SdkError.Generic(message:"Invalid reqData"), reject: reject)            
        }
    }
    
    @objc(payLnurl:amountSats:comment:resolver:rejecter:)
    func payLnurl(_ reqData:[String: Any], amountSats:UInt64, comment:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let lnUrlPayRequestData = BreezSDKMapper.asLnUrlPayRequestData(reqData: reqData) {
            do {
                let optionalComment = comment.count == 0 ? nil : comment
                let lnUrlPayResult = try getBreezServices().payLnurl(reqData: lnUrlPayRequestData, amountSats: amountSats, comment: optionalComment)                
                resolve(BreezSDKMapper.dictionaryOf(lnUrlPayResult: lnUrlPayResult))
            } catch let err {
                reject(RNBreezSDK.TAG, "Error calling payLnurl \(err)", err)
            }
        } else {
            rejectErr(err: SdkError.Generic(message:"Invalid reqData"), reject: reject)                        
        }
    }
    
    @objc(withdrawLnurl:amountSats:description:resolver:rejecter:)
    func withdrawLnurl(_ reqData:[String: Any], amountSats:UInt64, description:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let lnUrlWithdrawRequestData = BreezSDKMapper.asLnUrlWithdrawRequestData(reqData: reqData) {
            do {
                let optionalDescription = description.count == 0 ? nil : description
                let lnUrlCallbackStatus = try getBreezServices().withdrawLnurl(reqData: lnUrlWithdrawRequestData, amountSats: amountSats, description: optionalDescription)                
                resolve(BreezSDKMapper.dictionaryOf(lnUrlCallbackStatus: lnUrlCallbackStatus))
            } catch let err {
                rejectErr(err: err, reject: reject)      
            }
        } else {            
            rejectErr(err: SdkError.Generic(message:"Invalid reqData"), reject: reject)
        }
    }
    
    @objc(nodeInfo:rejecter:)
    func nodeInfo(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let nodeState = try getBreezServices().nodeInfo()
            resolve(BreezSDKMapper.dictionaryOf(nodeState: nodeState))                 
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(paymentByHash:resolver:rejecter:)
    func paymentByHash(_ hash:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let payment = try getBreezServices().paymentByHash(hash: hash) {
                resolve(BreezSDKMapper.dictionaryOf(payment: payment))
            } else {
                rejectErr(err: SdkError.Generic(message: "No available payment"), reject: reject)
            }        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listPayments:fromTimestamp:toTimestamp:resolver:rejecter:)
    func listPayments(_ filter:String, fromTimestamp:Int64, toTimestamp:Int64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let optionalFromTimestamp = fromTimestamp == 0 ? nil : fromTimestamp
            let optionalToTimestamp = toTimestamp == 0 ? nil : toTimestamp
            let payments = try getBreezServices().listPayments(filter: BreezSDKMapper.asPaymentTypeFilter(filter: filter), fromTimestamp: optionalFromTimestamp, toTimestamp: optionalToTimestamp)
            resolve(BreezSDKMapper.arrayOf(payments: payments))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(sweep:feeRateSatsPerVbyte:resolver:rejecter:)
    func sweep(_ toAddress:String, feeRateSatsPerVbyte:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().sweep(toAddress: toAddress, feeRateSatsPerVbyte: feeRateSatsPerVbyte)
            resolve(["status": "ok"])        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(fetchFiatRates:rejecter:)
    func fetchFiatRates(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let rates = try getBreezServices().fetchFiatRates()
            resolve(BreezSDKMapper.arrayOf(rates: rates))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(listFiatCurrencies:rejecter:)
    func listFiatCurrencies(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let fiatCurrencies = try getBreezServices().listFiatCurrencies()
            resolve(BreezSDKMapper.arrayOf(fiatCurrencies: fiatCurrencies))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(listLsps:rejecter:)
    func listLsps(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let lsps = try getBreezServices().listLsps()
            resolve(BreezSDKMapper.arrayOf(lsps: lsps))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(connectLsp:resolver:rejecter:)
    func connectLsp(_ lspId:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().connectLsp(lspId: lspId)
            resolve(["status": "ok"])        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(fetchLspInfo:resolver:rejecter:)
    func fetchLspInfo(_ lspId:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let lspInformation = try getBreezServices().fetchLspInfo(lspId: lspId) {
                resolve(BreezSDKMapper.dictionaryOf(lspInformation: lspInformation))
            } else {
                rejectErr(err: SdkError.Generic(message: "No available lsp info"), reject: reject)
            }        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(lspId:rejecter:)
    func lspId(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let lspId = try getBreezServices().lspId() {
                resolve(lspId)
            } else {
                rejectErr(err: SdkError.Generic(message: "No available lsp id"), reject: reject)
            }        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(closeLspChannels:rejecter:)
    func closeLspChannels(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().closeLspChannels()
            resolve(["status": "ok"])        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(receiveOnchain:rejecter:)
    func receiveOnchain(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let swapInfo = try getBreezServices().receiveOnchain()
            resolve(BreezSDKMapper.dictionaryOf(swapInfo: swapInfo))        
        } catch let err {
            rejectErr(err: err, reject: reject)
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
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(listRefundables:rejecter:)
    func listRefundables(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let swapInfos = try getBreezServices().listRefundables()
            resolve(BreezSDKMapper.arrayOf(swapInfos: swapInfos))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(refund:fromTimestamp:toTimestamp:resolver:rejecter:)
    func refund(_ swapAddress:String, toAddress:String, satPerVbyte:UInt32, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let result = try getBreezServices().refund(swapAddress: swapAddress, toAddress: toAddress, satPerVbyte: satPerVbyte)
            resolve(result)        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(fetchReverseSwapFees:rejecter:)
    func fetchReverseSwapFees(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let fees = try getBreezServices().fetchReverseSwapFees()
            resolve(fees)        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(inProgressReverseSwaps:rejecter:)
    func inProgressReverseSwaps(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let swaps = try getBreezServices().inProgressReverseSwaps()
            resolve(BreezSDKMapper.arrayOf(reverseSwapInfos: swaps))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sendOnchain:onchainRecipientAddress:pairHash:satPerVbyte:resolver:rejecter:)
    func sendOnchain(_ amountSat:UInt64, onchainRecipientAddress:String, pairHash:String, satPerVbyte:UInt64, resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let swapInfo = try getBreezServices().sendOnchain(amountSat: amountSat, onchainRecipientAddress: onchainRecipientAddress, pairHash: pairHash, satPerVbyte: satPerVbyte)
            resolve(BreezSDKMapper.dictionaryOf(reverseSwapInfo: swapInfo))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(executeDevCommand:resolver:rejecter:)
    func executeDevCommand(_ command:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let result = try getBreezServices().executeDevCommand(command: command)
            resolve(result)        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(recommendedFees:rejecter:)
    func recommendedFees(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let fees = try getBreezServices().recommendedFees()
            resolve(BreezSDKMapper.dictionaryOf(recommendedFees: fees))        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(buyBitcoin:resolver:rejecter:)
    func buyBitcoin(_ provider: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let buyBitcoinProvider = try BreezSDKMapper.asBitcoinProvider(provider: provider)
            let result = try getBreezServices().buyBitcoin(provider: buyBitcoinProvider)
            resolve(result)        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(backup:rejecter:)
    func backup(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().backup()
            resolve(["status": "ok"])        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(backupStatus:rejecter:)
    func backupStatus(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let status = try getBreezServices().backupStatus()
            resolve(BreezSDKMapper.dictionaryOf(backupStatus: status))        
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
