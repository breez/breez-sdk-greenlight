import Foundation
import BreezSDK

@objc(RNBreezSDK)
class RNBreezSDK: RCTEventEmitter {
    static let TAG: String = "BreezSDK"
    
    public static var emitter: RCTEventEmitter!
    public static var hasListeners: Bool = false

    private var breezServices: BlockingBreezServices!

    static var breezSdkDirectory: URL {
        let applicationDirectory = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
        let breezSdkDirectory = applicationDirectory.appendingPathComponent("breezSdk", isDirectory: true)
        
        if !FileManager.default.fileExists(atPath: breezSdkDirectory.path) {
            try! FileManager.default.createDirectory(atPath: breezSdkDirectory.path, withIntermediateDirectories: true)
        }
        
        return breezSdkDirectory
    }
    
    override init() {
        super.init()
        RNBreezSDK.emitter = self
    }

    @objc
    override static func moduleName() -> String! {
        TAG
    }
    
    override func supportedEvents() -> [String]! {
        return [BreezSDKListener.emitterName, BreezSDKLogStream.emitterName]
    }
    
    override func startObserving() {
        RNBreezSDK.hasListeners = true
    }
    
    override func stopObserving() {
        RNBreezSDK.hasListeners = false
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
    
    private func ensureWorkingDir(workingDir: String) throws {
        do {
            if !FileManager.default.fileExists(atPath: workingDir) {
                try FileManager.default.createDirectory(atPath: workingDir, withIntermediateDirectories: true)
            }
        } catch {
            throw SdkError.Generic(message: "Mandatory field workingDir must contain a writable directory")
        }
    }

    {% let obj_interface = "BreezSDK." -%}
    {% for func in ci.function_definitions() %}
    {%- if func.name()|ignored_function == false -%}
    {% include "TopLevelFunctionTemplate.swift" %}
    {% endif -%}
    {%- endfor %}  
    @objc(setLogStream:reject:)
    func setLogStream(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try BreezSDK.setLogStream(logStream: BreezSDKLogStream())            
            resolve(["status": "ok"])        
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    
    @objc(connect:resolve:reject:)
    func connect(_ req:[String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) -> Void {
        if self.breezServices != nil {
            reject("Generic", "BreezServices already initialized", nil)
            return
        }
            
        do {
            let connectRequest = try BreezSDKMapper.asConnectRequest(connectRequest: req)
            try ensureWorkingDir(workingDir: connectRequest.config.workingDir)

            self.breezServices = try BreezSDK.connect(req: connectRequest, listener: BreezSDKListener())                
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }
    {%- include "Objects.swift" %}
    
    func rejectErr(err: Error, reject: @escaping RCTPromiseRejectBlock) {
        var errorName = "Generic"
        var message = "\(err)"
        if let errAssociated = Mirror(reflecting: err).children.first {
            errorName = errAssociated.label ?? errorName
            if let associatedMessage = Mirror(reflecting: errAssociated.value).children.first {
                message = associatedMessage.value as! String
            }
        }
        reject(errorName, message, err)
    }
}

{% import "macros.swift" as swift %}
