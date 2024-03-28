import UserNotifications
import Foundation

struct AddressTxsConfirmedRequest: Codable {
    let address: String
}

class RedeemSwapTask : TaskProtocol {
    fileprivate let TAG = "RedeemSwapTask"
    
    internal var payload: String
    internal var contentHandler: ((UNNotificationContent) -> Void)?
    internal var bestAttemptContent: UNMutableNotificationContent?
    internal var logger: ServiceLogger
    internal var swapAddress: String? = nil
    
    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        self.payload = payload
        self.contentHandler = contentHandler
        self.bestAttemptContent = bestAttemptContent
        self.logger = logger
    }
    
    public func onEvent(e: BreezEvent) {
        if let address = self.swapAddress {
            switch e {
            case .swapUpdated(details: let swapInfo):
                self.logger.log(tag: TAG, line: "Received swap updated event: \(swapInfo.bitcoinAddress), current address: \(address) status: \(swapInfo.status)\n", level: "INFO")
                if address == swapInfo.bitcoinAddress {
                    if (swapInfo.paidMsat > 0) {
                        self.logger.log(tag: TAG, line: "Swap address \(swapInfo.bitcoinAddress) redeemed succesfully", level: "INFO")
                        let successRedeemSwap = ResourceHelper.shared.getString(key: Constants.SWAP_TX_CONFIRMED_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_TITLE)
                        self.displayPushNotification(title: successRedeemSwap, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_SWAP_TX_CONFIRMED)
                    }
                }
                break
            default:
                break
            }
        }
    }
    
    func start(breezSDK: BlockingBreezServices) throws {
        do {
            let addressTxsConfirmedRequest = try JSONDecoder().decode(AddressTxsConfirmedRequest.self, from: self.payload.data(using: .utf8)!)
            swapAddress = addressTxsConfirmedRequest.address
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to decode payload: \(e)", level: "ERROR")
            self.onShutdown()
            throw e
        }
        
        guard let address = swapAddress else {
            self.logger.log(tag: TAG, line: "Failed to process swap notification: swap address not in payload", level: "ERROR")
            self.onShutdown()
            return
        }
        
        do {
            try breezSDK.redeemSwap(swapAddress: address)
            self.logger.log(tag: TAG, line: "Found swap for \(address)", level: "DEBUG")
        } catch let e {
            self.logger.log(tag: TAG, line: "Failed to manually redeem swap notification: \(e)", level: "ERROR")
        }
    }

    func onShutdown() {
        let failRedeemSwap = ResourceHelper.shared.getString(key: Constants.SWAP_TX_CONFIRMED_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_FAILURE_TITLE)
        self.displayPushNotification(title: failRedeemSwap, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_SWAP_TX_CONFIRMED)
    }
}
