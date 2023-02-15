import Foundation

@objc(BreezSDK)
class BreezSDK: RCTEventEmitter {
    static let TAG: String = "BreezSDK"
    
    private var breezServices: BlockingBreezServices!
    
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
            let creds = try breez_sdk.registerNode(network: BreezSDKMapper.asNetwork(network: network), seed: seed)
            
            resolve(BreezSDKMapper.dictionaryOf(greenlightCredentials: creds))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling registerNode", err)
        }
    }
    
    @objc(recoverNode:seed:resolver:rejecter:)
    func recoverNode(_ network:String, seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let creds = try breez_sdk.recoverNode(network: BreezSDKMapper.asNetwork(network: network), seed: seed)
            
            resolve(BreezSDKMapper.dictionaryOf(greenlightCredentials: creds))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling recoverNode", err)
        }
    }
    
    @objc(startLogStream:rejecter:)
    func startLogStream(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try breez_sdk.setLogStream(logStream: BreezSDKLogStream(emitter: self))
            
            resolve("Log stream started")
        } catch let err {
            reject(BreezSDK.TAG, "Error calling setLogStream", err)
        }
    }
    
    @objc(initServices:deviceKey:deviceCert:seed:resolver:rejecter:)
    func initServices(_ apiKey:String, deviceKey:[UInt8], deviceCert:[UInt8], seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if self.breezServices != nil {
            reject(BreezSDK.TAG, "BreezServices already initialized", nil)
        } else {
            let creds = GreenlightCredentials(deviceKey: deviceKey, deviceCert: deviceCert)
            var config = breez_sdk.defaultConfig(envType: EnvironmentType.production)
            config.apiKey = apiKey
            
            do {
                self.breezServices = try breez_sdk.initServices(config: config, seed: seed, creds: creds, listener: BreezSDKListener(emitter: self))
                
                resolve("BreezServices initialized")
            } catch let err {
                reject(BreezSDK.TAG, "Error calling initServices", err)
            }
        }
    }
    
    @objc(start:rejecter:)
    func start(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                try breezServices.start()
                
                resolve("BreezServices started")
            } catch let err {
                reject(BreezSDK.TAG, "Error calling start", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(sync:rejecter:)
    func sync(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                try breezServices.sync()
                
                resolve("BreezServices syncing")
            } catch let err {
                reject(BreezSDK.TAG, "Error calling sync", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
   }
    
    @objc(stop:rejecter:)
    func stop(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                try breezServices.stop()
                
                resolve("BreezServices stopped")
            } catch let err {
                reject(BreezSDK.TAG, "Error calling stop", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(sendPayment:amountSats:resolver:rejecter:)
    func sendPayment(_ bolt11:String, amountSats:String?, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let response = try breezServices.sendPayment(bolt11: bolt11, amountSats: UInt64(amountSats ?? ""))
                
                resolve(BreezSDKMapper.dictionaryOf(payment: response))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling sendPayment", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(sendSpontaneousPayment:amountSats:resolver:rejecter:)
    func sendSpontaneousPayment(_ nodeId:String, amountSats:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                if let amount = UInt64(amountSats) {
                    let response = try breezServices.sendSpontaneousPayment(nodeId: nodeId, amountSats: amount)
                    
                    resolve(BreezSDKMapper.dictionaryOf(payment: response))
                } else {
                    reject(BreezSDK.TAG, "Invalid amountSats", nil)
                }
            } catch let err {
                reject(BreezSDK.TAG, "Error calling sendSpontaneousPayment", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(receivePayment:description:resolver:rejecter:)
    func receivePayment(_ amountSats:String, description:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                if let amount = UInt64(amountSats) {
                    let response = try breezServices.receivePayment(amountSats: amount, description: description)
                    
                    resolve(BreezSDKMapper.dictionaryOf(lnInvoice: response))
                } else {
                    reject(BreezSDK.TAG, "Invalid amountSats", nil)
                }
            } catch let err {
                reject(BreezSDK.TAG, "Error calling receivePayment", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(withdrawLnurl:amountSats:description:resolver:rejecter:)
    func withdrawLnurl(_ reqData:[String: Any], amountSats:String, description:String?, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                if let lnUrlWithdrawRequestData = BreezSDKMapper.asLnUrlWithdrawRequestData(reqData: reqData) {
                    if let amount = UInt64(amountSats) {
                        let response = try breezServices.withdrawLnurl(reqData: lnUrlWithdrawRequestData, amountSats: amount, description: description)
                        
                        resolve(BreezSDKMapper.dictionaryOf(lnUrlWithdrawCallbackStatus: response))
                    } else {
                        reject(BreezSDK.TAG, "Invalid amountSats", nil)
                    }
                } else {
                    reject(BreezSDK.TAG, "Invalid reqData", nil)
                }
            } catch let err {
                reject(BreezSDK.TAG, "Error calling receivePayment", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
}
