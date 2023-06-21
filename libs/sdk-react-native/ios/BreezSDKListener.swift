import Foundation
#if canImport(BreezSDK)
import BreezSDK
#endif

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
                                    "data": BreezSDKMapper.dictionaryOf(invoicePaidDetails: details)
                                   ])
        case let .newBlock(block):
            self.emitter.sendEvent(withName: BreezSDKListener.emitterName,
                                   body: [
                                    "type": "newBlock",
                                    "data": block
                                   ])
        case let .paymentFailed(details):
            self.emitter.sendEvent(withName: BreezSDKListener.emitterName,
                                   body: [
                                    "type": "paymentFailed",
                                    "data": BreezSDKMapper.dictionaryOf(paymentFailedData: details)
                                   ])
        case let .paymentSucceed(details):
            self.emitter.sendEvent(withName: BreezSDKListener.emitterName,
                                   body: [
                                    "type": "paymentSucceed",
                                    "data": BreezSDKMapper.dictionaryOf(payment: details)
                                   ])
       case .synced:
            self.emitter.sendEvent(withName: BreezSDKListener.emitterName,
                                   body: [
                                    "type": "synced"
                                   ])
        }
    }
}
