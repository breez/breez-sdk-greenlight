import Foundation
import BreezSDK

class BreezSDKNodeLogger: NSObject, Logger {
    var emitter: RCTEventEmitter
    
    static let emitterName: String = "breezSdkNodeLog"
    
    init(emitter: RCTEventEmitter) {
        self.emitter = emitter
    }
    
    func log(logMessage: LogMessage) {
        self.emitter.sendEvent(withName: BreezSDKNodeLogger.emitterName, 
                               body: BreezSDKMapper.dictionaryOf(logMessage: logMessage))
    }
}
