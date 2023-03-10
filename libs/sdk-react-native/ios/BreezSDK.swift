import Foundation

@objc(BreezSDK)
class BreezSDK: RCTEventEmitter {
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
        
        throw SdkError.Error(message: "BreezServices not initialized")
    }
    
    @objc(mnemonicToSeed:resolver:rejecter:)
    func mnemonicToSeed(_ phrase: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let seed = try breez_sdk.mnemonicToSeed(phrase: phrase)
            
            resolve(seed)
        } catch let err {
            reject(BreezSDK.TAG, "Error calling mnemonicToSeed", err)
        }
    }
    
    @objc(parseInput:resolver:rejecter:)
    func parseInput(_ input: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let inputType = try breez_sdk.parseInput(s: input)
            
            resolve(BreezSDKMapper.dictionaryOf(inputType: inputType))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling parseInput", err)
        }
    }
    
    @objc(parseInvoice:resolver:rejecter:)
    func parseInvoice(_ invoice: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let lnInvoice = try breez_sdk.parseInvoice(invoice: invoice)
            
            resolve(BreezSDKMapper.dictionaryOf(lnInvoice: lnInvoice))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling parseInvoice", err)
        }
    }
    
    @objc(registerNode:seed:resolver:rejecter:)
    func registerNode(_ network:String, seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let greenlightCredentials = try breez_sdk.registerNode(network: BreezSDKMapper.asNetwork(network: network), seed: seed)
            
            resolve(BreezSDKMapper.dictionaryOf(greenlightCredentials: greenlightCredentials))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling registerNode", err)
        }
    }
    
    @objc(recoverNode:seed:resolver:rejecter:)
    func recoverNode(_ network:String, seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let greenlightCredentials = try breez_sdk.recoverNode(network: BreezSDKMapper.asNetwork(network: network), seed: seed)
            
            resolve(BreezSDKMapper.dictionaryOf(greenlightCredentials: greenlightCredentials))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling recoverNode", err)
        }
    }
    
    @objc(startLogStream:rejecter:)
    func startLogStream(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try breez_sdk.setLogStream(logStream: BreezSDKLogStream(emitter: self))
            
            resolve(["status": "ok"])
        } catch let err {
            reject(BreezSDK.TAG, "Error calling setLogStream", err)
        }
    }
    
    @objc(initServices:deviceKey:deviceCert:seed:resolver:rejecter:)
    func initServices(_ apiKey:String, deviceKey:[UInt8], deviceCert:[UInt8], seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if self.breezServices != nil {
            reject(BreezSDK.TAG, "BreezServices already initialized", nil)
        } else {
            let greenlightCredentials = GreenlightCredentials(deviceKey: deviceKey, deviceCert: deviceCert)
            var config = breez_sdk.defaultConfig(envType: EnvironmentType.production)
            config.apiKey = apiKey
            config.workingDir = BreezSDK.breezSdkDirectory.absoluteString
            
            do {
                self.breezServices = try breez_sdk.initServices(config: config, seed: seed, creds: greenlightCredentials, listener: BreezSDKListener(emitter: self))
                
                resolve(["status": "ok"])
            } catch let err {
                reject(BreezSDK.TAG, "Error calling initServices", err)
            }
        }
    }
    
    @objc(start:rejecter:)
    func start(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().start()
            resolve(["status": "ok"])
        } catch let err {
            reject(BreezSDK.TAG, "Error calling start", err)
        }
    }
    
    @objc(sync:rejecter:)
    func sync(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().sync()
            resolve(["status": "ok"])
        } catch let err {
            reject(BreezSDK.TAG, "Error calling sync", err)
        }
   }
    
    @objc(stop:rejecter:)
    func stop(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().stop()
            resolve(["status": "ok"])
        } catch let err {
            reject(BreezSDK.TAG, "Error calling stop", err)
        }
    }
    
    @objc(sendPayment:amountSats:resolver:rejecter:)
    func sendPayment(_ bolt11:String, amountSats:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let optionalAmountSats = amountSats == 0 ? nil : amountSats
            let payment = try getBreezServices().sendPayment(bolt11: bolt11, amountSats: optionalAmountSats)
            resolve(BreezSDKMapper.dictionaryOf(payment: payment))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling sendPayment", err)
        }
    }
    
    @objc(sendSpontaneousPayment:amountSats:resolver:rejecter:)
    func sendSpontaneousPayment(_ nodeId:String, amountSats:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let payment = try getBreezServices().sendSpontaneousPayment(nodeId: nodeId, amountSats: amountSats)
            resolve(BreezSDKMapper.dictionaryOf(payment: payment))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling sendSpontaneousPayment", err)
        }
    }
    
    @objc(receivePayment:description:resolver:rejecter:)
    func receivePayment(_ amountSats:UInt64, description:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let lnInvoice = try getBreezServices().receivePayment(amountSats: amountSats, description: description)
            resolve(BreezSDKMapper.dictionaryOf(lnInvoice: lnInvoice))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling receivePayment", err)
        }
    }
    
    @objc(lnurlAuth:resolver:rejecter:)
    func lnurlAuth(_ reqData:[String: Any], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let lnUrlAuthRequestData = BreezSDKMapper.asLnUrlAuthRequestData(reqData: reqData) {
            do {
                let lnUrlAuthCallbackStatus = try getBreezServices().lnurlAuth(reqData: lnUrlAuthRequestData)
                
                resolve(BreezSDKMapper.dictionaryOf(lnUrlAuthCallbackStatus: lnUrlAuthCallbackStatus))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling lnurlAuth", err)
            }
        } else {
            reject(BreezSDK.TAG, "Invalid reqData", nil)
        }
    }
    
    @objc(withdrawLnurl:amountSats:description:resolver:rejecter:)
    func withdrawLnurl(_ reqData:[String: Any], amountSats:UInt64, description:String?, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let lnUrlWithdrawRequestData = BreezSDKMapper.asLnUrlWithdrawRequestData(reqData: reqData) {
            do {
                let lnUrlWithdrawCallbackStatus = try getBreezServices().withdrawLnurl(reqData: lnUrlWithdrawRequestData, amountSats: amountSats, description: description)
                
                resolve(BreezSDKMapper.dictionaryOf(lnUrlWithdrawCallbackStatus: lnUrlWithdrawCallbackStatus))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling withdrawLnurl", err)
            }
        } else {
            reject(BreezSDK.TAG, "Invalid reqData", nil)
        }
    }
    
    @objc(nodeInfo:rejecter:)
    func nodeInfo(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let nodeState = try getBreezServices().nodeInfo() {
                resolve(BreezSDKMapper.dictionaryOf(nodeState: nodeState))
            } else {
                reject(BreezSDK.TAG, "No available node info", nil)
            }
        } catch let err {
            reject(BreezSDK.TAG, "Error calling nodeInfo", err)
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
            reject(BreezSDK.TAG, "Error calling listPayments", err)
        }
    }
    
    @objc(sweep:feeRateSatsPerByte:resolver:rejecter:)
    func sweep(_ toAddress:String, feeRateSatsPerByte:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().sweep(toAddress: toAddress, feeRateSatsPerByte: feeRateSatsPerByte)
            resolve(["status": "ok"])
        } catch let err {
            reject(BreezSDK.TAG, "Error calling sweep", err)
        }
    }
    
    @objc(fetchFiatRates:rejecter:)
    func fetchFiatRates(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let rates = try getBreezServices().fetchFiatRates()
            resolve(BreezSDKMapper.arrayOf(rates: rates))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling fetchFiatRates", err)
        }
    }
    
    @objc(listFiatCurrencies:rejecter:)
    func listFiatCurrencies(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let fiatCurrencies = try getBreezServices().listFiatCurrencies()
            resolve(BreezSDKMapper.arrayOf(fiatCurrencies: fiatCurrencies))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling listFiatCurrencies", err)
        }
    }
    
    @objc(listLsps:rejecter:)
    func listLsps(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let lsps = try getBreezServices().listLsps()
            resolve(BreezSDKMapper.arrayOf(lsps: lsps))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling listLsps", err)
        }
    }
    
    @objc(connectLsp:resolver:rejecter:)
    func connectLsp(_ lspId:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().connectLsp(lspId: lspId)
            resolve(["status": "ok"])
        } catch let err {
            reject(BreezSDK.TAG, "Error calling connectLsp", err)
        }
    }
    
    @objc(fetchLspInfo:resolver:rejecter:)
    func fetchLspInfo(_ lspId:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let lspInformation = try getBreezServices().fetchLspInfo(lspId: lspId) {
                resolve(BreezSDKMapper.dictionaryOf(lspInformation: lspInformation))
            } else {
                reject(BreezSDK.TAG, "No available lsp info", nil)
            }
        } catch let err {
            reject(BreezSDK.TAG, "Error calling fetchLspInfo", err)
        }
    }
    
    @objc(lspId:rejecter:)
    func lspId(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let lspId = try getBreezServices().lspId() {
                resolve(lspId)
            } else {
                reject(BreezSDK.TAG, "No available lsp id", nil)
            }
        } catch let err {
            reject(BreezSDK.TAG, "Error calling lspId", err)
        }
    }
    
    @objc(closeLspChannels:rejecter:)
    func closeLspChannels(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try getBreezServices().closeLspChannels()
            resolve(["status": "ok"])
        } catch let err {
            reject(BreezSDK.TAG, "Error calling closeLspChannels", err)
        }

    }
    
    @objc(receiveOnchain:rejecter:)
    func receiveOnchain(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let swapInfo = try getBreezServices().receiveOnchain()
            resolve(BreezSDKMapper.dictionaryOf(swapInfo: swapInfo))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling receiveOnchain", err)
        }
    }
    
    @objc(inProgressSwap:rejecter:)
    func inProgressSwap(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            if let swapInfo = try getBreezServices().inProgressSwap() {
                resolve(BreezSDKMapper.dictionaryOf(swapInfo: swapInfo))
            } else {
                reject(BreezSDK.TAG, "No available in progress swap", nil)
            }
        } catch let err {
            reject(BreezSDK.TAG, "Error calling inProgressSwap", err)
        }
    }
    
    @objc(listRefundables:rejecter:)
    func listRefundables(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let swapInfos = try getBreezServices().listRefundables()
            resolve(BreezSDKMapper.arrayOf(swapInfos: swapInfos))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling listRefundables", err)
        }
    }
    
    @objc(refund:fromTimestamp:toTimestamp:resolver:rejecter:)
    func refund(_ swapAddress:String, toAddress:String, satPerVbyte:UInt32, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let result = try getBreezServices().refund(swapAddress: swapAddress, toAddress: toAddress, satPerVbyte: satPerVbyte)
            resolve(result)
        } catch let err {
            reject(BreezSDK.TAG, "Error calling refund", err)
        }
    }
    
    @objc(executeDevCommand:resolver:rejecter:)
    func executeDevCommand(_ command:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let result = try getBreezServices().executeDevCommand(command: command)
            resolve(result)
        } catch let err {
            reject(BreezSDK.TAG, "Error calling executeDevCommand", err)
        }
    }
    
    @objc(recommendedFees:rejecter:)
    func recommendedFees(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let fees = try getBreezServices().recommendedFees()
            resolve(BreezSDKMapper.dictionaryOf(recommendedFees: fees))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling recommendedFees", err)
        }
    }
}
