//
//  BreezListener.swift
//  BreezSDK
//
//  Created by Ross Savage on 04/02/2023.
//  Copyright © 2023 Facebook. All rights reserved.
//

import Foundation

class BreezListener: NSObject, EventListener {
    var emitter: RCTEventEmitter
    
    static let emitterName: String = "breezEvent"
    
    init(emitter: RCTEventEmitter) {
        self.emitter = emitter
    }
    
    func onEvent(e: BreezEvent) {
        switch(e) {
        case let .invoicePaid(details):
            self.emitter.sendEvent(withName: BreezListener.emitterName,
                                   body: [
                                    "type": "invoicePaid",
                                    "data": [
                                        "paymentHash": details.paymentHash,
                                        "bolt11": details.bolt11
                                    ]
                                   ])
        case let .newBlock(block):
            self.emitter.sendEvent(withName: BreezListener.emitterName,
                                   body: [
                                    "type": "newBlock",
                                    "data": [
                                        "block": block
                                       ]
                                    ])
        case .synced:
            self.emitter.sendEvent(withName: BreezListener.emitterName,
                                   body: [
                                    "type": "synced"
                                   ])
        case let .paymentSucceed(details):
            // TODO: fill emitted data with complete payment details
            self.emitter.sendEvent(withName: BreezListener.emitterName,
                                   body: [
                                    "type": "paymentSucceed",
                                    "data": [
                                        "id": details.id
                                    ]
                                   ])
        case let .paymentFailed(error):
            self.emitter.sendEvent(withName: BreezListener.emitterName,
                                   body: [
                                    "type": "paymentFailed",
                                    "data": [
                                        "error": error
                                    ]
                                   ])
        }
        
    }
}
