import Foundation

class BreezSDKListener: NSObject, EventListener {
    var emitter: RCTEventEmitter
    
    static let emitterName: String = "breezSdkEvent"
    
    init(emitter: RCTEventEmitter) {
        self.emitter = emitter
    }
    
    func onEvent(e: BreezEvent) {
        switch(e) {
        case let .invoicePaid(details):
            self.emitter.sendEvent(withName: BreezSDKListener.emitterName,
                                   body: [
                                    "type": "invoicePaid",
                                    "data": details
                                   ])
        case let .newBlock(block):
            self.emitter.sendEvent(withName: BreezSDKListener.emitterName,
                                   body: [
                                    "type": "newBlock",
                                    "data": block
                                    ])
        case .synced:
            self.emitter.sendEvent(withName: BreezSDKListener.emitterName,
                                   body: [
                                    "type": "synced"
                                   ])
        case let .paymentSucceed(details):
            self.emitter.sendEvent(withName: BreezSDKListener.emitterName,
                                   body: [
                                    "type": "paymentSucceed",
                                    "data": details
                                   ])
        case let .paymentFailed(error):
            self.emitter.sendEvent(withName: BreezSDKListener.emitterName,
                                   body: [
                                    "type": "paymentFailed",
                                    "data": error
                                   ])
        }
        
    }
}
