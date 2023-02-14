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

    @objc(initServices:deviceKey:deviceCert:seed:resolver:rejecter:)
    func initServices(_ apiKey:String, deviceKey:[UInt8], deviceCert:[UInt8], seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if (self.breezServices != nil) {
            reject(BreezSDK.TAG, "BreezServices already initialized", nil)
            return
        }
        
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
    
    @objc(startLogStream:rejecter:)
    func startLogStream(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        
        do {
            try breez_sdk.setLogStream(logStream: BreezSDKLogStream(emitter: self))
            
            resolve("Log stream started")
        } catch let err {
            reject(BreezSDK.TAG, "Error calling setLogStream", err)
        }
    }
}
