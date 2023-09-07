import Foundation
import BreezSDK

class BreezSDKListener: NSObject, EventListener {
    var emitter: RCTEventEmitter
    
    static let emitterName: String = "breezSdkEvent"
    
    init(emitter: RCTEventEmitter) {
        self.emitter = emitter
    }
    
    func onEvent(e: BreezEvent) {
        self.emitter.sendEvent(withName: BreezSDKListener.emitterName, 
                               body: BreezSDKMapper.dictionaryOf(breezEvent: e))
    }
}
